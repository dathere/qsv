# CLAUDE.md - Agent Skills Development Guide

This file provides guidance to Claude Code (claude.ai/code) when working with the qsv Agent Skills project.

## Project Overview

This is the **qsv Agent Skills** project - a TypeScript-based MCP (Model Context Protocol) server that exposes qsv's tabular data-wrangling capabilities to AI agents like Claude. It enables agents to discover, invoke, and compose qsv commands through a standardized protocol for processing CSV, TSV, Excel, JSONL, and other tabular data formats.

**Key Components**:
- **MCP Server**: Exposes qsv commands as MCP tools/resources
- **MCP Desktop Extension (MCPB)**: One-click installation bundle for Claude Desktop
- **Converted File Manager**: Tracks converted CSV files with automatic cleanup (LIFO cache)
- **Pipeline System**: Composes multi-step qsv workflows
- **Update Checker**: Monitors qsv binary versions and auto-regenerates skills
- **Type System**: Strong typing for qsv commands and parameters
- **Guidance Enhancement**: Intelligent tool descriptions with USE WHEN, COMMON PATTERNS, and CAUTION hints

**Goals**:
1. Make all 60 qsv commands discoverable and invokable by AI agents
2. Auto-generate tool definitions from qsv usage text (zero documentation debt)
3. Enable intelligent composition of complex data workflows with multi-format support
4. Provide seamless integration with Claude Desktop and other MCP clients
5. Help Claude make optimal tool choices through enhanced descriptions
6. Support diverse tabular data formats (CSV, TSV, Excel, JSONL, SSV, etc.)

## What's New

### Version 15.2.0
- **SQL Query Optimization** - New `qsv_data_profile` tool profiles CSV files for optimal SQL query composition
  - Uses `qsv frequency --toon` to generate column statistics in TOON format (token-efficient for LLMs)
  - Shows data types, cardinality, null counts, value distributions
  - Helps Claude choose optimal JOIN order, GROUP BY columns, and WHERE selectivity

### Version 15.1.1
- **Skill Version Sync** - Updated all 60 skill JSON files to version 15.1.1

### Version 15.1.0
- **Simplified Tool Guidance** - Removed redundant feature requirement hints (Polars, Luau) from tool descriptions
- **DuckDB Fallback** - Added guidance to use DuckDB as an alternative when sqlp encounters errors with complex queries
- **Expanded Error Prevention** - Added cat, dedup, sort, and searchset to commands with common mistake warnings
- **Streamlined Descriptions** - Removed verbose optimization hints that are now handled automatically

### Version 15.0.0
- **Tool Search Support** - New `qsv_search_tools` tool for discovering qsv commands by keyword, category, or regex
- **US Census MCP Integration** - Census MCP server awareness with integration guides
- **Expose All Tools Mode** - Auto-detects Claude clients for automatic tool exposure

### Version 14.2.0
- **Tool Search Support** - New `qsv_search_tools` tool for discovering qsv commands by keyword, category, or regex
- **Expose All Tools Mode** - `QSV_MCP_EXPOSE_ALL_TOOLS=true` exposes all 67 qsv tools for clients with tool search/deferred loading
- **Anthropic Tool Search Integration** - Compatible with Anthropic API Tool Search Tool (`tool_search_tool_bm25_20251119`)
- **Improved Tool Discovery** - Search by category (selection, filtering, transformation, etc.) or use regex patterns
- **Client Auto-Detection** - Auto-detects Claude clients (Desktop, Code, Cowork) for expose-all-tools mode
- **Configurable Examples** - `QSV_MCP_MAX_EXAMPLES` controls examples in tool descriptions (0-20, default: 5)

### Version 14.1.0
- **Versioned MCPB Packaging** - `.mcpb` files now include version (e.g., `qsv-mcp-server-14.1.0.mcpb`)
- **Token Optimization** - 66-76% reduction in tool description token usage
- **Windows EPERM Retry Logic** - Exponential backoff for Windows file locking errors
- **Streaming Executor** - Uses `spawn` instead of `execFileSync` for better output handling
- **Output Size Limits** - 50MB stdout limit prevents memory issues on large outputs
- **Cross-Platform Test Runner** - `scripts/run-tests.js` handles glob expansion for Node 20+
- **Help Request Handling** - `--help` requests skip input file validation

