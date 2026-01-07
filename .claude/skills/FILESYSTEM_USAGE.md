# Using QSV MCP Server with Local Files

The QSV MCP Server supports **direct access to local tabular data files** WITHOUT requiring uploads to Claude servers - making for increased security and reduced token usage. This includes CSV, TSV, Excel, JSONL, and more formats.

## Supported File Formats

The MCP server recognizes and processes all tabular formats supported by qsv:

### Native Formats (Direct Processing)
- **CSV** (`.csv`) - Comma-separated values
- **TSV** (`.tsv`, `.tab`) - Tab-separated values
- **SSV** (`.ssv`) - Semicolon-separated values
- **Snappy-compressed** (`.csv.sz`, `.tsv.sz`, `.tab.sz`, `.ssv.sz`) - Compressed formats

Uncompressed CSV/TSV/TAB/SSV files larger than 10 MB are automatically indexed for increased performance.

### Formats Requiring Automatic Conversion

These formats are automatically converted to CSV before processing:

- **Excel files** (`.xls`, `.xlsx`, `.xlsm`, `.xlsb`) - Converted via `qsv excel`
- **OpenDocument Spreadsheet** (`.ods`) - Converted via `qsv excel`
- **JSONL/NDJSON** (`.jsonl`, `.ndjson`) - Converted via `qsv jsonl`

When you select an Excel or JSONL file, the MCP server automatically:
1. Detects the file format
2. Runs the appropriate conversion command (`qsv excel` or `qsv jsonl`)
3. Creates a `.converted.csv` file (e.g., `data.xlsx.converted.csv`)
4. Uses the CSV for processing
5. Returns results normally

**No extra steps required** - just use the file as you would a CSV!

**Automatic Management of Converted Files:**
- Converted `.converted.csv` files are automatically managed with a LIFO (Last In First Out) cleanup system
- If the source file hasn't changed, existing converted files are reused (timestamp comparison)
- When total size of all converted files exceeds the limit (default: 1GB), the oldest files are automatically deleted
- Configure the size limit with `QSV_MCP_CONVERTED_LIFO_SIZE_GB` environment variable (in GB)
- The cache tracks all converted files in `.qsv-mcp-converted-cache.json` in your working directory

## Quick Start

### 1. Configure Your Claude Desktop

Add the QSV MCP server to your Claude Desktop configuration with optional environment variables:

**Location**: `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS)

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
        "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/Downloads:/Users/your-username/Documents:/Users/your-username/data"
      }
    }
  }
}
```

**Platform-specific notes:**
- **macOS/Linux**: Use colons (`:`) to separate directories in `QSV_MCP_ALLOWED_DIRS`
- **Windows**: Use semicolons (`;`) to separate directories, and use double backslashes in paths (e.g., `C:\\Users\\YourName\\Downloads`)

### 2. Restart Claude Desktop

After updating the configuration, restart Claude Desktop for the changes to take effect.

## Configuration Options

### Environment Variables

#### `QSV_MCP_BIN_PATH`
- **Description**: Path to the qsv binary executable
- **Default**: `qsv` (assumes qsv is in PATH)
- **Example (Unix)**: `"/usr/local/bin/qsv"`
- **Example (Windows)**: `"C:\\Program Files\\qsv\\qsv.exe"`
- **Security Note**: This should only point to a trusted qsv binary from the official installation. Ensure the binary path is not writable by untrusted users.

#### `QSV_MCP_WORKING_DIR`
- **Description**: The default working directory for relative file paths
- **Default**: Current process directory
- **Example (Unix)**: `"/Users/your-username/Downloads"`
- **Example (Windows)**: `"C:\\Users\\YourName\\Downloads"`

#### `QSV_MCP_ALLOWED_DIRS`
- **Description**: Delimited list of directories that can be accessed (security feature)
- **Delimiter**: Colon (`:`) on Unix/macOS, semicolon (`;`) on Windows
- **Default**: Only the working directory
- **Example (Unix)**: `"/Users/your-username/Downloads:/Users/your-username/Documents"`
- **Example (Windows)**: `"C:\\Users\\YourName\\Downloads;C:\\Users\\YourName\\Documents"`

#### `QSV_MCP_CONVERTED_LIFO_SIZE_GB`
- **Description**: Maximum total size (in GB) of all `.converted.csv` files before LIFO cleanup
- **Default**: `1` (1 GB)
- **How it works**:
  - Excel and JSONL files are automatically converted to CSV for processing
  - Converted files are cached and reused if the source hasn't changed
  - When total size exceeds this limit, the oldest converted files are deleted
  - A cache file (`.qsv-mcp-converted-cache.json`) tracks all converted files
- **Example**: `"2.5"` (allows up to 2.5 GB of converted files)

## Usage Examples

### Browse Available Files

Simply ask Claude to list files in your working directory:

```
What tabular data files are available in my Downloads folder?
```

Claude will use the `qsv_list_files` tool to show you all supported files:

