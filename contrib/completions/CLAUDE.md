# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a standalone Rust project that generates shell tab-completion files for the [qsv](../../README.md) CLI tool. It uses the Clap argument parser to programmatically produce completions for 7 shells: **Bash, Zsh, Fish, PowerShell, Nushell, Fig, and Elvish**.

The generated completion files live in `examples/` and cover 65 qsv commands (all except `applydp` and `generate`).

## Build & Generate Commands

```bash
# Regenerate all shell completions at once
bash generate_examples.bash

# Generate completions for a single shell
cargo run -- bash        # outputs to stdout
cargo run -- bash > examples/qsv.bash   # redirect to file

# Build without generating
cargo build
```

Valid shell arguments: `bash`, `zsh`, `fish`, `powershell`, `nushell`, `fig`, `elvish`

## Architecture

### Source Structure

- **`src/main.rs`** - Entry point; parses the shell argument and dispatches to `clap_complete::generate()`
- **`src/cli.rs`** - `build_cli()` assembles the full qsv `Command` tree: global flags (`--list`, `--envlist`, `--update`, `--updatenow`, `--version`) + all 65 subcommands
- **`src/cmd/mod.rs`** - Module declarations for all command files
- **`src/cmd/*.rs`** - One file per qsv command, each exporting a `<name>_cmd() -> Command` function

### Command Definition Patterns

**Simple command** (no subcommands):
```rust
pub fn count_cmd() -> Command {
    Command::new("count").args([
        arg!(--"human-readable"),
        arg!(--width),
        // ...
    ])
}
```

**Command with subcommands** (apply, cat, geocode, luau, python, pro, snappy, to):
```rust
pub fn apply_cmd() -> Command {
    let global_args = [arg!(--"new-column"), arg!(--rename), /* ... */];
    Command::new("apply")
        .subcommands([
            Command::new("operations").args(&global_args),
            Command::new("emptyreplace").args(&global_args),
            // ...
        ])
        .args(global_args)
}
```

### Naming Exceptions

- `enumerate` module exports `enum_cmd()` (not `enumerate_cmd()`)
- `python` module exports `py_cmd()` (not `python_cmd()`)

## Adding/Updating a Command

1. Create or edit `src/cmd/<command>.rs` with a `pub fn <command>_cmd() -> Command` function listing all flags
2. Add `pub mod <command>;` in `src/cmd/mod.rs`
3. Import and wire it in `src/cli.rs` (both the `use` import and the `build_cli()` subcommands list)
4. Run `bash generate_examples.bash` to regenerate all completion files

When updating flags for an existing command, only step 4 is needed.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` 4.5.x | Core CLI argument definition |
| `clap_complete` | Bash, Zsh, Fish, PowerShell, Elvish generation |
| `clap_complete_fig` | Fig (JavaScript) generation |
| `clap_complete_nushell` | Nushell generation |

## Relationship to Parent qsv Project

This project mirrors the qsv CLI's command structure but is independently compiled. When qsv adds/removes commands or flags, the corresponding `src/cmd/*.rs` files here must be manually updated to stay in sync. The reference for flags is the USAGE text in each `src/cmd/*.rs` file in the main qsv source tree.
