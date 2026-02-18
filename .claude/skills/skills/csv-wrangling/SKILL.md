# CSV Wrangling with qsv

## Standard Workflow Order

Always follow this sequence when processing CSV data:

0. **Setup (Cowork)** - `qsv_get_working_dir` (check current dir) -> `qsv_set_working_dir` (sync to workspace root if needed)
1. **Discover** - `sniff` (detect format, encoding, delimiter) -> `headers` -> `count`
2. **Index** - `index` (enables fast random access for subsequent commands)
3. **Profile** - `stats --cardinality --stats-jsonl` (creates cache used by smart commands)
4. **Inspect** - `slice --len 5` (preview rows), `frequency --frequency-jsonl` (value distributions with cache for reuse)
5. **Transform** - select, sort, dedup, apply, rename, search, etc.
6. **Validate** - `validate` (against JSON Schema), `stats` (verify results)
7. **Export** - `to` (XLSX, ODS, etc.), `tojsonl`, `table`

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
| Find/replace | `apply operations` | `sqlp` | `sqlp` for conditional replace |
| Reshape wide->long | `transpose --long` | - | DuckDB UNPIVOT (external) for complex reshaping |
| Reshape long->wide | `pivotp` | `sqlp` | Complex pivots |
| Concatenate files | `cat rows` | `cat rowskey` | Different column orders |
| Sample rows | `sample` | `slice` | `slice` for positional ranges |

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
sniff -> index -> safenames -> fixlengths -> trim (apply operations) -> dedup -> validate
```

### Profile and Analyze
```
sniff -> index -> stats --cardinality --stats-jsonl -> frequency -> sqlp (GROUP BY queries)
```
For CSV > 10MB, convert to Parquet before SQL queries: `sniff -> index -> stats -> to_parquet -> sqlp (using read_parquet())`

### Join and Enrich
```
index (both files) -> stats (both) -> joinp -> select (keep needed columns) -> sort
```

### Convert and Export
```
excel (to CSV) -> index -> stats -> select -> to ods/xlsx
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
- For CSV > 10MB needing SQL queries, convert to Parquet first with `qsv_to_parquet` for dramatically faster SQL. Parquet works ONLY with `sqlp` and DuckDB -- all other qsv commands need CSV/TSV/SSV input
