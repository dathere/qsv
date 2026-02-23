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
  RootsListChangedNotificationSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { fileURLToPath } from "node:url";
import { access, stat as fsStat } from "node:fs/promises";

import { SkillLoader } from "./loader.js";
import { SkillExecutor } from "./executor.js";
import { FilesystemResourceProvider } from "./mcp-filesystem.js";
import { config } from "./config.js";
import {
  COMMON_COMMANDS,
  createToolDefinition,
  createGenericToolDefinition,
  handleToolCall,
  handleGenericCommand,
  createConfigTool,
  createSearchToolsTool,
  createToParquetTool,
  createListFilesTool,
  createSetWorkingDirTool,
  createGetWorkingDirTool,
  handleConfigTool,
  handleSearchToolsCall,
  handleToParquetCall,
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
} from "./mcp-tools.js";
import { VERSION } from "./version.js";
import { UpdateChecker, getUpdateConfigFromEnv } from "./update-checker.js";

/**
 * Core tools that are always available (defer_loading: false)
 * These are essential utility tools that enable tool discovery and session management
 */
const CORE_TOOLS = [
  "qsv_search_tools",
  "qsv_config",
  "qsv_set_working_dir",
  "qsv_get_working_dir",
  "qsv_list_files",
  "qsv_command",
  "qsv_to_parquet",
  "qsv_index",
  "qsv_stats",
] as const;

/**
 * Default server instructions sent to MCP clients during initialization.
 * Injected into the system prompt by compatible clients (Claude Desktop, etc.).
 * Focuses on cross-tool workflows and operational constraints.
 * Can be overridden via QSV_MCP_SERVER_INSTRUCTIONS environment variable.
 * See: https://blog.modelcontextprotocol.io/posts/2025-11-03-using-server-instructions/
 */
const DEFAULT_SERVER_INSTRUCTIONS = `qsv is a tabular data-wrangling toolkit. Use qsv_search_tools to discover commands beyond the initially loaded core tools.

WORKFLOW ORDER: For new files: (1) qsv_list_files to discover files, (2) qsv_index for files >5MB, (3) qsv_stats --cardinality --stats-jsonl to create stats cache, (4) then run analysis/transformation commands. The stats cache accelerates: frequency, schema, tojsonl, sqlp, joinp, pivotp, diff, sample. SQL queries on CSV inputs auto-convert to Parquet before execution.

FILE HANDLING: Save outputs to files with descriptive names rather than returning large results to chat. Ensure output files are saved to the qsv working directory. Parquet is ONLY for sqlp/DuckDB; all other qsv commands require CSV/TSV/SSV input. The working directory is automatically synced from the MCP client's root directory (e.g., Claude Cowork's "Work in a folder"), or set manually with qsv_set_working_dir if needed.

TOOL COMPOSITION: qsv_sqlp auto-converts CSV inputs to Parquet, then routes to DuckDB when available for better SQL compatibility and performance; falls back to Polars SQL otherwise. For multi-file SQL queries, convert all files to Parquet first with qsv_to_parquet, then use read_parquet() references in SQL. For custom row-level logic, use qsv_command with command="luau".

MEMORY LIMITS: Commands dedup, sort, reverse, table, transpose load entire files into memory. For files >1GB, prefer extdedup/extsort alternatives via qsv_command. Check column cardinality with qsv_stats before running frequency or pivotp to avoid huge output.`;

/**
 * Resolved server instructions: uses custom instructions from
 * QSV_MCP_SERVER_INSTRUCTIONS env var if set, otherwise falls back to defaults.
 */
const QSV_SERVER_INSTRUCTIONS = config.serverInstructions || DEFAULT_SERVER_INSTRUCTIONS;

/**
 * QSV MCP Server implementation
 */
