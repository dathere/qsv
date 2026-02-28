# Project Overview

**qsv** is a blazingly-fast command-line CSV data-wrangling toolkit written in Rust.
It's a fork of xsv with extensive additional functionality.

**Current Version**: 16.1.0 | **MSRV**: Rust 1.93 | **Edition**: 2024

## Two Sub-Projects

### 1. Main qsv CLI (Rust)
- Root directory: `/`
- 68 commands in `src/cmd/`
- Binary variants: `qsv` (full), `qsvlite`, `qsvmcp`, `qsvdp`
- Entry points: `src/main.rs` (qsv + qsvmcp), `src/mainlite.rs`, `src/maindp.rs`

### 2. MCP Agent Skills (TypeScript)
- Directory: `.claude/skills/`
- MCP server exposing 51 qsv commands to AI agents
- TypeScript with Node.js >=18, MCP SDK ^1.26.0
- Version: 16.1.2 (independent of qsv binary version)

## Key Directories
- `src/cmd/` — Each command is a separate Rust module
- `src/util.rs` — Shared utilities across commands
- `src/config.rs` — CSV reader/writer config
- `.claude/skills/src/` — TypeScript MCP server source (14 files)
- `.claude/skills/qsv/` — 51 auto-generated skill JSON files
- `tests/` — Rust test files (one per command)
- `.claude/skills/tests/` — TypeScript test files
