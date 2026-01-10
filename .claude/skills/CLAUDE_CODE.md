# QSV MCP Server with Claude Code

Complete guide for using the qsv MCP Server with Claude Code (the CLI tool).

## What is Claude Code?

Claude Code is Anthropic's official CLI for Claude - a powerful terminal-based interface for AI-assisted coding and data work. Unlike Claude Desktop (the GUI app), Claude Code runs entirely in your terminal and integrates seamlessly with your development workflow.

### Benefits of Claude Code for qsv

| Feature | Benefit |
|---------|---------|
| **Terminal integration** | Work with files in your current directory without path specifications |
| **Command history** | Track qsv operations in your shell history |
| **Git integration** | See qsv operations in context of your repository |
| **Scriptable** | Automate data workflows with shell scripts |
| **SSH/Remote** | Use qsv on remote servers via SSH |
| **Lightweight** | No GUI overhead, runs anywhere |

---

## Prerequisites

### Required

1. **Claude Code CLI** - Install from [claude.ai/code](https://claude.ai/code)
   ```bash
   # Verify installation
   claude --version
   ```

2. **Node.js ≥ 18.0.0** - Required to run the MCP server
   ```bash
   node --version
   npm --version
   ```

3. **qsv binary** - Install via one of these methods:
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

4. **Verify qsv installation**:
   ```bash
   qsv --version
   # Should output: qsv 13.0.0 (or later)
   ```

---

## Installation

### Step 1: Build the MCP Server

```bash
cd /path/to/qsv/.claude/skills
npm install
npm run build
```

This compiles the TypeScript source to JavaScript in the `dist/` directory.

### Step 2: Configure Claude Code

**Option A: Automatic Configuration (Recommended)**

Run the interactive installer:

```bash
npm run mcp:install
```

This will:
1. Detect Claude Code installation
2. Prompt for configuration (qsv path, allowed directories, etc.)
3. Create or update `~/.config/claude-code/mcp_settings.json`
4. Verify the configuration

**Option B: Manual Configuration**

1. **Find your Claude Code config location**:
   - macOS/Linux: `~/.config/claude-code/mcp_settings.json`
   - Windows: `%APPDATA%\Claude Code\mcp_settings.json`

2. **Create or edit the file**:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "/absolute/path/to/qsv/.claude/skills/dist/mcp-server.js"
      ],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/your-username/data",
        "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/data:/Users/your-username/Downloads"
      }
    }
  }
}
```

**Important**:
- Use **absolute paths** (no `~` or relative paths)
- The `args` path must point to your built `mcp-server.js` file
- `QSV_MCP_BIN_PATH` can be omitted if qsv is in PATH (auto-detected)
- `QSV_MCP_WORKING_DIR` defaults to current directory if omitted
- Use `:` (colon) to separate paths on macOS/Linux, `;` (semicolon) on Windows

### Step 3: Restart Claude Code

```bash
# Exit current session
exit

# Start new session
claude
```

Or if Claude Code is already running, restart it to load the new MCP server configuration.

---

## Verifying Installation

### Check MCP Server Status

In Claude Code:

```
Check if the qsv MCP server is connected
```

Claude should confirm the qsv server is active and list available tools.

### View Configuration

```
Show me my qsv configuration using qsv_config
```

You should see:
- ✅ qsv binary path (auto-detected or configured)
- 📍 Version number
- 📁 Working directory
- 🔓 Allowed directories

### Test Basic Command

```
List CSV files in my current directory
```

Claude should use the `qsv_list_files` tool to show files.

### Test qsv Execution

```
Calculate statistics for data.csv
```

Claude should execute `qsv stats` and return results.

---

## Using qsv with Claude Code

### Working with Local Files

Claude Code can access files in your current directory and allowed directories:

```bash
# Navigate to your data directory
cd ~/data

# Start Claude Code
claude
```

Then in Claude Code:
```
Show me the first 10 rows of customers.csv
```

### Relative vs Absolute Paths

**Relative paths** (recommended in Claude Code):
```
Calculate statistics for ./data.csv
Filter rows in ../archive/sales.csv where amount > 100
```

**Absolute paths**:
```
Process /Users/me/data/customers.csv
Join /home/user/orders.csv with /home/user/products.csv
```

### Common Workflows

#### 1. Quick Data Inspection

```bash
cd ~/Downloads
claude
```

```
What CSV files are here? Show me stats for the largest one.
```

#### 2. Data Cleaning Pipeline

```
From customers.csv:
1. Remove duplicates by email
2. Filter to California customers only
3. Sort by revenue descending
4. Save to customers_clean.csv
```

#### 3. Multi-file Analysis

```
Compare the schemas of sales_2023.csv and sales_2024.csv.
Are they compatible for joining?
```

#### 4. Automated Workflows

Create a script that uses Claude Code:

```bash
#!/bin/bash
# process-daily-data.sh

