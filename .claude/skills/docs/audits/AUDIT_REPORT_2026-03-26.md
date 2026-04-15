# Documentation Audit Report
Generated: 2026-03-26 | Commit: ffb2485d5

## Executive Summary

| Metric | Count |
|--------|-------|
| Documents scanned | 12 |
| Claims verified | ~95 |
| Verified TRUE | ~77 (81%) |
| **Verified FALSE** | **18 (19%)** |
| Files fixed | 13 |
| Copilot review comments | 9 (all applied) |

## False Claims Requiring Fixes

### README.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| ~454 | `Version: 18.0.2` | package.json is `18.0.4` | Updated to 18.0.4 |
| ~123 | `npm run test-pipeline` | Script does not exist in package.json | Removed reference |

### README-MCP.md (8 locations)

**Note:** `CORE_TOOLS` array in `mcp-server.ts` has 11 entries, but `listTools` only exposes `qsv_browse_directory` when MCP Apps are enabled (line 606). Users see 10 tools by default, 11 with Apps. Docs now say "10 core tools (+1 app-only)".

| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 9 | "Only 10 core tools loaded initially" | Correct for default (10 exposed); internal array has 11 | Clarified: 10 + 1 app-only |
| 185 | "### 10 Core Tools (Always Loaded)" | 10 always exposed, browse_directory conditional | Kept as 10, added app-only note |
| 188-200 | Core tools table missing browse_directory | Conditionally exposed, not always loaded | Listed separately as app-only |
| 236-250 | Deferred loading section says "10 core tools" | 10 exposed by default | Kept as 10 with app-only note |
| 265-266 | EXPOSE_ALL_TOOLS description says "10" | 10 by default | Updated with app-only qualifier |
| 177 | Env var table says "10 core tools" | 10 by default | Updated with app-only qualifier |
| 394 | Architecture diagram: "10 Core Tools" | 10 by default | Updated |
| 611 | Footer: "10 core tools initially" | 10 by default | Updated with app-only note |

### CLAUDE.md (skills)
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 70 | `.claude-plugin/plugin.json declares the plugin` | Root has `.claude-plugin/marketplace.json`; `plugin.json` is at `.claude/skills/.claude-plugin/` | Corrected reference |
| 18 | "a `case` in `handleToolCall()`" | Core tools use `toolDispatchMap` in `mcp-server.ts` | Updated guidance |

### marketplace.json (repo root)
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 8, 15 | "68 commands" | `qsv --list` returns 66 commands | Updated to 66 |

### GEMINI.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 39 | "10 core tools" | 10 exposed + 1 app-only | Updated to "10 + 1 app-only" |

### docs/guides/GEMINI_CLI.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 94 | "10 core tools" list missing browse_directory | 11 tools including browse_directory | Updated |

### docs/guides/FILESYSTEM_USAGE.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 134-135 | "10 core tools" in QSV_MCP_EXPOSE_ALL_TOOLS docs | 11 | Updated |

### docs/reference/AUTO_UPDATE.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 323 | `Version: 18.0.2` | 18.0.4 | Updated |

### docs/reference/UPDATE_SYSTEM.md
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 245 | `Version: 18.0.2` | 18.0.4 | Updated |

### tests/deferred-loading.test.ts
| Line | Claim | Reality | Fix |
|------|-------|---------|-----|
| 18-32 | Local CORE_TOOLS has 10 entries, asserts `=== 10` | Source has 11 (includes browse_directory) | Added browse_directory, updated assertion |
| 90 | Comment says "9 core tools" and `// 8` | 11 core tools | Updated comments |
| 82 | Asserts `>= 0.8` (80% reduction) | 11/51 = 78% reduction | Adjusted to `>= 0.75` |

## Pattern Summary

| Pattern | Count | Root Cause |
|---------|-------|------------|
| Stale "10 core tools" count | 8 locations | `qsv_browse_directory` added to CORE_TOOLS without updating docs |
| Stale version "18.0.2" | 3 locations | Docs not updated after 18.0.3/18.0.4 releases |
| Stale command count "68" | 2 locations | marketplace.json not updated after command additions |
| Stale test assertions | 1 file | Test has local copy of CORE_TOOLS not synced with source |

**Recurring theme**: Same root cause as 2026-03-18 and 2026-03-22 audits — numeric counts in documentation not updated when code changes.

## Verified TRUE Claims

- All 5 operational limit constants match source code
- 51 skill JSON files confirmed
- 13 COMMON_COMMANDS entries confirmed
- COMMAND_GUIDANCE field names (whenToUse, commonPattern, errorPrevention) correct
- buildSkillExecParams and resolveParamAliases exist and behave as documented
- All file/path references in project structure valid (16/17, excluding test-pipeline)
- All docs/, scripts/, and src/ files exist as documented
- package.json version is 18.0.4
- All npm scripts exist (build, test, test:watch, test:coverage, test:examples, mcp:start, mcp:install, mcpb:package, plugin:package)

## Human Review Queue

- [ ] Verify `qsv_browse_directory` conditional exposure logic matches updated docs (conditionally exposed vs always in CORE_TOOLS array)
- [ ] Consider exporting CORE_TOOLS from mcp-server.ts so tests import from source instead of maintaining a local copy
