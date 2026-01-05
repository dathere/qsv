/**
 * Converted File Manager
 *
 * Manages .converted.csv files with LIFO cleanup based on total size threshold.
 * Reuses existing converted files if source hasn't changed (timestamp comparison).
 */

import { stat, unlink, readFile, writeFile, access, rename, readdir } from 'fs/promises';
import { constants } from 'fs';
import { dirname, basename } from 'path';
import { formatBytes } from './utils.js';

interface ConvertedFileEntry {
  convertedPath: string;
  sourcePath: string;
  sourceTimestamp: number;
  size: number;
  createdAt: number;
}

interface ConvertedFileCache {
  entries: ConvertedFileEntry[];
  totalSize: number;
}

export class ConvertedFileManager {
  private static readonly DEFAULT_MAX_SIZE_GB = 1;
  private static readonly CACHE_FILE = '.qsv-mcp-converted-cache.json';

  private maxSizeBytes: number;
  private cacheFilePath: string;

  constructor(workingDir: string, maxSizeGB?: number) {
    const sizeGB = maxSizeGB ??
                   (parseFloat(process.env.QSV_MCP_CONVERTED_LIFO_SIZE_GB || '') ||
                   ConvertedFileManager.DEFAULT_MAX_SIZE_GB);
    this.maxSizeBytes = sizeGB * 1024 * 1024 * 1024;
    this.cacheFilePath = `${workingDir}/${ConvertedFileManager.CACHE_FILE}`;
  }

  /**
   * Load cache from disk
   */
  private async loadCache(): Promise<ConvertedFileCache> {
    try {
      await access(this.cacheFilePath, constants.R_OK);
      const data = await readFile(this.cacheFilePath, 'utf-8');
      return JSON.parse(data);
    } catch {
      return { entries: [], totalSize: 0 };
    }
  }

  /**
   * Save cache to disk using atomic write (temp file + rename)
   * to prevent corruption from concurrent access
   */
  private async saveCache(cache: ConvertedFileCache): Promise<void> {
    try {
      // Write to temporary file first
      const tempPath = `${this.cacheFilePath}.tmp.${process.pid}.${Date.now()}`;
      await writeFile(tempPath, JSON.stringify(cache, null, 2));

      // Atomic rename to actual cache file
      await rename(tempPath, this.cacheFilePath);
    } catch (error) {
      console.error('[Converted File Manager] Failed to save cache:', error);
    }
  }

  /**
   * Check if a converted file exists and is still valid (source hasn't changed)
   */
  async getValidConvertedFile(sourcePath: string, convertedPath: string): Promise<string | null> {
    try {
      // Check if both files exist
      const [sourceStats, convertedStats] = await Promise.all([
        stat(sourcePath),
        stat(convertedPath).catch(() => null),
      ]);

      if (!convertedStats) {
        return null; // Converted file doesn't exist
      }

      // Check if source file is newer than converted file
      if (sourceStats.mtime.getTime() > convertedStats.mtime.getTime()) {
        console.error(`[Converted File Manager] Source modified, reconverting: ${sourcePath}`);
        return null; // Source has changed, need to reconvert
      }

      console.error(`[Converted File Manager] Reusing existing converted file: ${convertedPath}`);
      return convertedPath;

    } catch (error) {
      console.error('[Converted File Manager] Error checking converted file:', error);
      return null;
    }
  }

  /**
   * Register a newly created converted file and enforce LIFO size limit
   */
  async registerConvertedFile(
    sourcePath: string,
    convertedPath: string,
  ): Promise<void> {
    try {
      const cache = await this.loadCache();
      const convertedStats = await stat(convertedPath);
      const sourceStats = await stat(sourcePath);

      // Remove existing entry for this source if present
      cache.entries = cache.entries.filter(e => e.sourcePath !== sourcePath);

      // Add new entry
      const newEntry: ConvertedFileEntry = {
        convertedPath,
        sourcePath,
        sourceTimestamp: sourceStats.mtime.getTime(),
        size: convertedStats.size,
        createdAt: Date.now(),
      };

      cache.entries.push(newEntry);
      cache.totalSize = cache.entries.reduce((sum, e) => sum + e.size, 0);

      // Enforce size limit (LIFO - delete oldest first)
      await this.enforceSizeLimit(cache);

      await this.saveCache(cache);
    } catch (error) {
      console.error('[Converted File Manager] Error registering converted file:', error);
    }
  }

