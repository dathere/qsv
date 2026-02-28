# CLAUDE.md - Agent Skills Development Guide

This file provides guidance to Claude Code (claude.ai/code) when working with the qsv Agent Skills project.

## Project Overview

This is the **qsv Agent Skills** project - a TypeScript-based MCP (Model Context Protocol) server that exposes qsv's tabular data-wrangling capabilities to AI agents like Claude. It enables agents to discover, invoke, and compose qsv commands through a standardized protocol for processing CSV, TSV, Excel, JSONL, and other tabular data formats.

**Key Components**:
- **MCP Server**: Exposes qsv commands as MCP tools/resources
- **MCP Desktop Extension (MCPB)**: One-click installation bundle for Claude Desktop
- **Converted File Manager**: Tracks converted CSV files with automatic cleanup (LIFO cache)
- **Update Checker**: Monitors qsv binary versions and auto-regenerates skills
- **Type System**: Strong typing for qsv commands and parameters
- **Guidance Enhancement**: Intelligent tool descriptions with USE WHEN, COMMON PATTERNS, and CAUTION hints

**Goals**:
1. Make all 51 qsv commands discoverable and invocable by AI agents
2. Auto-generate tool definitions from qsv usage text (zero documentation debt)
3. Enable intelligent composition of complex data workflows with multi-format support
4. Provide seamless integration with Claude Desktop and other MCP clients
5. Help Claude make optimal tool choices through enhanced descriptions
6. Support diverse tabular data formats (CSV, TSV, Excel, JSONL, SSV, etc.)

See [CHANGELOG.md](./CHANGELOG.md) for version history and release notes.

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

# Test update checker
npm run test-update-checker

