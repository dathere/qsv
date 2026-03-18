# Getting Started with qsv + Claude Desktop

**qsv** is a free, open-source toolkit that gives Claude the ability to work directly with data files on your computer — CSV, Excel, JSONL, Parquet, and more. No uploads, no file size limits, no data leaving your machine.

---

## What You'll Need

- **Claude Desktop** — [download here](https://claude.ai/download)
- A **Claude Pro plan** (or higher) — required for Cowork ([see pricing](https://claude.ai/pricing))
- An **internet connection** for downloads
- About **10 minutes** of your time

---

## Two Pieces, One Toolkit

| Component | What it is | What it provides |
|-----------|-----------|-----------------|
| `.mcpb` (MCP Server) | The qsv tools Claude can call in Chat + Cowork | 68 data-wrangling commands, SQL queries, file conversion. **Auto-installs the qsv binary** — no separate download needed. |
| Cowork Plugin (via marketplace) | Workflow layer for Cowork sessions | 3 domain skills, 6 slash commands, 2 subagents for guided data workflows |

Install both for the full experience. The `.mcpb` is required; the plugin is optional but recommended for Cowork users.

---

## Choose Your Setup Guide

| Platform | Guide |
|----------|-------|
| **macOS** (Apple Silicon) | [MACOS_SETUP.md](./MACOS_SETUP.md) |
| **Windows** (10/11) | [WINDOWS_SETUP.md](./WINDOWS_SETUP.md) |

Each guide walks you through installing the MCP server and the Cowork plugin step by step. No programming experience required.

---

## Optional: Additional Data Sources

Once the core setup is complete, you can add more MCP servers to give Claude access to external data:

| Server | What it provides |
|--------|-----------------|
| **US Census Bureau** | Population, demographics, economics — via a local Docker container |
| **Wikidata** | The free, structured knowledge graph maintained by the Wikimedia Foundation |

See **[ADDITIONAL_SERVERS.md](./ADDITIONAL_SERVERS.md)** for installation instructions (both platforms covered).

---

## For Developers

| Tool | Guide |
|------|-------|
| **Claude Code** (CLI) | [CLAUDE_CODE.md](./CLAUDE_CODE.md) |
| **Gemini CLI** | [GEMINI_CLI.md](./GEMINI_CLI.md) |

---

## What Can You Do?

Once everything is set up, try these example prompts:

**Local file analysis:**
> "Show me statistics for sales.csv in my Downloads folder"

**Census data:**
> "Compare the median household income of Texas and New York using Census data"

**Wikidata:**
> "Find all Nobel Prize winners in Physics from the last 10 years using Wikidata"

**Combining servers:**
> "Look up the Wikidata entities for all US state capitals, then pull their Census population data and save the results as a CSV"

**Cowork with the plugin:**
> "/data-profile my latest CSV file, then /data-clean it based on the quality issues you find"

---

## Reference Documentation

- [Filesystem Usage Guide](./FILESYSTEM_USAGE.md) — supported formats, security, configuration details
- [Desktop Extension Reference](../desktop/README-MCPB.md) — full MCPB technical reference
- [qsv MCP Server README](../../README-MCP.md) — complete MCP server documentation
- [qsv Documentation](https://github.com/dathere/qsv#commands) — all qsv commands

---

*Last updated: 2026-03-18*
