---
name: data-clean
description: Clean a CSV/TSV/Excel file - fix headers, trim whitespace, remove duplicates, validate
user-invocable: true
argument-hint: "<file>"
allowed-tools:
  # Discovery
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  # Analysis
  - mcp__qsv__qsv_stats
  # Transform & Query
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_command
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
---

# Data Clean

Clean the given tabular data file by fixing common data quality issues.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Steps

1. **Index**: Run `qsv_index` on the file for fast random access in subsequent steps.

2. **Assess current state**: Run `qsv_sniff` and `qsv_count` to understand the file format and size.

3. **Profile for cleaning decisions**: Run `qsv_stats` with `cardinality: true, stats_jsonl: true`. Read `.stats.csv` to decide which cleaning steps are needed:

   | Stats Column | What It Reveals | Cleaning Action |
   |-------------|-----------------|-----------------|
   | `nullcount`, `sparsity` | Missing values per column | If sparsity > 0.5, decide: impute, drop column, or flag |
   | `cardinality` vs row count | Duplicate rows exist if any key column has cardinality < row count | Run `dedup` |
   | `min_length`, `max_length` | String length variation | Large gap suggests ragged data or embedded whitespace |
   | `sort_order` | Whether data is pre-sorted | Use `dedup --sorted` for streaming mode if sorted |
   | `mode`, `mode_count` | Dominant values | If mode_count > 80% of rows, investigate data entry defaults |
   | `type` | Inferred types | String columns that should be numeric indicate format issues |

4. **Check headers**: Run `qsv_headers` to inspect column names. If names contain spaces, special characters, or are duplicated, plan to use `safenames`.

5. **Build cleaning steps**: Apply these operations in order (skip any that aren't needed based on assessment):

   a. **`safenames`** - Normalize column names to safe, ASCII-only identifiers (removes spaces, special chars, ensures uniqueness)

   b. **`fixlengths`** - Ensure all rows have the same number of fields (pads short rows, truncates long rows)

   c. **`sqlp`** - Remove leading/trailing whitespace from columns using `TRIM()`. Example: `SELECT TRIM(col1) AS col1, TRIM(col2) AS col2 FROM _t_1`.

   d. **`dedup`** - Remove exact duplicate rows. Loads all data into memory and sorts internally. Use `--sorted` if input is already sorted to enable streaming mode with constant memory.

   e. **`validate`** - If a JSON Schema is available, validate against it and report violations.

6. **Verify results**: Run `qsv_count` on the output to confirm row count. Run `qsv_stats` with `cardinality: true` to verify improvements.

7. **Report changes**: Summarize what was cleaned:
   - Headers renamed (before -> after)
   - Rows with wrong field count (fixed by fixlengths)
   - Duplicate rows removed
   - Whitespace trimmed

## Cleaning Steps

Call each tool sequentially, passing the output of one step as input to the next:

1. `qsv_command` with `command: "safenames"`, `input_file: "<file>"`, `output_file: "step1.csv"`
2. `qsv_command` with `command: "fixlengths"`, `input_file: "step1.csv"`, `output_file: "step2.csv"`
3. `qsv_sqlp` with `input_file: "step2.csv"`, `sql: "SELECT TRIM(col1) AS col1, TRIM(col2) AS col2, ... FROM _t_1"`, `output_file: "step3.csv"` (list all columns with TRIM)
4. `qsv_command` with `command: "dedup"`, `input_file: "step3.csv"`, `output_file: "<output>"`

## Notes

- Always preserve the original file - write output to a new file
- For large files (> 100MB), `dedup` loads entire file into memory to sort and deduplicate; consider using `sqlp` with `SELECT DISTINCT` instead
- `safenames` uses `--mode conditional` by default (only renames if needed)
- If the user specifies particular columns to clean, use column selection syntax instead of cleaning all columns
- `dedup` loads all data into memory and sorts internally; if input is already sorted, use `--sorted` for streaming mode
- Use `qsv_search_tools` to find additional cleaning tools if needed (e.g., `replace` for regex substitution)
