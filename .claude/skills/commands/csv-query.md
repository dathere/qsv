---
name: csv-query
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
  - mcp__qsv__qsv_frequency
  # Exploration
  - mcp__qsv__qsv_search
  - mcp__qsv__qsv_select
  # Transform & Query
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_command
  # Export
  - mcp__qsv__qsv_to_parquet
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file> [query]"
description: Run SQL queries against CSV/TSV/Excel files using Polars SQL engine
---

# CSV Query

Query tabular data files using SQL via the Polars-powered `sqlp` command.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Decision Tree

**Is the query simple (single column filter, basic select)?**
- Yes -> Consider `select` + `search` for simpler operations
- No -> Use `sqlp` for full SQL support

**Does the query involve joins, GROUP BY, window functions, or complex expressions?**
- Yes -> Use `sqlp` (Polars SQL engine)

**Is the CSV file very large (> 10MB)?**
- Yes -> Consider converting to Parquet with `qsv_to_parquet` for faster repeated queries. Note: `sqlp` can also query CSV files of any size directly.

## Steps

1. **Prepare the file**: Run `qsv_index` and `qsv_stats` with `cardinality: true, stats_jsonl: true` to create index and stats cache.

2. **Read the stats cache**: Read `<FILESTEM>.stats.csv` (e.g., `data.stats.csv` for `data.csv`) to understand column metadata before writing SQL. This is the most important step for writing efficient queries.

3. **Run frequency on key columns**: For columns you plan to GROUP BY, filter on, or join on, run `qsv_frequency` to see actual value distributions. This reveals the best filter values and whether a GROUP BY will produce a manageable result set.

4. **Write and run SQL**: Use `qsv_sqlp` with the SQL query informed by stats and frequency data. The table name in SQL is the filename stem (e.g., `data.csv` -> `SELECT * FROM data`). For Parquet files, use `read_parquet('data.parquet')` as the table source instead.

5. **Refine if needed**: Check results and adjust the query.

## Using Stats to Write Better SQL

After reading the `.stats.csv` cache, use these columns to inform your SQL:

| Stats Column | How to Use in SQL |
|-------------|-------------------|
| `type` | Use correct casts and comparisons — don't quote integers, use date functions for Date/DateTime columns |
| `min` / `max` | Write precise WHERE clauses using actual data range (e.g., `WHERE price BETWEEN 10.5 AND 999.99` instead of arbitrary bounds) |
| `cardinality` | Estimate GROUP BY result size — low cardinality (< 100) is fast; high cardinality (> 10K) may need LIMIT or a different approach |
| `nullcount` | Only add COALESCE or IS NOT NULL where `nullcount` > 0 — skip null handling for columns with zero nulls |
| `sort_order` | Skip ORDER BY if data is already sorted on that column (sort_order = "Ascending"/"Descending") |
| `mean` / `stddev` | Write outlier filters: `WHERE col BETWEEN mean - 3*stddev AND mean + 3*stddev` |
| `sparsity` | Columns with sparsity > 0.5 are mostly null — avoid using them as join keys or GROUP BY columns |

### Using Frequency for Filter Values

Run `qsv_frequency --select col --limit 20` before writing WHERE clauses on categorical columns:

- **Pick selective filters**: If `frequency` shows "active" has 90% of rows, filtering on `WHERE status = 'active'` is wasteful — filter on the rare values instead
- **Validate expected values**: If you plan `WHERE category IN ('A','B','C')`, check frequency first to confirm those values exist and see if you're missing any
- **Avoid GROUP BY on high-cardinality columns**: If frequency shows thousands of unique values, GROUP BY will produce a huge result — add LIMIT or aggregate differently

## SQL Syntax Guide

The `sqlp` command uses Polars SQL dialect:

```sql
-- Basic select
SELECT col1, col2 FROM data WHERE col1 > 100

-- Aggregation
SELECT category, COUNT(*) as cnt, AVG(price) as avg_price
FROM data GROUP BY category ORDER BY cnt DESC

-- Window functions
SELECT *, ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) as rank
FROM employees

-- String operations
SELECT * FROM data WHERE col1 LIKE '%pattern%'

-- Date operations
SELECT *, EXTRACT(YEAR FROM date_col) as year FROM data

-- Multiple files (join)
SELECT a.*, b.name FROM file1 a JOIN file2 b ON a.id = b.id

-- CASE expressions
SELECT *, CASE WHEN amount > 1000 THEN 'high' ELSE 'low' END as tier FROM data
```

## Table Naming Convention

- File: `sales_2024.csv` -> Table: `sales_2024`
- File: `my-data.csv` -> Table: `"my-data"` (quote if contains special chars)
- Multiple files: each file is a separate table

## Notes

- `sqlp` uses the Polars engine - some PostgreSQL-specific syntax may not be supported
- For very complex queries that fail, suggest DuckDB as an alternative
- The stats cache helps Polars choose optimal data types for columns
- Results go to stdout by default; use `--output file.csv` for large result sets
- Column names are case-sensitive in SQL queries
- Use `LIMIT` to preview large result sets before running full queries
- `sqlp` can query multiple CSV files in a single SQL statement (useful for joins)
