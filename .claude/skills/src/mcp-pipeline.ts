/**
 * MCP Pipeline Tool Implementation
 *
 * Enables chaining multiple qsv operations in a single MCP tool call
 */

import type {
  McpPipelineStep,
  McpToolDefinition,
  McpToolResult,
} from "./types.js";
import type { SkillExecutor } from "./executor.js";
import type { SkillLoader } from "./loader.js";
import { QsvPipeline } from "./pipeline.js";
import { config } from "./config.js";

/**
 * Create the qsv_pipeline tool definition
 */
export function createPipelineToolDefinition(): McpToolDefinition {
  return {
    name: "qsv_pipeline",
    description: `Execute multi-step qsv workflows by chaining commands together. Each step's output becomes the next step's input.

💡 USE WHEN: You need 2+ operations in sequence (e.g., "remove duplicates, then sort by revenue DESC, then take top 100 rows").

🚀 BENEFITS:
- Automatic intermediate file management (you don't handle temp files)
- Automatic indexing between steps for performance
- Single coordinated operation from user perspective
- More efficient than separate tool calls (no redundant I/O)
- Better error handling (rollback on failure)

📋 COMMON WORKFLOWS:
1. **Data Cleaning**: dedup → select (remove columns) → validate
2. **Analysis**: stats → frequency (on specific columns) → tojsonl
3. **Filter & Sort**: search (filter rows) → select (pick columns) → sort → slice (top N)
4. **Complex Query**: select → search → apply → sort
5. **Aggregation**: sqlp (GROUP BY) → sort → slice

⚠️  LIMITATIONS:
- Max ${config.maxPipelineSteps} steps per pipeline (configurable)
- Linear workflows only (A → B → C), not branching/parallel
- Each step must succeed before next step runs
- All steps share same timeout (${Math.round(config.operationTimeoutMs / 1000)}s total)

⚠️  CAUTION:
- Memory-intensive commands (dedup, sort) in pipeline still load full data
- For very large files, consider breaking into separate operations
- Pipeline fails if any step fails (atomic operation)

📝 EXAMPLE - Top 10 Products by Revenue:
{
  "input_file": "sales.csv",
  "steps": [
    {
      "command": "dedup",
      "params": {}
    },
    {
      "command": "select",
      "params": {"selection": "product,revenue"}
    },
    {
      "command": "sort",
      "params": {"columns": "revenue", "reverse": true}
    },
    {
      "command": "slice",
      "params": {"end": 10}
    }
  ],
  "output_file": "top_products.csv"
}`,
    inputSchema: {
      type: "object",
      properties: {
        input_file: {
          type: "string",
          description:
            "Path to input CSV file (absolute or relative). Will be auto-indexed if >10MB for better performance.",
        },
        steps: {
          type: "array",
          description: `Array of pipeline steps to execute in order. Each step transforms the data and passes to next step. Max ${config.maxPipelineSteps} steps.`,
          items: {
            type: "object",
            properties: {
              command: {
                type: "string",
                description:
                  'The qsv command name (without "qsv_" prefix). Examples: "select", "dedup", "stats", "search", "sort", "slice".',
              },
              params: {
                type: "object",
                description:
                  'Parameters for this command. Keys are parameter names (use underscore for multi-word like "ignore_case"). Omit input_file (auto-piped from previous step).',
              },
            },
            required: ["command"],
          },
        },
        output_file: {
          type: "string",
          description:
            "Path to final output CSV file (optional). If omitted, small results (<850KB) return directly; large results auto-saved to working directory.",
        },
      },
      required: ["input_file", "steps"],
    },
  };
}

/**
 * Execute a qsv pipeline
 */
