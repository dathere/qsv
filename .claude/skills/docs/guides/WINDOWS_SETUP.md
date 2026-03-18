# Windows Setup — qsv MCP Server + Cowork Plugin

This guide walks you through installing the qsv MCP Server and the optional Cowork Plugin on Windows.

**No programming experience required.** Just follow the steps in order.

[< Back to Start Here](./START_HERE.md)

---

## Table of Contents

1. [Prerequisites](#1-prerequisites)
2. [Install the qsv MCP Server (MCPB Bundle)](#2-install-the-qsv-mcp-server-mcpb-bundle)
3. [Install the qsv Cowork Plugin](#3-install-the-qsv-cowork-plugin)
4. [Claude Pro and Cowork](#4-claude-pro-and-cowork)
5. [Troubleshooting](#5-troubleshooting)
6. [What's Next](#6-whats-next)

---

## 1. Prerequisites

- **Windows 10** (version 2004 or later) or **Windows 11**
- **Claude Desktop** installed — [download here](https://claude.ai/download)
- A **Claude Pro plan** (or higher) — required for Claude Cowork ([see pricing](https://claude.ai/pricing))
- An **internet connection** for downloads
- About **10 minutes** of your time

You'll use **PowerShell** to run a few commands. Don't worry — every command is provided for you to copy and paste. To open PowerShell: press `Win + X` and select **Windows Terminal** (or **PowerShell**).

---

## 2. Install the qsv MCP Server (MCPB Bundle)

The qsv MCP server lets Claude read, analyze, and transform your local data files (CSV, Excel, TSV, JSONL, and more) — without uploading anything.

> **No manual binary install needed.** The MCPB bundle automatically downloads and installs the qsv binary (qsvmcp) for you.

### Download the MCPB Bundle

Go to: **<https://github.com/dathere/qsv/releases/latest>**

Download the `.mcpb` file and save it to your Downloads folder.

### Install in Claude Desktop

1. Open **Claude Desktop**
2. Click your **profile icon** (bottom-left) > **Settings**
3. Go to the **Extensions** tab
4. Click **"Install from file"**
5. Select the `.mcpb` file you downloaded
6. Click **Install**

### Configure

After installation, you'll be prompted to set up:

- **Working Directory** — where Claude looks for your data files by default.
  Example: `%USERPROFILE%\Downloads`

- **Allowed Directories** — folders Claude is allowed to access (for security).
  Example: `%USERPROFILE%\Downloads;%USERPROFILE%\Documents`
  (Use `;` to separate multiple folders on Windows.)

- **qsv Binary Path** — usually auto-detected. If not found, the installer will prompt you to download it.

### Restart and Verify

Close and reopen Claude Desktop. Then start a new conversation and ask:

> "List data files in my Downloads folder"

If Claude shows you a list of files, the qsv MCP server is working.

---

## 3. Install the qsv Cowork Plugin

The Cowork plugin adds a workflow layer on top of the MCP server, giving Cowork sessions access to domain skills, slash commands, and specialized subagents for guided data workflows.

**What it adds:**

| Component | Details |
|-----------|---------|
| 3 domain skills | csv-wrangling, data-quality, qsv-performance |
| 6 slash commands | /csv-query, /data-clean, /data-convert, /data-describe, /data-join, /data-profile |
| 2 subagents | data-analyst, data-wrangler |

**Prerequisites:**
- The qsv MCP Server must be installed first ([Section 2](#2-install-the-qsv-mcp-server-mcpb-bundle))
- **Claude Code** must be installed — [install instructions](https://docs.anthropic.com/en/docs/claude-code/overview)

### Install via Marketplace

The qsv plugin is distributed through a plugin marketplace hosted in the qsv GitHub repository. Open **PowerShell** and run:

```powershell
claude plugin marketplace add dathere/qsv
claude plugin install qsv-data-wrangling@qsv-plugins
```

That's it! The plugin is automatically cached and kept up to date.

### Verify

1. Start a **new Cowork session** in Claude Desktop
2. Check the **Context panel** — the qsv skills should appear
3. Try a slash command like `/data-profile` to confirm everything works

---

## 4. Claude Pro and Cowork

**Claude Cowork** lets Claude work on longer, multi-step tasks in the background — like cleaning a dataset, running multiple queries, and summarizing results.

Cowork requires at least a **Claude Pro** plan. [See plans and pricing here.](https://claude.ai/pricing)

With the **qsv Cowork plugin** installed ([Section 3](#3-install-the-qsv-cowork-plugin)), your Cowork sessions gain:

- **Domain skills** that give Claude deep knowledge of CSV wrangling, data quality, and qsv performance optimization
- **Slash commands** like `/data-profile` and `/data-clean` for common data workflows
- **Specialized subagents** (data-analyst and data-wrangler) that can autonomously handle complex data tasks

**Try it out:** Open a Cowork session and type:

> "/data-profile ~\Downloads\sales.csv"

Claude will generate a comprehensive data profile with statistics, frequency distributions, and quality checks.

---

## 5. Troubleshooting

### General

| Problem | Solution |
| ------- | -------- |
| Claude doesn't seem to use the MCP servers | Make sure you restarted Claude Desktop after setup. Check Settings > Extensions. |
| Config file won't save | Make sure Claude Desktop is closed while editing the config file. |

### qsv MCP Server

| Problem | Solution |
| ------- | -------- |
| "qsv binary not found" | The MCPB installer should auto-install qsvmcp. Check Settings > Extensions > qsv to verify the binary path. If needed, download qsv manually from <https://qsv.dathere.com/download/windows>. |
| Claude says it can't find your file | Use the full file path (e.g., `C:\Users\you\Downloads\data.csv`) or ask Claude to "list data files" first. |
| Extension not appearing | Restart Claude Desktop completely. Check Settings > Extensions for "qsv Data Wrangling". If missing, reinstall the `.mcpb` file. |
| "qsv command not found" | Use `qsv_config` in a Claude chat to see diagnostics. Try setting the binary path manually in Settings > Extensions > qsv. |
| Large file timeouts | Increase the command timeout in Settings > Extensions > qsv > Advanced Settings. Pre-index large files: `qsv index large-file.csv`. |

### qsv Cowork Plugin

| Problem | Solution |
| ------- | -------- |
| `claude` command not found | Install Claude Code first: see [installation instructions](https://docs.anthropic.com/en/docs/claude-code/overview). |
| Marketplace add fails | Check your internet connection and that you can access `github.com/dathere/qsv`. |
| Slash commands don't appear | Start a **new** Cowork session after installation. Existing sessions won't pick up the plugin. |
| Skills not showing in Context panel | Restart Claude Desktop and start a fresh Cowork session. |
| Plugin not updating | Run `claude plugin update qsv-data-wrangling@qsv-plugins` to pull the latest version. |

---

## 6. What's Next

- **Add more data sources** — Install the US Census Bureau and Wikidata MCP servers: [ADDITIONAL_SERVERS.md](./ADDITIONAL_SERVERS.md)
- **Learn about file access** — Supported formats, security, and configuration: [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)
- **Explore the full reference** — [Desktop Extension Reference](../desktop/README-MCPB.md)

---

*Last updated: 2026-03-18*
