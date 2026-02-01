# Using QSV MCP Server with Gemini CLI

This guide explains how to configure and use the **qsv MCP Server** and its associated **Agent Skills** with the [Gemini CLI](https://github.com/google/gemini-cli) (Google's terminal-based AI agent).

## Overview

The QSV MCP Server exposes 60+ tabular data-wrangling commands as tools to the Gemini CLI. This allows you to perform complex data operations like statistics, joins, and filtering using natural language directly in your terminal.

## Prerequisites

1.  **Node.js ≥ 18.0.0**: Required to run the MCP server.
2.  **qsv binary**: Ensure `qsv` is installed and in your PATH.
    ```bash
    qsv --version
    ```
3.  **Gemini CLI**: Installed and authenticated.

## Installation

### Step 1: Build the MCP Server

Before the Gemini CLI can use the server, you must compile the TypeScript source code.

```bash
cd .claude/skills
npm install
npm run build
```

This generates the server entry point at `./dist/mcp-server.js`.

### Step 2: Configure Gemini CLI

You have two options for configuration. The **Global Setup** is the most convenient for general use, while the **Project-Specific Setup** provides maximum isolation.

#### Option A: Global "Set-and-Forget" Setup (Recommended)

By using the `${PWD}` template variable, you can configure `qsv` once in your global settings, and it will automatically adapt to whatever directory you are currently in.

1.  Open (or create) `~/.gemini/settings.json`.
2.  Add the following:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/absolute/path/to/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_WORKING_DIR": "${PWD}",
        "QSV_MCP_ALLOWED_DIRS": "${PWD}"
      }
    }
  }
}
```

> **Why this works**: The QSV MCP server expands `${PWD}` to the directory from which the Gemini CLI was launched. This makes `qsv` tools available and scoped to your current folder wherever you are.

---

#### Option B: Project-Specific Setup

If you want to isolate a specific project or restrict access to a particular dataset:

1.  Navigate to your data directory.
2.  Create a `.gemini` folder: `mkdir .gemini`
3.  Create `.gemini/settings.json` with your project settings (overrides global).

## Environment Variables Reference

| Variable | Description |
| :--- | :--- |
| `QSV_MCP_BIN_PATH` | Path to the `qsv` executable (usually `/usr/local/bin/qsv`). |
| `QSV_MCP_WORKING_DIR` | The directory where files are read from. Use `${PWD}` for auto-mapping. |
| `QSV_MCP_ALLOWED_DIRS` | A security list (colon-separated) of permitted directories. Use `${PWD}` for automatic local access. |

## Verifying the Setup

Launch a new Gemini CLI session in any directory containing data:

```bash
gemini "What qsv tools are available?"
```

## Common Workflows

### Data Discovery
```bash
gemini "What CSV files are available here?"
```

### Quick Analysis
```bash
gemini "Summarize the 'revenue' column in sales.csv"
```

### Data Pipeline
```bash
gemini "Filter customers.csv to 'active' status and join with orders.csv on 'id'"
```

## Advanced: Gemini inside qsv
The `qsv` binary itself features a `describegpt` command that can use Gemini's LLM for data analysis. You can use it as a standalone CLI tool:

```bash
qsv describegpt data.csv \
  --base-url https://generativelanguage.googleapis.com/v1beta/openai \
  --api-key $GEMINI_API_KEY \
  --all
```

## Documentation

- [QSV MCP Server README](skills/README-MCP.md)
- [QSV Agent Skills README](skills/README.md)
- [Claude Code Integration](skills/CLAUDE_CODE.md)
