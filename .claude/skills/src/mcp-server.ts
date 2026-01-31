#!/usr/bin/env node
/**
 * QSV MCP Server
 *
 * Model Context Protocol server exposing qsv's tabular data-wrangling commands
 * to Claude Desktop and other MCP clients.
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
  ListPromptsRequestSchema,
  GetPromptRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

import { SkillLoader } from "./loader.js";
import { SkillExecutor } from "./executor.js";
import { FilesystemResourceProvider } from "./mcp-filesystem.js";
import type { McpToolResult } from "./types.js";
import { config } from "./config.js";
import {
  isToolSearchCapableClient,
  getClientType,
  formatClientInfo,
} from "./client-detector.js";
import {
  COMMON_COMMANDS,
  createToolDefinition,
  createGenericToolDefinition,
  handleToolCall,
  handleGenericCommand,
  createConfigTool,
  createSearchToolsTool,
  createDataProfileTool,
  handleConfigTool,
  handleSearchToolsCall,
  handleDataProfileCall,
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
} from "./mcp-tools.js";
import {
  createPipelineToolDefinition,
  executePipeline,
} from "./mcp-pipeline.js";
import { VERSION } from "./version.js";
import { UpdateChecker, getUpdateConfigFromEnv } from "./update-checker.js";

/**
 * QSV MCP Server implementation
 */
class QsvMcpServer {
  private server: Server;
  private loader: SkillLoader;
  private executor: SkillExecutor;
  private filesystemProvider: FilesystemResourceProvider;
  private updateChecker: UpdateChecker;
  private loggedClientDetection: boolean = false;

  constructor() {
    this.server = new Server(
      {
        name: "qsv-server",
        version: VERSION,
      },
      {
        capabilities: {
          tools: {},
          resources: {},
          prompts: {},
        },
      },
    );

    this.loader = new SkillLoader();
    this.executor = new SkillExecutor(config.qsvBinPath, config.workingDir);

    // Initialize filesystem provider with configurable directories
    this.filesystemProvider = new FilesystemResourceProvider({
      workingDirectory: config.workingDir,
      allowedDirectories: config.allowedDirs,
      qsvBinPath: config.qsvBinPath,
    });

    // Initialize update checker with environment configuration
    this.updateChecker = new UpdateChecker(
      config.qsvBinPath,
      undefined, // Use default skills directory
      getUpdateConfigFromEnv(),
    );
  }

  /**
   * Initialize the server and register handlers
   */
  async initialize(): Promise<void> {
    console.error("");
    console.error("=".repeat(60));
    console.error("QSV MCP SERVER INITIALIZATION");
    console.error("=".repeat(60));
    console.error("");

    // Skills will be loaded on first ListTools request (lazy loading)
    console.error("[Init] Skills loading deferred to first request");
    console.error("");

    // Validate qsv binary
    this.logQsvValidation();

    // Check for updates (if enabled)
    await this.checkForUpdates();

    // Register tool handlers
    console.error("[Init] Registering tool handlers...");
    this.registerToolHandlers();
    console.error("[Init] ✓ Tool handlers registered");
    console.error("");

    // Register resource handlers
    console.error("[Init] Registering resource handlers...");
    this.registerResourceHandlers();
    console.error("[Init] ✓ Resource handlers registered");
    console.error("");

    // Register prompt handlers
    console.error("[Init] Registering prompt handlers...");
    this.registerPromptHandlers();
    console.error("[Init] ✓ Prompt handlers registered");
    console.error("");

    console.error("=".repeat(60));
    console.error("✅ QSV MCP SERVER READY");
    console.error("=".repeat(60));
    console.error("");
  }

