# Code Style and Conventions

## TypeScript Configuration
- Target: ES2022, Module: ES2022, strict mode enabled
- Module resolution: node, ESM (`"type": "module"` in package.json)
- All imports use `.js` extension (ESM convention, even for `.ts` source files)
- Declaration files and source maps generated

## Naming Conventions
- **Files**: kebab-case (e.g., `mcp-server.ts`, `tool-handlers.ts`, `command-guidance.ts`)
- **Functions**: camelCase (e.g., `handleToolCall`, `buildArgs`, `createToolDefinition`)
- **Classes**: PascalCase (e.g., `QsvMcpServer`, `SkillExecutor`, `SkillLoader`)
- **Constants**: UPPER_SNAKE_CASE for true constants (e.g., `MAX_MCP_RESPONSE_SIZE`, `CORE_TOOLS`)
- **Interfaces**: PascalCase (e.g., `QsvSkill`, `SkillParams`, `McpToolResult`)
- **Type aliases**: PascalCase (e.g., `SkillCategory`, `SlotResult`)

## Code Style
- JSDoc comments with `/** */` on exported functions and classes
- Functions use `export function` (no default exports observed)
- Error handling uses `unknown` type in catch blocks with `getErrorMessage()` utility
- MCP results use helper functions: `successResult()`, `errorResult()`, `completedDirResult()`
- `as const` assertions for type narrowing in result objects

## Module Organization
- Main server class: `mcp-server.ts` (QsvMcpServer)
- Tool definitions: `tool-definitions.ts` (creates MCP tool schemas)
- Tool handlers: `tool-handlers.ts` (processes tool calls)
- Tool guidance: `command-guidance.ts` (COMMAND_GUIDANCE map)
- Execution: `executor.ts` (SkillExecutor — builds and runs qsv commands)
- Config: `config.ts` (centralized env-var-driven configuration)
- Types: `types.ts` (shared interfaces)
- Utilities: `utils.ts` (shared helpers)

## Testing Conventions
- Test files: `tests/<module>.test.ts` matching source files
- Uses Node.js built-in `node:test` module (`describe`, `it`, `before`, `after`)
- Uses Node.js built-in `node:assert` for assertions
- Test helpers in `tests/test-helpers.ts`
