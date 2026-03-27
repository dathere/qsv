# Getting Started with qsv MCP Server + Cowork Plugin

**qsv** is a free, open-source data-wrangling toolkit that gives Claude the ability to work directly with data files on your computer — CSV, Excel, JSONL, Parquet, and more with blazing speed. It offers hundreds of built-in functions for data transformation, analysis, and validation, but typically requires being comfortable with command-line tools.

With the qsv MCP Server and Cowork plugin, you can use all of qsv's power through natural language commands in **Claude Desktop** — no programming required. Just ask Claude to analyze, clean, transform, or visualize your data files, and it will translate your requests into qsv commands behind the scenes.

No uploads, no file size limits, no data leaving your machine.

**No programming experience required.** Just follow the steps in order.

---

## What You'll Need

- **Claude Desktop** — [download here](https://claude.ai/download)
- An **internet connection** for downloads
- About **10 minutes** of your time

---

## Step 1: Install the MCP Server

The qsv MCP Server lets Claude read, analyze, and transform your local data files in place — only uploading statistical metadata to Claude to inform its operations. This section shows how to install and use it in **Claude Desktop Chat** and **Claude Cowork**; the same server also works with **Claude Code** and **Gemini CLI** via a separate build-from-source flow described [below](#for-developers).

> **No manual binary install needed.** The MCPB bundle automatically downloads and installs the qsv binary (qsvmcp) for you.

### Download

Go to: **<https://github.com/dathere/qsv/releases/latest>**

Download the `.mcpb` file and save it to your Downloads folder.

### Install in Claude Desktop

1. Double-click the downloaded `.mcpb` file. This should open Claude Desktop.
2. Click "Install". A Confirmation dialog will appear confirming the installation. Click "Install" again to proceed.
3. That's it! The MCP Server is now installed and running in the background.

### Configure (OPTIONAL)

After installation, you can customize the MCP Server settings in **Claude Desktop > Settings > Extensions > qsv**. The defaults work for most users, but here are some key settings you might want to adjust:

- **Working Directory** — where Claude looks for your data files by default.
  - **macOS:** `$HOME/Downloads`
  - **Windows:** `%USERPROFILE%\Downloads`

- **Allowed Directories** — folders Claude is allowed to access (for security).
  - **macOS:** `$HOME/Downloads:$HOME/Documents` (use `:` to separate folders)
  - **Windows:** `%USERPROFILE%\Downloads;%USERPROFILE%\Documents` (use `;` to separate folders)

- **qsv Binary Path** — usually auto-detected. If not found, the installer will prompt you to download it.

### Restart and Verify

Close and reopen Claude Desktop. Then start a new conversation and ask:

> "List data files in my Downloads folder"

If Claude shows you a list of files, the qsv MCP Server is working.

---

## Step 2 (Optional): Install the Cowork Plugin

The Cowork plugin adds a workflow layer on top of the MCP Server, giving **Claude Cowork** sessions access to domain skills, slash commands, and specialized subagents for guided data workflows.

> Cowork requires a **Claude Pro plan** (or higher). [See plans and pricing.](https://claude.com/pricing)

**Prerequisites:**
The qsv MCP Server must be installed first ([Step 1](#step-1-install-the-mcp-server))

### Install via Marketplace

1. Click your **profile icon** (bottom-left) > **Settings**
2. Click on **Capabilities** -> **Skills** -> **Go to Customize**
3. Go to **Personal plugins** and click on the + sign
4. Select "Add marketplace" from the list
5. A dialog will appear. Enter `dathere/qsv` and click "Synch"
6. You will be on the **Customize** panel again. Click the + sign next to "Personal plugins" and select "Browse plugins"
7. In the **Browse Plugins** dialog, click on the **Personal** tab and select the "qsv" plugin.
8. Click "Install" on the "Qsv-data-wrangling" plugin

### Verify

1. Start a **new Cowork session** in Claude Desktop
2. Check the **Context panel** — the qsv skills should appear
3. Try a slash command like `/data-profile` to confirm everything works

### What the Plugin Adds

| Component | Details |
|-----------|---------|
| 6 domain skills | bls-query, csv-wrangling, data-quality, genai-disclaimer, qsv-performance, reproducible-analysis |
| 8 slash commands | /csv-query, /data-clean, /data-convert, /data-describe, /data-join, /data-profile, /data-validate, /data-viz |
| 3 subagents | data-analyst, data-wrangler, policy-analyst |

---

## For Developers

The qsv MCP Server also works with CLI-based tools:

- **Claude Code** — Full setup guide: [CLAUDE_CODE.md](./CLAUDE_CODE.md)
- **Gemini CLI** — Full setup guide: [GEMINI_CLI.md](./GEMINI_CLI.md)

Both require building the MCP server from source (`npm install && npm run build` in `.claude/skills/`).

---

## Optional: Additional Data Sources

Once the core setup is complete, you can add more MCP servers to give Claude access to external data:

| Server | What it provides |
|--------|-----------------|
| **US Census Bureau** | Population, demographics, economics — via a local Docker container |
| **Wikidata** | The free, structured knowledge graph maintained by the Wikimedia Foundation |

See **[ADDITIONAL_SERVERS.md](./ADDITIONAL_SERVERS.md)** for installation instructions.

---

## Troubleshooting

### MCP Server

| Problem | Solution |
|---------|----------|
| "qsv binary not found" | The MCPB installer should auto-install qsvmcp. Check Settings > Extensions > qsv to verify the binary path. If needed, download manually: [macOS](https://qsv.dathere.com/download/macos-silicon) \| [Windows](https://qsv.dathere.com/download/windows). |
| Claude says it can't find your file | Use the full file path (e.g., `/Users/you/Downloads/data.csv` or `C:\Users\you\Downloads\data.csv`) or ask Claude to "list data files" first. |
| Extension not appearing | Restart Claude Desktop completely. Check Settings > Extensions for "qsv Data Wrangling". If missing, reinstall the `.mcpb` file. |
| "qsv command not found" | Use `qsv_config` in a Claude chat to see diagnostics. Try setting the binary path manually in Settings > Extensions > qsv. |
| Large file timeouts | Increase the command timeout in Settings > Extensions > qsv > Advanced Settings. Pre-index large files by asking Claude to "index large-file.csv". |
| **macOS:** "Operation not permitted" running qsv | Run `xattr -d com.apple.quarantine $(which qsvmcp)` in Terminal to clear the quarantine flag. |
| **macOS:** "Permission denied" | Try prefixing with `sudo`. You may also need to allow the app in **System Settings > Privacy & Security**. |
| Config file won't save | Make sure Claude Desktop is closed while editing the config file. |

### Cowork Plugin

| Problem | Solution |
|---------|----------|
| Cowork not available | Claude Cowork requires at least a [Claude Pro subscription](https://claude.com/pricing). |
| Marketplace add fails | Check your internet connection and that you can access `github.com/dathere/qsv`. |
| Slash commands don't appear | Start a **new** Cowork session after installation. Existing sessions won't pick up the plugin. |
| Skills not showing in Context panel | Restart Claude Desktop and start a fresh Cowork session. |
| Plugin not updating | Go to **Settings** -> **Capabilities** -> **Skills** -> **Go to Customize** -> **Personal plugins** -> **+ sign** -> **Browse plugins** and click on the "qsv" plugin to manually check for updates. |

---

## What's Next

- **Add more data sources** — Install the US Census Bureau and Wikidata MCP servers: [ADDITIONAL_SERVERS.md](./ADDITIONAL_SERVERS.md)
- **Learn about file access** — Supported formats, security, and configuration: [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)
- **Full technical reference** — [Desktop Extension Reference](../desktop/README-MCPB.md) | [MCP Server README](../../README-MCP.md)
- **All qsv commands** — [github.com/dathere/qsv](https://github.com/dathere/qsv#commands)

---

## Example Prompts

**Local file analysis:**
> "Show me statistics for sales.csv in my Downloads folder"

**SQL queries:**
> "Run a SQL query on orders.csv to find the top 10 customers by total spend"

**Cowork with the plugin:**
> "/data-profile my latest CSV file, then /data-clean it based on the quality issues you find"

**Combining data sources:**
> "Look up the Wikidata entities for all US state capitals, then pull their Census population data and save the results as a CSV"

---

*Last updated: 2026-03-23*