  /**
   * Update the timestamp of a reused converted file to prevent premature deletion
   */
  async touchConvertedFile(sourcePath: string): Promise<void> {
    try {
      const cache = await this.loadCache();
      const entry = cache.entries.find(e => e.sourcePath === sourcePath);

      if (entry) {
        // Update timestamp to current time
        entry.createdAt = Date.now();
        await this.saveCache(cache);
        console.error(`[Converted File Manager] Updated timestamp for reused file: ${entry.convertedPath}`);
      }
    } catch (error) {
      console.error('[Converted File Manager] Error updating timestamp:', error);
    }
  }

  /**
   * Enforce LIFO size limit by deleting oldest files
   */
  private async enforceSizeLimit(cache: ConvertedFileCache): Promise<void> {
    if (cache.totalSize <= this.maxSizeBytes) {
      return; // Under limit
    }

    // Sort by createdAt (oldest first) for LIFO deletion
    cache.entries.sort((a, b) => a.createdAt - b.createdAt);

    const toDelete: ConvertedFileEntry[] = [];
    let currentSize = cache.totalSize;

    // Mark oldest files for deletion until we're under the limit
    for (const entry of cache.entries) {
      if (currentSize <= this.maxSizeBytes) {
        break;
      }

      toDelete.push(entry);
      currentSize -= entry.size;
    }

    if (toDelete.length === 0) {
      return;
    }

    console.error(
      `[Converted File Manager] Size limit exceeded (${formatBytes(cache.totalSize)} > ${formatBytes(this.maxSizeBytes)}), ` +
      `deleting ${toDelete.length} oldest file(s)...`
    );

    // Delete files
    for (const entry of toDelete) {
      try {
        await unlink(entry.convertedPath);
        console.error(`[Converted File Manager] Deleted: ${entry.convertedPath} (${formatBytes(entry.size)})`);

        // Remove from cache
        cache.entries = cache.entries.filter(e => e.convertedPath !== entry.convertedPath);
        cache.totalSize -= entry.size;
      } catch (error) {
        console.error(`[Converted File Manager] Failed to delete ${entry.convertedPath}:`, error);
      }
    }
  }

  /**
   * Clean up orphaned entries (converted files that no longer exist)
   */
  async cleanupOrphanedEntries(): Promise<void> {
    try {
      const cache = await this.loadCache();
      const validEntries: ConvertedFileEntry[] = [];

      for (const entry of cache.entries) {
        try {
          await access(entry.convertedPath, constants.F_OK);
          validEntries.push(entry);
        } catch {
          // File doesn't exist, skip it
          console.error(`[Converted File Manager] Removing orphaned entry: ${entry.convertedPath}`);
        }
      }

      if (validEntries.length !== cache.entries.length) {
        cache.entries = validEntries;
        cache.totalSize = validEntries.reduce((sum, e) => sum + e.size, 0);
        await this.saveCache(cache);
      }

      // Clean up stale temp files (older than 1 hour)
      try {
        const cacheDir = dirname(this.cacheFilePath);
        const cacheFilename = basename(this.cacheFilePath);
        const files = await readdir(cacheDir);
        const staleThreshold = Date.now() - 60 * 60 * 1000; // 1 hour

        for (const file of files) {
          // Match temp files for this cache file: .qsv-mcp-converted-cache.json.tmp.*
          if (file.startsWith(`${cacheFilename}.tmp.`)) {
            const tempPath = `${cacheDir}/${file}`;
            try {
              const stats = await stat(tempPath);
              if (stats.mtime.getTime() < staleThreshold) {
                await unlink(tempPath);
                console.error(`[Converted File Manager] Deleted stale temp file: ${file}`);
              }
            } catch {
              // File might have been deleted by another process, ignore
            }
          }
        }
      } catch {
        // Directory read failed, not critical
      }
    } catch (error) {
      console.error('[Converted File Manager] Error cleaning up orphaned entries:', error);
    }
  }

}