### Version 14.0.0
- **MCP Desktop Extension (MCPB)** - User-friendly one-click installation
- **Enhanced Tool Descriptions** - USE WHEN, COMMON PATTERNS, CAUTION guidance
- **Stats Cache Auto-Generation** - Automatically enables `--stats-jsonl`
- **Production CI/CD** - Testing across Node.js 20, 22, 24 on all platforms
- **Security Improvements** - Secure command execution prevents injection

## Build Commands

### Development

```bash
# Build TypeScript to JavaScript
npm run build

# Build with test configuration
npm run build:test

# Watch mode (requires manual setup)
tsc --watch
```

### Testing

```bash
# Run all tests (builds first, uses cross-platform runner)
npm test

# Run tests in watch mode
npm run test:watch

# Run specific test file
node --test dist/tests/mcp-filesystem.test.js

# Test MCP server integration
npm run test:examples

# Test pipeline system
npm run test-pipeline

# Test update checker
npm run test-update-checker
```

### MCP Server Operations

```bash
# Start MCP server (stdio mode for Claude Desktop)
npm run mcp:start

# Install MCP server to Claude Desktop config
npm run mcp:install

# Package as MCPB (MCP Bundle) for distribution
npm run mcpb:package
```

## Architecture

### Directory Structure

```
.claude/skills/
├── src/                    # TypeScript source files
│   ├── mcp-server.ts      # Main MCP server entry point
│   ├── mcp-tools.ts       # MCP tool definitions with guidance enhancement
│   ├── mcp-filesystem.ts  # Filesystem operations via MCP
│   ├── mcp-pipeline.ts    # Multi-step pipeline execution
│   ├── converted-file-manager.ts  # LIFO cache for converted files
│   ├── config.ts          # Configuration and validation
│   ├── client-detector.ts # Client detection for auto-enabling features
│   ├── executor.ts        # qsv command execution (streaming)
│   ├── update-checker.ts  # Version detection and skill regeneration
│   ├── types.ts           # TypeScript type definitions
│   ├── utils.ts           # Utility functions
│   ├── version.ts         # Version management
│   ├── loader.ts          # Dynamic skill loading and searching
│   ├── pipeline.ts        # Fluent pipeline API for chaining qsv skills
│   └── index.ts           # Module exports
├── dist/                   # Compiled JavaScript output
├── tests/                  # Test files (TypeScript)
│   ├── client-detector.test.ts
│   ├── config.test.ts
│   ├── converted-file-manager.test.ts
│   ├── executor.test.ts
│   ├── mcp-filesystem.test.ts
│   ├── mcp-pipeline.test.ts
│   ├── mcp-tools.test.ts
│   ├── qsv-integration.test.ts
│   ├── tool-filtering.test.ts
│   ├── update-checker.test.ts
│   ├── utils.test.ts
│   └── version.test.ts
├── examples/               # Example usage scripts
├── docs/                   # Design documentation
│   └── design/            # Architecture and design docs
├── scripts/                # Build and deployment scripts
│   ├── install-mcp.js     # Installation helper
│   ├── package-mcpb.js    # MCPB packaging script
│   └── run-tests.js       # Cross-platform test runner
├── qsv/                    # Auto-generated skill JSON files (60)
├── node_modules/          # Dependencies
├── package.json           # NPM package configuration
├── tsconfig.json          # TypeScript compiler config
├── tsconfig.test.json     # Test-specific TypeScript config
├── manifest.json          # MCP Bundle manifest (spec v0.3)
├── README-MCP.md          # MCP Server documentation
├── README-MCPB.md         # Desktop Extension documentation
└── CLAUDE.md              # This file
```

### Core Modules

#### `mcp-server.ts` - MCP Server Entry Point
- Implements Model Context Protocol server
- Handles stdio communication with MCP clients (Claude Desktop)
- Registers tools, resources, and prompts
- Manages server lifecycle with graceful shutdown
- Auto-enables `--stats-jsonl` for stats command
- Integrates update checker for background version monitoring
- **Client auto-detection**: Uses `client-detector.ts` to identify Claude clients
- **Expose-all-tools mode**: Auto-enables for tool-search-capable clients

**Key Functions**:
```typescript
server.setRequestHandler(ListToolsRequestSchema, async () => { ... })
server.setRequestHandler(CallToolRequestSchema, async (request) => { ... })
server.setRequestHandler(ListResourcesRequestSchema, async () => { ... })
```

