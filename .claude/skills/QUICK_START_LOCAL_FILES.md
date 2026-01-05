# Quick Start: Using Local Tabular Data Files with QSV MCP Server

## 1. Configure Claude Desktop

Edit: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "/path/to/qsv/.claude/skills/dist/mcp-server.js"
      ],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/your-username/Downloads",
        "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/Downloads:/Users/your-username/Documents"
      }
    }
  }
}
```

**Important**: Update these paths to match your system:
- Replace `/path/to/qsv` with the actual path to your qsv installation
- Replace `/Users/your-username` with your home directory path
- On Windows, use semicolons (`;`) instead of colons (`:`) to separate directories in `QSV_MCP_ALLOWED_DIRS`
- On Windows, paths should look like `C:\\Users\\YourName\\Downloads`

## 2. Restart Claude Desktop

Close and reopen Claude Desktop.

## 3. Start Using Local Files

### Example Prompts:

**Browse files**:
```
List tabular data files in my Downloads folder
```

**Set working directory**:
```
Set working directory to ~/Downloads
```

**Analyze a file (no upload needed!)**:
```
What are the columns in allegheny_county_property_sale_transactions.csv?
```

**Get statistics**:
```
Show me statistics for the sale_price column in data.csv
```

**Work with relative paths**:
```
Show frequency distribution for the status column in customers.csv
```

## Key Benefits

✅ **No file uploads** - Direct filesystem access
✅ **Instant access** - No upload time
✅ **No size limits** - Process GB-sized files
✅ **Browse files** - See what's available before processing
✅ **Secure** - Only allowed directories are accessible

## New Tools Available

- `qsv_list_files` - Browse tabular data files in a directory (CSV, Excel, JSONL, etc.)
- `qsv_set_working_dir` - Change working directory
- `qsv_get_working_dir` - Check current working directory

All existing qsv tools (`qsv_stats`, `qsv_frequency`, etc.) now work with local file paths!

**Supported Formats**: CSV, TSV, SSV, Snappy-compressed, Excel (`.xls`, `.xlsx`, `.xlsm`, `.xlsb`), OpenDocument (`.ods`), JSONL/NDJSON

## Configuration Variables

| Variable | Purpose | Example (Unix) | Example (Windows) |
|----------|---------|----------------|-------------------|
| `QSV_MCP_BIN_PATH` | Path to qsv binary | `/usr/local/bin/qsv` | `C:\\Program Files\\qsv\\qsv.exe` |
| `QSV_MCP_WORKING_DIR` | Default directory for relative paths | `/Users/me/data` | `C:\\Users\\me\\data` |
| `QSV_MCP_ALLOWED_DIRS` | Delimited allowed directories | `/Users/me/data:/Users/me/downloads` | `C:\\Users\\me\\data;C:\\Users\\me\\downloads` |

**Note**: On Windows, use semicolons (`;`) to separate directories. On Unix/macOS, use colons (`:`).

## Need Help?

See [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md) for detailed documentation.
