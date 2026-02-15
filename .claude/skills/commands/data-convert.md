---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_command
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file> [format]"
description: Convert between CSV, TSV, Excel, JSONL, Parquet, and other tabular formats
---

# Data Convert

Convert tabular data files between formats.

## Cowork Setup

If running in Claude Code or Cowork, first call `qsv_get_working_dir` to check qsv's current working directory. If it differs from your workspace root (the directory where relative paths should resolve), call `qsv_set_working_dir` to sync it.

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
| Parquet | `to parquet` | `.parquet` |
| Excel XLSX | `to xlsx` | `.xlsx` |
| ODS | `to ods` | `.ods` |
| PostgreSQL | `to postgres` | - |
| SQLite | `to sqlite` | `.db` |
| Data Package | `to datapackage` | `.json` |

## Steps

1. **Detect source format**: Run `qsv_sniff` to identify the input format, delimiter, and encoding.

2. **Index if CSV**: If the input is CSV/TSV, run `qsv_index` for faster processing.

3. **Convert**: Use the appropriate command based on the target format:

   - **To CSV** (from Excel/JSONL): The MCP server handles this automatically when you pass non-CSV files to any qsv tool. Use `qsv_command` with `excel` for explicit control over sheet selection.

   - **To TSV**: Use `qsv_command` with `cmd: "fmt"` and `--out-delimiter '\t'`.

   - **To JSONL**: Use `qsv_command` with `cmd: "tojsonl"`.

   - **To Parquet**: Use `qsv_command` with `cmd: "to"`, `sub_cmd: "parquet"`.

   - **To Excel XLSX**: Use `qsv_command` with `cmd: "to"`, `sub_cmd: "xlsx"`.

   - **To ODS**: Use `qsv_command` with `cmd: "to"`, `sub_cmd: "ods"`.

   - **To SQLite**: Use `qsv_command` with `cmd: "to"`, `sub_cmd: "sqlite"`.

4. **Verify output**: Run `qsv_count` on the output (if CSV-based) to confirm row count matches input.

## Notes

- Excel conversion: Use `--sheet` to specify which sheet to convert (default: first sheet)
- JSONL output respects data types from stats cache - run `stats --stats-jsonl` first for better type inference
- Parquet conversion preserves data types efficiently and produces smaller files
- For CSV -> CSV reformatting (change delimiter, quoting), use `fmt` command
- Large Excel files may take longer to convert - the MCP server handles this transparently
- When converting multiple sheets from Excel, run `excel` with `--sheet` for each sheet
