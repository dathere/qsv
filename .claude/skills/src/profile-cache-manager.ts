/**
 * Profile Cache Manager
 *
 * Caches TOON profile output from qsv frequency --toon to avoid redundant
 * processing when profiling the same unchanged file with the same options.
 *
 * Key Features:
 * - File-based JSON cache with versioning
 * - Cache key based on: file path + mtime + size + options (limit, columns, no_stats)
 * - TTL-based expiration for automatic cache invalidation
 * - LIFO eviction when cache size limit is reached
 * - Atomic writes with temp file + rename pattern
 * - Windows EPERM retry with exponential backoff
 * - Secure file permissions (0o600)
 *
 * Concurrency Note:
 * This implementation does not include file locking. It assumes typical usage
 * is single-process within the MCP server. For multi-process scenarios, the
 * atomic write pattern (temp file + rename) provides basic protection, but
 * concurrent writes may result in last-writer-wins behavior.
 */

import {
  stat,
  readFile,
  writeFile,
  rename,
  chmod,
  unlink,
} from "fs/promises";
import type { Stats } from "fs";
import { join } from "path";
import { randomUUID } from "crypto";
import { config } from "./config.js";

/**
 * Estimated overhead per cache entry in bytes (JSON metadata, keys, formatting)
 * Used for more accurate size accounting
 */
const ENTRY_OVERHEAD_BYTES = 200;

/**
 * Type guard to check if an error is a NodeJS.ErrnoException
 */
function isNodeError(error: unknown): error is NodeJS.ErrnoException {
  return error instanceof Error && "code" in error;
}

/**
 * Options for profile generation that affect cache key
 */
export interface ProfileOptions {
  limit?: number;
  columns?: string;
  no_stats?: boolean;
}

/**
 * Cache entry for a single profile
 */
interface ProfileCacheEntry {
  sourcePath: string; // Absolute file path
  sourceTimestamp: number; // mtime (ms) for change detection
  sourceSize: number; // File size for change detection
  options: ProfileOptions; // Profile generation options
  profile: string; // TOON profile output
  size: number; // Profile string byte size
  createdAt: number; // Cache entry timestamp (used for LIFO eviction)
}

/**
 * Cache format version 1
 */
interface ProfileCacheV1 {
  version: 1;
  entries: ProfileCacheEntry[];
  totalSize: number;
  lastCleanup?: number;
}

/**
 * Metrics for tracking cache performance
 */
export interface ProfileCacheMetrics {
  hits: number;
  misses: number;
  evictions: number;
  expirations: number;
  errors: {
    loadErrors: number;
    saveErrors: number;
  };
}

/**
 * Profile Cache Manager
 * Caches TOON profile output for repeated qsv_data_profile calls
 */
export class ProfileCacheManager {
  private static readonly CACHE_FILE = ".qsv-mcp-profile-cache.json";
  private static readonly DEFAULT_MAX_SIZE_MB = 10;
  private static readonly DEFAULT_TTL_MS = 60 * 60 * 1000; // 1 hour

  private maxSizeBytes: number;
  private ttlMs: number;
  private cacheFilePath: string;
  private workingDir: string;
  private metrics: ProfileCacheMetrics;

  constructor(
    workingDir: string,
    options?: {
      maxSizeMB?: number;
      ttlMs?: number;
    },
  ) {
    this.workingDir = workingDir;
    this.cacheFilePath = join(workingDir, ProfileCacheManager.CACHE_FILE);

    // Use config values or provided options
    const maxSizeMB =
      options?.maxSizeMB ??
      config.profileCacheMaxSizeMB ??
      ProfileCacheManager.DEFAULT_MAX_SIZE_MB;
    this.maxSizeBytes = maxSizeMB * 1024 * 1024;

    this.ttlMs =
      options?.ttlMs ??
      config.profileCacheTtlMs ??
      ProfileCacheManager.DEFAULT_TTL_MS;

    // Initialize metrics
    this.metrics = {
      hits: 0,
      misses: 0,
      evictions: 0,
      expirations: 0,
      errors: {
        loadErrors: 0,
        saveErrors: 0,
      },
    };
  }

  /**
   * Check if options match between cache entry and request
   */
  private optionsMatch(
    entryOptions: ProfileOptions,
    requestOptions: ProfileOptions,
  ): boolean {
    return (
      entryOptions.limit === requestOptions.limit &&
      entryOptions.columns === requestOptions.columns &&
      entryOptions.no_stats === requestOptions.no_stats
    );
  }

