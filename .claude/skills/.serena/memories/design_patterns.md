# Design Patterns and Guidelines

## Tool Registration Pattern
- **Common commands**: Add to `COMMON_COMMANDS` array → automatically uses `handleGenericCommand`
- **Specialized tools**: Create definition function in `tool-definitions.ts`, add dispatch entry in `toolDispatchMap` in `mcp-server.ts`, and write handler in `tool-handlers.ts`
- Both types need a `COMMAND_GUIDANCE` entry with `whenToUse`, `commonPattern`, and optional `errorPrevention`

## Guidance Emoji Conventions
- 💡 `USE WHEN` — when to use this tool vs alternatives
- 📋 `COMMON PATTERN` — how this tool fits into workflows
- ⚠️ `CAUTION` — memory limits, file size constraints
- 🚀 `PERFORMANCE` — index acceleration, cache strategies
- 📊 — stats-related guidance

## Parameter Alias Trap
`buildSkillExecParams` skips `"input"` and `"output"` keys (they alias `input_file`/`output_file` via `resolveParamAliases`). Exception: if the skill declares `--input`/`--output` as a CLI option flag.

## Concurrency Model
Slot-based concurrency control in `concurrency.ts`. Operations acquire a slot before running qsv, release on completion. Configurable max concurrent operations and queue size.

## File Conversion
qsv operates on CSV/TSV/SSV/Parquet — not spreadsheet formats directly. Excel/ODS files auto-convert via the `excel` command before processing.

## Operational Limits
- `MAX_MCP_RESPONSE_SIZE`: 850 KB (safe for Claude Desktop < 1MB)
- `LARGE_FILE_THRESHOLD_BYTES`: 10 MB (triggers large-file handling)
- `MAX_OUTPUT_SIZE`: 50 MB (stdout/stderr cap per execution)
- Default timeout: 10 minutes
