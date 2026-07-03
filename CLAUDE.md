# CLAUDE.md

## General Rules

When unsure how to use a project tool or publish workflow, check existing docs first (e.g., marketplace docs, plugin docs) before guessing. Never fabricate CLI flags.

## Documentation

When counting items in documentation (tools, commands, features), always verify counts by explicitly listing and numbering each item. Never estimate counts.

Help files (e.g., command help docs, ToC entries) are auto-generated. Do NOT manually create or edit them — run the help generator instead and verify output.

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
- Regenerate Help and ToC entries: `qsv --generate-help-md`

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

## roborev is a Local Tool

`roborev` is a **local, on-host code-review daemon** — it stores reviews in a local data store under the repo and does not post to GitHub, GitLab, or any external code-review service. All `roborev` subcommands (`review`, `show`, `comment`, `close`, `fix`, `refine`, `respond`, `init`, `fix --open --list`) mutate local state only and should be treated as in-project tooling, not as external publishes. `roborev comment` and `roborev close` in particular are local-only annotations on a local job record; they do not send data to any external endpoint.

## Debugging

When debugging, state your hypothesis explicitly before investigating. If the first hypothesis fails, don't try variations of the same idea — step back and consider fundamentally different root causes.

## MCP Tool Usage

Always use Serena MCP tools (find_symbol, etc.) for code navigation and Context7 MCP for library documentation lookups. These tools are configured and should be preferred over Grep/Read for symbol-level exploration.

- Prefer `mcp__serena__replace_symbol_body` for whole-symbol replacement; verify the file after edit to catch duplicated tails or lost function bodies.
- Avoid `sed` with backreferences (`\1`) for multi-line edits — use the Edit tool instead.

## docopt / USAGE Editing Conventions

When editing docopt USAGE strings, never let a wrapped help line begin with a flag (e.g., `--grid-cols-wide`); docopt will parse it as an option definition and break arg parsing for all commands. Verify by running the viz test suite after any USAGE edit.

## Visualization Workflow

After implementing or fixing a viz/dashboard feature, always verify the result visually in-browser and regenerate the gallery before opening a PR.

## Testing / Plotly JSON

When editing plotly JSON in tests, edit programmatically rather than with the Edit tool, since plotly unicode-escapes angle brackets and the Edit tool normalizes the escaped form, causing assertion failures.