  /**
   * Check if source file has changed
   */
  private hasSourceChanged(
    entry: ProfileCacheEntry,
    sourceStats: Stats,
  ): boolean {
    // Check size first (fast)
    if (entry.sourceSize !== sourceStats.size) {
      return true;
    }

    // Check mtime
    if (sourceStats.mtime.getTime() !== entry.sourceTimestamp) {
      return true;
    }

    return false;
  }

  /**
   * Check if cache entry has expired based on TTL
   */
  private isExpired(entry: ProfileCacheEntry): boolean {
    const age = Date.now() - entry.createdAt;
    return age > this.ttlMs;
  }

  /**
   * Load cache from disk
   */
  private async loadCache(): Promise<ProfileCacheV1> {
    try {
      const data = await readFile(this.cacheFilePath, "utf-8");
      const cache = JSON.parse(data);

      // Validate cache structure
      if (cache.version !== 1 || !Array.isArray(cache.entries)) {
        console.error(
          "[ProfileCacheManager] Invalid cache format, creating new cache",
        );
        return { version: 1, entries: [], totalSize: 0 };
      }

      return cache as ProfileCacheV1;
    } catch (error: unknown) {
      if (isNodeError(error) && error.code === "ENOENT") {
        // Cache doesn't exist yet
        return { version: 1, entries: [], totalSize: 0 };
      }

      // JSON parse error or other corruption
      this.metrics.errors.loadErrors++;
      console.error(
        "[ProfileCacheManager] Cache load failed, creating new cache:",
        error,
      );
      return { version: 1, entries: [], totalSize: 0 };
    }
  }

  /**
   * Save cache to disk using atomic write pattern
   * Uses temp file + rename for atomicity
   * Retries on Windows EPERM errors
   * Cleans up temp files on failure
   */
  private async saveCache(cache: ProfileCacheV1): Promise<void> {
    const maxRetries = 3;
    const baseDelay = 50; // ms
    let lastError: Error | null = null;
    let tempPath: string | null = null;

    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      try {
        // Write to temporary file first
        tempPath = `${this.cacheFilePath}.tmp.${process.pid}.${randomUUID()}`;
        await writeFile(tempPath, JSON.stringify(cache, null, 2));

        // Set secure permissions
        await this.setSecurePermissions(tempPath);

        // Atomic rename
        await rename(tempPath, this.cacheFilePath);
        tempPath = null; // Successfully renamed, no cleanup needed

        // Ensure secure permissions on final file
        await this.setSecurePermissions(this.cacheFilePath);

        return; // Success
      } catch (error) {
        lastError = error as Error;

        // Clean up temp file on failure (best effort)
        if (tempPath) {
          await unlink(tempPath).catch(() => {});
          tempPath = null;
        }

        // Check if retryable Windows EPERM error
        const isEperm = (error as NodeJS.ErrnoException).code === "EPERM";
        const isWindows = process.platform === "win32";
        const shouldRetry = isEperm && isWindows && attempt < maxRetries;

        if (shouldRetry) {
          const delay = baseDelay * Math.pow(2, attempt) + Math.random() * 10;
          await new Promise((resolve) => setTimeout(resolve, delay));
          continue;
        }

        this.metrics.errors.saveErrors++;
        console.error("[ProfileCacheManager] Failed to save cache:", error);
        throw error;
      }
    }