#### `mcp-tools.ts` - Tool Definitions and Handlers
- Defines all qsv commands as MCP tools
- Handles tool invocation with parameter validation
- Returns structured results (CSV data, file paths, stats)
- **Adds guidance enhancement** (USE WHEN, COMMON PATTERNS, CAUTION hints)
- **Tool filtering** based on available qsv commands at runtime

**Key Constants**:
- `COMMON_COMMANDS`: 13 frequently-used commands (select, stats, moarstats, index, search, frequency, headers, count, slice, sqlp, joinp, cat, geocode)
- `ALWAYS_FILE_COMMANDS`: 23 commands that always output to files
- `METADATA_COMMANDS`: 5 commands returning metadata (count, headers, index, slice, sample)
- `AUTO_INDEX_THRESHOLD`: 10MB - files larger than this are auto-indexed

**Tool Structure with Guidance**:
```typescript
{
  name: "qsv_select",
  description: `Select columns from CSV...

💡 USE WHEN: Choosing specific columns. Use selection syntax: "1,3,5" for specific columns.

📋 COMMON PATTERN: Often first step in pipelines for column cleanup.

⚠️ CAUTION: Column indices are 1-based, not 0-based.`,
  inputSchema: {
    type: "object",
    properties: {
      input_file: { type: "string", description: "..." },
      selection: { type: "string", description: "..." },
    },
    required: ["input_file", "selection"]
  }
}
```

#### `executor.ts` - Command Execution
- Spawns qsv child processes using `spawn` for streaming output
- Handles stdin/stdout/stderr with proper buffering
- **Output size limit**: 50MB to prevent memory issues
- **Help request detection**: Skips input validation for `--help`
- **Subcommand support**: First-class handling of commands with subcommands
- **Stats cache auto-generation**: Forces `--stats-jsonl` for stats command
- Timeout management and error parsing

**Key Features**:
- Validates parameters before execution (unless `--help` requested)
- Builds shell-safe command arguments
- Extracts row counts from stderr for metadata
- Returns structured results with exit codes and timing

#### `update-checker.ts` - Version Management
- Detects qsv binary version at runtime
- Compares skill definition versions with binary version
- Checks GitHub releases for available updates
- Auto-regenerates skills when `autoRegenerateSkills` is enabled
- Persists version info in `.qsv-mcp-versions.json`

**Key Features**:
- Quick check (local only, no network)
- Full check (includes GitHub API call)
- Semantic version comparison
- Extension mode support (skips MCP server version checks)

#### `converted-file-manager.ts` - File Lifecycle Management
- LIFO (Last In, First Out) cache for converted files
- File locking to prevent race conditions
- Change detection via mtime, size, inode, and optional hash
- Cache corruption recovery with validation
- UUID-based temp file names for security
- Secure permissions (0o600)
- **Windows EPERM retry logic**: Exponential backoff for file locking errors

**Key Features**:
- Tracks conversions (Excel → CSV, JSON → CSV, etc.)
- Automatic cleanup with configurable TTL
- File size monitoring and performance metrics
- Conversion statistics

#### `mcp-pipeline.ts` - Workflow Composition
- Chains multiple qsv commands into pipelines
- Handles intermediate file management
- Error recovery and rollback
- Performance optimization (automatic indexing)

**Example Pipeline**:
```typescript
{
  steps: [
    { tool: "qsv_select", args: { selection: "!SSN,password" } },
    { tool: "qsv_dedup", args: {} },
    { tool: "qsv_stats", args: { everything: true } }
  ]
}
```

#### `config.ts` - Configuration System
- Environment variable loading with template expansion
- qsv binary detection and validation (5-second timeout)
- Available commands detection at runtime
- Working directory and allowed directories configuration
- Extension mode detection (`MCPB_EXTENSION_MODE`)

**Template Variables Supported**:
- `${HOME}`, `${USERPROFILE}` - User home directory
- `${DESKTOP}`, `${DOCUMENTS}`, `${DOWNLOADS}` - Common directories
- `${TEMP}`, `${TMPDIR}` - Temporary directories

