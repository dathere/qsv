# QSV Codebase Architecture and Testing Quality Analysis

## Executive Summary
qsv is a high-performance CSV data-wrangling toolkit in Rust (version 19.1.0, MSRV 1.95, Edition 2024). It features:
- 71 commands with conditional compilation via feature flags
- 4 binary variants (qsv, qsvmcp, qsvlite, qsvdp) with different feature sets
- Broad automated test coverage across 71 test modules (many feature-gated); test modules do not map one-to-one with commands
- Strong parallel processing and SIMD optimizations
- TypeScript MCP server integration for Claude Desktop

---

## 1. Command Dispatching Architecture

### Mechanism: Docopt + Enum Dispatch
**File**: `src/main.rs` (628 lines)

**Flow**:
1. Docopt parses CLI args with `options_first(true)` for flexible command parsing
2. Matches into `Command` enum (417-510) with feature-gated variants using `#[cfg(...)]`
3. `Command::run()` (512-627) performs case-validation, then massive match statement dispatching to `cmd::<command>::run(argv)`
4. Error handling via `CliResult<()>` with detailed error types (CliError enum)

**Key Features**:
- **Feature Gating**: Each command is conditionally compiled. Example:
  ```rust
  #[cfg(all(feature = "apply", feature = "feature_capable"))]
  Apply,
  ```
- **Command Routing**: Direct enum match to module function (e.g., `Command::Count => cmd::count::run(argv)`)
- **Version/Help Generation**: MCP skills and markdown help auto-generated from USAGE constants via `mcp_skills_gen.rs`

### Four Entry Points (Different Features)
| Binary | Source | Required Features | Use Case |
|--------|--------|-------------------|----------|
| `qsv` | `src/main.rs` | `feature_capable` | Full-featured CLI |
| `qsvmcp` | `src/main.rs` | `qsvmcp` (subset) | MCP server for Claude (62 commands) |
| `qsvlite` | `src/mainlite.rs` | `lite` (minimal) | Lightweight version (50 commands) |
| `qsvdp` | `src/maindp.rs` | `datapusher_plus` | Data pusher variant with geocode |

---

## 2. CSV I/O Abstraction

### Config-Driven I/O Layer
**File**: `src/config.rs` (670+ lines)

**Core Pattern**:
```rust
pub struct Config {
    path: PathBuf,
    idx_path: PathBuf,
    delimiter: u8,
    no_headers: bool,
    flexible: bool,
    trim: TrimFields,
    quote_style: QuoteStyle,
    // 20+ more fields...
}
```

**Key Methods**:
- `Config::reader()` / `Config::writer()` — factory methods returning CSV readers/writers
- `Config::reader_file_stdin()` — unified stdin/file input handling
- `Config::autoindex_file()` — automatic index creation for large files
- `Config::selection()` — column selection DSL via `select.rs`

**Format Detection**:
- `get_delim_by_extension()` — auto-detect delimiter from `.csv`, `.tsv`, `.ssv`
- `is_snappy_extension()` — compressed file handling
- `get_special_format()` — handles Excel, JSON, Parquet

**Features**:
- **Flexible CSV**: Handles ragged rows, quoting variations
- **Snappy Compression**: Native support for `.csv.snappy` files
- **Buffer Configuration**: Tunable read/write buffers (defaults: 8MB read, 32KB write)
- **Index Caching**: Auto-indexing for files >10MB (`AUTO_INDEXED`)

### Patched Dependencies
Key performance tuning in forked `csv` crate (`dathere/rust-csv` fork):
- SIMD-accelerated UTF-8 validation (uses `simdutf8`)
- Non-allocating `ByteRecord::trim()` / `StringRecord::trim()`
- SIMD scanning with `memchr` replacing byte-by-byte loops
- Reduced DFA lookup overhead

---

## 3. MCP Server Architecture

### TypeScript MCP Server (`.claude/skills/`)

