# Codebase Structure

## Root (Rust CLI)
```
src/
  main.rs          — Entry point for qsv + qsvmcp
  mainlite.rs      — Entry point for qsvlite
  maindp.rs        — Entry point for qsvdp
  cmd/             — 71 command modules (one .rs per command)
  util.rs          — Shared utility functions
  config.rs        — CSV reader/writer configuration
  select.rs        — Column selection DSL
  clitypes.rs      — Common CLI types and error handling
  index.rs         — CSV indexing for random access
  lookup.rs        — Lookup table for joins
  odhtcache.rs     — On-disk hash table caching
  mcp_skills_gen.rs — MCP skill JSON generation from USAGE text
  help_markdown_gen.rs — Help markdown generation
tests/             — test_<command>.rs files
```

## MCP Server (TypeScript)
```
.claude/skills/
  src/
    mcp-server.ts           — MCP server entry point (tools, resources, prompts)
    mcp-tools.ts            — Tool definitions + handlers (largest)
    executor.ts             — qsv process spawning (streaming)
    config.ts               — Configuration + env var loading
    mcp-filesystem.ts       — Filesystem operations via MCP
    mcp-sampling.ts         — Sampling for large files
    converted-file-manager.ts — LIFO cache for converted files
    update-checker.ts       — Version detection + skill regeneration
    duckdb.ts               — DuckDB integration for SQL queries
    browse-directory.ts     — Directory browsing
    installer.ts            — Installation utilities
    pipeline-manifest.ts    — Pipeline manifest handling
    utils.ts                — Shared utilities (getErrorMessage, isNodeError, etc.)
    loader.ts               — Dynamic skill loading + BM25 search
    types.ts                — TypeScript type definitions + SKILL_CATEGORIES
    bm25-search.ts          — BM25 search index for tool discovery
    version.ts              — Version constant
    index.ts                — Package entry point
    ui/directory-picker-html.ts — Directory picker UI
    wink-bm25-text-search.d.ts — Type definitions
    wink-nlp-utils.d.ts     — Type definitions
  tests/                    — <module>.test.ts files
  qsv/                     — 51 auto-generated skill JSON files
  scripts/                 — Build/deployment scripts
  agents/                  — 3 subagents (data-analyst, data-wrangler, policy-analyst)
  skills/                  — 15 domain knowledge skills
```

## Key Config Files
- `Cargo.toml` — Rust deps, feature flags, patched dependencies
- `.claude/skills/package.json` — TypeScript deps, scripts
- `.claude/skills/tsconfig.json` — TypeScript compiler config
- `CLAUDE.md` — Root project guidance
- `.claude/skills/CLAUDE.md` — MCP server development guide
