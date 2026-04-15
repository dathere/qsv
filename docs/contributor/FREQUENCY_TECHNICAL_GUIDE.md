# QSV Frequency Command: Comprehensive Technical Guide

## Table of Contents
1. [Introduction & Purpose](#introduction--purpose)
2. [Command Responsibilities](#command-responsibilities)
3. [Architecture Overview](#architecture-overview)
4. [Stats Cache Integration](#stats-cache-integration)
5. [Processing Pipeline](#processing-pipeline)
6. [Limit & Ranking Logic](#limit--ranking-logic)
7. [Data Structures](#data-structures)
8. [Output Modes](#output-modes)
9. [Parallel Execution](#parallel-execution)
10. [Performance Considerations](#performance-considerations)
11. [Extending the Command](#extending-the-command)
12. [Testing & Debugging](#testing--debugging)

---

## Introduction & Purpose

`qsv frequency` generates exact frequency distribution tables for selected CSV columns. It is designed to scale from small datasets to larger-than-memory workloads by combining streaming tallying with metadata derived from the stats cache. The command supports both CSV and JSON outputs, graceful handling of all-unique ID columns, and tunable output trimming.

### Why it matters
- **Exploratory analysis**: Surface categorical distributions quickly.
- **Data quality checks**: Spot skewed columns, NULL prevalence, or unexpected case variants.
- **Downstream automation**: JSON mode feeds pipelines that need column metadata, cardinalities, or stats summaries without re-running `stats`.

---

## Command Responsibilities

- **Row streaming**: Walk through CSV rows exactly once, tallying occurrences per column.
- **Unique detection**: Consult the stats cache to short-circuit ID columns into a single `<ALL_UNIQUE>` row.
- **Limit controls**: Apply combinations of `--limit`, `--unq-limit`, and `--lmt-threshold` to bound output size.
- **Data normalization**: Support trimming, case folding, NULL handling, and whitespace visualization toggles.
- **Parallel execution**: When the CSV is indexed and multi-threading is enabled, partition work across a thread pool.
- **JSON enrichment**: Optionally emit a nested JSON payload including column stats (type, nullcount, sparsity, uniqueness ratio).

---

## Architecture Overview

```
run(argv)
    ↓
Parse Args → derive Config (delimiter, headers, selection)
    ↓
stdin + --json ? → stream stdin to tempfile so stats cache can read it
    ↓
Config::indexed()? + jobs>1 → parallel_ftables(idx)
    │                          (thread pool, crossbeam channel)
    └── else → sequential_ftables()
                ↓
sel_headers(): determine selected columns + unique column vector
    ↓
ftables_unweighted() / ftables_weighted_internal(): iterate rows → add values into `Frequencies<Vec<u8>>`
    ↓
process_frequencies(): apply limits, format percentages, assign ranks
    ↓
CSV mode → write rows via `csv::Writer`
JSON mode → output_json(): enrich with stats metadata and pretty print
```

Key modules involved:
- `src/cmd/frequency.rs` (implementation)
- `src/config.rs` (`Config`, `indexed()` logic)
- `src/util.rs` (`get_args`, `get_stats_records`, whitespace visualization, mem checks)
- `stats` crate (`Frequencies`, `merge_all`)

---

## Stats Cache Integration

The stats cache produced by `qsv stats --stats-jsonl` is central to frequency's efficiency.

### Workflow
1. `get_unique_headers` calls `get_stats_records` with `StatsMode::Frequency` to retrieve:
   - `csv_fields`: ordered list of column names.
   - `csv_stats`: per-column `StatsData` (cardinality, nullcount, type, etc.).
   - `dataset_stats`: global metrics (rowcount).
2. Using this info, it populates several `OnceLock` caches:
   - `UNIQUE_COLUMNS_VEC`: indices where `cardinality == rowcount`.
   - `COL_CARDINALITY_VEC`: `(column_name, cardinality)` pairs.
   - `FREQ_ROW_COUNT`: dataset-wide row count fallback.
   - `STATS_RECORDS`: map column → `StatsData` (only when JSON mode is active).
3. During tallying:
   - Columns marked as all-unique are skipped entirely in the hot loop to avoid allocating large hashmaps. Later, `process_frequencies` inserts a synthetic `<ALL_UNIQUE>` row with 100% share.
   - Non-unique columns pre-size their `Frequencies` hashmaps using cached cardinality to minimize reallocation.

### Failure modes
- If the stats cache is missing or mismatched (`csv_fields` empty, counts differ), the command falls back to standard tallying by returning an empty unique vector.
- Setting `QSV_STATSCACHE_MODE=none` disables cache usage, forcing full frequency compilation even for ID columns (useful for audit sampling when combined with `--unq-limit`).

---

## Processing Pipeline

### 1. Argument Parsing
`util::get_args(USAGE, argv)` populates `Args`, mapping directly from CLI flags. Derived helpers include:
- `Args::rconfig()` → builds `Config` with delimiter, headers, selection.
- Auto tempfile creation for stdin + JSON (so the stats cache can run against a seekable file).

### 2. Memory Guard
If reading from a file, `util::mem_file_check` optionally verifies available memory when `--memcheck` is enabled.

### 3. Header Selection
`sel_headers` retrieves CSV headers (or positional names for `--no-headers`), applies column selection, and initializes the unique columns vector via the stats cache.

### 4. Frequency Table Construction
`ftables_unweighted()` and `ftables_weighted_internal()` are the core loops:
- Pre-computes a bool vector `all_unique_flag_vec` aligned with selected columns.
- Chooses a field processing closure up front based on `--ignore-case` and `--no-trim` to avoid repeated branching.
- Iterates rows via a `csv::ByteRecord` buffer, using `unsafe` indexing to skip bounds checks.
- Adds normalized values into per-column `Frequencies<Vec<u8>>` unless the column is all-unique.
- Inserts `(NULL)` sentinel when `--no-nulls` is not set and the field is empty.
- Shrinks tables post-loop when running in parallel to drop excess capacity.

### 5. Post-processing (`process_frequencies`)
- Rehydrates `<ALL_UNIQUE>` rows for short-circuited columns.
- Otherwise, calls `counts(ftab)` to compute sorted frequencies, percentages, optional "Other" buckets, and ranks.
- Formats percentages using `rust_decimal` with configurable decimal places or adaptive precision for negative `--pct-dec-places`.

### 6. Output Phase
- **CSV**: Streams rows via a `csv::Writer`, optionally visualizing whitespace markers.
- **JSON**: Builds a `FrequencyOutput` struct, injects column metadata (type, cardinality, nullcount, sparsity, uniqueness ratio, stats array), and serializes with `simd_json::to_string_pretty`. Empty `stats` arrays are pruned afterwards with a regex substitution.

---

## Limit & Ranking Logic

The `counts` function encapsulates the primary business rules:

1. **Ordering**
   - Default: descending by count with stable tie handling.
   - `--asc`: ascending order with ranks reversed (least frequent gets rank 1).
   - `--other-sorted`: includes the "Other" bucket in sort order instead of pinning it to the end.

2. **Ties & Ranks**
   - Values with equal count share the same rank (`current_rank`).
   - Next rank jumps by the size of the tie group, aligning with dense ranking semantics.
   - "Other" rows receive rank 0 to indicate they summarize omitted values.

3. **Limit Mechanics**
   - `--limit > 0`: truncate to N entries after sorting. If column is all-unique and `--unq-limit != --limit`, the unique limit takes precedence.
   - `--limit == 0`: no limit at all.
   - `--limit < 0`: filter out values whose count is below `abs(limit)`.
   - `--unq-limit`: when `all_unique` and limit is positive, sample the first U values instead of dumping all.
   - `--lmt-threshold`: only apply limit/unique-limit logic when unique cardinality ≥ threshold.

4. **Other Bucket**
   - Computes `other_count = total_count - sum(counts_kept)`. If positive and `--other-text != <NONE>`, appends `"Other (<HumanCount>)"` with the remaining rows' aggregate percentage.
   - `--other-text` accepts arbitrary strings, enabling localization (see tests using "其他" or "Ibang halaga").

5. **Whitespace & Null Handling**
   - By default, leading/trailing ASCII whitespace trimmed; `--no-trim` disables.
   - `--vis-whitespace` converts non-printable characters into markers (leverages `util::visualize_whitespace`).
   - Empty strings become `(NULL)` unless `--no-nulls` drops them entirely.

---

## Data Structures

```rust
// Cached stats metadata for JSON mode
static STATS_RECORDS: OnceLock<HashMap<String, StatsData>> = OnceLock::new();

// Tracks column indices whose cardinality == rowcount
static UNIQUE_COLUMNS_VEC: OnceLock<Vec<usize>> = OnceLock::new();

// Cached `(column_name, cardinality)` pairs used to pre-size hashmaps
static COL_CARDINALITY_VEC: OnceLock<Vec<(String, u64)>> = OnceLock::new();

// Shared rowcount fallback
static FREQ_ROW_COUNT: OnceLock<u64> = OnceLock::new();

#[derive(Clone)]
struct ProcessedFrequency {
    value: Vec<u8>,
    count: u64,
    percentage: f64,
    formatted_percentage: String,
    rank: u32,
}

#[derive(Serialize)]
struct FrequencyField {
    field: String,
    r#type: String,
    cardinality: u64,
    nullcount: u64,
    sparsity: f64,
    uniqueness_ratio: f64,
    stats: Vec<FieldStats>,
    frequencies: Vec<FrequencyEntry>,
}
```

Notes:
- `Frequencies<Vec<u8>>` originates from the `stats` crate and internally uses `foldhash` for performant hash maps.
- `FieldStats` stores stats cache values as `serde_json::Value` to allow numeric or string representations.
- `trim_bs_whitespace` is an `unsafe` inline helper that scans byte slices without extra allocations.

---

## Output Modes

### CSV (default)
- Always writes header row `field,value,count,percentage,rank`.
- Column name becomes `1`, `2`, ... when `--no-headers` is passed.
- Percentages formatted using `Decimal::round_dp_with_strategy`; negative `--pct-dec-places` triggers adaptive scale (up to abs value).
- Visual whitespace toggled via `--vis-whitespace`.

### JSON (`--json`)
- Root object (`FrequencyOutput`) includes:
  - `input`: resolved path or `stdin` sentinel.
  - `description`: reconstructed CLI arguments.
  - `rowcount` & `fieldcount`.
  - `fields`: array of `FrequencyField` objects.
- Each field contains:
  - `cardinality`: either reused rowcount (all unique) or `ftab.len()`.
  - `nullcount` / `sparsity` / `uniqueness_ratio`: derived from stats cache.
  - `stats`: optional vector unless `--no-stats` is set. Numeric values converted to `serde_json::Number` when possible.
  - `frequencies`: list mirroring CSV rows.
- Finishes by post-processing the pretty JSON string to remove empty `stats` blocks for cleaner output.

---

## Parallel Execution

When `Config::indexed()?` returns `Some(idx)` and `util::njobs(flag_jobs) > 1`, `parallel_ftables` kicks in:

1. Determine `chunk_size` and number of chunks from index count and desired jobs.
2. Spawn a `ThreadPool` with `njobs` workers.
3. For each chunk:
   - Clone `Args` and selection.
   - Seek the index to chunk start, iterate `chunk_size` records via `idx.byte_records().take(chunk_size)`.
   - Build partial frequency tables and send them over a bounded `crossbeam_channel`.
4. After joining, merge partial `Frequencies` using `stats::merge_all` which delegates to `Frequencies::merge` under the hood.

### Index prerequisites
- The same indexing logic described in the stats guide applies: qsv auto-creates or reuses `.idx` files based on file size, `QSV_AUTOINDEX_SIZE`, and stats-specific negative cache thresholds.
- `--jobs 1` or reading from stdin forces sequential mode even if an index exists.

### Thread safety
- Each worker gets its own `Config` and reader handle; `OnceLock` caches are read-only after initialization.
- `Frequencies` implements `Send`/`Sync`, allowing safe cross-thread merging.

---

## Performance Considerations

- **Avoiding full ID tallies**: The stats cache short-circuit is the biggest win. Without it, an ID column with millions of unique values would allocate a hashmap entry per row.
- **Closure hoisting**: `process_field` is selected once based on flags (`ignore-case`, `no-trim`), eliminating per-cell branching.
- **Unsafe loops**: `row.unwrap_unchecked()` and `get_unchecked` accesses trade compile-time checks for speed. Each unsafe block includes a "safety:" comment explaining invariants (e.g., selection length matches vector sizes).
- **Decimal formatting**: `Decimal::from_f64` avoids float representation quirks and ensures deterministic CSV output across platforms.
- **Whitespace visualization**: Leveraging a pre-sized `String` buffer prevents per-value allocation churn when markers are enabled.
- **JSON cleanup**: The regex pass to remove empty stats arrays keeps payload small without additional serialization logic.

---

## Extending the Command

### 1. Adding a New CLI Flag
- Update the `Args` struct and `USAGE` string.
- Ensure `util::get_args` mapping handles the new flag.
- Thread the flag through relevant helper methods (`process_frequencies`, `counts`, etc.).
- Add tests covering combinations with existing options (`--limit`, `--json`, etc.).

### 2. Customizing Rank Behavior
- Modify `counts` to adjust how `current_rank` advances.
- Update tests that assert ranks (multiple exist in `tests/test_frequency.rs`).

### 3. Alternative Sampling for All-Unique Columns
- Extend `process_frequencies` to support more sampling strategies when `--unq-limit` applies.
- You may need to ensure deterministic ordering by storing pre-selected values elsewhere (currently, it just truncates the `counts` vector).

### 4. Additional JSON Metadata
- `FrequencyField` already houses `stats` from the cache. To add new derived fields, update `FieldStats` population inside `output_json`.
- Remember to guard behind flags where appropriate (`--no-stats`).

### 5. Performance Profiling Hooks
- Add optional logging via `log::debug!` if necessary. Existing code favors zero-cost in the hot path, so only enable logs behind explicit flags/env vars.

---

## Testing & Debugging

### Test Suite
- `tests/test_frequency.rs`: Comprehensive coverage of limits, sorting, NULL handling, stats cache interactions, JSON output, whitespace visualization, and property tests (`prop_frequency`, `prop_frequency_indexed`).

```bash
# Run entire suite
cargo test --test test_frequency -- --test-threads=1

# Focus on JSON behavior
cargo test test_frequency::frequency_json

# Property test for indexed equivalence (can be slow)
cargo test test_frequency::prop_frequency_indexed -- --ignored
```

### Manual Checks
- **Stats cache presence**
  ```bash
  qsv stats --stats-jsonl data.csv
  qsv frequency --limit 0 data.csv
  ```
- **Force all-unique expansion**
  ```bash
  QSV_STATSCACHE_MODE=none qsv frequency --limit 0 data.csv | head
  ```
- **Inspect whitespace markers**
  ```bash
  qsv frequency --vis-whitespace --limit 0 weird.csv | less
  ```
- **JSON output**
  ```bash
  qsv frequency --json --select 1 data.csv | jq '.fields[0]'
  ```

### Debugging Tips
- Use `RUST_LOG=debug` alongside ad-hoc `log::debug!` statements when developing new features. Remember to guard them or compile to dev builds only.
- For performance regressions, record a profiling session using `samply` or `perf` and inspect hot spots in `ftables_unweighted`/`ftables_weighted_internal` and `counts`.
- When adding new unsafe code, document invariants with `safety:` comments and consider writing targeted tests that exercise boundary conditions (empty columns, single-row datasets, high cardinality).

---

By understanding how the frequency command orchestrates stats-derived metadata, efficient tallying, and flexible output formatting, you can confidently extend it while preserving its performance profile.
