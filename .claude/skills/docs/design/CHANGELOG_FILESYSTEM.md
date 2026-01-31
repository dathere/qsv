# Filesystem Access Enhancement - Changelog

## Summary

Enhanced the QSV MCP Server to support **direct local filesystem access**, eliminating the need to upload CSV files to Claude Desktop.

## Changes Made

### New Files

1. **`src/mcp-filesystem.ts`** - New filesystem resource provider
   - Provides secure file browsing capabilities
   - Path canonicalization and validation using `fs.realpath()`
   - File preview generation
   - Security restrictions for directory access
   - Cross-platform file URI generation
   - Platform-aware path delimiter handling

3. **`FILESYSTEM_USAGE.md`** - Comprehensive usage guide
   - Configuration instructions
   - Security features documentation
   - Complete workflow examples
   - Troubleshooting guide

4. **`QUICK_START_LOCAL_FILES.md`** - Quick reference guide
   - Fast setup instructions
   - Common prompts
   - Configuration examples

5. **`CHANGELOG_FILESYSTEM.md`** - This file

### Modified Files

1. **`src/mcp-server.ts`**
   - Integrated `FilesystemResourceProvider`
   - Added environment variable support (`QSV_WORKING_DIR`, `QSV_ALLOWED_DIRS`)
   - Added three new MCP tools:
     - `qsv_list_files` - Browse CSV files
     - `qsv_set_working_dir` - Change working directory
     - `qsv_get_working_dir` - Get current working directory
   - Updated resource handlers to combine filesystem and example resources
   - Enhanced tool call handler to pass filesystem provider

2. **`src/mcp-tools.ts`**
   - Added `filesystemProvider` parameter to `handleToolCall()`
   - Implemented automatic path resolution for `input_file` and `output_file`
   - Added error handling for path resolution

3. **`README.md`**
   - Added prominent section about local file access
   - Updated overview to mention filesystem access
   - Added links to new documentation

## New Features

### 1. Browse Local CSV Files
```
User: "List CSV files in my Downloads folder"
```

Claude uses `qsv_list_files` to show available files without requiring uploads.

### 2. Set Working Directory
```
User: "Set working directory to ~/Downloads"
```

All subsequent relative file paths are resolved from this directory.

### 3. Automatic Path Resolution
```
User: "Show me stats for data.csv"
```

The MCP server automatically resolves `data.csv` relative to the working directory and validates access permissions.

### 4. Security Features

- **Path Validation**: Only files within `QSV_ALLOWED_DIRS` can be accessed
- **Directory Traversal Protection**: Prevents `../../../etc/passwd` style attacks
- **Extension Filtering**: Only CSV-related files (`.csv`, `.tsv`, `.tab`, `.ssv`, Snappy-compressed formats, Excel, ODS, JSONL/NDJSON) are listed
- **Preview Limits**: Maximum 1MB preview size, 20 lines max

### 5. Resource Browser Integration

Local CSV files appear in Claude Desktop's resource browser:
- Browse files visually
- See file previews
- Get file metadata (size, modification time)
- Copy file paths for use in prompts

## Configuration

