# QSV Desktop Extension Guide

Complete guide for using the qsv MCP Server as a Claude Desktop Extension.

## What is the Desktop Extension?

The qsv Desktop Extension packages the MCP Server as a `.mcpb` file (MCP Bundle) that can be installed in Claude Desktop with a simple double-click. No terminal commands, no config file editing, no Node.js installation required.

### Benefits over Legacy MCP Installation

| Aspect | Legacy MCP Server | Desktop Extension |
|--------|------------------|-------------------|
| Installation | 6 manual steps, terminal required | 2 steps, no terminal |
| Configuration | Edit JSON files manually | User-friendly settings UI |
| Updates | Manual reinstallation | Automatic via marketplace |
| Discovery | Search GitHub | Built-in extension directory |
| Requirements | Node.js, npm, git | Just Claude Desktop |
| Security | Config files in plain text | OS keychain storage |

---

## Prerequisites

### Required

1. **Claude Desktop** - Download from [claude.ai](https://claude.ai/)
2. **qsv binary** - Install via one of these methods:

   ```bash
   # macOS (Homebrew)
   brew install qsv

   # macOS/Linux (pre-built binary)
   curl -LO https://github.com/dathere/qsv/releases/latest/download/qsv-$(uname -s)-$(uname -m).zip
   unzip qsv-*.zip
   sudo mv qsv /usr/local/bin/

   # Windows (Scoop)
   scoop install qsv

   # From source (requires Rust)
   cargo install qsv --features all_features
   ```

3. **Verify qsv installation**:

   ```bash
   qsv --version
   # Should output: qsv 16.1.0 (or later)
   ```

---

## Installation

### Step 1: Download Extension

**Option A: From Extension Marketplace** (when available)
1. Open Claude Desktop
2. Go to Settings → Extensions
3. Search for "qsv"
4. Click "Install"

**Option B: Manual Installation**
1. Download `qsv-mcp-server.mcpb` from GitHub releases
2. Or build it yourself:
   ```bash
   cd /path/to/qsv/.claude/skills
   npm install
   npm run mcpb:package
   ```

### Step 2: Install Extension

1. **Drag and drop** the `.mcpb` file into Claude Desktop settings, OR
2. **Double-click** the `.mcpb` file (opens with Claude Desktop), OR
3. In Claude Desktop: Settings → Extensions → "Install from file" → select `.mcpb` file

### Step 3: Configure Extension (Optional)

After installation, Claude Desktop will prompt you to configure the extension. **Good news: most settings have smart defaults and auto-detection!**

1. **qsv Binary Path** (optional - auto-detected!)
   - **Leave empty** for automatic detection (recommended)
   - Auto-detects from: PATH, `/usr/local/bin`, `/opt/homebrew/bin`, `~/.cargo/bin`
   - Only set manually if auto-detection fails or you have a custom installation
   - Example: `/usr/local/bin/qsv`

2. **Default Working Directory** (optional)
   - Default: `${HOME}/Downloads` (auto-expands to your Downloads folder)
   - Where qsv commands run by default
   - You can change this to any folder

3. **Allowed Directories** (optional)
   - Default: Only working directory
   - Add additional directories for file access (one per entry)
   - Leave empty to restrict access to working directory only

4. **Advanced Settings** (optional)
   - Command Timeout: 600000ms (10 minutes)
   - Max Output Size: 50MB
   - Auto-Regenerate Skills: false
   - Check for Updates: true

💡 **Tip**: Use the `qsv_config` tool to verify your configuration and see detected paths!

### Step 4: Restart Claude Desktop

Close and reopen Claude Desktop to activate the extension.

---

## Verifying Installation

### Check Configuration

In Claude Desktop chat:

```
qsv_config
```

Claude should show your configuration with:
- ✅ qsv binary path (auto-detected or manually set)
- 📍 Version number
- 📁 Working directory
- 🔓 Allowed directories

**If auto-detection failed**, the diagnostics section will show which paths were checked and why detection failed.

### Test Extension is Active

```
List available qsv commands
```

Claude should respond with a list of available qsv commands.

### Test Basic Command

```
Can you show me an example of using qsv stats?
```

Claude should provide examples and offer to run commands.

### Test File Access

```
List data files in my Downloads folder
```

Claude should use the `qsv_list_files` tool to show your tabular data files (CSV, Excel, TSV, JSONL, etc.).

---

## Using the Extension

### Working with Local Files

The extension provides direct access to your local CSV, Excel, and JSONL files:

```
Show me the first few rows of ~/Downloads/data.csv
```

Claude will use `qsv_get_file_preview` to display the file contents.

### Running qsv Commands

```
Calculate statistics for the price column in ~/Downloads/sales.csv
```

Claude will use `qsv_stats` with the appropriate arguments.

### Complex Pipelines

```
From ~/Downloads/customers.csv:
1. Remove duplicate emails
2. Keep only customers from California
3. Sort by revenue descending
4. Show top 10
```

Claude will chain multiple qsv operations sequentially.

### Available Operations

The extension provides MCP tools covering:

- **Data Selection**: select, slice, sample
- **Statistics**: stats, moarstats, frequency
- **Filtering**: search, searchset, dedup
- **Transformation**: apply, rename, replace
- **Aggregation**: groupby, pivot
- **Joining**: join, joinp
- **Validation**: validate, schema, safenames
- **Conversion**: excel, json, jsonl, to
- **Formatting**: fmt, table, fixlengths
- **Filesystem**: list_files, get_file_preview, set_working_dir

For complete documentation, see [README.md](./README.md).

---

## Configuration

### Accessing Settings

1. Open Claude Desktop
2. Go to Settings → Extensions
3. Find "qsv Data Wrangling"
4. Click gear icon ⚙️

### Configuration Options

#### qsv Binary Path

**Purpose**: Location of qsv executable

**Default**: Auto-detection (recommended - leave empty)

**Auto-detection checks** (in order):
1. System PATH using `which qsv` (macOS/Linux) or `where qsv` (Windows)
2. Common installation locations:
   - macOS/Linux: `/usr/local/bin/qsv`, `/opt/homebrew/bin/qsv`, `~/.cargo/bin/qsv`, `~/.local/bin/qsv`
   - Windows: `C:\Program Files\qsv\qsv.exe`, `C:\qsv\qsv.exe`, `%USERPROFILE%\scoop\shims\qsv.exe`

**Manual override examples**:
- macOS/Linux: `/usr/local/bin/qsv`
- Windows: `C:\Program Files\qsv\qsv.exe`

**Finding your path manually**:
```bash
which qsv          # macOS/Linux
where qsv           # Windows
```

**Verification**: Use `qsv_config` tool to see detected path and diagnostics

#### Working Directory

**Purpose**: Default directory for qsv operations

**Default**: `~/Downloads` (macOS/Linux) or `%USERPROFILE%\Downloads` (Windows)

**When to change**: If you primarily work with files in a different location

#### Allowed Directories

**Purpose**: Security - restrict file access to specific directories

**Default**: Only working directory (empty by default)

**Format**:
- macOS/Linux: Colon-separated paths (`/path1:/path2`)
- Windows: Semicolon-separated paths (`C:\path1;C:\path2`)

**Examples**:
- Allow all: Leave empty
- Specific folders: `/Users/you/Data:/Users/you/Projects`
- Windows: `C:\Users\You\Data;C:\Users\You\Documents`

#### Timeout Settings

**Command Timeout**: Maximum time for qsv operations (default: 10 minutes)

**Use cases**:
- Large files: Increase to 15-30 minutes
- Quick operations only: Decrease to 1-2 minutes

#### Output Size Limit

**Max Output Size**: Maximum size of command output (default: 50MB)

**Behavior**:
- Outputs < 850KB: Returned directly
- Outputs > 850KB: Saved to temp file, path returned
- Outputs > limit: Error with suggestion to adjust

#### Update Settings

**Auto-Regenerate Skills**: Automatically update skill definitions when qsv version changes (default: false)

**Requirements**:
- qsv repository cloned locally
- qsv binary built with "mcp" feature

**Check for Updates**: Check for new qsv releases on startup (default: true)

**Show Update Notifications**: Display update notifications in logs (default: true)

---

## Troubleshooting

### Extension Not Appearing

**Symptom**: qsv extension doesn't show in Claude Desktop

**Solutions**:
1. **Restart Claude Desktop** - Close completely and reopen
2. **Check installation**:
   - Settings → Extensions
   - Look for "qsv Data Wrangling"
3. **Reinstall extension** - Remove and reinstall the `.mcpb` file
4. **Check Claude Desktop logs**:
   - macOS: `~/Library/Logs/Claude/`
   - Windows: `%APPDATA%\Claude\logs\`
   - Look for MCP-related errors

### "qsv command not found"

**Symptom**: Extension installed but commands fail with "command not found"

**Solutions**:
1. **Check configuration and diagnostics**:
   ```
   qsv_config
   ```
   This shows:
   - Whether qsv was detected
   - Which paths were checked
   - Why auto-detection failed (if it did)

2. **Verify qsv is installed**:
   ```bash
   qsv --version
   ```

3. **Try manual path configuration**:
   - Find qsv location: `which qsv` (macOS/Linux) or `where qsv` (Windows)
   - Settings → Extensions → qsv → Configuration
   - Set "qsv Binary Path" to the full path (e.g., `/usr/local/bin/qsv`)
   - Save and restart Claude Desktop

4. **Reinstall qsv** if not found:
   - See [Prerequisites](#prerequisites) section

### Permission Denied Errors

**Symptom**: "Permission denied" when accessing files

**Solutions**:
1. **Check allowed directories**:
   - Verify file is in an allowed directory
   - Update "Allowed Directories" setting if needed
2. **Grant Claude Desktop file access** (macOS):
   - System Preferences → Security & Privacy → Files and Folders
   - Enable access for Claude Desktop
3. **Check file permissions**:
   ```bash
   ls -l ~/path/to/file.csv
   ```

### Large File Timeouts

**Symptom**: Commands timeout on large files

**Solutions**:
1. **Increase timeout**:
   - Settings → Extensions → qsv → Advanced Settings
   - Increase "Command Timeout" to 15-30 minutes
2. **Use streaming commands**:
   - Most qsv commands stream data (constant memory)
   - Avoid commands marked with 🤯 (memory-intensive)
3. **Pre-index large files**:
   ```bash
   qsv index large-file.csv
   ```
4. **Use compression**:
   ```bash
   qsv snappy compress large-file.csv
   ```

### Skills Outdated

**Symptom**: Warning about qsv version mismatch

**Background**: Skills are JSON files describing qsv commands. When qsv updates, skills may be outdated.

**Solutions**:
1. **Manual update** (if auto-regenerate disabled):
   ```bash
   cd /path/to/qsv
   qsv --update-mcp-skills
   ```
2. **Enable auto-regenerate**:
   - Settings → Extensions → qsv → Advanced Settings
   - Enable "Auto-Regenerate Skills"
   - Requires qsv repository cloned locally
3. **Update qsv**:
   ```bash
   qsv --update
   ```

### Extension Updates

**Automatic Updates** (when available):
- Extension marketplace handles updates automatically
- Notifications appear in Claude Desktop

**Manual Updates**:
1. Download latest `.mcpb` file
2. Settings → Extensions → qsv → Remove
3. Install new `.mcpb` file
4. Reconfigure settings (will be preserved)
5. Restart Claude Desktop

---

## Comparison with Other Installation Methods

All three installation methods provide **identical qsv functionality** but differ in installation, interface, and use cases:

### Desktop Extension (This Guide)

✅ **Use Desktop Extension if**:
- You want simplest installation
- You prefer GUI configuration and visual interface
- You want automatic updates from marketplace
- You're a non-technical user
- You work primarily in Claude Desktop

### Claude Code (CLI)

✅ **Use Claude Code if**:
- You work primarily in the terminal
- You need to automate data workflows
- You work on remote servers via SSH
- You want tighter git integration
- You prefer keyboard-driven interfaces

**Documentation**: [CLAUDE_CODE.md](./CLAUDE_CODE.md)

### Legacy MCP Server

✅ **Use Legacy MCP Server if**:
- You need maximum configuration flexibility
- You use MCP-compatible tools besides Claude Desktop
- You prefer config files over GUI
- You're comfortable with terminal/npm
- You want latest features immediately (no marketplace delay)

### Migration

**From Legacy MCP Server to Desktop Extension**:
1. Install extension (see [Installation](#installation))
2. Configure settings to match your `claude_desktop_config.json`
3. Remove legacy MCP server from config:
   ```bash
   # Edit ~/Library/Application Support/Claude/claude_desktop_config.json
   # Remove the "qsv" entry from "mcpServers"
   ```
4. Restart Claude Desktop

**From Desktop Extension to Claude Code**:
1. Build the MCP server: `cd .claude/skills && npm install && npm run build`
2. Run installer: `npm run mcp:install` (detects Claude Code automatically)
3. Keep or remove Desktop Extension (both can coexist)

**From Desktop Extension to Legacy MCP Server**:
1. Remove extension: Settings → Extensions → qsv → Remove
2. Follow instructions in [README.md](./README.md#installation)

**Using Multiple Methods Simultaneously**:
You can use Desktop Extension and Claude Code at the same time - they use separate config files and don't conflict.

---

## Advanced Topics

### Custom Skill Definitions

The extension uses skill definitions from `.claude/skills/qsv/*.json`.

**To customize**:
1. Extension mode loads skills from bundled files (read-only)
2. For custom skills, use legacy MCP server mode
3. Edit JSON files in qsv repository
4. Run `qsv --update-mcp-skills` to regenerate
5. Repackage extension: `npm run mcpb:package`

### Working with Large Datasets

**Best practices**:
1. **Index files first**: `qsv index large-file.csv` enables fast random access
2. **Use stats cache**: `qsv stats --stats-jsonl large-file.csv` creates cache for "smart" commands
3. **Compress data**: Snappy compression (`.sz` extension) provides fast compression/decompression
4. **Stream operations**: Prefer streaming commands over memory-intensive ones
5. **Increase limits**: Adjust timeout and output size settings

### Integration with Other Tools

**Exporting results**:
- Results are saved as CSV files
- Use qsv to convert to other formats:
  ```bash
  qsv to xlsx input.csv
  qsv sqlp input.csv 'COPY (SELECT * FROM input) TO output.parquet'
  ```

**Pipeline with external tools**:
- qsv outputs can be piped to other commands
- Example: `qsv stats input.csv | qsv table`

### Security Considerations

**File Access**:
- Extension respects "Allowed Directories" setting
- Symlinks are followed (can escape allowed directories)
- Consider restricting to specific folders for production use

**Sensitive Data**:
- Skills and data never leave your machine
- Extension runs locally (no cloud processing)
- Credentials stored in OS keychain (not config files)

**Code Execution**:
- qsv binary must be trusted
- Extension does not execute arbitrary code
- All operations go through qsv command-line interface

---

## Frequently Asked Questions

### Can I use the extension without qsv installed?

No. The extension requires qsv binary to be installed separately. The extension is a wrapper that makes qsv easier to use with Claude, not a replacement for qsv itself.

### Does the extension upload my data to Claude?

No. All processing happens locally on your machine. The extension runs qsv commands locally and returns results to Claude Desktop. Your data never leaves your computer.

### Can I use the extension with Claude API (not Desktop)?

No. Desktop Extensions only work with Claude Desktop. For Claude API integration, use the MCP SDK directly or the legacy MCP server.

### How do I update qsv?

```bash
qsv --update          # If qsv was installed via self-update
brew upgrade qsv       # If installed via Homebrew
cargo install qsv      # If installed via Cargo
```

After updating qsv, the extension will detect the version change and prompt you to regenerate skills (or do it automatically if enabled).

### Can I have multiple versions of the extension?

No. Claude Desktop allows only one instance of each extension. However, you can switch between extension and legacy MCP server configurations.

### What's the file size limit?

The extension can process files of any size, but practical limits exist:
- **Timeout**: Default 10 minutes (configurable)
- **Memory**: Memory-intensive commands (🤯) load entire file
- **Output**: Results > 850KB saved to temp files

For very large files (> 10GB), consider using qsv directly in terminal.

### Can I contribute to the extension?

Yes! The extension is open source:
- Repository: https://github.com/dathere/qsv
- Extension code: `.claude/skills/` directory
- Issues: https://github.com/dathere/qsv/issues
- Pull requests welcome!

---

## Getting Help

### Documentation

- **Quick Start**: [QUICK_START.md](./QUICK_START.md)
- **Full Guide**: [README.md](../../README.md)
- **Filesystem Usage**: [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)
- **Auto-Update System**: [AUTO_UPDATE.md](../reference/AUTO_UPDATE.md)
- **qsv Documentation**: https://github.com/dathere/qsv#commands

### Support

- **GitHub Issues**: https://github.com/dathere/qsv/issues
- **Discussions**: https://github.com/dathere/qsv/discussions
- **Discord**: https://discord.gg/dathere (coming soon)

### Logs

**Claude Desktop logs** (for extension debugging):
- macOS: `~/Library/Logs/Claude/mcp*.log`
- Windows: `%APPDATA%\Claude\logs\mcp*.log`
- Linux: `~/.local/share/Claude/logs/mcp*.log`

**MCP Server logs**:
- Extension logs appear in Claude Desktop logs
- Look for `[qsv-data-wrangling]` prefixed messages

---

## Changelog

### Version 16.1.2 (2026-02-18)
- See [CHANGELOG.md](../../CHANGELOG.md) for full version history

---

**Status**: ✅ Production Ready
**Extension ID**: `qsv-data-wrangling`
**Package Size**: ~11MB (compressed)
**Supported Platforms**: macOS, Windows, Linux
