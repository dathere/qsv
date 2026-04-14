---
name: data-convert
description: Convert between CSV, TSV, Excel, JSONL, Parquet, and other tabular formats
user-invocable: true
argument-hint: "<file> [format]"
allowed-tools: [mcp__qsv__qsv_sniff, mcp__qsv__qsv_count, mcp__qsv__qsv_headers, mcp__qsv__qsv_index, mcp__qsv__qsv_command, mcp__qsv__qsv_to_parquet, mcp__qsv__qsv_list_files, mcp__qsv__qsv_search_tools, mcp__qsv__qsv_get_working_dir, mcp__qsv__qsv_set_working_dir]
---

# Data Convert

Convert tabular data files between formats.

> **Cowork note:** If relative paths don't resolve, call `mcp__qsv__qsv_get_working_dir` and `mcp__qsv__qsv_set_working_dir` to sync the working directory.

## Supported Conversions

### Input Formats (auto-detected)
- CSV (`.csv`), TSV (`.tsv`/`.tab`), SSV (`.ssv`)
- Excel (`.xlsx`, `.xls`, `.xlsm`, `.xlsb`)
- OpenDocument (`.ods`)
- JSONL/NDJSON (`.jsonl`, `.ndjson`)
- Snappy-compressed variants (`.csv.sz`, etc.)

### Output Formats
| Format | Command | Extension |
|--------|---------|-----------|
| CSV | `select` (identity) or `fmt` | `.csv` |
| TSV | `fmt --out-delimiter '\t'` | `.tsv` |
| JSONL | `tojsonl` | `.jsonl` |
| JSON | `slice --json` | `.json` |
| Parquet | `mcp__qsv__qsv_to_parquet` (core tool) | `.parquet` |
| XLSX | `to xlsx` (via `mcp__qsv__qsv_command`) | `.xlsx` |
| ODS | `to ods` (via `mcp__qsv__qsv_command`) | `.ods` |
| SQLite | `to sqlite` (via `mcp__qsv__qsv_command`) | `.db` |
| PostgreSQL | `to postgres` (via `mcp__qsv__qsv_command`) | N/A |
| Data Package | `to datapackage` (via `mcp__qsv__qsv_command`) | `.json` |

## Steps

1. **Index**: Run `mcp__qsv__qsv_index` on the file for fast random access in subsequent steps.

2. **Detect source format**: Run `mcp__qsv__qsv_sniff` to identify the input format, delimiter, and encoding.

3. **Convert**: Use the appropriate command based on the target format:

   - **To CSV** (from Excel/JSONL): The MCP server handles this automatically when you pass non-CSV files to any qsv tool. Use `mcp__qsv__qsv_command` with `excel` for explicit control over sheet selection.

   - **To TSV**: Use `mcp__qsv__qsv_command` with `command: "fmt"`, `options: {"out-delimiter": "\t"}`.

   - **To JSONL**: Use `mcp__qsv__qsv_command` with `command: "tojsonl"`.

   - **To Parquet (single file)**: Use `mcp__qsv__qsv_to_parquet` (core tool) — auto-generates stats cache and Polars schema for optimal type inference.

   - **To Parquet (batch)**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["parquet", "output_dir"]` for batch conversion with explicit compression control.

   - **To XLSX**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["xlsx", "output.xlsx"]`.

   - **To ODS**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["ods", "output.ods"]`.

   - **To SQLite**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["sqlite", "output.db"]`.

   - **To PostgreSQL**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["postgres", "connection_string"]`.

   - **To Data Package**: Use `mcp__qsv__qsv_command` with `command: "to"`, `args: ["datapackage", "output.json"]`.

4. **Verify output**: Run `mcp__qsv__qsv_count` on the output (if CSV-based) to confirm row count matches input.

## Notes

- Excel conversion: Use `--sheet` to specify which sheet to convert (default: first sheet)
- JSONL output respects data types from stats cache - run `stats --stats-jsonl` first for better type inference
- Parquet conversion preserves data types efficiently and produces smaller files
- For CSV -> CSV reformatting (change delimiter, quoting), use `fmt` command
- Large Excel files may take longer to convert - the MCP server handles this transparently
- When converting multiple sheets from Excel, run `excel` with `--sheet` for each sheet
