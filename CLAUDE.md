# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Current Version**: 16.1.0 | **MSRV**: Rust 1.93

qsv is a blazingly-fast command-line CSV data-wrangling toolkit written in Rust. It's a fork of xsv with extensive additional functionality, focusing on performance, reliability, and comprehensive data manipulation capabilities.
It's the data-wrangling, analysis and FAIRification engine of several datHere products - qsv pro and Datapusher+, in particular.

## Build Commands

### Building Variants

qsv has the following binary variants with mutually exclusive feature flags:

```bash
# qsv - full-featured variant (use for development)
cargo build --locked --bin qsv -F all_features

# qsvlite - minimal variant
cargo build --locked --bin qsvlite -F lite

# qsvmcp - MCP server optimized variant (shares main.rs with qsv)
cargo build --locked --bin qsvmcp -F qsvmcp

# qsvdp - DataPusher+ optimized variant
cargo build --locked --bin qsvdp -F datapusher_plus
```

Do not use the `cargo build --release` option during development as it takes a long time.

### Testing

```bash
# Test qsv with all features
cargo test --features all_features

# Test qsvlite
cargo test --features lite

# Test qsvmcp
cargo test --features qsvmcp

# Test qsvdp
cargo test --features datapusher_plus

# Test specific command (e.g., stats)
cargo t stats -F all_features

# Test with specific features only
cargo t luau -F feature_capable,luau,polars
```

### Code Quality

```bash
# Format code (requires nightly)
cargo +nightly fmt

# Run clippy with performance warnings
cargo +nightly clippy -F all_features -- -W clippy::perf
```

## Architecture

### Source Code Organization

- **`src/main.rs`**, **`src/mainlite.rs`**, **`src/maindp.rs`** - Entry points for the binary variants (`qsv` and `qsvmcp` share `main.rs`)
- **`src/cmd/`** - Each command is a separate module (68 commands total)
- **`src/util.rs`** - Shared utility functions used across commands
- **`src/config.rs`** - Configuration handling, CSV reader/writer setup
- **`src/select.rs`** - Column selection DSL implementation
- **`src/clitypes.rs`** - Common CLI types and error handling
- **`src/index.rs`** - CSV indexing implementation for random access
- **`src/lookup.rs`** - Lookup table functionality for joins
- **`src/odhtcache.rs`** - On-disk hash table caching
- **`src/mcp_skills_gen.rs`** - MCP skill JSON generation from command USAGE text

### Command Structure

Each command in `src/cmd/` follows a standard pattern:

1. **Usage text** - Docopt-formatted usage at the top as a static string
2. **Args struct** - Serde-deserializable struct matching the usage text
3. **`run()` function** - Main entry point taking `&[&str]` argv
4. **Configuration** - Uses `Config::new()` to set up CSV reader/writer
5. **Processing logic** - Command-specific implementation

Example pattern from any command:
```rust
static USAGE: &str = r#"
Command description...

Usage:
    qsv command [options] [<input>]
"#;

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_output: Option<String>,
    // ... other flags
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    // Command implementation...
    Ok(())
}
```

### Commands with Subcommands

Eight commands have subcommands that provide specialized functionality:

1. **apply** (4 subcommands):
   - `operations` - 40 string, format, currency, regex & NLP operators (multi-column capable)
   - `emptyreplace` - replace empty cells with replacement string (multi-column capable)
   - `dynfmt` - dynamically construct columns from templates using formatstr
   - `calcconv` - parse and evaluate math expressions with units/conversions

2. **luau** (2 subcommands):
   - `map` - create new columns by mapping Luau script results for each row
   - `filter` - filter rows using Luau scripts (rows returning true are kept)

3. **cat** (3 subcommands):
   - `rows` - concatenate by rows (requires same column order)
   - `rowskey` - concatenate by rows (handles different columns/orders)
   - `columns` - concatenate by columns

4. **snappy** (4 subcommands):
   - `compress` - compress input using Snappy framing format (multithreaded)
   - `decompress` - decompress Snappy-compressed input
   - `check` - quickly check if first 50 bytes are valid Snappy data
   - `validate` - validate entire input is valid Snappy format

5. **geocode** (12 subcommands):
   - `suggest` - suggest city from partial name using Geonames index
   - `suggestnow` - suggest city from command line (no CSV input)
   - `reverse` - find city from WGS-84 coordinates using Geonames index
   - `reversenow` - find city from command line coordinates
   - `countryinfo` - get country info from ISO-3166 2-letter code
   - `countryinfonow` - get country info from command line
   - `iplookup` - lookup location from IP address using MaxMind GeoLite2
   - `iplookupnow` - lookup location from command line IP
   - `index-check` - check status of local Geonames index
   - `index-update` - update local Geonames index
   - `index-load` - load/rebuild local Geonames index
   - `index-reset` - reset local Geonames index

