# Using QSV MCP Server with Gemini CLI

This guide explains how to configure and use the **qsv MCP Server** and its associated **Agent Skills** with the [Gemini CLI](https://github.com/google/gemini-cli) (Google's terminal-based AI agent).

## Overview

The QSV MCP Server exposes **56** tabular data-wrangling commands as tools to the Gemini CLI. To optimize performance and token usage, the server follows a **Deferred Loading** pattern:

1.  **7 Core Tools** are loaded initially (Search, Config, Working Dir, Filesystem, Pipeline).
2.  **Additional Tools** are discovered via the `qsv_search_tools` tool and added dynamically to the session.

This allows the Gemini CLI to stay focused on your specific data task without being overwhelmed by 56+ tool definitions.

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
        "QSV_MCP_PLUGIN_MODE": "true",
        "QSV_MCP_WORKING_DIR": "${PWD}",
        "QSV_MCP_ALLOWED_DIRS": "${PWD}"
      }
    }
  }
}
```

> **Note**: Replace `/absolute/path/to/qsv` with the actual absolute path to your qsv repository. The Gemini CLI does not expand `~` in the `args` or `env` values.

---

#### Option B: Project-Specific Setup

If you want to isolate a specific project or restrict access to a particular dataset:

1.  Navigate to your data directory.
2.  Create a `.gemini` folder: `mkdir .gemini`
3.  Create `.gemini/settings.json` with your project settings (overrides global).

## Environment Variables Reference

| Variable | Description | Default |
| :--- | :--- | :--- |
| `QSV_MCP_PLUGIN_MODE` | Set to `true` to enable plugin mode (relaxed directory security). **Required for Gemini CLI** since it doesn't set `CLAUDE_PLUGIN_ROOT`. | Unset (auto-detect) |
| `QSV_MCP_BIN_PATH` | Path to the `qsv` executable. | Auto-detected |
| `QSV_MCP_WORKING_DIR` | The directory where files are read from. Use `${PWD}` for auto-mapping. | `${PWD}` |
| `QSV_MCP_ALLOWED_DIRS` | A security list (colon-separated) of permitted directories. | Working Dir |
| `QSV_MCP_OPERATION_TIMEOUT_MS` | Command timeout in milliseconds. | `600000` (10m) |

## Verifying the Setup

Launch a new Gemini CLI session in any directory containing data:

```bash
gemini "What qsv core tools are available?"
```

You should see the 7 core tools: `qsv_search_tools`, `qsv_config`, `qsv_set_working_dir`, `qsv_get_working_dir`, `qsv_list_files`, `qsv_pipeline`, and `qsv_command`.

## Common Workflows

### Discovering Tools
If you need a specific command (e.g., for duplicates), ask Gemini to search for it:
```bash
gemini "What tools can help me find duplicates in a CSV?"
```
This will trigger `qsv_search_tools`, which uses **BM25 relevance ranking** to find `qsv_dedup` and other relevant tools.

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
The `qsv` binary itself features a `describegpt` command that can use Gemini's LLM for data analysis (data dictionaries, descriptions, etc.):

```bash
qsv describegpt data.csv \
  --base-url https://generativelanguage.googleapis.com/v1beta/openai \
  --api-key $GEMINI_API_KEY \
  --all
```

## Documentation

- [QSV MCP Server README](../../README-MCP.md)
- [QSV Agent Skills README](../../README.md)
- [Claude Code Integration](./CLAUDE_CODE.md)
- [Local Files Usage](./FILESYSTEM_USAGE.md)
