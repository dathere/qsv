# Quick Start Guide - QSV Stats Development

## TL;DR

The `stats` command in qsv is a high-performance CSV statistics engine that:
- **Infers data types** for each column (6 types: NULL, Integer, Float, String, Date, DateTime)
- **Computes statistics** from streaming (mean, sum, stddev) to non-streaming (median, quartiles, modes)
- **Processes files** either sequentially or in parallel (with index)
- **Caches results** to avoid recomputation
- **Supports 44+ output columns** with detailed statistics

## File Location
`/path/to/qsv/src/cmd/stats.rs` (~4,694 lines)

## Key Entry Points

| Function | Purpose |
|----------|---------|
| `run(argv)` | Main entry point - orchestrates entire command |
| `sequential_stats()` | Process CSV in single thread |
| `parallel_stats()` | Process CSV in multiple threads using index |
| `compute()` | Core computation loop - processes records |
| `Stats::add()` | Type inference & stat updates for single value |
| `stats_to_records()` | Convert computed stats to CSV output |

## Architecture Flow
```
Input CSV
    ↓
Check Cache (if exists and valid, skip computation)
    ↓
Has Index? → Yes → parallel_stats() (multi-threaded)
    ↓ No
sequential_stats() (single-threaded)
    ↓
compute() - Processes each row:
    For each column:
        Stats::add() - Infers type, updates statistics
    ↓
Convert Stats to CSV Records
    ↓
Cache Results
    ↓
Output to stdout/file
```

## Core Data Structures

### `Stats` Struct (664+ bytes per column)
```rust
struct Stats {
    typ: FieldType,              // Detected data type
    nullcount: u64,              // NULL value count
    sum: Option<TypedSum>,       // Numeric sum (with overflow detection)
    online: Option<OnlineStats>, // Mean/stddev (Welford's algorithm)
    modes: Option<Unsorted<Vec<u8>>>, // For mode computation
    unsorted_stats: Option<Unsorted<f64>>, // For median/quartiles
    minmax: Option<TypedMinMax>, // Min/max values
    // ... 40+ more fields for different statistics
}
```

### `FieldType` Enum - Type Inference
```rust
enum FieldType {
    TNull,      // All empty/missing (default)
    TString,    // Text (fallback)
    TFloat,     // Decimals
    TInteger,   // Whole numbers
    TDate,      // Dates only
    TDateTime,  // Dates with times
}
```

## Key Rust Concepts Used

| Concept | Used For |
|---------|----------|
| **Ownership** | CSV record data flows through pipeline |
| **Traits** | `Serialize` (output), `Commute` (merge stats) |
| **Generics** | `compute<I: Iterator>()` works with any record iterator |
| **Unsafe** | Performance: skip bounds checks in hot loops |
| **Channels** | Thread communication (results from worker threads) |
| **OnceLock** | Static initialization of type inference flags |
| **Derives** | `#[derive(Serialize)]` for output/caching |

## Performance Optimizations

1. **Unsafe Bounds Skipping** - Hot loop avoids checking array bounds
2. **Cache-Line Alignment** - `#[repr(C, align(64))]` prevents false sharing
3. **Register Allocation** - Cache flags in local variables to avoid memory access
4. **Welford's Algorithm** - O(1) memory for mean/stddev computation
5. **Parallel Processing** - Multi-threaded for large files with index
6. **Result Caching** - `.stats.csv` files avoid recomputation

## Type Inference Process

For each cell value, tries in order:
```
1. Empty? → TNull
2. Parse as i64? → TInteger
3. Parse as f64? → TFloat
4. Date inference enabled? → Try parsing dates → TDate or TDateTime
5. Default → TString
```

## Caching System

Creates three files for `input.csv`:
- `input.stats.csv` - Statistics in CSV format
- `input.stats.csv.json` - Metadata (args, timestamp, duration)
- `input.stats.csv.data.jsonl` - Statistics in JSONL format (optional)

Reuses cache if:
- ✅ Arguments are identical
- ✅ Input file hasn't changed (modification time)
- ✅ qsv version is the same

## Statistics Computed

### Streaming (constant memory):
- sum, min, max, range
- mean
- standard deviation, variance, coefficient of variation
- string length statistics
- sort order detection
- skewness, kurtosis

### Non-Streaming (requires loading all values):
- exact median (requires sorting)
- quartiles (Q1, Q2, Q3, IQR)
- percentiles
- modes and antimodes
- cardinality (unique value count)
- Median Absolute Deviation (MAD)

## Common Modifications

### Adding a New Statistic
1. Add field to `StatsData` struct
2. Add computation in `Stats::add()` 
3. Add to `Stats::to_record()` for output
4. Add header in `stats_headers()`
5. Write tests

### Changing Type Inference
Edit `Stats::add()` method logic or `BooleanPattern::matches()`

### Performance Tuning
- Profile with: `samply record ./target/release/qsv stats large.csv`
- Check cache alignment and field ordering
- Consider Welford's algorithm precision vs performance

## Testing

```bash
# Run stats tests
cargo test --test test_stats -- --test-threads=1

# Test specific feature
cargo test test_stats::integer_stats

# With logging
RUST_LOG=debug cargo test test_stats

# Manual test
echo "name,age,score
Alice,30,95.5
Bob,25,87.2" | ./target/release/qsv stats
```

## Related Resources

- Full technical guide: `STATS_TECHNICAL_GUIDE.md` (this repo)
- qsv wiki: https://github.com/dathere/qsv/wiki
- stats command docs: `/docs/PERFORMANCE.md`
- Test cases: `/tests/test_stats.rs` (~5,184 lines)

## Quick Debugging

```bash
# Enable logging
RUST_LOG=debug ./qsv stats file.csv

# Check cached stats
cat file.stats.csv.json | jq .

# View stats in JSON
cat file.stats.csv.data.jsonl | jq . | head

# Force recompute
./qsv stats --force file.csv

# Single-threaded
./qsv stats --jobs 1 file.csv
```

## Important Files

| File | Purpose |
|------|---------|
| `src/cmd/stats.rs` | Main implementation (~4,694 lines) |
| `src/config.rs` | CSV reader configuration |
| `src/select.rs` | Column selection logic |
| `src/util.rs` | Utility functions |
| `tests/test_stats.rs` | Comprehensive test suite (~5,184 lines) |
| `Cargo.toml` | Dependencies (see `stats` and `csv` crates) |

---

**Pro Tip**: Start by reading the function-level comments and the `run()` function to understand the overall flow, then dive into specific areas you want to modify.
