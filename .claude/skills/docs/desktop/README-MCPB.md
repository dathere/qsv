# QSV MCP Desktop Extension

The easiest way to give Claude Desktop/Cowork, blazing-fast, powerful tabular data-wrangling capabilities.

## What Is This?

The QSV MCP Desktop Extension teaches Claude Desktop to work with tabular data — CSV, TSV, Excel, Parquet, JSONL, and more. Once installed, just ask Claude to analyze, clean, transform, or summarize your data in plain English. No code. No technical knowledge required.

Under the hood, it's powered by **qsv**: a purpose-built CLI tool written in highly-optimized Rust for wrangling real-world, messy data. It's exponentially faster than Python-based tools like Pandas — processing millions of rows in seconds through multi-threading, streaming algorithms, vectorized query engines (Polars and DuckDB) and automatic metadata compilation that continuously optimizes every operation.

**Ask Claude things like:**
- *"Show me statistics for the revenue column in sales.csv"*
- *"Remove duplicate rows from contacts.xlsx"*
- *"Sort products.tsv by price, highest to lowest"*
- *"Find all rows in data.jsonl where status is 'pending'"*
- *"Summarize this spreadsheet as a report"*
- *"Top 10 complaints by borough, grouped by month, for the past two years"*

Claude handles the rest automatically — no commands to remember, no syntax to learn.

The extension uses **qsvmcp**, a streamlined variant of qsv purpose-built for MCP server use. It ships with 65 commands (vs 75 in the full qsv binary), plus built-in session logging for increased reproducibility.

## Installation (Simple)

### Step 1: Download

Download the extension file:
- Go to: https://github.com/dathere/qsv/releases/latest
- Save the `.mcpb` file to your Downloads folder

### Step 2: Install in Claude Desktop

Double-click the downloaded `qsv-mcp-server-<version>.mcpb` file. This automatically opens Claude Desktop and prompts you to install the extension.

### Step 3: There is no step 3! The extension is ready to use. Just restart Claude Desktop and start asking Claude about your data files.

After installation, you can optionally configure the extension in Claude Desktop's settings:

**Working Directory** (defaults to your Downloads (Mac) or home folder (Windows))
- Where Claude will look for your data files (CSV, Excel, TSV, JSONL, etc.)
- Example: `/Users/yourname/Documents` (Mac) or `C:\Users\yourname\Documents` (Windows)
- You can use `$HOME` or `${HOME}` as shortcuts for your home folder

**Allowed Directories** (Optional but recommended for security)
- Limits which folders Claude can access
- Example: `$HOME/Downloads:$HOME/Documents` (Mac) or `%USERPROFILE%\Downloads;%USERPROFILE%\Documents` (Windows)
- Use `:` to separate folders on Mac/Linux, `;` on Windows
- Leave blank to allow access to all folders

**QSV Binary Path** (Usually auto-detected)
- The extension will try to find **qsvmcp** first, then fall back to **qsv**
- If it can't find either, you'll be prompted to download it
- You can also specify a custom path if you have qsvmcp/qsv installed elsewhere
- qsvmcp is preferred as it's optimized for MCP server use

**DuckDB Options** (Optional)
- If you have DuckDB installed, you can enable it so qsv will use it instead of Polars for SQL queries. Though Polars is fast, built-in, and a dialect of PostgreSQL, DuckDB offers additional features, may perform better on certain queries and is more well-known to AI models, which may lead to better query generation.

## Verifying Installation

Once installed, you can test it:

1. Open a conversation in Claude Desktop
2. Ask: "Can you list the data files in my Documents folder?"
3. If installed correctly, Claude will show you the available tabular data files (CSV, Excel, TSV, JSONL, etc.)

## Supported File Formats

The extension works with many tabular data formats:

**Native formats** (used directly):
- CSV (`.csv`), TSV (`.tsv`, `.tab`), SSV (`.ssv`)
- Compressed (`.csv.sz`, `.tsv.sz`)

**Auto-converted formats** (automatically converted to CSV):
- Excel: `.xls`, `.xlsx`, `.xlsm`, `.xlsb`
- OpenDocument: `.ods`
- JSON Lines: `.jsonl`, `.ndjson`

You don't need to convert Excel or JSON files manually - Claude will do it automatically!

## Example Usage

