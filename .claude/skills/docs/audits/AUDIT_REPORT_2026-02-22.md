# Documentation Audit Report

Generated: 2026-02-22 | Commit: 56fd6103d | **Status: All false claims corrected**

## Executive Summary

| Metric | Count |
|--------|-------|
| Documents scanned | 11 |
| Claims verified | ~220+ |
| Verified TRUE | ~180 (82%) |
| **Verified FALSE** | **~40 (18%)** |
| Accuracy by doc | 56%–100% |

### Overall Assessment

The **main qsv CLAUDE.md** (repo root) is **100% accurate** — all 36+ claims verified true. The **MCP server CLAUDE.md** is ~87% accurate with 10 false claims. The **guides and reference docs** range from 56% to 100% accuracy, with **SKILLS_API.md** (59%) and **DESKTOP_EXTENSION.md** (56%) being the most outdated.

## False Claims Requiring Fixes

### `.claude/skills/CLAUDE.md` (MCP Server CLAUDE.md)

| # | Location | Claim | Reality | Fix |
|---|----------|-------|---------|-----|
| 1 | Line ~557, ~736 | MCP SDK `^1.25.2` | Actual: `^1.26.0` per package.json | Update both occurrences |
| 2 | Line ~142 | `ALWAYS_FILE_COMMANDS`: 33 commands | Actual: **34** commands (includes `searchset`) | Update count |
| 3 | Line ~145 | `AUTO_INDEX_THRESHOLD`: 10MB | Constant is named **`AUTO_INDEX_SIZE_MB`**, not `AUTO_INDEX_THRESHOLD` | Fix constant name |
| 4 | Lines ~78-111 | Directory structure shows 12 src/ files | Actually **16** files. Missing: `index.ts`, `duckdb.ts`, `wink-bm25-text-search.d.ts`, `wink-nlp-utils.d.ts` | Add missing files |
| 5 | Line ~184 | `VERSION` "(not package.json)" | `version.ts` reads from package.json at runtime — parenthetical is misleading | Clarify or remove parenthetical |
| 6 | Line ~123 | "~85% token reduction" | MEMORY.md says ~80%; figure unverifiable | Align or remove specific percentage |
| 7 | Line ~199 | "Automatic cleanup with configurable TTL" | Cleanup is **LIFO/size-based** (`QSV_MCP_CONVERTED_LIFO_SIZE_GB`), not TTL-based | Change "TTL" to "size limit" |
| 8 | — | `QSV_MCP_GITHUB_REPO` listed under config.ts section | Actually in **update-checker.ts**, not config.ts | Move to correct section |
| 9 | — | Missing env vars | `QSV_MCP_SERVER_INSTRUCTIONS` and `QSV_MCP_PLUGIN_MODE` exist in config.ts but undocumented | Add to env var list |
| 10 | — | `test:coverage` npm script undocumented | Exists in package.json but not mentioned in CLAUDE.md | Add to testing section |

### `docs/reference/SKILLS_API.md` — 9 false claims (59% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | `getStats()` shows `total: 65` | Actual: **56** skills | Update example output |
| 2 | `QsvSkill` has `test_file?` field | Field was **removed** (CHANGELOG 14.1.0) | Remove from interface |
| 3 | `SkillParams.args` typed `Record<string, any>` | Actual: `Record<string, unknown>` | Update type |
| 4 | Categories include "analysis" | Renamed to **"documentation"** (CHANGELOG 16.1.0) | Update category name |
| 5 | "65 .json files" in troubleshooting | Actual: **56** | Update count |
| 6 | Link `../../docs/AGENT_SKILLS_DESIGN.md` | Moved to `../design/AGENT_SKILLS_DESIGN.md` | Fix relative path |
| 7 | Link `../../docs/AGENT_SKILLS_INTEGRATION.md` | Moved to `../design/AGENT_SKILLS_INTEGRATION.md` | Fix relative path |
| 8 | Link `../../docs/AGENT_SKILLS_POC_SUMMARY.md` | Moved to `../design/AGENT_SKILLS_POC_SUMMARY.md` | Fix relative path |
| 9 | `CommandSpec` has `binary` field | Field was **removed** (CHANGELOG 16.1.0) | Remove from interface |

### `docs/guides/DESKTOP_EXTENSION.md` — 7 false claims (56% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | Minimum qsv "0.133.0 (or later)" | Minimum is **16.0.0** per config.ts | Update version |
| 2 | "25 MCP tools" | Manifest lists **20** tools | Update count |
| 3 | "66 qsv commands packaged as MCP tools" | **56** skill JSON files | Update count |
| 4 | Default timeout "5 minutes" (appears twice) | Default is **10 minutes** (600000ms) | Update both occurrences |
| 5 | Default allowed dirs `~/Downloads:~/Documents` | Default is **empty array** | Correct default |
| 6 | `to parquet` subcommand reference | Removed in **16.0.0** — use `qsv_to_parquet` MCP tool or `sqlp` | Remove reference |
| 7 | Changelog section version "13.0.0" | Current: **16.1.2** | Update or remove |