```
Found 5 tabular data files:

- allegheny_county_property_sale_transactions.csv (Tabular data file: allegheny_county_property_sale_transactions.csv)
- sales_data.xlsx (Tabular data file: sales_data.xlsx)
- customers.tsv (Tabular data file: customers.tsv)
- events.jsonl (Tabular data file: events.jsonl)
- products.ods (Tabular data file: products.ods)

Use these file paths in qsv commands via the input_file parameter.
Excel and JSONL files will be automatically converted.
```

### Work with Files Using Relative Paths

Once you've set the working directory, you can use relative paths:

```
What are the columns in allegheny_county_property_sale_transactions.csv?
```

Claude will automatically use the working directory to resolve the path and run:
```
qsv_headers with input_file: "allegheny_county_property_sale_transactions.csv"
```

### Work with Files Using Absolute Paths

You can also use absolute paths:

```
Analyze /Users/your-username/data/sales.csv
```

Or on Windows:
```
Analyze C:\Users\YourName\data\sales.csv
```

### Change Working Directory Mid-Session

```
Set working directory to ~/Documents/data
```

Claude will use the `qsv_set_working_dir` tool:
```
Working directory set to: /Users/your-username/Documents/data

All relative file paths will now be resolved from this directory.
```

### List Files Recursively

```
List all CSV files in my Documents folder and its subdirectories
```

Claude will use `qsv_list_files` with `recursive: true`.

## Data Handling

### Input Files

- All qsv tools accept `input_file` parameter (absolute or relative path)
- qsv reads directly from your local filesystem
- No input file size limitations (qsv streams large files efficiently)
- Uncompressed files > 10MB are automatically indexed for improved performance
- Excel and JSONL files are automatically converted to CSV before processing

### Output Files

When processing data, you can either specify an output file or let the MCP server handle it intelligently:

**With `output_file` specified**:
- qsv writes results directly to the specified file
- Tool returns metadata about the operation (file path, size, duration)

**Without `output_file` (automatic handling)**:
- **Small outputs (≤ 850KB)**: Returned directly in the chat for immediate viewing
- **Large outputs (> 850KB)**: Automatically saved to your working directory with a timestamped filename

**Smart large file handling**: The MCP server automatically detects when output would exceed Claude Desktop's 1MB response limit and saves it to disk instead. This prevents timeouts and memory issues when processing large datasets.

Example of large output handling:
```
User: "Select all columns from large_dataset.csv"

Claude: ✅ Large output saved to file (too large to display in chat)

File: qsv-select-2026-01-07_14-30-45.csv
Location: /Users/your-username/Downloads
Size: 2.5 MB
Duration: 150ms

The file is now available in your working directory.
You can:
- Open it in your preferred application
- Process it further with additional qsv commands
- Specify an output_file parameter to control where results are saved
```

## Available Filesystem Tools

### `qsv_list_files`
Browse CSV files in a directory.

**Parameters**:
- `directory` (optional): Directory path (relative or absolute)
- `recursive` (optional): Scan subdirectories (default: false)

**Example prompts**:
- "List CSV files in my Downloads"
- "Show me all CSV files in ./data recursively"
- "What files are in /Users/me/Documents?"

### `qsv_set_working_dir`
Change the working directory for subsequent operations.

**Parameters**:
- `directory` (required): New working directory path

**Example prompts**:
- "Set working directory to ~/Downloads"
- "Change to /Users/me/data directory"
- "Use ~/Documents as the working directory"

### `qsv_get_working_dir`
Check the current working directory.

**Example prompts**:
- "What's the current working directory?"
- "Where are we working from?"

## Complete Workflow Example

Here's a complete example of working with local files:

**User**: "I have some property sale transaction data in my Downloads folder. Can you help me analyze it?"

**Claude**:
1. Uses `qsv_list_files` to show available CSV files
2. User selects the file they want
3. Uses `qsv_headers` to show column names
4. Uses `qsv_stats` to show statistics
5. Uses `qsv_frequency` for distribution analysis
6. All without uploading the file!

## Security Features

### Path Validation
**What is validated:**
- All `input_file` and `output_file` parameters in qsv command tools
- Pipeline `input_file` and `output_file` parameters in `qsv_pipeline`
- Working directory changes via `qsv_set_working_dir`
- File browsing via `qsv_list_files` (with recursive subdirectory validation)
- File preview requests in resource browser

**How validation works:**
- Paths are canonicalized using `fs.realpath()` to resolve symlinks
- Canonical paths are checked against allowed directories
- Attempts to access files outside allowed directories are rejected
- Prevents directory traversal attacks (e.g., `../../etc/passwd`)

**Security notes:**
- All file operations go through the same validation layer
- Server runs with same permissions as Node.js process
- Error messages don't reveal allowed directory paths

### Default Restrictions
- Only CSV-related files are listed (`.csv`, `.tsv`, `.tab`, `.ssv`, Snappy-compressed formats (`.csv.sz`, etc.), and Excel/ODS/JSONL formats)
- Maximum preview size: 1MB
- Preview limited to first 20 lines
- Hidden directories (starting with `.`) are skipped during recursive scans

