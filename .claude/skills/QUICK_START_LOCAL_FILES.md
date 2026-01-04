# Quick Start: Using Local CSV Files with QSV MCP Server

## 1. Configure Claude Desktop

Edit: `~/Library/Application Support/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "/Users/joelnatividad/.claude-worktrees/qsv/frosty-lichterman/.claude/skills/dist/mcp-server.js"
      ],
      "env": {
        "QSV_WORKING_DIR": "/Users/joelnatividad/Downloads",
        "QSV_ALLOWED_DIRS": "/Users/joelnatividad/Downloads:/Users/joelnatividad/Documents"
      }
    }
  }
}
```

**Important**: Update the paths to match your system!

## 2. Restart Claude Desktop

Close and reopen Claude Desktop.

## 3. Start Using Local Files

### Example Prompts:

**Browse files**:
```
List CSV files in my Downloads folder
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

- `qsv_list_files` - Browse CSV files in a directory
- `qsv_set_working_dir` - Change working directory
- `qsv_get_working_dir` - Check current working directory

All existing qsv tools (`qsv_stats`, `qsv_frequency`, etc.) now work with local file paths!

## Configuration Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `QSV_WORKING_DIR` | Default directory for relative paths | `/Users/me/data` |
| `QSV_ALLOWED_DIRS` | Colon-separated allowed directories | `/Users/me/data:/Users/me/downloads` |

## Need Help?

See [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md) for detailed documentation.
