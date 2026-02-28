# Style and Conventions

## Rust
- Format with `cargo +nightly fmt` (custom rustfmt.toml)
- Clippy with perf warnings: `cargo +nightly clippy -F all_features -- -W clippy::perf`
- `unsafe` blocks require `// safety:` comments
- `unwrap()`/`expect()` require `// safety:` comments when justified
- Each command follows: USAGE text → Args struct → `run()` function → Config → Processing
- Use `util::get_args(USAGE, argv)?` for argument parsing
- Use `Config::new()` for CSV reader/writer setup
- Tests use `workdir` helper, `svec!` macro for string vectors

## TypeScript (MCP Server)
- Strict TypeScript config
- Avoid `any` — use `unknown` + type guards
- Use `async`/`await` over promise chains
- `const` over `let`, never `var`
- Template literals for string interpolation
- Use `getErrorMessage(error)` from `utils.ts` in catch blocks (not inline pattern)
- Use `isNodeError(error)` from `utils.ts` for NodeJS.ErrnoException checks
- Use `errorResult()`/`successResult()` from `utils.ts` for MCP responses
- Tests use `node:test` and `node:assert` (no external framework)
- Error typing: `catch (error: unknown)` with type guards, never `catch (error: any)`

## General
- No emojis unless user requests them
- Avoid over-engineering — minimal changes for current task
- Don't add docstrings/comments to unchanged code
