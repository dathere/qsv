/**
 * QSV Agent Skills
 * Auto-generated skills for qsv CSV data-wrangling toolkit
 */

export { SkillLoader } from './loader.js';
export { SkillExecutor } from './executor.js';
export { QsvPipeline } from './pipeline.js';

export type {
  QsvSkill,
  CommandSpec,
  Argument,
  Option,
  Example,
  BehavioralHints,
  SkillParams,
  SkillResult,
  PipelineStep,
  PipelineResult,
  SkillCategory,
  TestExample,
  TestExamples,
  McpToolProperty,
  McpToolDefinition,
  McpToolResult,
  McpResource,
  McpResourceContent,
  McpPipelineStep,
} from './types.js';

// MCP Server Components
export { ExampleResourceProvider } from './mcp-resources.js';
export {
  COMMON_COMMANDS,
  createToolDefinition,
  createGenericToolDefinition,
  handleToolCall,
  handleGenericCommand,
} from './mcp-tools.js';
export {
  createPipelineToolDefinition,
  executePipeline,
  pipelineToShellScript,
} from './mcp-pipeline.js';

// Re-export for convenience
export { SkillLoader as default } from './loader.js';