**Structure** (21 files):
```
.claude/skills/src/
├── mcp-server.ts           — MCP protocol + tool dispatch
├── mcp-tools.ts            — Tool definitions & handlers (largest)
├── executor.ts             — qsv process spawning & streaming
├── mcp-sampling.ts         — Sampling for large files
├── config.ts               — Config + env var loading
├── duckdb.ts               — SQL translation to Polars/DuckDB
├── loader.ts               — Dynamic skill loading + BM25 search
├── converted-file-manager.ts — LIFO file cache
├── mcp-filesystem.ts       — Filesystem operations
├── browse-directory.ts     — Directory browsing
├── installer.ts            — Installation utilities
├── pipeline-manifest.ts    — Pipeline manifest handling
├── update-checker.ts       — Version detection + skill regen
├── bm25-search.ts          — Tool discovery index
├── types.ts                — TypeScript type definitions
├── utils.ts                — Error handling utilities
├── version.ts              — Version constant
├── index.ts                — Package entry point
├── ui/directory-picker-html.ts — Directory picker UI
├── wink-bm25-text-search.d.ts — Type definitions
└── wink-nlp-utils.d.ts     — Type definitions
```

**Tool Generation**:
- 55 auto-generated skill JSON files in `qsv/` directory
- Generated from qsv USAGE text via Rust generator (`src/mcp_skills_gen.rs`)
- MCP tools added via guidance hints (whenToUse, commonPattern, etc.)

**Process Model**:
- Spawns `qsv`/`qsvmcp` subprocess per tool invocation (non-persistent)
- Streaming stdout/stderr capture with configurable timeouts (default 10min)
- Lifecycle callbacks for resource tracking (`onSpawn`, `onExit`)

**Resource Limits** (verified 2026-03-18):
| Limit | Value | Location |
|-------|-------|----------|
| MCP Response | 850 KB | `mcp-tools.ts` |
| Large File Threshold | 10 MB | Auto-switches to sampling |
| Max stderr | 50 MB | `executor.ts` |
| Log Message | 4096 chars | Truncated silently |
| Default Timeout | 10 min | Configurable |

**Sampling Strategy** (NEW):
- Files > 10MB automatically sampled (deterministic seed)
- Result clearly marked as "sample" vs "complete"
- Reduces token usage while preserving data patterns

---

## 4. Feature Flags and Conditional Compilation

### Feature Hierarchy

**Default Features**:
```toml
default = ["mimalloc"]  # High-performance memory allocator
```

**Feature Preset Groups**:
```toml
distrib_features = ["feature_capable", "apply", "fetch", "foreach", 
                    "geocode", "luau", "mcp", "polars", "python", "to"]
all_features = ["distrib_features", "magika", "self_update", "ui"]
qsvmcp = ["feature_capable", "geocode", "luau", "mcp", "polars", "self_update"]
lite = []  # Minimal: base + standard commands only
```

**Command-Specific Features** (with optional dependencies):
- `apply` — NLP/encoding: censor, gender_guesser, strsim, thousands, titlecase, whatlang, sentiment
- `clipboard` — arboard (clipboard access)
- `color` — Terminal UI: anstream, crossterm, terminal-colorsaurus
- `fetch` / `fetchpost` — HTTP: governor (rate limiting), flate2, sled cache
- `foreach` — Bash execution (no additional deps, uses threadpool)
- `geocode` — Geo: geosuggest, geozero, sled cache
- `luau` — Script engine: mlua (Lua VM)
- `mcp` — (no deps; gates MCP code)
- `polars` — Analytics: full Polars 1.0+ with SQL, parquet, arrow
- `python` — pyo3 (Python integration)
- `prompt` — UI: rfd (file picker)
- `to` — Export: csvs_convert (Parquet/PostgreSQL/SQLite/Excel)
- `lens` — Interactive: csvlens
- `magika` — File type: magika + ort (ML-based detection)
- `self_update` — Auto-update: self_update with signatures

**Architecture Pattern**:
```rust
// In src/main.rs (lines 109-260)
#[cfg(feature = "apply")]
enabled_commands.push_str("    apply       Apply series of transformations...\n");

// In Command enum (lines 420-510)
#[cfg(all(feature = "apply", feature = "feature_capable"))]
Apply,

// In Command::run (lines 534-535)
#[cfg(all(feature = "apply", feature = "feature_capable"))]
Command::Apply => cmd::apply::run(argv),
```

**Polars Integration** (Advanced):
- Conditional polars features enable SQL engines: `polars = ["dep:polars"]`
- Polars nightly: `nightly-polars = ["polars/nightly", "polars/simd"]`
- Commands using polars: `joinp`, `pivotp`, `sqlp`, `to`, `scoresql`

