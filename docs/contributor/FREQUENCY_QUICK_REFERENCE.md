# Quick Start Guide - QSV Frequency Development

## TL;DR

The `frequency` command produces exact frequency distribution tables that:
- **Short-circuit ID columns** by reusing the stats cache to avoid O(cardinality) memory
- **Stream rows sequentially or in parallel**, depending on CSV index availability
- **Emit CSV (default) or rich JSON** with per-column stats metadata
- **Support tunable limits** (`--limit`, `--unq-limit`, `--lmt-threshold`) for controlling output size
- **Handle data normalization** with trimming, case folding, NULL handling, and whitespace visualization options

## File Location
`src/cmd/frequency.rs` (~1,024 lines)

## Key Entry Points

| Function | Purpose |
|----------|---------|
| `run(argv)` | Main entry point; parses args, wires sequential/parallel paths, dispatches CSV vs JSON output |
| `sequential_ftables()` | Builds frequency tables in a single thread when no index or jobs=1 |
| `parallel_ftables(idx)` | Uses the CSV index plus a thread pool to partition work |
| `ftables(sel, it, nchunks)` | Core loop that tallies values into `Frequencies` structs |
| `process_frequencies(...)` | Normalizes raw counts into percentages/ranks and applies limits |
| `output_json(...)` | Formats nested JSON output, enriching with stats cache metadata |
| `get_unique_headers(...)` | Reads stats cache to detect all-unique columns and stash cardinalities |

## Architecture Flow
```
Input CSV (stdin or file)
    ↓
Config::indexed()? → Some(idx) & jobs>1 → parallel_ftables()
    │                                       (ThreadPool + crossbeam channels)
    └─ None / jobs=1 → sequential_ftables()
            ↓
ftables(): iterate rows → tally values per selected column
            ↓
process_frequencies(): apply limits, rank, percentage formatting
            ↓
CSV output         JSON output (if --json)
    ↓                    ↓
Writer flush        output_json(): enrich with stats cache → pretty JSON
```

## Core Data Structures

| Struct | Role |
|--------|------|
| `Frequencies<Vec<u8>>` | Foldhash-backed frequency table per column |
| `ProcessedFrequency` | Shared result struct for CSV/JSON containing value/count/percentage/rank |
| `FrequencyField` | JSON output node combining column metadata, stats, and rows |
| `OnceLock` statics | `UNIQUE_COLUMNS_VEC`, `COL_CARDINALITY_VEC`, `FREQ_ROW_COUNT`, `STATS_RECORDS` |

## Key Rust Concepts Used

| Concept | Usage |
|---------|-------|
| `OnceLock` | Lazily cache stats-derived metadata accessible across threads |
| `threadpool::ThreadPool` | Execute indexed chunks in parallel |
| `crossbeam_channel` | Collect `Frequencies` batches from workers |
| `unsafe` access | Skip bounds checks in hot loops when selecting fields |
| `serde::Serialize` | Emit structured JSON frequency output |
| `Decimal` + rounding strategies | Format percentages deterministically |

## Performance Highlights

- **Stats cache integration**: `get_unique_headers` inspects `StatsData` to avoid building hashmaps for columns where `cardinality == rowcount`.
- **Capacity planning**: Reuses cached cardinalities to pre-size `Frequencies` and reduce reallocations.
- **Case-folding pipeline**: Pre-computes closure variants for trimming/case handling to minimize branching per cell.
- **Parallel aware**: When indexed, each worker opens its own reader handle, seeks via index, and shrinks tables post-merge.
- **Human-friendly limits**: "Other" bucket uses `HumanCount` formatting; negative limits prune low-frequency values without extra passes.

## Limits, Thresholds & Unique Handling

- `--limit N` → top-N most common (N=0 disables); negative N keeps values with count ≥ |N|.
- `--unq-limit U` → sample U values when column is all-unique and `--limit` would otherwise dump every row.
- `--lmt-threshold T` → apply limits only when a column has ≥ T unique values.
- `--all-unique-text` → customize placeholder in place of `<ALL_UNIQUE>`.
- Set `QSV_STATSCACHE_MODE=none` to bypass stats cache and force full tallies (useful for auditing sampled output).

## Output Modes

- **CSV (default)**: Columns = `field,value,count,percentage,rank`. Rank is dense by count; "Other" rows carry rank 0 unless sorted.
- **JSON**: Adds dataset metadata, column stats (type, cardinality, nullcount, sparsity, uniqueness_ratio), and optionally stats vectors unless `--no-stats`.

## Testing

```bash
# Run Rust unit tests for frequency
cargo test --test test_frequency -- --test-threads=1

# Property-based indexed/non-indexed equivalence tests
cargo test test_frequency::prop_frequency
cargo test test_frequency::prop_frequency_indexed

# Manual smoke test
./target/release/qsv frequency --limit 0 tests/data/boston311-100.csv

# JSON mode sanity check
./target/release/qsv frequency --json --limit 5 data.csv | jq .
```

## Related Resources

- Technical deep dive: `FREQUENCY_TECHNICAL_GUIDE.md`
- Implementation: `src/cmd/frequency.rs`
- Tests: `tests/test_frequency.rs`
- Stats cache producer: `src/cmd/stats.rs`
- Indexing behavior overview: `docs/PROJECT_TECHNICAL_OVERVIEW.md`

## Quick Debugging Tips

```bash
# Inspect stats cache before running frequency
ls -1 *.stats.csv*

# Force ignoring stats cache
QSV_STATSCACHE_MODE=none ./target/release/qsv frequency file.csv

# Visualize whitespace markers
./target/release/qsv frequency --vis-whitespace --limit 0 file.csv | column -t

# Single-threaded for reproducibility
./target/release/qsv frequency --jobs 1 file.csv

# Profile with perf-like sampling
samply record ./target/release/qsv frequency --limit 0 large.csv
```

---

**Pro Tip**: Read `get_unique_headers` and `counts` first—their interplay with the stats cache and limit logic explains most of the command's behavior.
