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
  ListPromptsRequestSchema,
  GetPromptRequestSchema,
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
          prompts: {},
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

    // Register prompt handlers
    console.error('About to register prompt handlers...');
    this.registerPromptHandlers();
    console.error('Prompt handlers registered');

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
          console.error('   qsv --update-mcp-skills');
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
   * Register prompt handlers
   */
  private registerPromptHandlers(): void {
    // List prompts handler
    this.server.setRequestHandler(ListPromptsRequestSchema, async () => {
      return {
        prompts: [
          {
            name: 'welcome',
            description: 'Welcome message and quick start guide for qsv',
            arguments: [],
          },
          {
            name: 'examples',
            description: 'Show common qsv usage examples',
            arguments: [],
          },
        ],
      };
    });

    // Get prompt handler
    this.server.setRequestHandler(GetPromptRequestSchema, async (request) => {
      const { name } = request.params;

      if (name === 'welcome') {
        return {
          messages: [
            {
              role: 'user',
              content: {
                type: 'text',
                text: 'Hello! Tell me about qsv and how to get started.',
              },
            },
            {
              role: 'assistant',
              content: {
                type: 'text',
                text: `# Welcome to qsv Data Wrangling! 🎉

I'm your qsv assistant, ready to help you wrangle CSV, Excel, and JSONL files with ease.

## What is qsv?

qsv is a blazingly-fast command-line toolkit with 66 commands for:
- ✅ **Transforming** data (select, rename, replace, apply)
- ✅ **Analyzing** data (stats, frequency, describe)
- ✅ **Validating** data (schema, validate, safenames)
- ✅ **Querying** data with SQL (sqlp, joinp)
- ✅ **Converting** formats (Excel, JSONL, Parquet)

## 🔒 Privacy & Security

- **100% local processing** - your data never leaves your machine
- **Restricted access** - only works with directories you approve
- **No cloud uploads** - all operations happen on your computer

## Quick Start

**1. List your CSV files**
\`\`\`
List CSV files in my Downloads folder
\`\`\`

**2. Preview a file**
\`\`\`
Show me the first few rows of data.csv
\`\`\`

**3. Get statistics**
\`\`\`
Calculate statistics for the price column in sales.csv
\`\`\`

**4. Filter and transform**
\`\`\`
From customers.csv, remove duplicates and sort by revenue
\`\`\`

## Common Commands

- **qsv_stats** - Calculate statistics (sum, mean, min, max, etc.)
- **qsv_frequency** - Show value distributions
- **qsv_search** - Filter rows with regex patterns
- **qsv_select** - Select/exclude columns
- **qsv_pipeline** - Chain multiple operations

## Need Help?

- Ask "Show me qsv examples" for more usage patterns
- Say "List available qsv commands" to see all 66 commands
- Ask about specific operations like "How do I deduplicate a CSV?"

## Ready to Start?

Just tell me what you want to do with your data, and I'll help you use qsv to get it done!`,
              },
            },
          ],
        };
      }

      if (name === 'examples') {
        return {
          messages: [
            {
              role: 'user',
              content: {
                type: 'text',
                text: 'Show me common qsv usage examples',
              },
            },
            {
              role: 'assistant',
              content: {
                type: 'text',
                text: `# Common qsv Usage Examples

## Data Exploration

**Preview a CSV file:**
\`\`\`
Show me the first 10 rows of data.csv
\`\`\`

**Get column statistics:**
\`\`\`
Calculate statistics for all columns in sales.csv
\`\`\`

**Show value frequency:**
\`\`\`
Show the frequency distribution of the 'status' column in orders.csv
\`\`\`

## Data Cleaning

**Remove duplicates:**
\`\`\`
Remove duplicate rows from customers.csv and save as cleaned.csv
\`\`\`

**Filter rows:**
\`\`\`
From sales.csv, keep only rows where the price column is greater than 100
\`\`\`

**Fix column names:**
\`\`\`
Make all column names in data.csv safe (remove special characters)
\`\`\`

## Data Transformation

**Select columns:**
\`\`\`
From users.csv, select only the name, email, and city columns
\`\`\`

**Rename columns:**
\`\`\`
In data.csv, rename 'old_name' to 'new_name'
\`\`\`

**Sort data:**
\`\`\`
Sort sales.csv by revenue in descending order
\`\`\`

## Complex Workflows

**Multi-step pipeline:**
\`\`\`
From customers.csv:
1. Remove duplicate emails
2. Keep only customers from California
3. Sort by revenue descending
4. Take top 100
5. Save as top_customers.csv
\`\`\`

**Join datasets:**
\`\`\`
Join orders.csv with customers.csv on customer_id
\`\`\`

## Data Validation

**Check schema:**
\`\`\`
Validate sales.csv against this schema: price is number, date is date format
\`\`\`

**Find data quality issues:**
\`\`\`
Check products.csv for empty values, duplicates, and invalid data
\`\`\`

## Format Conversion

**Excel to CSV:**
\`\`\`
Convert spreadsheet.xlsx to CSV format
\`\`\`

**CSV to JSONL:**
\`\`\`
Convert data.csv to JSONL format
\`\`\`

---

**Try any of these patterns with your own files!** Just describe what you want to do, and I'll use qsv to make it happen.`,
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