**Key Environment Variables**:
- `QSV_MCP_BIN_PATH`: Path to qsv binary
- `QSV_MCP_WORKING_DIR`: Working directory for file operations
- `QSV_MCP_ALLOWED_DIRS`: Colon-separated list of allowed directories
- `QSV_MCP_OPERATION_TIMEOUT_MS`: Operation timeout (default: 120000ms)
- `QSV_MCP_TIMEOUT_MS`: Alternative timeout for desktop extensions (default: 5 minutes)
- `QSV_MCP_MAX_PIPELINE_STEPS`: Max pipeline steps (default: 50)
- `QSV_MCP_MAX_FILES_PER_LISTING`: Max files in directory listing (default: 1000)
- `QSV_MCP_MAX_CONCURRENT_OPERATIONS`: Max concurrent ops (default: 10)
- `QSV_MCP_MAX_OUTPUT_SIZE`: Max output size in bytes (default: 50MB)
- `QSV_MCP_MAX_EXAMPLES`: Max examples in tool descriptions (default: 5, range: 0-20)
- `QSV_MCP_CONVERTED_LIFO_SIZE_GB`: Cache size in GB (default: 1.0)
- `QSV_MCP_AUTO_REGENERATE_SKILLS`: Auto-regenerate on version change
- `QSV_MCP_CHECK_UPDATES_ON_STARTUP`: Check for updates at startup
- `QSV_MCP_NOTIFY_UPDATES`: Show update notifications
- `QSV_MCP_GITHUB_REPO`: GitHub repo for update checks (default: dathere/qsv)
- `QSV_MCP_EXPOSE_ALL_TOOLS`: Optional boolean for expose-all-tools (auto-detects by default)
- `MCPB_EXTENSION_MODE`: Desktop extension mode flag

#### `client-detector.ts` - Client Detection
- Detects MCP client type (Claude Desktop, Code, Cowork)
- Auto-enables expose-all-tools mode for known Claude clients
- Uses strict pattern matching to avoid misclassification

**Key Functions**:
- `isToolSearchCapableClient(clientInfo)`: Check if client supports tool search
- `getClientType(clientInfo)`: Get client type enum for logging/analytics
- `formatClientInfo(clientInfo)`: Format client info for human-readable logging

**Exports**:
- `ClientType`: Type union of `'claude-desktop' | 'claude-code' | 'claude-cowork' | 'claude-generic' | 'other' | 'unknown'`

#### `loader.ts` - Dynamic Skill Loading
- Loads skill definitions from JSON files in `qsv/` directory
- Provides skill searching and categorization
- Supports dynamic skill discovery at runtime

**Key Methods**:
- `loadAll()`: Load all skills from JSON files
- `load(skillName)`: Load a specific skill by name
- `search(query)`: Search skills by name, description, or category
- `getByCategory(category)`: Get skills filtered by category
- `getCategories()`: Get all available categories
- `getStats()`: Get statistics (total skills, examples, options, args)

**Skill Categories** (10 categories):
- `selection` - Column selection and reordering
- `filtering` - Row filtering and search
- `transformation` - Data transformation and modification
- `aggregation` - Statistics and aggregation
- `joining` - Joining and merging datasets
- `validation` - Data validation and schema checking
- `formatting` - Output formatting and display
- `conversion` - Format conversion (CSV, JSON, Excel, etc.)
- `analysis` - Data analysis and profiling
- `utility` - Utility commands (index, count, headers, etc.)

### Data Flow

```
┌─────────────────┐
│  Claude Agent   │
│  (MCP Client)   │
└────────┬────────┘
         │ MCP Protocol (stdio)
         ▼
┌─────────────────┐
│   mcp-server    │
│  ListTools()    │
│  CallTool()     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐      ┌──────────────────┐
│   mcp-tools     │─────▶│  executor.ts     │
│  Validate args  │      │  spawn()         │
│  Add guidance   │      │  (streaming)     │
└────────┬────────┘      └─────────┬────────┘
         │                         │
         │                         ▼
         │                  ┌─────────────┐
         │                  │ qsv binary  │
         │                  │ (Rust)      │
         │                  │ Multi-format│
         │                  │ support     │
         │                  └─────────────┘
         │
         ▼
┌─────────────────┐
│ converted-file- │
│   manager.ts    │
│ Excel→CSV       │
│ JSONL→CSV       │
└─────────────────┘
```

## Development Workflow

### Adding a New MCP Tool

When qsv adds a new command or you need to expose an existing one:

1. **Update `mcp-tools.ts`**:
   ```typescript
   // Add to TOOL_DEFINITIONS array
   {
     name: "qsv_yourcommand",
     description: `Brief description from qsv usage text

