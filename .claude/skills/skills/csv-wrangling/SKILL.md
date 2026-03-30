---
name: csv-wrangling
description: Standard workflow order, tool selection matrix, and composition patterns for qsv CSV data wrangling
---

# CSV Wrangling with qsv

## Standard Workflow Order

Always follow this sequence when processing CSV data:

0. **Setup (Cowork)** - If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync
1. **Index** - `index` (enables fast random access for subsequent commands)
2. **Discover** - `sniff` (detect format, encoding, delimiter) -> `headers` -> `count`
3. **Profile** - `stats --cardinality --stats-jsonl` (creates cache used by smart commands)
4. **Inspect** - `slice --len 5` (preview rows), `frequency --frequency-jsonl` (value distributions with cache for reuse)
5. **Transform** - select, sort, dedup, rename, replace, search, sqlp, etc.
6. **Validate** - `validate` (against JSON Schema), `stats` (verify results)
7. **Export** - `tojsonl`, `table`, `qsv_to_parquet`
8. **Document** - `describegpt --all` (AI-generated Data Dictionary, Description & Tags)

## Tool Selection Matrix

| Task | Best Tool | Alternative | When to Use Alternative |
|------|-----------|-------------|------------------------|
| Select columns | `select` | `sqlp` | Need computed columns |
| Filter rows | `search` | `sqlp` | Complex WHERE conditions |
| Sort data | `sort` | `sqlp` | Need ORDER BY with LIMIT |
| Remove duplicates | `dedup` | `sqlp` | Need GROUP BY dedup |
| Join two files | `joinp` | `join` | `join` for memory-constrained |
| Aggregate/GROUP BY | `sqlp` | `frequency` | `frequency` for simple counts; `--frequency-jsonl` creates cache |
| Column stats | `stats` | `moarstats` | `moarstats` for extended stats |
| Find/replace | `replace` | `sqlp` | `sqlp` for conditional replace |
| Reshape wide->long | `transpose --long` | - | DuckDB UNPIVOT (external) for complex reshaping |
| Reshape long->wide | `pivotp` | `sqlp` | Complex pivots |
| Concatenate files | `cat rows` | `cat rowskey` | Different column orders |
| Sample rows | `sample` | `slice` | `slice` for positional ranges |
| Document dataset | `describegpt` | — | AI-generated Data Dictionary, Description & Tags |

## qsv Selection Syntax

Used by `select`, `search`, `sort`, `dedup`, `frequency`, and other commands:

| Syntax | Meaning | Example |
|--------|---------|---------|
| `name` | Column by name | `select "City"` |
| `1` | Column by 1-based index | `select 1` |
| `1,3,5` | Multiple columns | `select 1,3,5` |
| `1-5` | Range (inclusive) | `select 1-5` |
| `!col` | Exclude column | `select '!SSN'` |
| `!1-3` | Exclude range | `select '!1-3'` |
| `/regex/` | Match column names | `select '/^price/'` |

## Common Pipeline Patterns

### Clean and Deduplicate
```
sniff -> index -> safenames -> fixlengths -> sqlp (TRIM) -> dedup -> validate
```

### Profile and Analyze
```
sniff -> index -> stats --cardinality --stats-jsonl -> read .stats.csv -> frequency (on key columns) -> sqlp (GROUP BY queries)
```
**Before writing SQL**: read `.stats.csv` to learn column types, cardinality, nullcount, min/max, sort order. Run `frequency` on columns you'll GROUP BY or filter on. Use this to write precise WHERE clauses, correct type casts, and avoid unnecessary COALESCE.

For repeated SQL queries on large CSV (> 10MB), consider converting to Parquet: `sniff -> index -> stats -> to_parquet -> sqlp (using read_parquet())`. Note: `sqlp` can query CSV of any size directly.

### Join and Enrich
```
index (both files) -> stats (both) -> joinp -> select (keep needed columns) -> sort
```

### Profile and Document
```
sniff -> index -> stats --cardinality --stats-jsonl -> describegpt --all
```

### Convert and Export
```
excel (to CSV) -> index -> stats -> select -> tojsonl / qsv_to_parquet
```

## Delimiter Handling

- CSV (`,`): default, no flag needed
- TSV (`\t`): use `--delimiter '\t'` or file extension `.tsv`
- SSV (`;`): use `--delimiter ';'` or file extension `.ssv`
- Auto-detect: set `QSV_SNIFF_DELIMITER=1` environment variable

## Important Notes

- Column indices are **1-based**, not 0-based
- `--no-headers` flag changes behavior significantly - most commands assume headers exist
- Output goes to stdout by default; use `--output file.csv` to write to file
- Many commands auto-detect `.sz` (Snappy compressed) files transparently
- `cat rows` requires same column order; use `cat rowskey` for different schemas
- `dedup` loads all data into memory and sorts internally; use `--sorted` flag if input is already sorted to enable streaming mode with constant memory
- `sort` loads entire file into memory; for huge files use `sqlp` with ORDER BY
- For repeated SQL queries on large CSV (> 10MB), consider converting to Parquet with `qsv_to_parquet` for faster performance. Parquet works ONLY with `sqlp` and DuckDB — all other qsv commands need CSV/TSV/SSV input

## Tool Discovery

Use **`qsv_search_tools`** to discover commands beyond the initially loaded core tools. There are 51+ qsv skill-based commands covering selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, and more.

## Operational Notes

- **Timeout**: Default operation timeout is 10 minutes (configurable via `QSV_MCP_OPERATION_TIMEOUT_MS`, max 30 min). Allow operations to run to completion.
- **Memory**: `dedup`, `sort`, `reverse`, `table`, `transpose`, `pragmastat`, and `stats` (with extended stats) load entire files into memory. For files >1GB, prefer `extdedup`/`extsort` via `qsv_command`.
- **Cowork path architecture**: qsv runs on the HOST machine. File paths must be valid on the host. Always verify with `qsv_get_working_dir`.
- **Sequential operations**: Prefer sequential over parallel qsv calls to avoid queuing delays: index → stats → analysis.
- **Large files (>5GB)**: Let `qsv_frequency` run to completion. Only fall back to `qsv_sqlp` with GROUP BY if the server timeout is exceeded.
- **Context window**: Save outputs to files rather than returning to chat. Use `qsv_slice` or `qsv_sqlp` with LIMIT to inspect subsets.
