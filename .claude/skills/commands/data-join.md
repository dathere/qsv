---
name: data-join
version: 18.0.0
license: MIT
allowed-tools:
  # Discovery
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  # Analysis
  - mcp__qsv__qsv_stats
  # Exploration
  - mcp__qsv__qsv_select
  # Transform & Query
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_joinp
  - mcp__qsv__qsv_command
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file1> <file2>"
description: Join two datasets with automatic strategy selection (joinp vs join vs sqlp)
---

# Data Join

Join two tabular data files on common columns.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Strategy Selection

| Scenario | Best Tool | Why |
|----------|-----------|-----|
| Standard equi-join | `joinp` | Polars engine, fastest |
| Non-equi join (>, <, BETWEEN) | `sqlp` | SQL supports complex conditions |
| Cross join / cartesian | `sqlp` | `CROSS JOIN` syntax |
| Memory-constrained | `join` | Streaming, lower memory |
| Fuzzy/approximate match | `joinp --asof` | Nearest-match join |

## Steps

1. **Index both files**: Run `qsv_index` on both files for fast random access.

2. **Inspect both files**: Run `qsv_headers` on both files to identify column names. Determine which columns to join on.

3. **Profile join columns**: Run `qsv_stats` with `cardinality: true, stats_jsonl: true` on both files. Check the cardinality of join columns to determine optimal table order.

4. **Choose strategy**:
   - If cardinality of join column in file1 > file2, put file1 on the left
   - For `joinp`: smaller cardinality table should be on the right for best performance
   - If join condition is complex (non-equi), use `sqlp`
   - If join involves date/time matching where exact dates won't align (e.g., quarterly to monthly, event dates to nearest reporting period), use `joinp --asof`

5. **Execute join**: Use `qsv_joinp` for standard joins:
   ```
   joinp --left/--inner/--full/--cross
     left_columns: "id"
     right_columns: "id"
     input_file: "file1.csv"
     right_input: "file2.csv"
   ```

   Or use `qsv_sqlp` for complex joins:
   ```sql
   SELECT a.*, b.col1, b.col2
   FROM file1 a
   JOIN file2 b ON a.id = b.id AND a.date BETWEEN b.start_date AND b.end_date
   ```

   For ASOF (nearest-match) joins, use `qsv_joinp` with `--asof`:
   ```
   joinp --asof
     left_columns: "date"
     right_columns: "date"
     input_file: "events.csv"
     right_input: "reference.csv"
     strategy: "backward"
     allow_exact_matches: true
   ```
   - `strategy: "backward"` (default) — match to the last right row with key < left key
   - `strategy: "forward"` — match to the first right row with key > left key
   - `strategy: "nearest"` — match to the numerically closest row (supports `--tolerance`)
   - Add `--left_by`/`--right_by` to restrict matching within subgroups (e.g., per jurisdiction)
   - Add `allow_exact_matches: true` to include equal keys (<=, >=); default is strict inequality (<, >)

6. **Clean up result**: Use `qsv_select` to remove duplicate join columns or unnecessary columns from the result.

7. **Verify**: Run `qsv_count` on the result. Compare with input counts to validate join behavior:
   - Inner join: result <= min(left, right)
   - Left join: result >= left count
   - Full outer: result >= max(left, right)
   - ASOF: result = left count (every left row gets a match or null, like a left join)

## Join Types

| Type | `joinp` Flag | SQL | Behavior |
|------|-------------|-----|----------|
| Inner | (default) | `JOIN` | Only matching rows |
| Left | `--left` | `LEFT JOIN` | All left + matching right |
| Full outer | `--full` | `FULL OUTER JOIN` | All rows from both |
| Cross | `--cross` | `CROSS JOIN` | Cartesian product |
| Anti | `--anti` | `NOT IN` / `NOT EXISTS` | Left rows without match |
| Semi | `--semi` | `EXISTS` | Left rows with match (no right cols) |
| ASOF | `--asof` | _(use joinp)_ | Nearest-key match (temporal/numeric) |

## Notes

- `joinp` uses the Polars engine and is significantly faster than `join` for large files
- The stats cache helps `joinp` optimize join execution
- For joining on multiple columns, separate column names with commas: `left_columns: "col1,col2"`
- Column names must match exactly (case-sensitive)
- If join columns have different names, specify separately: `left_columns: "id"`, `right_columns: "customer_id"`
- For one-to-many joins, the result will have more rows than either input
- `joinp` handles null values in join columns (nulls don't match by default)
- ASOF joins implicitly enable `--try-parsedates` — no need to pass it explicitly
- For ASOF joins with subgroups, use `--left_by` and `--right_by` (e.g., match nearest date *per jurisdiction*)
- The `--tolerance` option limits how far the nearest match can be: use duration strings for dates (`1d`, `30d`, `365d`) or positive integers for numeric keys
- ASOF joins require sorted join columns; both datasets are auto-sorted unless `--no-sort` is set
