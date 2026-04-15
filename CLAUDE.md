# CLAUDE.md

## General Rules

When unsure how to use a project tool or publish workflow, check existing docs first (e.g., marketplace docs, plugin docs) before guessing. Never fabricate CLI flags.

## Documentation

When counting items in documentation (tools, commands, features), always verify counts by explicitly listing and numbering each item. Never estimate counts.

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

## Rust / qsv Development

For qsv Rust work: after editing code, always run `cargo test` and `cargo clippy` before committing. For feature-gated code, test with the relevant feature flag enabled.

## Code Review Response

For Copilot/code review responses: apply the fix, run tests, commit, and reply to the review comment. Do not dismiss review findings without verifying them in code first.

## Debugging

When debugging, state your hypothesis explicitly before investigating. If the first hypothesis fails, don't try variations of the same idea — step back and consider fundamentally different root causes.