**Binary Variants via Features**:
```toml
[[bin]]
name = "qsv"
required-features = ["feature_capable"]

[[bin]]
name = "qsvmcp"
required-features = ["qsvmcp"]  # = feature_capable + geocode + luau + mcp + polars + self_update

[[bin]]
name = "qsvlite"
required-features = ["lite"]  # Minimal set, no Polars/fetch/apply/etc.

[[bin]]
name = "qsvdp"
required-features = ["datapusher_plus"]  # = geocode + self_update
```

---

## 5. Testing Coverage and Quality

### Test File Organization
**Location**: `tests/` directory, 71 test modules (including feature-gated)

**Test Coverage by Category**:
| Category | Count | Examples |
|----------|-------|----------|
| Core I/O | 8 | count, cat, behead, headers, input, index, sniff, select |
| Transformation | 15 | sort, dedup, join, replace, rename, split, partition, transpose |
| Stats/Analysis | 5 | stats, frequency, pragmastat, moarstats |
| Advanced (Polars) | 4 | joinp, pivotp, sqlp, scoresql |
| Specialized | 8 | geocode, fetch, luau, python, apply, describe |
| Edge Cases | 5 | combos, comments, 100, validate, diff |

**Test File Structure**:
```rust
// tests/tests.rs (lines 1-148)
extern crate quickcheck;
extern crate rand;

// Feature-gated test modules
#[cfg(feature = "apply")]
mod test_apply;

#[cfg(all(feature = "polars", feature = "feature_capable"))]
mod test_sqlp;
```

**Test Patterns** (Comprehensive):

1. **Fixture Management** (`Workdir` utility):
   ```rust
   pub struct Workdir {
       root: PathBuf,
       dir: PathBuf,           // Unique UUID-based test directory
       flexible: bool,
   }
   
   // Methods
   wrk.create(name, rows)                  // CSV creation
   wrk.create_indexed(name, rows)          // With auto-index
   wrk.create_with_delim(name, rows, b'\t')  // Custom delimiter
   wrk.command("count")                    // Subprocess execution
   wrk.stdout(&mut cmd)                    // Capture output
   wrk.path(name)                          // Test file path
   ```

2. **Deterministic Test Data** (`svec!` macro):
   ```rust
   #[test]
   fn count_simple() {
       let wrk = Workdir::new("count_simple");
       wrk.create_indexed("in.csv", vec![
           svec!["letter", "number"],
           svec!["alpha", "13"],
           svec!["beta", "24"],
       ]);
   ```

3. **Subprocess Execution**:
   - Tests spawn actual qsv binary via `wrk.command()`
   - Captures stdout/stderr for assertions
   - Integration tests, not unit tests

4. **Property-Based Testing**:
   ```rust
   fn qcheck<T: Testable>(p: T) {
       QuickCheck::new()
           .quickcheck(p as fn(...) -> bool);
   }
   ```
   Used for fuzzy testing of parsing/serialization

5. **Feature-Gated Tests**:
   - Each test module has `#[cfg(...)]` guard matching command's feature
   - Example: `test_sqlp.rs` is `#[cfg(feature = "polars")]`
   - Prevents build failure if feature is disabled

### Test Infrastructure

**Workdir Cleanup**:
- Each test gets unique UUID-based directory under `xit/<test_name>/`
- Auto-cleaned in `Workdir::drop()` impl
- Prevents test pollution

**Serial Test Support**:
- `serial_test` crate for tests requiring exclusive access
- File-based locking (`file_locks` feature)

**QuickCheck Integration**:
- Randomized property testing for robustness
- RNG seeded for reproducibility
- Arbitrary generators for valid CSV data

### Coverage Analysis

**Most Core Commands Have Integration Tests**:
- ✓ Core commands (count, select, cat, etc.) — broadly covered
- ✓ Many advanced commands (sqlp, joinp, luau, python) — feature-gated
- ✓ Edge cases (empty files, malformed CSVs, large files) covered in many command tests
- `fetchpost` is tested within `test_fetch.rs` (shared module)

**Feature-gated Test Modules**:
- `test_applydp.rs` exists and is included from `tests/tests.rs` behind `#[cfg(feature = "datapusher_plus")]`
- Coverage varies by compiled feature set, so command-test parity is not 1:1

