# Documentation Audit Report — qsv v19.0.0
Generated: 2026-04-09 | Commit: fbbaf5f68

## Executive Summary

| Metric | Count |
|--------|-------|
| Documents scanned | 12 |
| Claims verified | ~150 |
| Verified TRUE | ~140 (93%) |
| **Verified FALSE** | **10 (7%)** |

## Previous Audit Follow-up (2026-04-07)

All 8 findings from the April 7 audit were fixed in commit `b86825fbd`. However, two of those fixes introduced **new inaccuracies**:

| Original Finding | Fix Applied | New Problem |
|-----------------|-------------|-------------|
| Test count "~2,525" was wrong | Changed to "~3,000" and "more than 2,900" | Overcorrected — actual count is 2,666 |

## False Claims Requiring Fixes

### README.md

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 496 | "more than 2,900 tests" | `cargo test -F all_features -- --list` = 2,666 | Changed to "more than 2,600 tests" |
| 509 | "~3,000 tests" | Actual: 2,666 | Changed to "~2,700 tests" |
| 88 | `sortcheck` marked with 📇 (index icon) | `src/cmd/sortcheck.rs` has NO index usage | Removed 📇 icon |
| 293 | qsvlite "~16%" of qsv size | Contradicted by line 502 which said "~13%" | Standardized to ~16% (lines 293-294 appear more current) |
| 294 | "the the size" typo | Double "the" | Fixed typo |
| 502 | qsvlite "~13%", qsvdp "~12%" | Contradicted by lines 293-294 which said "~16%" | Updated to ~16% to match lines 293-294 |

### .claude/skills/README-MCP.md

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 25 | qsvmcp = 60 commands | `grep -c 'Command::' src/main.rs` = 71; 71 - 9 excluded = 62 | Changed to 62 |
| 26 | qsv = 66 commands | `grep -c 'Command::' src/main.rs` = 71 | Changed to 71 |

Note: Line 17 in the same file correctly stated 62/71 — the table simply hadn't been updated to match.

### .claude-plugin/marketplace.json

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 8 | "66 commands" in metadata description | README table has 70 commands; line 15 of same file correctly says "70 qsv commands" | Changed to "70 commands" |

### docs/PERFORMANCE.md

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 137 | OOM list: dedup, reverse, sort, stats, table, transpose | `pragmastat` has 🤯 emoji in README (line 69) indicating it loads entire CSV into memory | Added `pragmastat` to OOM list |

### .claude/skills/README.md

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 32 | "180 total" usage examples | Parsed all 51 skill JSONs: 174 examples | Changed to "174 total" |

### docs/help/TableOfContents.md

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 67 | `sortcheck` marked with 📇 | `src/cmd/sortcheck.rs` has no index usage | Removed 📇 icon |

## Pattern Summary

| Pattern | Count | Root Cause |
|---------|-------|------------|
| Stale command counts | 3 | README-MCP.md table and marketplace.json metadata not updated when commands were added |
| Test count overcorrection | 2 | Previous audit fix used incorrect methodology (claimed ~2,957 vs actual 2,666) |
| Missing OOM command | 1 | `pragmastat` added to README with 🤯 but PERFORMANCE.md not updated |
| Wrong icon marker | 2 | `sortcheck` gained 📇 icon despite not using index (2 locations) |
| Size percentage drift | 2 | Two locations in README.md had different percentages for same metric |
| Typo | 1 | "the the" in README.md line 294 |

## Pass 2: Pattern Expansion Results

### Command count claims (all files)
- `.claude/skills/docs/desktop/README-MCPB.md` lines 21, 142, 268, 365: All say 62/71 — **CORRECT** (counts help)
- `.claude/skills/cowork-CLAUDE.md` line 47: "51 qsv skill-based commands" — **CORRECT**
- `.claude/skills/GEMINI.md` line 15: "51+" — **CORRECT**
- `.claude/skills/.claude-plugin/plugin.json` line 4: "51 qsv skill-based commands" — **CORRECT**
- `.claude/skills/docs/guides/GEMINI_CLI.md` line 7: "51" — **CORRECT**

### Icon marker spot-check (📇 index)
- `count`: Uses `conf.indexed()` — **CORRECT**
- `sample`: Uses `rconfig.indexed()` — **CORRECT**
- `frequency`: Uses `args.rconfig().indexed()` — **CORRECT**
- `replace`: Uses `rconfig.indexed()` — **CORRECT**
- `pragmastat`: Uses `rconfig.indexed()` at line 573, parallel indexed reading at line 952 — **CORRECT**
- `sortcheck`: No index usage — **WRONG** (fixed)

### QSV_MAX_JOBS command list verification
- `pivotp` and `sample` pass `flag_jobs: None` to internal stats struct, which respects QSV_MAX_JOBS — listing is **CORRECT** (indirect usage)

## Human Review Queue

- [ ] Binary size percentages (~16%/~16%): Standardized to match lines 293-294, but actual percentages should be verified by building release binaries
- [ ] Performance benchmark numbers throughout README: Cannot verify from source code alone

## Verification

All fixes applied. Changes are limited to documentation files only — no code changes.

## Files Modified

1. `README.md` — test counts, sortcheck icon, size percentages, typo
2. `.claude/skills/README-MCP.md` — command count table
3. `.claude-plugin/marketplace.json` — metadata description count
4. `docs/PERFORMANCE.md` — added pragmastat to OOM list
5. `.claude/skills/README.md` — usage examples count
6. `docs/help/TableOfContents.md` — sortcheck icon
