# QSV MCP Server

Model Context Protocol (MCP) server that exposes qsv's 66 CSV data-wrangling commands to Claude Desktop.

## Overview

The QSV MCP Server enables Claude Desktop to interact with qsv through natural language, providing:

- **22 MCP Tools**: 20 common commands as individual tools + 1 generic tool + 1 pipeline tool
- **Local File Access**: Works directly with your local tabular data files
- **Natural Language Interface**: No need to remember command syntax
- **Pipeline Support**: Chain multiple operations together seamlessly

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

### Prerequisites

1. **qsv** must be installed:
   ```bash
   # macOS
   brew install qsv

   # Or download from https://github.com/dathere/qsv/releases
   ```

2. **Node.js** >= 18.0.0

3. **Claude Desktop** installed

### Automated Installation

```bash
cd /path/to/qsv/.claude/skills
npm install
npm run mcp:install
```

This script will:
1. Check for qsv binary
2. Build TypeScript
3. Update Claude Desktop config
4. Provide verification steps

### Manual Installation

1. **Build the MCP server:**
   ```bash
   cd /path/to/qsv/.claude/skills
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

**Resource Limits**: The server enforces limits to prevent resource exhaustion and DoS attacks. These limits are configurable via environment variables but have reasonable defaults for most use cases.

3. **Restart Claude Desktop**

## Available Tools

### 20 Common Command Tools

Individual MCP tools for the most frequently used commands:

| Tool | Description |
|------|-------------|
| `qsv_select` | Column selection (most frequently used) |
| `qsv_stats` | Statistical analysis |
| `qsv_frequency` | Value distribution |
| `qsv_search` | Pattern-based filtering |
| `qsv_sort` | Sorting operations |
| `qsv_dedup` | Duplicate removal |
| `qsv_join` | CSV joining |
| `qsv_count` | Row counting |
| `qsv_headers` | Header operations |
| `qsv_slice` | Row selection |
| `qsv_apply` | Column transformations |
| `qsv_rename` | Column renaming |
| `qsv_schema` | Schema inference |
| `qsv_validate` | Data validation |
| `qsv_sample` | Random sampling |
| `qsv_moarstats` | Comprehensive statistics with data type inference |
| `qsv_index` | Create index for fast random access |
| `qsv_template` | Template-based transformations |
| `qsv_diff` | Compare two CSV files |
| `qsv_cat` | Concatenate CSV files |

### Generic Command Tool

`qsv_command` - Execute any of the remaining 46 qsv commands:
- `to`, `tojsonl`, `flatten`, `partition`, `pseudo`, `reverse`, `sniff`, etc.
- Full list: https://github.com/dathere/qsv#commands

### Pipeline Tool

`qsv_pipeline` - Chain multiple operations together:
```
User: "Remove duplicates from sales.csv, then calculate statistics on the revenue column"

Claude executes pipeline:
1. qsv dedup
2. qsv stats -s revenue
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
│  • 22 MCP Tools (commands)                  │
│  • 3 Filesystem Tools (list/browse files)  │
│  • Local file access & validation          │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│          qsv Binary                         │
│  (CSV processing on local filesystem)      │
└─────────────────────────────────────────────┘
```

## Data Handling

### Input

- Tools accept `input_file` parameter (absolute or relative path)
- qsv reads directly from your local filesystem
- No file size limitations (qsv streams large files efficiently)

### Output

- Optional `output_file` parameter
- **If provided**: qsv writes to file, tool returns metadata
- **If omitted**: qsv writes to stdout, tool returns CSV content (for small results)

### File Paths

- Relative paths resolved from Claude Desktop's working directory
- Absolute paths recommended for clarity
- Use forward slashes on all platforms (macOS, Windows, Linux)

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
Loaded 66 skills
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
│   ├── mcp-tools.ts          # Tool definitions
│   ├── mcp-filesystem.ts     # Filesystem resource provider
│   ├── mcp-pipeline.ts       # Pipeline tool
│   ├── types.ts              # Type definitions
│   ├── loader.ts             # Skill loader
│   ├── executor.ts           # Skill executor
│   └── pipeline.ts           # Pipeline API
├── scripts/
│   └── install-mcp.js        # Installation helper
├── mcp-config.json           # Config template
└── README-MCP.md             # This file
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

## Performance

- **Server Startup**: < 100ms (66 skills loaded)
- **Tool Execution**: < 10ms overhead + qsv processing time
- **File Processing**: Depends on qsv performance (generally very fast)
- **Streaming**: Large files processed efficiently by qsv

## Security Considerations

- **Local Files Only**: qsv only accesses files on your local filesystem
- **No Network Access**: MCP server does not make network requests
- **User Control**: Claude Desktop prompts before executing tools
- **Sandboxing**: Consider running in restricted environment for untrusted data
- **Binary Trust**: The `QSV_MCP_BIN_PATH` environment variable should only point to a trusted qsv binary from the official installation. Ensure the binary path is not writable by untrusted users.

## Future Enhancements

Potential additions for future versions:

1. **Prompt Templates** - Pre-built workflows (e.g., "data cleaning pipeline")
2. **Streaming Results** - For very large outputs
3. **Inline CSV Support** - Process small CSV snippets without files
4. **Progress Updates** - Track progress of long-running operations
5. **Stats Cache Integration** - Leverage qsv's stats cache
6. **Parallel Execution** - Run independent pipeline steps concurrently

## Resources

- [QSV Documentation](https://github.com/dathere/qsv)
- [MCP Specification](https://modelcontextprotocol.io/)
- [Claude Desktop](https://claude.ai/desktop)
- [QSV Skills README](./README.md)
- [Filesystem Usage Guide](./FILESYSTEM_USAGE.md)

## Support

For issues or questions:

1. Check troubleshooting section above
2. Review Claude Desktop logs
3. Open issue at: https://github.com/dathere/qsv/issues

---

**Updated**: 2026-01-04
**Version**: 13.0.0
**Tools**: 25 (20 common + 1 generic + 1 pipeline + 3 filesystem)
**Skills**: 66 qsv commands
**Status**: ✅ Production Ready