    // Should never reach here
    this.metrics.errors.saveErrors++;
    throw lastError || new Error("Failed to save cache");
  }

  /**
   * Set secure permissions on a file (0o600)
   */
  private async setSecurePermissions(filePath: string): Promise<void> {
    try {
      await chmod(filePath, 0o600);
    } catch (error) {
      // Log but don't fail - permissions aren't critical on all platforms
      console.error(
        `[ProfileCacheManager] Warning: Failed to set secure permissions on ${filePath}:`,
        error,
      );
    }
  }

  /**
   * Recalculate total size from entries
   * Includes per-entry overhead to account for JSON metadata
   */
  private recalculateTotalSize(cache: ProfileCacheV1): number {
    return cache.entries.reduce(
      (sum, e) => sum + e.size + ENTRY_OVERHEAD_BYTES,
      0,
    );
  }

  /**
   * Get cached profile if valid (file unchanged, same options, not expired)
   * Returns null if cache miss
   * Removes stale/expired entries immediately when detected
   */
  async getCachedProfile(
    filePath: string,
    options: ProfileOptions,
  ): Promise<string | null> {
    try {
      // Get source file stats
      const sourceStats = await stat(filePath);

      // Load cache
      const cache = await this.loadCache();

      // Find matching entry
      for (let i = 0; i < cache.entries.length; i++) {
        const entry = cache.entries[i];

        // Check path match
        if (entry.sourcePath !== filePath) {
          continue;
        }

        // Check options match
        if (!this.optionsMatch(entry.options, options)) {
          continue;
        }

        // Check if source has changed - remove stale entry immediately
        if (this.hasSourceChanged(entry, sourceStats)) {
          console.error(
            `[ProfileCacheManager] Cache miss: source file changed for ${filePath}, removing stale entry`,
          );
          cache.entries.splice(i, 1);
          cache.totalSize = this.recalculateTotalSize(cache);
          this.metrics.misses++;

          // Save updated cache (best-effort)
          await this.saveCache(cache).catch((err) => {
            console.error(
              "[ProfileCacheManager] Warning: Failed to remove stale entry:",
              err,
            );
          });
          return null;
        }

        // Check if expired - remove expired entry immediately
        if (this.isExpired(entry)) {
          console.error(
            `[ProfileCacheManager] Cache miss: entry expired for ${filePath}, removing`,
          );
          cache.entries.splice(i, 1);
          cache.totalSize = this.recalculateTotalSize(cache);
          this.metrics.expirations++;
          this.metrics.misses++;

          // Save updated cache (best-effort)
          await this.saveCache(cache).catch((err) => {
            console.error(
              "[ProfileCacheManager] Warning: Failed to remove expired entry:",
              err,
            );
          });
          return null;
        }

        // Cache hit!
        console.error(`[ProfileCacheManager] Cache hit for ${filePath}`);
        this.metrics.hits++;

        // Note: We don't update lastAccessedAt on disk since we use LIFO (createdAt)
        // for eviction, not LRU. This avoids unnecessary disk I/O on the hot path.

        return entry.profile;
      }

      // No matching entry found
      console.error(`[ProfileCacheManager] Cache miss: no entry for ${filePath}`);
      this.metrics.misses++;
      return null;
    } catch (error) {
      console.error("[ProfileCacheManager] Error getting cached profile:", error);
      this.metrics.misses++;
      return null;
    }
  }

  /**
   * Cache a profile for future use
   */
  async cacheProfile(
    filePath: string,
    options: ProfileOptions,
    profile: string,
  ): Promise<void> {
    try {
      // Get source file stats
      const sourceStats = await stat(filePath);

      // Load cache
      const cache = await this.loadCache();

      // Calculate profile size in bytes
      const profileSize = Buffer.byteLength(profile, "utf-8");

      // Remove any existing entry for this file + options
      cache.entries = cache.entries.filter(
        (e) =>
          !(e.sourcePath === filePath && this.optionsMatch(e.options, options)),
      );

      // Create new entry
      const newEntry: ProfileCacheEntry = {
        sourcePath: filePath,
        sourceTimestamp: sourceStats.mtime.getTime(),
        sourceSize: sourceStats.size,
        options: {
          limit: options.limit,
          columns: options.columns,
          no_stats: options.no_stats,
        },
        profile,
        size: profileSize,
        createdAt: Date.now(),
      };

      cache.entries.push(newEntry);

      // Recalculate total size
      cache.totalSize = this.recalculateTotalSize(cache);

      // Enforce size limit (TTL expiration + LIFO eviction)
      await this.enforceSizeLimit(cache);

      // Save cache
      await this.saveCache(cache);

      console.error(
        `[ProfileCacheManager] Cached profile for ${filePath} (${profileSize} bytes)`,
      );
    } catch (error) {
      console.error("[ProfileCacheManager] Error caching profile:", error);
      // Don't throw - caching failures shouldn't break the main operation
    }
  }

  /**
   * Invalidate all cached profiles for a file
   */
  async invalidate(filePath: string): Promise<void> {
    try {
      const cache = await this.loadCache();

      const originalCount = cache.entries.length;
      cache.entries = cache.entries.filter((e) => e.sourcePath !== filePath);

      if (cache.entries.length !== originalCount) {
        cache.totalSize = this.recalculateTotalSize(cache);
        await this.saveCache(cache);
        console.error(
          `[ProfileCacheManager] Invalidated ${originalCount - cache.entries.length} entries for ${filePath}`,
        );
      }
    } catch (error) {
      console.error("[ProfileCacheManager] Error invalidating cache:", error);
    }
  }

  /**
   * Cleanup expired entries and enforce size limit
   */
  async cleanup(): Promise<void> {
    try {
      const cache = await this.loadCache();
      await this.enforceSizeLimit(cache);
      await this.saveCache(cache);
    } catch (error) {
      console.error("[ProfileCacheManager] Error during cleanup:", error);
    }
  }

  /**
   * Enforce size limit via TTL expiration and LIFO eviction
   */
  private async enforceSizeLimit(cache: ProfileCacheV1): Promise<void> {
    const now = Date.now();

    // First, remove expired entries
    const beforeExpiration = cache.entries.length;
    cache.entries = cache.entries.filter((entry) => {
      const expired = now - entry.createdAt > this.ttlMs;
      if (expired) {
        this.metrics.expirations++;
      }
      return !expired;
    });

    if (cache.entries.length !== beforeExpiration) {
      console.error(
        `[ProfileCacheManager] Expired ${beforeExpiration - cache.entries.length} entries`,
      );
    }

    // Recalculate size after expiration
    cache.totalSize = this.recalculateTotalSize(cache);

    // If still over limit, use LIFO eviction
    if (cache.totalSize > this.maxSizeBytes) {
      // Sort by createdAt (oldest first) for LIFO deletion
      const sortedEntries = [...cache.entries].sort(
        (a, b) => a.createdAt - b.createdAt,
      );

      const toDelete: ProfileCacheEntry[] = [];
      let currentSize = cache.totalSize;

      for (const entry of sortedEntries) {
        if (currentSize <= this.maxSizeBytes) {
          break;
        }
        toDelete.push(entry);
        currentSize -= entry.size + ENTRY_OVERHEAD_BYTES;
        this.metrics.evictions++;
      }

      if (toDelete.length > 0) {
        console.error(
          `[ProfileCacheManager] Evicting ${toDelete.length} oldest entries (LIFO)`,
        );

        // Remove evicted entries
        const deletePaths = new Set(
          toDelete.map((e) => `${e.sourcePath}::${JSON.stringify(e.options)}`),
        );
        cache.entries = cache.entries.filter(
          (e) => !deletePaths.has(`${e.sourcePath}::${JSON.stringify(e.options)}`),
        );
      }
    }

    // Final size recalculation
    cache.totalSize = this.recalculateTotalSize(cache);
    cache.lastCleanup = now;
  }

  /**
   * Get cache statistics
   */
  getStats(): {
    entries: number;
    sizeBytes: number;
    maxSizeBytes: number;
    ttlMs: number;
    hitRate: number;
    metrics: Readonly<ProfileCacheMetrics>;
  } {
    const totalRequests = this.metrics.hits + this.metrics.misses;
    const hitRate = totalRequests > 0 ? this.metrics.hits / totalRequests : 0;

    return {
      entries: 0, // Will be populated from cache
      sizeBytes: 0, // Will be populated from cache
      maxSizeBytes: this.maxSizeBytes,
      ttlMs: this.ttlMs,
      hitRate,
      metrics: { ...this.metrics },
    };
  }

  /**
   * Get full stats including cache data (async)
   */
  async getFullStats(): Promise<{
    entries: number;
    sizeBytes: number;
    maxSizeBytes: number;
    ttlMs: number;
    hitRate: number;
    metrics: Readonly<ProfileCacheMetrics>;
  }> {
    try {
      const cache = await this.loadCache();
      const totalRequests = this.metrics.hits + this.metrics.misses;
      const hitRate = totalRequests > 0 ? this.metrics.hits / totalRequests : 0;

      return {
        entries: cache.entries.length,
        sizeBytes: cache.totalSize,
        maxSizeBytes: this.maxSizeBytes,
        ttlMs: this.ttlMs,
        hitRate,
        metrics: { ...this.metrics },
      };
    } catch {
      return this.getStats();
    }
  }

  /**
   * Clear the entire cache
   */
  async clear(): Promise<void> {
    try {
      await unlink(this.cacheFilePath);
      console.error("[ProfileCacheManager] Cache cleared");
    } catch (error: unknown) {
      if (!isNodeError(error) || error.code !== "ENOENT") {
        console.error("[ProfileCacheManager] Error clearing cache:", error);
      }
    }
  }

  /**
   * Reset metrics counters
   */
  resetMetrics(): void {
    this.metrics = {
      hits: 0,
      misses: 0,
      evictions: 0,
      expirations: 0,
      errors: {
        loadErrors: 0,
        saveErrors: 0,
      },
    };
  }
}
