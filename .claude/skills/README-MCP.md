# QSV MCP Server

Model Context Protocol (MCP) server that exposes qsv's tabular data-wrangling commands to Claude Desktop. The server works with **qsvmcp** (preferred) or the full **qsv** binary, exposing 53 skill-based commands optimized for AI agent workflows.

## Overview

The QSV MCP Server enables Claude Desktop to interact with qsv through natural language, providing:

- **Deferred Tool Loading**: 10 core tools loaded initially (~80% token reduction), plus 1 app-only tool (`qsv_browse_directory`) when MCP Apps are enabled. Additional tools are discovered via search and added dynamically.
- **BM25 Search**: Intelligent tool discovery using probabilistic relevance ranking
- **Local File Access**: Works directly with your local tabular data files
- **Natural Language Interface**: No need to remember command syntax
- **Intelligent Guidance**: Enhanced tool descriptions help Claude make optimal decisions

## Recommended Binary: qsvmcp

The **qsvmcp** binary variant is purpose-built for MCP server use. It includes only the 62 commands needed by the MCP server (vs 71 in the full qsv binary), resulting in a smaller, faster binary.

**Features included in qsvmcp**: Polars, Luau scripting, geocoding, self-update, MCP skill generation (`--update-mcp-skills`), and the `log` command for MCP audit logging.

**Commands excluded from qsvmcp** (not needed for MCP): `apply`, `clipboard`, `color`, `fetch`, `fetchpost`, `foreach`, `lens`, `prompt`, and `py` — 9 commands total.

| Binary | Commands | MCP Server Support | Notes |
|--------|----------|-------------------|-------|
| **qsvmcp** | 62 | Preferred | Optimized for MCP, smaller binary |
| **qsv** | 71 | Supported | Full-featured, includes extra commands not used by MCP |
| qsvlite | — | Not supported | Missing Polars and other required features |
| qsvdp | — | Not supported | DataPusher+ variant, missing required features |

To build qsvmcp from source:
```bash
cargo build --locked --bin qsvmcp -F qsvmcp
```

## Supported File Formats

The MCP server works with all tabular data formats supported by qsv:

**Native formats** (direct processing):
- CSV (`.csv`), TSV (`.tsv`, `.tab`), SSV (`.ssv`)
- Snappy-compressed (`.csv.sz`, `.tsv.sz`, `.tab.sz`, `.ssv.sz`)

**Auto-converted formats** (transparent conversion):
- **Excel**: `.xls`, `.xlsx`, `.xlsm`, `.xlsb` → converted via `qsv excel`
- **OpenDocument**: `.ods` → converted via `qsv excel`
- **JSONL/NDJSON**: `.jsonl`, `.ndjson` → converted via `qsv jsonl`

**Parquet support** (via sqlp/DuckDB):
- **Parquet**: `.parquet`, `.pq` → queryable via `qsv_sqlp` with `read_parquet()`
- **CSV → Parquet**: Use `qsv_to_parquet` (core tool) for optimized SQL queries on large files
- Parquet files work ONLY with `sqlp` and DuckDB — all other qsv commands require CSV/TSV/SSV input

Excel and JSONL files are automatically converted to CSV before processing - no extra steps needed!

## Installation

### Option 1: MCP Desktop Extension (Recommended)

The **MCP Desktop Extension** (MCPB) provides the easiest installation experience:

