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
import { ExampleResourceProvider } from './mcp-resources.js';
import { FilesystemResourceProvider } from './mcp-filesystem.js';
import type { McpToolResult } from './types.js';
import {
  COMMON_COMMANDS,
  createToolDefinition,
  createGenericToolDefinition,
  handleToolCall,
  handleGenericCommand,
} from './mcp-tools.js';
import {
  createPipelineToolDefinition,
  executePipeline,
} from './mcp-pipeline.js';

/**
 * QSV MCP Server implementation
 */
class QsvMcpServer {
  private server: Server;
  private loader: SkillLoader;
  private executor: SkillExecutor;
  private resourceProvider: ExampleResourceProvider;
  private filesystemProvider: FilesystemResourceProvider;

  constructor() {
    this.server = new Server(
      {
        name: 'qsv-server',
        version: '12.0.0',
      },
      {
        capabilities: {
          tools: {},
          resources: {},
        },
      },
    );

    this.loader = new SkillLoader();
    this.executor = new SkillExecutor(process.env.QSV_BIN_PATH || 'qsv');
    this.resourceProvider = new ExampleResourceProvider(this.loader);

    // Initialize filesystem provider with configurable directories
    const workingDir = process.env.QSV_WORKING_DIR || process.cwd();
    // Use platform-appropriate delimiter: semicolon on Windows, colon on Unix
    const pathDelimiter = process.platform === 'win32' ? ';' : ':';
    const allowedDirs = process.env.QSV_ALLOWED_DIRS
      ? process.env.QSV_ALLOWED_DIRS.split(pathDelimiter)
      : [];

    this.filesystemProvider = new FilesystemResourceProvider({
      workingDirectory: workingDir,
      allowedDirectories: allowedDirs,
    });
  }

  /**
   * Initialize the server and register handlers
   */
  async initialize(): Promise<void> {
    // Load all skills
    console.error('Loading QSV skills...');
    const skills = await this.loader.loadAll();
    console.error(`Loaded ${skills.size} skills`);

    // Register tool handlers
    this.registerToolHandlers();

    // Register resource handlers
    this.registerResourceHandlers();

    console.error('QSV MCP Server initialized successfully');
  }

  /**
   * Register MCP tool handlers
   */
  private registerToolHandlers(): void {
    // List tools handler
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      const tools = [];

      // Add 20 common command tools
      for (const command of COMMON_COMMANDS) {
        const skillName = `qsv-${command}`;
        const skill = await this.loader.load(skillName);

        if (skill) {
          tools.push(createToolDefinition(skill));
        } else {
          console.error(`Warning: Failed to load skill ${skillName}`);
        }
      }

      // Add generic qsv_command tool
      tools.push(createGenericToolDefinition(this.loader));

      // Add pipeline tool
      tools.push(createPipelineToolDefinition());

      // Add filesystem tools
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

      const cursor = request.params?.cursor;

      if (cursor) {
        console.error(`Pagination cursor: ${cursor}`);
      }

      // When pagination is active, only return examples (which support pagination)
      // When no cursor, return filesystem files + first page of examples
      if (cursor) {
        // Pagination active - only return examples
        const exampleResult = await this.resourceProvider.listResources(undefined, cursor);

        console.error(
          `Returning ${exampleResult.resources.length} example resources` +
          (exampleResult.nextCursor ? ` (more available)` : ' (last page)'),
        );

        return {
          resources: exampleResult.resources,
          nextCursor: exampleResult.nextCursor,
        };
      } else {
        // First page - return filesystem files + first page of examples
        const filesystemResult = await this.filesystemProvider.listFiles(undefined, false);
        const exampleResult = await this.resourceProvider.listResources(undefined, undefined);

        const allResources = [
          ...filesystemResult.resources,
          ...exampleResult.resources,
        ];

        console.error(
          `Returning ${allResources.length} resources ` +
          `(${filesystemResult.resources.length} files, ${exampleResult.resources.length} examples)` +
          (exampleResult.nextCursor ? ` (more examples available)` : ''),
        );

        return {
          resources: allResources,
          nextCursor: exampleResult.nextCursor,
        };
      }
    });

    // Read resource handler
    this.server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
      const { uri } = request.params;

      console.error(`Reading resource: ${uri}`);

      try {
        let resource;

        // Check if it's a file:/// URI (filesystem resource)
        if (uri.startsWith('file:///')) {
          resource = await this.filesystemProvider.getFileContent(uri);
        } else {
          // Otherwise, it's an example resource
          resource = await this.resourceProvider.getResource(uri);
        }

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
 * Main entry point
 */
async function main(): Promise<void> {
  const server = new QsvMcpServer();

  try {
    await server.initialize();
    await server.start();

    // Keep the process running
    process.on('SIGINT', () => {
      console.error('Shutting down QSV MCP Server...');
      process.exit(0);
    });
  } catch (error) {
    console.error('Fatal error starting QSV MCP Server:', error);
    process.exit(1);
  }
}

// Run the server
main();
