# QSV MCP Server

Model Context Protocol (MCP) server that exposes qsv's 67 tabular data-wrangling commands to Claude Desktop.

## Overview

The QSV MCP Server enables Claude Desktop to interact with qsv through natural language, providing:

- **26 MCP Tools**: 13 common commands as individual tools + 1 generic tool + 1 pipeline tool + 1 search tool + 4 utility + 3 filesystem tools (or 62+ in expose-all mode)
- **Local File Access**: Works directly with your local tabular data files
- **Natural Language Interface**: No need to remember command syntax
- **Pipeline Support**: Chain multiple operations together seamlessly
- **Intelligent Guidance**: Enhanced tool descriptions help Claude make optimal decisions

## What's New in 14.0.0

- **MCP Desktop Extension (MCPB)** - One-click installation for Claude Desktop
- **Enhanced Tool Descriptions** - USE WHEN guidance, COMMON PATTERNS, and ERROR PREVENTION hints
- **Token Optimization** - Concise descriptions reduce token usage while maintaining accuracy
- **Stats Cache Auto-Generation** - Automatically creates stats cache for "smart" commands
- **Production-Ready CI/CD** - Comprehensive testing across Node.js 20, 22, 24 on all platforms
- **Also works with Claude Code!** - wrangle data while wrangling code - [Install in Claude Code](CLAUDE_CODE.md)

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

