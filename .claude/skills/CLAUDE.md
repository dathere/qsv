# CLAUDE.md - Agent Skills Development Guide

This file provides guidance to Claude Code (claude.ai/code) when working with the qsv Agent Skills project.

## Project Overview

This is the **qsv Agent Skills** project - a TypeScript-based MCP (Model Context Protocol) server that exposes qsv's tabular data-wrangling capabilities to AI agents like Claude. It enables agents to discover, invoke, and compose qsv commands through a standardized protocol for processing CSV, TSV, Excel, JSONL, and other tabular data formats.

**Key Components**:
- **MCP Server**: Exposes qsv commands as MCP tools/resources
- **MCP Desktop Extension (MCPB)**: One-click installation bundle for Claude Desktop
- **Converted File Manager**: Tracks converted CSV files with automatic cleanup
- **Pipeline System**: Composes multi-step qsv workflows
- **Update Checker**: Monitors qsv binary versions and updates
- **Type System**: Strong typing for qsv commands and parameters
- **Guidance Enhancement**: Intelligent tool descriptions with USE WHEN, COMMON PATTERNS, and CAUTION hints

**Goals**:
1. Make all 67 qsv commands discoverable and invokable by AI agents
2. Auto-generate tool definitions from qsv usage text (zero documentation debt)
3. Enable intelligent composition of complex data workflows with multi-format support
4. Provide seamless integration with Claude Desktop and other MCP clients
5. Help Claude make optimal tool choices through enhanced descriptions
6. Support diverse tabular data formats (CSV, TSV, Excel, JSONL, SSV, etc.)

## What's New in 14.0.0

- **MCP Desktop Extension (MCPB)** - User-friendly one-click installation
- **Enhanced Tool Descriptions** - USE WHEN, COMMON PATTERNS, CAUTION guidance
- **Token Optimization** - Concise descriptions from README command table
- **Stats Cache Auto-Generation** - Automatically enables `--stats-jsonl`
- **Production CI/CD** - Testing across Node.js 20, 22, 24 on all platforms
- **Security Improvements** - `execFileSync` prevents command injection

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
# Run all tests (builds first)
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
│   ├── converted-file-manager.ts  # Tracks converted files
│   ├── config.ts          # Configuration and validation
│   ├── executor.ts        # qsv command execution
│   ├── types.ts           # TypeScript type definitions
│   ├── utils.ts           # Utility functions
│   ├── version.ts         # Version management
│   └── loader.ts          # Dynamic skill loading
├── dist/                   # Compiled JavaScript output
├── tests/                  # Test files (TypeScript)
├── examples/               # Example usage scripts
├── docs/                   # Design documentation
│   └── design/            # Architecture and design docs
├── scripts/                # Build and deployment scripts
│   ├── install-mcp.js     # Installation helper
│   └── package-mcpb.js    # MCPB packaging script
├── qsv/                    # Auto-generated skill JSON files
├── node_modules/          # Dependencies
├── package.json           # NPM package configuration
├── tsconfig.json          # TypeScript compiler config
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
- Manages server lifecycle
- Auto-enables `--stats-jsonl` for stats command

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

#### `converted-file-manager.ts` - File Lifecycle Management
- Tracks all files created during agent workflows
- Automatic cleanup of temporary files
- Maintains conversion history
- Supports file retention policies

**Key Features**:
- Tracks conversions (Excel → CSV, JSON → CSV, etc.)
- Automatic cleanup with configurable TTL
- File size monitoring
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

#### `executor.ts` - Command Execution
- Spawns qsv child processes using `execFileSync` (secure)
- Handles stdin/stdout/stderr streaming
- Timeout management
- Error parsing and reporting

#### `config.ts` - Configuration System
- Environment variable loading
- qsv binary detection and validation
- Feature flag checking
- Performance tuning parameters

**Key Environment Variables**:
- `QSV_MCP_BIN_PATH`: Path to qsv binary
- `QSV_MCP_WORKING_DIR`: Working directory for file operations
- `QSV_MCP_ALLOWED_DIRS`: Colon-separated list of allowed directories
- `QSV_MCP_OPERATION_TIMEOUT_MS`: Operation timeout (default: 120000)
- `QSV_MCP_MAX_PIPELINE_STEPS`: Max pipeline steps (default: 50)

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
│  Validate args  │      │  execFileSync    │
│  Add guidance   │      │  (secure spawn)  │
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

- Each module has a corresponding test file: `tests/test_<module>.ts`
- Tests use Node.js built-in test runner (no external framework)
- Use `workdir` helper for creating temporary test directories
- Tests should clean up after themselves
- Integration tests should use real qsv binary
- CI runs on Node.js 20, 22, and 24 across macOS, Windows, Linux

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
- Avoid `any` type - use unknown and type guards instead
- Define interfaces for all qsv command parameters
- Use discriminated unions for result types

