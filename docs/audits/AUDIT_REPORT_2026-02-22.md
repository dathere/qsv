# Documentation Audit Report

Generated: 2026-02-22 | Commit: 159b77e22 | Branch: master

## Executive Summary

| Metric | Count |
|--------|-------|
| Documents scanned | 12 |
| Claims verified | ~140 |
| Verified TRUE | ~105 (75%) |
| **Verified FALSE** | **~30 (21%)** |
| Needs human review | ~5 (4%) |

## False Claims Requiring Fixes

### CLAUDE.md

No false claims found. All 10 major claims verified TRUE, including version (16.1.0), MSRV (1.93), command count (68), subcommand counts, MCP skills count (56), entry points, Polars commands, and stats cache commands.

### README.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Line ~256 | Feature list `feature_capable,apply,fetch,foreach,geocode,luau,polars,python,self_update,to,ui` equivalent to `-F all_features` | `all_features` also includes `mcp` and `magika` via `distrib_features`; the explicit list is incomplete | HIGH | Update explicit feature list to include `mcp` and `magika`, or remove explicit list and just use `-F all_features` |
| Line ~282 | "four binary variants" (counting qsvpy) | Only 3 `[[bin]]` entries in Cargo.toml: qsv, qsvlite, qsvdp. `qsvpy` is a compile-time variant of qsv, not a distinct binary | LOW | Clarify that qsvpy is qsv compiled with Python feature |
| `diff` command row | Marked with `🪄` (uses stats/frequency cache) | `diff.rs` contains zero references to `stats`, `frequency`, `get_stats_records`, or `StatsMode` | MEDIUM | Remove `🪄` marker from `diff` |
| Lines ~486, ~498 | "~2,448 tests" | Actual count is ~1,867 `#[test]` functions across the codebase | MEDIUM | Update test count |
| Line ~461 | Display text says `dotenv.template.yaml` | Actual filename is `dotenv.template` (no `.yaml` extension); hyperlink target is correct | LOW | Fix display text to `dotenv.template` |

### docs/FEATURES.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| `all_features` description | Lists features without `mcp` and `magika` | `all_features` = `distrib_features` + `magika` + `self_update` + `ui`; `distrib_features` includes `mcp` | HIGH | Add `mcp` and `magika` to the documented list |
| `nightly` feature | Claims it enables nightly in `hashbrown` and `polars` | `hashbrown` is not a dependency; `polars` has a separate `nightly-polars` feature. Actual: `crc32fast/nightly`, `pyo3/nightly`, `rand/simd_support`, `simd-json/hints`, `foldhash/nightly` | HIGH | Update to match actual `nightly` feature contents in Cargo.toml |
| `ui` feature | Says enables "clipboard, lens, and prompt" | Actually enables `clipboard`, `color`, `prompt`, `lens` (`color` is missing) | MEDIUM | Add `color` to the `ui` feature description |
| `distrib_features` | Claims "all features except `self_update`" | Missing `self_update`, `ui`, AND `magika` | MEDIUM | Clarify which features are excluded |
| `color` feature | Not documented anywhere | `color` is a standalone feature in Cargo.toml (line 437) and part of `ui` | LOW | Add `color` feature documentation |

### docs/ENVIRONMENT_VARIABLES.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| `QSV_REDIS_CONNSTR` default | `redis:127.0.0.1:6379/1` | Code uses `redis://127.0.0.1:6379/1` (missing `//`) | LOW | Fix to `redis://127.0.0.1:6379/1` (also `QSV_FP_REDIS_CONNSTR`, `QSV_DG_REDIS_CONNSTR`) |
| `QSV_MAX_JOBS` command list | Lists 15 commands | Code shows `--jobs` flag in ~28 commands; missing `moarstats`, `searchset`, `pragmastat`, `sample`, `template`, `replace`, `geocode`, `excel`, `sqlp`, `search`, `pivotp`, `jsonl`, `datefmt` | MEDIUM | Update command list |
| `QSV_PROGRESSBAR` command list | Lists 11 commands | Missing at least `moarstats`, `template`, `geocode`, `sniff`, `datefmt` | MEDIUM | Update command list |
| Polars commands list | Lists only `count`, `joinp`, `pivotp`, `sqlp` | `color`, `lens`, `schema` also use Polars | LOW | Update to include all Polars-using commands |

### docs/PERFORMANCE.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Benchmark script | `scripts/benchmark-basic.sh` | Actual file is `scripts/benchmarks.sh` | MEDIUM | Fix filename reference |
| Line ~161 | Write buffer default is 256k | Code has `DEFAULT_WTR_BUFFER_CAPACITY = 512 * (1 << 10)` = 512k; same doc correctly says 512k at line ~204 | HIGH | Fix to 512k (self-contradictory) |
| Line ~46 | `diff` uses stats cache for fingerprint comparison | `diff.rs` has zero references to stats/cache/fingerprint | MEDIUM | Remove `diff` from stats cache users |
| Line ~208 | Env var `QSV_REDIS_TTL_SECONDS` | Actual env var is `QSV_REDIS_TTL_SECS` | HIGH | Fix env var name |
| Line ~209 | Env var `QSV_DISK_CACHE_TTL_SECONDS` | Actual env var is `QSV_DISKCACHE_TTL_SECS` (no underscore, SECS not SECONDS) | HIGH | Fix env var name |
| Line number refs | `config.rs#L16` for buffer capacity | Constant is at line 23 | LOW | Update line reference (or remove, since lines drift) |

