# Documentation Audit Report

Generated: 2026-03-02 | Commit: 72641e4ca | Auditor: Claude Opus 4.6 | **Status: ALL FIXES APPLIED**

## Executive Summary

| Metric | Count |
|--------|-------|
| Documents scanned | 14 |
| Claims verified | ~200+ |
| **Verified FALSE** | **~39** |
| Documents with no issues | 3 |

## False Claims Requiring Fixes

### CLAUDE.md (Main Development Guide)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| `ALWAYS_FILE_COMMANDS` | 34 commands | Actually **32** entries (lines 65-96 of `mcp-tools.ts`) | Change to 32 |
| `NON_TABULAR_COMMANDS` | 9 commands | Actually **8** entries (4 METADATA + tojsonl, template, schema, validate) | Change to 8 |
| Version Synchronization | "`version.ts` reads qsv binary version at runtime" | `version.ts` reads the **MCP server version** from `package.json`, not the qsv binary version. Binary version detection is in `update-checker.ts` | Reword to clarify |
| Adding a New MCP Tool | "Add to `TOOL_DEFINITIONS` array" | No `TOOL_DEFINITIONS` array exists. Tools are defined via individual `create*Definition()` functions and dynamically from skill JSON | Rewrite the example section |
| converted-file-manager.ts | "UUID-derived temp file names (16-char random hex)" | Uses full `randomUUID()` (36-char UUIDs). The 16-char substring is only for content hash truncation | Fix description |

### README-MCP.md

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Throughout (lines 1, 173, 198, 233, 461, 524, 569) | "56 skills/commands/tools" | Actually **51** skill JSON files | Change all to 51 |
| Lines 9, 154, 162, 208, 235, 569 | "9 core tools" | `CORE_TOOLS` has **10** entries (includes `qsv_log`) | Change to 10 |
| Line 145 | `QSV_MCP_MAX_CONCURRENT_OPERATIONS` default is `10` | Default is **3** (plugin mode) or **1** (otherwise), never 10 | Fix default value |
| Line 9 | "~85% token reduction" | CLAUDE.md says **~80%** | Change to ~80% |
| Lines 411, 421 | `--update-mcp-skill` (singular) | Actual flag is `--update-mcp-skills` (plural) | Add missing 's' |
| Lines 472-487 | Project structure listing | Missing `duckdb.ts` | Add `duckdb.ts` |
| Line 491 | Scripts listing | Missing `cowork-setup.js` and `run-tests.js` | Add missing scripts |

### CLAUDE_CODE.md (Guide)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Lines 89, 95, 381, 747, 805 | Config at `~/.config/claude-code/mcp_settings.json` | Claude Code uses `~/.claude/settings.json` or `.mcp.json` | Update paths |
| Lines 399, 805 | Logs at `~/.config/claude-code/logs/` | Actual path is `~/.claude/logs/` or similar | Update path |
| Line 823 | Link to `./examples/` | No `examples/` dir in `docs/guides/` | Fix to `../../examples/` |

### DESKTOP_EXTENSION.md (Guide)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Line 167 | Tool `qsv_get_file_preview` | **Does not exist** in codebase | Remove or replace with actual tool |
| Line 202 | `get_file_preview` in filesystem tools | **Does not exist** | Remove from list |
| Line 197 | "Aggregation: groupby, pivot" | No `groupby` or `pivot` commands. Commands are `pivotp`, `pragmastat` | Fix command names |
| Lines 204, 466 | Links to `./README.md` | No README.md in `docs/guides/` | Fix to `../../README.md` |

### GEMINI_CLI.md (Guide)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Lines 7, 12 | "56 commands" | Actually **51** | Change to 51 |
| Lines 9, 94 | "9 Core Tools" | Actually **10** (missing `qsv_log`) | Change to 10, add `qsv_log` |

### FILESYSTEM_USAGE.md (Guide)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Lines 134-135 | "7 core tools" | Actually **10** | Change to 10 |
| Line 133 | "56+ tools" | Actually **51** | Change to 51 |
| Lines 343-348 | MCP resources browser exposes CSV files | `registerResourceHandlers` returns empty array `resources: []` | Remove or mark as not yet implemented |

### SKILLS_API.md (Reference)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Lines 96, 321 | `getStats()` shows `total: 56` | Actually **51** | Change to 51 |
| Line 107 | `new SkillExecutor('qsv')` (one param) | Constructor requires two params: `(binPath, workingDir)` | Fix constructor example |

