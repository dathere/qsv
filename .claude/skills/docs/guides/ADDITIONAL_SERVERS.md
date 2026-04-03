# Additional MCP Servers — Census + Wikidata + FBI Crime Data

These optional MCP servers extend Claude Desktop with access to external data sources. Install them after completing the [Getting Started guide](./START_HERE.md).

> **Note:** The Census Bureau, BLS, and FBI Crime Data servers provide **U.S.-only data** — they are useful only when analyzing U.S. jurisdictions. The Wikidata server provides worldwide structured data.

[< Back to Start Here](./START_HERE.md)

---

## Table of Contents

1. [How to Edit the Claude Desktop Config File](#1-how-to-edit-the-claude-desktop-config-file)
2. [US Census Bureau MCP Server](#2-us-census-bureau-mcp-server)
3. [Wikidata MCP Server](#3-wikidata-mcp-server)
4. [FBI Crime Data MCP Server](#4-fbi-crime-data-mcp-server)
5. [Combined Config Example](#5-combined-config-example)

---

## 1. How to Edit the Claude Desktop Config File

Both servers below require adding entries to Claude Desktop's configuration file. Here's how to find and edit it.

### macOS

**Config file location:**
```
~/Library/Application Support/Claude/claude_desktop_config.json
```

**Option A — Terminal (easiest):**

```bash
open -a TextEdit "$HOME/Library/Application Support/Claude/claude_desktop_config.json"
```

**Option B — Finder:**

1. Open Finder
2. Click **Go** in the menu bar, then **Go to Folder...**
3. Paste: `~/Library/Application Support/Claude/`
4. Double-click `claude_desktop_config.json` to open it in TextEdit

### Windows

**Config file location:**
```
%APPDATA%\Claude\claude_desktop_config.json
```

This typically resolves to:
```
C:\Users\YOUR_USERNAME\AppData\Roaming\Claude\claude_desktop_config.json
```

**Option A — PowerShell (easiest):**

```powershell
notepad "$env:APPDATA\Claude\claude_desktop_config.json"
```

**Option B — File Explorer:**

1. Press `Win + R` to open the Run dialog
2. Type `%APPDATA%\Claude` and press Enter
3. Double-click `claude_desktop_config.json` to open it in Notepad

> **Tip:** After editing the config file, always close and reopen Claude Desktop for changes to take effect.

---

## 2. US Census Bureau MCP Server

The [US Census Bureau MCP server](https://github.com/dathere/us-census-bureau-data-api-mcp) gives Claude access to US Census data (population, demographics, economics, and more).

This one requires a few extra tools: **Docker**, **Node.js**, and a free **Census API key**.

### What This Provides

- Population data by state, county, city
- Demographics (age, race, ethnicity)
- Economic data (income, employment, housing)
- American Community Survey data

### Install Prerequisites

#### macOS

> **Need Homebrew?** Docker and Node.js are installed via [Homebrew](https://brew.sh), the standard macOS package manager. If you don't have it yet, install it first:
>
> ```bash
> /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
> ```
>
> Follow the on-screen instructions, then verify with `brew --version`.

**Install Docker Desktop:**

```bash
brew install --cask docker
```

After installation:

1. Open **Docker Desktop** from your Applications folder
2. Follow the setup wizard (accept the terms, grant permissions)
3. Wait for Docker to finish starting (you'll see a green "Running" indicator)

> **First time?** Docker may ask for your password and require a restart. This is normal.

**Install Node.js:**

```bash
brew install node
```

Verify:

```bash
node --version
```

You should see `v20.x.x` or higher.

#### Windows

> **Need Scoop?** Node.js is installed via [Scoop](https://scoop.sh), a command-line installer for Windows. If you don't have it yet, install it first in PowerShell:
>
> ```powershell
> Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
> irm get.scoop.sh | iex
> ```
>
> Verify with `scoop --version`.

**Install Docker Desktop:**

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

**Install Node.js:**

```powershell
scoop install nodejs
```

Verify:

```powershell
node --version
```

You should see `v20.x.x` or higher.

### Get a Census API Key

1. Go to: **<https://api.census.gov/data/key_signup.html>**
2. Fill out the form (name and email)
3. Check your email for the API key — it arrives almost instantly
4. **Save your API key** somewhere handy (you'll need it in a moment)

### Download and Set Up

#### macOS

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

#### Windows

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

### Initialize the Database

Make sure **Docker Desktop is running**, then run:

```bash
docker compose --profile prod run --rm census-mcp-db-init sh -c "npm run migrate:up && npm run seed"
```

This downloads the required containers and sets up the Census database. It may take a few minutes the first time.

### Add to Claude Desktop Config

Open the Claude Desktop config file (see [Section 1](#1-how-to-edit-the-claude-desktop-config-file)) and add this inside `"mcpServers"`:

#### macOS

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

#### Windows

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

> **Note (Windows):** The Census server uses `bash` via WSL. The `"command": "bash"` above relies on WSL's `bash.exe` being on your Windows PATH (it is by default after WSL installation). To verify, run `where bash` in PowerShell — it should show `C:\Windows\System32\bash.exe`. If not found, make sure WSL is installed. If you have Git Bash installed and it resolves first, use `"command": "wsl"` with `"args": ["bash", "path/to/mcp-connect.sh"]` instead.

### Verify Census Server

Close and reopen Claude Desktop. Make sure **Docker Desktop is running**, then ask:

> "What is the population of California?"

If Claude returns Census data, the server is working.

### Census Troubleshooting

| Problem | Solution |
| ------- | -------- |
| "Cannot connect to Docker daemon" | Open Docker Desktop and wait for it to finish starting (green "Running" indicator). |
| Database initialization fails | Make sure Docker Desktop is running, then try the `docker compose` command again. |
| "Invalid API key" | Double-check your Census API key. You can request a new one at <https://api.census.gov/data/key_signup.html>. |
| Census server won't start | Make sure Docker Desktop is running — it's needed every time you use the Census server. |
| WSL 2 not installed (Windows) | Run `wsl --install` in PowerShell (as Administrator) and restart your computer. Docker Desktop requires WSL 2. |

---

## 3. Wikidata MCP Server

The [Wikidata MCP Server](https://github.com/philippesaade-wmde/WikidataMCP) gives Claude access to [Wikidata](https://www.wikidata.org/) — the free, structured knowledge graph maintained by the Wikimedia Foundation. It provides tools for searching entities, running SPARQL queries, reading entity claims, and more.

There are two ways to set it up. **Method A is recommended** — it uses a hosted server so you don't need to install anything.

### What This Provides

- Search Wikidata entities by name or description
- Run SPARQL queries against the Wikidata knowledge graph
- Read entity claims and properties
- Look up relationships between entities

### Method A: Remote Hosted Server (Recommended)

Wikimedia Deutschland hosts a public endpoint for the Wikidata MCP Server. No local installation is needed — just add the config to Claude Desktop.

Open the Claude Desktop config file (see [Section 1](#1-how-to-edit-the-claude-desktop-config-file)) and add this inside `"mcpServers"`:

```json
"Wikidata MCP": {
  "url": "https://wd-mcp.wmcloud.org/mcp/"
}
```

That's it! Skip to [Verify Wikidata Server](#verify-wikidata-server) below.

### Method B: Local Installation (Alternative)

If you prefer to run the server locally (e.g., for development or offline use), follow these steps.

#### macOS

**Install uv** (a fast Python package manager) — requires [Homebrew](https://brew.sh):

```bash
brew install uv
```

**Clone the repository:**

```bash
cd ~/Documents
```

```bash
git clone https://github.com/philippesaade-wmde/WikidataMCP.git
```

```bash
cd WikidataMCP
```

**Install dependencies:**

```bash
uv sync
```

**Add to Claude Desktop config:**

Open the Claude Desktop config file (see [Section 1](#1-how-to-edit-the-claude-desktop-config-file)) and add this inside `"mcpServers"`:

```json
"Wikidata MCP": {
  "command": "uv",
  "args": ["run", "fastmcp", "run", "./main.py"],
  "cwd": "/Users/YOUR_USERNAME/Documents/WikidataMCP"
}
```

Replace `YOUR_USERNAME` with your macOS username (run `whoami` in Terminal to check).

#### Windows

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

Open the Claude Desktop config file (see [Section 1](#1-how-to-edit-the-claude-desktop-config-file)) and add this inside `"mcpServers"`:

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

### Wikidata Troubleshooting

| Problem | Solution |
| ------- | -------- |
| Wikidata server not responding | Check your internet connection. The hosted server at `wd-mcp.wmcloud.org` requires internet access. |
| Timeout on SPARQL queries | Complex queries may take time. Try simpler queries first, or break them into smaller parts. |
| Local server won't start (macOS) | Make sure `uv` is installed (`brew install uv`) and you ran `uv sync` in the WikidataMCP directory. |
| Local server won't start (Windows) | Make sure `uv` is installed (`scoop install uv`) and you ran `uv sync` in the WikidataMCP directory. |

---

## 4. FBI Crime Data MCP Server

The [FBI Crime Data MCP Server](https://github.com/dathere/fbi-crime-data-mcp) gives Claude access to the FBI's [Crime Data Explorer](https://cde.ucr.cjis.gov/) API — query crime statistics, arrest data, hate crimes, NIBRS incidents, law enforcement employment, and more.

This one is lightweight — no Docker or database required. You just need **Python/uv** and a free **API key**.

### What This Provides

- Crime statistics (violent crime, property crime, homicide, etc.) — rates, actuals, clearances
- NIBRS incident-based data across 70+ offense types
- Arrest data with demographic breakdowns
- National crime trends and percent changes
- Hate crime incidents by bias motivation
- Expanded homicide and property crime details
- Law enforcement employment, use of force, and officer safety data
- Agency lookup by state, ORI code, or judicial district

### Install Prerequisites

The FBI Crime Data MCP server runs via `uvx` (part of `uv`), so you just need `uv` installed.

#### macOS

> **Need Homebrew?** See the [Census section above](#install-prerequisites) for Homebrew installation instructions.

```bash
brew install uv
```

#### Windows

> **Need Scoop?** See the [Census section above](#install-prerequisites-1) for Scoop installation instructions.

```powershell
scoop install uv
```

### Get an API Key

1. Go to: **<https://api.data.gov/signup/>**
2. Fill out the form (name and email)
3. Check your email for the API key — it arrives almost instantly
4. **Save your API key** somewhere handy (you'll need it in a moment)

> **Note:** Without an API key, the server falls back to `DEMO_KEY` which is limited to 30 requests per hour. With a registered key, you get 1,000 requests per hour.

### Add to Claude Desktop Config

Open the Claude Desktop config file (see [Section 1](#1-how-to-edit-the-claude-desktop-config-file)) and add this inside `"mcpServers"`:

```json
"fbi-crime-data": {
  "command": "uvx",
  "args": ["fbi-crime-data-mcp"],
  "env": {
    "FBI_API_KEY": "YOUR_FBI_API_KEY"
  }
}
```

Replace `YOUR_FBI_API_KEY` with the API key from your email.

> **Note:** This works on both macOS and Windows — `uvx` handles downloading and running the server automatically. No cloning or local setup needed.

### Verify FBI Crime Data Server

Close and reopen Claude Desktop. Then start a new conversation and ask:

> "What are the national crime trends for 2023?"

If Claude returns FBI crime statistics, the server is working.

### FBI Crime Data Troubleshooting

| Problem | Solution |
| ------- | -------- |
| "uvx: command not found" | Make sure `uv` is installed (`brew install uv` on macOS, `scoop install uv` on Windows). |
| "API rate limit exceeded" | Register for a free API key at <https://api.data.gov/signup/> and set `FBI_API_KEY` in the config. Without a key, the limit is 30 requests/hour. |
| Server starts but returns no data | Some data endpoints have limited date ranges. Try a recent year (2022 or 2023). Use `get_reference_data` to check available data. |
| Invalid API key error | Double-check your API key. You can request a new one at <https://api.data.gov/signup/>. |

---

## 5. Combined Config Example

After setting up all servers, your config file should look something like this:

### macOS

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": [
        "/path/auto-configured-by-mcpb/dist/mcp-server.js"
      ],
      "env": {
        "QSV_MCP_WORKING_DIR": "/Users/YOUR_USERNAME/Downloads",
        "QSV_MCP_ALLOWED_DIRS": "/Users/YOUR_USERNAME/Downloads:/Users/YOUR_USERNAME/Documents"
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
    },
    "Wikidata MCP": {
      "url": "https://wd-mcp.wmcloud.org/mcp/"
    },
    "fbi-crime-data": {
      "command": "uvx",
      "args": ["fbi-crime-data-mcp"],
      "env": {
        "FBI_API_KEY": "YOUR_FBI_API_KEY"
      }
    }
  }
}
```

### Windows

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
    },
    "fbi-crime-data": {
      "command": "uvx",
      "args": ["fbi-crime-data-mcp"],
      "env": {
        "FBI_API_KEY": "YOUR_FBI_API_KEY"
      }
    }
  }
}
```

> **Important:** The `qsv` entry is managed by the MCPB installer — you shouldn't need to edit it by hand. The qsv binary path is auto-detected. The Census, Wikidata, and FBI Crime Data entries are added manually.

---

*Last updated: 2026-04-03*
