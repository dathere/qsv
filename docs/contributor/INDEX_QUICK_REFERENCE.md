# Quick Start Guide - QSV Index Development

## TL;DR

The `index` command builds random-access sidecar files (`.csv.idx`) so other commands can:
- **Seek by row offsets quickly**, enabling multi-threaded scans (`stats`, `frequency`, `slice`, `split`)
- **Auto-refresh stale indexes** whenever the CSV changes before the next indexed command runs
- **Respect auto-index policies** driven by `QSV_AUTOINDEX_SIZE` and `--cache-threshold` fallbacks
- **Avoid stdin/snappy pitfalls** by limiting operation to regular, seekable CSV files

## File Location
`src/cmd/index.rs` (69 lines)

## Key Entry Points

| Function | Purpose |
|----------|---------|
| `run(argv)` | Parses args, rejects snappy inputs, opens reader/writer, delegates index creation |
| `RandomAccessSimple::create(...)` | csv-index crate routine that writes byte offsets to the index |
| `Config::autoindex_file()` | (in `config.rs`) convenience hook reused across commands to build/update indexes |
| `Config::index_files()` | Shared logic that detects stale/missing indexes and auto-creates them on demand |

## Architecture Flow
```
CLI args
  ↓
util::get_args → Args { input, output? }
  ↓
Reject *.sz (snappy)
  ↓
Resolve index path → util::idx_path(<input>) or explicit --output
  ↓
Config::new(input) → reader_file()
  ↓
BufWriter (DEFAULT_WTR_BUFFER_CAPACITY)
  ↓
RandomAccessSimple::create(reader, writer)
  ↓
Flush writer → ready-to-use .idx sidecar
```

## Core Components to Know

| Component | Location | Role |
|-----------|----------|------|
| `Args` struct | `index.rs` | Deserializes CLI flags via `serde::Deserialize` |
| `Config::reader_file()` | `config.rs` | Returns a seekable CSV reader (snappy-aware) |
| `util::idx_path()` | `src/util.rs` | Computes `<input>.idx` (same directory as CSV) |
| `RandomAccessSimple` | `csv-index` crate | Writes row-start byte offsets and total byte length |
| `DEFAULT_WTR_BUFFER_CAPACITY` | `config.rs` | Large buffer for efficient sequential writes |

## Key Rust Concepts Used

| Concept | Usage |
|---------|-------|
| `serde::Deserialize` | Populate `Args` from CLI parsing |
| `BufWriter` | Buffer index writes to minimize syscalls |
| `PathBuf` | Manage derived output paths |
| Error propagation (`?`) | Bubble up I/O errors cleanly to the CLI surface |

## Performance & Behavior Notes

- **Snappy guard**: refuses `.sz` inputs because random seeking is impossible on compressed frames.
- **Auto-index ecosystem**: Other commands call `Config::index_files()`; if `QSV_AUTOINDEX_SIZE` is set and file size exceeds it, an index is built automatically—no manual `index` run needed.
- **Stale index recovery**: The next indexed command will transparently rebuild via `autoindex_file()` if CSV mtime is newer than `.idx`.
- **Warning threshold**: When no index exists and file size ≥ `NO_INDEX_WARNING_FILESIZE` (~10 MB), qsv warns that performance may suffer.

## Testing

```bash
# Dedicated index tests
cargo test --test test_index -- --test-threads=1

# Verify stale index auto-refresh (from tests)
cargo test test_index::index_outdated_stats

# Check autoindex threshold logic
cargo test test_index::index_autoindex_threshold_reached
```

## Related Resources

- Technical deep dive: `INDEX_TECHNICAL_GUIDE.md`
- Auto-index internals: `src/config.rs` (`autoindex_file`, `index_files`, `indexed`)
- Utility helpers: `src/util.rs` (`idx_path`, file metadata helpers)
- Usage docs: `docs/PROJECT_TECHNICAL_OVERVIEW.md` indexing section

## Quick Debugging Tips

```bash
# Manually build an index
./target/release/qsv index data.csv

# Use custom output path (diagnostics only)
./target/release/qsv index -o /tmp/data.idx data.csv

# Inspect whether qsv thinks a file is indexed (debug logging)
RUST_LOG=debug ./target/release/qsv stats data.csv

# Force rebuild by touching CSV then running an indexed command
touch data.csv
./target/release/qsv slice -i 10 data.csv
```

---

**Pro Tip**: Study `Config::index_files()` before touching the CLI—it governs auto-index creation, stale detection, and the user warnings other commands rely on.
