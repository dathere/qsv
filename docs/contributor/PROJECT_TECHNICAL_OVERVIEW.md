# qsv Project Technical Overview (New Contributor Edition)

Welcome! This document gives you a practical, technical map of the qsv codebase so you can onboard quickly, understand how pieces fit together, and start contributing with confidence.

If you’re new to Rust, this overview highlights the specific language patterns qsv uses (traits, iterators, generics, optional unsafe for perf, feature gating) and points you to the best places to begin.

---

## What is qsv?

qsv is a high-performance command-line toolkit for working with CSV and other tabular data. It focuses on speed, predictable behavior, and pragmatic ergonomics for data engineering tasks (profiling, selection, joins, transforms, validation, format conversion, indexing, and more).

- Language: Rust (edition 2024; minimum Rust 1.93)
- Binaries: `qsv` (full-featured), `qsvlite` (reduced feature set), `qsvdp` (Datapusher+ oriented)
- Philosophy: modern Rust, aggressive performance optimizations (including optional unsafe with documented “safety:” comments), comprehensive tests, and frequent dependency updates.

Key links in this repo:
- README: overall goals and non-goals
- `docs/` folder: deep dives (performance, features, environment variables, etc.)
- `resources/` folder: examples, vendored bits, and test data
- `tests/` folder: extensive integration-style tests per command

Useful docs to skim first:
- `docs/PERFORMANCE_TLDR.md` and `docs/PERFORMANCE.md`
- `docs/FEATURES.md`
- `docs/ENVIRONMENT_VARIABLES.md`
- `docs/Validate.md`, `docs/Fetch.md`, `docs/contributor/STATS_TECHNICAL_GUIDE.md` (overview for the stats command)

---

## Repository Layout

Top-level:
- `src/` — source code
  - `main.rs`, `mainlite.rs`, `maindp.rs` — entry points for the different binaries
  - `cmd/` — one module per command (e.g., `stats.rs`, `index.rs`, `select.rs`, etc.)
  - Core modules used across commands:
    - `config.rs` — CSV reader/writer configuration, delimiters, file/IO setup
    - `util.rs` — shared utilities (logging, panic handling, memory checks, conversions)
    - `index.rs` — CSV indexing support (random access & parallel processing enablement)
    - `select.rs` — column selection DSL and resolution
    - `lookup.rs`, `odhtcache.rs`, `clitypes.rs` — supporting types & helpers
- `tests/` — integration tests organized by command (`test_stats.rs`, `test_join.rs`, etc.)
- `docs/` — documentation (performance notes, features, user guides)
- `scripts/` — helper scripts (benchmarks, templates)
- `resources/` — examples and test fixtures
- `Cargo.toml` — workspace manifest with features, dependencies, and binary targets

---

## Build Targets and Feature Flags

qsv uses Cargo features to conditionally include commands and dependencies. Some commands (e.g., those using Polars, Python, Luau, geocoding, fetching) are gated behind feature flags.

Binaries:
- `qsv` (full-featured): requires `feature_capable` and additional features as needed
- `qsvlite`: built with `lite` feature
- `qsvdp`: built with `datapusher_plus`

Examples (macOS, zsh):
```sh
# Build full-featured binary (features are additive; pick what you need)
cargo build --release --features "feature_capable,polars,fetch,geocode"

# Build the lite binary
cargo build --release --bin qsvlite --features lite

# Build Datapusher+ oriented binary
cargo build --release --bin qsvdp --features datapusher_plus
```

Notes:
- The command list shown by `qsv --list` depends on the enabled features at build time.
- Two general-purpose allocators are available: mimalloc (default) and standard. You can choose between them via feature flags depending on your platform and needs.

---

## Runtime Architecture