**Example**:
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

### Parallelism

- Default to `--jobs` based on CPU count
- Limit concurrent qsv processes to prevent resource exhaustion
- Use Polars-powered commands (joinp, sqlp) for large datasets

### Caching Strategies

- **Stats Cache**: Auto-generated `.stats.csv.data.jsonl` files
- **Index Files**: Preserve `.csv.idx` files between operations
- **Converted Files**: Cache Excel→CSV conversions (LIFO cache with configurable size)

## Integration with Main qsv Project

### Dependency Management

This project depends on:
1. **qsv binary**: Must be in PATH or specified via `QSV_MCP_BIN_PATH`
2. **qsv version**: Should match package.json major version (currently 14)
3. **Feature flags**: Some tools require specific qsv features (Polars, etc.)

### Version Synchronization

- `package.json` version tracks qsv version
- `version.ts` reads qsv binary version at runtime
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
qsv --update-mcp-skill

# Then rebuild TypeScript
cd .claude/skills && npm run build
```

The skill generator:
- Parses qsv USAGE text using qsv-docopt
- Extracts concise descriptions from README command table
- Extracts performance hints (📇 indexed, 🤯 memory-intensive) from README
- Creates JSON skill files in `.claude/skills/qsv/`

## Deployment

### MCPB Packaging

Create distributable MCP Bundle:

```bash
npm run mcpb:package
```

**Generated**:
- `qsv-mcp-server.mcpb`: Bundled server + dependencies
- `manifest.json`: Metadata for MCP registry (spec v0.3)
- Icons and assets

**Installation**:
Users can install via Claude Desktop Extensions by pointing to the `.mcpb` file.

**Desktop Extension Features**:
- Auto-detects qsv binary or offers to download
- Template variable expansion (`$HOME`, `${HOME}`)
- Cross-platform support (macOS, Windows, Linux)
- Secure execution via `execFileSync`

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

Skill JSON files are **auto-generated** from qsv's USAGE text via the `qsv --update-mcp-skill` command:

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
./target/debug/qsv --update-mcp-skill

# Then rebuild TypeScript
cd .claude/skills && npm run build
```

**Token Optimization**:
- Skill descriptions use concise README text instead of verbose USAGE text
- Guidance hints help Claude select the right tool
- Full documentation available on-demand via `--help` flag

## Important Files

- **`package.json`**: Dependencies, scripts, versioning
- **`tsconfig.json`**: TypeScript compiler configuration
- **`manifest.json`**: MCP Bundle manifest (spec v0.3)
- **`src/mcp-server.ts`**: Main entry point
- **`src/mcp-tools.ts`**: Tool definitions with guidance enhancement
- **`src/config.ts`**: Environment and settings
- **`scripts/package-mcpb.js`**: MCPB packaging script
- **`../../src/mcp_skills_gen.rs`**: Rust skill generator (in main qsv repo)
- **`docs/design/AGENT_SKILLS_DESIGN.md`**: Architecture vision
- **`README-MCP.md`**: MCP Server documentation
- **`README-MCPB.md`**: Desktop Extension documentation
- **`CLAUDE.md`**: This file

## Common Patterns

### Executing qsv Command

```typescript
import { execFileSync } from "child_process";

function executeQsv(args: string[]): string {
  try {
    const result = execFileSync("qsv", args, {
      encoding: "utf-8",
      timeout: 120000,
      maxBuffer: 100 * 1024 * 1024 // 100MB
    });
    return result;
  } catch (error) {
    throw new Error(`qsv failed: ${error.message}`);
  }
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

**MCP connection issues**:
- Restart Claude Desktop
- Check MCP server logs in Claude Desktop developer console
- Verify configuration in `claude_desktop_config.json`

**MCPB installation issues**:
- Ensure manifest.json follows spec v0.3
- Check that qsv binary path is accessible
- Verify allowed directories exist

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
10. **Use execFileSync** - secure execution prevents command injection

## Related Documentation

- [MCP Specification](https://modelcontextprotocol.io/)
- [qsv Main Documentation](../../README.md)
- [qsv Command Reference](../../docs/)
- [Agent Skills Design](docs/design/AGENT_SKILLS_DESIGN.md)
- [MCP Server README](./README-MCP.md)
- [Desktop Extension README](./README-MCPB.md)
- [Claude Desktop Integration](https://claude.ai/docs)

---

**Document Version**: 1.2
**Last Updated**: 2026-01-13
**Target qsv Version**: 14.x
**Maintainer**: Joel Natividad