### docs/contributor/PROJECT_TECHNICAL_OVERVIEW.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Line 13 | "minimum Rust 1.90" | MSRV is 1.93 (Cargo.toml) | HIGH | Update to 1.93 |
| Lines 27, 240 | `STATS_TECHNICAL_GUIDE.md` at "repo root" | File is at `docs/contributor/STATS_TECHNICAL_GUIDE.md` | MEDIUM | Fix path |
| Line 165 | `cargo fmt` | Should be `cargo +nightly fmt` (requires nightly) | MEDIUM | Update to `cargo +nightly fmt` |
| Line 166 | `cargo clippy --all-features -- -D warnings` | Should be `cargo +nightly clippy -F all_features -- -W clippy::perf` | MEDIUM | Update clippy command |

### docs/contributor/STATS_TECHNICAL_GUIDE.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Line 267 | `FieldType` includes `TBool` variant | Actual enum has 6 variants: `TNull`, `TString`, `TFloat`, `TInteger`, `TDate`, `TDateTime` -- no `TBool` | HIGH | Remove `TBool` from documentation |
| Lines 422-425 | `TypedSum` is an enum with `Integer(i64)`, `Float(f64)`, `FloatOverflow` | `TypedSum` is actually a **struct**: `struct TypedSum { float: Option<f64>, integer: i64, stotlen: u64 }` | HIGH | Update to show actual struct definition |
| Line 257 | "seven data types" inferred | Only 6 types in `FieldType` enum | MEDIUM | Change to "six data types" |
| Line 672 | Cache file `mydata.stats.csv.json` | User-facing extension is `.stats.csv.jsonl` | MEDIUM | Fix extension |
| Lines 545-546 | `compute()` signature missing `weight_col_idx` param | Actual: `fn compute<I>(&self, sel: &Selection, it: I, weight_col_idx: Option<usize>) -> Vec<Stats>` | MEDIUM | Add `weight_col_idx` parameter |
| Line 919 | Hardcoded path `/Users/pascal/git-hub/qsv` | Personal path in contributor docs | LOW | Replace with generic path |
| Line 1065 | References `copilot-instructions.md` without path | File exists at `.github/copilot-instructions.md`, not at repo root | LOW | Fix path reference |
| Line 1067 | "Rust 1.90+" | MSRV is 1.93 | HIGH | Update to 1.93 |

### docs/contributor/STATS_QUICK_REFERENCE.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Line 13 | "stats.rs is 3,345 lines" | Actual: ~4,694 lines | MEDIUM | Update line count |
| Line 179/208 | "test_stats.rs is 2,895 lines" / "~520 tests" | Actual: ~5,184 lines | MEDIUM | Update line count |

### docs/contributor/FREQUENCY_TECHNICAL_GUIDE.md

| Section | Claim | Reality | Severity | Fix |
|---------|-------|---------|----------|-----|
| Architecture diagram / Section 4 | References function `ftables()` | No bare `ftables()` exists; actual functions are `ftables_unweighted()` and `ftables_weighted_internal()` | MEDIUM | Update function name references |

## Pattern Summary

| Pattern | Count | Root Cause |
|---------|-------|------------|
| MSRV outdated (1.90 → 1.93) | 3 | MSRV bumped, contributor docs not updated |
| Feature list incomplete/wrong | 4 | Features added (`mcp`, `magika`, `color`) without updating docs |
| Command list incomplete for env vars | 2 | New commands added `--jobs`/progressbar without updating env var docs |
| Code structure changed, docs not updated | 4 | `TypedSum` refactored, `compute()` signature changed, file grew |
| Wrong file paths/names | 3 | Files renamed/moved without updating references |
| Wrong env var names | 2 | `QSV_REDIS_TTL_SECONDS` / `QSV_DISK_CACHE_TTL_SECONDS` vs actual `_SECS` / `DISKCACHE_TTL_SECS` |
| Self-contradictory values | 1 | PERFORMANCE.md says 256k AND 512k for write buffer default |
| `diff` falsely listed as stats cache user | 2 | In both README (`🪄` marker) and PERFORMANCE.md |
| Stale counts/numbers | 3 | Test count, line counts not updated as code grew |
| Wrong function names in docs | 2 | Functions renamed/refactored without updating docs |

## Human Review Queue

- [ ] README `diff` command: Verify if `🪄` marker was intentional (perhaps `diff` was planned to use stats cache but doesn't yet?)
- [ ] STATS_TECHNICAL_GUIDE lines 889-908: Verify if `stats_to_records()` parallel output pattern description is still accurate after recent refactoring
- [ ] FEATURES.md `distrib_features`: Confirm intended scope -- should it include `ui` and `magika` or just the "core" features?
- [ ] PERFORMANCE.md: Review all GitHub line-number links for drift
- [ ] STATS_QUICK_REFERENCE: Update all line counts and test counts to current values

## Documents Scanned

1. `README.md` - Main project documentation
2. `CLAUDE.md` - Claude Code instructions
3. `docs/FEATURES.md` - Feature flag documentation
4. `docs/ENVIRONMENT_VARIABLES.md` - Environment variables
5. `docs/PERFORMANCE.md` - Performance tuning guide
6. `docs/COMMAND_DEPENDENCIES.md` - Inter-command dependencies
7. `docs/contributor/PROJECT_TECHNICAL_OVERVIEW.md` - Technical overview
8. `docs/contributor/STATS_TECHNICAL_GUIDE.md` - Stats command deep dive
9. `docs/contributor/STATS_QUICK_REFERENCE.md` - Stats quick reference
10. `docs/contributor/INDEX_TECHNICAL_GUIDE.md` - Index command guide
11. `docs/contributor/FREQUENCY_TECHNICAL_GUIDE.md` - Frequency command guide
12. `docs/contributor/README.md` - Contributor docs index