### Entry points and command dispatch
- `src/main.rs` parses top-level arguments (via Docopt), initializes logging and environment, and dispatches to a `Command` enum variant.
- Each command’s implementation lives in `src/cmd/<name>.rs` and exposes `pub fn run(argv: &[&str]) -> CliResult<()>`.
- The `Command` enum and dispatcher are defined in `main.rs`. Adding a new command requires wiring it into that enum and its `run` match arm (behind the appropriate `#[cfg(feature = ...)]` if it’s feature-gated).

### CSV IO and configuration
- `config.rs` provides a `Config` builder that encapsulates input/output paths, delimiter, headers/no-headers mode, selection, and writer setup.
- The `csv` crate is used for parsing and writing. Delimiter inference and extensions-based suggestions are supported (e.g., TSV).

### Indexing and parallelism
- `csv-index` enables fast random access to rows. When present, commands can split work into chunks and process in parallel (threadpool + channels).
- `index` command creates `.csv.idx` files; many performance-oriented commands (like `stats`) auto-detect/use them.

Quick rules: when is a CSV “indexed”?
- If a fresh companion index exists (typically `<file>.csv.idx`), or qsv transparently (auto-)creates one, `Config::indexed()` returns Some and parallel paths are used.
- Auto-indexing triggers when file size ≥ a configured threshold:
  - Global: `QSV_AUTOINDEX_SIZE` (bytes)
  - For `stats`: `--cache-threshold -<bytes>` sets a per-run threshold; `-…5` deletes the index after.
- Not indexed for stdin (`-`) or Snappy files (`.sz`). If large (≥ 100MB) and not auto-indexed, qsv warns and proceeds sequentially.

See details: “How stats decides if a file is indexed” in `docs/STATS_TECHNICAL_GUIDE.md`.

### Performance patterns
- Hot paths may use `unsafe` to skip bounds checks when invariants are guaranteed by prior logic (“safety:” comments explain why it’s safe).
- SIMD-friendly libraries are used where applicable (`simd_json` on little-endian platforms, `atoi_simd`, etc.).
- Cache-aware data layout (e.g., `#[repr(C, align(64))]`) reduces false sharing in multi-threaded code.
- Threading via `threadpool` and `crossbeam-channel`.

### Error and logging conventions
- Errors flow via `CliResult<T>` and `CliError` (see `clitypes.rs`).
- User-facing errors are surfaced clearly; IO/CSV issues are categorized; broken pipe handled specially.
- Logging is set up in `util::init_logger()`; `RUST_LOG` controls verbosity.

---

## Key Commands and How They Work (at a glance)

- `stats` — Infers types and computes summary statistics. Streaming (O(1) memory) and non-streaming calculations. Uses index for parallelism and caches results to `*.stats.csv`. See `STATS_TECHNICAL_GUIDE.md` for an in-depth walkthrough.
- `index` — Builds a `.csv.idx` file enabling constant-time row count and efficient random access, unlocking multi-threaded processing in other commands.
- `select` — Column selection DSL (names, ranges, regex). Used by many commands to filter/reorder columns earlier for speed.
- `join` / `joinp` — Joins across CSVs; `joinp` leverages Polars (requires `polars` feature).
- `validate` — Validates CSVs against RFC4180 or JSON Schema (schema can be generated from `stats`).
- `json`, `jsonl`, `tojsonl` — Conversions to/from JSON/JSONL.
- `fetch`, `fetchpost` — HTTP-enriched data retrieval per row (feature-gated), with pre/post request hook support.
- `datefmt` — Robust date/datetime parsing/formatting.
- Many more: `sort`, `sortcheck`, `sample`, `flatten`, `explode`, `frequency`, `schema`, `template`, etc. Use `qsv --list` to see what your build supports.

---

## Testing Strategy

- Integration-style tests live in `tests/` and are organized per command: `tests/test_stats.rs`, `tests/test_join.rs`, etc.
- A custom `Workdir` harness manages temp dirs and fixtures. Tests typically:
  1. Set up a small CSV (inline)
  2. Run the command via the compiled test binary
  3. Assert output snippets/headers/counts

