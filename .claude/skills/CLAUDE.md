# CLAUDE.md - Agent Skills Development Guide

## Tools & commands

- Run specific test file (after `npm test` populates `dist/`): `node --test dist/tests/<module>.test.js`
- Filter tests: `node --test --test-name-pattern="pattern" dist/tests/`
- Regenerate skill JSONs after qsv update: `cargo run --bin qsv -F qsvmcp -- --update-mcp-skills` from repo root, then `npm run build` here. The flag is gated on the `mcp` feature, which is in `qsvmcp` and (via `distrib_features`) in `all_features` — so a local `-F all_features` build works too. The **prebuilt release binaries do not** include it: `publish.yml` builds them from an explicit feature list that omits `mcp`, so a downloaded `qsv` rejects the flag as unknown.

## Workflow requirements

### Adding a new MCP tool

Common commands go in the `COMMON_COMMANDS` array in `tool-constants.ts` (re-exported via the `mcp-tools.ts` barrel) and use `handleGenericCommand` automatically. Specialized tools need a dedicated definition function, an entry in `toolDispatchMap` in `mcp-server.ts`, and a handler function.

Both types need a `COMMAND_GUIDANCE` entry with `whenToUse`, `commonPattern`, and optionally `errorPrevention`.

### Guidance emoji conventions

- 💡 `USE WHEN` — when to use this tool vs alternatives
- 📋 `COMMON PATTERN` — how this tool fits into workflows
- ⚠️ `CAUTION` — memory limits, file size constraints, feature requirements
- 🚀 `PERFORMANCE` — index acceleration tips, cache strategies
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

## Operational limits

Response/file-size and timeout limits live in `tool-constants.ts` and `executor.ts`; see `config.ts` for the full configuration system (`QSV_MCP_*` env vars).

## Project-specific context

### Parameter alias trap

`buildSkillExecParams` skips `"input"` and `"output"` keys (they alias `input_file`/`output_file` via `resolveParamAliases`). Exception: if the skill declares `--input` or `--output` as a CLI option flag, the key passes through. Only matches long-form options, not short flags (`-i`/`-o`).

### Supported binaries

Only `qsvmcp` (preferred) and `qsv` (full) are supported. `qsvlite` and `qsvdp` are not — they lack Polars and other required features. Verify with `qsvmcp --version` or `qsv --version` (look for `polars-X.Y.Z` in feature list).

### Plugin mode

The repo-root `.claude-plugin/marketplace.json` (at `qsv/.claude-plugin/marketplace.json`) declares the plugin for the marketplace and points its `source` at `./.claude/skills`. The plugin-local `.claude/skills/.claude-plugin/plugin.json` carries the plugin manifest (name, version, author, keywords) but does NOT embed an MCP-server config or set `QSV_MCP_EXPOSE_ALL_TOOLS`. The MCP server starts via the MCPB `manifest.json` (`server.mcp_config`); MCP tool names become `mcp__qsv__qsv_*`. Three agents (data-analyst, data-wrangler, policy-analyst) for clear boundaries.

### Skills auto-generation

Skill JSON files in `qsv/` are auto-generated from qsv USAGE text via `--update-mcp-skills` (needs an `mcp`-featured build — see Tools & commands above). The Rust generator (`../../src/mcp_skills_gen.rs`) parses docopt usage, extracts descriptions and performance hints from README. The `mcp-tools.ts` layer adds guidance hints on top.

### Spreadsheet format handling

qsv commands operate on CSV/TSV/SSV and Parquet — not directly on spreadsheet formats (`.xls`, `.xlsx`, `.xlsm`, `.xlsb`, `.ods`). Use the `excel` command to convert spreadsheets to CSV first before processing with other qsv commands. The MCP server auto-converts Excel/ODS files when passed as input, but skills and agents should be aware of the underlying requirement. See the `data-convert` skill and the "Convert and Export" pipeline in `csv-wrangling` for details.
