/**
 * QSV Agent Skills Type Definitions
 */

export interface QsvSkill {
  name: string;
  version: string;
  description: string;
  category: string;
  command: CommandSpec;
  examples: Example[];
  hints?: BehavioralHints;
}

export interface CommandSpec {
  binary: string;
  subcommand: string;
  args: Argument[];
  options: Option[];
}

export interface Argument {
  name: string;
  type: 'string' | 'number' | 'file' | 'regex';
  required: boolean;
  description: string;
  examples?: string[];
  enum?: string[];
}

export interface Option {
  flag: string;
  short?: string;
  type: 'flag' | 'string' | 'number';
  description: string;
  default?: string;
}

export interface Example {
  description: string;
  command: string;
}

export interface BehavioralHints {
  streamable: boolean;
  indexed?: boolean;
  memory: 'constant' | 'proportional' | 'full';
}

export interface SkillParams {
  args?: Record<string, any>;
  options?: Record<string, any>;
  stdin?: string | Buffer;
  inputFile?: string;
}

export interface SkillResult {
  success: boolean;
  output: string;
  stderr: string;
  metadata: {
    command: string;
    duration: number;
    rowsProcessed?: number;
    exitCode: number;
  };
}

export interface PipelineStep {
  skillName: string;
  params: SkillParams;
}

export interface PipelineResult {
  output: Buffer;
  steps: SkillResult[];
  totalDuration: number;
}

export type SkillCategory =
  | 'selection'
  | 'filtering'
  | 'transformation'
  | 'aggregation'
  | 'joining'
  | 'validation'
  | 'formatting'
  | 'conversion'
  | 'analysis'
  | 'utility';

/**
 * MCP (Model Context Protocol) Types
 */
export interface McpToolProperty {
  type: 'string' | 'number' | 'boolean' | 'object' | 'array';
  description?: string;
  default?: string | number | boolean;
  items?: Record<string, unknown>;
  enum?: string[];
}

export interface McpToolDefinition {
  name: string;
  description: string;
  inputSchema: {
    type: 'object';
    properties: Record<string, McpToolProperty>;
    required?: string[];
  };
}

export interface McpToolResult {
  content: Array<{
    type: 'text' | 'resource';
    text?: string;
    resource?: string;
  }>;
  isError?: boolean;
}

export interface McpResource {
  uri: string;
  name: string;
  description?: string;
  mimeType?: string;
}

export interface McpResourceContent {
  uri: string;
  mimeType?: string;
  text?: string;
}

export interface McpPipelineStep {
  command: string;
  params: Record<string, unknown>;
}

/**
 * CSV file metadata (cached from qsv commands)
 */
export interface FileMetadata {
  rowCount: number | null;
  columnCount: number | null;
  columnNames: string[];
  hasStatsCache: boolean;
  cachedAt: number; // Timestamp for cache expiration
}

/**
 * File information structure for resource content
 */
export interface FileInfo {
  file: {
    name: string;
    path: string;
    absolutePath: string;
    size: number;
    sizeFormatted: string;
    modified: string;
    extension: string;
  };
  preview: string;
  usage: {
    description: string;
    examples: string[];
  };
  conversion?: {
    required: boolean;
    command: string;
    note: string;
  };
  metadata?: {
    rowCount: number;
    columnCount: number;
    columnNames: string[];
    hasStatsCache: boolean;
  };
}

/**
 * Filesystem provider with extended capabilities
 */
export interface FilesystemProviderExtended {
  resolvePath: (path: string) => Promise<string>;
  needsConversion: (path: string) => boolean;
  getConversionCommand: (path: string) => string | null;
  getWorkingDirectory: () => string;
  listFiles: (directory?: string, recursive?: boolean) => Promise<{ resources: McpResource[] }>;
}