**Negative/Error Cases**:
- Test utility `wrk.stdout()` captures output
- Exit codes inferred from subprocess result
- Some tests check error messages

---

## 6. Performance Optimizations

### Parallel Processing (Rayon)

**18 Commands Using Parallel Processing** (direct rayon imports in src/cmd/):
Commands: `apply`, `applydp`, `count`, `datefmt`, `dedup`, `excel`, `geocode`, `jsonl`, `moarstats`, `pragmastat`, `sample`, `schema`, `sort`, `split`, `stats`, `template`, `tojsonl`, `validate`

**Patterns**:
- `use rayon::prelude::*;` imports parallel iterators
- `.par_iter()`, `.par_chunks()`, etc. for data-parallel operations
- `rayon::ThreadPool` for custom thread pools

**Example** (src/cmd/stats.rs):
```rust
// Parallel stats computation with memory-aware chunking
let which_stats = args.which_stats();
if args.sequential_stats() {
    args.parallel_stats(...)  // Uses rayon internally
}
```

### SIMD Optimization

**Direct SIMD Usage**:
1. **`simdutf8` (0.1)**: SIMD UTF-8 validation for CSV parsing
2. **`csv-nose` (1.0)**: CSV parsing with `runtime-dispatch-simd`
3. **`simd-json` (0.17)**: SIMD-accelerated JSON parsing (with `hints` in nightly)
4. **`atoi_simd` (0.18)**: SIMD integer parsing
5. **`base64-simd` (0.8, optional)**: SIMD base64 encoding

**Forked CSV Crate Optimizations**:
- SIMD memchr replacing byte-by-byte UTF-8 scanning
- Fast `is_non_numeric()` helper
- Memchr-based `needs_quotes()` lookup

**BLAKE3 Hashing**:
```toml
blake3 = { version = "1.8", features = ["rayon", "mmap"] }
```
- `rayon` — parallel hash computation
- `mmap` — memory-mapped file hashing

### Caching Strategies

**On-Disk Hash Table Cache** (`src/odhtcache.rs`):
```rust
pub struct ExtDedupCache {
    memo: HashMap<Vec<u8>, bool>,           // In-memory
    memo_limit: usize,                      // Capacity limit
    temp_file: Option<File>,                // Overflow to disk
    mmap: Option<Mmap>,                     // Memory-mapped overflow
}
```
- Used by `extdedup` command (large deduplication)
- Spills to disk when memory threshold exceeded
- Memory-mapped access for overflow data
- Tests: 11 unit tests covering edge cases

**Cached HTTP/Redis** (`cached` crate):
```toml
cached = { version = "0.59", features = [
    "ahash", "disk_store", "redis_ahash"
] }
```
- Used by `fetch` / `geocode` commands
- Redis support for distributed caching

**CSV Index Caching**:
- Auto-creates `.idx` files for random access
- Threshold: >10MB files auto-indexed (`AUTO_INDEXED`)
- Index format: `csv-index` crate

### Memory Optimization

**MiMalloc Allocator**:
```rust
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```
- 20-30% faster than default allocator
- Better large allocation handling

**Stats Command Memory Awareness** (`src/cmd/stats.rs`):
- `calculate_memory_aware_chunk_size()` — adapts to available RAM
- Prevents OOM on huge CSVs
- `WhichStats::needs_memory_aware_chunking()` — detects heavy operations

**Release Profile Tuning**:
```toml
[profile.release]
codegen-units = 1   # Full LTO
debug = false
lto = true          # Link-time optimization
opt-level = 3       # Maximum optimization
strip = true        # Remove debug symbols
```

### Build Optimization

**Patched Dependencies for Performance**:
- Custom `csv` fork with SIMD enhancements
- Latest calamine for Excel parsing
- Custom geosuggest fork with latest upstream

---

## 7. Argument Parsing Setup (Docopt)

### Docopt Integration

**Crate**: `qsv_docopt` (custom fork of `docopt`)

**Pattern**:
```rust
// 1. Define USAGE constant per command
const USAGE: &str = r#"
Usage:
  qsv count [<input>] [options]

Options:
  -h, --help           Display this message
  --no-headers         CSV has no header row
"#;

// 2. Derive Args struct with Deserialize
#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_help: bool,
    flag_no_headers: bool,
}

// 3. Parse and dispatch
fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());
}
```

