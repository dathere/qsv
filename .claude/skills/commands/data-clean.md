---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_command
  - mcp__qsv__qsv_pipeline
  - mcp__qsv__qsv_search_tools
argument-hint: "<file>"
description: Clean a CSV/TSV/Excel file - fix headers, trim whitespace, remove duplicates, validate
---

# Data Clean

Clean the given tabular data file by fixing common data quality issues.

## Steps

1. **Assess current state**: Run `qsv_sniff` and `qsv_count` to understand the file format and size.

2. **Check headers**: Run `qsv_headers` to inspect column names. If names contain spaces, special characters, or are duplicated, plan to use `safenames`.

3. **Build cleaning pipeline**: Construct a `qsv_pipeline` with these steps (skip any that aren't needed based on assessment):

   a. **`safenames`** - Normalize column names to safe, ASCII-only identifiers (removes spaces, special chars, ensures uniqueness)

   b. **`fixlengths`** - Ensure all rows have the same number of fields (pads short rows, truncates long rows)

   c. **`apply operations trim`** - Remove leading/trailing whitespace from all columns. Use selection syntax to target specific columns or all columns.

   d. **`dedup`** - Remove exact duplicate rows. Note: requires sorted input or use `--sorted` if already sorted.

   e. **`validate`** - If a JSON Schema is available, validate against it and report violations.

4. **Verify results**: Run `qsv_count` on the output to confirm row count. Run `qsv_stats` with `cardinality: true` to verify improvements.

5. **Report changes**: Summarize what was cleaned:
   - Headers renamed (before -> after)
   - Rows with wrong field count (fixed by fixlengths)
   - Duplicate rows removed
   - Whitespace trimmed

## Pipeline Template

```json
{
  "steps": [
    { "tool": "qsv_command", "args": { "cmd": "safenames", "input_file": "<file>", "output": "step1.csv" } },
    { "tool": "qsv_command", "args": { "cmd": "fixlengths", "input_file": "step1.csv", "output": "step2.csv" } },
    { "tool": "qsv_command", "args": { "cmd": "apply", "sub_cmd": "operations", "operations": "trim", "input_file": "step2.csv", "output": "step3.csv" } },
    { "tool": "qsv_command", "args": { "cmd": "sort", "input_file": "step3.csv", "output": "step4.csv" } },
    { "tool": "qsv_command", "args": { "cmd": "dedup", "input_file": "step4.csv", "output": "<output>" } }
  ]
}
```

## Notes

- Always preserve the original file - write output to a new file
- For large files (> 100MB), `sort` and `dedup` load entire file into memory; consider using `sqlp` with `SELECT DISTINCT` instead
- `safenames` uses `--mode conditional` by default (only renames if needed)
- If the user specifies particular columns to clean, use column selection syntax instead of cleaning all columns
- `dedup` requires sorted input; the pipeline sorts before deduplication
- Use `qsv_search_tools` to find additional cleaning tools if needed (e.g., `replace` for regex substitution)
