/**
 * MCP tool definition creation functions.
 */

import type {
  QsvSkill,
  McpToolDefinition,
  McpToolProperty,
} from "./types.js";
import { SKILL_CATEGORIES } from "./types.js";
import { enhanceDescription, enhanceParameterDescription } from "./command-guidance.js";
import { COMMON_COMMANDS, MAX_LOG_MESSAGE_LEN } from "./tool-constants.js";
import { config } from "./config.js";
import { mapSchemaType } from "./file-operations.js";
import type { SkillLoader } from "./loader.js";

/**
 * Convert a QSV skill to an MCP tool definition
 */
export function createToolDefinition(skill: QsvSkill): McpToolDefinition {
  const properties: Record<string, McpToolProperty> = {
    input_file: {
      type: "string",
      description:
        "Path to input CSV file. Use absolute paths for reliability. Required for command execution, but may be omitted when help=true.",
    },
    help: {
      type: "boolean",
      description: "Set to true to view command documentation instead of executing.",
    },
  };

  // input_file is enforced at runtime (not in schema) so help-only calls work
  const required: string[] = [];

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

      // Positional args enforced at runtime (not in schema) so help-only calls work
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

  // Add _reason meta-parameter for audit logging
  properties._reason = {
    type: "string",
    description:
      "Optional human-readable reason for this invocation, recorded in the MCP audit log. If omitted, the tool name is used.",
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

Common commands via this tool: join, sort, dedup, rename, validate, sample, template, diff, schema, and 30+ more.

❓ HELP: For any command details, use options={"--help": true}. Example: command="sort", options={"--help": true}`,
    inputSchema: {
      type: "object",
      properties: {
        command: {
          type: "string",
          description:
            'The qsv command to execute (e.g., "sort", "sample", "partition")',
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
        _reason: {
          type: "string",
          description:
            "Optional human-readable reason for this invocation, recorded in the MCP audit log. If omitted, the tool name is used.",
        },
      },
      required: ["command"],
    },
  };
}

/**
 * Create qsv_list_files tool definition
 */
export function createListFilesTool(): McpToolDefinition {
  return {
    name: "qsv_list_files",
    description: `List tabular data files in a directory for browsing and discovery.

💡 USE WHEN:
- User asks "what files do I have?" or "what's in my Downloads folder?"
- Starting a session and need to discover available datasets
- User mentions a directory but not a specific file
- Verifying files exist before processing

🔍 SHOWS: File name, size, format type, last modified date.

📂 SUPPORTED FORMATS:
- **Native CSV**: .csv, .tsv, .tab, .ssv (and .sz snappy-compressed)
- **Excel** (auto-converts): .xls, .xlsx, .xlsm, .xlsb, .ods
- **JSONL** (auto-converts): .jsonl, .ndjson

🚀 WORKFLOW: Always list files first when user mentions a directory. This helps you:
1. See what files are available
2. Get exact file names (avoid typos)
3. Check file sizes (prepare for large files)
4. Identify file formats (know if conversion needed)

💡 TIP: Use non-recursive (default) for faster listing, recursive when searching subdirectories.`,
    inputSchema: {
      type: "object",
      properties: {
        directory: {
          type: "string",
          description:
            "Directory path (absolute or relative to working directory). Omit to use current working directory.",
        },
        recursive: {
          type: "boolean",
          description:
            "Scan subdirectories recursively (default: false). Enable for deep directory searches. May be slow for large directory trees.",
        },
      },
    },
  };
}

/**
 * Create qsv_set_working_dir tool definition
 */
export function createSetWorkingDirTool(): McpToolDefinition {
  return {
    name: "qsv_set_working_dir",
    description: `Change the working directory for all subsequent file operations.

💡 USE WHEN:
- User says "work with files in my Downloads folder"
- Switching between different data directories
- User provides directory path without specific file
- Setting up environment for multiple file operations

⚙️  BEHAVIOR:
- All relative file paths resolved from this directory
- Affects: qsv_list_files, all qsv commands with input_file
- Persists for entire session (until changed again)
- Validates directory exists and is accessible
- Pass "auto" as directory to re-enable automatic root-based sync

🔒 SECURITY: Only allowed directories can be set (configured in server settings).

💡 TIP: Set working directory once at session start, then use simple filenames like "data.csv" instead of full paths.
Call without arguments to show an interactive directory picker (when supported by the MCP client).`,
    inputSchema: {
      type: "object",
      properties: {
        directory: {
          type: "string",
          description:
            'New working directory path (absolute or relative). Must be within allowed directories for security. Omit to show an interactive directory picker.',
        },
      },
      required: [],
    },
    ...(config.enableMcpApps
      ? {
          _meta: {
            "ui/resourceUri": "ui://qsv/directory-picker",
            ui: {
              resourceUri: "ui://qsv/directory-picker",
            },
          },
        }
      : {}),
  };
}

/**
 * Create qsv_browse_directory tool definition (App-only helper).
 * Hidden from the LLM — only callable by the directory picker App.
 */
export function createBrowseDirectoryTool(): McpToolDefinition {
  return {
    name: "qsv_browse_directory",
    description: "Browse a directory's contents for the directory picker App. Returns subdirectories with tabular file counts.",
    inputSchema: {
      type: "object",
      properties: {
        directory: {
          type: "string",
          description: "Absolute path to browse. Defaults to the current working directory.",
        },
      },
      required: [],
    },
    _meta: {
      ui: {
        visibility: ["app"],
      },
    },
  };
}

/**
 * Create qsv_get_working_dir tool definition
 */
export function createGetWorkingDirTool(): McpToolDefinition {
  return {
    name: "qsv_get_working_dir",
    description: `Get the current working directory path.

💡 USE WHEN:
- Confirming where files will be read from/written to
- User asks "where am I working?" or "what's my current directory?"
- Debugging file path issues
- Verifying working directory before operations

📍 RETURNS: Absolute path to current working directory.

💡 TIP: Call this after qsv_set_working_dir to confirm the change succeeded.`,
    inputSchema: {
      type: "object",
      properties: {},
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
- **Category**: Filter by category (selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, documentation, utility)
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
            "Filter by category: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, documentation, utility",
          enum: [...SKILL_CATEGORIES],
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
 * Create the qsv_log tool definition.
 */
export function createLogTool(): McpToolDefinition {
  return {
    name: "qsv_log",
    description: `Write a structured entry to the qsv audit log (qsvmcp.log) for reproducibility.

💡 USE WHEN:
- Recording key reasoning or decisions that led to a particular tool choice
- Summarizing results after a workflow completes
- Adding free-form annotations to the audit log

📋 COMMON PATTERN:
1. Log "agent_reasoning" before complex decisions (e.g., choosing joinp over join)
2. Log "result_summary" after completing a workflow

📝 ENTRY TYPES:
- agent_reasoning — Why you chose a particular approach
- agent_action — A significant action taken (beyond automatic audit logging)
- result_summary — Outcome of a completed workflow
- note — Free-form annotation

⚠️ CAUTION: Keep messages concise. Max ${MAX_LOG_MESSAGE_LEN} chars (truncated silently). Newlines are collapsed to spaces. Logging never fails the workflow.`,
    inputSchema: {
      type: "object",
      properties: {
        entry_type: {
          type: "string",
          enum: ["agent_reasoning", "agent_action", "result_summary", "note"],
          description: "Category of log entry.",
        },
        message: {
          type: "string",
          description: "The log message content.",
        },
      },
      required: ["entry_type", "message"],
    },
  };
}

/**
 * Create the qsv_setup tool definition.
 * Exposed when qsv binary is not found, so the LLM can offer to install it.
 */
export function createSetupToolDefinition(): McpToolDefinition {
  const platformHint =
    process.platform === "darwin" || process.platform === "win32"
      ? "direct download from GitHub Releases"
      : "manual download from GitHub releases";

  return {
    name: "qsv_setup",
    description:
      `Install the qsv data-wrangling toolkit on this machine. ` +
      `On this platform, installation uses: ${platformHint}. ` +
      `Call this tool with confirm=true to proceed with installation.`,
    inputSchema: {
      type: "object",
      properties: {
        confirm: {
          type: "boolean",
          description:
            "Must be true to proceed with installation. " +
            "This ensures the user explicitly approves the install action.",
        },
      },
      required: ["confirm"],
    },
  };
}
