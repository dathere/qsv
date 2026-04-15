# Project Overview

**qsv** is a blazingly-fast command-line CSV data-wrangling toolkit written in Rust.
It's a fork of xsv with extensive additional functionality.

**Current Version**: 18.0.0 | **MSRV**: Rust 1.94 | **Edition**: 2024

## Two Sub-Projects

### 1. Main qsv CLI (Rust)
- Root directory: `/`
- 71 commands in `src/cmd/`
- Binary variants: `qsv` (full), `qsvlite`, `qsvmcp`, `qsvdp`
- Entry points: `src/main.rs` (qsv + qsvmcp), `src/mainlite.rs`, `src/maindp.rs`

### 2. MCP Agent Skills (TypeScript)
- Directory: `.claude/skills/`
- MCP server exposing 51 qsv commands to AI agents
- TypeScript with Node.js >=18, MCP SDK ^1.26.0
- Version: 18.0.5

## Key Directories
- `src/cmd/` — Each command is a separate Rust module
- `src/util.rs` — Shared utilities across commands
- `src/config.rs` — CSV reader/writer config
- `.claude/skills/src/` — TypeScript MCP server source (21 files)
- `.claude/skills/qsv/` — 51 auto-generated skill JSON files
- `tests/` — Rust test files (`tests/tests.rs` declares 71 `mod test_*;` modules, including feature-gated ones)
- `.claude/skills/tests/` — TypeScript test files
- `.claude/skills/agents/` — 3 subagents (data-analyst, data-wrangler, policy-analyst)
- `.claude/skills/skills/` — 15 domain knowledge skills