### `docs/desktop/README-MCPB.md` — 6 false claims (67% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | "26 tools" in architecture | Manifest lists **20** tools | Update count |
| 2 | `execFileSync` for security | Changed to **`spawn`** (streaming) in 15.0.0 | Update to `spawn` |
| 3 | Operation timeout default "120s" | Default is **10 minutes** (600000ms) | Update value |
| 4 | `QSV_MCP_MAX_CONCURRENT_OPERATIONS` default 10 | Changed to **1** in 16.0.0 | Update default |
| 5 | Profile cache env vars listed | `QSV_MCP_PROFILE_CACHE_*` vars **removed** in 16.0.0 | Remove section |
| 6 | "67 commands" for qsv binary | Approximately correct but inconsistent with "56 skills" elsewhere | Clarify distinction |

### `docs/reference/UPDATE_SYSTEM.md` — 3 false claims (75% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | "No new npm dependencies" | `wink-bm25-text-search` and `wink-nlp-utils` added in 15.3.0 | Update dependency list |
| 2 | Document version "13.0.0" | Current: **16.1.2** | Update version |
| 3 | "Rust required only for auto-regeneration" | Auto-regen uses `qsv --update-mcp-skills` (prebuilt binary), not Rust toolchain | Clarify |

### `docs/guides/CLAUDE_CODE.md` — 2 false claims (89% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | qsv version "13.0.0 (or later)" | Minimum is **16.0.0** | Update version |
| 2 | "5 minute timeout" for minimal config | Default is **10 minutes** | Update value |

### `docs/reference/AUTO_UPDATE.md` — 1 false claim (93% accuracy)

| # | Claim | Reality | Fix |
|---|-------|---------|-----|
| 1 | Document version "13.0.0" | Current: **16.1.2** | Update footer version |

### Fully Accurate Documents (100%)

- **`CLAUDE.md`** (repo root) — 36+ claims, all verified true
- **`docs/guides/QUICK_START.md`** — 12 claims, all verified true
- **`CHANGELOG.md`** — 5 claims, all verified true

## Pattern Summary

| Pattern | Count | Root Cause |
|---------|-------|------------|
| Outdated version numbers (13.0.0, 0.133.0) | 5 | Docs not updated after version bumps |
| Wrong tool/command counts (25, 26, 33, 65, 66) | 7 | Counts changed as tools added/removed, docs stale |
| Wrong default values (5min, 120s, 10 concurrent) | 6 | Defaults changed in 15.0.0/16.0.0, docs not updated |
| Removed features still documented | 4 | Profile cache, `to parquet`, `test_file`, `binary` field removed |
| Renamed/moved items | 4 | "analysis"→"documentation" category, docs reorganized to subdirs |
| Missing documentation for new features | 3 | New env vars, npm scripts, src files added without doc updates |
| Wrong constant/function names | 2 | `AUTO_INDEX_THRESHOLD` vs `AUTO_INDEX_SIZE_MB` |

## Priority Recommendations

### P0 — Fix immediately (user-facing, causes confusion)
1. **SKILLS_API.md**: Complete rewrite needed — 9 false claims, 59% accuracy
2. **DESKTOP_EXTENSION.md**: Major update needed — 7 false claims, 56% accuracy
3. **README-MCPB.md**: Significant update needed — 6 false claims, 67% accuracy

### P1 — Fix soon (developer-facing)
4. **MCP Server CLAUDE.md**: Update MCP SDK version, constant names, directory structure, env vars
5. **UPDATE_SYSTEM.md**: Update version, dependency claims
6. **CLAUDE_CODE.md**: Update minimum version and timeout default

### P2 — Minor fixes
7. **AUTO_UPDATE.md**: Update footer version number

## Human Review Queue

- [ ] Verify `DESKTOP_EXTENSION.md` tool count — is it 20 (manifest) or something else after deferred loading?
- [ ] Confirm whether `QSV_MCP_SERVER_INSTRUCTIONS` and `QSV_MCP_PLUGIN_MODE` should be documented publicly
- [x] Verify stats cache filename: `.stats.csv.data.jsonl` is canonical (confirmed in `stats.rs:1474`, `src/cmd/stats.rs`, and `src/util.rs`) — both CLAUDE.md files now consistent
- [ ] Check if "~80% token reduction" or "~85% token reduction" is correct for deferred loading
