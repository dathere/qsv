---
name: data-convert
version: 18.0.0
license: MIT
allowed-tools:
  # Discovery
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  # Transform & Query
  - mcp__qsv__qsv_command
  # Export
  - mcp__qsv__qsv_to_parquet
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file> [format]"
description: Convert between CSV, TSV, Excel, JSONL, Parquet, and other tabular formats
---

# Data Convert

Convert tabular data files between formats.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

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
| Parquet | `qsv_to_parquet` (core tool) | `.parquet` |

## Steps

1. **Index**: Run `qsv_index` on the file for fast random access in subsequent steps.

2. **Detect source format**: Run `qsv_sniff` to identify the input format, delimiter, and encoding.

3. **Convert**: Use the appropriate command based on the target format:

   - **To CSV** (from Excel/JSONL): The MCP server handles this automatically when you pass non-CSV files to any qsv tool. Use `qsv_command` with `excel` for explicit control over sheet selection.

   - **To TSV**: Use `qsv_command` with `command: "fmt"`, `options: {"out-delimiter": "\t"}`.

   - **To JSONL**: Use `qsv_command` with `command: "tojsonl"`.

   - **To Parquet**: Use `qsv_to_parquet` (dedicated core tool).

4. **Verify output**: Run `qsv_count` on the output (if CSV-based) to confirm row count matches input.

## Notes

- Excel conversion: Use `--sheet` to specify which sheet to convert (default: first sheet)
- JSONL output respects data types from stats cache - run `stats --stats-jsonl` first for better type inference
- Parquet conversion preserves data types efficiently and produces smaller files
- For CSV -> CSV reformatting (change delimiter, quoting), use `fmt` command
- Large Excel files may take longer to convert - the MCP server handles this transparently
- When converting multiple sheets from Excel, run `excel` with `--sheet` for each sheet
