# Project Overview

**qsv** is a blazingly-fast command-line CSV data-wrangling toolkit written in Rust.
It's a fork of xsv with extensive additional functionality.

**Current Version**: 19.1.0 | **MSRV**: Rust 1.95 | **Edition**: 2024

## Two Sub-Projects

### 1. Main qsv CLI (Rust)
- Root directory: `/`
- Commands in `src/cmd/`
- Binary variants and command counts:
  - `qsv` (full): 71 commands
  - `qsvmcp` (MCP server): 62 commands (includes `to`)
  - `qsvlite` (minimal): 50 commands
  - `qsvdp` (datapusher_plus): 40 commands
- Entry points: `src/main.rs` (qsv + qsvmcp), `src/mainlite.rs`, `src/maindp.rs`

### 2. MCP Agent Skills (TypeScript)
- Directory: `.claude/skills/`
- MCP server exposing qsv commands to AI agents via 55 auto-generated skill JSONs
- TypeScript with Node.js >=18, MCP SDK ^1.26.0
- Version: 19.1.1

## Key Directories
- `src/cmd/` — Each command is a separate Rust module
- `src/util.rs` — Shared utilities across commands
- `src/config.rs` — CSV reader/writer config
- `.claude/skills/src/` — TypeScript MCP server source (30 .ts files, including 2 .d.ts type declarations)
- `.claude/skills/qsv/` — 55 auto-generated skill JSON files
- `tests/` — Rust test files (`tests/tests.rs` declares feature-gated `mod test_*;` modules)
- `.claude/skills/tests/` — TypeScript test files
- `.claude/skills/agents/` — 3 subagents (data-analyst, data-wrangler, policy-analyst)
- `.claude/skills/skills/` — 15 domain knowledge skills