1. Download `qsv-mcp-server.mcpb` from [releases](https://github.com/dathere/qsv/releases/latest)
2. Open Claude Desktop Settings → Extensions
3. Click "Install from file" and select the `.mcpb` file
4. Configure your allowed directories when prompted
5. Restart Claude Desktop

The Desktop Extension:
- **Auto-detects qsvmcp/qsv** - Finds your qsvmcp or qsv installation, or offers to download it
- **Cross-platform** - Works on macOS, Windows, and Linux
- **Secure** - Uses `spawn` with array arguments to prevent command injection
- **Template Variables** - Supports `$HOME`, `${HOME}` in config paths

See the [MCP Bundle documentation](./docs/desktop/README-MCPB.md) for detailed instructions.
See the [Getting Started Guide](./docs/guides/START_HERE.md) for a walkthrough of using the MCP server with Claude Desktop.

### Option 2: Automated Installation (Developer)

```bash
git clone https://github.com/dathere/qsv.git
cd qsv/.claude/skills
npm install
npm run mcp:install
```

This script will:
1. Check for qsv binary
2. Build TypeScript
3. Update Claude Desktop config
4. Provide verification steps

### Option 3: Manual Installation

#### Prerequisites

1. **qsvmcp** (preferred) or **qsv** must be installed:
   ```bash
   # macOS (installs qsv with all variants including qsvmcp)
   brew install qsv

   # Or use mise (https://mise.jdx.dev)
   mise use -g ubi:dathere/qsv

   # Or download from https://github.com/dathere/qsv/releases

   # Or build qsvmcp from source
   cargo build --locked --bin qsvmcp -F qsvmcp
   ```

2. **Node.js** >= 18.0.0

3. **Claude Desktop** installed

#### Steps

1. **Build the MCP server:**
   ```bash
   cd /path/to/qsv_repo/.claude/skills
   npm install
   npm run build
   ```

2. **Configure Claude Desktop:**

   Edit `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS):
   ```json
   {
     "mcpServers": {
       "qsv": {
         "command": "node",
         "args": ["/absolute/path/to/qsv/.claude/skills/dist/mcp-server.js"],
         "env": {
           "QSV_MCP_BIN_PATH": "/usr/local/bin/qsvmcp",
           "QSV_MCP_WORKING_DIR": "/Users/your-username/Downloads",
           "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/Downloads:/Users/your-username/Documents",
           "QSV_MCP_CONVERTED_LIFO_SIZE_GB": "1",
           "QSV_MCP_OPERATION_TIMEOUT_MS": "600000",
           "QSV_MCP_MAX_FILES_PER_LISTING": "1000",
           "QSV_MCP_MAX_CONCURRENT_OPERATIONS": "1"
         }
       }
     }
   }
   ```

   **Other platforms:**
   - Windows: `%APPDATA%\Claude\claude_desktop_config.json`
   - Linux: `~/.config/Claude/claude_desktop_config.json`

> **NOTE**: You can further customize qsv's behavior by taking advantage of the "env" section
> in "mcpServers" to add more QSV environment variables.

> **SECURITY**: The `QSV_MCP_BIN_PATH` environment variable should only point to a trusted qsvmcp or qsv binary.
> The MCP server executes this binary with user-provided file paths, so ensure it points to the
> official installation and is not writable by untrusted users.

3. **Restart Claude Desktop**

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `QSV_MCP_BIN_PATH` | `qsvmcp` (falls back to `qsv`) | Path to qsvmcp/qsv binary |
| `QSV_MCP_WORKING_DIR` | Current directory | Working directory for relative paths |
| `QSV_MCP_ALLOWED_DIRS` | None | Colon-separated (semicolon on Windows) list of allowed directories |
| `QSV_MCP_CONVERTED_LIFO_SIZE_GB` | `1` | Maximum size for converted file cache (0.1-100 GB) |
| `QSV_MCP_OPERATION_TIMEOUT_MS` | `600000` | Operation timeout in milliseconds (1s-30min, default 10 minutes) |
| `QSV_MCP_MAX_FILES_PER_LISTING` | `1000` | Maximum files to return in a single listing (1-100k) |
| `QSV_MCP_MAX_CONCURRENT_OPERATIONS` | `1` (`3` in plugin mode) | Maximum concurrent operations (1-100) |
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | `false` | Automatically regenerate skills when qsv version changes |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | `true` | Check for updates when MCP server starts |
| `QSV_MCP_NOTIFY_UPDATES` | `true` | Show update notifications in logs |
| `QSV_MCP_GITHUB_REPO` | `dathere/qsv` | GitHub repository to check for releases |
| `QSV_MCP_SERVER_INSTRUCTIONS` | (built-in) | Custom server instructions sent during MCP initialization. Overrides built-in workflow guidance. Leave empty for defaults. |
| `QSV_MCP_MAX_OUTPUT_SIZE` | `52428800` | Maximum output size in bytes (50MB) |
| `QSV_MCP_MAX_EXAMPLES` | `5` | Maximum examples in tool descriptions (0-20) |
| `QSV_MCP_PLUGIN_MODE` | unset | Force plugin mode (for Gemini CLI etc.) |
| `QSV_MCP_EXPOSE_ALL_TOOLS` | unset | Controls tool exposure mode. `true`: expose all 51+ tools immediately (no deferred loading). `false`: use only 10 core tools (+1 app-only tool when Apps enabled; no deferred additions). Unset (default): use deferred loading (10 core tools + tools discovered via search) |

**Resource Limits**: The server enforces limits to prevent resource exhaustion and DoS attacks. These limits are configurable via environment variables but have reasonable defaults for most use cases.

**Auto-Update**: The server includes built-in update detection and can automatically regenerate skills when qsv is updated. See [docs/reference/AUTO_UPDATE.md](./docs/reference/AUTO_UPDATE.md) for details.

## Available Tools

### 10 Core Tools (Always Loaded)

These tools are always available immediately:

| Tool | Description |
|------|-------------|
| `qsv_search_tools` | Search for qsv tools by keyword, category, or regex (BM25-powered) |
| `qsv_config` | Display current configuration |
| `qsv_set_working_dir` | Change working directory for file operations |
| `qsv_get_working_dir` | Get current working directory |
| `qsv_list_files` | List tabular data files in a directory |
| `qsv_log` | Write to the MCP audit log |
| `qsv_command` | Execute any qsv command with a skill definition |
| `qsv_to_parquet` | Convert CSV to Parquet format |
| `qsv_index` | Create index for fast random access |
| `qsv_stats` | Statistical analysis (creates stats cache) |

> **App-only tool:** `qsv_browse_directory` (interactive directory browser) is also available when `QSV_MCP_ENABLE_APPS=true` and the client supports MCP Apps UI.

### 13 Common Command Tools (Loaded on Demand)

Tools for frequently used commands, loaded when discovered via search:

| Tool | Description |
|------|-------------|
| `qsv_select` | Column selection (most frequently used) |
| `qsv_moarstats` | Comprehensive statistics with data type inference |
| `qsv_search` | Pattern-based filtering |
| `qsv_frequency` | Value distribution |
| `qsv_headers` | Header operations |
| `qsv_count` | Row counting (instant with index) |
| `qsv_slice` | Row selection |
| `qsv_sniff` | File format detection and metadata |
| `qsv_sqlp` | SQL queries (Polars engine) |
| `qsv_joinp` | High-performance joins (Polars engine) |
| `qsv_cat` | Concatenate CSV files |
| `qsv_geocode` | Geocoding operations |
| `qsv_describegpt` | AI-powered data description and documentation |

### Generic Command Tool

`qsv_command` - Execute any qsv command with a skill definition (53 commands):
- `to`, `tojsonl`, `partition`, `pseudo`, `reverse`, `sniff`, `sort`, `dedup`, `join`, `rename`, `validate`, `sample`, `template`, `diff`, `schema`, etc.
- Full list: https://github.com/dathere/qsv#commands

## Tool Search and Deferred Loading

The MCP server implements Anthropic's Tool Search Tool pattern for optimal token efficiency:

### Deferred Loading (Default)

Only 10 core tools are loaded initially, reducing token usage by ~80%:

| Core Tool | Purpose |
|-----------|---------|
| `qsv_search_tools` | Find tools by keyword, category, or regex (BM25-powered) |
| `qsv_config` | View current configuration |
| `qsv_set_working_dir` | Change working directory |
| `qsv_get_working_dir` | Get current working directory |
| `qsv_list_files` | List tabular data files |
| `qsv_log` | Write to the MCP audit log |
| `qsv_command` | Execute any qsv command |
| `qsv_to_parquet` | Convert CSV to Parquet format |
| `qsv_index` | Create index for fast random access |
| `qsv_stats` | Statistical analysis (creates stats cache) |

> `qsv_browse_directory` is also loaded when the client supports MCP Apps UI.

When Claude searches for tools, discovered tools are dynamically added to subsequent ListTools responses.

### BM25 Search

The `qsv_search_tools` tool uses probabilistic BM25 relevance ranking:
- **Field weighting**: name (3x), category (2x), description (1x), examples (0.5x)
- **Text preprocessing**: stemming, lowercasing, negation propagation
- **Smart fallback**: substring search if BM25 index not yet built

### Manual Override
Use `QSV_MCP_EXPOSE_ALL_TOOLS` environment variable to override deferred loading:
- `true`: Always expose all 51+ tools immediately (no deferred loading)
- `false`: Always use 10 core tools only (+1 app-only tool when MCP Apps available; disables deferred loading)
- Unset: Default behavior - 10 core tools (+1 app-only tool when MCP Apps available) with deferred loading (recommended)

### Built-in Tool Search (`qsv_search_tools`)

Search for qsv tools using BM25 relevance ranking:

```
User: "What tools can help me join two CSV files?"

Claude calls: qsv_search_tools
Parameters:
  query: "join"

Result (ranked by relevance):
  **qsv_join** [joining]
    Inner, outer, left, right, cross, anti & semi joins
    💡 Join CSV files (<50MB). For large/complex joins, use qsv_joinp.

  **qsv_joinp** [joining]
    Polars-powered joins for large files
    💡 Fast Polars-powered joins for large files (>50MB)
```

**Search Modes**:
- **BM25 Keyword**: `query: "duplicate"` - relevance-ranked matches across names, descriptions, examples
- **Category Filter**: `category: "filtering"` - filter by category
- **Regex**: `query: "/sort|order/"` - use regex patterns for advanced matching

**Available Categories**: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, documentation, utility

**Note**: Tools found via search are automatically added to Claude's available tools for the session.

### Anthropic API Integration

For clients using the Anthropic API directly, configure Tool Search:

```json
{
  "tool_choice": {
    "type": "tool_search_tool_bm25_20251119",
    "defer_loading": true
  },
  "mcp_toolset": {
    "servers": [{
      "name": "qsv",
      "transport": {
        "type": "stdio",
        "command": "node",
        "args": ["/path/to/mcp-server.js"]
      }
    }]
  }
}
```

With `defer_loading: true`, Claude discovers tools via search only when needed, reducing context usage.

## Enhanced Tool Descriptions

Tool descriptions include **intelligent contextual guidance** to help Claude make optimal decisions:

- **💡 USE WHEN** - Specific use-case recommendations (e.g., when to use `join` vs `joinp`)
- **📋 COMMON PATTERNS** - Workflow patterns showing command combinations
- **⚠️ CAUTION** - Warnings about memory limits, file size constraints
- **🚀 PERFORMANCE** - Index acceleration tips and cache strategies

Example for `qsv_dedup`:
```
💡 USE WHEN: Removing duplicate rows. Memory-intensive - loads entire CSV.
Good for small-medium files. For very large files (>1GB), use qsv_extdedup instead.

📋 COMMON PATTERN: Often followed by stats or frequency to analyze cleaned data:
dedup → stats to see distribution after removing duplicates.

⚠️ CAUTION: Memory-intensive - loads entire file. For files >1GB, this may
fail with OOM. Use qsv_extdedup for very large files.
```

## Usage Examples

### Example 1: Column Selection

```
User: "Select columns 1-5 from data.csv"

Claude calls: qsv_select
Parameters:
  input_file: "data.csv"
  selection: "1-5"

Result: CSV with columns 1-5
```

### Example 2: Statistical Analysis

```
User: "Calculate statistics for the price column in products.csv"

Claude calls: qsv_stats
Parameters:
  input_file: "products.csv"
  select: "price"

Result: Statistics (mean, median, min, max, etc.)
```

### Example 3: Converting to Parquet

```
User: "Convert data.csv to Parquet format"

Claude calls: qsv_to_parquet
Parameters:
  input_file: "data.csv"

Result: Parquet file created with optimized data types (data.parquet)
```

## Architecture

```
┌─────────────────────────────────────────────┐
│           Claude Desktop                    │
│  (Natural language interactions)           │
└──────────────────┬──────────────────────────┘
                   │ MCP Protocol (JSON-RPC 2.0)
┌──────────────────▼──────────────────────────┐
│          QSV MCP Server                     │
│  • 10 Core Tools (always loaded)            │
│  • 51+ tools via deferred loading          │
│  • BM25-powered tool search                │
│  • Enhanced descriptions & guidance        │
│  • Local file access & validation          │
│  • Format auto-detection & conversion      │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│     qsvmcp Binary (preferred) / qsv         │
│  (Tabular data processing on local         │
│   filesystem: CSV, TSV, SSV, Excel, JSONL) │
└─────────────────────────────────────────────┘
```

## Data Handling

### Input

- Tools accept `input_file` parameter (absolute or relative path)
- qsv reads directly from your local filesystem (supports CSV, TSV/TAB, SSV, Excel, JSONL, and more)
- No input file size limitations (qsv streams large files efficiently)
- Auto-indexing for files > 10MB improves performance

### Output

- Optional `output_file` parameter
- **If provided**: qsv writes to file, tool returns metadata
- **If omitted**:
  - Small outputs (≤ 850KB): Returned directly in chat
  - Large outputs (> 850KB): Automatically saved to working directory with timestamped filename

**Smart large file handling**: The server automatically detects when output would exceed Claude Desktop's limits and saves it to disk instead, preventing timeouts and memory issues.

**Format auto-detection**: qsv automatically handles different formats (CSV, TSV, SSV, Excel, JSONL) based on file extensions or content sniffing. Excel and JSONL files are automatically converted to CSV for processing.

### Stats Cache Auto-Generation

When running `qsv_stats`, the MCP server automatically enables `--stats-jsonl` to create cache files that speed up subsequent operations with "smart" commands (`frequency`, `schema`, `tojsonl`, `sqlp`, `joinp`, `pivotp`, `diff`, `sample`).

### File Paths

- Relative paths resolved from Claude Desktop's working directory
- Absolute paths recommended for clarity
- Use forward slashes on all platforms (macOS, Windows, Linux)

## Skills Auto-Update

MCP Skills stay in sync with qsv commands via `qsvmcp --update-mcp-skills` (or `qsv --update-mcp-skills`):

- **Integrated Tool** - No separate binary needed (requires `mcp` feature flag, included in qsvmcp)
- **Auto-Generation** - Parses qsv USAGE text to generate skill definitions
- **Performance Hints** - Extracts emoji legends (📇 indexed, 🤯 memory-intensive) from README
- **Token Optimized** - Concise descriptions extracted from README command table

To regenerate skills after updating qsvmcp/qsv:
```bash
qsvmcp --update-mcp-skills   # or: qsv --update-mcp-skills
cd .claude/skills && npm run build
```

## Troubleshooting

### Tools Not Appearing in Claude Desktop

1. Check Claude Desktop config:
   ```bash
   cat ~/Library/Application\ Support/Claude/claude_desktop_config.json
   ```

2. Verify path to mcp-server.js is absolute and correct

3. Restart Claude Desktop

### MCP Server Errors

Check Claude Desktop logs for errors:

**macOS:**
```bash
tail -f ~/Library/Logs/Claude/mcp*.log
```

**Common issues:**
- qsvmcp/qsv binary not in PATH → Set `QSV_MCP_BIN_PATH` env var (qsvmcp preferred)
- TypeScript not built → Run `npm run build`
- File permissions → Ensure qsv has read access to CSV files

### Test MCP Server Manually

```bash
cd /path/to/qsv/.claude/skills
npm run mcp:start
```

The server should start and log:
```
Loading QSV skills...
Loaded 53 skills
QSV MCP Server initialized successfully
QSV MCP Server running on stdio
```

Press Ctrl+C to stop.

## Development

### Project Structure

```
.claude/skills/
├── src/
│   ├── mcp-server.ts         # Main MCP server
│   ├── mcp-tools.ts          # Tool definitions with guidance
│   ├── mcp-filesystem.ts     # Filesystem resource provider
│   ├── mcp-sampling.ts       # MCP sampling support
│   ├── bm25-search.ts        # BM25 search index for tool discovery
│   ├── browse-directory.ts   # Directory browser (MCP Apps)
│   ├── config.ts             # Configuration and validation
│   ├── converted-file-manager.ts  # LIFO cache for converted files
│   ├── executor.ts           # Skill executor
│   ├── installer.ts          # Binary installer
│   ├── loader.ts             # Skill loader
│   ├── update-checker.ts     # Version detection and skill regeneration
│   ├── types.ts              # Type definitions
│   ├── duckdb.ts             # DuckDB integration for SQL queries
│   ├── utils.ts              # Utility functions
│   ├── version.ts            # Version management
│   ├── wink-bm25-text-search.d.ts  # BM25 type declarations
│   ├── wink-nlp-utils.d.ts   # NLP utils type declarations
│   ├── ui/                   # UI components
│   │   └── directory-picker-html.ts
│   └── index.ts              # Module exports
├── scripts/
│   ├── install-mcp.js        # Installation helper
│   ├── package-mcpb.js       # MCPB packaging script
│   ├── package-plugin.js     # Plugin packaging script
│   ├── cowork-setup.cjs      # Claude Cowork integration setup
│   └── run-tests.js          # Cross-platform test runner
├── mcp-config.json           # Config template
├── README-MCP.md             # This file
└── docs/
    ├── guides/               # User guides
    ├── reference/            # Technical reference
    └── desktop/README-MCPB.md  # Desktop Extension documentation
```

### Building

```bash
npm run build
```

### Testing

Test the server manually:
```bash
npm run mcp:start
```

Test with Claude Desktop:
1. Configure Claude Desktop (see Installation)
2. Restart Claude Desktop
3. Try commands like "select columns from data.csv"

Run automated tests:
```bash
npm test
```

## Performance

- **Server Startup**: < 100ms (53 skills loaded)
- **Tool Execution**: < 10ms overhead + qsv processing time
- **File Processing**: Depends on qsv performance (generally very fast)
- **Streaming**: Large files processed efficiently by qsv

## Security Considerations

- **Local Files Only**: qsv only accesses files on your local filesystem
- **Directory Restrictions**: Only allowed directories can be accessed
- **No Network Access**: MCP server does not make network requests
- **User Control**: Claude Desktop prompts before executing tools
- **Secure Execution**: Uses `spawn` with array arguments to prevent command injection
- **Sandboxing**: Consider running in restricted environment for untrusted data
- **Binary Trust**: The `QSV_MCP_BIN_PATH` environment variable should only point to a trusted qsvmcp or qsv binary from the official installation. Ensure the binary path is not writable by untrusted users.

## Future Enhancements

Potential additions for future versions:

1. **Streaming Results** - For very large outputs
2. **Inline CSV Support** - Process small CSV snippets without files
3. **Progress Updates** - Track progress of long-running operations

## Resources

- [QSV Documentation](https://github.com/dathere/qsv)
- [MCP Specification](https://modelcontextprotocol.io/)
- [Claude Desktop](https://claude.ai/desktop)
- [QSV Skills README](./README.md)
- [MCP Desktop Extension](./docs/desktop/README-MCPB.md)
- [Filesystem Usage Guide](./docs/guides/FILESYSTEM_USAGE.md)
- [Auto-Update Guide](./docs/reference/AUTO_UPDATE.md)

## Support

For issues or questions:

1. Check troubleshooting section above
2. Review Claude Desktop logs
3. Open issue at: https://github.com/dathere/qsv/issues

---

**Updated**: 2026-04-14
**Version**: 19.1.1
**Tools**: 10 core tools initially (+1 app-only), 53 when discovered via search
**Skills**: 53 qsv commands
**Status**: Production Ready
