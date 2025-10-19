# QSV Index Command: Comprehensive Technical Guide

## Table of Contents
1. [Introduction & Purpose](#introduction--purpose)
2. [Command Responsibilities](#command-responsibilities)
3. [Architecture Overview](#architecture-overview)
4. [Args & CLI Parsing](#args--cli-parsing)
5. [Index File Generation](#index-file-generation)
6. [Auto-Index Integration](#auto-index-integration)
7. [Data Structures & Buffering](#data-structures--buffering)
8. [Error Handling & Edge Cases](#error-handling--edge-cases)
9. [Interactions with Other Commands](#interactions-with-other-commands)
10. [Performance Considerations](#performance-considerations)
11. [Extending the Command](#extending-the-command)
12. [Testing & Debugging](#testing--debugging)

---

## Introduction & Purpose

`qsv index` creates a random-access sidecar for a CSV file (`input.csv.idx`). The sidecar records byte offsets for each row so downstream commands can seek quickly, split work across threads, and detect stale data. While the CLI code is small, it anchors the broader auto-index ecosystem implemented in `config.rs`.

### Why indexes matter
- `stats`, `frequency`, `slice`, `split`, `table`, and other heavy hitters use indexes to parallelize workloads without re-parsing the entire file for each worker.
- Automatic index maintenance (via `QSV_AUTOINDEX_SIZE` and stale detection) relies on the index format produced here.
- Index files are stable on disk: once created, they can be reused repeatedly until the CSV changes.

---

## Command Responsibilities

1. Validate the input file path (no stdin, no snappy `.sz`).
2. Resolve the output location (`<input>.idx` by default or explicit `--output`).
3. Stream the CSV rows once, capturing row boundaries using the `csv-index` crate.
4. Flush the resulting index file and exit cleanly.

The command does **not** load the CSV into memory—it only walks the file sequentially.

---

## Architecture Overview

```
run(argv)
  ↓
util::get_args → Args { input, output? }
  ↓
Reject *.sz (snappy not supported)
  ↓
Resolve target idx path (default or --output)
  ↓
Config::new(input) → reader_file()
  ↓
BufWriter<fs::File> with DEFAULT_WTR_BUFFER_CAPACITY
  ↓
RandomAccessSimple::create(reader, writer)
  ↓
Flush writer → ready-to-use index
```

Key crates: `csv-index` for index creation, `serde` for args deserialization.

---

## Args & CLI Parsing

```rust
#[derive(Deserialize)]
struct Args {
    arg_input:   String,
    flag_output: Option<String>,
}
```
- Populated via `util::get_args(USAGE, argv)`.
- `arg_input` must be a path on disk; stdin (`-`) is unsupported.
- `flag_output` is optional; when omitted the command falls back to `util::idx_path(input)` (e.g., `data.csv` → `data.csv.idx`).

The USAGE string doubles as user help and ensures flags stay synchronized with the CLI.

---

## Index File Generation

The core of `run` is straightforward:

1. **Snappy check**: `input.to_lowercase().ends_with(".sz")` short-circuits with a CLI error. Snappy streams cannot be randomly indexed, so there is no point continuing.

2. **Path resolution**:
   ```rust
   let pidx = match args.flag_output {
       None => util::idx_path(Path::new(&args.arg_input)),
       Some(p) => PathBuf::from(&p),
   };
   ```
   `util::idx_path` appends `.idx` beside the source file in a cross-platform manner.

3. **Reader setup**:
   ```rust
   let rconfig = Config::new(Some(args.arg_input).as_ref());
   let mut rdr = rconfig.reader_file()?; // seekable File-based CSV reader
   ```
   `Config::reader_file()` honors delimiter inference and snappy decoding where appropriate (snappy was already filtered out earlier).

4. **Writer setup**:
   ```rust
   let mut wtr = io::BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, fs::File::create(pidx)?);
   ```
   The default buffer (currently 16 MiB) matches the buffering strategy used by other writers.

5. **Index creation**:
   ```rust
   RandomAccessSimple::create(&mut rdr, &mut wtr)?;
   io::Write::flush(&mut wtr)?;
   ```
   `RandomAccessSimple` walks the CSV once, emitting byte offsets for every record boundary. The flush ensures data hits disk before returning.

---

## Auto-Index Integration

While `index.rs` only builds indexes, the broader lifecycle is governed by `Config`:

- **`Config::autoindex_file()`** (lines ~540-575): silently creates an index when either `QSV_AUTOINDEX_SIZE` is set and the CSV size meets the threshold or when stale indexes are detected.
- **`Config::index_files()`** (lines ~578-663): central check used by other commands to determine if an index exists, auto-create one, and refresh stale indexes by comparing modification times.
- **`Config::indexed()`**: wraps `index_files()` and opens an `Indexed<fs::File, fs::File>` handle used by workloads that support parallel iteration.

Environment knobs:
- `QSV_AUTOINDEX_SIZE` (bytes): auto-create indexes when file size ≥ threshold.
- `NO_INDEX_WARNING_FILESIZE`: warn (but don’t auto-index) when a large file lacks an index.
- Stats-specific `--cache-threshold` negative values can momentarily override `autoindex_size` per run.

Understanding these helpers is essential before modifying `index.rs`, because they assume the index format produced here.

---

## Data Structures & Buffering

| Structure | Purpose |
|-----------|---------|
| `BufWriter<fs::File>` | Minimizes disk writes; sized using `DEFAULT_WTR_BUFFER_CAPACITY` |
| `csv::Reader<fs::File>` | Provided by `Config`, ensures consistent delimiter/header handling |
| `RandomAccessSimple` | Records row offsets (and final byte length) into the index file |
| `PathBuf` | Carries resolved output path |

The index format is binary (not CSV) and is consumed by `csv-index` when seeking rows later on.

---

## Error Handling & Edge Cases

- **Snappy input**: explicitly rejected with `fail_incorrectusage_clierror!`.
- **I/O errors**: both reader creation and writer creation propagate errors to the CLI, resulting in explicit failure messages.
- **Stdin**: unsupported; `get_args` requires a positional `<input>` path.
- **Custom output path**: allowed but seldom useful; downstream commands expect `<input>.idx`. The flag exists for diagnostics or alternate workflows.

`RandomAccessSimple::create` directly returns errors from the underlying `csv` crate if the file is malformed.

---

## Interactions with Other Commands

- **`stats` / `frequency`**: check `Config::indexed()`; if the index is missing and file size meets thresholds, they call `autoindex_file()` automatically.
- **`slice`**: uses the index (`-i` flag) for O(1) jumps to row ranges; tests verify stale index recovery.
- **`count`**: tolerates stale indexes by auto-refreshing before counting.

Tests in `tests/test_index.rs` cover these scenarios, ensuring that stale indexes are transparently rebuilt and that environment thresholds behave as expected.

---

## Performance Considerations

- **Sequential scan only**: index creation is a single-pass operation bound by disk throughput.
- **Large buffer**: writing through `BufWriter` reduces syscall overhead when emitting offsets.
- **No multi-threading**: index creation is fast enough that parallelization would complicate correctness without a noticeable benefit.
- **Compatibility**: Because the auto-index logic depends on stable file naming, avoid changing extension conventions without auditing `util::idx_path` and `Config::index_files`.

---

## Extending the Command

Potential enhancements and their touchpoints:

1. **Alternate index formats**: would require replacing `RandomAccessSimple` and coordinating with every consumer (`csv-index` crate, `Indexed` wrappers). High risk—validate compatibility carefully.
2. **Progress reporting**: wrap the reader loop with a progress bar by exposing a custom `Read` wrapper that reports bytes processed. Ensure overhead stays minimal.
3. **Compression safety checks**: additional guards (e.g., gz) could be added if future features permit gz decoding but not indexing.
4. **Metadata output**: storing build version or timestamp in a sidecar JSON may help debugging but would need consumption updates.

Whenever the index format changes, update `Config::index_files` tests and integration tests to catch regressions.

---

## Testing & Debugging

### Automated Tests

`tests/test_index.rs` validates key behaviors:
- `index_outdated_*`: verifies stale indexes trigger auto-refresh and don’t block dependent commands.
- `index_autoindex_threshold_*`: ensures `QSV_AUTOINDEX_SIZE` drives automatic creation when thresholds are crossed.

Run with:
```bash
cargo test --test test_index -- --test-threads=1
```

### Manual Checks

```bash
# Build an index manually
./target/release/qsv index data.csv
ls -lh data.csv.idx

# Confirm indexed command sees index
touch data.csv
RUST_LOG=debug ./target/release/qsv stats data.csv

# Force auto-index by environment threshold
QSV_AUTOINDEX_SIZE=100 ./target/release/qsv slice -i 10 data.csv
```

Enable logging (`RUST_LOG=info` or `debug`) to observe messages like `index stale... autoindexing...` emitted from `Config::index_files`.

---

By understanding the tight coupling between `index.rs`, the `csv-index` crate, and `Config`’s auto-index logic, you can safely extend indexing capabilities while keeping downstream commands fast and reliable.
