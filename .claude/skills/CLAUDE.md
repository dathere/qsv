# CLAUDE.md - Agent Skills Development Guide

This file provides guidance to Claude Code (claude.ai/code) when working with the qsv Agent Skills project.

## Project Overview

This is the **qsv Agent Skills** project - a TypeScript-based MCP (Model Context Protocol) server that exposes qsv's CSV data-wrangling capabilities to AI agents like Claude. It enables agents to discover, invoke, and compose qsv commands through a standardized protocol.

**Key Components**:
- **MCP Server**: Exposes qsv commands as MCP tools/resources
- **Converted File Manager**: Tracks converted CSV files with automatic cleanup
- **Pipeline System**: Composes multi-step qsv workflows
- **Update Checker**: Monitors qsv binary versions and updates
- **Type System**: Strong typing for qsv commands and parameters

**Goals**:
1. Make all 67+ qsv commands discoverable and invokable by AI agents
2. Auto-generate tool definitions from qsv usage text (zero documentation debt)
3. Enable intelligent composition of complex data workflows
4. Provide seamless integration with Claude Desktop and other MCP clients

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
│   ├── mcp-tools.ts       # MCP tool definitions and handlers
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
├── node_modules/          # Dependencies
├── package.json           # NPM package configuration
├── tsconfig.json          # TypeScript compiler config
└── CLAUDE.md              # This file
```

### Core Modules

#### `mcp-server.ts` - MCP Server Entry Point
- Implements Model Context Protocol server
- Handles stdio communication with MCP clients (Claude Desktop)
- Registers tools, resources, and prompts
- Manages server lifecycle

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

**Tool Structure**:
```typescript
{
  name: "qsv_select",
  description: "Select columns from CSV...",
  inputSchema: {
    type: "object",
    properties: {
      input_file: { type: "string", description: "..." },
      selection: { type: "string", description: "..." },
      // ... other parameters
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
- Spawns qsv child processes
- Handles stdin/stdout/stderr streaming
- Timeout management
- Error parsing and reporting

#### `config.ts` - Configuration System
- Environment variable loading
- qsv binary detection and validation
- Feature flag checking
- Performance tuning parameters

**Key Environment Variables**:
- `QSV_BIN_PATH`: Path to qsv binary
- `QSV_AUTOINDEX_SIZE`: Auto-index threshold
- `QSV_TEMP_DIR`: Temporary file location
- `QSV_MAX_WORKERS`: Parallelism limit

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
│  Validate args  │      │  Spawn qsv       │
└────────┬────────┘      └─────────┬────────┘
         │                         │
         │                         ▼
         │                  ┌─────────────┐
         │                  │ qsv binary  │
         │                  │ (Rust)      │
         │                  └─────────────┘
         │
         ▼
┌─────────────────┐
│ converted-file- │
│   manager.ts    │
│ Track outputs   │
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
     description: "Brief description from qsv usage text",
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

### Modifying Existing Tools

**IMPORTANT**: When updating tool definitions:
1. Always read the current qsv usage text first (`qsv yourcommand --help`)
2. Keep parameter names consistent with qsv flag names (snake_case)
3. Mark required vs optional parameters correctly
4. Include parameter validation (file existence, value ranges)
5. Update tests to cover new parameters

### Testing Conventions

- Each module has a corresponding test file: `tests/test_<module>.ts`
- Tests use Node.js built-in test runner (no external framework)
- Use `workdir` helper for creating temporary test directories
- Tests should clean up after themselves
- Integration tests should use real qsv binary

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

### Prompts (Future)

Can expose qsv workflows as reusable prompts:

```typescript
server.setRequestHandler(ListPromptsRequestSchema, async () => ({
  prompts: [
    {
      name: "data-cleaning-workflow",
      description: "Clean CSV data: remove duplicates, validate, sort"
    }
  ]
}));
```

## Performance Considerations

### File Size Thresholds

- Use `QSV_AUTOINDEX_SIZE` to automatically index large files
- Enable stats caching for files >100MB
- Consider memory limits when loading entire CSVs

### Parallelism

- Default to `--jobs` based on CPU count
- Limit concurrent qsv processes to prevent resource exhaustion
- Use Polars-powered commands (joinp, sqlp) for large datasets

### Caching Strategies

- **Stats Cache**: Reuse `.stats.csv.stats.jsonl` across tool calls
- **Index Files**: Preserve `.csv.idx` files between operations
- **Converted Files**: Cache Excel→CSV conversions

## Integration with Main qsv Project

### Dependency Management

This project depends on:
1. **qsv binary**: Must be in PATH or specified via `QSV_BIN_PATH`
2. **qsv version**: Should match package.json version (currently 13.0.0)
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

## Deployment

### MCPB Packaging

Create distributable MCP Bundle:

```bash
npm run mcpb:package
```

**Generated**:
- `qsv-mcp-server.mcpb`: Bundled server + dependencies
- `manifest.json`: Metadata for MCP registry
- Icons and assets

**Installation**:
Users can install via Claude Desktop or other MCP clients by pointing to the `.mcpb` file.

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
        "QSV_BIN_PATH": "/usr/local/bin/qsv"
      }
    }
  }
}
```

## Auto-Generation from qsv Usage Text

### Future Goals (See design docs)

The long-term vision is to auto-generate MCP tool definitions from qsv's USAGE text:

1. **Parser**: Extract from `static USAGE: &str` in Rust source
2. **Generator**: Create TypeScript tool definitions
3. **CI/CD**: Auto-update on qsv command changes
4. **Validation**: Ensure examples execute successfully

**Status**: Currently manual. See `docs/design/AGENT_SKILLS_DESIGN.md` for roadmap.

## Important Files

- **`package.json`**: Dependencies, scripts, versioning
- **`tsconfig.json`**: TypeScript compiler configuration
- **`src/mcp-server.ts`**: Main entry point
- **`src/mcp-tools.ts`**: Tool definitions (modify most frequently)
- **`src/config.ts`**: Environment and settings
- **`docs/design/AGENT_SKILLS_DESIGN.md`**: Architecture vision
- **`CLAUDE.md`**: This file

## Common Patterns

### Executing qsv Command

```typescript
import { spawn } from "child_process";

function executeQsv(args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    const qsv = spawn("qsv", args);
    let output = "";

    qsv.stdout.on("data", (data) => output += data.toString());
    qsv.stderr.on("data", (data) => console.error(data.toString()));

    qsv.on("close", (code) => {
      if (code === 0) resolve(output);
      else reject(new Error(`qsv exited with code ${code}`));
    });
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
- Set `QSV_BIN_PATH` environment variable
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

## Development Best Practices

1. **Always test with real qsv binary** - don't mock qsv in integration tests
2. **Keep tools simple** - one qsv command per MCP tool
3. **Use pipelines for composition** - chain simple tools, don't create mega-tools
4. **Validate early** - check file existence and parameters before spawning qsv
5. **Provide context in errors** - include command, args, file paths
6. **Document examples** - every tool should have usage examples
7. **Clean up temporary files** - use ConvertedFileManager
8. **Match qsv conventions** - parameter names, flag styles, output formats

## Related Documentation

- [MCP Specification](https://modelcontextprotocol.io/)
- [qsv Main Documentation](../../README.md)
- [qsv Command Reference](../../docs/)
- [Agent Skills Design](docs/design/AGENT_SKILLS_DESIGN.md)
- [Claude Desktop Integration](https://claude.ai/docs)

---

**Document Version**: 1.0
**Last Updated**: 2026-01-10
**Target qsv Version**: 13.0.0
**Maintainer**: Joel Natividad