Once installed, you can have natural conversations with Claude about your data:

### Example 1: Basic Analysis
```
You: "I have a file called sales_2024.csv in my Downloads folder.
      Can you show me summary statistics?"

Claude: [Reads the file and shows statistics like row count,
         column names, data types, min/max values, etc.]
```

### Example 2: Data Cleaning
```
You: "Remove all duplicate rows from contacts.csv and save the
      result to contacts_clean.csv"

Claude: [Removes duplicates and creates the clean file]
        "Done! Removed 47 duplicate rows. The clean file has
        1,234 unique rows."
```

### Example 3: Filtering and Sorting
```
You: "From products.csv, show me only the items where category is
      'Electronics', sorted by price from high to low"

Claude: [Filters and sorts the data, showing the results]
```

### Example 4: Multi-Step Processing
```
You: "Clean up sales.csv by removing duplicates, then calculate
      statistics on the revenue column, then take the top 100 rows"

Claude: [Performs all three operations in sequence]
```

## Troubleshooting

### Extension doesn't appear in Claude Desktop

**Solution:**
1. Make sure you restarted Claude Desktop after installation
2. Check Settings → Extensions to verify it's installed
3. Look for "qsv" in the list of installed extensions
4. Try uninstalling and reinstalling the extension

### "qsv binary not found" error

**Solution:**
1. The extension needs **qsvmcp** (preferred) or **qsv** to be installed on your computer
2. Download from: https://github.com/dathere/qsv/releases (prebuilt binaries include qsvmcp)
3. After downloading:
   - **Mac**: Install using Homebrew (`brew install qsv`) or place qsvmcp/qsv in `/usr/local/bin/`
   - **Windows**: Place `qsvmcp.exe` (or `qsv.exe`) in a folder in your PATH, or specify the full path in settings
   - **Linux**: Place in `/usr/local/bin/` or specify the full path in settings
4. Restart Claude Desktop

> **Note**: qsvmcp is the recommended binary — it includes only the 60 commands needed by the MCP server and is smaller and faster than the full qsv binary.

### "Permission denied" or "Access denied" errors

**Solution:**
1. Check your "Allowed Directories" configuration
2. Make sure the folder you're trying to access is in the allowed list
3. On Mac: You may need to grant Claude Desktop permission to access folders in System Preferences → Privacy & Security
4. Try using absolute paths instead of relative paths (e.g., `/Users/you/Documents/data.csv`)

### Commands are very slow

**Solution:**
1. For large files (>10MB), qsv will automatically create an index the first time
2. This is a one-time cost - subsequent operations will be much faster
3. You can manually create an index by asking Claude: "Create an index for largefile.csv"

### Claude says it can't find my file

**Solution:**
1. Use the full path: `/Users/yourname/Documents/data.csv` (Mac) or `C:\Users\yourname\Documents\data.csv` (Windows)
2. Check your spelling - file names are case-sensitive on Mac/Linux
3. Make sure the file is in your working directory or an allowed directory
4. Ask Claude: "List the data files in my Documents folder" to see what it can access

## Updating

To update to a newer version:

1. Download the latest `.mcpb` file from the releases page
2. In Claude Desktop: Settings → Extensions
3. Find "qsv" in the list
4. Click **Uninstall**
5. Install the new version using the steps above

The extension will automatically detect when your qsv binary is updated and adjust accordingly.

## Uninstalling

To remove the extension:

1. Open Claude Desktop
2. Go to Settings → Extensions
3. Find "qsv" in the list
4. Click **Uninstall**
5. Restart Claude Desktop

## Privacy & Security

**What the extension does:**
- Reads and writes data files on your local computer
- Only accesses folders you explicitly allow
- Never sends your raw data over the internet. Only sends summarized metadata (column names, data types, statistics) to the Model for analysis and decision-making
- All processing happens locally on your machine

**What the extension doesn't do:**
- Never uploads your data to external servers
- Never makes network requests (except to check for qsv updates)
- Never accesses files outside your allowed directories
- Never modifies files unless you explicitly ask

**Security best practices:**
- Use "Allowed Directories" to limit access to specific folders
- Don't allow access to system folders or sensitive directories
- Only work with data files you trust
- Review Claude's proposed actions before confirming

## Getting Help

Need assistance?

