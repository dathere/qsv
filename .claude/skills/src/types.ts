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
  test_file?: string;
  examples_ref?: string;
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
 * Test-based Examples (load-as-needed)
 */
export interface TestExample {
  name: string;
  description: string;
  input?: {
    data?: string[][];
    filename?: string;
    content?: string;
  };
  command: string;
  args: string[];
  options: Record<string, string>;
  expected?: {
    data?: string[][];
    stdout?: string;
    stderr?: string;
  };
  tags: string[];
}

export interface TestExamples {
  skill: string;
  version: string;
  examples: TestExample[];
}

/**
 * MCP (Model Context Protocol) Types
 */
export interface McpToolProperty {
  type: 'string' | 'number' | 'boolean' | 'object' | 'array';
  description?: string;
  default?: string | number | boolean;
  items?: Record<string, unknown>;
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
