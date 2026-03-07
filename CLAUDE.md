# CLAUDE.md

## Tools & commands

- Build qsv: `cargo build --locked --bin qsv -F all_features`
- Build qsvlite: `cargo build --locked --bin qsvlite -F lite`
- Build qsvmcp: `cargo build --locked --bin qsvmcp -F qsvmcp`
- Build qsvdp: `cargo build --locked --bin qsvdp -F datapusher_plus`
- Do not use `--release` during development.
- Test qsv: `cargo test -F all_features`
- Test qsvlite: `cargo test -F lite`
- Test qsvmcp: `cargo test -F qsvmcp`
- Test qsvdp: `cargo test -F datapusher_plus`
- Test single command: `cargo t stats -F all_features`
- Test specific function: `cargo t test_stats::stats_cache -F all_features`
- Regenerate MCP skill JSONs: `qsv --update-mcp-skills`

## Workflow requirements

Adding a new command requires changes in multiple places:
1. Create `src/cmd/yourcommand.rs` following the pattern in any existing command
2. Add module declaration in `src/cmd/mod.rs`
3. Add command registration in `src/main.rs` (conditional on features)
4. Add feature flag in `Cargo.toml` if needed
5. Create `tests/test_yourcommand.rs`
6. Add usage text with examples and link to test file
7. Update README.md with command description