💡 USE WHEN: Specific use case guidance.

📋 COMMON PATTERN: How this fits into workflows.

⚠️ CAUTION: Any warnings about memory, performance, etc.`,
     inputSchema: {
       type: "object",
       properties: {
         input_file: {
           type: "string",
           description: "Input CSV file path"
         },
         your_parameter: {
           type: "string",
           description: "Parameter description"
         }
       },
       required: ["input_file"]
     }
   }
   ```

2. **Add handler in `handleToolCall()`**:
   ```typescript
   case "qsv_yourcommand":
     return await handleYourCommand(args);
   ```

3. **Implement handler function**:
   ```typescript
   async function handleYourCommand(args: any): Promise<ToolResult> {
     validateFileExists(args.input_file);

     const qsvArgs = buildQsvArgs("yourcommand", args);
     const result = await executeQsv(qsvArgs);

     return {
       content: [
         { type: "text", text: result.output }
       ]
     };
   }
   ```

4. **Add tests in `tests/`**:
   ```typescript
   test("qsv_yourcommand should process CSV", async () => {
     const result = await callTool("qsv_yourcommand", {
       input_file: "test.csv",
       your_parameter: "value"
     });
     assert.strictEqual(result.success, true);
   });
   ```

5. **Update documentation**: Add example to relevant docs in `docs/`

### Guidance Enhancement System

Tool descriptions include intelligent guidance to help Claude make optimal decisions:

- **💡 USE WHEN** - When to use this tool vs alternatives
- **📋 COMMON PATTERN** - How this tool fits into workflows
- **⚠️ CAUTION** - Memory limits, file size constraints, feature requirements
- **🚀 PERFORMANCE** - Index acceleration tips, cache strategies

**Guidelines for Writing Guidance**:
1. Keep USE WHEN concise (1-2 sentences)
2. Reference alternative tools when applicable (`join` vs `joinp`)
3. Include file size thresholds for memory-intensive commands
4. Mention when index acceleration is available (📇)
5. Note if command loads entire file into memory (🤯)

### Modifying Existing Tools

**IMPORTANT**: When updating tool definitions:
1. Always read the current qsv usage text first (`qsv yourcommand --help`)
2. Keep parameter names consistent with qsv flag names (snake_case)
3. Mark required vs optional parameters correctly
4. Include parameter validation (file existence, value ranges)
5. Update tests to cover new parameters
6. Update guidance hints if behavior changes

### Testing Conventions

- Each module has a corresponding test file: `tests/<module>.test.ts`
- Tests use Node.js built-in test runner (no external framework)
- Use `workdir` helper for creating temporary test directories
- Tests should clean up after themselves
- Integration tests should use real qsv binary
- CI runs on Node.js 20, 22, and 24 across macOS, Windows, Linux
- Cross-platform test runner (`scripts/run-tests.js`) handles glob expansion

**Test Structure**:
```typescript
import { test } from "node:test";
import assert from "node:assert";

test("descriptive test name", async () => {
  // Arrange
  const input = createTestData();

  // Act
  const result = await functionUnderTest(input);

  // Assert
  assert.strictEqual(result.status, "success");
});
```

### Running Single Tests

```bash
# Run all tests in a file
node --test dist/tests/mcp-tools.test.js

# Run with filter pattern
node --test --test-name-pattern="qsv_select" dist/tests/

# Run with debugging
node --inspect --test dist/tests/mcp-tools.test.js
```

## TypeScript Conventions

### Type Safety

- Use strict TypeScript configuration (see `tsconfig.json`)
- Avoid `any` type - use `unknown` and type guards instead
- Define interfaces for all qsv command parameters
- Use discriminated unions for result types
- Use type guards for error handling in catch blocks

**Example - Result Types**:
```typescript
interface QsvSelectArgs {
  input_file: string;
  selection: string;
  output?: string;
  no_headers?: boolean;
  delimiter?: string;
}

type QsvResult =
  | { success: true; output: string; file_path?: string }
  | { success: false; error: string; exit_code: number };
```

**Example - Error Type Guard**:
```typescript
// Type guard for Node.js errors with error codes
function isNodeError(error: unknown): error is NodeJS.ErrnoException {
  return error instanceof Error && 'code' in error;
}

// Usage in catch blocks
try {
  await someFileOperation();
} catch (error: unknown) {
  if (isNodeError(error) && error.code === 'ENOENT') {
    // Handle file not found
  }
  throw error;
}
```