1. **Check the troubleshooting section** above for common issues
2. **Review the examples** to see how to phrase your requests
3. **Ask Claude** - it can explain what it's doing and suggest alternatives
4. **Report issues** at: https://github.com/dathere/qsv/issues

## What's Next?

Learn more about qsv's capabilities:
- [QSV Documentation](https://github.com/dathere/qsv) - Full command reference
- [MCP Server README](../../README-MCP.md) - Technical details about the extension
- [QSV Performance Guide](https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md) - Optimization tips

---

# Technical Details

*This section is for developers, system administrators, and advanced users.*

## Desktop Extension Format (MCPB)

The `.mcpb` file is a Model Context Protocol Bundle - a standardized format for distributing MCP servers to Claude Desktop.

**Bundle structure:**
```
qsv-mcp-server.mcpb
├── manifest.json          # Extension metadata (MCP Bundle spec v0.3)
├── dist/
│   └── mcp-server.js     # Main MCP server (Node.js)
├── node_modules/          # Dependencies
└── package.json           # NPM package definition
```

**Manifest specification:**
- Follows MCP Bundle spec v0.3
- Defines entry point, configuration schema, and resource requirements
- Includes version, author, license information

## Architecture

```
┌─────────────────────────────────────┐
│      Claude Desktop (MCP Client)   │
└──────────────┬──────────────────────┘
               │ stdio (JSON-RPC 2.0)
┌──────────────▼───────────────────────────┐
│  QSV MCP Server (Node.js/TypeScript)     │
│  • Tool definitions (11 core + deferred) │
│  • Parameter validation                  │
│  • File conversion manager               │
│  • Format auto-detection                 │
└──────────────┬───────────────────────────┘
               │ spawn (streaming, secure)
┌──────────────▼──────────────────────┐
│   qsvmcp binary (preferred) / qsv   │
│  • Tabular data processing         │
│  • 65 commands (qsvmcp) / 75 (qsv) │
│  • High-performance operations      │
│  • Multi-format support             │
└─────────────────────────────────────┘
```

## Configuration

The extension is configured via environment variables in Claude Desktop's MCP server configuration.

**Core configuration:**
```json
{
  "command": "node",
  "args": ["/path/to/dist/mcp-server.js"],
  "env": {
    "QSV_MCP_BIN_PATH": "/usr/local/bin/qsvmcp",
    "QSV_MCP_WORKING_DIR": "/Users/you/Documents",
    "QSV_MCP_ALLOWED_DIRS": "/Users/you/Downloads:/Users/you/Documents"
  }
}
```

**All environment variables:**

| Variable | Default | Description |
|----------|---------|-------------|
| `QSV_MCP_BIN_PATH` | `qsvmcp` (falls back to `qsv`) | Path to qsvmcp/qsv binary (supports template vars) |
| `QSV_MCP_WORKING_DIR` | Current dir | Default directory for relative paths |
| `QSV_MCP_ALLOWED_DIRS` | None | Colon-separated (`:`) allowed directories (semicolon `;` on Windows) |
| `QSV_MCP_CONVERTED_LIFO_SIZE_GB` | `1` | Max cache size for converted files (0.1-100 GB) |
| `QSV_MCP_OPERATION_TIMEOUT_MS` | `600000` | Operation timeout in milliseconds (1s-30min, default 10 minutes) |
| `QSV_MCP_MAX_FILES_PER_LISTING` | `1000` | Max files in directory listings (1-100k) |
| `QSV_MCP_MAX_CONCURRENT_OPERATIONS` | `1` | Max concurrent operations (1-100) |
| `QSV_MCP_AUTO_REGENERATE_SKILLS` | `false` | Auto-regenerate when qsv version changes |
| `QSV_MCP_CHECK_UPDATES_ON_STARTUP` | `true` | Check for updates on startup |
| `QSV_MCP_NOTIFY_UPDATES` | `true` | Show update notifications |
| `QSV_MCP_GITHUB_REPO` | `dathere/qsv` | GitHub repo for release checks |

**Template variable expansion:**
- `$HOME` and `${HOME}` expand to user's home directory
- Works in `QSV_MCP_BIN_PATH`, `QSV_MCP_WORKING_DIR`, and `QSV_MCP_ALLOWED_DIRS`

## Security Considerations

### Command Injection Prevention

The MCP server uses `spawn()` with array arguments instead of `exec()` to prevent shell injection attacks:

```typescript
// Secure - arguments passed as array to spawn
spawn(qsvBinaryPath, ['select', '1-5', userFile], { stdio: ['pipe', 'pipe', 'pipe'] });

// NOT used - vulnerable to injection
exec(`qsv select 1-5 ${userFile}`);  // NEVER DONE
```

### Directory Access Control

The `QSV_MCP_ALLOWED_DIRS` setting enforces access control:

1. Before accessing any file, the server validates the resolved absolute path
2. Paths are normalized (symlinks resolved, `..` traversal prevented)
3. If `ALLOWED_DIRS` is set, the path must be within one of the allowed directories
4. If `ALLOWED_DIRS` is empty/unset, all directories are accessible (not recommended)

**Implementation:**
```typescript
function validateFileAccess(filePath: string): void {
  const resolved = path.resolve(filePath);
  const allowed = getAllowedDirs(); // From QSV_MCP_ALLOWED_DIRS

  if (allowed.length > 0) {
    const isAllowed = allowed.some(dir =>
      resolved.startsWith(path.resolve(dir))
    );
    if (!isAllowed) {
      throw new Error(`Access denied: ${resolved} is not in allowed directories`);
    }
  }
}
```

### Resource Limits

The MCP server enforces limits to prevent DoS attacks and resource exhaustion:

- **Operation timeout**: Default 10 minutes (600000ms), prevents hung operations
- **Max file listings**: Default 1000 files, prevents directory enumeration attacks
- **Max concurrent ops**: Default 1, prevents resource exhaustion
- **Converted file cache**: LIFO eviction with size limit (default 1GB)

### Binary Trust

**CRITICAL**: The `QSV_MCP_BIN_PATH` must point to a trusted qsvmcp or qsv binary:

- Only use official releases from https://github.com/dathere/qsv/releases
- The **qsvmcp** binary is recommended — it's optimized for MCP server use with only the 60 commands needed
- Verify binary integrity (checksums provided in releases)
- Ensure binary path is not writable by untrusted users
- Do not use binaries from unknown sources
- **qsvlite** and **qsvdp** are not supported — they lack required features (Polars, etc.)

## File Conversion System

The MCP server automatically converts Excel and JSONL files to CSV.

**Conversion flow:**
```
1. User requests operation on data.xlsx
2. Server detects .xlsx extension
3. Creates temporary CSV: /tmp/converted-{hash}-data.csv
4. Executes: qsv excel data.xlsx --output /tmp/converted-{hash}-data.csv
5. Performs requested operation on converted file
6. Tracks converted file in LIFO cache
7. Cleans up when cache exceeds size limit
```

**Cache management:**
- LIFO (Last In, First Out) eviction strategy
- Configurable size limit (default 1GB)
- Automatic cleanup of oldest conversions
- Preserves frequently-used conversions
- Conversion metadata tracked in memory

**Supported conversions:**
- Excel → CSV: `qsv excel` (supports .xls, .xlsx, .xlsm, .xlsb, .ods)
- JSONL → CSV: `qsv jsonl` (supports .jsonl, .ndjson)

## Stats Cache Optimization

The MCP server automatically enables `--stats-jsonl` when running `qsv stats` to create cache files that accelerate subsequent "smart" commands.

**Smart commands that use stats cache:**
- `frequency` - Uses cardinality info to optimize memory allocation
- `schema` - Uses data type inference from stats
- `tojsonl` - Uses stats for type detection
- `sqlp`, `joinp`, `pivotp` - Uses stats for query optimization
- `describegpt` - Uses stats for context-aware descriptions
- `moarstats` - Uses stats cache for enriched statistics
- `sample` - Uses row count for sample size calculation

**Cache files:**
- `.stats.csv` - Human-readable statistics
- `.stats.csv.data.jsonl` - Machine-readable cache (used by smart commands)

**Cache validation:**
- Checks file modification times
- Regenerates if source CSV is newer than cache
- Can force regeneration with `--force` flag

## Tool Guidance System

The MCP server enhances tool descriptions with contextual guidance to help Claude make optimal decisions.

**Guidance types:**
- 💡 **USE WHEN** - When to use this tool vs alternatives
- 📋 **COMMON PATTERNS** - Typical workflows and command combinations
- ⚠️ **CAUTION** - Memory limits, file size constraints, feature requirements
- 🚀 **PERFORMANCE** - Index acceleration, cache strategies, optimization tips

**Example (qsv_dedup):**
```
💡 USE WHEN: Removing duplicate rows. Memory-intensive - loads entire CSV.
Good for small-medium files. For very large files (>1GB), use qsv_extdedup instead.

📋 COMMON PATTERN: Often followed by stats or frequency to analyze cleaned data:
dedup → stats to see distribution after removing duplicates.

⚠️ CAUTION: Memory-intensive - loads entire file. For files >1GB, this may
fail with OOM. Use qsv_extdedup for very large files.

🚀 PERFORMANCE: Run qsv_index first for files >10MB to enable parallel processing.
```

This guidance helps Claude:
1. Choose the right command for the task
2. Suggest alternative commands when appropriate
3. Warn about potential issues before they occur
4. Optimize performance automatically

## Development

### Building the MCPB

```bash
cd .claude/skills
npm install
npm run build
npm run mcpb:package
```

This creates `qsv-mcp-server.mcpb` in the current directory.

**Packaging process:**
1. Validates manifest.json against MCP Bundle spec v0.3
2. Compiles TypeScript to JavaScript
3. Bundles server code and dependencies
4. Creates .mcpb archive with proper structure
5. Validates bundle integrity

### Testing

```bash
# Run all tests
npm test

# Test MCP server integration
npm run mcp:start

# Test with Claude Desktop
npm run mcp:install  # Install to Claude Desktop
# Then test in Claude Desktop UI
```

### Debugging

**Enable debug logging:**
```bash
# Set in Claude Desktop config
{
  "env": {
    "DEBUG": "qsv:*",
    "QSV_LOG_LEVEL": "debug"
  }
}
```

**Check logs:**
- macOS: `~/Library/Logs/Claude/mcp*.log`
- Windows: `%APPDATA%\Claude\logs\mcp*.log`
- Linux: `~/.config/Claude/logs/mcp*.log`

## Deployment Considerations

### System Requirements

- **Node.js**: >= 18.0.0 (runtime for MCP server)
- **qsv**: >= 16.0.0 (CSV processing engine)
- **Claude Desktop**: Latest version
- **Memory**: 2GB minimum, 8GB+ recommended for large files
- **Disk space**: 500MB for extension + converted file cache

### Performance Tuning

**For large files (>100MB):**
```json
{
  "env": {
    "QSV_MCP_OPERATION_TIMEOUT_MS": "900000",
    "QSV_MCP_CONVERTED_LIFO_SIZE_GB": "5"
  }
}
```

**For production environments:**
```json
{
  "env": {
    "QSV_MCP_MAX_CONCURRENT_OPERATIONS": "20",
    "QSV_MCP_AUTO_REGENERATE_SKILLS": "true",
    "QSV_MCP_CHECK_UPDATES_ON_STARTUP": "true"
  }
}
```

### Auto-Update Strategy

The MCP server includes built-in update detection:

1. Checks qsv binary version on startup
2. Compares against cached version
3. If changed, optionally regenerates skill definitions
4. Checks GitHub releases for newer versions
5. Logs update notifications

**Configuration:**
```json
{
  "env": {
    "QSV_MCP_AUTO_REGENERATE_SKILLS": "true",
    "QSV_MCP_CHECK_UPDATES_ON_STARTUP": "true",
    "QSV_MCP_NOTIFY_UPDATES": "true"
  }
}
```

## Contributing

To contribute improvements to the Desktop Extension:

1. Fork the qsv repository
2. Make changes in `.claude/skills/`
3. Run tests: `npm test`
4. Build: `npm run build`
5. Test with Claude Desktop: `npm run mcp:install`
6. Submit pull request

**Areas for contribution:**
- Additional tool definitions
- Improved error messages
- Performance optimizations
- Documentation improvements
- Bug fixes

## License

Same license as qsv: MIT OR Apache-2.0

---

**Updated**: 2026-03-23
**Version**: 18.0.2
**Format**: MCP Bundle (MCPB) v0.3
**Compatibility**: Claude Desktop 1.0+
