# QSV MCP Server

Model Context Protocol (MCP) server that exposes 55 of qsv's tabular data-wrangling commands to Claude Desktop.

## Overview

The QSV MCP Server enables Claude Desktop to interact with qsv through natural language, providing:

- **Deferred Tool Loading**: Only 8 core tools loaded initially (~85% token reduction), with tools discovered via search added dynamically
- **BM25 Search**: Intelligent tool discovery using probabilistic relevance ranking
- **Local File Access**: Works directly with your local tabular data files
- **Natural Language Interface**: No need to remember command syntax
- **Pipeline Support**: Chain multiple operations together seamlessly
- **Intelligent Guidance**: Enhanced tool descriptions help Claude make optimal decisions

## What's New

### Version 15.3.1
- **Also works with Gemini CLI!** - Google's terminal agent with MCP support - [Install in Gemini CLI](docs/guides/GEMINI_CLI.md)


### Version 15.3.0
- **BM25 Search Integration** - Upgraded `qsv_search_tools` from substring matching to BM25 relevance ranking
  - Uses `wink-bm25-text-search` for probabilistic information retrieval
  - Field-weighted search: name (3x), category (2x), description (1x), examples (0.5x)
  - Text preprocessing with stemming, lowercasing, and negation propagation
- **Deferred Tool Loading** - Implements Anthropic's Tool Search Tool pattern
  - Only 8 core tools loaded initially (reduces token usage ~85%)
  - Tools found via search are dynamically added to subsequent ListTools responses
  - Core tools: `qsv_search_tools`, `qsv_config`, `qsv_set_working_dir`, `qsv_get_working_dir`, `qsv_list_files`, `qsv_pipeline`, `qsv_command`, `qsv_to_parquet`
- **Removed `qsv_data_profile`** - Tool produced ~60KB output filling context window; use `qsv stats --cardinality --stats-jsonl` instead

### Version 15.1.1
- **Skill Version Sync** - Updated all 55 skill JSON files to version 15.1.1

### Version 15.1.0
- **Simplified Tool Guidance** - Removed redundant feature requirement hints (Polars, Luau) from tool descriptions
- **DuckDB Fallback** - Added guidance to use DuckDB as an alternative when sqlp encounters errors with complex queries
- **Expanded Error Prevention** - Added cat, dedup, sort, and searchset to commands with common mistake warnings
- **Streamlined Descriptions** - Removed verbose optimization hints that are now handled automatically

### Version 15.0.0
- **Tool Search Support** - New `qsv_search_tools` for discovering qsv commands by keyword, category, or regex
- **US Census MCP Integration** - Census MCP server awareness with integration guides
- **Streaming Executor** - Uses `spawn` instead of `execFileSync` for better output handling
- **Output Size Limits** - 50MB stdout limit prevents memory issues on large outputs
- **Token Optimization** - 66-76% reduction in tool description token usage
- **Windows EPERM Retry Logic** - Exponential backoff for file locking errors
- **Also works with Claude Code!** - wrangle data while wrangling code - [Install in Claude Code](docs/guides/CLAUDE_CODE.md)

## Supported File Formats

The MCP server works with all tabular data formats supported by qsv:

**Native formats** (direct processing):
- CSV (`.csv`), TSV (`.tsv`, `.tab`), SSV (`.ssv`)
- Snappy-compressed (`.csv.sz`, `.tsv.sz`, `.tab.sz`, `.ssv.sz`)

**Auto-converted formats** (transparent conversion):
- **Excel**: `.xls`, `.xlsx`, `.xlsm`, `.xlsb` → converted via `qsv excel`
- **OpenDocument**: `.ods` → converted via `qsv excel`
- **JSONL/NDJSON**: `.jsonl`, `.ndjson` → converted via `qsv jsonl`

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
- **Auto-detects qsv** - Finds your qsv installation or offers to download it
- **Cross-platform** - Works on macOS, Windows, and Linux
- **Secure** - Uses `spawn` with array arguments to prevent command injection
- **Template Variables** - Supports `$HOME`, `${HOME}` in config paths

See the [MCP Bundle documentation](./docs/desktop/README-MCPB.md) for detailed instructions.

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