### Allowed Directories
Configure `QSV_MCP_ALLOWED_DIRS` to explicitly whitelist directories:

```json
{
  "env": {
    "QSV_MCP_ALLOWED_DIRS": "/Users/me/safe/data:/Users/me/safe/outputs"
  }
}
```

**Security recommendations:**
- Only whitelist directories containing data you want Claude to access
- Avoid whitelisting broad directories like `/Users/your-username` or `C:\`
- Be aware that users with filesystem access can read any file within whitelisted directories
- Symlinks within allowed directories pointing outside those directories may pose risks

## Resources Browser

The MCP server also exposes local CSV files as browsable resources in Claude Desktop's resource panel:

1. CSV files in your working directory appear as resources
2. Click on a resource to see file info and preview
3. Use the file path shown in the resource to reference it in commands

## Troubleshooting

### "Access denied" errors
**Problem**: File path is outside allowed directories

**Solution**: Add the directory to `QSV_MCP_ALLOWED_DIRS`:
```json
{
  "env": {
    "QSV_MCP_ALLOWED_DIRS": "/Users/me/Downloads:/path/to/your/data"
  }
}
```

### "File not found" errors
**Problem**: Relative path doesn't resolve correctly

**Solution**:
1. Check working directory: "What's the current working directory?"
2. Use absolute path: `/full/path/to/file.csv`
3. Set working directory: "Set working directory to /path/to/directory"

### Files don't appear in `qsv_list_files`
**Problem**: File extension not recognized

**Solution**: Ensure your file has one of these extensions:
- `.csv` (comma-separated)
- `.tsv` or `.tab` (tab-separated)
- `.ssv` (semicolon-separated)
- `.csv.sz`, `.tsv.sz`, `.tab.sz`, `.ssv.sz` (Snappy compressed)
- `.xls`, `.xlsx`, `.xlsm`, `.xlsb` (Excel formats)
- `.ods` (OpenDocument Spreadsheet)
- `.jsonl`, `.ndjson` (JSON Lines)

## Tips & Best Practices

### 1. Set Working Directory First
Start your session by setting the working directory to where your data lives:
```
Set working directory to ~/Downloads
```

### 2. List Files Before Processing
Browse available files before asking Claude to work with them:
```
List CSV files in current directory
```

### 3. Use Relative Paths
Once the working directory is set, use short relative paths:
```
Show me the first 10 rows of data.csv
```

### 4. Organize Your Data
Keep related CSV files in dedicated directories:
```
~/data/sales/
~/data/customers/
~/data/inventory/
```

### 5. Use Resources Panel
In Claude Desktop, check the Resources panel to:
- Browse available CSV files
- See file previews
- Copy file paths for use in prompts

## Advanced Configuration

### Multiple Data Directories

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["path/to/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/me/primary-data",
        "QSV_MCP_ALLOWED_DIRS": "/Users/me/primary-data:/Users/me/secondary-data:/Users/me/archive-data:/Users/me/Downloads"
      }
    }
  }
}
```

### Network Shares (macOS)

```json
{
  "env": {
    "QSV_MCP_WORKING_DIR": "/Volumes/SharedData",
    "QSV_MCP_ALLOWED_DIRS": "/Volumes/SharedData:/Volumes/Backups"
  }
}
```

### Symlinks and Aliases
The server resolves symlinks and aliases to their real paths, then validates against allowed directories.

## Comparison: File Upload vs. Local Access

### File Upload (Old Way)
❌ Manual upload for every file
❌ Upload time for large files
❌ File size limits
❌ Temporary file storage
❌ Cannot process files larger than upload limit

### Local Access (New Way)
✅ Instant access to local files
✅ No file size limits (qsv can handle GBs)
✅ No upload time
✅ Direct file system access
✅ Browse files with `qsv_list_files`
✅ Work with multiple files easily
✅ Files stay on your machine

## Example Session

```
User: Set working directory to ~/Downloads

Claude: Working directory set to: /Users/your-username/Downloads

User: List CSV files

Claude: Found 3 CSV files:
- property_sales.csv
- sales_2024.csv
- customers.csv

User: What are the columns in property_sales.csv?

Claude: [Uses qsv_headers tool with the file path]
The file has these columns:
- property_id
- sale_date
- sale_price
- property_type
... (etc)

User: Show me statistics for the sale_price column

Claude: [Uses qsv_stats tool]
Statistics for sale_price:
- Count: 45,231
- Mean: $156,780.50
- Median: $125,000
... (etc)
```

All of this happens **without uploading the file** to Claude!

## Next Steps

1. Update your Claude Desktop configuration with the paths above
2. Restart Claude Desktop
3. Try: "Set working directory to ~/Downloads"
4. Try: "List CSV files"
5. Start analyzing your local CSV files without uploads!

## Getting Help

If you encounter issues:
1. Check the Claude Desktop developer console for MCP server logs
2. Verify your `claude_desktop_config.json` syntax is valid JSON
3. Ensure file paths in `QSV_MCP_ALLOWED_DIRS` exist and are accessible
4. Check file permissions on the directories
