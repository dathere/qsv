# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a standalone Rust project that generates shell tab-completion files for the [qsv](../../README.md) CLI tool. It **auto-generates** completions for 7 shells (**Bash, Zsh, Fish, PowerShell, Nushell, Fig, and Elvish**) by reading the `static USAGE` text from qsv's source files (`src/cmd/*.rs`) at build time.

The generated completion files live in `examples/` and cover 66 qsv commands (all except `applydp` and `generate`).

## Build & Generate Commands

```bash
# Regenerate all shell completions at once (must run from contrib/completions/)
bash generate_examples.bash

# Generate completions for a single shell
cargo run -- bash        # outputs to stdout
cargo run -- bash > examples/qsv.bash   # redirect to file

# Build without generating
cargo build
```

**Important:** Must be run from within the qsv repository, since it reads `src/cmd/*.rs` at runtime.

Valid shell arguments: `bash`, `zsh`, `fish`, `powershell`, `nushell`, `fig`, `elvish`

## Architecture

### Source Structure

- **`src/main.rs`** - Entry point; finds the repo root, builds the Command tree, and dispatches to `clap_complete::generate()`
- **`src/usage_parser.rs`** - Core auto-generation module that reads qsv source files and builds `clap::Command` definitions from USAGE text

### How Auto-Generation Works

1. **`find_repo_root()`** walks up from CWD to find the qsv repo (looks for `Cargo.toml` + `src/cmd/`)
2. **`build_cli()`** scans `src/cmd/*.rs` files and for each:
   - Extracts the `static USAGE` string (handles `r#"` and `r##"` delimiters)
   - Determines the CLI command name (handles aliases: `enumerate` → `enum`, `python` → `py`)
   - Parses the USAGE text with `qsv_docopt::parse::Parser` to discover long flags, their types (boolean vs value-taking), and subcommands
   - Extracts short flag mappings (`-d, --delimiter`) by parsing the USAGE text directly
   - Builds a `clap::Command` with all flags and subcommands
3. Assembles everything under a root `qsv` command with global flags

### Key Design Decisions

- **Short flags** are extracted by direct text parsing (not from docopt's SynonymMap, which doesn't expose synonyms through iteration)
- **Subcommands** get a copy of all parent flags (slightly permissive, but matches prior behaviour)
- **Duplicate short flags** (e.g., `-j` used by both `--jobs` and `--globals-json`) are resolved by keeping only the first occurrence
- **Invalid flag names** (decorative dash lines parsed as flags) are filtered out
- Files skipped: `mod.rs`, `python.pyo3-23.rs` (internal variant), `applydp.rs` (DataPusher+-specific)

## Adding/Updating a Command

No manual steps needed in this project! When qsv adds/removes commands or flags in `src/cmd/*.rs`, just regenerate:

```bash
bash generate_examples.bash
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` 4.5.x (with `string` feature) | Core CLI argument definition |
| `clap_complete` | Bash, Zsh, Fish, PowerShell, Elvish generation |
| `clap_complete_fig` | Fig (JavaScript) generation |
| `clap_complete_nushell` | Nushell generation |
| `qsv_docopt` 1.9 | USAGE text parsing (same parser used by qsv itself) |

## Relationship to Parent qsv Project

This project reads the `static USAGE` text from qsv's `src/cmd/*.rs` files at runtime. Completions automatically stay in sync with the qsv CLI -- no manual command definition files to maintain.
