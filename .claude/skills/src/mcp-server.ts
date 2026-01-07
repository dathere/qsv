#!/usr/bin/env node
/**
 * QSV MCP Server
 *
 * Model Context Protocol server exposing qsv's CSV data-wrangling commands
 * to Claude Desktop and other MCP clients.
 */

import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  ReadResourceRequestSchema,
} from '@modelcontextprotocol/sdk/types.js';

import { SkillLoader } from './loader.js';
import { SkillExecutor } from './executor.js';
import { FilesystemResourceProvider } from './mcp-filesystem.js';
import type { McpToolResult } from './types.js';
import { config } from './config.js';
import {
  COMMON_COMMANDS,
  createToolDefinition,
  createGenericToolDefinition,
  handleToolCall,
  handleGenericCommand,
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
} from './mcp-tools.js';
import {
  createPipelineToolDefinition,
  executePipeline,
} from './mcp-pipeline.js';
import { VERSION } from './version.js';
import { UpdateChecker, getUpdateConfigFromEnv } from './update-checker.js';

/**
 * QSV MCP Server implementation
 */
class QsvMcpServer {
  private server: Server;
  private loader: SkillLoader;
  private executor: SkillExecutor;
  private filesystemProvider: FilesystemResourceProvider;
  private updateChecker: UpdateChecker;

  constructor() {
    this.server = new Server(
      {
        name: 'qsv-server',
        version: VERSION,
      },
      {
        capabilities: {
          tools: {},
          resources: {},
        },
      },
    );

    this.loader = new SkillLoader();
    this.executor = new SkillExecutor(config.qsvBinPath);

    // Initialize filesystem provider with configurable directories
    this.filesystemProvider = new FilesystemResourceProvider({
      workingDirectory: config.workingDir,
      allowedDirectories: config.allowedDirs,
    });

    // Initialize update checker with environment configuration
    this.updateChecker = new UpdateChecker(
      config.qsvBinPath,
      undefined, // Use default skills directory
      getUpdateConfigFromEnv()
    );
  }

  /**
   * Initialize the server and register handlers
   */
  async initialize(): Promise<void> {
    // Load all skills
    console.error('Loading QSV skills...');
    const skills = await this.loader.loadAll();
    console.error(`Loaded ${skills.size} skills`);

    // Check for updates (if enabled)
    await this.checkForUpdates();

    // Register tool handlers
    console.error('About to register tool handlers...');
    this.registerToolHandlers();
    console.error('Tool handlers registered');

    // Register resource handlers
    console.error('About to register resource handlers...');
    this.registerResourceHandlers();
    console.error('Resource handlers registered');

    console.error('QSV MCP Server initialized successfully');
  }

  /**
   * Check for updates and optionally auto-regenerate skills
   */
  private async checkForUpdates(): Promise<void> {
    try {
      // Quick check first (no network calls)
      const quickCheck = await this.updateChecker.quickCheck();

      if (quickCheck.skillsOutdated) {
        console.error('');
        console.error('⚠️  VERSION MISMATCH DETECTED ⚠️');
        console.error(`   qsv binary: ${quickCheck.versions.qsvBinaryVersion}`);
        console.error(`   Skills generated with: ${quickCheck.versions.skillsGeneratedWithVersion}`);
        console.error('');

        // Attempt auto-regeneration if configured
        const autoRegenerated = await this.updateChecker.autoRegenerateSkills();

        if (autoRegenerated) {
          console.error('✅ Skills auto-regenerated successfully');
          console.error('   Please restart the MCP server to load updated skills');
          console.error('');
        } else {
          console.error('ℹ️  To update skills manually, run:');
          console.error('   cargo run --bin qsv-skill-gen --features all_features');
          console.error('   Then restart the MCP server');
          console.error('');
        }
      }

      // Perform full check (with network calls) in the background
      // This won't block server startup
      setImmediate(async () => {
        try {
          const fullCheck = await this.updateChecker.checkForUpdates();

          if (fullCheck.recommendations.length > 0) {
            console.error('');
            console.error('📦 UPDATE CHECK RESULTS:');
            fullCheck.recommendations.forEach(rec => {
              console.error(rec);
            });
            console.error('');
          }
        } catch (error) {
          // Non-critical error - don't block server
          console.error('[UpdateChecker] Background update check failed:', error);
        }
      });
    } catch (error) {
      // Non-critical error - don't block server startup
      console.error('[UpdateChecker] Failed to check for updates:', error);
    }
  }

