# Project Overview

**Name**: `@qsv/agent-skills` (v19.0.0)
**Purpose**: MCP (Model Context Protocol) server and Claude Cowork plugin for the qsv tabular data-wrangling CLI toolkit. Exposes qsv commands as MCP tools to Claude Desktop and other MCP clients.

## What It Does
- Wraps 62+ qsv CLI commands as MCP tools with type-safe parameter schemas
- Auto-generates skill JSON files from qsv's USAGE text (`qsv --update-mcp-skills`)
- Provides BM25-based tool search, concurrency management, DuckDB integration, file conversion (Excel/ODS → CSV), and working directory management
- Ships as both an MCP server (`.mcpb` bundles) and a Claude Cowork plugin (`.plugin` bundles)
- Includes 3 agent personas (data-analyst, data-wrangler, policy-analyst) and 15 skill workflows

## Tech Stack
- **Language**: TypeScript (ES2022, strict mode, ES modules)
- **Runtime**: Node.js ≥ 18
- **MCP SDK**: `@modelcontextprotocol/sdk` ^1.26.0
- **Search**: wink-bm25-text-search for tool discovery
- **Testing**: Node.js built-in test runner (`node --test`)
- **Coverage**: c8
- **Build**: `tsc` (TypeScript compiler)
- **Binary entry**: `dist/mcp-server.js`
