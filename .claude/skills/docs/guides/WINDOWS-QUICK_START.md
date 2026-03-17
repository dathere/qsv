# "Have we achieved Accelerated Civic Intelligence (ACI)?" Quick Start — Windows

This guide walks you through installing Claude Desktop and three powerful data MCP servers on Windows:

1. **qsv MCP Server** — Slice, dice and run Polars SQL queries on local CSV, Excel, JSONL and Parquet files with qsv's full command set
2. **US Census Bureau MCP Server** — Access US Census data (population, demographics, economics) via a local Docker container
3. **Wikidata MCP Server** — Query the Wikidata knowledge graph

When you're done, Claude will be able to work with your local data files, run SQL queries, pull US Census data, and look up Wikidata information — all from a single Claude Chat or Cowork window.

This will allow you to recreate the scenarios described in the ["Have we achieved ACI?"](https://dathere.com/2026/01/the-peoples-api-is-finally-here/) blog post series.

**No programming experience required.** Just follow the steps in order.

---

## Table of Contents

1. [What You'll Need](#1-what-youll-need)
2. [Install the qsv MCP Server (MCPB Bundle)](#2-install-the-qsv-mcp-server-mcpb-bundle)
3. [Install the qsv Cowork Plugin](#3-install-the-qsv-cowork-plugin)
4. [Install the US Census Bureau MCP Server](#4-install-the-us-census-bureau-mcp-server)
5. [Install the Wikidata MCP Server](#5-install-the-wikidata-mcp-server)
6. [Editing the Claude Desktop Config File](#6-editing-the-claude-desktop-config-file)
7. [Claude Pro and Cowork](#7-claude-pro-and-cowork)
8. [Troubleshooting](#8-troubleshooting)
9. [What's Next](#9-whats-next)

---

## 1. What You'll Need

Before you start, make sure you have:

- **Windows 10** (version 2004 or later) or **Windows 11**
- **Claude Desktop** installed — [download here](https://claude.ai/download)
- A **Claude Pro plan** (or higher) — required for Claude Cowork ([see pricing](https://claude.ai/pricing))
- An **internet connection** for downloads
- About **10 minutes** of your time

You'll use **PowerShell** to run a few commands. Don't worry — every command is provided for you to copy and paste. To open PowerShell: press `Win + X` and select **Windows Terminal** (or **PowerShell**).

> **Two pieces, one toolkit:**
>
> | Component | What it is | What it provides |
> |-----------|-----------|-----------------|
> | `.mcpb` (MCP Server) | The qsv tools Claude can call in Chat + Cowork | 52 data-wrangling commands, SQL queries, file conversion. **Auto-installs the qsv binary** — no separate download needed. |
> | `.plugin` (Cowork Plugin) | Workflow layer for Cowork sessions | 3 domain skills, 5 slash commands, 2 subagents for guided data workflows |
>
> Install both for the full experience. The `.mcpb` is required; the `.plugin` is optional but recommended for Cowork users.

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
| 5 slash commands | /csv-query, /data-clean, /data-convert, /data-join, /data-profile |
| 2 subagents | data-analyst, data-wrangler |

**Prerequisites:**
- The qsv MCP Server must be installed first ([Section 2](#2-install-the-qsv-mcp-server-mcpb-bundle))
- You must have opened Cowork at least once with any marketplace plugin installed (e.g., the Data or Productivity plugin)

### Download

Go to: **<https://github.com/dathere/qsv/releases/latest>**

Download both:
- The `.plugin` file (e.g., `qsv-data-wrangling-17.0.0.plugin`)
- The `install-cowork-plugin.ps1` script

Save both to the same folder (e.g., your Downloads folder).

### Install

Open **PowerShell** and run:

```powershell
cd ~\Downloads
powershell -ExecutionPolicy Bypass -File install-cowork-plugin.ps1 qsv-data-wrangling-17.0.0.plugin
```

> **Note:** Replace `17.0.0` with the version you downloaded.
>
> The `-ExecutionPolicy Bypass` flag allows the script to run without changing your system's execution policy.

The script will:
1. Extract the plugin files
2. Register the plugin in Claude Desktop's Cowork environment
3. Display a confirmation when done

### Verify

1. Start a **new Cowork session** in Claude Desktop
2. Check the **Context panel** — the qsv skills should appear
3. Try a slash command like `/data-profile` to confirm everything works

---

## 4. Install the US Census Bureau MCP Server

The [US Census Bureau MCP server](https://github.com/dathere/us-census-bureau-data-api-mcp) gives Claude access to US Census data (population, demographics, economics, and more).

This one requires a few extra tools: **Docker Desktop**, **Node.js**, and a free **Census API key**.

> **Need Scoop?** Node.js is installed via [Scoop](https://scoop.sh), a command-line installer for Windows. If you don't have it yet, install it first in PowerShell:
>
> ```powershell
> Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
> irm get.scoop.sh | iex
> ```
>
> Verify with `scoop --version`.

### 4a. Install Docker Desktop

Docker is a tool that runs applications in isolated containers. The Census server uses it for its database.

> **Prerequisite:** Docker Desktop for Windows requires **WSL 2** (Windows Subsystem for Linux). If you haven't enabled it yet:
>
> ```powershell
> wsl --install
> ```
>
> Restart your computer after running this command.

1. Download **Docker Desktop** from: **<https://www.docker.com/products/docker-desktop/>**
2. Run the installer and follow the setup wizard
3. When prompted, ensure **"Use WSL 2 instead of Hyper-V"** is checked
4. After installation, open **Docker Desktop** and wait for it to finish starting (you'll see a green "Running" indicator)

> **First time?** Docker may require a restart and will ask you to sign in or create a Docker account. A free account is fine.

### 4b. Install Node.js

```powershell
scoop install nodejs
```

Verify:

```powershell
node --version
```

You should see `v20.x.x` or higher.

### 4c. Get a Census API Key

1. Go to: **<https://api.census.gov/data/key_signup.html>**
2. Fill out the form (name and email)
3. Check your email for the API key — it arrives almost instantly
4. **Save your API key** somewhere handy (you'll need it in a moment)

### 4d. Download and Set Up the Census Server

In PowerShell, run these commands one at a time:

```powershell
cd ~\Documents
```

```powershell
git clone https://github.com/dathere/us-census-bureau-data-api-mcp.git
```

```powershell
cd us-census-bureau-data-api-mcp
```

> **Don't have `git`?** Install it with `scoop install git`, or [download the ZIP file](https://github.com/dathere/us-census-bureau-data-api-mcp/archive/refs/heads/main.zip), unzip it, and move the folder to your Documents.

### 4e. Initialize the Database

Make sure **Docker Desktop is running**, then run:

```powershell
docker compose --profile prod run --rm census-mcp-db-init sh -c "npm run migrate:up && npm run seed"
```

This downloads the required containers and sets up the Census database. It may take a few minutes the first time.

### 4f. Add to Claude Desktop Config

Open the Claude Desktop config file (see [Section 6](#6-editing-the-claude-desktop-config-file) for instructions) and add this inside `"mcpServers"`:

```json
"mcp-census-api": {
  "command": "bash",
  "args": [
    "C:\\Users\\YOUR_USERNAME\\Documents\\us-census-bureau-data-api-mcp\\scripts\\mcp-connect.sh"
  ],
  "env": {
    "CENSUS_API_KEY": "YOUR_CENSUS_API_KEY"
  }
}
```

Replace:
- `YOUR_USERNAME` with your Windows username (run `$env:USERNAME` in PowerShell to check)
- `YOUR_CENSUS_API_KEY` with the API key from your email

> **Note:** The Census server uses `bash` via WSL. The `"command": "bash"` above relies on WSL's `bash.exe` being on your Windows PATH (it is by default after WSL installation). To verify, run `where bash` in PowerShell — it should show `C:\Windows\System32\bash.exe`. If not found, make sure WSL is installed (see [Section 4a](#4a-install-docker-desktop)). If you have Git Bash installed and it resolves first, use `"command": "wsl"` with `"args": ["bash", "path/to/mcp-connect.sh"]` instead.

### Verify Census Server

Close and reopen Claude Desktop. Make sure **Docker Desktop is running**, then ask:

> "What is the population of California?"

If Claude returns Census data, the server is working.

---

## 5. Install the Wikidata MCP Server

The [Wikidata MCP Server](https://github.com/philippesaade-wmde/WikidataMCP) gives Claude access to [Wikidata](https://www.wikidata.org/) — the free, structured knowledge graph maintained by the Wikimedia Foundation. It provides tools for searching entities, running SPARQL queries, reading entity claims, and more.

There are two ways to set it up. **Method A is recommended** — it uses a hosted server so you don't need to install anything.

### Method A: Remote Hosted Server (Recommended)

Wikimedia Deutschland hosts a public endpoint for the Wikidata MCP Server. No local installation is needed — just add the config to Claude Desktop.

Open the Claude Desktop config file (see [Section 6](#6-editing-the-claude-desktop-config-file) for instructions) and add this inside `"mcpServers"`:

```json
"Wikidata MCP": {
  "url": "https://wd-mcp.wmcloud.org/mcp/"
}
```

That's it! Skip to [Verify Wikidata Server](#verify-wikidata-server) below.

### Method B: Local Installation (Alternative)

If you prefer to run the server locally (e.g., for development or offline use), follow these steps.

**Install uv** (a fast Python package manager) — requires [Scoop](https://scoop.sh):

```powershell
scoop install uv
```

**Clone the repository:**

```powershell
cd ~\Documents
```

```powershell
git clone https://github.com/philippesaade-wmde/WikidataMCP.git
```

```powershell
cd WikidataMCP
```

**Install dependencies:**

```powershell
uv sync
```

**Add to Claude Desktop config:**

Open the Claude Desktop config file (see [Section 6](#6-editing-the-claude-desktop-config-file) for instructions) and add this inside `"mcpServers"`:

```json
"Wikidata MCP": {
  "command": "uv",
  "args": ["run", "fastmcp", "run", "./main.py"],
  "cwd": "C:\\Users\\YOUR_USERNAME\\Documents\\WikidataMCP"
}
```

Replace `YOUR_USERNAME` with your Windows username (run `$env:USERNAME` in PowerShell to check).

### Verify Wikidata Server

Close and reopen Claude Desktop. Then start a new conversation and ask:

> "What is the Wikidata entity for Albert Einstein?"

If Claude returns Wikidata entity information (like Q937), the server is working.

---

## 6. Editing the Claude Desktop Config File

Some MCP servers need to be added manually to Claude Desktop's configuration file. Here's how to find and edit it.

### Where is the Config File?

```
%APPDATA%\Claude\claude_desktop_config.json
```

This typically resolves to:
```
C:\Users\YOUR_USERNAME\AppData\Roaming\Claude\claude_desktop_config.json
```

### How to Open It

**Option A — PowerShell (easiest):**

```powershell
notepad "$env:APPDATA\Claude\claude_desktop_config.json"
```

**Option B — File Explorer:**

1. Press `Win + R` to open the Run dialog
2. Type `%APPDATA%\Claude` and press Enter
3. Double-click `claude_desktop_config.json` to open it in Notepad

### What the Config Looks Like

After setting up all three servers, your config file should look something like this:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "C:\\path\\auto-configured-by-mcpb\\dist\\mcp-server.js"
      ],
      "env": {
        "QSV_MCP_WORKING_DIR": "C:\\Users\\YOUR_USERNAME\\Downloads",
        "QSV_MCP_ALLOWED_DIRS": "C:\\Users\\YOUR_USERNAME\\Downloads;C:\\Users\\YOUR_USERNAME\\Documents"
      }
    },
    "mcp-census-api": {
      "command": "bash",
      "args": [
        "C:\\Users\\YOUR_USERNAME\\Documents\\us-census-bureau-data-api-mcp\\scripts\\mcp-connect.sh"
      ],
      "env": {
        "CENSUS_API_KEY": "YOUR_CENSUS_API_KEY"
      }
    },
    "Wikidata MCP": {
      "url": "https://wd-mcp.wmcloud.org/mcp/"
    }
  }
}
```

> **Important:** The `qsv` entry is managed by the MCPB installer — you shouldn't need to edit it by hand. The qsv binary path is auto-detected. The Census and Wikidata entries are added manually.
>
> **Tip:** After editing the config file, always close and reopen Claude Desktop for changes to take effect.

---

## 7. Claude Pro and Cowork

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

## 8. Troubleshooting

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

### qsv Cowork Plugin

| Problem | Solution |
| ------- | -------- |
| "local-desktop-app-uploads not found" | You need to have opened Cowork at least once with any marketplace plugin installed. Install one from the marketplace (e.g., Data or Productivity), then retry the installer. |
| Slash commands don't appear | Start a **new** Cowork session after installation. Existing sessions won't pick up the plugin. |
| Skills not showing in Context panel | Restart Claude Desktop and start a fresh Cowork session. |
| ExecutionPolicy error running the script | Use the full command: `powershell -ExecutionPolicy Bypass -File install-cowork-plugin.ps1 <plugin-file>`. This bypasses the policy for that single script run without changing system settings. |

### Wikidata MCP Server

| Problem | Solution |
| ------- | -------- |
| Wikidata server not responding | Check your internet connection. The hosted server at `wd-mcp.wmcloud.org` requires internet access. |
| Timeout on SPARQL queries | Complex queries may take time. Try simpler queries first, or break them into smaller parts. |
| Local server won't start | Make sure `uv` is installed (`scoop install uv`) and you ran `uv sync` in the WikidataMCP directory. |

### US Census Bureau MCP Server

| Problem | Solution |
| ------- | -------- |
| "Cannot connect to Docker daemon" | Open Docker Desktop and wait for it to finish starting (green "Running" indicator). |
| Database initialization fails | Make sure Docker Desktop is running, then try the `docker compose` command again. |
| "Invalid API key" | Double-check your Census API key. You can request a new one at <https://api.census.gov/data/key_signup.html>. |
| Census server won't start | Make sure Docker Desktop is running — it's needed every time you use the Census server. |
| WSL 2 not installed | Run `wsl --install` in PowerShell (as Administrator) and restart your computer. Docker Desktop requires WSL 2. |

---

## 9. What's Next

Now that everything is set up, here are some things you can try:

### Example Prompts

**Using qsv (local file analysis):**
> "Show me statistics for sales.csv in my Downloads folder"

**Using Census data:**
> "Compare the median household income of Texas and New York using Census data"

**Using Wikidata:**
> "Find all Nobel Prize winners in Physics from the last 10 years using Wikidata"

**Combining servers:**
> "Look up the Wikidata entities for all US state capitals, then pull their Census population data and save the results as a CSV"

**Using Cowork with the plugin:**
> "/data-profile my latest CSV file, then /data-clean it based on the quality issues you find"

### Learn More

- **qsv MCP Server**: [Desktop Extension Guide](../desktop/README-MCPB.md) | [Filesystem Usage](./FILESYSTEM_USAGE.md)
- **US Census Bureau MCP Server**: [GitHub Repository](https://github.com/dathere/us-census-bureau-data-api-mcp)
- **Claude Desktop**: [Official Documentation](https://claude.ai/docs)
- **Wikidata**: [Documentation](https://www.wikidata.org/wiki/Wikidata:MCP)

---

*Last updated: 2026-03-16*
