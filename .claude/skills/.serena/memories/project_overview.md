# Project Overview

**Name**: `@qsv/agent-skills` (v20.1.0)
**Repo location**: `/Users/joelnatividad/GitHub/qsv/.claude/skills/`
**Purpose**: MCP (Model Context Protocol) server and Claude Cowork plugin for the qsv tabular data-wrangling CLI toolkit. Exposes qsv commands as MCP tools to Claude Desktop, Claude Code/Cowork, and other MCP clients.

## What It Does
- Wraps qsv CLI commands as MCP tools with type-safe parameter schemas (55 skill JSONs in `qsv/`)
- Auto-generates skill JSON files from qsv's USAGE text (`qsv --update-mcp-skills` from repo root)
- Provides BM25-based tool search, slot-based concurrency, DuckDB integration, spreadsheet conversion (Excel/ODS → CSV), and working directory management
- Ships as both an MCP server (`.mcpb` bundles) and a Claude Cowork plugin (`.plugin` bundles)
- Includes 3 agent personas (data-analyst, data-wrangler, policy-analyst) and 15 skill workflows

## Tech Stack
- **Language**: TypeScript (ES2022, strict mode, ES modules)
- **Runtime**: Node.js ≥ 18
- **MCP SDK**: `@modelcontextprotocol/sdk` ^1.26.0 (+ `@modelcontextprotocol/ext-apps` ^1.2.0)
- **Search**: `wink-bm25-text-search` + `wink-nlp-utils` for tool discovery
- **Testing**: Node.js built-in test runner (`node --test`)
- **Coverage**: c8
- **Build**: `tsc` (TypeScript compiler)
- **Binary entry**: `dist/mcp-server.js` (bin name `qsv-mcp-server`)

## Version sync
- `package.json` and `manifest.json` versions are validated for parity in `src/version.ts` — keep them aligned during release prep.
