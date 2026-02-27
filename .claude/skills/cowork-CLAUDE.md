# qsv Data Wrangling - Workflow Guide

This CLAUDE.md was auto-deployed by the qsv plugin to provide workflow guidance.
You can edit or replace it — it will NOT be overwritten on future sessions.
To disable auto-deployment, set `QSV_NO_COWORK_SETUP=1` in your shell environment before launching Claude Code.

---

## Workflow Order

For new files:
1. **`qsv_list_files`** to discover files in the working directory
2. **`qsv_index`** for files >10MB (enables faster processing)
3. **`qsv_stats --cardinality --stats-jsonl`** to create a stats cache
4. Then run analysis/transformation commands

The stats cache accelerates: `frequency`, `schema`, `tojsonl`, `sqlp`, `joinp`, `pivotp`, `describegpt`, `moarstats`, `sample`.

`qsv_sqlp` auto-converts CSV inputs to Parquet before execution.

## File Handling

- Save outputs to files with descriptive names rather than returning large results to chat.
- Ensure output files are saved to the qsv working directory.
- **Parquet** is ONLY for `sqlp`/DuckDB; all other qsv commands require CSV/TSV/SSV input.
- The working directory is automatically synced from the MCP client's root directory when available.
- If the auto-synced directory is incorrect or no root is provided, call **`qsv_set_working_dir`** to set it manually.
- In Claude Cowork, verify the working directory matches the "Work in a folder" path by calling **`qsv_get_working_dir`**, and correct it with **`qsv_set_working_dir`** if needed.

## Tool Composition

- **`qsv_sqlp`** auto-converts CSV inputs to Parquet, then routes to DuckDB when available for better SQL compatibility and performance; falls back to Polars SQL otherwise.
- For multi-file SQL queries, convert all files to Parquet first with **`qsv_to_parquet`**, then use `read_parquet()` references in SQL.
- For custom row-level logic, use **`qsv_command`** with `command="luau"`.

## Memory Limits

Commands `dedup`, `sort`, `reverse`, `table`, `transpose`, `pragmastat`, and `stats` (with extended stats) load entire files into memory.

For files >1GB, prefer `extdedup`/`extsort` alternatives via **`qsv_command`**.

Check column cardinality with **`qsv_stats`** before running `frequency` or `pivotp` to avoid huge output.

## Tool Discovery

Use **`qsv_search_tools`** to discover commands beyond the initially loaded core tools. There are 51 qsv commands available covering selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, and more. Tool names may change across versions; check `qsv_search_tools` if any are unrecognized.

## Operation Timeout

qsv operations can take significant time on larger files. The MCP server's default operation timeout is 10 minutes (configurable via `QSV_MCP_OPERATION_TIMEOUT_MS`, max 30 minutes). Allow operations to run to completion.

## Cowork-Specific Notes

- **Path Architecture**: qsv runs on the HOST machine. File paths must be valid
  on the host. Always verify with `qsv_get_working_dir`.
- **Sequential Operations**: Prefer sequential over parallel qsv calls to avoid
  queuing delays: index → stats → analysis.
- **Large Files (>5GB)**: Let `qsv_frequency` run to completion (server timeout
  is 10 min). Only fall back to `qsv_sqlp` with GROUP BY if the server timeout
  is exceeded. Use `extsort`/`extdedup` via `qsv_command` instead of `sort`/`dedup`.
- **Context Window**: Save outputs to files rather than returning to chat.
  Use `qsv_slice` or `qsv_sqlp` with LIMIT to inspect subsets.