  /**
   * Log qsv binary validation results
   */
  private logQsvValidation(): void {
    const validation = config.qsvValidation;

    if (validation.valid) {
      console.error("");
      console.error("✅ qsv binary validated successfully");
      console.error(`   Path: ${validation.path}`);
      console.error(`   Version: ${validation.version}`);
      console.error("");
    } else {
      console.error("");
      console.error("❌ qsv binary validation FAILED");
      console.error(`   ${validation.error}`);
      console.error("");
      console.error(
        "⚠️  The extension will not function without a valid qsv binary",
      );
      console.error("");

      if (config.isExtensionMode) {
        console.error("To fix this in Claude Desktop:");
        console.error(
          "   1. Install qsv from: https://github.com/dathere/qsv#installation",
        );
        console.error("   2. Ensure qsv is in your system PATH");
        console.error("   3. Open Claude Desktop Settings > Extensions > qsv");
        console.error(
          `   4. Update "qsv Binary Path" to the correct path (or leave as "qsv" if in PATH)`,
        );
        console.error(
          "   5. Save settings (extension will auto-restart and re-validate)",
        );
      } else {
        console.error("To fix this:");
        console.error(
          "   1. Install qsv from: https://github.com/dathere/qsv#installation",
        );
        console.error("   2. Ensure qsv is in your PATH, or");
        console.error(
          "   3. Set QSV_MCP_BIN_PATH to the absolute path of your qsv binary",
        );
        console.error("   4. Restart the MCP server");
      }
      console.error("");
    }
  }

  /**
   * Check for updates and optionally auto-regenerate skills
   */
  private async checkForUpdates(): Promise<void> {
    try {
      // Quick check first (no network calls)
      const quickCheck = await this.updateChecker.quickCheck();

      if (quickCheck.skillsOutdated) {
        console.error("");
        console.error("⚠️  VERSION MISMATCH DETECTED ⚠️");
        console.error(`   qsv binary: ${quickCheck.versions.qsvBinaryVersion}`);
        console.error(
          `   Skills generated with: ${quickCheck.versions.skillsGeneratedWithVersion}`,
        );
        console.error("");

        // Attempt auto-regeneration if configured
        const autoRegenerated = await this.updateChecker.autoRegenerateSkills();

        if (autoRegenerated) {
          console.error("✅ Skills auto-regenerated successfully");
          console.error(
            "   Please restart the MCP server to load updated skills",
          );
          console.error("");
        } else {
          console.error("ℹ️  To update skills manually, run:");
          console.error("   qsv --update-mcp-skills");
          console.error("   Then restart the MCP server");
          console.error("");
        }
      }

      // Perform full check (with network calls) in the background
      // This won't block server startup
      setImmediate(async () => {
        try {
          const fullCheck = await this.updateChecker.checkForUpdates();

          if (fullCheck.recommendations.length > 0) {
            console.error("");
            console.error("📦 UPDATE CHECK RESULTS:");
            fullCheck.recommendations.forEach((rec) => {
              console.error(rec);
            });
            console.error("");
          }
        } catch (error) {
          // Non-critical error - don't block server
          console.error(
            "[UpdateChecker] Background update check failed:",
            error,
          );
        }
      });
    } catch (error) {
      // Non-critical error - don't block server startup
      console.error("[UpdateChecker] Failed to check for updates:", error);
    }
  }

