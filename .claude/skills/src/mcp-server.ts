#!/usr/bin/env node
/**
 * QSV MCP Server
 *
 * Model Context Protocol server exposing qsv's tabular data-wrangling commands
 * to Claude Desktop and other MCP clients.
 */

// Low-level Server is intentionally used for advanced dispatch/deferred-loading patterns
// that McpServer's high-level API does not support.
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
import { randomUUID } from "node:crypto";
import { resolve, sep } from "node:path";
import { access, readFile, realpath, writeFile } from "node:fs/promises";
import { homedir } from "node:os";

import { SkillLoader } from "./loader.js";
import { SkillExecutor, runQsvSimple } from "./executor.js";
import { FilesystemResourceProvider } from "./mcp-filesystem.js";
import { config, revalidateQsvBinary } from "./config.js";
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
  createBrowseDirectoryTool,
  createLogTool,
  createSetupToolDefinition,
  handleSetupCall,
  handleConfigTool,
  handleSearchToolsCall,
  handleToParquetCall,
  handleLogCall,
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
  setToolsWorkingDir,
  loadCommandGuidance,
} from "./mcp-tools.js";
// @ts-ignore — moduleResolution:"node" can't resolve exports-mapped subpath; runtime resolves fine
import { getUiCapability, RESOURCE_MIME_TYPE } from "@modelcontextprotocol/ext-apps/server";
import { getDirectoryPickerHtml } from "./ui/directory-picker-html.js";
import { scanDirectory } from "./browse-directory.js";
import { VERSION } from "./version.js";
import { getErrorMessage, errorResult, successResult, completedDirResult } from "./utils.js";
import { UpdateChecker, getUpdateConfigFromEnv } from "./update-checker.js";
import { PipelineManifest } from "./pipeline-manifest.js";
import { WorkingDirManager } from "./working-dir-manager.js";
import { PIPELINE_METADATA, type PipelineMetadata } from "./mcp-tools.js";

/**
 * Directories under $HOME that the browse-directory tool must never enter.
 * Each entry must be a directory (not a file) — the check resolves these as
 * paths and tests whether the target dir equals or is a child of the entry.
 * Promoted to module level so the array is created once, not on every call.
 */
const SENSITIVE_DIRS = [
  ".ssh", ".gnupg", ".gpg", ".pki",
  ".aws", ".azure", ".config/gcloud",
  ".kube",
  ".password-store", ".local/share/keyrings",
  ".docker",
];

/**
 * Core tools that are always available (defer_loading: false)
 * These are essential utility tools that enable tool discovery and session management
 */