### Environment Variables

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["path/to/dist/mcp-server.js"],
      "env": {
        "QSV_WORKING_DIR": "/Users/me/data",
        "QSV_ALLOWED_DIRS": "/Users/me/data:/Users/me/Downloads"
      }
    }
  }
}
```

**`QSV_WORKING_DIR`**: Default directory for relative paths
**`QSV_ALLOWED_DIRS`**: Colon-separated list of accessible directories

## Usage Examples

### Before (File Upload Required)
```
User: [Uploads allegheny_county_property_sale_transactions.csv]
User: "Analyze this file"
```

### After (No Upload Needed)
```
User: "Set working directory to ~/Downloads"
User: "What CSV files are available?"
User: "Analyze allegheny_county_property_sale_transactions.csv"
```

## Benefits

| Feature | Before | After |
|---------|--------|-------|
| File access | Upload required | Direct filesystem access |
| Upload time | Minutes for large files | Instant |
| File size limit | Limited by upload | No limit (qsv can handle GBs) |
| File browsing | Manual | Automated with `qsv_list_files` |
| Security | Upload validation | Path validation + directory whitelisting |
| Resource usage | Temporary storage | Direct file access |
| Multi-file workflows | Upload each file | Browse and select from filesystem |

## Breaking Changes

**None** - This is a backward-compatible enhancement. Existing functionality remains unchanged.

## Migration Guide

### For New Users

1. Follow [QUICK_START.md](../guides/QUICK_START.md)
2. Configure `QSV_WORKING_DIR` and `QSV_ALLOWED_DIRS`
3. Restart Claude Desktop
4. Start using local file paths in prompts

### For Existing Users

Your existing MCP server configuration will continue to work. To enable filesystem access:

1. Add environment variables to your configuration:
   ```json
   "env": {
     "QSV_WORKING_DIR": "/path/to/your/data",
     "QSV_ALLOWED_DIRS": "/path/to/your/data:/other/paths"
   }
   ```

2. Restart Claude Desktop

3. Start using the new tools:
   - `qsv_list_files`
   - `qsv_set_working_dir`
   - `qsv_get_working_dir`

## Technical Details

### Architecture

```
┌─────────────────────┐
│  Claude Desktop     │
│                     │
│  User asks about    │
│  local CSV file     │
└──────────┬──────────┘
           │
           │ MCP Protocol
           ▼
┌─────────────────────┐
│  QSV MCP Server     │
│                     │
│  ┌───────────────┐  │
│  │ Filesystem    │  │
│  │ Provider      │  │
│  │               │  │
│  │ - List files  │  │
│  │ - Resolve     │  │
│  │   paths       │  │
│  │ - Validate    │  │
│  │   access      │  │
│  └───────┬───────┘  │
│          │          │
│  ┌───────▼───────┐  │
│  │ Tool Handler  │  │
│  │               │  │
│  │ - qsv_stats   │  │
│  │ - qsv_headers │  │
│  │ - etc.        │  │
│  └───────┬───────┘  │
└──────────┼──────────┘
           │
           │ Execute qsv
           ▼
┌─────────────────────┐
│  Local Filesystem   │
│                     │
│  CSV files          │
└─────────────────────┘
```

### Path Resolution Flow

1. User provides file path (relative or absolute)
2. `FilesystemResourceProvider.resolvePath()` is called
3. Path is resolved relative to working directory if not absolute
4. Resolved path is validated against allowed directories
5. If valid, path is passed to qsv executor
6. qsv reads/processes the file
7. Results returned to Claude Desktop

### Security Model

- **Whitelist-based**: Only explicitly allowed directories are accessible
- **Path canonicalization**: Paths are resolved using `fs.realpath()` to handle symlinks and relative paths
- **Path traversal protection**: Canonical paths are validated to ensure they don't escape allowed directories
- **Working directory validation**: Setting a new working directory is only allowed within existing allowed directories
- **Case-sensitive validation**: Path comparisons respect filesystem case-sensitivity

**Security Limitations:**
- Users should avoid placing symlinks that point outside allowed directories within those directories
- Output file validation requires parent directory to exist
- The server has the same filesystem permissions as the Node.js process running it

## Performance Impact

- **Minimal**: Path resolution adds <1ms overhead
- **File browsing**: O(n) where n = number of files in directory
- **No caching**: Direct filesystem access, no intermediate storage
- **Memory**: Constant memory usage for path resolution

## Testing

Build and test:
```bash
cd .claude/skills
npm install
npm run build
```

The build should complete without errors, producing output in `dist/`.

## Future Enhancements

Potential future improvements:
- File watching for automatic resource refresh
- S3/cloud storage integration
- Remote file access via SSH/SFTP
- File upload fallback for unsupported paths
- Advanced filtering (by date, size, pattern)
- Caching for frequently accessed files

## Support

For issues or questions:
1. Check [FILESYSTEM_USAGE.md](../guides/FILESYSTEM_USAGE.md) for troubleshooting
2. Review [QUICK_START.md](../guides/QUICK_START.md) for setup
3. Verify your Claude Desktop configuration is valid JSON
4. Check MCP server logs in Claude Desktop developer console

## Version

- **Initial Release**: 2026-01-04
- **QSV Version**: 12.0.0
- **MCP SDK Version**: 1.25.1