1. **qsv** must be installed:
   ```bash
   # macOS
   brew install qsv

   # Or use mise (https://mise.jdx.dev)
   mise use -g ubi:dathere/qsv

   # Or download from https://github.com/dathere/qsv/releases
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
           "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
           "QSV_MCP_WORKING_DIR": "/Users/your-username/Downloads",
           "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/Downloads:/Users/your-username/Documents",
           "QSV_MCP_CONVERTED_LIFO_SIZE_GB": "1",
           "QSV_MCP_OPERATION_TIMEOUT_MS": "600000",
           "QSV_MCP_MAX_FILES_PER_LISTING": "1000",
           "QSV_MCP_MAX_PIPELINE_STEPS": "50",
           "QSV_MCP_MAX_CONCURRENT_OPERATIONS": "10"
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

> **SECURITY**: The `QSV_MCP_BIN_PATH` environment variable should only point to a trusted qsv binary.
> The MCP server executes this binary with user-provided file paths, so ensure it points to the
> official qsv installation and is not writable by untrusted users.

3. **Restart Claude Desktop**

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `QSV_MCP_BIN_PATH` | `qsv` | Path to qsv binary |
| `QSV_MCP_WORKING_DIR` | Current directory | Working directory for relative paths |
| `QSV_MCP_ALLOWED_DIRS` | None | Colon-separated (semicolon on Windows) list of allowed directories |
| `QSV_MCP_CONVERTED_LIFO_SIZE_GB` | `1` | Maximum size for converted file cache (0.1-100 GB) |
| `QSV_MCP_OPERATION_TIMEOUT_MS` | `600000` | Operation timeout in milliseconds (1s-30min, default 10 minutes) |
| `QSV_MCP_MAX_FILES_PER_LISTING` | `1000` | Maximum files to return in a single listing (1-100k) |
| `QSV_MCP_MAX_PIPELINE_STEPS` | `50` | Maximum steps in a pipeline (1-1000) |
| `QSV_MCP_MAX_CONCURRENT_OPERATIONS` | `10` | Maximum concurrent operations (1-100) |
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | `false` | Automatically regenerate skills when qsv version changes |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | `true` | Check for updates when MCP server starts |
| `QSV_MCP_NOTIFY_UPDATES` | `true` | Show update notifications in logs |
| `QSV_MCP_GITHUB_REPO` | `dathere/qsv` | GitHub repository to check for releases |
| `QSV_MCP_EXPOSE_ALL_TOOLS` | unset | Controls tool exposure mode. `true`: expose all 55+ tools immediately (no deferred loading). `false`: use only 8 core tools (no deferred additions). Unset (default): use deferred loading (8 core tools + tools discovered via search) |

**Resource Limits**: The server enforces limits to prevent resource exhaustion and DoS attacks. These limits are configurable via environment variables but have reasonable defaults for most use cases.

**Auto-Update**: The server includes built-in update detection and can automatically regenerate skills when qsv is updated. See [docs/reference/AUTO_UPDATE.md](./docs/reference/AUTO_UPDATE.md) for details.

## Available Tools

### 8 Core Tools (Always Loaded)

These tools are always available immediately:

| Tool | Description |
|------|-------------|
| `qsv_search_tools` | Search for qsv tools by keyword, category, or regex (BM25-powered) |
| `qsv_config` | Display current configuration |
| `qsv_set_working_dir` | Change working directory for file operations |
| `qsv_get_working_dir` | Get current working directory |
| `qsv_list_files` | List tabular data files in a directory |
| `qsv_pipeline` | Chain multiple qsv operations together |
| `qsv_command` | Execute any of the 55 qsv commands |
| `qsv_to_parquet` | Convert CSV to Parquet format |

### 13 Common Command Tools (Loaded on Demand)

Tools for frequently used commands, loaded when discovered via search:

| Tool | Description |
|------|-------------|
| `qsv_select` | Column selection (most frequently used) |
| `qsv_stats` | Statistical analysis (creates cache) |
| `qsv_moarstats` | Comprehensive statistics with data type inference |
| `qsv_index` | Create index for fast random access |
| `qsv_search` | Pattern-based filtering |
| `qsv_frequency` | Value distribution |
| `qsv_headers` | Header operations |
| `qsv_count` | Row counting (instant with index) |
| `qsv_slice` | Row selection |
| `qsv_sqlp` | SQL queries (Polars engine) |
| `qsv_joinp` | High-performance joins (Polars engine) |
| `qsv_cat` | Concatenate CSV files |
| `qsv_geocode` | Geocoding operations |

### Generic Command Tool

`qsv_command` - Execute any of the 55 qsv commands:
- `to`, `tojsonl`, `partition`, `pseudo`, `reverse`, `sniff`, `sort`, `dedup`, `join`, `apply`, `rename`, `validate`, `sample`, `template`, `diff`, `schema`, etc.
- Full list: https://github.com/dathere/qsv#commands

### Additional Utility Tools

- `qsv_welcome` - Welcome message and quick start guide
- `qsv_examples` - Show common usage examples

### Pipeline Tool

`qsv_pipeline` - Chain multiple operations together:
```
User: "Remove duplicates from sales.csv, then calculate statistics on the revenue column"