1. Download `qsv-mcp-server.mcpb` from [releases](https://github.com/dathere/qsv/releases/download/14.0.0/qsv-mcp-server-14.1.0.mcpb)
2. Open Claude Desktop Settings → Extensions
3. Click "Install from file" and select the `.mcpb` file
4. Configure your allowed directories when prompted
5. Restart Claude Desktop

The Desktop Extension:
- **Auto-detects qsv** - Finds your qsv installation or offers to download it
- **Cross-platform** - Works on macOS, Windows, and Linux
- **Secure** - Uses `execFileSync` to prevent command injection
- **Template Variables** - Supports `$HOME`, `${HOME}` in config paths

See the [MCP Bundle documentation](./README-MCPB.md) for detailed instructions.

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
           "QSV_MCP_OPERATION_TIMEOUT_MS": "120000",
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
| `QSV_MCP_OPERATION_TIMEOUT_MS` | `120000` | Operation timeout in milliseconds (1s-30min) |
| `QSV_MCP_MAX_FILES_PER_LISTING` | `1000` | Maximum files to return in a single listing (1-100k) |
| `QSV_MCP_MAX_PIPELINE_STEPS` | `50` | Maximum steps in a pipeline (1-1000) |
| `QSV_MCP_MAX_CONCURRENT_OPERATIONS` | `10` | Maximum concurrent operations (1-100) |
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | `false` | Automatically regenerate skills when qsv version changes |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | `true` | Check for updates when MCP server starts |
| `QSV_MCP_NOTIFY_UPDATES` | `true` | Show update notifications in logs |
| `QSV_MCP_GITHUB_REPO` | `dathere/qsv` | GitHub repository to check for releases |
| `QSV_MCP_EXPOSE_ALL_TOOLS` | `false` | Expose all 62+ qsv tools instead of just common 13. Enable for clients with tool search/deferred loading support |

**Resource Limits**: The server enforces limits to prevent resource exhaustion and DoS attacks. These limits are configurable via environment variables but have reasonable defaults for most use cases.

**Auto-Update**: The server includes built-in update detection and can automatically regenerate skills when qsv is updated. See [AUTO_UPDATE.md](./AUTO_UPDATE.md) for details.

## Available Tools

### 13 Common Command Tools

Individual MCP tools for the most frequently used commands:

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

`qsv_command` - Execute any of the remaining 49+ qsv commands not exposed as individual tools:
- `to`, `tojsonl`, `flatten`, `partition`, `pseudo`, `reverse`, `sniff`, `sort`, `dedup`, `join`, `apply`, `rename`, `validate`, `sample`, `template`, `diff`, `schema`, etc.
- Full list: https://github.com/dathere/qsv#commands

### Utility Tools

- `qsv_welcome` - Welcome message and quick start guide
- `qsv_config` - Display current configuration
- `qsv_examples` - Show common usage examples
- `qsv_search_tools` - Search for qsv tools by keyword or category

### Pipeline Tool

`qsv_pipeline` - Chain multiple operations together:
```
User: "Remove duplicates from sales.csv, then calculate statistics on the revenue column"

Claude executes pipeline:
1. qsv dedup
2. qsv stats -s revenue
```

### Filesystem Tools

- `qsv_list_files` - List tabular data files in a directory
- `qsv_set_working_dir` - Change working directory for file operations
- `qsv_get_working_dir` - Get current working directory

### Tool Search Tool

- `qsv_search_tools` - Search for qsv tools by keyword, category, or regex pattern

## Tool Search Support

The MCP server supports two modes for tool discovery:

### Standard Mode (Default)
Exposes 22 tools: 13 common commands + 1 generic + 1 pipeline + 4 utility + 3 filesystem tools.
Optimized for token efficiency in typical workflows.

### Expose All Tools Mode
When `QSV_MCP_EXPOSE_ALL_TOOLS=true`, exposes all 62+ qsv command tools individually.
Recommended for:
- Clients with native tool search (Anthropic API Tool Search Tool)
- Clients supporting deferred tool loading (`defer_loading: true`)
- Advanced workflows requiring direct access to specialized commands

### Built-in Tool Search (`qsv_search_tools`)

Search for qsv tools without exposing all tools:

```
User: "What tools can help me join two CSV files?"

Claude calls: qsv_search_tools
Parameters:
  query: "join"

Result:
  **qsv_join** [joining]
    Inner, outer, left, right, cross, anti & semi joins
    💡 Join CSV files (<50MB). For large/complex joins, use qsv_joinp.

  **qsv_joinp** [joining]
    Polars-powered joins for large files
    💡 Fast Polars-powered joins for large files (>50MB)
```

**Search Modes**:
- **Keyword**: `query: "duplicate"` - matches names, descriptions, examples
- **Category**: `query: "filter", category: "filtering"` - filter by category
- **Regex**: `query: "/sort|order/"` - use regex patterns for advanced matching

**Available Categories**: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, analysis, utility

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
│  • 13 Common Tools + 1 Generic + 1 Pipeline │
│  • 1 Search Tool + 4 Utility + 3 Filesystem │
│  • 62+ tools in expose-all mode            │
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
Loaded 67 skills
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
└── README-MCPB.md            # Desktop Extension documentation
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

- **Server Startup**: < 100ms (67 skills loaded)
- **Tool Execution**: < 10ms overhead + qsv processing time
- **File Processing**: Depends on qsv performance (generally very fast)
- **Streaming**: Large files processed efficiently by qsv

## Security Considerations

- **Local Files Only**: qsv only accesses files on your local filesystem
- **Directory Restrictions**: Only allowed directories can be accessed
- **No Network Access**: MCP server does not make network requests
- **User Control**: Claude Desktop prompts before executing tools
- **Secure Execution**: Uses `execFileSync` to prevent command injection
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
- [MCP Desktop Extension](./README-MCPB.md)
- [Filesystem Usage Guide](./FILESYSTEM_USAGE.md)
- [Auto-Update Guide](./AUTO_UPDATE.md)

## Support

For issues or questions:

1. Check troubleshooting section above
2. Review Claude Desktop logs
3. Open issue at: https://github.com/dathere/qsv/issues

---

**Updated**: 2026-01-19
**Version**: 14.2.0
**Tools**: 26 standard mode (13 common + 1 generic + 1 pipeline + 1 search + 4 utility + 3 filesystem) or 62+ in expose-all mode
**Skills**: 62 qsv commands
**Status**: ✅ Production Ready
