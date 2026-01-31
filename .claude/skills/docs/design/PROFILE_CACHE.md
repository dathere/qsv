# Profile Cache Implementation Handoff

## Problem Statement

The `qsv_data_profile` tool (added in PR #3402) runs `qsv frequency --toon` each time it's called. While `qsv frequency` benefits from the stats cache for performance, the actual TOON profile output is not cached by the MCP server. This means:

1. Repeated profile requests for the same unchanged file re-run the full command
2. Claude has no way to know if a valid profile already exists from a previous session
3. For large files, this adds unnecessary latency

## Current State

### What Exists
- `qsv_data_profile` tool in `src/mcp-tools.ts` (lines 1757-2099)
- `ConvertedFileManager` in `src/converted-file-manager.ts` - existing LIFO cache for Excel/JSONL→CSV conversions
- Stats cache (`.stats.csv.data.jsonl`) used by `qsv frequency` internally

### What's Missing
- No caching of TOON profile output in the MCP server
- No way to detect if a profile is still valid (file unchanged)

## Proposed Solution

Create a `ProfileCacheManager` similar to `ConvertedFileManager` that:

1. Caches TOON output keyed by: `file_path + mtime + size + limit + columns + no_stats`
2. Returns cached profile if file hasn't changed and same options used
3. Auto-expires stale entries (configurable TTL)
4. Uses LIFO eviction when cache size limit reached

## Implementation Plan

### 1. Create `src/profile-cache-manager.ts`

```typescript
interface ProfileCacheEntry {
  sourcePath: string;
  sourceHash: string;  // mtime + size combination
  options: {
    limit?: number;
    columns?: string;
    no_stats?: boolean;
  };
  profile: string;     // TOON output
  createdAt: number;
  lastAccessedAt: number;
}

interface ProfileCache {
  version: string;
  entries: Map<string, ProfileCacheEntry>;
}

export class ProfileCacheManager {
  private cacheFile: string;
  private maxCacheSizeMB: number;
  private ttlMs: number;

  constructor(workingDir: string, options?: {
    maxCacheSizeMB?: number;  // Default: 10MB
    ttlMs?: number;           // Default: 1 hour
  });

  // Get cached profile if valid
  async getCachedProfile(
    filePath: string,
    options: { limit?: number; columns?: string; no_stats?: boolean }
  ): Promise<string | null>;

  // Store profile in cache
  async cacheProfile(
    filePath: string,
    options: { limit?: number; columns?: string; no_stats?: boolean },
    profile: string
  ): Promise<void>;

  // Invalidate cache for a file (when file changes)
  async invalidate(filePath: string): Promise<void>;

  // Clean up expired entries
  async cleanup(): Promise<void>;

  // Get cache stats
  getStats(): { entries: number; sizeBytes: number; hitRate: number };
}
```

### 2. Modify `handleDataProfileCall` in `src/mcp-tools.ts`

```typescript
export async function handleDataProfileCall(
  params: Record<string, unknown>,
  filesystemProvider?: FilesystemProviderExtended,
): Promise<...> {
  // ... existing validation code ...

  // Check profile cache first
  const profileCache = new ProfileCacheManager(workingDir);
  const cachedProfile = await profileCache.getCachedProfile(inputFile, {
    limit: params.limit as number | undefined,
    columns: params.columns as string | undefined,
    no_stats: params.no_stats as boolean | undefined,
  });

  if (cachedProfile) {
    console.error(`[MCP Tools] data_profile: Using cached profile for ${inputFile}`);
    return {
      content: [{ type: "text", text: cachedProfile }],
    };
  }

  // ... existing qsv frequency --toon execution ...

  // Cache the result
  if (result.exitCode === 0) {
    await profileCache.cacheProfile(inputFile, {
      limit: params.limit as number | undefined,
      columns: params.columns as string | undefined,
      no_stats: params.no_stats as boolean | undefined,
    }, stdout);
  }

  // ... rest of function ...
}
```

### 3. Add Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `QSV_MCP_PROFILE_CACHE_SIZE_MB` | `10` | Max size for profile cache |
| `QSV_MCP_PROFILE_CACHE_TTL_MS` | `3600000` | Profile cache TTL (1 hour) |
| `QSV_MCP_PROFILE_CACHE_ENABLED` | `true` | Enable/disable profile caching |

### 4. Add Tests in `tests/profile-cache-manager.test.ts`

- Cache hit/miss scenarios
- Cache invalidation when file changes
- TTL expiration
- LIFO eviction when size limit reached
- Options matching (same file, different options = different cache entry)
- Concurrent access handling

## Key Files to Modify

| File | Changes |
|------|---------|
| `src/profile-cache-manager.ts` | New file - cache manager implementation |
| `src/mcp-tools.ts` | Integrate cache into `handleDataProfileCall` |
| `src/config.ts` | Add new environment variables |
| `tests/profile-cache-manager.test.ts` | New file - tests |
| `README-MCP.md` | Document new environment variables |
| `CLAUDE.md` | Update changelog |

## Design Considerations

### Cache Key Strategy
The cache key should be a hash of:
- Absolute file path
- File mtime (modification time)
- File size
- Options: `limit`, `columns`, `no_stats`

This ensures:
- Same file with different options = different cache entries
- File modification invalidates all cached profiles for that file

### Cache Storage
Two options:

**Option A: JSON file (like ConvertedFileManager)**
- Pros: Simple, persistent across server restarts
- Cons: Large profiles could make file unwieldy

**Option B: In-memory with optional persistence**
- Pros: Fast, no disk I/O for hits
- Cons: Lost on server restart, memory usage

**Recommendation**: Start with Option A for consistency with existing code, but limit individual profile size (e.g., 1MB max per entry).

### Cache Invalidation Triggers
1. File mtime/size change (automatic via cache key)
2. Manual invalidation via new `qsv_invalidate_profile_cache` tool (optional)
3. TTL expiration
4. LIFO eviction when size limit reached

### Thread Safety
The current implementation does not include explicit file locking:
- Assumes typical usage is single-process within the MCP server
- Relies on atomic write pattern (temp file + rename) for basic protection
- Concurrent writes may result in last-writer-wins behavior
- Windows EPERM retry with exponential backoff for file system contention

Future hardening (if higher concurrency becomes a concern):
- Add file locking similar to `ConvertedFileManager`
- Use read locks for cache reads and write locks for cache writes

## Implementation Order

1. Create `ProfileCacheManager` class with basic functionality
2. Add tests for cache manager
3. Integrate into `handleDataProfileCall`
4. Add environment variables to `config.ts`
5. Update documentation
6. Test end-to-end with MCP server

## Questions to Resolve

1. Should there be a `qsv_clear_profile_cache` tool for manual cache management?
2. Should cache be per-working-directory or global?
3. Should we log cache hit/miss statistics periodically?

## Related Files for Reference

- `src/converted-file-manager.ts` - Similar caching pattern to follow
- `src/mcp-tools.ts:1757-2099` - Current `handleDataProfileCall` implementation
- `src/config.ts` - Environment variable handling

---

**Created**: 2026-01-31
**PR Context**: #3402 (feat: add qsv_data_profile tool)
**Author**: Claude Opus 4.5