6. **to** (5 subcommands):
   - `postgres` - convert CSV to PostgreSQL
   - `sqlite` - convert CSV to SQLite
   - `xlsx` - convert CSV to Excel XLSX
   - `ods` - convert CSV to ODS (OpenDocument Spreadsheet)
   - `datapackage` - convert CSV to Frictionless Data Package

7. **pro** (2 subcommands):
   - `lens` - run csvlens in new Alacritty terminal window (Windows only)
   - `workflow` - import file into qsv pro Workflow

8. **validate** (1 subcommand):
   - `schema` - validate JSON Schema itself (draft 2020-12)

When working with these commands, note that subcommands are specified after the main command:
```bash
qsv apply operations trim,upper col1 file.csv
qsv luau map newcol "a + b" file.csv
qsv cat rows file1.csv file2.csv
```

### Key Architectural Patterns

**Streaming vs Memory-Intensive Commands**:
- Most commands stream CSV data row-by-row for constant memory usage
- Commands marked with 🤯 load entire CSV into memory (`dedup`, `pragmastat`, `reverse`, `sort`, `stats` with extended stats, `table`, `transpose`) - streaming modes available for `dedup`, `stats`, and `transpose`
- Commands marked with 😣 use memory proportional to column cardinality (`frequency`, `join`, `schema`, `tojsonl`)

**Index-Accelerated Processing**:
- Commands marked with 📇 can use CSV indices for faster processing
- Indices enable constant-time row counting, instant slicing, and random access
- Multithreaded processing (🏎️) often requires an index

**Stats Cache**:
- `stats` command creates `.stats.csv` and `.stats.csv.data.jsonl` cache files (via `--stats-jsonl`)
- Other "smart" commands (`describegpt`, `frequency`, `joinp`, `pivotp`, `sample`, `schema`, `sqlp`, `stats`, `tojsonl`) use the stats cache to optimize processing
- Cache validity checked via file modification times

**Frequency Cache**:
- `frequency` command creates `.freq.csv` and `.freq.csv.data.jsonl` cache files via `--frequency-jsonl`
- Other "smart" commands can use frequency cache to optimize processing

**Polars Integration**:
- Commands with 🐻‍❄️ use the latest Polars for vectorized query execution
- Currently: `color`, `count`, `joinp`, `lens`, `pivotp`, `prompt`, `schema`, `sqlp`
- Polars schema can be generated by `schema --polars` and used for optimized data type inference
- Polars is particularly useful for processing larger-than-memory CSV files

## Development Workflow

### Adding a New Command

1. Create `src/cmd/yourcommand.rs` following the standard pattern
2. Add module declaration in `src/cmd/mod.rs`
3. Add command registration in `src/main.rs` (conditional compilation based on features)
4. Add feature flag in `Cargo.toml` if needed
5. Create test file `tests/test_yourcommand.rs`
6. Add usage text with detailed examples and link to test file
7. Update README.md with command description

### Testing Conventions

- Each command has its own test file: `tests/test_<command>.rs`
- Tests use the `workdir` helper to create temporary test directories
- Use `svec!` macro for creating `Vec<String>` from string literals
- Tests double as documentation - link them from usage text
- Property-based tests use `quickcheck` for randomized testing

### Running Single Tests

```bash
# Run all tests for a specific command
cargo t test_stats -F all_features

# Run specific test function
cargo t test_stats::stats_cache -F all_features

# Run with different features
cargo t test_count -F feature_capable,polars
```

## Important Technical Details

### Memory Management

- Default allocator: **mimalloc** (can use standard with feature flags)
- OOM prevention: Two modes controlled by `QSV_MEMORY_CHECK` environment variable
  - NORMAL: checks if file size < TOTAL memory - 20% headroom
  - CONSERVATIVE: checks if file size < AVAILABLE memory - 20% headroom
- Commands marked with 🤯 load entire CSV into memory
- Commands marked with 😣 use memory proportional to column cardinality

### Performance Considerations

- **Always create an index** for files you'll process multiple times (`qsv index`)
- Set `QSV_AUTOINDEX_SIZE` environment variable to auto-index files above a size threshold
- Stats cache dramatically speeds up "smart" commands - run `qsv stats --stats-jsonl` first
- Use `--jobs` flag to control parallelism (defaults to number of logical processors)
- Snappy compression (.sz extension) provides fast compression/decompression
- Prebuilt binaries have `self_update` feature enabled for easy updates (`qsv --update`)
- Polars-powered commands (🐻‍❄️) can process larger-than-memory files efficiently

### CSV Handling

- Follows RFC 4180 with some flexibility for "real-world" CSVs
- UTF-8 encoding required (use `input` command to normalize)
- Automatic delimiter detection via `QSV_SNIFF_DELIMITER` or file extension
- Extensions: `.csv` (comma), `.tsv`/`.tab` (tab), `.ssv` (semicolon)
- Automatic compression for `.sz` files (Snappy framing format)

