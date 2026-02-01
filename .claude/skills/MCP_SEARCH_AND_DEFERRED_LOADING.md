# MCP BM25 Search and Deferred Loading Implementation

## Status: In Progress (Addressing Copilot Review)

**Branch:** `mcp_tool_search_tool_refactor`
**PR:** https://github.com/dathere/qsv/pull/3407
**Copilot Review:** https://github.com/dathere/qsv/pull/3407#pullrequestreview-3733776670

---

## What Was Implemented

### 1. BM25 Search Integration
- Upgraded `qsv_search_tools` from substring matching to BM25 relevance ranking
- Field-weighted search: name (3x), category (2x), description (1x), examples (0.5x)
- Text preprocessing with stemming, lowercasing

### 2. Deferred Tool Loading
- Only 7 core tools loaded initially (reduces token usage ~85%)
- Tools found via search are dynamically added to subsequent ListTools responses
- Core tools always available: `qsv_search_tools`, `qsv_config`, `qsv_set_working_dir`, `qsv_get_working_dir`, `qsv_list_files`, `qsv_pipeline`, `qsv_command`

### 3. Files Modified/Created

| File | Status | Description |
|------|--------|-------------|
| `src/bm25-search.ts` | **UPDATED** | Custom BM25 implementation (replaced AGPL version) |
| `src/wink-bm25-text-search.d.ts` | **TO DELETE** | No longer needed |
| `src/wink-nlp-utils.d.ts` | **TO DELETE** | No longer needed |
| `src/loader.ts` | Modified | BM25 integration |
| `src/mcp-server.ts` | Modified | Deferred loading, CORE_TOOLS constant |
| `src/mcp-tools.ts` | Modified | handleSearchToolsCall marks tools as loaded |
| `src/types.ts` | **NEEDS FIX** | Remove unused DeferredLoadingConfig types |
| `manifest.json` | Modified | Added defer_loading config |
| `package.json` | **NEEDS FIX** | Remove wink dependencies |
| `tests/bm25-search.test.ts` | Created | BM25 unit tests |

---

## Copilot Review Comments - Action Items

### 1. ✅ FIXED: Code Duplication in reset() (bm25-search.ts)
**Comment:** Reset duplicates constructor configuration logic.
**Action:** Extracted into `initializeState()` private method - DONE in latest bm25-search.ts

### 2. ⏳ TODO: Remove Unused Types (types.ts:196-211)
**Comment:** `DeferredLoadingConfig` and `ManifestConfig` types are not used.
**Action:** Remove lines 196-211 from `src/types.ts`:
```typescript
// DELETE THESE LINES:
export interface DeferredLoadingConfig {
  defer_loading: boolean;
}

export interface ManifestConfig {
  default_config?: DeferredLoadingConfig;
  tool_configs?: Record<string, DeferredLoadingConfig>;
}
```

### 3. ⏳ TODO: Fix Duplicate stem Declaration (wink-nlp-utils.d.ts:36)
**Comment:** `stem` is declared twice in TokensFunctions interface.
**Action:** DELETE the entire file `src/wink-nlp-utils.d.ts` - no longer needed since we removed wink dependencies.

### 4. ⏳ TODO: AGPL License Issue (package.json:40)
**Comment:** `wink-bm25-text-search` is AGPL-3.0 licensed, conflicts with MIT.
**Action:**
1. Run: `npm uninstall wink-bm25-text-search wink-nlp-utils`
2. Delete: `src/wink-bm25-text-search.d.ts`
3. Delete: `src/wink-nlp-utils.d.ts`
4. The new `src/bm25-search.ts` is already updated with a custom MIT-compatible implementation

### 5. ⏳ TODO: Add Tests for Deferred Loading (mcp-tools.ts:1709-1719)
**Comment:** Missing test coverage for marking tools as loaded.
**Action:** Add tests to `tests/mcp-tools.test.ts`:
```typescript
test("handleSearchToolsCall marks found tools as loaded", async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const loadedTools = new Set<string>();

  await handleSearchToolsCall(
    { query: "select", limit: 5 },
    loader,
    loadedTools
  );

  // Verify tools were added to the set
  assert.ok(loadedTools.size > 0, "Should mark found tools as loaded");
  // Verify naming transformation (qsv- to qsv_)
  for (const tool of loadedTools) {
    assert.ok(tool.startsWith("qsv_"), "Tool names should use underscore format");
  }
});

test("handleSearchToolsCall works when loadedTools is undefined", async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  // Should not throw when loadedTools is undefined
  const result = await handleSearchToolsCall(
    { query: "join", limit: 5 },
    loader,
    undefined
  );

  assert.ok(result.content.length > 0, "Should return results");
});
```

---

## Remaining Steps to Complete

1. **Remove AGPL dependencies:**
   ```bash
   npm uninstall wink-bm25-text-search wink-nlp-utils
   ```

2. **Delete unused type declaration files:**
   ```bash
   rm src/wink-bm25-text-search.d.ts
   rm src/wink-nlp-utils.d.ts
   ```

3. **Remove unused types from src/types.ts** (lines 196-211)

4. **Add deferred loading tests** to tests/mcp-tools.test.ts

5. **Build and test:**
   ```bash
   npm run build
   npm test
   ```

6. **Reply to Copilot review comments** on PR

7. **Amend or create new commit** with fixes

---

## Key Implementation Details

### Custom BM25 Algorithm (src/bm25-search.ts)
- No external dependencies (MIT-compatible)
- Standard BM25 scoring with k1=1.2, b=0.75
- Simple Porter-like stemmer for common English suffixes
- Field-weighted scoring for name, category, description, examples

### Deferred Loading (src/mcp-server.ts)
- `CORE_TOOLS` constant defines always-loaded tools
- `loadedTools` Set tracks tools discovered via search
- ListTools handler includes both core tools and searched tools

### Search Tool Handler (src/mcp-tools.ts)
- `handleSearchToolsCall` accepts optional `loadedTools` parameter
- Found tools are added to the set with `qsv_` prefix format

---

## Test Commands

```bash
# Build
npm run build

# Run all tests
npm test

# Run specific test file
node --test dist/tests/bm25-search.test.js

# Run only BM25 tests
npm run build:test && node --test dist/tests/bm25-search.test.js
```

---

## Version Info

- **Current Version:** 15.3.0
- **Previous Version:** 15.2.0
- **Node.js Required:** >=18.0.0

---

## Files Reference

### Core Implementation
- `src/bm25-search.ts` - Custom BM25 search implementation
- `src/loader.ts` - SkillLoader with BM25 integration
- `src/mcp-server.ts` - Server with deferred loading
- `src/mcp-tools.ts` - Tool handlers

### Tests
- `tests/bm25-search.test.ts` - BM25 unit tests (9 tests)

### Configuration
- `manifest.json` - MCP manifest with defer_loading config
- `package.json` - Dependencies and version

### Documentation
- `CHANGELOG.md` - Release notes for 15.3.0
- `CLAUDE.md` - Updated What's New section