# Run tests with coverage
npm run test:coverage
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
│   ├── mcp-server.ts      # Main entry point (tools, resources, prompts)
│   ├── mcp-tools.ts       # Tool definitions with guidance enhancement
│   ├── mcp-filesystem.ts  # Filesystem operations via MCP
│   ├── converted-file-manager.ts  # LIFO cache for converted files
│   ├── config.ts          # Configuration and validation
│   ├── duckdb.ts          # DuckDB integration for SQL queries
│   ├── executor.ts        # qsv command execution (streaming)
│   ├── update-checker.ts  # Version detection and skill regeneration
│   ├── types.ts           # TypeScript type definitions
│   ├── utils.ts           # Utility functions
│   ├── version.ts         # Version management
│   ├── index.ts           # Package entry point
│   ├── loader.ts          # Dynamic skill loading and searching
│   ├── bm25-search.ts     # BM25 search index for tool discovery
│   ├── wink-bm25-text-search.d.ts  # Type declarations for wink-bm25
│   └── wink-nlp-utils.d.ts  # Type declarations for wink-nlp-utils
├── tests/                  # Test files (each module has <module>.test.ts)
│   └── test-helpers.ts     # Shared utilities (createTestDir, createTestCSV, QSV_AVAILABLE)
├── scripts/                # Build and deployment scripts
│   ├── install-mcp.js     # Installation helper
│   ├── package-mcpb.js    # MCPB packaging script
│   ├── cowork-setup.js    # Claude Cowork integration setup
│   └── run-tests.js       # Cross-platform test runner
├── qsv/                    # Auto-generated skill JSON files (51, targeting qsvmcp commands)
├── docs/                   # Guides, reference, design docs
├── .claude-plugin/plugin.json  # Claude Plugin manifest
├── .mcp.json               # MCP server config for plugin (server key: "qsv")
├── commands/               # Slash commands (5: data-profile, data-clean, csv-query, data-convert, data-join)
├── agents/                 # Subagents (data-analyst, data-wrangler)
├── skills/                 # Domain knowledge (csv-wrangling, data-quality, qsv-performance)
├── package.json            # Dependencies, scripts, versioning
├── tsconfig.json / tsconfig.test.json  # TypeScript compiler configs
├── manifest.json           # MCP Bundle manifest (spec v0.3)
├── CHANGELOG.md            # Release notes
└── ../../src/mcp_skills_gen.rs  # Rust skill generator (in main qsv repo)
```

### Core Modules

#### `mcp-server.ts` - MCP Server Entry Point
- Implements Model Context Protocol server
- Handles stdio communication with MCP clients (Claude Desktop)
- Registers tools, resources, and prompts
- Manages server lifecycle with graceful shutdown
- Auto-enables `--stats-jsonl` for stats command
- Integrates update checker for background version monitoring
- **Server instructions**: Provides cross-tool workflow guidance via MCP `initialize` response
- **Deferred tool loading**: Only 9 core tools loaded initially (~80% token reduction)
- **Environment-controlled exposure**: Use `QSV_MCP_EXPOSE_ALL_TOOLS=true` for all tools
- **Roots auto-sync**: `syncWorkingDirFromRoots()` runs on startup and on `RootsListChangedNotification`; manual override via `qsv_set_working_dir`, passing `"auto"` re-enables sync

**Key Constants**:
- `MAX_ROOTS_SYNC_RETRIES`: 3 — max retries for roots directory sync
- `SHUTDOWN_TIMEOUT_MS`: 2000 — graceful shutdown timeout (ms)
- `UPDATE_CHECK_TIMEOUT_MS`: 30_000 — background update check timeout (ms)

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
- `COMMON_COMMANDS`: 11 frequently-used commands (select, moarstats, search, frequency, headers, count, slice, sqlp, joinp, cat, geocode)
- `ALWAYS_FILE_COMMANDS`: 34 commands that always output to files
- `METADATA_COMMANDS`: 4 commands returning metadata (count, headers, index, sniff)
- `NON_TABULAR_COMMANDS`: 9 commands whose output is not tabular (skips TSV formatting)
- `BINARY_OUTPUT_FORMATS`: Set of binary formats (parquet, arrow, avro)
- `COMMAND_GUIDANCE`: `Record<string, CommandGuidance>` — Unified per-command guidance map consolidating when-to-use, common patterns, error prevention, complementary servers, and memory/index/mistake warnings into a single structure
- `LARGE_FILE_THRESHOLD_BYTES`: 10MB — files larger than this are auto-indexed (replaces `AUTO_INDEX_SIZE_MB`)
- `MAX_MCP_RESPONSE_SIZE`: 850KB — responses exceeding this are saved to file instead of returned inline

**Key Exported Functions**:
- `isBinaryOutputFormat(commandName, params)` - Detect if command output is binary (parquet/arrow/avro)
- `buildConversionArgs(...)` - Build conversion arguments for format transforms
- `detectDelimiter(filePath)` - Detect delimiter from file extension
- `parseCSVLine(line, delimiter)` - Parse a single CSV line into fields

#### `executor.ts` - Command Execution
- Spawns qsv child processes using `spawn` for streaming output
- Handles stdin/stdout/stderr with proper buffering
- **Output size limit**: 50MB to prevent memory issues
- **Timeout handling**: Configurable timeout (default 10 minutes via `config.operationTimeoutMs`) with graceful termination (SIGTERM → SIGKILL)
- **Help request detection**: Skips input validation for `--help`
- **Subcommand support**: First-class handling of commands with subcommands
- **Stats cache auto-generation**: Forces `--stats-jsonl` for stats command
- **Frequency JSONL auto-enable**: Auto-adds `--frequency-jsonl` for frequency command (version-guarded >= 16.1.0)
- Timeout management and error parsing

**Key Constants**:
- `FREQUENCY_JSONL_MIN_VERSION`: `"16.1.0"` — minimum qsv binary version supporting `--frequency-jsonl`
- `MAX_STDERR_SIZE`: 50MB — stderr buffer limit

**Key Exports**:
- `runQsvSimple(binPath, args, options?)` - Lightweight shared executor (default timeout: 60 seconds) with `onSpawn`/`onExit` callbacks for process tracking (used by update-checker)
- `SkillExecutor` class - Full-featured skill executor with validation, stats cache, and subcommand support (default timeout: 10 minutes via `config.operationTimeoutMs`, capped at 30 min max)

**Key Features**:
- Validates parameters before execution (unless `--help` requested)
- Builds shell-safe command arguments
- Extracts row counts from stderr for metadata
- Returns structured results with exit codes and timing
- Graceful process termination on timeout (exit code 124)

#### `utils.ts` - Shared Utility Functions
- `compareVersions(v1, v2)` - Semantic version comparison, strips pre-release/build metadata
- `formatBytes(bytes)` - Human-readable byte formatting
- `levenshteinDistance(str1, str2)` - Fuzzy string matching for file suggestions
- `findSimilarFiles(target, files, max)` - Find similar filenames sorted by edit distance
- `errorResult(message)` - Create MCP error response with `isError: true`
- `successResult(text)` - Create MCP success response

#### `update-checker.ts` - Version Management
- Detects qsv binary version at runtime
- Compares skill definition versions with binary version
- Checks GitHub releases for available updates
- Auto-regenerates skills when `autoRegenerateSkills` is enabled
- Persists version info in `.qsv-mcp-versions.json`

**Key Features**:
- Quick check (local only, no network)
- Full check (includes GitHub API call, repo configurable via `QSV_MCP_GITHUB_REPO`, default: `dathere/qsv`)
- `getMcpServerVersion()` imports `VERSION` constant from `version.ts` (which reads from package.json at runtime)
- `compareVersions()` shared from `utils.ts`
- Extension mode support (skips MCP server version checks)

#### `converted-file-manager.ts` - File Lifecycle Management
- LIFO (Last In, First Out) cache for converted files
- File locking to prevent race conditions
- Change detection via mtime, size, inode, and optional hash
- Cache corruption recovery with validation
- UUID-derived temp file names (16-char random hex) for security
- Secure permissions (0o600)
- **Windows EPERM retry logic**: Exponential backoff for file locking errors

**Key Features**:
- Tracks conversions (Excel → CSV, JSON → CSV, etc.)
- Automatic cleanup with configurable size limit (LIFO eviction)
- File size monitoring and performance metrics
- Conversion statistics

#### `config.ts` - Configuration System
- Environment variable loading with template expansion
- qsv binary detection and validation (5-second timeout)
- Available commands detection at runtime
- Working directory and allowed directories configuration
- Extension mode detection (`MCPB_EXTENSION_MODE`)
- Plugin mode detection (`CLAUDE_PLUGIN_ROOT` set AND `MCPB_EXTENSION_MODE` NOT enabled)
  - In plugin mode, directory security is relaxed (auto-expand `allowedDirs`)
  - Working directory defaults to `${PWD}` instead of `${DOWNLOADS}`

**Key Environment Variables**:
- `QSV_MCP_BIN_PATH`: Path to qsv binary
- `QSV_MCP_WORKING_DIR`: Working directory for file operations
- `QSV_MCP_ALLOWED_DIRS`: Colon-separated list of allowed directories
- `QSV_MCP_OPERATION_TIMEOUT_MS`: Operation timeout (default: 600000ms / 10 minutes)
- `QSV_MCP_MAX_FILES_PER_LISTING`: Max files in directory listing (default: 1000)
- `QSV_MCP_MAX_CONCURRENT_OPERATIONS`: Max concurrent ops (default: 3 in plugin mode, 1 otherwise)
- `QSV_MCP_CONCURRENCY_WAIT_TIMEOUT_MS`: Queue wait timeout for concurrency slots (default: 120000ms / 2 min, max: 600000)
- `QSV_MCP_MAX_OUTPUT_SIZE`: Max output size in bytes (default: 50MB)
- `QSV_MCP_MAX_EXAMPLES`: Max examples in tool descriptions (default: 5, range: 0-20)
- `QSV_MCP_CONVERTED_LIFO_SIZE_GB`: Cache size in GB (default: 1.0)
- `QSV_MCP_AUTO_REGENERATE_SKILLS`: Auto-regenerate on version change
- `QSV_MCP_CHECK_UPDATES_ON_STARTUP`: Check for updates at startup
- `QSV_MCP_NOTIFY_UPDATES`: Show update notifications
- `QSV_MCP_EXPOSE_ALL_TOOLS`: Controls tool exposure (`true`: all tools, `false`: core only, unset: deferred loading)
- `QSV_MCP_SERVER_INSTRUCTIONS`: Custom server instructions (default: empty)
- `QSV_MCP_PLUGIN_MODE`: Explicit plugin mode override (default: auto-detected)
- `QSV_MCP_DUCKDB_BIN_PATH`: Path to DuckDB binary (default: auto-detect from PATH)
- `QSV_MCP_USE_DUCKDB`: Enable/disable DuckDB routing for SQL queries (default: false)
- `QSV_MCP_OUTPUT_FORMAT`: Output format for tabular data - "tsv" (default, token-efficient) or "csv"
- `QSV_MCP_GITHUB_REPO`: GitHub repo for update checks (default: `dathere/qsv`, used in `update-checker.ts`)
- `MCPB_EXTENSION_MODE`: Desktop extension mode flag

#### `loader.ts` - Dynamic Skill Loading
- Loads skill definitions from JSON files in `qsv/` directory
- Provides skill searching and categorization
- Supports dynamic skill discovery at runtime

**Key Methods**:
- `loadAll()`: Load all skills from JSON files (uses `Promise.all` for parallel I/O)
- `load(skillName)`: Load a specific skill by name
- `loadByNames(skillNames)`: Load multiple skills by name, returns `Map<string, QsvSkill>`
- `search(query)`: Search skills by name, description, or category
- `getByCategory(category)`: Get skills filtered by category
- `getCategories()`: Get all available categories
- `getStats()`: Get statistics via single-pass accumulation (total skills, examples, options, args)
- `isAllLoaded()`: Check if all skills have been loaded
- `isBM25Indexed()`: Check if BM25 search index has been built

**Skill Categories** (10 categories):
- `selection` - Column selection and reordering
- `filtering` - Row filtering and search
- `transformation` - Data transformation and modification
- `aggregation` - Statistics and aggregation
- `joining` - Joining and merging datasets
- `validation` - Data validation and schema checking
- `formatting` - Output formatting and display
- `conversion` - Format conversion (CSV, JSON, Excel, etc.)
- `documentation` - AI-powered data documentation and description
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
   async function handleYourCommand(args: Record<string, unknown>): Promise<ToolResult> {
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

**Stats-Aware Guidance (📊)**:

Run `qsv stats --cardinality --stats-jsonl` first to understand data characteristics. Then read the resulting `.stats.csv` file (token-efficient CSV) rather than the `.stats.csv.data.jsonl`.

Here's how stats help each tool:

| Tool | What Stats Reveals | Why It Helps |
|------|---------------------|--------------|
| `joinp` | Join column cardinality | Optimal table order (smaller cardinality on right) |
| `frequency` | High-cardinality columns | Avoid huge output from ID/timestamp columns |
| `dedup` | Uniqueness per column | Skip dedup if key column is all-unique |
| `sort` | Sort order | Skip sorting if already sorted |
| `pivotp` | Pivot column cardinality | Avoid overly wide output (>1000 columns) |

The 📊 emoji marks stats-related guidance in tool descriptions.

### Modifying Existing Tools

**IMPORTANT**: When updating tool definitions:
1. Always read the current qsv usage text first (`qsv yourcommand --help`)
2. Keep parameter names consistent with qsv flag names (snake_case)
3. Mark required vs optional parameters correctly
4. Include parameter validation (file existence, value ranges)
5. Update tests to cover new parameters
6. Update guidance hints if behavior changes

**Parameter alias handling**: `buildSkillExecParams` skips `"input"` and `"output"` keys by default (they are aliases for `input_file`/`output_file` via `resolveParamAliases`). However, if a skill declares `--input` or `--output` as a distinct CLI option (flag), the key is allowed through automatically. Note: this check only matches long-form options (`--input`/`--output`), not short flags (`-i`/`-o`); a positional arg named `input` is always consumed as the input file path.

### Testing Conventions

- Each module has a corresponding test file: `tests/<module>.test.ts`
- Tests use Node.js built-in test runner (no external framework)
- Use `workdir` helper for creating temporary test directories
- Tests should clean up after themselves
- Integration tests should use real qsv binary
- CI runs on Node.js 20, 22, and 24 across macOS, Windows, Linux
- Cross-platform test runner (`scripts/run-tests.js`) handles glob expansion
- Common test utilities extracted to `tests/test-helpers.ts` (createTestDir, cleanupTestDir, createTestCSV, QSV_AVAILABLE)

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

MCP protocol integration (tool registration, resources, prompts) is implemented in `src/mcp-server.ts`. Read that file for the actual handlers.

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

- **Stats Cache**: `qsv stats --stats-jsonl` (auto-enabled by MCP server) creates `<FILESTEM>.stats.csv` and `<FILESTEM>.stats.csv.data.jsonl`. Prefer reading the `.stats.csv` file directly — it's a standard CSV that's far more token-efficient than the equivalent `.data.jsonl`. The `.data.jsonl` exists for programmatic use by qsv's "smart" commands internally.
- **Frequency Cache**: `qsv frequency` writes its output CSV (typically `<FILESTEM>.freq.csv` when `--output <FILESTEM>.freq.csv` is used; otherwise to stdout or an MCP-managed temp file). Adding `--frequency-jsonl` (auto-enabled by the MCP server) also writes the JSONL cache file `<FILESTEM>.freq.csv.data.jsonl`. Prefer reading the output `.freq.csv` directly — it's a standard CSV that's far more token-efficient than the equivalent `.data.jsonl`. The `.data.jsonl` contains per-column frequency distributions with ALL_UNIQUE/HIGH_CARDINALITY sentinels for programmatic use by qsv's "smart" commands internally. Not used when `--ignore-case`, `--no-trim`, or `--weight` are active.
- **Index Files**: Preserve `.csv.idx` files between operations
- **Converted Files**: Cache Excel→CSV conversions (LIFO cache with configurable size)
- **Version Cache**: `.qsv-mcp-versions.json` tracks version state

## Integration with Main qsv Project

### Dependency Management

This project depends on:
1. **qsv binary**: Must be in PATH or specified via `QSV_MCP_BIN_PATH`
2. **qsv version**: Should match package.json major version (currently 16)
3. **Feature flags**: Some tools require specific qsv features (Polars, etc.)
4. **Node.js**: Requires Node.js 18.0.0 or later
5. **MCP SDK**: Uses @modelcontextprotocol/sdk ^1.26.0

### Version Synchronization

- `package.json` version tracks qsv version
- `version.ts` reads qsv binary version at runtime
- `update-checker.ts` compares versions and suggests updates
- CI checks ensure compatibility

### Skills Auto-Generation

Skill JSON files are **auto-generated** from qsv's USAGE text via `qsv --update-mcp-skills`. The generator (`../../src/mcp_skills_gen.rs`, called from `../../src/main.rs`) parses `static USAGE: &str` with qsv-docopt, extracts concise descriptions and performance hints (📇 🤯 😣) from README, and creates JSON files in `.claude/skills/qsv/`. The `mcp-tools.ts` layer then adds guidance hints (when-to-use, patterns, cautions).

**Regenerating skills** (e.g., after qsv update):
```bash
# From qsv repo root
cargo build --bin qsv -F all_features
./target/debug/qsv --update-mcp-skills

