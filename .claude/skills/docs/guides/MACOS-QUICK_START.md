# Quick Start: Setting Up Claude Desktop with Data MCP Servers on macOS

This guide walks you through installing Claude Desktop and three powerful data MCP servers on your Mac. When you're done, Claude will be able to work with your local CSV and Excel files, run SQL queries, and pull US Census data — all from a single chat window.

This will allow you to recreate the scenarios described in the ["Have we achieved AGI?"](https://dathere.com/2026/01/the-peoples-api-is-finally-here/) blog post series.

**No programming experience required.** Just follow the steps in order.

---

## Table of Contents

1. [What You'll Need](#1-what-youll-need)
2. [Install Homebrew](#2-install-homebrew)
3. [Install DuckDB CLI](#3-install-duckdb-cli)
4. [Install the qsv Binary](#4-install-the-qsv-binary)
5. [Install the qsv MCP Server](#5-install-the-qsv-mcp-server-mcpb-bundle)
6. [Install the MotherDuck MCP Server](#6-install-the-motherduck-mcp-server-in-memory-duckdb)
7. [Install the US Census Bureau MCP Server](#7-install-the-us-census-bureau-mcp-server)
8. [Editing the Claude Desktop Config File](#8-editing-the-claude-desktop-config-file)
9. [Claude Pro and Cowork](#9-claude-pro-and-cowork)
10. [Troubleshooting](#10-troubleshooting)
11. [What's Next](#11-whats-next)

---

## 1. What You'll Need

Before you start, make sure you have:

- A **Mac** (Apple Silicon)
- **Claude Desktop** installed — [download here](https://claude.ai/download)
- A **Claude Pro plan** (or higher) — required for Claude Cowork ([see pricing](https://claude.ai/pricing))
- An **internet connection** for downloads
- About **15–20 minutes** of your time

You'll also use **Terminal** (found in Applications > Utilities) to run a few commands. Don't worry — every command is provided for you to copy and paste.

---

## 2. Install Homebrew

[Homebrew](https://brew.sh) is the standard package manager for macOS. It makes installing command-line tools easy.

**If you already have Homebrew**, skip to [Step 3](#3-install-duckdb-cli).

Open **Terminal** and paste this command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Follow the on-screen instructions. When it's done, verify the installation:

```bash
brew --version
```

You should see something like `Homebrew 4.x.x`.

---

## 3. Install DuckDB CLI

[DuckDB](https://duckdb.org) is a fast SQL database engine that works with local files. Both the qsv and MotherDuck MCP servers use it under the hood.

In Terminal, run:

```bash
brew install duckdb
```

Verify it installed correctly:

```bash
duckdb --version
```

You should see a version number like `v1.x.x`.

---

## 4. Install the qsv Binary

qsv is the engine that powers the qsv MCP server. It's a high-performance toolkit for working with CSV, Excel, and other tabular data files.

### Download

Go to: **<https://qsv.dathere.com/download/macos-silicon>**

This downloads a DMG file containing the qsv binary.

> **Intel Mac?** Use <https://qsv.dathere.com/download/macos-intel> instead.

### Install

1. **Open the DMG** by double-clicking it in Finder.
2. **Copy the `qsv` binary** to `/usr/local/bin/`:

   If the folder doesn't exist yet, create it first:

   ```bash
   sudo mkdir -p /usr/local/bin
   ```

   Then copy the binary (adjust the path if your DMG mounts with a different name):

   ```bash
   cp /Volumes/qsv/qsv /usr/local/bin/qsv
   ```

3. **Remove the quarantine flag.** macOS blocks unsigned binaries by default. Run this to allow qsv to execute:

   ```bash
   xattr -d com.apple.quarantine /usr/local/bin/qsv
   ```

4. **Verify** the installation:

   ```bash
   qsv --version
   ```

   You should see something like `qsv 16.1.0`.

5. **Eject the DMG** in Finder (drag it to Trash or right-click > Eject).

---

## 5. Install the qsv MCP Server (MCPB Bundle)

The qsv MCP server lets Claude read, analyze, and transform your local data files (CSV, Excel, TSV, JSONL, and more) — without uploading anything.

### Download the MCPB Bundle

Go to: **<https://github.com/dathere/qsv/releases/download/16.1.0/qsv-mcp-server-16.1.0.mcpb>**

Save the `.mcpb` file to your Downloads folder.

### Install in Claude Desktop

1. Open **Claude Desktop**
2. Click your **profile icon** (bottom-left) > **Settings**
3. Go to the **Extensions** tab
4. Click **"Install from file"**
5. Select the `qsv-mcp-server-16.1.0.mcpb` file you downloaded
6. Click **Install**

### Configure

After installation, you'll be prompted to set up:

- **Working Directory** — where Claude looks for your data files by default.
  Example: `$HOME/Downloads`

- **Allowed Directories** — folders Claude is allowed to access (for security).
  Example: `$HOME/Downloads:$HOME/Documents`
  (Use `:` to separate multiple folders.)

- **qsv Binary Path** — should auto-detect as `/usr/local/bin/qsv`. If not, enter it manually.

### Restart and Verify

Close and reopen Claude Desktop. Then start a new conversation and ask:

> "List data files in my Downloads folder"

If Claude shows you a list of files, the qsv MCP server is working.

---

## 6. Install the MotherDuck MCP Server (In-Memory DuckDB)

The [MotherDuck MCP server](https://github.com/motherduckdb/mcp-server-motherduck) gives Claude the ability to run SQL queries using DuckDB. We'll set it up in **in-memory mode** — no cloud account or token needed.

### Install uv (Python Package Runner)

The MotherDuck server uses `uvx` to run without a manual install. First, install `uv`:

```bash
brew install uv
```

Verify:

```bash
uv --version
```

### Install via MCPB Bundle

Download the MCPB bundle:

**<https://github.com/motherduckdb/mcp-server-motherduck/releases/latest/download/mcp-server-motherduck.mcpb>**

Then install in Claude Desktop:

1. Open **Claude Desktop**
2. Click your **profile icon** > **Settings** > **Extensions**
3. Click **"Install from file"**
4. Select the `mcp-server-motherduck.mcpb` file
5. Click **Install**

### Alternative: Manual Configuration

If you prefer manual setup (or the MCPB doesn't work), add the server to your Claude Desktop config file. See [Section 8](#8-editing-the-claude-desktop-config-file) for how to open the config file, then add this entry inside `"mcpServers"`:

```json
"DuckDB (in-memory, r/w)": {
  "command": "uvx",
  "args": [
    "mcp-server-motherduck",
    "--db-path", ":memory:",
    "--read-write",
    "--allow-switch-databases"
  ],
  "env": {
    "HOME": "/Users/YOUR_USERNAME"
  }
}
```

Replace `YOUR_USERNAME` with your actual macOS username. (To find it, run `whoami` in Terminal.)

> **Note:** No MotherDuck token is needed for in-memory mode. DuckDB runs entirely on your Mac.

### Restart and Verify

Close and reopen Claude Desktop. Then ask:

> "Create a DuckDB table with some sample data and query it"

If Claude creates and queries a table successfully, the MotherDuck server is working.

---

## 7. Install the US Census Bureau MCP Server

The [US Census Bureau MCP server](https://github.com/dathere/us-census-bureau-data-api-mcp) gives Claude access to US Census data (population, demographics, economics, and more).

This one requires a few extra tools: **Docker**, **Node.js**, and a free **Census API key**.

### 7a. Install Docker Desktop

Docker is a tool that runs applications in isolated containers. The Census server uses it for its database.

```bash
brew install --cask docker
```

After installation:

1. Open **Docker Desktop** from your Applications folder
2. Follow the setup wizard (accept the terms, grant permissions)
3. Wait for Docker to finish starting (you'll see a green "Running" indicator)

> **First time?** Docker may ask for your password and require a restart. This is normal.

### 7b. Install Node.js

```bash
brew install node
```

Verify:

```bash
node --version
```

You should see `v20.x.x` or higher.

### 7c. Get a Census API Key

1. Go to: **<https://api.census.gov/data/key_signup.html>**
2. Fill out the form (name and email)
3. Check your email for the API key — it arrives almost instantly
4. **Save your API key** somewhere handy (you'll need it in a moment)

### 7d. Download and Set Up the Census Server

In Terminal, run these commands one at a time:

```bash
cd ~/Documents
```

```bash
git clone https://github.com/dathere/us-census-bureau-data-api-mcp.git
```

```bash
cd us-census-bureau-data-api-mcp
```

> **Don't have `git`?** You can also [download the ZIP file](https://github.com/dathere/us-census-bureau-data-api-mcp/archive/refs/heads/main.zip), unzip it, and move the folder to your Documents.

### 7e. Initialize the Database

Make sure **Docker Desktop is running**, then run:

```bash
docker compose --profile prod run --rm census-mcp-db-init sh -c "npm run migrate:up && npm run seed"
```

This downloads the required containers and sets up the Census database. It may take a few minutes the first time.

### 7f. Add to Claude Desktop Config

Open the Claude Desktop config file (see [Section 8](#8-editing-the-claude-desktop-config-file) for instructions) and add this inside `"mcpServers"`:

```json
"mcp-census-api": {
  "command": "bash",
  "args": [
    "/Users/YOUR_USERNAME/Documents/us-census-bureau-data-api-mcp/scripts/mcp-connect.sh"
  ],
  "env": {
    "CENSUS_API_KEY": "YOUR_CENSUS_API_KEY"
  }
}
```

Replace:
- `YOUR_USERNAME` with your macOS username (run `whoami` in Terminal to check)
- `YOUR_CENSUS_API_KEY` with the API key from your email

### Restart and Verify

Close and reopen Claude Desktop. Make sure **Docker Desktop is running**, then ask:

> "What is the population of California?"

If Claude returns Census data, the server is working.

---

## 8. Editing the Claude Desktop Config File

Some MCP servers need to be added manually to Claude Desktop's configuration file. Here's how to find and edit it.

### Where Is the Config File?

```
~/Library/Application Support/Claude/claude_desktop_config.json
```

### How to Open It

**Option A — Terminal (easiest):**

```bash
open -a TextEdit "$HOME/Library/Application Support/Claude/claude_desktop_config.json"
```

**Option B — Finder:**

1. Open Finder
2. Click **Go** in the menu bar, then **Go to Folder...**
3. Paste: `~/Library/Application Support/Claude/`
4. Double-click `claude_desktop_config.json` to open it in TextEdit

### What the Config Looks Like

After setting up all three servers, your config file should look something like this:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "/path/auto-configured-by-mcpb/dist/mcp-server.js"
      ],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/YOUR_USERNAME/Downloads",
        "QSV_MCP_ALLOWED_DIRS": "/Users/YOUR_USERNAME/Downloads:/Users/YOUR_USERNAME/Documents"
      }
    },
    "DuckDB (in-memory, r/w)": {
      "command": "uvx",
      "args": [
        "mcp-server-motherduck",
        "--db-path", ":memory:",
        "--read-write",
        "--allow-switch-databases"
      ],
      "env": {
        "HOME": "/Users/YOUR_USERNAME"
      }
    },
    "mcp-census-api": {
      "command": "bash",
      "args": [
        "/Users/YOUR_USERNAME/Documents/us-census-bureau-data-api-mcp/scripts/mcp-connect.sh"
      ],
      "env": {
        "CENSUS_API_KEY": "YOUR_CENSUS_API_KEY"
      }
    }
  }
}
```

> **Important:** The `qsv` entry is usually managed by the MCPB installer — you shouldn't need to edit it by hand. The MotherDuck and Census entries are added manually.

> **Tip:** After editing the config file, always close and reopen Claude Desktop for changes to take effect.

---

## 9. Claude Pro and Cowork

**Claude Cowork** lets Claude work on longer, multi-step tasks in the background — like cleaning a dataset, running multiple queries, and summarizing results.

Cowork requires at least a **Claude Pro** plan. [See plans and pricing here.](https://claude.ai/pricing)

---

## 10. Troubleshooting

### General

| Problem | Solution |
|---------|----------|
| Claude doesn't seem to use the MCP servers | Make sure you restarted Claude Desktop after setup. Check Settings > Extensions. |
| "Permission denied" when running a command | Try prefixing with `sudo` (e.g., `sudo cp ...`). On macOS, you may also need to allow the app in **System Settings > Privacy & Security**. |
| Config file won't save | Make sure Claude Desktop is closed while editing the config file. |

### qsv MCP Server

| Problem | Solution |
|---------|----------|
| "qsv binary not found" | Make sure qsv is at `/usr/local/bin/qsv`. Run `which qsv` to check. |
| "Operation not permitted" running qsv | Run `xattr -d com.apple.quarantine /usr/local/bin/qsv` to clear the quarantine flag. |
| Claude says it can't find your file | Use the full file path (e.g., `/Users/you/Downloads/data.csv`) or ask Claude to "list data files" first. |

### MotherDuck MCP Server

| Problem | Solution |
|---------|----------|
| "uvx: command not found" | Install uv: `brew install uv`. Then restart Claude Desktop. |
| Server fails to start | Check that `uv` is up to date: `uv self update`. |
| "motherduck_token" error | Make sure you're using `--db-path :memory:` (no token needed for in-memory mode). |

### US Census Bureau MCP Server

| Problem | Solution |
|---------|----------|
| "Cannot connect to Docker daemon" | Open Docker Desktop and wait for it to finish starting (green "Running" indicator). |
| Database initialization fails | Make sure Docker Desktop is running, then try the `docker compose` command again. |
| "Invalid API key" | Double-check your Census API key. You can request a new one at <https://api.census.gov/data/key_signup.html>. |
| Census server won't start | Make sure Docker Desktop is running — it's needed every time you use the Census server. |

### DuckDB CLI

| Problem | Solution |
|---------|----------|
| "duckdb: command not found" | Run `brew install duckdb` and open a new Terminal window. |

---

## 11. What's Next

Now that everything is set up, here are some things you can try:

### Example Prompts

**Using qsv (local file analysis):**
> "Show me statistics for sales.csv in my Downloads folder"

**Using DuckDB (SQL queries):**
> "Load my Downloads/products.csv into DuckDB and find the top 10 products by revenue"

**Using Census data:**
> "Compare the median household income of Texas and New York using Census data"

**Combining servers:**
> "Load the Census population data for all US states into DuckDB, then export it as a CSV file"

### Learn More

- **qsv MCP Server**: [Desktop Extension Guide](../desktop/README-MCPB.md) | [Filesystem Usage](./FILESYSTEM_USAGE.md)
- **MotherDuck MCP Server**: [GitHub Repository](https://github.com/motherduckdb/mcp-server-motherduck)
- **US Census Bureau MCP Server**: [GitHub Repository](https://github.com/dathere/us-census-bureau-data-api-mcp)
- **Claude Desktop**: [Official Documentation](https://claude.ai/docs)
- **DuckDB**: [Documentation](https://duckdb.org/docs/)

---

*Last updated: 2026-02-15 | qsv MCP Server v16.1.0*
