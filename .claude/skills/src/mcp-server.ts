#!/usr/bin/env node
/**
 * QSV MCP Server
 *
 * Model Context Protocol server exposing qsv's 66 CSV data-wrangling commands
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
    this.executor = new SkillExecutor();
    this.resourceProvider = new ExampleResourceProvider(this.loader);
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
      tools.push(createGenericToolDefinition());

      // Add pipeline tool
      tools.push(createPipelineToolDefinition());

      console.error(`Registered ${tools.length} tools`);

      return { tools };
    });

    // Call tool handler
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      console.error(`Tool called: ${name}`);

      try {
        // Handle pipeline tool
        if (name === 'qsv_pipeline') {
          return await executePipeline(args || {}, this.loader);
        }

        // Handle generic command tool
        if (name === 'qsv_command') {
          return await handleGenericCommand(
            args || {},
            this.executor,
            this.loader,
          );
        }

        // Handle common command tools
        if (name.startsWith('qsv_')) {
          return await handleToolCall(
            name,
            args || {},
            this.executor,
            this.loader,
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
      // TODO: Implement pagination using request.params?.cursor when resource
      //       listing supports cursors. For now, we ignore the cursor and
      //       return all available resources.

      console.error('Listing resources...');

      try {
        const resources = await this.resourceProvider.listResources();

        console.error(`Found ${resources.length} example resources`);

        // MCP supports pagination with cursors, but for now we return all
        return {
          resources,
        };
      } catch (error) {
        console.error('Error listing resources:', error);

        return {
          resources: [],
        };
      }
    });

    // Read resource handler
    this.server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
      const { uri } = request.params;

      console.error(`Reading resource: ${uri}`);

      try {
        const resource = await this.resourceProvider.getResource(uri);

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