export async function executePipeline(
  params: Record<string, unknown>,
  loader: SkillLoader,
  filesystemProvider?: {
    resolvePath: (path: string) => Promise<string>;
    getWorkingDirectory: () => string;
  },
) {
  try {
    let inputFile = params.input_file as string | undefined;
    let outputFile = params.output_file as string | undefined;
    const steps = params.steps as McpPipelineStep[] | undefined;

    // Validate required parameters
    if (!inputFile) {
      return {
        content: [
          {
            type: "text" as const,
            text: "Error: input_file parameter is required",
          },
        ],
        isError: true,
      };
    }

    // Resolve file paths using filesystem provider if available
    if (filesystemProvider) {
      try {
        inputFile = await filesystemProvider.resolvePath(inputFile);
        if (outputFile) {
          outputFile = await filesystemProvider.resolvePath(outputFile);
        }
      } catch (error) {
        return {
          content: [
            {
              type: "text" as const,
              text: `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`,
            },
          ],
          isError: true,
        };
      }
    }

    if (!steps || !Array.isArray(steps) || steps.length === 0) {
      return {
        content: [
          {
            type: "text" as const,
            text: "Error: steps parameter is required and must be a non-empty array",
          },
        ],
        isError: true,
      };
    }

    // Enforce pipeline step limit
    if (steps.length > config.maxPipelineSteps) {
      return {
        content: [
          {
            type: "text" as const,
            text: `Error: Pipeline exceeds maximum step limit (${config.maxPipelineSteps}). Requested ${steps.length} steps.`,
          },
        ],
        isError: true,
      };
    }

    // Validate pipeline steps
    for (let i = 0; i < steps.length; i++) {
      const step = steps[i];

      if (!step.command || typeof step.command !== "string") {
        return {
          content: [
            {
              type: "text" as const,
              text: `Error: Step ${i + 1} missing required 'command' property or command is not a string`,
            },
          ],
          isError: true,
        };
      }

      if (
        step.params &&
        (typeof step.params !== "object" ||
          step.params === null ||
          Array.isArray(step.params))
      ) {
        return {
          content: [
            {
              type: "text" as const,
              text: `Error: Step ${i + 1} 'params' must be an object (not null or array)`,
            },
          ],
          isError: true,
        };
      }
    }

    // Create pipeline with executor that uses the configured qsv binary path
    // and working directory for consistent file resolution
    // This prevents 'spawn qsv ENOENT' errors when qsv is not in PATH
    const { SkillExecutor } = await import("./executor.js");
    const workingDir =
      filesystemProvider?.getWorkingDirectory() || config.workingDir;
    const executor = new SkillExecutor(config.qsvBinPath, workingDir);
    const pipeline = new QsvPipeline(loader, executor);

    // Add steps to pipeline
    for (const step of steps) {
      const { command, params: stepParams = {} } = step;

      // Build the pipeline using the fluent API
      await addStepToPipeline(pipeline, command, stepParams);
    }

    // Read input file
    const fs = await import("fs/promises");
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
        .map(
          (s, i) =>
            `  ${i + 1}. ${s.metadata.command} (${s.metadata.duration}ms)`,
        )
        .join("\n");

      return {
        content: [
          {
            type: "text" as const,
            text: `Pipeline executed successfully!\n\nOutput written to: ${outputFile}\n\nSteps executed:\n${stepSummary}\n\nTotal duration: ${result.totalDuration}ms`,
          },
        ],
      };
    } else {
      // Return CSV output
      return {
        content: [
          {
            type: "text" as const,
            text: result.output.toString("utf-8"),
          },
        ],
      };
    }
  } catch (error) {
    return {
      content: [
        {
          type: "text" as const,
          text: `Pipeline execution failed: ${error instanceof Error ? error.message : String(error)}`,
        },
      ],
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
    case "select":
      if (!params.selection) {
        throw new Error(`'select' command requires 'selection' parameter`);
      }
      pipeline.select(params.selection as string, params);
      break;

    case "search":
      if (!params.pattern) {
        throw new Error(`'search' command requires 'pattern' parameter`);
      }
      pipeline.search(
        params.pattern as string,
        params.column as string | undefined,
        params,
      );
      break;

    case "dedup":
      pipeline.dedup(params);
      break;

    case "sort":
      if (!params.column) {
        throw new Error(`'sort' command requires 'column' parameter`);
      }
      pipeline.sortBy(params.column as string, params);
      break;

    case "slice":
      pipeline.slice(
        params.start as number | undefined,
        params.end as number | undefined,
        params,
      );
      break;

    case "stats":
      pipeline.stats(params);
      break;

    case "frequency":
      pipeline.frequency(params);
      break;

    case "apply":
      if (!params.operations || !params.column) {
        throw new Error(
          `'apply' command requires 'operations' and 'column' parameters`,
        );
      }
      pipeline.apply(
        params.operations as string,
        params.column as string,
        params,
      );
      break;

    case "rename":
      if (!params.columns || !params.newNames) {
        throw new Error(
          `'rename' command requires 'columns' and 'newNames' parameters`,
        );
      }
      pipeline.rename(
        params.columns as string,
        params.newNames as string,
        params,
      );
      break;

    case "join":
      if (!params.columns || !params.file) {
        throw new Error(
          `'join' command requires 'columns' and 'file' parameters`,
        );
      }
      pipeline.join(params.columns as string, params.file as string, params);
      break;

    default:
      // For commands without dedicated methods, use the generic add() method
      // Ensure params is a valid object (not null or array)
      if (
        params &&
        typeof params === "object" &&
        params !== null &&
        !Array.isArray(params)
      ) {
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
  const inputFile = (params.input_file as string) || "input.csv";
  const steps = (params.steps as McpPipelineStep[]) || [];
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