# Then rebuild TypeScript
cd .claude/skills && npm run build
```

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

For implementation patterns (command execution, argument building, validation, output handling), refer to `src/executor.ts` and `src/mcp-tools.ts` which contain the actual implementations.

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

**qsv missing Polars**:
- The MCP server requires `qsvmcp` (preferred) or the full `qsv` binary with Polars enabled
- Only `qsvmcp` and `qsv` are supported — `qsvlite` and `qsvdp` are not
- Check with `qsvmcp --version` or `qsv --version` — look for `polars-X.Y.Z` in the feature list
- Install from https://github.com/dathere/qsv#installation

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
3. **Validate early** - check file existence and parameters before spawning qsv
5. **Provide context in errors** - include command, args, file paths
6. **Document examples** - every tool should have usage examples
7. **Clean up temporary files** - use ConvertedFileManager
8. **Match qsv conventions** - parameter names, flag styles, output formats
9. **Add guidance hints** - help Claude choose the right tool for the job
10. **Use spawn for execution** - streaming output prevents memory issues
11. **Use proper error typing** - use `error: unknown` with type guards instead of `error: any`

## Claude Plugin Structure

The plugin layer (`.claude-plugin/`, `.mcp.json`, `commands/`, `agents/`, `skills/`) is purely additive - no MCP server code changes needed. See directory structure above for layout.

**How it layers**: `.claude-plugin/plugin.json` declares the plugin and points to `.mcp.json` (server key `"qsv"`, tools become `mcp__qsv__qsv_*`). Commands orchestrate MCP tools into workflows. Agents (analyst/wrangler) have restricted tool lists. Skills provide concise domain knowledge reference tables.

**Key design decisions**: `QSV_MCP_EXPOSE_ALL_TOOLS=true` in plugin config since Claude Code/Cowork handle large tool lists well. Two separate agents rather than one monolithic agent for clearer boundaries.

## Related Documentation

- [MCP Specification](https://modelcontextprotocol.io/)
- [qsv Main Documentation](../../README.md)
- [qsv Project CLAUDE.md](../../CLAUDE.md) - Main qsv development guide (build commands, architecture, code conventions)
- [qsv Command Reference](../../docs/)

**Guides:**
- [Quick Start Guide](docs/guides/QUICK_START.md)
- [macOS Quick Start Guide](docs/guides/MACOS-QUICK_START.md)
- [Claude Code Integration](docs/guides/CLAUDE_CODE.md)
- [Gemini CLI Integration](docs/guides/GEMINI_CLI.md)
- [Desktop Extension Guide](docs/guides/DESKTOP_EXTENSION.md)
- [Filesystem Usage](docs/guides/FILESYSTEM_USAGE.md)

**Reference:**
- [Skills API](docs/reference/SKILLS_API.md)
- [Auto-Update Configuration](docs/reference/AUTO_UPDATE.md)
- [Update System](docs/reference/UPDATE_SYSTEM.md)
- [CI/CD Pipeline](docs/reference/CI.md)
- [Changelog](./CHANGELOG.md)

**Desktop Extension:**
- [MCP Server README](./README-MCP.md)
- [Desktop Extension README (MCPB)](docs/desktop/README-MCPB.md)

**Design Documents:**
- [Agent Skills Design](docs/design/AGENT_SKILLS_DESIGN.md)
- [Agent Skills Integration](docs/design/AGENT_SKILLS_INTEGRATION.md)
- [Agent Skills POC Summary](docs/design/AGENT_SKILLS_POC_SUMMARY.md)
- [Agent Skills Complete Summary](docs/design/AGENT_SKILLS_COMPLETE_SUMMARY.md)

**Audits:**
- [Audit Report 2026-02-22](docs/audits/AUDIT_REPORT_2026-02-22.md)

**External:**
- [Claude Desktop Integration](https://claude.ai/docs)

---

**Document Version**: 2.3
**Last Updated**: 2026-02-25
**Target qsv Version**: 16.x
**Node.js Version**: >=18.0.0
**MCP SDK Version**: ^1.26.0
**Maintainer**: Joel Natividad
