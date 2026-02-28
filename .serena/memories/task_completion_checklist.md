# Task Completion Checklist

## After Editing Rust Code
1. `cargo +nightly fmt` — format code
2. `cargo +nightly clippy -F all_features -- -W clippy::perf` — lint
3. `cargo test --features all_features` (or targeted: `cargo t <command> -F all_features`)
4. If MCP skills affected: `./target/debug/qsv --update-mcp-skills`

## After Editing TypeScript (MCP Server)
1. `npm run build` — compile TypeScript
2. `npm test` — run all 393+ tests
3. If skill JSONs changed: rebuild with `npm run build`

## Before Committing
- Stage specific files (avoid `git add -A`)
- Never commit `.env`, credentials, or large binaries
- Commit message: conventional commits style (feat/fix/refactor/docs)
- End with `Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>`
- Use HEREDOC for commit messages

## Hooks (configured in .claude/settings.json)
- PreToolUse blocks: `Cargo.lock`, `.claude/skills/qsv/*.json`, `contrib/completions/examples/`
- PostToolUse: auto-runs `cargo +nightly fmt` on `.rs` file edits
