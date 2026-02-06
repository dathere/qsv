---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_moarstats
  - mcp__qsv__qsv_frequency
  - mcp__qsv__qsv_slice
  - mcp__qsv__qsv_command
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file>"
description: Profile a CSV/TSV/Excel file - detect format, compute statistics, show value distributions
---

# Data Profile

Profile the given tabular data file to understand its structure, types, and distributions.

## Cowork Setup

If running in Claude Code or Cowork, first call `qsv_get_working_dir` to check qsv's directory. If it differs from your session CWD, call `qsv_set_working_dir` to sync it.

## Steps

1. **Detect format**: Run `qsv_sniff` on the file to detect delimiter, encoding, preamble, and row count estimate.

2. **Count rows**: Run `qsv_count` to get the exact row count.

3. **Get headers**: Run `qsv_headers` to list all column names and positions.

4. **Create index**: Run `qsv_index` on the file for fast access in subsequent steps.

5. **Compute statistics**: Run `qsv_stats` with `cardinality: true` and `stats_jsonl: true` to generate full column statistics and cache them. Include `--everything` for comprehensive stats (mean, median, mode, stddev, quartiles, etc.).

6. **Show distributions**: Run `qsv_frequency` with `limit: 10` to show top value distributions for each column. For high-cardinality columns (cardinality close to row count), note them as likely unique identifiers.

7. **Preview data**: Run `qsv_slice` with `len: 5` to show the first 5 rows as a sample.

## Report Format

Present a summary with:
- **File info**: format, delimiter, encoding, row count, column count
- **Column overview**: table with name, type, nulls, cardinality, min, max, mean (where applicable)
- **Key observations**: unique identifiers, high-null columns, type mismatches, notable distributions
- **Data quality flags**: any issues found (high sparsity, mixed types, ragged rows)

## Notes

- For Excel/JSONL files, the MCP server auto-converts to CSV first
- The stats cache created in step 5 accelerates subsequent commands (frequency, schema, sqlp, joinp)
- If the file has no headers, mention this and use column indices
