---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_select
  - mcp__qsv__qsv_search
  - mcp__qsv__qsv_frequency
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_command
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file> [query]"
description: Run SQL queries against CSV/TSV/Excel files using Polars SQL engine
---

# CSV Query

Query tabular data files using SQL via the Polars-powered `sqlp` command.

## Cowork Setup

If running in Claude Code or Cowork, first call `qsv_get_working_dir` to check qsv's directory. If it differs from your session CWD, call `qsv_set_working_dir` to sync it.

## Decision Tree

**Is the query simple (single column filter, basic select)?**
- Yes -> Consider `select` + `search` for simpler operations
- No -> Use `sqlp` for full SQL support

**Does the query involve joins, GROUP BY, window functions, or complex expressions?**
- Yes -> Use `sqlp` (Polars SQL engine)

## Steps

1. **Prepare the file**: Run `qsv_index` and `qsv_stats` with `cardinality: true, stats_jsonl: true` to create index and stats cache. This helps `sqlp` optimize query execution.

2. **Inspect schema**: Run `qsv_headers` to see column names. Use these exact names in SQL queries.

3. **Write and run SQL**: Use `qsv_sqlp` with the SQL query. The table name in SQL is the filename stem (e.g., `data.csv` -> `SELECT * FROM data`).

4. **Refine if needed**: Check results and adjust the query.

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