### Code Conventions

- Use `unsafe` blocks with `// safety:` comments explaining why it's safe
- Use `unwrap()` and `expect()` with `// safety:` comments when justified
- Extensive clippy configuration in `src/main.rs` - follow existing patterns
- Format with `cargo +nightly fmt` (uses custom rustfmt.toml settings)
- Apply clippy suggestions unless there's documented reason not to

### Dependency Management

- qsv uses latest stable Rust (current MSRV: **1.93**)
- Uses Rust edition 2024
- Aggressive MSRV policy - matches Homebrew's supported Rust version
- Uses latest versions of dependencies when possible
- Custom forks in `[patch.crates-io]` section for unreleased fixes/features
- Forks are often for PRs awaiting to be merged.
- Polars pinned to specific commit/tag upstream of the latest Rust release as their Rust release cycle lags behind their Python binding's release cycle.

### Feature-Gated Dependencies

**Magika (AI-powered file type detection)**:
- Requires `all_features` feature flag to enable
- Used by `sniff` command for enhanced MIME type detection
- Falls back to `file-format` crate in qsvlite/qsvdp variants
- Important consideration for MUSL builds (Magika not available)

### Key Environment Variables

- `QSV_SNIFF_PREAMBLE` - Number of rows to sniff for preamble detection
- `QSV_SKIP_FORMAT_CHECK` - Skip MIME type checking for faster processing
- `QSV_FORCE_COLOR` - Force colorized output even when not TTY
- `QSV_THEME` - Color theme (DARK/LIGHT)
- `QSV_DISKCACHE_TTL_SECS` - Disk cache TTL for geocode/fetch commands
- `QSV_MEMORY_CHECK` - Memory safety mode (NORMAL/CONSERVATIVE)
- `QSV_AUTOINDEX_SIZE` - Auto-index files above this size threshold
- `QSV_STATSCACHE_MODE` - Stats cache behavior (`auto`/`force`/`none`)
- `QSV_FREQ_CHUNK_MEMORY_MB` - Memory-aware chunking for frequency

See `docs/ENVIRONMENT_VARIABLES.md` for complete list.

### Important Files

- **`Cargo.toml`** - Extensive feature flags and patched dependencies
- **`CLAUDE.md`** - This file - guidance for Claude Code when working with qsv
- **`dotenv.template`** - All environment variables with defaults
- **`CHANGELOG.md`** - Version history and release notes (repo root, not in docs/)
- **`docs/ENVIRONMENT_VARIABLES.md`** - Environment variable documentation
- **`docs/PERFORMANCE.md`** - Performance tuning guide
- **`docs/FEATURES.md`** - Feature flag documentation
- **`docs/COMMAND_DEPENDENCIES.md`** - Inter-command dependencies and stats cache relationships
- **`resources/`** - Luau vendor files, template defaults, test data
- **`README.md`** - Main project documentation with command list and examples

## Common Patterns

### Parsing Arguments
```rust
let args: Args = util::get_args(USAGE, argv)?;
```

### Setting Up Config
```rust
let conf = Config::new(args.arg_input.as_ref())
    .delimiter(args.flag_delimiter)
    .no_headers(args.flag_no_headers)
    .flexible(args.flag_flexible);
```

### Reading CSV
```rust
let mut rdr = conf.reader()?;
let headers = rdr.byte_headers()?.clone();
for result in rdr.byte_records() {
    let record = result?;
    // Process record
}
```

### Writing CSV
```rust
let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
wtr.write_record(&headers)?;
for record in records {
    wtr.write_record(&record)?;
}
wtr.flush()?;
```

### Progress Bars
```rust
let progress = util::get_progress_bar(row_count)?;
// In loop:
progress.inc(1);
// After loop:
progress.finish();
```

### Using Index
```rust
if let Some(idx) = conf.indexed()? {
    let count = idx.count();
    // Fast indexed operations
} else {
    // Fallback to scanning
}
```

### Stats Cache Usage
```rust
use crate::util::{get_stats_records, StatsMode};

// StatsMode variants: Schema, Frequency, FrequencyForceStats, PolarsSchema, Outliers, None
if let Some((stats_headers, stats_records)) =
    get_stats_records(&args.arg_input, StatsMode::Schema, &args.into())?
{
    // Use cached stats
}
```

## MCP Agent Skills

qsv can auto-generate MCP skill definitions for AI agent integration:

```bash
# Regenerate MCP skill JSON files from USAGE text
qsv --update-mcp-skills
```

- Skills are generated from command USAGE text and README command table
- **51 MCP skills** generated (targeting qsvmcp commands; vs 68 CLI commands)
- Examples sections in USAGE text are parsed for agent-friendly examples
- Section headers (lines starting with "==") are skipped during parsing
- Generated files are stored in `.claude/skills/qsv/`
- See `.claude/skills/CLAUDE.md` for MCP server development guide
