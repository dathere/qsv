# Design Patterns and Guidelines

## Tool Registration Pattern
- **Common commands**: Add the command name to `COMMON_COMMANDS` in `src/tool-constants.ts` → automatically uses `handleGenericCommand`
- **Specialized tools**: Create definition function in `src/tool-definitions.ts`, add dispatch entry to `toolDispatchMap` in `src/mcp-server.ts`, and write handler in `src/tool-handlers.ts`
- Both kinds also need a `COMMAND_GUIDANCE` entry in `src/command-guidance.ts` with `whenToUse`, `commonPattern`, and optional `errorPrevention`

## Current COMMON_COMMANDS (in `src/tool-constants.ts`)
`select`, `moarstats`, `search`, `frequency`, `headers`, `count`, `slice`, `sniff`, `sqlp`, `joinp`, `cat`, `geocode`, `describegpt`

## Tool Exposure Modes (`QSV_MCP_EXPOSE_ALL_TOOLS`)
- `true` — expose all available qsv tools at startup
- `false` — only the tools in `CORE_TOOLS` (defined in `src/mcp-server.ts`): `qsv_search_tools`, `qsv_config`, `qsv_set_working_dir`, `qsv_get_working_dir`, `qsv_browse_directory`, `qsv_list_files`, `qsv_log`, `qsv_command`, `qsv_to_parquet`, `qsv_index`, `qsv_stats` (+1 app-only tool when Apps enabled)
- unset (default) — deferred-loading mode: `CORE_TOOLS` plus `COMMON_COMMANDS` filtered by `availableCommands` from the qsv binary; remaining tools are discoverable via `qsv_search_tools`
The MCPB extension manifest (`manifest.json`) does NOT set `QSV_MCP_EXPOSE_ALL_TOOLS`, so the default deferred-loading mode applies unless a user explicitly sets the env var.

## Guidance Emoji Conventions
- 💡 `USE WHEN` — when to use this tool vs alternatives
- 📋 `COMMON PATTERN` — how this tool fits into workflows
- ⚠️ `CAUTION` — memory limits, file size constraints
- 🚀 `PERFORMANCE` — index acceleration, cache strategies
- 📊 — stats-related guidance
- 🎲 — synthesize / randomness guidance (legend added 2026-05)

## Parameter Alias Trap
`buildSkillExecParams` skips `"input"` and `"output"` keys (they alias `input_file`/`output_file` via `resolveParamAliases`). Exception: if the skill declares `--input`/`--output` as a CLI option flag, the key passes through. Only matches long-form options, not short flags (`-i`/`-o`).

## Concurrency Model
Slot-based concurrency control in `src/concurrency.ts`. Operations acquire a slot before running qsv, release on completion. Configurable max concurrent operations and queue size via `QSV_MCP_MAX_CONCURRENT_OPERATIONS` and `QSV_MCP_CONCURRENCY_WAIT_TIMEOUT_MS`. `initiateShutdown()` + `killAllProcesses()` for graceful teardown.

## File Conversion
qsv operates on CSV/TSV/SSV/Parquet — not spreadsheet formats directly. Excel/ODS files auto-convert via the `excel` command before processing; conversion tracked by `ConvertedFileManager` (LIFO-bounded by `QSV_MCP_CONVERTED_LIFO_SIZE_GB`).

## Operational Limits (in `src/tool-constants.ts`)
- `MAX_MCP_RESPONSE_SIZE`: 850 KB (safe for Claude Desktop < 1MB)
- `LARGE_FILE_THRESHOLD_BYTES`: 10 MB (triggers large-file handling)
- `DEFAULT_MAX_OUTPUT_SIZE`: 50 MB (stdout/stderr cap per execution; overridable via `QSV_MCP_MAX_OUTPUT_SIZE`)
- `MAX_LOG_MESSAGE_LEN`: 4096 chars (longer logs truncated with `…`)
- `AUTO_INDEX_SIZE_MB`: 10 MB (auto-index threshold)
- Default timeout: 10 minutes (configurable via `QSV_MCP_OPERATION_TIMEOUT_MS`, max 30 min)

## Reproducibility / Pipeline Manifest
`PIPELINE_METADATA` and `FINAL_OUTPUT_FILE` are `Symbol.for(...)` keys attached to MCP tool results (Symbol-keyed properties are invisible to `JSON.stringify`, so they never leak into the protocol response). `pipeline-manifest.ts` collects these to produce a reproducibility manifest.

## Versioning
`src/version.ts` exports the resolved version from `package.json` and asserts parity with `manifest.json`. Both files MUST stay in sync on release.
