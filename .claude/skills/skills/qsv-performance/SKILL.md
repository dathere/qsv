---
name: qsv-performance
description: Performance guide covering index files, stats cache, and frequency cache accelerators for qsv
---

# qsv Performance Guide

## Three Accelerators

### 1. Index Files (`.csv.idx`)
**Created by**: `qsv index`
**Used by**: count, slice, sample, split, stats, frequency, schema, and others marked with 📇

| Benefit | Without Index | With Index |
|---------|--------------|------------|
| Row count | Scan entire file | Instant (stored in index) |
| Random access | Sequential scan | O(1) lookup |
| Multithreaded | Not possible | Enabled for many commands |
| Slicing | Read from start | Jump to position |

**Rule**: Always run `index` first if you'll run 2+ commands on the same file.

**Auto-indexing**: The MCP server auto-indexes files > 10MB.

### 2. Stats Cache (`.stats.csv` + `.stats.csv.data.jsonl`)
**Created by**: `qsv stats --cardinality --stats-jsonl`
**Used by**: frequency, schema, tojsonl, sqlp, joinp, pivotp, diff, sample (smart commands)

| Smart Command | What It Uses from Cache |
|--------------|------------------------|
| `frequency` | Cardinality to skip all-unique columns |
| `schema` | Data types for JSON Schema generation |
| `sqlp` | Column types for Polars optimization |
| `joinp` | Cardinality for optimal join order |
| `pivotp` | Cardinality to estimate output width |
| `diff` | Column types for comparison |

**Rule**: Run `stats --cardinality --stats-jsonl` before using any smart command.

**Auto-caching**: The MCP server auto-adds `--stats-jsonl` to stats commands.

### 3. Polars Engine
**Commands**: sqlp, joinp, pivotp, count (with `--polars-len`), schema (with `--polars`)

| Benefit | Standard (csv crate) | Polars Engine |
|---------|---------------------|---------------|
| Processing model | Row-by-row streaming | Vectorized columnar |
| Memory | Streaming (constant) | Columnar (efficient) |
| Parallelism | Single-threaded | Multi-threaded |
| Large files | Limited by memory | Larger-than-memory |
| SQL support | N/A | Full SQL dialect |

**Rule**: Use Polars commands (sqlp, joinp, pivotp) for files > 100MB or complex queries.

### Parquet Acceleration
For repeated SQL queries on large CSV (> 10MB), consider converting to Parquet with `mcp__qsv__qsv_to_parquet`. Parquet is a columnar format that speeds up repeated SQL queries in `mcp__qsv__qsv_sqlp`. Use `read_parquet('file.parquet')` as the table source. DuckDB is the preferred engine for Parquet queries; `mcp__qsv__qsv_sqlp` with `SKIP_INPUT` as the `input_file` value also works. Note: `mcp__qsv__qsv_sqlp` can query CSV of any size directly — Parquet is an optimization for repeated queries, not a requirement. Parquet works ONLY with `mcp__qsv__qsv_sqlp` and DuckDB — all other qsv commands require CSV/TSV/SSV input.

## Memory-Aware Command Selection

### Commands That Load Entire File into Memory (🤯)
`dedup`, `reverse`, `sort`, `stats` (with extended stats), `table`, `transpose`

### Commands with Memory Proportional to Cardinality (😣)
`frequency`, `join`, `schema`, `tojsonl`

### Streaming Commands (constant memory)
Everything else - `select`, `search`, `slice`, `replace`, `count`, etc.

## Large File Decision Tree

```
File size?
├── < 10MB: Any command works fine
├── 10MB - 100MB:
│   ├── Always: index first
│   ├── Repeated SQL: consider Parquet with qsv_to_parquet
│   ├── Prefer: streaming commands
│   └── OK: memory-intensive if < available RAM
├── 100MB - 1GB:
│   ├── Always: index + stats cache first
│   ├── Repeated SQL: consider Parquet with qsv_to_parquet
│   ├── Prefer: Polars commands (sqlp, joinp, pivotp)
│   ├── Avoid: sort, reverse, table (load entire file)
│   └── Alternative: sqlp with ORDER BY LIMIT instead of sort
└── > 1GB:
    ├── Must: index + stats cache
    ├── Repeated SQL: convert to Parquet with qsv_to_parquet
    ├── Must: Polars commands only for joins/queries
    ├── Avoid: all 🤯 commands
    └── Consider: split into chunks, process, cat rows
```

## Performance Tips

| Tip | Why |
|-----|-----|
| Use `--output file.csv` | Avoids stdout buffering overhead |
| Use `count` before `stats` | Fast row count for progress bars |
| Use `select` early in pipeline | Reduce columns = faster processing |
| Use `--no-headers` only when needed | Header detection is cheap |
| Use `slice --len N` for previews | Don't read entire file to inspect |
| Prefer `joinp` over `join` | Polars engine is significantly faster |
| Use `frequency --limit N` | Don't compute all unique values |
| Use `stats --cardinality` | Enables smart optimizations downstream |

## Concurrent Operations

The MCP server limits concurrent qsv operations (default: 1). For multiple independent files, the agent can issue separate tool calls.

## Timeout Handling

- Default timeout: 10 minutes (`QSV_MCP_OPERATION_TIMEOUT_MS`)
- Long operations (sort on huge files) may timeout
- If timeout occurs: try Polars alternative or split the file
- Exit code 124 indicates timeout
