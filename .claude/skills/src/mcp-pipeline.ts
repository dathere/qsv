/**
 * MCP Pipeline Tool Implementation
 *
 * Enables chaining multiple qsv operations in a single MCP tool call
 */

import type { McpPipelineStep, McpToolDefinition, McpToolResult } from './types.js';
import type { SkillExecutor } from './executor.js';
import type { SkillLoader } from './loader.js';
import { QsvPipeline } from './pipeline.js';

/**
 * Create the qsv_pipeline tool definition
 */
export function createPipelineToolDefinition(): McpToolDefinition {
  return {
    name: 'qsv_pipeline',
    description: 'Chain multiple qsv operations together in a single pipeline. Automatically pipes data between steps.',
    inputSchema: {
      type: 'object',
      properties: {
        input_file: {
          type: 'string',
          description: 'Path to input CSV file (absolute or relative)',
        },
        steps: {
          type: 'array',
          description: 'Array of pipeline steps to execute in order',
          items: {
            type: 'object',
            properties: {
              command: {
                type: 'string',
                description: 'The qsv command to execute (e.g., "select", "dedup", "stats")',
              },
              params: {
                type: 'object',
                description: 'Parameters for this command (arguments and options)',
              },
            },
            required: ['command'],
          },
        },
        output_file: {
          type: 'string',
          description: 'Path to output CSV file (optional, returns to stdout if omitted)',
        },
      },
      required: ['input_file', 'steps'],
    },
  };
}

/**
 * Execute a qsv pipeline
 */
export async function executePipeline(
  params: Record<string, unknown>,
  loader: SkillLoader,
) {
  try {
    const inputFile = params.input_file as string | undefined;
    const steps = params.steps as McpPipelineStep[] | undefined;
    const outputFile = params.output_file as string | undefined;

    // Validate required parameters
    if (!inputFile) {
      return {
        content: [{
          type: 'text' as const,
          text: 'Error: input_file parameter is required',
        }],
        isError: true,
      };
    }

    if (!steps || !Array.isArray(steps) || steps.length === 0) {
      return {
        content: [{
          type: 'text' as const,
          text: 'Error: steps parameter is required and must be a non-empty array',
        }],
        isError: true,
      };
    }

    // Validate pipeline steps
    for (let i = 0; i < steps.length; i++) {
      const step = steps[i];

      if (!step.command || typeof step.command !== 'string') {
        return {
          content: [{
            type: 'text' as const,
            text: `Error: Step ${i + 1} missing required 'command' property or command is not a string`,
          }],
          isError: true,
        };
      }

      if (step.params && typeof step.params !== 'object') {
        return {
          content: [{
            type: 'text' as const,
            text: `Error: Step ${i + 1} 'params' must be an object`,
          }],
          isError: true,
        };
      }
    }

    // Create pipeline
    const pipeline = new QsvPipeline(loader);

    // Add steps to pipeline
    for (const step of steps) {
      const { command, params: stepParams = {} } = step;

      // Build the pipeline using the fluent API
      await addStepToPipeline(pipeline, command, stepParams);
    }

    // Read input file
    const fs = await import('fs/promises');
    const inputData = await fs.readFile(inputFile);

    // Execute pipeline
    const startTime = Date.now();
    const result = await pipeline.execute(inputData);
    const duration = Date.now() - startTime;

    // Handle output
    if (outputFile) {
      // Write to file
      await fs.writeFile(outputFile, result.output);

      const stepSummary = result.steps
        .map((s, i) => `  ${i + 1}. ${s.metadata.command} (${s.metadata.duration}ms)`)
        .join('\n');

      return {
        content: [{
          type: 'text' as const,
          text: `Pipeline executed successfully!\n\nOutput written to: ${outputFile}\n\nSteps executed:\n${stepSummary}\n\nTotal duration: ${result.totalDuration}ms`,
        }],
      };
    } else {
      // Return CSV output
      return {
        content: [{
          type: 'text' as const,
          text: result.output.toString('utf-8'),
        }],
      };
    }
  } catch (error) {
    return {
      content: [{
        type: 'text' as const,
        text: `Pipeline execution failed: ${error instanceof Error ? error.message : String(error)}`,
      }],
      isError: true,
    };
  }
}

/**
 * Add a step to the pipeline using the fluent API
 */
async function addStepToPipeline(
  pipeline: QsvPipeline,
  command: string,
  params: Record<string, unknown>,
): Promise<void> {
  // Map common commands to pipeline methods
  switch (command) {
    case 'select':
      if (!params.selection) {
        throw new Error(`'select' command requires 'selection' parameter`);
      }
      pipeline.select(params.selection as string, params);
      break;

    case 'search':
      if (!params.pattern) {
        throw new Error(`'search' command requires 'pattern' parameter`);
      }
      pipeline.search(
        params.pattern as string,
        params.column as string | undefined,
        params,
      );
      break;

    case 'dedup':
      pipeline.dedup(params);
      break;

    case 'sort':
      if (!params.column) {
        throw new Error(`'sort' command requires 'column' parameter`);
      }
      pipeline.sortBy(params.column as string, params);
      break;

    case 'slice':
      pipeline.slice(
        params.start as number | undefined,
        params.end as number | undefined,
        params,
      );
      break;

    case 'stats':
      pipeline.stats(params);
      break;

    case 'frequency':
      pipeline.frequency(params);
      break;

    case 'apply':
      if (!params.operations || !params.column) {
        throw new Error(`'apply' command requires 'operations' and 'column' parameters`);
      }
      pipeline.apply(
        params.operations as string,
        params.column as string,
        params,
      );
      break;

    case 'rename':
      if (!params.columns || !params.newNames) {
        throw new Error(`'rename' command requires 'columns' and 'newNames' parameters`);
      }
      pipeline.rename(
        params.columns as string,
        params.newNames as string,
        params,
      );
      break;

    case 'join':
      if (!params.columns || !params.file) {
        throw new Error(`'join' command requires 'columns' and 'file' parameters`);
      }
      pipeline.join(
        params.columns as string,
        params.file as string,
        params,
      );
      break;

    default:
      // For commands without dedicated methods, use the generic add() method
      // Ensure params is a valid object
      if (params && typeof params === 'object' && !Array.isArray(params)) {
        pipeline.add(`qsv-${command}`, {
          args: {},
          options: params,
        });
      } else {
        pipeline.add(`qsv-${command}`, {
          args: {},
          options: {},
        });
      }
      break;
  }
}

/**
 * Generate shell script from pipeline parameters
 */
export async function pipelineToShellScript(
  params: Record<string, unknown>,
  loader: SkillLoader,
): Promise<string> {
  const inputFile = params.input_file as string || 'input.csv';
  const steps = params.steps as McpPipelineStep[] || [];
  const outputFile = params.output_file as string | undefined;

  // Create pipeline
  const pipeline = new QsvPipeline(loader);

  // Add steps
  for (const step of steps) {
    await addStepToPipeline(pipeline, step.command, step.params || {});
  }

  // Generate shell script
  let script = await pipeline.toShellScript();

  // Prepend input file as stdin source
  script = `cat ${inputFile} | ${script}`;

  // Add output redirection if specified
  if (outputFile) {
    script += ` > ${outputFile}`;
  }

  return script;
}
