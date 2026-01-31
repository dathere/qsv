/**
 * Tests for ProfileCacheManager
 *
 * Tests cache hit/miss scenarios, TTL expiration, LIFO eviction,
 * options matching, and change detection.
 */

import { test, describe, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { mkdtemp, rm, writeFile, utimes, stat } from "fs/promises";
import { join } from "path";
import { tmpdir } from "os";
import { ProfileCacheManager, type ProfileOptions } from "../src/profile-cache-manager.js";

// Test directory for temporary files
let testDir: string;

async function createTestDir(): Promise<string> {
  return await mkdtemp(join(tmpdir(), "profile-cache-test-"));
}

async function cleanupTestDir(dir: string): Promise<void> {
  try {
    await rm(dir, { recursive: true, force: true });
  } catch {
    // Ignore cleanup errors
  }
}

async function createTestCsv(dir: string, name: string, content: string): Promise<string> {
  const filePath = join(dir, name);
  await writeFile(filePath, content);
  return filePath;
}

describe("ProfileCacheManager", () => {
  beforeEach(async () => {
    testDir = await createTestDir();
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("should return null for cache miss (file not in cache)", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    const result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, null);

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.metrics.misses, 1);
    assert.strictEqual(stats.metrics.hits, 0);
  });

  test("should return cached profile for cache hit", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const mockProfile = "TOON profile output here";

    // Cache the profile
    await manager.cacheProfile(csvPath, {}, mockProfile);

    // Get cached profile
    const result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, mockProfile);

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.metrics.hits, 1);
    assert.strictEqual(stats.entries, 1);
  });

  test("should return null when file has changed (mtime)", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const mockProfile = "TOON profile output here";

    // Cache the profile
    await manager.cacheProfile(csvPath, {}, mockProfile);

    // Modify the file (change mtime)
    const now = new Date();
    const newMtime = new Date(now.getTime() + 10000); // 10 seconds in future
    await utimes(csvPath, newMtime, newMtime);

    // Get cached profile - should be null due to mtime change
    const result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, null);
  });

  test("should return null when file has changed (size)", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const mockProfile = "TOON profile output here";

    // Cache the profile
    await manager.cacheProfile(csvPath, {}, mockProfile);

    // Get current stats to preserve mtime
    const stats = await stat(csvPath);

    // Modify the file content (changes size)
    await writeFile(csvPath, "a,b,c\n1,2,3\n4,5,6\n7,8,9\n");

    // Restore original mtime so only size differs
    await utimes(csvPath, stats.atime, stats.mtime);

    // Get cached profile - should be null due to size change
    const result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, null);
  });

  test("should return different cache entries for different options", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const profile1 = "Profile with limit 5";
    const profile2 = "Profile with limit 10";

    const options1: ProfileOptions = { limit: 5 };
    const options2: ProfileOptions = { limit: 10 };

    // Cache profiles with different options
    await manager.cacheProfile(csvPath, options1, profile1);
    await manager.cacheProfile(csvPath, options2, profile2);

    // Verify each returns correct profile
    const result1 = await manager.getCachedProfile(csvPath, options1);
    const result2 = await manager.getCachedProfile(csvPath, options2);

    assert.strictEqual(result1, profile1);
    assert.strictEqual(result2, profile2);

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 2);
  });

  test("should handle columns option correctly", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const profile1 = "Profile for columns a,b";
    const profile2 = "Profile for columns c";

    const options1: ProfileOptions = { columns: "a,b" };
    const options2: ProfileOptions = { columns: "c" };

    await manager.cacheProfile(csvPath, options1, profile1);
    await manager.cacheProfile(csvPath, options2, profile2);

    const result1 = await manager.getCachedProfile(csvPath, options1);
    const result2 = await manager.getCachedProfile(csvPath, options2);

    assert.strictEqual(result1, profile1);
    assert.strictEqual(result2, profile2);
  });

  test("should handle no_stats option correctly", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const profileWithStats = "Profile with stats";
    const profileNoStats = "Profile without stats";

    const options1: ProfileOptions = { no_stats: false };
    const options2: ProfileOptions = { no_stats: true };

    await manager.cacheProfile(csvPath, options1, profileWithStats);
    await manager.cacheProfile(csvPath, options2, profileNoStats);

    const result1 = await manager.getCachedProfile(csvPath, options1);
    const result2 = await manager.getCachedProfile(csvPath, options2);

    assert.strictEqual(result1, profileWithStats);
    assert.strictEqual(result2, profileNoStats);
  });

  test("should expire entries after TTL", async () => {
    // Use very short TTL for testing
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 100, // 100ms TTL
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");
    const mockProfile = "TOON profile output here";

    // Cache the profile
    await manager.cacheProfile(csvPath, {}, mockProfile);

    // Verify it's cached
    let result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, mockProfile);

    // Wait for TTL to expire
    await new Promise((resolve) => setTimeout(resolve, 150));

    // Should now be expired
    result = await manager.getCachedProfile(csvPath, {});
    assert.strictEqual(result, null);

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.metrics.expirations, 1);
  });

  test("should evict oldest entries when size limit exceeded (LIFO)", async () => {
    // Use very small size limit
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 0.001, // ~1KB limit
      ttlMs: 60 * 60 * 1000,
    });

    // Create test files
    const csvPath1 = await createTestCsv(testDir, "test1.csv", "a,b,c\n1,2,3\n");
    const csvPath2 = await createTestCsv(testDir, "test2.csv", "d,e,f\n4,5,6\n");
    const csvPath3 = await createTestCsv(testDir, "test3.csv", "g,h,i\n7,8,9\n");

    // Create profiles that will exceed the limit
    const largeProfile = "X".repeat(500); // 500 bytes each

    // Cache profiles in order (oldest first)
    await manager.cacheProfile(csvPath1, {}, largeProfile);
    await new Promise((resolve) => setTimeout(resolve, 10)); // Small delay to ensure order
    await manager.cacheProfile(csvPath2, {}, largeProfile);
    await new Promise((resolve) => setTimeout(resolve, 10));
    await manager.cacheProfile(csvPath3, {}, largeProfile);

    // The oldest entry (csvPath1) should have been evicted
    const result1 = await manager.getCachedProfile(csvPath1, {});
    const result3 = await manager.getCachedProfile(csvPath3, {});

    // Oldest should be evicted, newest should remain
    assert.strictEqual(result1, null, "Oldest entry should be evicted");
    assert.strictEqual(result3, largeProfile, "Newest entry should remain");

    const stats = await manager.getFullStats();
    assert.ok(stats.metrics.evictions > 0, "Should have evictions");
  });

  test("should invalidate all entries for a file", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    // Cache multiple profiles for same file with different options
    await manager.cacheProfile(csvPath, { limit: 5 }, "Profile 1");
    await manager.cacheProfile(csvPath, { limit: 10 }, "Profile 2");

    let stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 2);

    // Invalidate all entries for the file
    await manager.invalidate(csvPath);

    stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 0);
  });

  test("should handle cleanup correctly", async () => {
    // Use short TTL
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 100, // 100ms TTL
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    await manager.cacheProfile(csvPath, {}, "Profile");

    // Wait for TTL to expire
    await new Promise((resolve) => setTimeout(resolve, 150));

    // Run cleanup
    await manager.cleanup();

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 0);
  });

  test("should clear entire cache", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath1 = await createTestCsv(testDir, "test1.csv", "a,b,c\n1,2,3\n");
    const csvPath2 = await createTestCsv(testDir, "test2.csv", "d,e,f\n4,5,6\n");

    await manager.cacheProfile(csvPath1, {}, "Profile 1");
    await manager.cacheProfile(csvPath2, {}, "Profile 2");

    let stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 2);

    await manager.clear();

    stats = await manager.getFullStats();
    assert.strictEqual(stats.entries, 0);
  });

  test("should track metrics correctly", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    // Cache miss
    await manager.getCachedProfile(csvPath, {});

    // Cache the profile
    await manager.cacheProfile(csvPath, {}, "Profile");

    // Cache hit
    await manager.getCachedProfile(csvPath, {});
    await manager.getCachedProfile(csvPath, {});

    const stats = await manager.getFullStats();
    assert.strictEqual(stats.metrics.misses, 1);
    assert.strictEqual(stats.metrics.hits, 2);
    assert.ok(stats.hitRate > 0.6, "Hit rate should be > 60%");
  });

  test("should reset metrics correctly", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    // Generate some metrics
    await manager.getCachedProfile(csvPath, {});
    await manager.cacheProfile(csvPath, {}, "Profile");
    await manager.getCachedProfile(csvPath, {});

    let stats = manager.getStats();
    assert.ok(stats.metrics.misses > 0 || stats.metrics.hits > 0);

    // Reset metrics
    manager.resetMetrics();

    stats = manager.getStats();
    assert.strictEqual(stats.metrics.misses, 0);
    assert.strictEqual(stats.metrics.hits, 0);
    assert.strictEqual(stats.metrics.evictions, 0);
    assert.strictEqual(stats.metrics.expirations, 0);
  });

  test("should handle non-existent file gracefully", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const nonExistentPath = join(testDir, "does-not-exist.csv");

    // Should return null without throwing
    const result = await manager.getCachedProfile(nonExistentPath, {});
    assert.strictEqual(result, null);
  });

  test("should handle corrupt cache file gracefully", async () => {
    // Write invalid JSON to cache file
    const cacheFile = join(testDir, ".qsv-mcp-profile-cache.json");
    await writeFile(cacheFile, "{ invalid json }}}");

    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    // Should handle gracefully and create new cache
    await manager.cacheProfile(csvPath, {}, "Profile");
    const result = await manager.getCachedProfile(csvPath, {});

    assert.strictEqual(result, "Profile");
  });

  test("should handle combined options correctly", async () => {
    const manager = new ProfileCacheManager(testDir, {
      maxSizeMB: 10,
      ttlMs: 60 * 60 * 1000,
    });

    const csvPath = await createTestCsv(testDir, "test.csv", "a,b,c\n1,2,3\n");

    const options: ProfileOptions = {
      limit: 5,
      columns: "a,b",
      no_stats: true,
    };

    await manager.cacheProfile(csvPath, options, "Combined options profile");

    // Same options should hit
    const result1 = await manager.getCachedProfile(csvPath, options);
    assert.strictEqual(result1, "Combined options profile");

    // Different options should miss
    const differentOptions: ProfileOptions = {
      limit: 5,
      columns: "a,b",
      no_stats: false, // Different
    };
    const result2 = await manager.getCachedProfile(csvPath, differentOptions);
    assert.strictEqual(result2, null);
  });
});