**Key Features**:
- **`options_first(true)`** — Allows subcommands and flexible arg order
- **`Deserialize` trait** — Type-safe argument mapping
- **Automatic help** — `-h` triggers `CliError::Help(usage_text)`
- **Version support** — `.version(Some(util::version()))`

**Command-Level Help** (Not Global):
- Each command module has `run(argv)` function
- Command-level USAGE constants per file
- Hierarchical help: `qsv count -h` vs `qsv -h`

### Environment Variable Integration

**`.env` Loading**:
```rust
if util::load_dotenv().is_err() {
    return QsvExitCode::Bad;
}
```

**QSV-Specific Env Vars**:
- `QSV_CUSTOM_DELIMITER` — Set default delimiter
- `QSV_AUTOINDEX_SIZE` — Auto-index threshold
- `QSV_PREFER_DMY` — Date parsing preference
- `QSV_NO_UPDATE_CHECK` — Disable auto-update
- `QSV_MCP_*` — MCP server config (see config.ts)

**Display via `--envlist`**:
- Calls `util::show_env_vars()`
- Documents all QSV_* variables

---

## 8. Binary Variant Differentiation

### Comparison Table

| Aspect | qsv | qsvmcp | qsvlite | qsvdp |
|--------|-----|--------|---------|-------|
| **Source** | main.rs | main.rs | mainlite.rs | maindp.rs |
| **Feature Set** | feature_capable | qsvmcp preset | lite (minimal) | datapusher_plus |
| **Commands** | 71 (full) | 62 (MCP-exposed) | 50 (lite) | 40 (geo-enabled) |
| **Key Features** | All | MCP + Polars | Core only | Geocode + LLM |
| **Polars** | ✓ | ✓ | ✗ | ✗ |
| **LLM** (apply, describe) | ✓ | ✓ | ✗ | ✓ |
| **Fetch/Geocode** | ✓ | ✓ | ✗ | ✓ |
| **Luau/Python** | ✓ | ✓ | ✗ | ✗ |
| **Size (approx)** | 80MB | 75MB | 25MB | 45MB |

### Feature Matrix by Command

**Core (All Variants)**:
behead, cat, count, datefmt, dedup, describegpt, diff, edit, enum, excel, exclude, explode, extdedup, extsort, fill, fixlengths, flatten, fmt, frequency, headers, help, index, input, join, json, jsonl, partition, pragmastat, pro, pseudo, rename, replace, reverse, safenames, sample, schema, search, searchset, select, slice, sniff, sort, sortcheck, stats, moarstats, table, template (qsv+qsvmcp+qsvdp only), transpose, tojsonl, validate

**Polars-Only** (qsv + qsvmcp):
joinp, pivotp, sqlp, to, scoresql

**Optional LLM/Scripting**:
apply (NLP-heavy) — qsv, qsvmcp, qsvdp | luau, python — qsv, qsvmcp | fetch, fetchpost — qsv, qsvmcp | geocode, geoconvert — qsv, qsvmcp, qsvdp | foreach — qsv, qsvmcp

**UI-Only** (qsv):
clipboard, color, lens, prompt

---

## Summary: Quality Metrics

| Metric | Status | Evidence |
|--------|--------|----------|
| **Command Coverage** | Comprehensive | 71 commands; 71 feature-gated test modules with shared/conditional coverage |
| **Feature Isolation** | Excellent | Conditional compilation gates, feature presets |
| **Parallel Processing** | Excellent | 18 commands using rayon, memory-aware chunking |
| **SIMD Optimization** | Advanced | Custom CSV fork, 5+ SIMD deps, runtime dispatch |
| **Error Handling** | Robust | 8 error types, detailed context, logging integration |
| **Testing** | Strong | 71 test modules (feature-gated), integration tests, property-based testing |
| **I/O Abstraction** | Clean | Config-driven CSV handling, format detection, compression |
| **Documentation** | Excellent | Auto-generated help markdown, MCP guidance hints |
| **MCP Integration** | Production-Ready | 55 auto-generated skills, 30 .ts source files (incl. 2 .d.ts), streaming executor, resource limits |

---

**Last Updated**: April 25, 2026 | **Analysis Depth**: Comprehensive (45+ files examined; counts refreshed against current tree)
