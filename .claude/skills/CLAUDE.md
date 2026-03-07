# CLAUDE.md - Agent Skills Development Guide

## Tools & commands

- Build: `npm run build`
- Test: `npm test` (builds with test config automatically)
- Run specific test file: `node --test dist/tests/<module>.test.js`
- Filter tests: `node --test --test-name-pattern="pattern" dist/tests/`
- Watch mode: `npm run test:watch`
- Coverage: `npm run test:coverage`
- Example scripts: `npm run test:examples`
- Regenerate skill JSONs after qsv update: `qsv --update-mcp-skills` from repo root, then `npm run build` here

## Workflow requirements

### Adding a new MCP tool

Common commands go in the `COMMON_COMMANDS` array in `mcp-tools.ts` and use `handleGenericCommand` automatically. Specialized tools need a dedicated definition function, a `case` in `handleToolCall()`, and a handler function.

Both types need a `COMMAND_GUIDANCE` entry with `useWhen`, `commonPattern`, and optionally `caution`.

### Guidance emoji conventions

- `USE WHEN` — when to use this tool vs alternatives
- `COMMON PATTERN` — how this tool fits into workflows
- `CAUTION` — memory limits, file size constraints, feature requirements
- `PERFORMANCE` — index acceleration tips, cache strategies
- 📊 marks stats-related guidance

Stats-aware guidance — run `qsv stats --cardinality --stats-jsonl` first, then read `.stats.csv` (not `.data.jsonl`):

| Tool | What Stats Reveals | Why It Helps |
|------|---------------------|--------------|
| `joinp` | Join column cardinality | Optimal table order (smaller cardinality on right) |
| `frequency` | High-cardinality columns | Avoid huge output from ID/timestamp columns |
| `dedup` | Uniqueness per column | Skip dedup if key column is all-unique |
| `sort` | Sort order | Skip sorting if already sorted |
| `pivotp` | Pivot column cardinality | Avoid overly wide output (>1000 columns) |

### Cache reading preference

Prefer reading `.stats.csv` and `.freq.csv` directly over their `.data.jsonl` counterparts — standard CSV is far more token-efficient. The `.data.jsonl` files exist for programmatic use by qsv's "smart" commands internally.

## Operational limits (quick reference)

> Values verified 2026-03-07. If in doubt, grep the source files below for current values.

| Constant | Value | Location |
|----------|-------|----------|
| `MAX_MCP_RESPONSE_SIZE` | 850 KB | `mcp-tools.ts` — safe for Claude Desktop (< 1MB) |
| `LARGE_FILE_THRESHOLD_BYTES` | 10 MB | `mcp-tools.ts` — triggers large-file handling |
| `MAX_LOG_MESSAGE_LEN` | 4096 chars | `mcp-tools.ts` — silently truncated beyond this |
| `MAX_OUTPUT_SIZE` | 50 MB | `executor.ts` — stdout/stderr cap per execution |
| Default timeout | 10 min | `executor.ts` — configurable via params or config |

See `config.ts` for the full configuration system (`QSV_MCP_*` env vars).

## Project-specific context

### Parameter alias trap

`buildSkillExecParams` skips `"input"` and `"output"` keys (they alias `input_file`/`output_file` via `resolveParamAliases`). Exception: if the skill declares `--input` or `--output` as a CLI option flag, the key passes through. Only matches long-form options, not short flags (`-i`/`-o`).

### Supported binaries

Only `qsvmcp` (preferred) and `qsv` (full) are supported. `qsvlite` and `qsvdp` are not — they lack Polars and other required features. Verify with `qsvmcp --version` or `qsv --version` (look for `polars-X.Y.Z` in feature list).

### Plugin mode

`.claude-plugin/plugin.json` declares the plugin, points to `.mcp.json` (server key `"qsv"`, tools become `mcp__qsv__qsv_*`). Uses `QSV_MCP_EXPOSE_ALL_TOOLS=true` since Claude Code/Cowork handle large tool lists well. Two separate agents (analyst/wrangler) for clear boundaries.

### Skills auto-generation

Skill JSON files in `qsv/` are auto-generated from qsv USAGE text via `qsv --update-mcp-skills`. The Rust generator (`../../src/mcp_skills_gen.rs`) parses docopt usage, extracts descriptions and performance hints from README. The `mcp-tools.ts` layer adds guidance hints on top.
