/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

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