### Error Handling

- Use Result types instead of throwing errors in tool handlers
- Provide context in error messages (command, args, file paths)
- Log errors for debugging but return structured errors to MCP clients
- Handle qsv-specific errors (file not found, invalid CSV, etc.)

**Pattern**:
```typescript
try {
  const result = await executeQsv(args);
  return {
    content: [{ type: "text", text: result.output }],
    isError: false
  };
} catch (error) {
  return {
    content: [{
      type: "text",
      text: `Error: ${error.message}\nCommand: qsv ${args.join(" ")}`
    }],
    isError: true
  };
}
```

### Code Style

- Use `async`/`await` instead of promise chains
- Prefer `const` over `let`, never use `var`
- Use template literals for string interpolation
- Format with default TypeScript formatter
- Use meaningful variable names (no single-letter except loop counters)

## MCP Protocol Integration

### Tool Registration

Tools are registered with the MCP server in `mcp-server.ts`:

```typescript
server.setRequestHandler(ListToolsRequestSchema, async () => ({
  tools: TOOL_DEFINITIONS
}));
```

### Resource Exposure

Expose qsv metadata and stats as MCP resources:

```typescript
server.setRequestHandler(ListResourcesRequestSchema, async () => ({
  resources: [
    {
      uri: "qsv://stats/cache",
      name: "QSV Stats Cache",
      description: "Cached statistics for processed files"
    }
  ]
}));
```

### Prompts

Expose qsv workflows as reusable prompts:

```typescript
server.setRequestHandler(ListPromptsRequestSchema, async () => ({
  prompts: [
    {
      name: "qsv_welcome",
      description: "Welcome message and quick start guide"
    },
    {
      name: "qsv_examples",
      description: "Common qsv usage examples"
    }
  ]
}));
```

## Performance Considerations

### File Size Thresholds

- Auto-indexing for files > 10MB improves performance
- Enable stats caching via `--stats-jsonl` (auto-enabled by MCP server)
- Consider memory limits when loading entire CSVs
- Output truncated at 50MB to prevent memory issues

### Parallelism

- Default to `--jobs` based on CPU count
- Limit concurrent qsv processes to prevent resource exhaustion
- Use Polars-powered commands (joinp, sqlp) for large datasets

### Caching Strategies

- **Stats Cache**: Auto-generated `.stats.csv.data.jsonl` files
- **Index Files**: Preserve `.csv.idx` files between operations
- **Converted Files**: Cache Excel→CSV conversions (LIFO cache with configurable size)
- **Version Cache**: `.qsv-mcp-versions.json` tracks version state

## Integration with Main qsv Project

### Dependency Management

This project depends on:
1. **qsv binary**: Must be in PATH or specified via `QSV_MCP_BIN_PATH`
2. **qsv version**: Should match package.json major version (currently 15)
3. **Feature flags**: Some tools require specific qsv features (Polars, etc.)
4. **Node.js**: Requires Node.js 18.0.0 or later
5. **MCP SDK**: Uses @modelcontextprotocol/sdk ^1.25.2

### Version Synchronization

- `package.json` version tracks qsv version
- `version.ts` reads qsv binary version at runtime
- `update-checker.ts` compares versions and suggests updates
- CI checks ensure compatibility

### Feature Detection

```typescript
const features = await detectQsvFeatures();
if (!features.includes("polars")) {
  throw new Error("qsv_joinp requires qsv built with Polars feature");
}
```

### Skills Auto-Update

Regenerate skills when qsv is updated:

```bash
# From qsv repo root (requires mcp feature flag)
qsv --update-mcp-skills

# Then rebuild TypeScript
cd .claude/skills && npm run build
```

The skill generator:
- Parses qsv USAGE text using qsv-docopt
- Extracts concise descriptions from README command table
- Extracts performance hints (📇 indexed, 🤯 memory-intensive) from README
- Creates JSON skill files in `.claude/skills/qsv/`
- The skill generator is implemented in the `../../src/mcp_skills_gen.rs` Rust module which is called from `../../src/main.rs`

## Deployment

### MCPB Packaging

Create distributable MCP Bundle:

```bash
npm run mcpb:package
```

**Generated**:
- `qsv-mcp-server-<version>.mcpb`: Versioned bundle (e.g., `qsv-mcp-server-14.1.0.mcpb`)
- `manifest.json`: Metadata for MCP registry (spec v0.3)
- Icons and assets