class QsvMcpServer {
  private server: Server;
  private loader: SkillLoader;
  private executor: SkillExecutor;
  private filesystemProvider: FilesystemResourceProvider;
  private updateChecker: UpdateChecker;
  private loggedToolMode: boolean = false;
  private toolsListedOnce: boolean = false;
  private manuallySetWorkingDir = false;
  private syncingRoots = false;
  private pendingRootsSync = false;
  private rootsSyncRetries = 0;
  private static readonly MAX_ROOTS_SYNC_RETRIES = 3;

  /**
   * Track which tools have been loaded via search (for deferred loading).
   * Tools are added here when discovered via qsv_search_tools
   * and will be included in subsequent ListTools responses.
   *
   * Concurrency note: JavaScript's single-threaded event loop guarantees that
   * Set operations (add/has/iteration) are atomic per tick. No synchronization
   * is needed. A ListTools call may not immediately reflect tools from an
   * in-flight search—this is eventual consistency by design.
   */
  private loadedTools: Set<string> = new Set();

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
        instructions: QSV_SERVER_INSTRUCTIONS,
      },
    );

    this.loader = new SkillLoader();
    this.executor = new SkillExecutor(config.qsvBinPath, config.workingDir);

    // Initialize filesystem provider with configurable directories
    this.filesystemProvider = new FilesystemResourceProvider({
      workingDirectory: config.workingDir,
      allowedDirectories: config.allowedDirs,
      qsvBinPath: config.qsvBinPath,
      pluginMode: config.isPluginMode,
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
      // This won't block server startup; abort after 30 seconds to cancel underlying fetch
      // The async callback's returned promise is intentionally ignored; it has its own try/catch
      setImmediate(async () => {
        const UPDATE_CHECK_TIMEOUT_MS = 30_000;
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), UPDATE_CHECK_TIMEOUT_MS);
        timer.unref();
        try {
          const fullCheck = await this.updateChecker.checkForUpdates(controller.signal);

          if (fullCheck.recommendations.length > 0) {
            console.error("");
            console.error("📦 UPDATE CHECK RESULTS:");
            fullCheck.recommendations.forEach((rec) => {
              console.error(rec);
            });
            console.error("");
          }
        } catch (error: unknown) {
          // Non-critical error - don't block server
          console.error(
            "[UpdateChecker] Background update check failed:",
            error,
          );
        } finally {
          clearTimeout(timer);
        }
      });
    } catch (error: unknown) {
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
        // - true: expose all tools immediately (no deferred loading)
        // - false: expose only 9 core tools (no deferred loading additions)
        // - undefined (default): use deferred loading (9 core tools + search-discovered tools)
        const shouldExposeAll = config.exposeAllTools === true;

        // Log tool mode once per session
        if (!this.loggedToolMode) {
          if (shouldExposeAll) {
            console.error(
              "[Server] ✅ Exposing all tools (QSV_MCP_EXPOSE_ALL_TOOLS=true)",
            );
          } else if (config.exposeAllTools === false) {
            console.error(
              "[Server] Using 9 core tools only (QSV_MCP_EXPOSE_ALL_TOOLS=false)",
            );
          } else {
            console.error(
              "[Server] Using deferred loading (9 core tools + search-discovered)",
            );
          }
          this.loggedToolMode = true;
        }

        // Check if we should expose all tools
        if (shouldExposeAll) {
          // Expose all available skills as individual tools
          // Load all skills (cached after first call)
          if (!this.loader.isAllLoaded()) {
            console.error("[Server] First request - loading all skills...");
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
            } catch (error: unknown) {
              console.error(
                `[Server] ✗ Error creating tool definition for ${skillName}:`,
                error,
              );
            }
          }

          console.error(
            `[Server] ✓ Loaded ${loadedCount} tools (skipped ${skippedCount} unavailable commands)`,
          );
        } else if (config.exposeAllTools === false) {
          // Core tools only mode: only expose the 9 core tools
          // No COMMON_COMMANDS, no search-discovered tools
          console.error(
            `[Server] Core tools only mode - skipping command tools`,
          );
          // Tools will be added below (generic, config, search, filesystem)
        } else {
          // Deferred loading mode (default when exposeAllTools is undefined):
          // 1. Expose common command tools
          // 2. Include tools that have been loaded via qsv_search_tools
          const filteredCommands = availableCommands
            ? COMMON_COMMANDS.filter((cmd) => availableCommands.includes(cmd))
            : COMMON_COMMANDS; // Fallback to all if availableCommands not detected

          // Load only the skills we need (batch loading)
          const skillNames = filteredCommands.map((cmd) => `qsv-${cmd}`);

          // Also include any tools that have been loaded via search
          // These are tools discovered via qsv_search_tools
          const searchedToolNames = Array.from(this.loadedTools)
            .filter((name) => !CORE_TOOLS.includes(name as typeof CORE_TOOLS[number]))
            .map((name) => name.replace("qsv_", "qsv-"));

          // Combine and deduplicate skill names
          const allSkillNames = [...new Set([...skillNames, ...searchedToolNames])];

          console.error(
            `[Server] Loading ${skillNames.length} common + ${searchedToolNames.length} searched skills...`,
          );

          const loadedSkills = await this.loader.loadByNames(allSkillNames);
          console.error(
            `[Server] Loaded ${loadedSkills.size}/${allSkillNames.length} skills`,
          );

          // Log any skills that failed to load (requested but not returned)
          const loadedSkillNamesSet = new Set(loadedSkills.keys());
          const failedSkillNames = allSkillNames.filter(
            (name) => !loadedSkillNamesSet.has(name),
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
            } catch (error: unknown) {
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
          console.error(`[Server] Loaded ${tools.length} command tools (${skillNames.length} common + ${searchedToolNames.length} from search)`);
        }

        // Load skill-based core tools (qsv_index, qsv_stats)
        // In expose-all mode these are already loaded above; in deferred/core-only they need explicit loading
        if (!shouldExposeAll) {
          const skillCoreTools = ["qsv-index", "qsv-stats"];
          const loadedSkillCoreTools = await this.loader.loadByNames(skillCoreTools);
          for (const [skillName, skill] of loadedSkillCoreTools) {
            const commandName = skillName.replace("qsv-", "");
            if (availableCommands && !availableCommands.includes(commandName)) {
              continue;
            }
            try {
              tools.push(createToolDefinition(skill));
            } catch (error: unknown) {
              console.error(`[Server] ✗ Error loading core skill ${skillName}:`, error);
            }
          }
        }

        // Add generic qsv_command tool
        console.error("[Server] Adding generic command tool...");
        try {
          const genericTool = createGenericToolDefinition(this.loader);
          tools.push(genericTool);
        } catch (error: unknown) {
          console.error("[Server] Error creating generic tool:", error);
          throw error;
        }

        // Add config, search, and conversion tools
        console.error("[Server] Adding config, search, and conversion tools...");
        try {
          tools.push(createConfigTool());
          tools.push(createSearchToolsTool());
          tools.push(createToParquetTool());
          console.error("[Server] config, search, and conversion tools added successfully");
        } catch (error: unknown) {
          console.error("[Server] Error creating config/search/conversion tools:", error);
          throw error;
        }

        // Add filesystem tools
        tools.push(createListFilesTool());
        tools.push(createSetWorkingDirTool());
        tools.push(createGetWorkingDirTool());

        console.error(`[Server] Registered ${tools.length} tools`);
        if (!this.toolsListedOnce) {
          console.error(
            `[Server] Tool names: ${tools.map((t) => t.name).join(", ")}`,
          );
          this.toolsListedOnce = true;
        }

        const response = { tools };

        return response;
      });
      console.error("[Server] Tool handlers registered successfully");
    } catch (error: unknown) {
      console.error("[Server] Error registering tool handlers:", error);
      throw error;
    }

    // Call tool handler
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      console.error(`Tool called: ${name}`);

      try {
        // Tool dispatch chain: ordered from most specific to most general.
        // Each handler has a different signature/dependency set, so a handler map
        // would require a uniform interface wrapper with no real benefit.
        // Order: filesystem → generic command → config → search →
        //        to_parquet → skill-based (qsv_*) → unknown tool error.

        // Handle filesystem tools
        if (name === "qsv_list_files") {
          const directory = typeof args?.directory === "string" ? args.directory : undefined;
          const recursive = typeof args?.recursive === "boolean" ? args.recursive : false;

          const result = await this.filesystemProvider.listFiles(
            directory,
            recursive,
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
          if (
            typeof args?.directory !== "string" ||
            args.directory.trim().length === 0
          ) {
            return {
              content: [
                {
                  type: "text" as const,
                  text: "Invalid or missing 'directory' argument for qsv_set_working_dir. Please provide a non-empty string path.",
                },
              ],
              isError: true,
            };
          }

          const directory = args.directory.trim();

          // "auto" is a reserved keyword — not treated as a filesystem path
          if (directory.toLowerCase() === "auto") {
            const previousDir = this.filesystemProvider.getWorkingDirectory();
            let autoSyncEnabled = false;
            let currentDir = previousDir;
            let syncErrorMessage: string | null = null;

            try {
              await this.syncWorkingDirFromRoots();
              // Only mark auto-sync as enabled if the sync completed successfully
              this.manuallySetWorkingDir = false;
              autoSyncEnabled = true;
              currentDir = this.filesystemProvider.getWorkingDirectory();
            } catch (syncError) {
              const message =
                syncError instanceof Error ? syncError.message : String(syncError);
              syncErrorMessage = message;
              console.error(
                `[Roots] Auto-sync from "auto" keyword failed: ${message}`,
              );
            }

            if (autoSyncEnabled) {
              return {
                content: [
                  {
                    type: "text" as const,
                    text: `Auto-sync re-enabled. Working directory is now: ${currentDir}\n\nThe working directory will automatically follow the MCP client's root directory.`,
                  },
                ],
              };
            }

            // Auto-sync could not be enabled; surface a user-visible error.
            return {
              content: [
                {
                  type: "text" as const,
                  text:
                    `Failed to enable automatic root-based working directory sync.\n` +
                    (syncErrorMessage
                      ? `Reason: ${syncErrorMessage}\n`
                      : "") +
                    `The working directory remains: ${previousDir}\n\n` +
                    `You can continue using this directory, or choose a different path. ` +
                    `Pass "auto" again once your MCP client exposes a compatible file:// root to re-enable automatic sync.`,
                },
              ],
              isError: true,
            };
          }

          const newWorkingDir = this.updateWorkingDirectory(directory);
          this.manuallySetWorkingDir = true;

          return {
            content: [
              {
                type: "text" as const,
                text: `Working directory set to: ${newWorkingDir}\n\nAll relative file paths will now be resolved from this directory. Pass "auto" to re-enable automatic root-based sync.`,
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
          return await handleSearchToolsCall(
            args || {},
            this.loader,
            this.loadedTools,
          );
        }

        // Handle CSV to Parquet conversion tool
        if (name === "qsv_to_parquet") {
          return await handleToParquetCall(args || {}, this.filesystemProvider);
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
      } catch (error: unknown) {
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

        throw new Error(`Resource not found: ${uri}. Use qsv_list_files tool to browse available files.`);
      },
    );
  }

  /**
   * Update the working directory for both filesystem provider and executor.
   * Returns the resolved working directory path.
   */
  private updateWorkingDirectory(directory: string): string {
    this.filesystemProvider.setWorkingDirectory(directory);
    const resolved = this.filesystemProvider.getWorkingDirectory();
    this.executor.setWorkingDirectory(resolved);
    return resolved;
  }

  /**
   * Auto-sync working directory from MCP client roots.
   * Used when the client communicates its root directory (e.g., Claude Cowork's "Work in a folder").
   */
  private async syncWorkingDirFromRoots(): Promise<void> {
    if (this.manuallySetWorkingDir) {
      console.error("[Roots] Skipping auto-sync; working directory was manually set via qsv_set_working_dir");
      return;
    }
    if (this.syncingRoots) {
      console.error("[Roots] Sync already in progress, will re-sync when done");
      this.pendingRootsSync = true;
      return;
    }
    this.syncingRoots = true;
    let syncSucceeded = false;
    try {
      const { roots } = await this.server.listRoots();
      const fileRoot = roots.find(r => r.uri.startsWith("file://"));
      if (!fileRoot) {
        if (roots.length > 0) {
          console.error(`[Roots] ${roots.length} root(s) found but none use file:// URI; working directory not auto-set`);
        }
        return;
      }
      const rootPath = fileURLToPath(fileRoot.uri);
      try {
        await access(rootPath);
      } catch {
        console.error(`[Roots] Skipping non-existent root path: ${rootPath}`);
        return;
      }
      try {
        if (!(await fsStat(rootPath)).isDirectory()) {
          console.error(`[Roots] Skipping root path (not a directory): ${rootPath}`);
          return;
        }
      } catch {
        console.error(`[Roots] Cannot stat root path: ${rootPath}`);
        return;
      }
      const resolved = this.updateWorkingDirectory(rootPath);
      console.error(`[Roots] Auto-set working directory to: ${resolved}`);
      if (roots.length > 1) {
        console.error(`[Roots] Note: ${roots.length - 1} additional root(s) ignored; only the first file:// root is used`);
      }
      syncSucceeded = true;
    } catch (error: unknown) {
      // Best-effort feature — suppress expected errors when client doesn't support roots
      if (error instanceof Error) {
        const rawCode = "code" in error ? (error as Record<string, unknown>).code : undefined;
        const errorCode = typeof rawCode === "string" ? Number(rawCode) : rawCode;
        if (errorCode === -32601) {
          // JSON-RPC Method Not Found — client doesn't implement roots
          console.error(`[Roots] Client does not support roots (code -32601)`);
          return;
        }
        const msg = error.message.toLowerCase();
        if (msg.includes("not supported") || msg.includes("method not found")) {
          // Fallback string matching for SDKs that don't set error codes
          console.error(`[Roots] Client does not support roots: ${error.message}`);
          return;
        }
        console.error(`[Roots] Failed to sync working directory: ${error.message}`);
      } else {
        console.error(`[Roots] Failed to sync working directory: ${String(error)}`);
      }
    } finally {
      this.syncingRoots = false;
      if (this.pendingRootsSync) {
        this.pendingRootsSync = false;
        if (this.rootsSyncRetries < QsvMcpServer.MAX_ROOTS_SYNC_RETRIES) {
          this.rootsSyncRetries++;
          console.error(`[Roots] Running pending re-sync (attempt ${this.rootsSyncRetries}/${QsvMcpServer.MAX_ROOTS_SYNC_RETRIES})`);
          await this.syncWorkingDirFromRoots();
          // Each recursive call manages syncSucceeded independently;
          // the retry counter resets only when the *current* call succeeded
        } else {
          console.error(`[Roots] Max re-sync retries (${QsvMcpServer.MAX_ROOTS_SYNC_RETRIES}) reached, skipping pending sync`);
          this.rootsSyncRetries = 0;
        }
      } else if (syncSucceeded) {
        // Reset retry counter only when sync actually succeeded and no pending re-sync is queued
        this.rootsSyncRetries = 0;
      }
    }
  }

  /**
   * Start the server
   */
  async start(): Promise<void> {
    const transport = new StdioServerTransport();

    console.error("Starting QSV MCP Server...");

    await this.server.connect(transport);
    console.error("QSV MCP Server running on stdio");

    // Auto-sync working directory from MCP client roots
    await this.syncWorkingDirFromRoots();

    // Listen for root changes mid-session
    this.server.setNotificationHandler(
      RootsListChangedNotificationSchema,
      async () => {
        await this.syncWorkingDirFromRoots();
      },
    );
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
  } catch (error: unknown) {
    console.error("Fatal error starting QSV MCP Server:", error);
    process.exit(1);
  }
}

// Run the server
main();
