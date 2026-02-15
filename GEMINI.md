# qsv: Blazing-fast Data-Wrangling Toolkit

## Project Overview

`qsv` is a high-performance command-line program for querying, slicing, sorting, analyzing, filtering, enriching, transforming, validating, joining, formatting, and documenting tabular data (CSV, Excel, etc.). It is a fork of the popular `xsv` utility, significantly expanded with numerous features and commands.

### Key Technologies
- **Language:** Rust (requires latest stable or nightly)
- **CLI Parser:** `docopt` (via a maintained fork `qsv_docopt`)
- **Core Engine:** `csv` crate for fast parsing, `polars` for vectorized queries in certain commands.
- **Embedded Scripting:** `Luau` (a fast Lua dialect) and `Python` (optional).
- **Template Engine:** `MiniJinja`.

### Architecture
- **Modular Commands:** Each command is implemented as a self-contained module in `src/cmd/`.
- **Streaming by Default:** Most commands process data in a streaming fashion with constant memory usage, enabling processing of arbitrarily large files.
- **Feature Gated:** Functionality is modularized using Rust feature flags (e.g., `polars`, `luau`, `python`, `geocode`).
- **Performance Focused:** Employs multithreading (via `rayon`), custom memory allocators (`mimalloc`), and extensive caching strategies.

## Building and Running

### Build Variants
`qsv` supports several binary variants tailored for different needs:
- `qsv`: Feature-capable variant (all features enabled).
- `qsvlite`: Slimmed-down version (similar to original `xsv`).
- `qsvdp`: Optimized for `DataPusher+`.
- `qsvpy`: Variant with Python support enabled.

### Key Commands
- **Build (Release):** `cargo build --release --locked --bin qsv --features all_features`
- **Test:** `cargo test --features all_features`
- **Lint:** `cargo +nightly clippy -F all_features -- -W clippy::perf`
- **Format:** `cargo +nightly fmt`

## Development Conventions

- **Rust Version:** Requires latest stable Rust supported by Homebrew. Nightly is required for formatting and clippy.
- **Coding Style:**
  - Strict adherence to `rustfmt` (nightly).
  - Use `clippy` for performance and correctness checks.
  - `unwrap()` and `expect()` are allowed but should have a safety comment.
- **CLI Design:** Uses `docopt` for usage-driven argument parsing. Usage text is embedded at the top of each command's source file.
- **Error Handling:** Standardized via `CliError` and `CliResult` in `src/clitypes.rs`. Uses macros like `fail!`, `fail_clierror!`, etc.
- **Testing:** Extensive test suite (~2,448 tests) in the `tests/` directory. Each command should have its own `test_<command>.rs` file.
- **Logging:** Uses `flexi_logger`. Level can be controlled via `QSV_LOG_LEVEL`.

## Configuration

`qsv` is highly configurable via environment variables (prefix `QSV_`) and `.env` files.
Key variables include:
- `QSV_DEFAULT_DELIMITER`: Set default delimiter.
- `QSV_MAX_JOBS`: Control multithreading.
- `QSV_AUTOINDEX_SIZE`: Automatically create indices for large files.
- `QSV_MEMORY_CHECK`: Enable Out-of-Memory prevention.

For a full list of environment variables, refer to `docs/ENVIRONMENT_VARIABLES.md`.