Running tests:
```sh
# Run all tests
cargo test

# Run stats-only tests (large suite)
cargo test --test test_stats -- --test-threads=1

# Focus on a specific test module or name
cargo test test_join
```

Tips:
- Some tests are performance-sensitive; avoid adding flakiness.
- Many commands depend on `stats` caches or indexes; tests may create/clean them.

---

## Coding Standards & Conventions

- Edition 2024, stable Rust, newest practical dependency versions.
- Unsafe code allowed only with clear `// safety:` comments; the same goes for `unwrap()` usage with rationale.
- Keep hot loops tight; prefer iterators and pre-allocated buffers where possible.
- Follow existing patterns for argument parsing (Docopt), configuration via `Config`, and per-command `run(argv)` signature.
- Linting and formatting: `clippy.toml` and `rustfmt.toml` are provided. Aim for zero warnings in changed code.

```sh
# Format and lint before committing
cargo +nightly fmt
cargo +nightly clippy -F all_features -- -W clippy::perf
```

---

## Environment Configuration

qsv behavior can be tuned via environment variables (delimiter defaults, date parsing preferences, cache thresholds, etc.). See `docs/ENVIRONMENT_VARIABLES.md`.

Examples:
```sh
# Prefer day-month-year when inferring dates (stats)
export QSV_PREFER_DMY=1

# Default delimiter when reading stdin
export QSV_DEFAULT_DELIMITER=\t
```

---

## How to Add a New Command

1. Create a module `src/cmd/<your_command>.rs` exposing `pub fn run(argv: &[&str]) -> CliResult<()>`.
2. Gate with `#[cfg(feature = "...") ]` if the command has optional dependencies.
3. Wire it in `main.rs`:
   - Add a variant to the `Command` enum (gated appropriately)
   - Add a `match` arm to dispatch to your module’s `run()`
   - Add to the `--list` output string (behind the same feature gate)
4. Add tests in `tests/test_<your_command>.rs` using the `Workdir` harness.
5. Update relevant docs under `docs/`.

Design tips:
- Reuse `Config` for CSV IO and `select` for column filtering.
- If your command benefits from random access or parallelism, consider `csv-index` and chunked processing via `threadpool` and `crossbeam-channel`.
- For heavy numeric work, prefer streaming/online algorithms; only load whole columns when necessary.

---

## First 30 Minutes: A Suggested Path

1. Build and list commands:
```sh
cargo build --release
./target/release/qsv --list
```
2. Run a few sample commands on fixtures under `resources/test/`.
3. Read `src/main.rs` for command routing, then open a command module (e.g., `src/cmd/stats.rs`) to see the pattern.
4. Skim `docs/PERFORMANCE_TLDR.md` and `docs/FEATURES.md`.
5. Run the test suite for one command (e.g., stats):
```sh
cargo test --test test_stats -- --test-threads=1
```

---

## Performance Toolbox (When You’re Ready)

- Online/streaming algorithms (e.g., Welford for mean/variance)
- Index-aware parallel processing (split by chunks, merge results via `Commute` trait)
- Memory layout and cache friendliness (`#[repr(C, align(64))]` where justified)
- SIMD-friendly parsing and string handling (e.g., `simd_json` on LE targets)
- Snappy compression for large intermediate outputs (see `snappy` command)

---

## Where to Ask for Help

- Open a GitHub Discussion or Issue with a focused question
- Use test files under `resources/test/` to reproduce and share minimal examples
- Cross-reference existing command modules for implementation patterns

---

## See Also

- Deep dive: `docs/contributor/STATS_TECHNICAL_GUIDE.md` and `docs/PERFORMANCE.md`
- Features reference: `docs/FEATURES.md`
- Environment variables: `docs/ENVIRONMENT_VARIABLES.md`
- Validation & schema generation: `docs/Validate.md`
- Fetch & HTTP integration: `docs/Fetch.md`

Happy hacking! 🚀
