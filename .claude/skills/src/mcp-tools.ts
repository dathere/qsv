/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

import { spawn } from 'child_process';
import type { QsvSkill, Argument, Option, McpToolDefinition, McpToolProperty } from './types.js';
import type { SkillExecutor } from './executor.js';
import type { SkillLoader } from './loader.js';

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
        if ('needsConversion' in filesystemProvider && 'getConversionCommand' in filesystemProvider) {
          const provider = filesystemProvider as any;
          if (provider.needsConversion(inputFile)) {
            const conversionCmd = provider.getConversionCommand(inputFile);
            console.error(`[MCP Tools] File requires conversion using qsv ${conversionCmd}`);

            // Convert file using qsv excel or qsv jsonl
            try {
              // Determine which qsv binary to use (from env or default)
              const qsvBin = process.env.QSV_BIN_PATH || 'qsv';

              // Run conversion command: qsv excel/jsonl <input> --output <temp.csv>
              const tempCsv = inputFile + '.converted.csv';
              const conversionArgs = [conversionCmd, inputFile, '--output', tempCsv];

              console.error(`[MCP Tools] Running conversion: ${qsvBin} ${conversionArgs.join(' ')}`);

              await new Promise<void>((resolve, reject) => {
                const proc = spawn(qsvBin, conversionArgs, {
                  stdio: ['ignore', 'ignore', 'pipe']
                });

                let stderr = '';
                proc.stderr?.on('data', (chunk) => {
                  stderr += chunk.toString();
                });

                proc.on('close', (code) => {
                  if (code === 0) {
                    resolve();
                  } else {
                    reject(new Error(`Conversion failed with exit code ${code}: ${stderr}`));
                  }
                });
                proc.on('error', reject);
              });

              // Use the converted CSV as input
              inputFile = tempCsv;
              console.error(`[MCP Tools] Conversion successful: ${tempCsv}`);

              // Auto-index the converted CSV if it's >10MB
              try {
                const { stat } = await import('fs/promises');
                const stats = await stat(tempCsv);
                const fileSizeMB = stats.size / (1024 * 1024);

                if (fileSizeMB > 10) {
                  console.error(`[MCP Tools] Converted file is ${fileSizeMB.toFixed(1)}MB, creating index...`);

                  await new Promise<void>((resolve) => {
                    const indexProc = spawn(qsvBin, ['index', tempCsv], {
                      stdio: ['ignore', 'ignore', 'pipe']
                    });

                    let indexStderr = '';
                    indexProc.stderr?.on('data', (chunk) => {
                      indexStderr += chunk.toString();
                    });

                    indexProc.on('close', (code) => {
                      if (code === 0) {
                        console.error(`[MCP Tools] Index created for converted file`);
                      } else {
                        console.error(`[MCP Tools] Index creation failed (continuing anyway): ${indexStderr}`);
                      }
                      resolve();
                    });
                    indexProc.on('error', (err) => {
                      console.error(`[MCP Tools] Index creation error (continuing anyway):`, err);
                      resolve();
                    });
                  });
                }
              } catch (indexError) {
                console.error(`[MCP Tools] Auto-indexing converted file error (continuing anyway):`, indexError);
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

        // Auto-index native CSV files if they're large (>10MB) and not indexed
        // Note: Snappy-compressed files (.sz) cannot be indexed
        try {
          const { stat, access } = await import('fs/promises');
          const { constants } = await import('fs');
          const { basename } = await import('path');

          // Check if this is an indexable CSV format (not snappy-compressed)
          const filename = basename(inputFile).toLowerCase();
          const isIndexable =
            filename.endsWith('.csv') || filename.endsWith('.tsv') ||
            filename.endsWith('.tab') || filename.endsWith('.ssv');

          if (isIndexable) {
            const stats = await stat(inputFile);
            const fileSizeMB = stats.size / (1024 * 1024);
            const indexPath = inputFile + '.idx';

            // Check if index exists
            let indexExists = false;
            try {
              await access(indexPath, constants.F_OK);
              indexExists = true;
            } catch {
              indexExists = false;
            }

            // Create index if file is >10MB and not already indexed
            if (fileSizeMB > 10 && !indexExists) {
              console.error(`[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, creating index...`);

              const qsvBin: string = process.env.QSV_BIN_PATH || 'qsv';
              const indexArgs = ['index', inputFile];

              await new Promise<void>((resolve) => {
                const proc = spawn(qsvBin, indexArgs, {
                  stdio: ['ignore', 'ignore', 'pipe']
                });

                let stderr = '';
                proc.stderr?.on('data', (chunk) => {
                  stderr += chunk.toString();
                });

                proc.on('close', (code) => {
                  if (code === 0) {
                    console.error(`[MCP Tools] Index created successfully: ${indexPath}`);
                    resolve();
                  } else {
                    // Don't fail if indexing fails - just log and continue
                    console.error(`[MCP Tools] Index creation failed (continuing anyway): ${stderr}`);
                    resolve();
                  }
                });
                proc.on('error', (err) => {
                  console.error(`[MCP Tools] Index creation error (continuing anyway):`, err);
                  resolve();
                });
              });
            } else if (indexExists) {
              console.error(`[MCP Tools] Index already exists: ${indexPath}`);
            } else {
              console.error(`[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, skipping auto-indexing`);
            }
          }
        } catch (indexError) {
          // Don't fail if indexing fails - just log and continue
          console.error(`[MCP Tools] Auto-indexing error (continuing anyway):`, indexError);
        }

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