**Installation**:
Users can install via Claude Desktop Extensions by pointing to the `.mcpb` file.

**Desktop Extension Features**:
- Auto-detects qsv binary or offers to download
- Template variable expansion (`$HOME`, `${HOME}`, `${DESKTOP}`, etc.)
- Cross-platform support (macOS, Windows, Linux)
- Secure execution via `spawn`

### Claude Desktop Integration

```bash
npm run mcp:install
```

This updates Claude Desktop's MCP configuration at:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

**Configuration Added**:
```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/path/to/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_ALLOWED_DIRS": "/Users/you/Downloads:/Users/you/Documents"
      }
    }
  }
}
```

## Auto-Generation from qsv Usage Text

### Current Implementation

Skill JSON files are **auto-generated** from qsv's USAGE text via the `qsv --update-mcp-skills` command:

1. **Parser**: `src/mcp_skills_gen.rs` extracts from `static USAGE: &str` using qsv-docopt
2. **Descriptions**: Concise descriptions from README.md command table (optimized for tokens)
3. **Performance Hints**: Emoji legends (📇 indexed, 🤯 memory-intensive, 😣 proportional) from README
4. **Detailed Help**: Full documentation available via `qsv <command> --help`
5. **Generator**: Creates JSON skill files in `.claude/skills/qsv/`
6. **Enhancement**: `mcp-tools.ts` adds guidance hints (when-to-use, patterns, cautions)

**Regenerating Skills**:
```bash
# From qsv repo root
cargo build --bin qsv -F all_features
./target/debug/qsv --update-mcp-skills

# Then rebuild TypeScript
cd .claude/skills && npm run build
```

**Token Optimization**:
- Skill descriptions use concise README text instead of verbose USAGE text
- Guidance hints help Claude select the right tool
- Full documentation available on-demand via `--help` flag
- 66-76% reduction in token usage compared to full USAGE text

## Important Files

- **`package.json`**: Dependencies, scripts, versioning (Node.js >=18.0.0)
- **`tsconfig.json`**: TypeScript compiler configuration (ES2022 target)
- **`tsconfig.test.json`**: Test-specific TypeScript configuration
- **`manifest.json`**: MCP Bundle manifest (spec v0.3)
- **`src/mcp-server.ts`**: Main entry point
- **`src/mcp-tools.ts`**: Tool definitions with guidance enhancement
- **`src/client-detector.ts`**: Client detection for auto-enabling features
- **`src/loader.ts`**: Dynamic skill loading and searching
- **`src/executor.ts`**: Streaming command execution
- **`src/update-checker.ts`**: Version management and skill regeneration
- **`src/config.ts`**: Environment and settings
- **`scripts/package-mcpb.js`**: MCPB packaging script
- **`scripts/run-tests.js`**: Cross-platform test runner
- **`../../src/mcp_skills_gen.rs`**: Rust skill generator (in main qsv repo)
- **`docs/design/AGENT_SKILLS_DESIGN.md`**: Architecture vision
- **`README-MCP.md`**: MCP Server documentation
- **`README-MCPB.md`**: Desktop Extension documentation
- **`CLAUDE.md`**: This file

## Common Patterns

### Executing qsv Command

```typescript
import { spawn } from "child_process";

async function executeQsv(args: string[]): Promise<{
  stdout: string;
  stderr: string;
  exitCode: number;
}> {
  return new Promise((resolve, reject) => {
    const proc = spawn("qsv", args, {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';
    const MAX_STDOUT_SIZE = 50 * 1024 * 1024; // 50MB limit

    proc.stdout.on('data', chunk => {
      if (stdout.length < MAX_STDOUT_SIZE) {
        stdout += chunk.toString();
      }
    });

    proc.stderr.on('data', chunk => {
      stderr += chunk.toString();
    });

    proc.on('close', exitCode => {
      resolve({ stdout, stderr, exitCode: exitCode || 0 });
    });

    proc.on('error', reject);
  });
}
```

### Building qsv Arguments

```typescript
function buildQsvArgs(
  command: string,
  args: Record<string, any>
): string[] {
  const qsvArgs = [command];

  // Add flags
  if (args.no_headers) qsvArgs.push("--no-headers");
  if (args.delimiter) qsvArgs.push("--delimiter", args.delimiter);

  // Add positional arguments
  if (args.selection) qsvArgs.push(args.selection);
  if (args.input_file) qsvArgs.push(args.input_file);

  // Add output flag
  if (args.output) qsvArgs.push("--output", args.output);

  return qsvArgs;
}
```