  /**
   * Register MCP tool handlers
   */
  private registerToolHandlers(): void {
    console.error("[Server] Registering tool handlers...");
    try {
      // List tools handler
      this.server.setRequestHandler(ListToolsRequestSchema, async () => {
        console.error("[Server] Handling tools/list request...");
        const tools = [];

        // Get available commands from qsv binary
        const availableCommands = config.qsvValidation.availableCommands;

        // Determine if we should expose all tools
        // Priority:
        // 1. config.exposeAllTools === true → always expose all
        // 2. config.exposeAllTools === false → always expose only common (override auto-detect)
        // 3. config.exposeAllTools === undefined → auto-detect based on client
        const clientInfo = this.server.getClientVersion();
        let shouldExposeAll: boolean;
        let reason: string;

        if (config.exposeAllTools === true) {
          shouldExposeAll = true;
          reason = "QSV_MCP_EXPOSE_ALL_TOOLS=true";
        } else if (config.exposeAllTools === false) {
          shouldExposeAll = false;
          reason = "QSV_MCP_EXPOSE_ALL_TOOLS=false (explicit disable)";
        } else {
          // Auto-detect based on client
          shouldExposeAll = isToolSearchCapableClient(clientInfo);
          reason = shouldExposeAll
            ? `auto-detected ${getClientType(clientInfo)} client`
            : "unknown client (using common tools only)";
        }

        // Log client detection once per session
        if (!this.loggedClientDetection) {
          const clientDesc = formatClientInfo(clientInfo);
          if (shouldExposeAll) {
            console.error(`[Server] ✅ Enabling all tools mode (${reason})`);
            console.error(`[Server]    Client: ${clientDesc}`);
          } else {
            console.error(`[Server] Using common tools mode (${reason})`);
            console.error(`[Server]    Client: ${clientDesc}`);
          }
          this.loggedClientDetection = true;
        }

        // Check if we should expose all tools (for clients with tool search support)
        if (shouldExposeAll) {
          // Expose all available skills as individual tools
          // Load all skills (cached after first call)
          if (!this.loader.isAllLoaded()) {
            console.error("[Server] First request - loading all skills...");
          } else {
            console.error(
              "[Server] Expose all tools mode - using cached skills",
            );
          }

          const allSkills = await this.loader.loadAll();
          let loadedCount = 0;
          let skippedCount = 0;

          for (const [skillName, skill] of allSkills) {
            // Extract command name from skill name (qsv-select -> select)
            const commandName = skillName.replace("qsv-", "");

            // Skip if command not available in qsv binary
            if (availableCommands && !availableCommands.includes(commandName)) {
              skippedCount++;
              continue;
            }

            try {
              const toolDef = createToolDefinition(skill);
              tools.push(toolDef);
              loadedCount++;
            } catch (error) {
              console.error(
                `[Server] ✗ Error creating tool definition for ${skillName}:`,
                error,
              );
            }
          }

          console.error(
            `[Server] ✓ Loaded ${loadedCount} tools (skipped ${skippedCount} unavailable commands)`,
          );
        } else {
          // Standard mode: Only expose common command tools
          const filteredCommands = availableCommands
            ? COMMON_COMMANDS.filter((cmd) => availableCommands.includes(cmd))
            : COMMON_COMMANDS; // Fallback to all if availableCommands not detected

          // Load only the skills we need (batch loading)
          const skillNames = filteredCommands.map((cmd) => `qsv-${cmd}`);
          console.error(
            `[Server] Loading ${skillNames.length} common command skills...`,
          );

          const loadedSkills = await this.loader.loadByNames(skillNames);
          console.error(
            `[Server] Loaded ${loadedSkills.size}/${filteredCommands.length} common skills`,
          );

          // Log any skills that failed to load (requested but not returned)
          const loadedSkillNames = new Set(loadedSkills.keys());
          const failedSkillNames = skillNames.filter(
            (name) => !loadedSkillNames.has(name),
          );
          if (failedSkillNames.length > 0) {
            console.error(
              `[Server] ⚠ Failed to load skills: ${failedSkillNames.join(", ")}`,
            );
          }

          for (const [skillName, skill] of loadedSkills) {
            try {
              const toolDef = createToolDefinition(skill);
              tools.push(toolDef);
            } catch (error) {
              console.error(
                `[Server] ✗ Error creating tool definition for ${skillName}:`,
                error,
              );
            }
          }

          // Log any skipped commands
          if (availableCommands) {
            const skippedCommands = COMMON_COMMANDS.filter(
              (cmd) => !availableCommands.includes(cmd),
            );
            if (skippedCommands.length > 0) {
              console.error(
                `[Server] ⚠ Skipped unavailable commands: ${skippedCommands.join(", ")}`,
              );
            }
          }
          console.error(`[Server] Loaded ${tools.length} common command tools`);
        }

        // Add generic qsv_command tool
        console.error("[Server] Adding generic command tool...");
        try {
          const genericTool = createGenericToolDefinition(this.loader);
          console.error(
            "[Server] Generic tool created:",
            JSON.stringify(genericTool).substring(0, 200),
          );
          tools.push(genericTool);
          console.error("[Server] Generic tool added successfully");
        } catch (error) {
          console.error("[Server] Error creating generic tool:", error);
          throw error;
        }

        // Add pipeline tool
        console.error("[Server] Adding pipeline tool...");
        try {
          const pipelineTool = createPipelineToolDefinition();
          console.error(
            "[Server] Pipeline tool created:",
            JSON.stringify(pipelineTool).substring(0, 200),
          );
          tools.push(pipelineTool);
          console.error("[Server] Pipeline tool added successfully");
        } catch (error) {
          console.error("[Server] Error creating pipeline tool:", error);
          throw error;
        }

        // Add config, search, and data profile tools
        console.error("[Server] Adding config, search, and data profile tools...");
        try {
          tools.push(createConfigTool());
          tools.push(createSearchToolsTool());
          tools.push(createDataProfileTool());
          console.error("[Server] config, search, and data profile tools added successfully");
        } catch (error) {
          console.error("[Server] Error creating config/search/data profile tools:", error);
          throw error;
        }

        // Add filesystem tools
        console.error("[Server] Adding filesystem tools...");
        tools.push({
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
        });

        tools.push({
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

🔒 SECURITY: Only allowed directories can be set (configured in server settings).

💡 TIP: Set working directory once at session start, then use simple filenames like "data.csv" instead of full paths.`,
          inputSchema: {
            type: "object",
            properties: {
              directory: {
                type: "string",
                description:
                  "New working directory path (absolute or relative). Must be within allowed directories for security.",
              },
            },
            required: ["directory"],
          },
        });

        tools.push({
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
        });

        console.error(`[Server] Registered ${tools.length} tools`);
        console.error(
          `[Server] Tool names: ${tools.map((t) => t.name).join(", ")}`,
        );

        const response = { tools };
        console.error(
          `[Server] Returning ${response.tools.length} tools to client`,
        );

        return response;
      });
      console.error("[Server] Tool handlers registered successfully");
    } catch (error) {
      console.error("[Server] Error registering tool handlers:", error);
      throw error;
    }

    // Call tool handler
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      console.error(`Tool called: ${name}`);

      try {
        // Handle filesystem tools
        if (name === "qsv_list_files") {
          const directory = args?.directory as string | undefined;
          const recursive = args?.recursive as boolean | undefined;

          const result = await this.filesystemProvider.listFiles(
            directory,
            recursive || false,
          );

          const fileList = result.resources
            .map((r) => `- ${r.name} (${r.description})`)
            .join("\n");

          return {
            content: [
              {
                type: "text" as const,
                text: `Found ${result.resources.length} tabular data files:\n\n${fileList}\n\nUse these file paths (relative or absolute) in qsv commands via the input_file parameter.`,
              },
            ],
          };
        }

        if (name === "qsv_set_working_dir") {
          const directory = args?.directory as string;
          this.filesystemProvider.setWorkingDirectory(directory);

          // Also update executor's working directory so qsv processes
          // write secondary output files (like *.stats.bivariate.csv) to the correct location
          const newWorkingDir = this.filesystemProvider.getWorkingDirectory();
          this.executor.setWorkingDirectory(newWorkingDir);

          return {
            content: [
              {
                type: "text" as const,
                text: `Working directory set to: ${newWorkingDir}\n\nAll relative file paths will now be resolved from this directory.`,
              },
            ],
          };
        }

        if (name === "qsv_get_working_dir") {
          return {
            content: [
              {
                type: "text" as const,
                text: `Current working directory: ${this.filesystemProvider.getWorkingDirectory()}`,
              },
            ],
          };
        }

        // Handle pipeline tool
        if (name === "qsv_pipeline") {
          return await executePipeline(
            args || {},
            this.loader,
            this.filesystemProvider,
          );
        }

        // Handle generic command tool
        if (name === "qsv_command") {
          return await handleGenericCommand(
            args || {},
            this.executor,
            this.loader,
            this.filesystemProvider,
          );
        }

        // Handle config tool
        if (name === "qsv_config") {
          return await handleConfigTool(this.filesystemProvider);
        }

        // Handle search tools
        if (name === "qsv_search_tools") {
          return await handleSearchToolsCall(args || {}, this.loader);
        }

        // Handle data profile tool
        if (name === "qsv_data_profile") {
          return await handleDataProfileCall(args || {}, this.filesystemProvider);
        }

        // Handle common command tools
        if (name.startsWith("qsv_")) {
          return await handleToolCall(
            name,
            args || {},
            this.executor,
            this.loader,
            this.filesystemProvider,
          );
        }

        // Unknown tool
        return {
          content: [
            {
              type: "text" as const,
              text: `Unknown tool: ${name}`,
            },
          ],
          isError: true,
        };
      } catch (error) {
        console.error(`Error executing tool ${name}:`, error);

        return {
          content: [
            {
              type: "text" as const,
              text: `Error: ${error instanceof Error ? error.message : String(error)}`,
            },
          ],
          isError: true,
        };
      }
    });
  }

  /**
   * Register MCP resource handlers
   */
  private registerResourceHandlers(): void {
    // List resources handler - no file resources exposed
    this.server.setRequestHandler(ListResourcesRequestSchema, async () => {
      console.error("Listing resources...");
      console.error("Returning 0 resources (file resources disabled)");

      return {
        resources: [],
      };
    });

    // Read resource handler - minimal implementation since no resources are exposed
    this.server.setRequestHandler(
      ReadResourceRequestSchema,
      async (request) => {
        const { uri } = request.params;

        console.error(`Reading resource: ${uri}`);

        throw new Error(`Resource not found: ${uri}`);
      },
    );
  }

  /**
   * Register MCP prompt handlers
   */
  private registerPromptHandlers(): void {
    // List prompts handler
    this.server.setRequestHandler(ListPromptsRequestSchema, async () => {
      console.error("Listing prompts...");

      const prompts = [
        {
          name: "qsv_census_integration",
          description:
            "Guide for using Census MCP Server with qsv for US demographic enrichment",
        },
      ];

      console.error(`Returning ${prompts.length} prompts`);

      return {
        prompts,
      };
    });

    // Get prompt handler
    this.server.setRequestHandler(GetPromptRequestSchema, async (request) => {
      const { name } = request.params;

      console.error(`Getting prompt: ${name}`);

      // Handle Census integration prompt
      if (name === "qsv_census_integration") {
        return {
          messages: [
            {
              role: "assistant" as const,
              content: {
                type: "text" as const,
                text: `# Census MCP Server Integration Guide

## Overview
When working with US geographic data in qsv, you can enrich your analysis using the Census MCP Server.

## When to Use Census MCP Server
Use Census MCP when your data contains:
- City names (use qsv_geocode suggest first)
- State names or 2-letter codes
- County names or FIPS codes
- ZIP codes or Census tract IDs

## Census MCP Tools

### \`resolve-geography-fips\`
Converts place names to FIPS codes.
- Input: City name, state name, or county name
- Output: FIPS code for Census API queries

### \`fetch-aggregate-data\`
Gets demographic data for a geography.
- Population, age distribution
- Median household income
- Education attainment
- Housing statistics

## Workflow Examples

### Example 1: Enrich City Data with Demographics
1. **qsv_geocode suggest** - Standardize city names
2. **qsv_stats** - Analyze the geocoded data
3. **Census resolve-geography-fips** - Get FIPS codes for cities
4. **Census fetch-aggregate-data** - Get demographics by FIPS
5. The Census API returns JSON, use qsv_json to convert it to CSV.
6. **qsv_joinp** - Join demographics back to original data

### Example 2: Validate Geographic Codes
1. **qsv_frequency** - Check unique values in state/county columns
2. **Census resolve-geography-fips** - Validate codes exist
3. **qsv_search** - Find rows with invalid/unmatched codes

## Column Pattern Recognition
Geographic columns often have names containing:
city, state, county, fips, zip, zipcode, postal, place, municipality,
region, location, geo, tract, cbsa, msa, metro, congressional, district

## Important Notes
- **US Data Only**: Census MCP Server only provides US Census data
- **FIPS Codes**: Many Census queries require FIPS codes - use resolve-geography-fips first
- Alternatively, instead of resolve-geography-fips, the qsv_geocode command outputs FIPS codes.
- **Data Freshness**: Census data is typically 1-2 years behind current year
- **Rate Limits**: Be mindful of Census API rate limits when making bulk requests
- When retrieving data from the Census MCP Server, remember to save them locally as CSV in the qsv working directory so you don't fill up your context window with large JSON responses.
- Refrain from using stdout for qsv commands. Save outputs to files in the qsv working directory and reference those files in subsequent commands.
- Always save output files with descriptive names in the working directory for easy reference in subsequent commands.
`,
              },
            },
          ],
        };
      }

      throw new Error(`Unknown prompt: ${name}`);
    });
  }

  /**
   * Start the server
   */
  async start(): Promise<void> {
    const transport = new StdioServerTransport();

    console.error("Starting QSV MCP Server...");

    await this.server.connect(transport);

    console.error("QSV MCP Server running on stdio");
  }
}

/**
 * Graceful shutdown handler
 */
function setupShutdownHandlers(): void {
  const SHUTDOWN_TIMEOUT_MS = 2000; // 2 seconds max for graceful shutdown
  let shutdownInProgress = false;

  const handleShutdown = (signal: string) => {
    if (shutdownInProgress) {
      console.error(`[Server] Force exit on second ${signal}`);
      process.exit(1);
    }

    shutdownInProgress = true;
    console.error(`[Server] Received ${signal}, shutting down gracefully...`);

    // Set hard timeout to force exit
    const forceExitTimer = setTimeout(() => {
      console.error("[Server] Shutdown timeout exceeded, forcing exit");
      process.exit(1);
    }, SHUTDOWN_TIMEOUT_MS);

    // Prevent the timeout from keeping the process alive
    forceExitTimer.unref();

    // Initiate shutdown
    initiateShutdown();

    // Kill all child processes
    killAllProcesses();

    // Wait briefly for processes to exit, then force exit
    setTimeout(() => {
      const remaining = getActiveProcessCount();
      if (remaining > 0) {
        console.error(
          `[Server] ${remaining} processes still active, forcing exit`,
        );
      } else {
        console.error("[Server] Graceful shutdown complete");
      }
      process.exit(0);
    }, 100);
  };

  // Handle both SIGTERM (from Claude Desktop) and SIGINT (Ctrl+C)
  process.on("SIGTERM", () => handleShutdown("SIGTERM"));
  process.on("SIGINT", () => handleShutdown("SIGINT"));
}

/**
 * Main entry point
 */
async function main(): Promise<void> {
  const server = new QsvMcpServer();

  try {
    await server.initialize();
    await server.start();

    // Setup graceful shutdown handlers
    setupShutdownHandlers();

    console.error(
      "[Server] Ready to accept requests (Press Ctrl+C to shutdown)",
    );
  } catch (error) {
    console.error("Fatal error starting QSV MCP Server:", error);
    process.exit(1);
  }
}

// Run the server
main();