  /**
   * Register MCP tool handlers
   */
  private registerToolHandlers(): void {
    console.error('[Server] Registering tool handlers...');
    try {
      // List tools handler
      this.server.setRequestHandler(ListToolsRequestSchema, async () => {
        console.error('[Server] Handling tools/list request...');
        const tools = [];

      // Add 20 common command tools
      console.error('[Server] Loading common command tools...');
      for (const command of COMMON_COMMANDS) {
        const skillName = `qsv-${command}`;
        try {
          const skill = await this.loader.load(skillName);

          if (skill) {
            const toolDef = createToolDefinition(skill);
            tools.push(toolDef);
          } else {
            console.error(`Warning: Failed to load skill ${skillName}`);
          }
        } catch (error) {
          console.error(`Error creating tool definition for ${skillName}:`, error);
        }
      }

      // Add generic qsv_command tool
      console.error('[Server] Adding generic command tool...');
      try {
        const genericTool = createGenericToolDefinition(this.loader);
        console.error('[Server] Generic tool created:', JSON.stringify(genericTool).substring(0, 200));
        tools.push(genericTool);
        console.error('[Server] Generic tool added successfully');
      } catch (error) {
        console.error('[Server] Error creating generic tool:', error);
        throw error;
      }

      // Add pipeline tool
      console.error('[Server] Adding pipeline tool...');
      try {
        const pipelineTool = createPipelineToolDefinition();
        console.error('[Server] Pipeline tool created:', JSON.stringify(pipelineTool).substring(0, 200));
        tools.push(pipelineTool);
        console.error('[Server] Pipeline tool added successfully');
      } catch (error) {
        console.error('[Server] Error creating pipeline tool:', error);
        throw error;
      }

      // Add filesystem tools
      console.error('[Server] Adding filesystem tools...');
      tools.push({
        name: 'qsv_list_files',
        description: 'List tabular data files (CSV, TSV, etc.) in a directory. Use this to browse available files before processing them.',
        inputSchema: {
          type: 'object',
          properties: {
            directory: {
              type: 'string',
              description: 'Directory path (absolute or relative to working directory). Defaults to current working directory.',
            },
            recursive: {
              type: 'boolean',
              description: 'Recursively scan subdirectories (default: false)',
            },
          },
        },
      });

      tools.push({
        name: 'qsv_set_working_dir',
        description: 'Set the working directory for relative file paths. All subsequent file operations will be relative to this directory.',
        inputSchema: {
          type: 'object',
          properties: {
            directory: {
              type: 'string',
              description: 'New working directory path',
            },
          },
          required: ['directory'],
        },
      });

      tools.push({
        name: 'qsv_get_working_dir',
        description: 'Get the current working directory',
        inputSchema: {
          type: 'object',
          properties: {},
        },
      });

      console.error(`Registered ${tools.length} tools`);

        return { tools };
      });
      console.error('[Server] Tool handlers registered successfully');
    } catch (error) {
      console.error('[Server] Error registering tool handlers:', error);
      throw error;
    }

    // Call tool handler
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      console.error(`Tool called: ${name}`);

      try {
        // Handle filesystem tools
        if (name === 'qsv_list_files') {
          const directory = args?.directory as string | undefined;
          const recursive = args?.recursive as boolean | undefined;

          const result = await this.filesystemProvider.listFiles(
            directory,
            recursive || false,
          );

          const fileList = result.resources
            .map(r => `- ${r.name} (${r.description})`)
            .join('\n');

          return {
            content: [{
              type: 'text' as const,
              text: `Found ${result.resources.length} tabular data files:\n\n${fileList}\n\nUse these file paths (relative or absolute) in qsv commands via the input_file parameter.`,
            }],
          };
        }

        if (name === 'qsv_set_working_dir') {
          const directory = args?.directory as string;
          this.filesystemProvider.setWorkingDirectory(directory);

          return {
            content: [{
              type: 'text' as const,
              text: `Working directory set to: ${this.filesystemProvider.getWorkingDirectory()}\n\nAll relative file paths will now be resolved from this directory.`,
            }],
          };
        }

        if (name === 'qsv_get_working_dir') {
          return {
            content: [{
              type: 'text' as const,
              text: `Current working directory: ${this.filesystemProvider.getWorkingDirectory()}`,
            }],
          };
        }

        // Handle pipeline tool
        if (name === 'qsv_pipeline') {
          return await executePipeline(args || {}, this.loader, this.filesystemProvider);
        }

        // Handle generic command tool
        if (name === 'qsv_command') {
          return await handleGenericCommand(
            args || {},
            this.executor,
            this.loader,
            this.filesystemProvider,
          );
        }

        // Handle common command tools
        if (name.startsWith('qsv_')) {
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
          content: [{
            type: 'text' as const,
            text: `Unknown tool: ${name}`,
          }],
          isError: true,
        };
      } catch (error) {
        console.error(`Error executing tool ${name}:`, error);

        return {
          content: [{
            type: 'text' as const,
            text: `Error: ${error instanceof Error ? error.message : String(error)}`,
          }],
          isError: true,
        };
      }
    });
  }

  /**
   * Register MCP resource handlers
   */
  private registerResourceHandlers(): void {
    // List resources handler
    this.server.setRequestHandler(ListResourcesRequestSchema, async (request) => {
      console.error('Listing resources...');

      // Only return filesystem files
      const filesystemResult = await this.filesystemProvider.listFiles(undefined, false);

      console.error(`Returning ${filesystemResult.resources.length} file resources`);

      return {
        resources: filesystemResult.resources,
      };
    });

    // Read resource handler
    this.server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
      const { uri } = request.params;

      console.error(`Reading resource: ${uri}`);

      try {
        // Only handle file:/// URIs
        if (!uri.startsWith('file:///')) {
          throw new Error(`Unsupported resource URI: ${uri}`);
        }

        const resource = await this.filesystemProvider.getFileContent(uri);

        if (!resource) {
          throw new Error(`Resource not found: ${uri}`);
        }

        return {
          contents: [{
            uri: resource.uri,
            mimeType: resource.mimeType,
            text: resource.text,
          }],
        };
      } catch (error) {
        console.error(`Error reading resource ${uri}:`, error);

        throw error;
      }
    });
  }

  /**
   * Start the server
   */
  async start(): Promise<void> {
    const transport = new StdioServerTransport();

    console.error('Starting QSV MCP Server...');

    await this.server.connect(transport);

    console.error('QSV MCP Server running on stdio');
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
      console.error('[Server] Shutdown timeout exceeded, forcing exit');
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
        console.error(`[Server] ${remaining} processes still active, forcing exit`);
      } else {
        console.error('[Server] Graceful shutdown complete');
      }
      process.exit(0);
    }, 100);
  };

  // Handle both SIGTERM (from Claude Desktop) and SIGINT (Ctrl+C)
  process.on('SIGTERM', () => handleShutdown('SIGTERM'));
  process.on('SIGINT', () => handleShutdown('SIGINT'));
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

    console.error('[Server] Ready to accept requests (Press Ctrl+C to shutdown)');
  } catch (error) {
    console.error('Fatal error starting QSV MCP Server:', error);
    process.exit(1);
  }
}

// Run the server
main();