const CORE_TOOLS = [
  "qsv_search_tools",
  "qsv_config",
  "qsv_set_working_dir",
  "qsv_get_working_dir",
  "qsv_browse_directory",
  "qsv_list_files",
  "qsv_log",
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

WORKFLOW ORDER for new files:
1. qsv_list_files to discover files
2. qsv_index for files >10MB
3. qsv_stats --cardinality --stats-jsonl to create the initial stats cache (moarstats auto-runs to enrich with ~25 additional columns)
4. (Optional) qsv_moarstats --advanced or --bivariate for deeper analysis
5. Run analysis/transformation commands
The stats cache (specifically .data.jsonl) is what accelerates smart commands: frequency, schema, tojsonl, sqlp, joinp, pivotp, describegpt, moarstats, sample. SQL queries on CSV inputs auto-convert to Parquet before execution.

FILE HANDLING: Save outputs to files with descriptive names rather than returning large results to chat. Ensure output files are saved to the qsv working directory. Parquet is ONLY for sqlp/DuckDB; all other qsv commands require CSV/TSV/SSV input. Spreadsheets (.xls, .xlsx, .xlsm, .xlsb, .ods) must be converted to CSV first using the excel command before other qsv commands can process them. The working directory is automatically synced from the MCP client's root directory when available. If the auto-synced directory is incorrect or no root is provided, call qsv_set_working_dir to set it manually. In Claude Cowork, verify the working directory matches the "Work in a folder" path by calling qsv_get_working_dir, and correct it with qsv_set_working_dir if needed.

TOOL COMPOSITION: qsv_sqlp auto-converts CSV inputs to Parquet, then routes to DuckDB when available for better SQL compatibility and performance; falls back to Polars SQL otherwise. Before writing SQL, read the qsv_stats output to understand column types, cardinality, null counts, and value ranges; optionally run qsv_frequency for categorical value distributions. For multi-file SQL queries, convert all files to Parquet first with qsv_to_parquet, then use read_parquet() references in SQL. For custom row-level logic, use qsv_command with command="luau". In Claude Cowork, DuckDB must run on the host machine, not the Linux container — ensure the DuckDB binary path points to the host installation.

CACHE AWARENESS: Before running commands, check for existing caches to save time and tokens.
- Stats cache (<FILESTEM>.stats.csv, e.g. data.stats.csv for input data.csv): read this compact CSV first — use cardinality to pick optimal join order (smaller on right in joinp), skip sorting if already sorted, skip dedup if a key column is all-unique, and pass column types to sqlp/joinp instead of re-sniffing.
- Frequency cache (<FILESTEM>.freq.csv, e.g. data.freq.csv if saved via --output): prefer reading this output CSV directly — it's far more token-efficient than the JSONL sidecar. The MCP server auto-manages this when running qsv_frequency.
- .data.jsonl variants (<FILESTEM>.stats.csv.data.jsonl and <FILESTEM>.freq.csv.data.jsonl): optimized for qsv's internal smart command reuse, less token-efficient than the CSV caches.

MEMORY LIMITS: Commands dedup, sort, reverse, table, transpose, pragmastat load entire files into memory. For files >1GB, prefer extdedup/extsort alternatives via qsv_command. Check column cardinality with qsv_stats before running frequency or pivotp to avoid huge output.

OPERATION TIMEOUT: qsv operations can take significant time, especially on larger files. The MCP server's default operation timeout is 10 minutes (configurable via QSV_MCP_OPERATION_TIMEOUT_MS, max 30 minutes). Do NOT use a shorter client-side timeout — allow operations to run to completion or until the server's configured timeout. Check the current timeout setting with qsv_config. CONCURRENT OPERATIONS: Parallel tool calls are automatically queued. For optimal throughput in Claude Cowork, execute pipeline steps sequentially (index → stats → analysis).

REPRODUCIBILITY LOG: Use qsv_log to create a verifiable audit trail.${config.isPluginMode ? ` User prompts are logged automatically — do NOT log them manually.` : ` Include a brief summary of the user's request in your first agent_reasoning entry for reproducibility.`} Log key decisions (entry_type: "agent_reasoning"), actions taken (entry_type: "agent_action"), and outcomes (entry_type: "result_summary"). The log file (qsvmcp.log) records both automatic tool invocations (s-/e- prefixed) and your explicit entries (u- prefixed). Keep entries concise but sufficient for a third party to reproduce your workflow. Avoid excessive logging — for simple interactions, a result_summary alone is enough. Reserve agent_reasoning for non-obvious decisions.${config.isPluginMode ? `

COWORK/PLUGIN-MODE NOTES:
- PATH ARCHITECTURE: qsv runs on the HOST machine, not inside the Linux container. File paths must be valid on the host. Always verify with qsv_get_working_dir.
- SEQUENTIAL OPERATIONS: Prefer sequential over parallel qsv calls to avoid queuing delays: index → stats → analysis.
- LARGE FILES (>5GB): Let qsv_frequency run to completion (default server timeout is 10 min, configurable via QSV_MCP_OPERATION_TIMEOUT_MS). Only fall back to qsv_sqlp with GROUP BY if the server timeout is exceeded. Use extsort/extdedup via qsv_command instead of sort/dedup.
- CONTEXT WINDOW: Save outputs to files rather than returning to chat. Use qsv_slice or qsv_sqlp with LIMIT to inspect subsets.` : ""}`;

/**
 * Resolved server instructions: uses custom instructions from
 * QSV_MCP_SERVER_INSTRUCTIONS env var if set, otherwise falls back to defaults.
 */
const QSV_SERVER_INSTRUCTIONS = config.serverInstructions || DEFAULT_SERVER_INSTRUCTIONS;

/**
 * QSV MCP Server implementation
 */
/**
 * Tool handler signature for the dispatch map.
 * Handlers receive tool arguments and return an MCP-compatible result.
 */
type ToolHandler = (args: Record<string, unknown>) => Promise<{
  content: Array<{ type: string; text: string }>;
  isError?: boolean;
  structuredContent?: unknown;
}>;

class QsvMcpServer {
  private server: Server;
  private loader: SkillLoader;
  private executor: SkillExecutor;
  private filesystemProvider: FilesystemResourceProvider;
  private updateChecker: UpdateChecker;
  private loggedToolMode: boolean = false;
  private toolsListedOnce: boolean = false;
  /** Raw client extensions captured before Zod strips them from capabilities */
  private rawClientExtensions: Record<string, unknown> | undefined;
  /** Manages working directory state: confirmation, elicitation, roots sync. */
  private workingDirManager!: WorkingDirManager;

  /** Pipeline manifest for reproducibility — records tool steps with BLAKE3 hashes. */
  private pipelineManifest: PipelineManifest | null = null;

  /**
   * Dispatch map for tool handlers, keyed by tool name.
   * Initialized in the constructor after all dependencies are set.
   * The `qsv_*` wildcard for skill-based tools is handled as a fallback
   * after the map lookup in registerToolHandlers().
   */
  private toolDispatchMap: Record<string, ToolHandler>;

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

    // Initialize working directory manager (elicitation, roots sync)
    this.workingDirManager = new WorkingDirManager(
      this.server,
      this.filesystemProvider,
      (directory: string) => this.updateWorkingDirectory(directory),
    );

    // Initialize update checker with environment configuration
    this.updateChecker = new UpdateChecker(
      config.qsvBinPath,
      undefined, // Use default skills directory
      getUpdateConfigFromEnv(),
    );

    // Initialize tool dispatch map — closures capture `this`, so all
    // property references resolve to their current values at call time.
    this.toolDispatchMap = {
      qsv_setup: (args) => this.handleSetup(args),
      qsv_log: (args) => handleLogCall(args, this.filesystemProvider.getWorkingDirectory()),
      qsv_list_files: (args) => this.handleListFiles(args),
      qsv_set_working_dir: (args) => this.handleSetWorkingDir(args),
      qsv_get_working_dir: () =>
        Promise.resolve(successResult(`Current working directory: ${this.filesystemProvider.getWorkingDirectory()}`)),
      qsv_browse_directory: (args) => this.handleBrowseDirectoryDispatch(args),
      qsv_command: (args) =>
        handleGenericCommand(args, this.executor, this.loader, this.filesystemProvider, this.server),
      qsv_config: () => handleConfigTool(this.filesystemProvider),
      qsv_search_tools: (args) =>
        handleSearchToolsCall(args, this.loader, this.loadedTools),
      qsv_to_parquet: (args) =>
        handleToParquetCall(args, this.filesystemProvider),
    };
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

    // Initialize pipeline manifest for reproducibility
    const qsvVersion = config.qsvValidation?.version ?? "unknown";
    this.pipelineManifest = new PipelineManifest(
      randomUUID(),
      this.filesystemProvider.getWorkingDirectory(),
      qsvVersion,
      VERSION,
      config.qsvBinPath,
    );
    console.error("[Init] ✓ Pipeline manifest initialized");

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
      console.error(`   Polars: ${validation.polarsVersion}`);
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

        // When qsv binary is not found/valid, only expose the setup tool + config diagnostic
        if (!config.qsvValidation.valid) {
          console.error("[Server] qsv binary not valid — exposing qsv_setup + qsv_config only");
          return {
            tools: [
              createSetupToolDefinition(),
              createConfigTool(),
            ],
          };
        }

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
        // Only expose the browse directory tool when MCP Apps are enabled
        if (config.enableMcpApps && this.clientSupportsApps()) {
          tools.push(createBrowseDirectoryTool());
        }

        // Add logging tool
        tools.push(createLogTool());

        console.error(`[Server] Registered ${tools.length} tools`);
        if (!this.toolsListedOnce) {
          console.error(
            `[Server] Tool names: ${tools.map((t) => t.name).join(", ")}`,
          );
          this.toolsListedOnce = true;
        }

        return { tools };
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

      // MCP audit logging: generate invocation ID and extract optional _reason.
      // _reason is an optional meta-parameter declared in tool input schemas
      // that MCP clients may pass to provide human-readable context for the
      // invocation. It is stripped from toolArgs before dispatch and only used
      // in the audit log. If absent, the tool name is used as the reason.
      const startTime = Date.now();
      const invocationId = randomUUID();

      // Extract and remove _reason before forwarding args
      const reason = typeof args?._reason === "string" ? args._reason : name;
      const toolArgs = args
        ? Object.fromEntries(
            Object.entries(args).filter(([k]) => k !== "_reason"),
          )
        : args;

      // Build start message: reason + tool arguments summary (truncated to avoid
      // OS argument-length limits and oversized log entries for large payloads)
      const MAX_ARGS_LOG_LEN = 1024;
      const rawArgsSummary = toolArgs ? JSON.stringify(toolArgs) : "";
      const argsSummary = rawArgsSummary.length > MAX_ARGS_LOG_LEN
        ? rawArgsSummary.slice(0, MAX_ARGS_LOG_LEN) + "…[truncated]"
        : rawArgsSummary;
      const startMsg = reason + (argsSummary ? ` | args: ${argsSummary}` : "");

      // Log start (fire-and-forget — logging must not break tool calls)
      // Start entries only at "info" level; "error" level only logs failures
      // Safe to capture once: config is immutable after initialization
      const auditLogEnabled = config.mcpLogLevel !== "off";
      // Skip automatic audit logging for qsv_log to avoid recursive noise
      const skipAuditLog = name === "qsv_log";
      if ((config.mcpLogLevel === "info" || config.mcpLogLevel === "debug") && !skipAuditLog) {
        runQsvSimple(config.qsvBinPath, ["log", name, `s-${invocationId}`, startMsg], {
          timeoutMs: 5_000,
          cwd: this.filesystemProvider.getWorkingDirectory(),
        }).catch(() => {});
      }

      // First-tool-use working directory prompt: if the working directory has not
      // been confirmed (via roots sync, manual set, or elicitation), prompt the
      // user to select one before the first data-processing tool call.
      if (!QsvMcpServer.ELICITATION_EXEMPT_TOOLS.has(name)) {
        await this.workingDirManager.ensureConfirmedForTool();
      }

      // Dispatch tool and log result
      const dispatch = async () => {
        // Look up handler in the dispatch map first, then fall back to
        // skill-based tools (qsv_*) and finally unknown tool error.
        const handler = this.toolDispatchMap[name];
        if (handler) {
          return await handler(toolArgs || {});
        }

        // Skill-based tools (qsv_index, qsv_stats, qsv_select, etc.)
        if (name.startsWith("qsv_")) {
          return await handleToolCall(
            name,
            toolArgs || {},
            this.executor,
            this.loader,
            this.filesystemProvider,
            this.server,
          );
        }

        // Unknown tool
        return errorResult(`Unknown tool: ${name}`);
      };

      try {
        const result = await dispatch();

        // Log end with elapsed time
        const elapsedMs = Date.now() - startTime;
        const elapsedSecs = (elapsedMs / 1000).toFixed(2);
        const isError = "isError" in result && result.isError === true;
        if (auditLogEnabled && !skipAuditLog && (config.mcpLogLevel === "info" || config.mcpLogLevel === "debug" || isError)) {
          const endMsg = isError
            ? `error(${elapsedSecs}s): tool returned error`
            : `ok(${elapsedSecs}s)`;
          runQsvSimple(config.qsvBinPath, ["log", name, `e-${invocationId}`, endMsg], {
            timeoutMs: 5_000,
            cwd: this.filesystemProvider.getWorkingDirectory(),
          }).catch(() => {});
        }

        // Record pipeline step for reproducibility manifest
        if (this.pipelineManifest && name.startsWith("qsv_")) {
          const meta = (result as Record<string | symbol, unknown>)[PIPELINE_METADATA] as PipelineMetadata | undefined;
          const rawErr = isError ? ((result as { content?: Array<{ text?: string }> }).content?.[0]?.text ?? "") : undefined;
          const errorMessage = rawErr !== undefined
            ? (rawErr.length > MAX_ARGS_LOG_LEN ? rawErr.slice(0, MAX_ARGS_LOG_LEN) + "…[truncated]" : rawErr)
            : undefined;
          // For qsv_command, derive the actual tool name from args so
          // classifyKind/isDeterministic work correctly on the underlying command.
          const pipelineToolName = name === "qsv_command" && typeof (toolArgs ?? {}).command === "string"
            ? `qsv_${(toolArgs as { command: string }).command}`
            : name;
          try {
            await this.pipelineManifest.recordStep({
              invocationId,
              toolName: pipelineToolName,
              toolArgs: toolArgs ?? {},
              reason: reason !== name ? reason : null,
              commandLine: meta?.commandLine ?? null,
              inputFile: meta?.inputFile ?? null,
              outputFile: meta?.outputFile ?? null,
              additionalInputFiles: meta?.additionalInputFiles ?? [],
              durationMs: meta?.durationMs ?? elapsedMs,
              success: meta?.success ?? !isError,
              errorMessage,
            });
          } catch (err) {
            console.error(`[PipelineManifest] Failed to record step: ${getErrorMessage(err)}`);
          }
        }

        return result;
      } catch (error: unknown) {
        // Log error end with elapsed time (always log errors unless fully off)
        if (auditLogEnabled && !skipAuditLog) {
          const elapsedSecs = ((Date.now() - startTime) / 1000).toFixed(2);
          const errMsg = getErrorMessage(error);
          const truncatedErr = errMsg.length > MAX_ARGS_LOG_LEN
            ? errMsg.slice(0, MAX_ARGS_LOG_LEN) + "…[truncated]"
            : errMsg;
          runQsvSimple(config.qsvBinPath, ["log", name, `e-${invocationId}`, `error(${elapsedSecs}s): ${truncatedErr}`], {
            timeoutMs: 5_000,
            cwd: this.filesystemProvider.getWorkingDirectory(),
          }).catch(() => {});
        }

        console.error(`Error executing tool ${name}:`, error);
        return errorResult(`Error: ${getErrorMessage(error)}`);
      }
    });
  }

  /**
   * Register MCP resource handlers
   */
  private registerResourceHandlers(): void {
    // List resources handler — expose the directory picker App resource when enabled
    this.server.setRequestHandler(ListResourcesRequestSchema, async () => {
      const resources = config.enableMcpApps
        ? [
            {
              uri: "ui://qsv/directory-picker",
              name: "Directory Picker",
              description: "Interactive directory browser for selecting the qsv working directory",
              mimeType: RESOURCE_MIME_TYPE,
            },
          ]
        : [];

      return { resources };
    });

    // Read resource handler — serves App HTML for ui:// resources
    this.server.setRequestHandler(
      ReadResourceRequestSchema,
      async (request) => {
        const { uri } = request.params;

        // Always serve the directory picker resource if requested — Claude Desktop
        // may cache the URI from a previous session even when it's no longer listed.
        if (uri === "ui://qsv/directory-picker") {
          return {
            contents: [
              {
                uri,
                mimeType: RESOURCE_MIME_TYPE,
                text: getDirectoryPickerHtml(process.platform === "win32" ? (process.env.SYSTEMDRIVE || "C:") + "\\" : "/"),
                // No CSP needed — the App HTML uses an inline SDK shim
                // with no external CDN dependencies.
              },
            ],
          };
        }

        throw new Error(`Resource not found: ${uri}. Use qsv_list_files tool to browse available files.`);
      },
    );
  }

  /**
   * Check if the connected MCP client supports MCP Apps (interactive UI rendering).
   * Returns true when the client advertises the `io.modelcontextprotocol/ui` extension
   * with the required MIME type.
   */
  private clientSupportsApps(): boolean {
    // Build a capabilities-like object with the raw extensions we captured
    // before the SDK's Zod parsing stripped them.
    const capsWithExtensions = this.rawClientExtensions
      ? { extensions: this.rawClientExtensions }
      : null;
    const uiCap = getUiCapability(capsWithExtensions);
    return Boolean(uiCap?.mimeTypes?.includes(RESOURCE_MIME_TYPE));
  }

  // ── Extracted tool handlers for dispatch map ──────────────────────────────

  /**
   * Handle qsv_setup tool call.
   * Installs qsv, revalidates the binary, and notifies the client to re-fetch tools.
   */
  private async handleSetup(
    args: Record<string, unknown>,
  ): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
    const setupResult = await handleSetupCall(args, () => {
      const revalidation = revalidateQsvBinary();
      if (revalidation.validation.valid) {
        // Reconstruct executor and update filesystem provider with new binary path.
        // Use the current working directory rather than the possibly stale config.workingDir.
        this.executor = new SkillExecutor(
          revalidation.path,
          this.filesystemProvider.getWorkingDirectory(),
        );
        this.filesystemProvider.updateQsvBinPath(revalidation.path);
      }
      return revalidation;
    });
    // Send tools/list_changed notification so the client re-fetches the full tool list
    if (setupResult.revalidated) {
      this.server.sendToolListChanged().catch((err: unknown) =>
        console.error("[Server] Failed to send tools/list_changed:", err),
      );
    }
    return setupResult.response;
  }

  /**
   * Handle qsv_list_files tool call.
   * Lists tabular data files in the specified or current working directory.
   */
  private async handleListFiles(
    args: Record<string, unknown>,
  ): Promise<{ content: Array<{ type: string; text: string }> }> {
    const directory = typeof args?.directory === "string" ? args.directory : undefined;
    const recursive = typeof args?.recursive === "boolean" ? args.recursive : false;

    const result = await this.filesystemProvider.listFiles(directory, recursive);

    const fileList = result.resources
      .map((r) => `- ${r.name} (${r.description})`)
      .join("\n");

    return successResult(
      `Found ${result.resources.length} tabular data files:\n\n${fileList}\n\nUse these file paths (relative or absolute) in qsv commands via the input_file parameter.`,
    );
  }

  /**
   * Handle qsv_set_working_dir tool call.
   * Supports: no-arg (interactive picker / elicitation / suggestions),
   * "auto" keyword (re-enable root-based sync), and explicit directory paths.
   */
  private async handleSetWorkingDir(
    args: Record<string, unknown>,
  ): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean; structuredContent?: unknown }> {
    if (
      typeof args?.directory !== "string" ||
      args.directory.trim().length === 0
    ) {
      // No directory provided — try App UI, elicitation, or text suggestions

      // 1. If MCP Apps are enabled and client supports them, return structuredContent
      if (config.enableMcpApps && this.clientSupportsApps()) {
        const candidates = await this.workingDirManager.discoverDirectories();
        const currentDir = this.filesystemProvider.getWorkingDirectory();
        // structuredContent is an MCP Apps extension not yet in the SDK's
        // CallToolResult type. Cast to satisfy the compiler while still
        // delivering the payload to App-capable clients.
        return {
          content: [{ type: "text" as const, text: "Opening interactive directory picker..." }],
          structuredContent: {
            currentPath: currentDir,
            knownDirs: candidates,
            homeDir: homedir(),
          },
        } as { content: Array<{ type: "text"; text: string }> };
      }

      // 2. Try interactive elicitation form
      const elicitResult = await this.workingDirManager.elicitWorkingDirectory();
      if (elicitResult.directory) {
        const newDir = this.updateWorkingDirectory(elicitResult.directory);
        this.workingDirManager.markManuallySet();
        return successResult(`Working directory set to: ${newDir}\n\nAll relative file paths will now be resolved from this directory. Pass "auto" to re-enable automatic root-based sync.`);
      }

      // 3. Return suggestions as success so the agent can pick a directory
      return successResult(elicitResult.fallback || "No directory selected. Please call qsv_set_working_dir with an explicit directory path.");
    }

    const directory = args.directory.trim();

    // "auto" is a reserved keyword — not treated as a filesystem path
    if (directory.toLowerCase() === "auto") {
      const previousDir = this.filesystemProvider.getWorkingDirectory();
      let autoSyncEnabled = false;
      let currentDir = previousDir;
      let syncErrorMessage: string | null = null;

      try {
        // Clear the manual flag so syncWorkingDirFromRoots() doesn't skip
        this.workingDirManager.clearManuallySet();
        await this.workingDirManager.syncWorkingDirFromRoots();
        // Only mark auto-sync as enabled if the sync completed successfully
        this.workingDirManager.confirmDirectory();
        autoSyncEnabled = true;
        currentDir = this.filesystemProvider.getWorkingDirectory();
      } catch (syncError: unknown) {
        const message = getErrorMessage(syncError);
        syncErrorMessage = message;
        // Restore the manual flag so roots notifications don't change the
        // directory behind the user's back after a failed "auto" attempt.
        this.workingDirManager.markManuallySet();
        console.error(
          `[Roots] Auto-sync from "auto" keyword failed: ${message}`,
        );
      }

      if (autoSyncEnabled) {
        const autoMessage = `Auto-sync re-enabled. Working directory is now: ${currentDir}\n\nThe working directory will automatically follow the MCP client's root directory.`;

        if (config.enableMcpApps && this.clientSupportsApps()) {
          return completedDirResult(autoMessage, currentDir);
        }
        return successResult(autoMessage);
      }

      // Auto-sync could not be enabled; surface a user-visible error.
      return errorResult(
        `Failed to enable automatic root-based working directory sync.\n` +
        (syncErrorMessage
          ? `Reason: ${syncErrorMessage}\n`
          : "") +
        `The working directory remains: ${previousDir}\n\n` +
        `You can continue using this directory, or choose a different path. ` +
        `Pass "auto" again once your MCP client exposes a compatible file:// root to re-enable automatic sync.`,
      );
    }

    const newWorkingDir = this.updateWorkingDirectory(directory);
    this.workingDirManager.markManuallySet();

    const message = `Working directory set to: ${newWorkingDir}\n\nAll relative file paths will now be resolved from this directory. Pass "auto" to re-enable automatic root-based sync.`;

    // When Apps are enabled the client always renders the App UI (due to
    // the tool-level _meta annotation).  Return completedDirResult so the
    // picker shows a minimal checkmark confirmation instead of a broken
    // "Failed to load directory" state.
    if (config.enableMcpApps && this.clientSupportsApps()) {
      return completedDirResult(message, newWorkingDir);
    }
    return successResult(message);
  }

  /**
   * Handle qsv_browse_directory dispatch.
   * Validates MCP Apps support before delegating to the actual handler.
   */
  private async handleBrowseDirectoryDispatch(
    args: Record<string, unknown>,
  ): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
    if (!(config.enableMcpApps && this.clientSupportsApps())) {
      return errorResult(
        "The qsv_browse_directory tool is only available when MCP Apps are enabled and supported by the client.",
      );
    }
    return await this.handleBrowseDirectory(args);
  }

  /**
   * Handle qsv_browse_directory tool call.
   * Returns subdirectories with tabular file counts, breadcrumbs, and parent path.
   * Used by the directory picker App to navigate the filesystem.
   */
  private async handleBrowseDirectory(
    args: Record<string, unknown>,
  ): Promise<{ content: Array<{ type: "text"; text: string }>; isError?: boolean }> {
    const rawDir =
      typeof args.directory === "string" && args.directory.trim().length > 0
        ? args.directory.trim()
        : this.filesystemProvider.getWorkingDirectory();

    try {
      // Resolve to absolute path without restricting to allowed directories.
      // The browse tool is read-only (lists subdirs only), hidden from the LLM,
      // and its purpose is to let users pick ANY directory as working dir.
      // Resolve relative paths against the qsv working directory (not process.cwd())
      const baseDir = this.filesystemProvider.getWorkingDirectory();
      const absPath = resolve(baseDir, rawDir);
      let targetDir: string;
      try {
        targetDir = await realpath(absPath);
      } catch {
        targetDir = absPath;
      }

      // Defense-in-depth: block browsing sensitive directories.
      // Canonicalize home via realpath to match targetDir (e.g. macOS
      // /Users/… vs /System/Volumes/Data/Users/…).
      const home = homedir();
      let canonHome: string;
      try {
        canonHome = await realpath(home);
      } catch {
        canonHome = home;
      }
      const caseInsensitive = process.platform === "darwin" || process.platform === "win32";
      const normalize = (p: string) => caseInsensitive ? p.toLowerCase() : p;
      const target = normalize(targetDir);
      for (const sensitive of SENSITIVE_DIRS) {
        const blocked = normalize(resolve(canonHome, sensitive));
        if (target === blocked || target.startsWith(blocked + sep)) {
          return errorResult(`Access to "${sensitive}" is not allowed for security reasons.`);
        }
      }

      const result = await scanDirectory(targetDir);
      return successResult(JSON.stringify(result));
    } catch (err) {
      return errorResult(getErrorMessage(err));
    }
  }

  /** Expose the pipeline manifest for shutdown finalization. */
  getPipelineManifest(): PipelineManifest | null {
    return this.pipelineManifest;
  }

  /**
   * Update the working directory for both filesystem provider and executor.
   * Returns the resolved working directory path.
   */
  private updateWorkingDirectory(directory: string): string {
    this.filesystemProvider.setWorkingDirectory(directory);
    const resolved = this.filesystemProvider.getWorkingDirectory();
    this.executor.setWorkingDirectory(resolved);
    setToolsWorkingDir(resolved);
    this.pipelineManifest?.updateWorkingDir(resolved);
    return resolved;
  }

  /**
   * Tools that should NOT trigger the first-use working directory elicitation.
   * These are configuration, discovery, and logging tools.
   */
  private static readonly ELICITATION_EXEMPT_TOOLS = new Set([
    "qsv_config",
    "qsv_setup",
    "qsv_log",
    "qsv_search_tools",
    "qsv_set_working_dir",
    "qsv_get_working_dir",
    "qsv_browse_directory",
  ]);

  /**
   * Deploy cowork-CLAUDE.md workflow guide to the working directory (non-fatal).
   * Replaces the SessionStart hook (cowork-setup.cjs) which doesn't fire in Cowork.
   * Only runs in plugin mode. Silently skips if the file already exists or on any error.
   */
  private async deployWorkflowGuide(): Promise<void> {
    try {
      const workingDir = this.executor.getWorkingDirectory();

      // Resolve the template path relative to this file's directory
      // In the built output, dist/mcp-server.js → look for cowork-CLAUDE.md in parent
      const thisDir = new URL(".", import.meta.url);
      const pluginRoot = resolve(fileURLToPath(thisDir), "..");
      const templatePath = resolve(pluginRoot, "cowork-CLAUDE.md");

      // Check template exists
      try {
        await access(templatePath);
      } catch {
        console.error("[Deploy] cowork-CLAUDE.md template not found, skipping workflow guide deployment");
        return;
      }

      // Guard: don't deploy into the plugin's own directory
      const resolvedWorking = await realpath(workingDir);
      const resolvedPlugin = await realpath(pluginRoot);
      if (resolvedWorking === resolvedPlugin || resolvedWorking.startsWith(resolvedPlugin + sep)) {
        console.error("[Deploy] Working directory is inside plugin root, skipping");
        return;
      }

      const template = await readFile(templatePath, "utf-8");

      // Deploy both files: CLAUDE.md (for Claude Code CLI) and .cowork-instructions.md (for Cowork per-folder context)
      const deployIfMissing = async (filename: string, label: string): Promise<void> => {
        const target = resolve(workingDir, filename);
        try {
          await access(target);
          console.error(`[Deploy] ${label} already exists at ${target}, not overwriting`);
        } catch {
          try {
            await writeFile(target, template, "utf-8");
            console.error(`[Deploy] Deployed ${label} to ${target}`);
          } catch (writeErr: unknown) {
            console.error(`[Deploy] Could not write ${label} to ${workingDir}: ${getErrorMessage(writeErr)}`);
          }
        }
      };

      await deployIfMissing("CLAUDE.md", "CLAUDE.md");
      await deployIfMissing(".cowork-instructions.md", ".cowork-instructions.md");
    } catch (error: unknown) {
      // Non-fatal — never block server startup
      console.error(`[Deploy] Workflow guide deployment failed (non-fatal): ${getErrorMessage(error)}`);
    }
  }

  /**
   * Start the server
   */
  async start(): Promise<void> {
    const transport = new StdioServerTransport();

    // Only intercept the transport when MCP Apps are enabled — the interception
    // wraps the SDK's onmessage handler and may interfere with elicitation and
    // other SDK-managed features in some clients (e.g., MCPB).
    if (config.enableMcpApps) {
      // Capture raw client extensions before the MCP SDK's Zod parsing strips them.
      // The SDK schema doesn't include `extensions` (no .passthrough()), so
      // getClientCapabilities() loses the `io.modelcontextprotocol/ui` field
      // that Claude Desktop sends for MCP Apps support.
      Object.defineProperty(transport, "onmessage", {
        set: (handler: ((msg: unknown) => void) | undefined) => {
          const wrapper = (msg: unknown) => {
            const m = msg as { method?: string; params?: { capabilities?: { extensions?: Record<string, unknown> } } };
            if (m?.method === "initialize" && m.params?.capabilities?.extensions) {
              this.rawClientExtensions = m.params.capabilities.extensions;
            }
            handler?.(msg);
          };
          (transport as unknown as { _onmessage?: typeof wrapper })._onmessage = wrapper;
        },
        get: () => (transport as unknown as { _onmessage?: (msg: unknown) => void })._onmessage,
        configurable: true,
      });
    }

    // Load command guidance from YAML before tools are listed
    await loadCommandGuidance();

    console.error("Starting QSV MCP Server...");

    await this.server.connect(transport);
    console.error("QSV MCP Server running on stdio");

    // Auto-sync working directory from MCP client roots
    await this.workingDirManager.syncWorkingDirFromRoots();

    // In plugin mode, deploy workflow guide to working directory (fire-and-forget).
    // This replaces the SessionStart hook (cowork-setup.cjs) which doesn't fire in Cowork.
    // Non-blocking: server is ready to accept requests while files are being written.
    if (config.isPluginMode) {
      this.deployWorkflowGuide().catch((err: unknown) =>
        console.error(`[Deploy] Workflow guide deployment failed: ${getErrorMessage(err)}`),
      );
    }

    // Listen for root changes mid-session
    this.server.setNotificationHandler(
      RootsListChangedNotificationSchema,
      async () => {
        await this.workingDirManager.syncWorkingDirFromRoots();
      },
    );
  }
}

/**
 * Graceful shutdown handler
 */
function setupShutdownHandlers(manifest: PipelineManifest | null): void {
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

    // Initiate shutdown first — prevents new tool calls from being accepted,
    // so no new recordStep() calls will start after this point.
    initiateShutdown();

    // Finalize pipeline manifest (sync — hashing was done incrementally,
    // so this only writes two small files)
    if (manifest) {
      const result = manifest.finalize();
      if (result) {
        console.error(`[Server] Pipeline manifest written: ${result.jsonPath}`);
      }
    }

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

    // Setup graceful shutdown handlers (pass manifest for finalization)
    setupShutdownHandlers(server.getPipelineManifest());

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