Claude executes pipeline:
1. qsv dedup
2. qsv stats -s revenue
```

## Tool Search and Deferred Loading

The MCP server implements Anthropic's Tool Search Tool pattern for optimal token efficiency:

### Deferred Loading (Default)

Only 8 core tools are loaded initially, reducing token usage by ~85%:

| Core Tool | Purpose |
|-----------|---------|
| `qsv_search_tools` | Find tools by keyword, category, or regex (BM25-powered) |
| `qsv_config` | View current configuration |
| `qsv_set_working_dir` | Change working directory |
| `qsv_get_working_dir` | Get current working directory |
| `qsv_list_files` | List tabular data files |
| `qsv_pipeline` | Chain multiple operations |
| `qsv_command` | Execute any qsv command |
| `qsv_to_parquet` | Convert CSV to Parquet format |

When Claude searches for tools, discovered tools are dynamically added to subsequent ListTools responses.

### BM25 Search

The `qsv_search_tools` tool uses probabilistic BM25 relevance ranking:
- **Field weighting**: name (3x), category (2x), description (1x), examples (0.5x)
- **Text preprocessing**: stemming, lowercasing, negation propagation
- **Smart fallback**: substring search if BM25 index not yet built

### Manual Override
Use `QSV_MCP_EXPOSE_ALL_TOOLS` environment variable to override deferred loading:
- `true`: Always expose all 55+ tools immediately (no deferred loading)
- `false`: Always use 8 core tools only (disables deferred loading)
- Unset: Default behavior - 8 core tools with deferred loading (recommended)

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

**Available Categories**: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, analysis, utility

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

### Example 3: Data Cleaning Pipeline

```
User: "Clean sales.csv by removing duplicates, then sort by revenue descending, then take top 100"

Claude calls: qsv_pipeline
Parameters:
  input_file: "sales.csv"
  steps: [
    { command: "dedup" },
    { command: "sort", params: { columns: "revenue", reverse: true } },
    { command: "slice", params: { start: 0, end: 100 } }
  ]

Result: Cleaned, sorted, and sliced CSV
```

### Example 4: Using Generic Tool

```
User: "Convert data.csv to Parquet format"

Claude calls: qsv_command
Parameters:
  command: "to"
  input_file: "data.csv"
  args: { output: "data.parquet" }

Result: Parquet file created
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
│  • 8 Core Tools (always loaded)            │
│  • 55+ tools via deferred loading          │
│  • BM25-powered tool search                │
│  • Enhanced descriptions & guidance        │
│  • Local file access & validation          │
│  • Format auto-detection & conversion      │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│          qsv Binary                         │
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

MCP Skills stay in sync with qsv commands via `qsv --update-mcp-skill`:

- **Integrated Tool** - No separate binary needed (requires `mcp` feature flag)
- **Auto-Generation** - Parses qsv USAGE text to generate skill definitions
- **Performance Hints** - Extracts emoji legends (📇 indexed, 🤯 memory-intensive) from README
- **Token Optimized** - Concise descriptions extracted from README command table

To regenerate skills after updating qsv:
```bash
qsv --update-mcp-skill
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
- qsv binary not in PATH → Set `QSV_MCP_BIN_PATH` env var
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
Loaded 55 skills
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
│   ├── mcp-pipeline.ts       # Pipeline tool
│   ├── types.ts              # Type definitions
│   ├── loader.ts             # Skill loader
│   ├── executor.ts           # Skill executor
│   └── pipeline.ts           # Pipeline API
├── scripts/
│   ├── install-mcp.js        # Installation helper
│   └── package-mcpb.js       # MCPB packaging script
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

- **Server Startup**: < 100ms (55 skills loaded)
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
- **Binary Trust**: The `QSV_MCP_BIN_PATH` environment variable should only point to a trusted qsv binary from the official installation. Ensure the binary path is not writable by untrusted users.

## Future Enhancements

Potential additions for future versions:

1. **Streaming Results** - For very large outputs
2. **Inline CSV Support** - Process small CSV snippets without files
3. **Progress Updates** - Track progress of long-running operations
4. **Parallel Execution** - Run independent pipeline steps concurrently

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

**Updated**: 2026-02-06
**Version**: 16.0.0
**Tools**: 8 core tools initially (deferred loading), 55+ when discovered via search
**Skills**: 55 qsv commands
**Status**: Production Ready