### Validating Tool Arguments

```typescript
function validateSelectArgs(args: any): QsvSelectArgs {
  // Skip validation for help requests
  if (args.help) {
    return args as QsvSelectArgs;
  }

  if (!args.input_file) {
    throw new Error("input_file is required");
  }

  if (!fs.existsSync(args.input_file)) {
    throw new Error(`File not found: ${args.input_file}`);
  }

  if (!args.selection) {
    throw new Error("selection is required");
  }

  return args as QsvSelectArgs;
}
```

### Handling CSV Output

```typescript
// Return CSV data directly
return {
  content: [{
    type: "text",
    text: csvOutput,
    mimeType: "text/csv"
  }]
};

// Or return file path for large results
return {
  content: [{
    type: "resource",
    resource: {
      uri: `file://${outputPath}`,
      mimeType: "text/csv"
    }
  }]
};
```

## Troubleshooting

### Common Issues

**qsv not found**:
- Set `QSV_MCP_BIN_PATH` environment variable
- Ensure qsv is in PATH
- Check qsv version: `qsv --version`

**Type errors after build**:
- Delete `dist/` and rebuild: `rm -rf dist && npm run build`
- Check `tsconfig.json` configuration

**Tests failing**:
- Ensure qsv binary is available
- Check test data files exist
- Verify working directory is project root
- Run `npm run build:test` before running tests

**MCP connection issues**:
- Restart Claude Desktop
- Check MCP server logs in Claude Desktop developer console
- Verify configuration in `claude_desktop_config.json`

**MCPB installation issues**:
- Ensure manifest.json follows spec v0.3
- Check that qsv binary path is accessible
- Verify allowed directories exist

**Windows EPERM errors**:
- File locking due to antivirus or other processes
- Automatic retry with exponential backoff handles most cases
- If persistent, check if file is open in another application

**Output truncated**:
- Large outputs (>50MB) are truncated to prevent memory issues
- Use `--output` option to write results to a file instead

**Skills outdated warning**:
- Run `qsv --update-mcp-skills` to regenerate skills
- Then rebuild: `npm run build`
- Check `.qsv-mcp-versions.json` for version state

## Development Best Practices

1. **Always test with real qsv binary** - don't mock qsv in integration tests
2. **Keep tools simple** - one qsv command per MCP tool
3. **Use pipelines for composition** - chain simple tools, don't create mega-tools
4. **Validate early** - check file existence and parameters before spawning qsv
5. **Provide context in errors** - include command, args, file paths
6. **Document examples** - every tool should have usage examples
7. **Clean up temporary files** - use ConvertedFileManager
8. **Match qsv conventions** - parameter names, flag styles, output formats
9. **Add guidance hints** - help Claude choose the right tool for the job
10. **Use spawn for execution** - streaming output prevents memory issues
11. **Use proper error typing** - use `error: unknown` with type guards instead of `error: any`

## Related Documentation

- [MCP Specification](https://modelcontextprotocol.io/)
- [qsv Main Documentation](../../README.md)
- [qsv Project CLAUDE.md](../../CLAUDE.md) - Main qsv development guide (build commands, architecture, code conventions)
- [qsv Command Reference](../../docs/)
- [Agent Skills Design](docs/design/AGENT_SKILLS_DESIGN.md)
- [Agent Skills Integration](docs/design/AGENT_SKILLS_INTEGRATION.md)
- [Agent Skills POC Summary](docs/design/AGENT_SKILLS_POC_SUMMARY.md)
- [Agent Skills Complete Summary](docs/design/AGENT_SKILLS_COMPLETE_SUMMARY.md)
- [Filesystem Changelog](docs/design/CHANGELOG_FILESYSTEM.md)
- [MCP Server README](./README-MCP.md)
- [Desktop Extension README](./README-MCPB.md)
- [Claude Desktop Integration](https://claude.ai/docs)

---

**Document Version**: 1.7
**Last Updated**: 2026-01-26
**Target qsv Version**: 15.x
**Node.js Version**: >=18.0.0
**MCP SDK Version**: ^1.25.2
**Maintainer**: Joel Natividad