### README-MCPB.md (Desktop Extension)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Line 23 | Download URL tag `16.1.0` with file `16.1.2` | Tag/file version mismatch | Align tag and file version |
| Line 257 | "Tool definitions (20 tools)" | Not accurate for any configuration | Fix to "10 core + 51 skill-based" |
| Line 308 | `$USER` / `${USER}` template variable | `expandTemplateVars` does NOT support `USER` | Remove `$USER` from docs |
| Line 220 | Link `./README-MCP.md` | Wrong relative path from `docs/desktop/` | Fix to `../../README-MCP.md` |

### CI.md (Reference)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Lines 19-20 | "Node.js 22 (current)", "3 test combinations" | CI uses Node.js `[20, 22, 24]` = **9 combinations** (3 OS x 3 Node) | Update versions and count |
| Line 34 | "28 tests" | Test count has changed since this was written | Verify current count or use generic phrasing |

### AUTO_UPDATE.md (Reference)

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| Line 208 | `"mcpServerVersion": "13.0.0"` in example | Current version is **16.1.2** | Update example version |

### Plugin Configuration

| Location | Claim | Reality | Fix |
|----------|-------|---------|-----|
| `.claude-plugin/plugin.json` | `"version": "16.1.1"` | `package.json` version is **16.1.2** | Sync to 16.1.2 |

## Gap Detection: Undocumented Code

### Missing from CLAUDE.md "Key Exports" Sections

| Module | Documented | Actual | Gap |
|--------|-----------|--------|-----|
| `mcp-tools.ts` | 4 exports | 29 exports | 25 undocumented (process management, tool creators, handlers) |
| `utils.ts` | 6 exports | 10 exports | 4 undocumented (`getErrorMessage`, `isNodeError`, `isReservedCachePath`, `reservedCachePathError`) |
| `config.ts` | ~3 exports | 13 exports | 10 undocumented (parser functions, validators) |
| `duckdb.ts` | 0 (no section) | 10 exports | Entire module undocumented in Key Exports |

### Other Gaps

| Item | Detail |
|------|--------|
| `MAX_STDERR_SIZE` in 3 files | Documented only for `executor.ts` (50MB). Also exists in `mcp-tools.ts` (1MB) and `duckdb.ts` (1MB) |
| `SessionStart` hook | `plugin.json` has a `SessionStart` hook running `cowork-setup.js`, not documented in CLAUDE.md plugin structure section |
| `CLAUDE_PLUGIN_ROOT` env var | Mentioned in prose but NOT listed in the "Key Environment Variables" section |
| 8 test files | Not referenced in docs (executor-timeout, tsv-output, tool-filtering, concurrency-slots, qsv-integration, deferred-loading, mcp-server, version) |

## Pattern Summary

| Pattern | Count | Root Cause |
|---------|-------|------------|
| Wrong skill/tool count "56" → 51 | 8 occurrences across 5 docs | Count not updated after skill consolidation |
| Wrong core tools count "9" or "7" → 10 | 6 occurrences across 4 docs | `qsv_log` addition not propagated to docs |
| Non-existent tool references | 2 occurrences | `qsv_get_file_preview` removed but docs not updated |
| Broken relative links | 4 occurrences across 3 docs | Links use `./` relative to wrong directory |
| Version drift | 3 occurrences | Version bumps not propagated to all files |

## Documents with No Issues

- `docs/guides/QUICK_START.md`
- `docs/guides/MACOS-QUICK_START.md`
- `docs/reference/UPDATE_SYSTEM.md`

## Recommendations

1. **Immediate**: Fix the pervasive "56" → "51" count across all docs
2. **Immediate**: Fix "9"/"7" → "10" core tools count
3. **Immediate**: Remove `qsv_get_file_preview` references (ghost tool)
4. **Immediate**: Sync `plugin.json` version to 16.1.2
5. **High**: Fix broken relative links in DESKTOP_EXTENSION.md, README-MCPB.md
6. **High**: Fix `--update-mcp-skill` → `--update-mcp-skills` typo in README-MCP.md
7. **Medium**: Update CLAUDE.md export documentation for mcp-tools.ts, utils.ts, config.ts, duckdb.ts
8. **Medium**: Rewrite "Adding a New MCP Tool" section to match actual patterns (no `TOOL_DEFINITIONS` array)
9. **Low**: Update example version numbers in AUTO_UPDATE.md
10. **Low**: Consider documenting the additional `MAX_STDERR_SIZE` constants in mcp-tools.ts and duckdb.ts