cd /data/daily
claude << EOF
Process today's data:
1. Validate data.csv schema
2. Calculate statistics
3. Generate frequency tables
4. Export results to /reports/
EOF
```

---

## Configuration Options

### Environment Variables

All environment variables from the legacy MCP server are supported:

#### `QSV_MCP_BIN_PATH`
- **Description**: Path to qsv executable
- **Default**: Auto-detected from PATH and common locations
- **Example**: `"/usr/local/bin/qsv"`
- **Auto-detection**: Checks PATH, `/usr/local/bin`, `/opt/homebrew/bin`, `~/.cargo/bin`, etc.

#### `QSV_MCP_WORKING_DIR`
- **Description**: Default directory for relative paths
- **Default**: Current working directory (wherever you run Claude Code)
- **Example**: `"/Users/you/data"`
- **Tip**: Leave unset to use current directory (most flexible for CLI usage)

#### `QSV_MCP_ALLOWED_DIRS`
- **Description**: Colon-separated list of allowed directories (security)
- **Default**: Only working directory
- **Example (Unix)**: `"/Users/you/data:/Users/you/Downloads"`
- **Example (Windows)**: `"C:\\Users\\You\\Data;C:\\Users\\You\\Downloads"`
- **Tip**: Set to parent directories to allow subdirectory access

#### `QSV_MCP_TIMEOUT_MS`
- **Description**: Command timeout in milliseconds
- **Default**: `300000` (5 minutes)
- **Increase for**: Large file operations

#### `QSV_MCP_MAX_OUTPUT_SIZE`
- **Description**: Maximum output size in bytes
- **Default**: `52428800` (50MB)
- **Behavior**: Outputs > 850KB auto-saved to temp files

#### `QSV_MCP_CONVERTED_LIFO_SIZE_GB`
- **Description**: Max total size of converted Excel/JSONL files (in GB)
- **Default**: `1` (1GB)
- **Purpose**: Auto-cleanup of `.converted.csv` files

### Advanced Configuration

**Multiple qsv configurations** (e.g., different allowed directories):

```json
{
  "mcpServers": {
    "qsv-work": {
      "command": "node",
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_WORKING_DIR": "/work/data",
        "QSV_MCP_ALLOWED_DIRS": "/work/data:/work/reports"
      }
    },
    "qsv-personal": {
      "command": "node",
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_WORKING_DIR": "/Users/me/personal",
        "QSV_MCP_ALLOWED_DIRS": "/Users/me/personal:/Users/me/Downloads"
      }
    }
  }
}
```

---

## Comparison: Claude Code vs Claude Desktop

| Aspect | Claude Code (CLI) | Claude Desktop (GUI) |
|--------|------------------|---------------------|
| **Installation** | Edit JSON config file | Install `.mcpb` bundle or JSON config |
| **Interface** | Terminal-based | GUI application |
| **File access** | Current directory + allowed dirs | Configured directories only |
| **Configuration** | `mcp_settings.json` | GUI settings or JSON config |
| **Updates** | Manual (git pull + rebuild) | Automatic via marketplace |
| **Best for** | Developers, automation, remote work | General users, visual workflows |
| **Scriptability** | Highly scriptable | Limited to GUI |
| **Resource usage** | Lightweight | More resource-intensive |
| **Remote usage** | Works over SSH | Local only |

### When to Use Claude Code

✅ **Use Claude Code if**:
- You work primarily in the terminal
- You need to automate data workflows
- You work on remote servers via SSH
- You want tighter git integration
- You prefer keyboard-driven interfaces
- You need to script Claude interactions

### When to Use Claude Desktop

✅ **Use Claude Desktop if**:
- You prefer graphical interfaces
- You want the simplest installation
- You want automatic updates
- You need visual file browsing
- You're a non-technical user

**You can use both!** Install the Desktop Extension for general use and configure Claude Code for development work.

---

## Troubleshooting

### MCP Server Not Connecting

**Symptom**: Claude Code doesn't recognize qsv tools

**Solutions**:
1. **Check config file exists**:
   ```bash
   ls -la ~/.config/claude-code/mcp_settings.json
   # or on Windows: dir "%APPDATA%\Claude Code\mcp_settings.json"
   ```

2. **Validate JSON syntax**:
   ```bash
   cat ~/.config/claude-code/mcp_settings.json | jq .
   # Should pretty-print without errors
   ```

3. **Check server path**:
   ```bash
   node /path/to/qsv/.claude/skills/dist/mcp-server.js
   # Should start without errors
   ```

4. **Check Claude Code logs**:
   ```bash
   # macOS/Linux
   tail -f ~/.config/claude-code/logs/*.log

   # Windows
   type "%APPDATA%\Claude Code\logs\*.log"
   ```

### "qsv binary not found"

**Symptom**: MCP server connects but qsv commands fail

**Solutions**:
1. **Check auto-detection** using `qsv_config` tool in Claude Code

2. **Verify qsv in PATH**:
   ```bash
   which qsv  # macOS/Linux
   where qsv  # Windows
   ```

3. **Set explicit path** in config:
   ```json
   "env": {
     "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv"
   }
   ```

4. **Test qsv directly**:
   ```bash
   qsv --version
   qsv stats --help
   ```

### Permission Denied Errors

**Symptom**: Cannot access files even in allowed directories

**Solutions**:
1. **Check allowed directories** in config:
   ```json
   "QSV_MCP_ALLOWED_DIRS": "/Users/you/data:/Users/you/Downloads"
   ```

2. **Use absolute paths** in allowed dirs (no `~` or `$HOME`)

3. **Verify file permissions**:
   ```bash
   ls -l /path/to/file.csv
   # Should be readable by your user
   ```

4. **Check parent directory access**:
   ```bash
   # File must be in or under an allowed directory
   pwd  # Check current directory
   ```

### Large File Timeouts

**Symptom**: Commands timeout on large files

**Solutions**:
1. **Increase timeout** in config:
   ```json
   "env": {
     "QSV_MCP_TIMEOUT_MS": "900000"  // 15 minutes
   }
   ```

2. **Index large files first**:
   ```bash
   qsv index large-file.csv
   ```

3. **Use streaming commands** (avoid 🤯 memory-intensive operations)

4. **Enable compression**:
   ```bash
   qsv snappy compress large-file.csv
   # Creates large-file.csv.sz (much faster I/O)
   ```

### Server Updates Not Reflected

**Symptom**: Code changes don't appear after rebuild

**Solutions**:
1. **Rebuild TypeScript**:
   ```bash
   cd /path/to/qsv/.claude/skills
   npm run build
   ```

2. **Restart Claude Code completely**:
   ```bash
   # Kill all claude processes
   pkill -f claude

   # Start fresh
   claude
   ```

3. **Clear Node.js cache** (if issues persist):
   ```bash
   rm -rf dist/
   npm run build
   ```

---

## Advanced Usage

### Scripting with Claude Code

**Automated data processing pipeline**:

```bash
#!/bin/bash
# daily-report.sh

set -e

DATA_DIR="/data/daily"
REPORT_DIR="/reports/$(date +%Y-%m-%d)"

mkdir -p "$REPORT_DIR"
cd "$DATA_DIR"

# Use Claude Code for data processing
claude --non-interactive << EOF
Process today's sales data:

1. Validate sales.csv against schema
2. Calculate statistics for amount column
3. Generate frequency table for category
4. Create pivot table: category x payment_method
5. Save all results to $REPORT_DIR

Report any data quality issues found.
EOF
```

**Git pre-commit hook** (validate CSV data):

```bash
#!/bin/bash
# .git/hooks/pre-commit

CSV_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.csv$')

if [ -n "$CSV_FILES" ]; then
  echo "Validating CSV files..."

  for file in $CSV_FILES; do
    claude --non-interactive << EOF
Validate $file:
- Check schema is consistent
- Verify no duplicate IDs
- Confirm all required fields present
Report ONLY if issues found.
EOF
  done
fi
```

### Integration with Shell Tools

**Combine qsv with other CLI tools**:

```bash
# Find CSV files and analyze with Claude
find /data -name "*.csv" -mtime -7 | while read f; do
  echo "Analyzing $f"
  claude << EOF
Quick stats summary for $f - show row count, column count, and any data quality issues.
EOF
done
```

**Use Claude Code in pipes**:

```bash
# Generate report and pipe to formatter
claude << 'EOF' | pandoc -f markdown -t pdf -o report.pdf
Create a comprehensive data quality report for all CSV files in current directory.
Include:
- File inventory
- Schema validation results
- Statistics summaries
- Recommendations for improvements
EOF
```

### Remote Server Usage

**SSH into remote server**:

```bash
ssh data-server.com
cd /var/data
claude
```

Then use qsv tools as if local:
```
List CSV files here and analyze the largest one
```

**VS Code Remote SSH** with Claude Code:

1. Connect to remote via VS Code Remote SSH
2. Open terminal in VS Code
3. Run `claude` in remote terminal
4. qsv operations run on remote server

---

## Configuration Examples

### Development Workflow

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/Users/dev/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_WORKING_DIR": "/Users/dev/projects",
        "QSV_MCP_ALLOWED_DIRS": "/Users/dev/projects:/tmp",
        "QSV_MCP_TIMEOUT_MS": "600000",
        "QSV_MCP_MAX_OUTPUT_SIZE": "104857600"
      }
    }
  }
}
```

### Data Science Workflow

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/home/scientist/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/home/scientist/.cargo/bin/qsv",
        "QSV_MCP_ALLOWED_DIRS": "/data:/home/scientist/notebooks:/mnt/storage",
        "QSV_MCP_TIMEOUT_MS": "1800000",
        "QSV_MCP_CONVERTED_LIFO_SIZE_GB": "5"
      }
    }
  }
}
```

### Minimal Configuration

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"]
    }
  }
}
```

This uses all defaults:
- Auto-detects qsv binary
- Uses current directory as working dir
- Allows only working directory access
- 5 minute timeout
- 50MB output limit

---

## Tips & Best Practices

### 1. Use Relative Paths

```bash
cd ~/data
claude
```

Then in Claude:
```
Process ./customers.csv  # Easier than full paths
```

### 2. Leverage Shell History

Claude Code commands are in your shell history:

```bash
history | grep claude
# Review previous data operations
```

### 3. Create Aliases

```bash
# Add to ~/.bashrc or ~/.zshrc
alias qsv-claude='cd ~/data && claude'
alias qsv-stats='claude -c "Calculate statistics for"'
```

### 4. Use in Scripts

Claude Code supports non-interactive mode:

```bash
claude --non-interactive << EOF
Analyze data.csv and save report to report.md
EOF
```

### 5. Combine with Git

```bash
# After data processing
git add processed_data.csv
git commit -m "$(claude << 'EOF'
Summarize changes to processed_data.csv in one line
EOF
)"
```

### 6. Directory-specific Configs

Use different configs per project:

```bash
# In project root
echo '{"mcpServers": {"qsv": {...}}}' > .claude/mcp_settings.json
export CLAUDE_MCP_CONFIG=.claude/mcp_settings.json
claude
```

---

## Frequently Asked Questions

### Can I use Claude Code and Claude Desktop simultaneously?

Yes! They use separate configuration files:
- Claude Code: `~/.config/claude-code/mcp_settings.json`
- Claude Desktop: `~/Library/Application Support/Claude/claude_desktop_config.json`

You can even point them to the same MCP server instance.

### Does Claude Code require a Pro subscription?

Claude Code follows the same subscription model as Claude. Check [claude.ai/pricing](https://claude.ai/pricing) for current requirements.

### Can I use qsv over SSH?

Yes! Install Claude Code on the remote server and configure the qsv MCP server there. All operations run remotely.

### How do I update the MCP server?

```bash
cd /path/to/qsv/.claude/skills
git pull
npm install
npm run build
# Restart Claude Code
```

### Can I disable auto-detection and always use a specific qsv path?

Yes, set `QSV_MCP_BIN_PATH` explicitly:

```json
"env": {
  "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv"
}
```

### What happens if qsv is not installed?

The MCP server will fail validation and show an error. Use `qsv_config` to see diagnostics about why detection failed.

---

## Getting Help

### Documentation

- **Quick Start**: This guide
- **Desktop Extension**: [DESKTOP_EXTENSION.md](./DESKTOP_EXTENSION.md)
- **Full MCP Guide**: [README.md](./README.md)
- **Filesystem Usage**: [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)
- **qsv Documentation**: https://github.com/dathere/qsv#commands

### Support

- **GitHub Issues**: https://github.com/dathere/qsv/issues
- **Discussions**: https://github.com/dathere/qsv/discussions
- **Discord**: https://discord.gg/dathere (coming soon)

### Logs

**Claude Code logs**:
- macOS/Linux: `~/.config/claude-code/logs/`
- Windows: `%APPDATA%\Claude Code\logs\`

**MCP Server logs**:
- Look for `[qsv]` prefixed messages in Claude Code logs
- Enable debug mode: `export NODE_DEBUG=mcp` before starting Claude Code

---

## What's Next?

Once you're comfortable with basic usage, explore:

1. **Pipeline Composition** - Chain multiple qsv commands together
2. **Complex Queries** - Use `qsv sqlp` for SQL-like data queries
3. **Data Validation** - Create automated validation workflows
4. **Integration Scripts** - Build data processing automation
5. **Remote Workflows** - Process data on remote servers

Check out the [examples directory](./examples/) for inspiration!

---

**Status**: ✅ Production Ready
**Compatibility**: Claude Code v1.0+
**MCP Protocol**: 2025-06-18
**Last Updated**: 2026-01-10
