/**
 * Converted File Manager
 *
 * Manages .converted.csv files with LIFO cleanup based on total size threshold.
 * Reuses existing converted files if source hasn't changed (timestamp comparison).
 *
 * Key Features:
 * - File-based locking to prevent concurrent conversion race conditions
 * - Robust size tracking with atomic updates
 * - Cache corruption recovery with validation and backup
 * - UUID-based temp file names for collision prevention
 * - Conversion cleanup on failure with in-progress tracking
 * - Path validation to prevent directory traversal attacks
 * - Secure file permissions (0o600) for cache and lock files
 * - Enhanced change detection (mtime + size + inode + optional hash)
 * - Performance metrics tracking
 */

import { stat, unlink, readFile, writeFile, access, rename, readdir, open, realpath, chmod } from 'fs/promises';
import type { FileHandle } from 'fs/promises';
import { constants } from 'fs';
import type { Stats } from 'fs';
import { dirname, basename, join, resolve, relative, sep } from 'path';
import { randomUUID, createHash } from 'crypto';
import { formatBytes } from './utils.js';
import { config } from './config.js';

/**
 * Cache format version 1 with enhanced metadata
 */
interface ConvertedFileCacheV1 {
  version: 1;
  entries: ConvertedFileEntry[];
  totalSize: number;
  lastCleanup?: number;
  nextSequence?: number; // Monotonic counter for stable LIFO sort
}

/**
 * Legacy cache format (version 0) for migration
 */
interface ConvertedFileCacheV0 {
  entries: ConvertedFileEntry[];
  totalSize: number;
}

/**
 * Union type for all cache versions
 */
type ConvertedFileCache = ConvertedFileCacheV1 | ConvertedFileCacheV0;

interface ConvertedFileEntry {
  convertedPath: string;
  sourcePath: string;
  sourceTimestamp: number;
  sourceSize?: number;        // Enhanced change detection
  sourceInode?: number;       // Unix inode for change detection
  sourceHash?: string;        // Optional hash for critical files
  size: number;
  createdAt: number;
  sequence?: number;          // Monotonic sequence for stable LIFO sort
}

/**
 * Tracks conversions in progress to enable cleanup on failure
 */
interface ConversionInProgress {
  sourcePath: string;
  convertedPath: string;
  startedAt: number;
  pid: number;
}

interface ConversionTracker {
  conversions: ConversionInProgress[];
}

/**
 * Metrics for tracking cache performance
 */
export interface ConversionMetrics {
  conversions: {
    total: number;
    successful: number;
    failed: number;
  };
  cache: {
    hits: number;
    misses: number;
    evictions: number;
    orphansRemoved: number;
  };
  cleanup: {
    runs: number;
    filesDeleted: number;
    bytesFreed: number;
    partialConversionsRemoved: number;
  };
  errors: {
    conversionErrors: number;
    cacheLoadErrors: number;
    cacheSaveErrors: number;
    deletionErrors: number;
  };
}

/**
 * File-based lock for preventing concurrent conversions of the same file
 */
class ConversionLock {
  private baseLockPath: string;
  private currentLockPath: string | null = null;
  private lockFd: FileHandle | null = null;
  private static readonly STALE_LOCK_AGE_MS = 10 * 60 * 1000; // 10 minutes

  constructor(lockPath: string) {
    this.baseLockPath = lockPath;
  }

  /**
   * Acquire an exclusive lock for the given source path
   * Returns true if lock acquired, false if already locked
   */
  async acquireLock(sourcePath: string): Promise<boolean> {
    // Generate lock file path based on source path hash
    const sourceHash = createHash('sha256').update(sourcePath).digest('hex').substring(0, 16);
    this.currentLockPath = `${this.baseLockPath}.lock.${sourceHash}`;

    try {
      // Clean up stale lock if it exists
      await this.cleanupStaleLock();

      // Try to create lock file exclusively (fails if exists)
      this.lockFd = await open(this.currentLockPath, constants.O_CREAT | constants.O_EXCL | constants.O_WRONLY, 0o600);

      // Write lock metadata
      const lockData = JSON.stringify({
        sourcePath,
        pid: process.pid,
        startedAt: Date.now(),
      });
      try {
        await this.lockFd.writeFile(lockData, 'utf-8');
      } catch (writeError) {
        // Cleanup on write failure: close fd and delete lock file
        if (this.lockFd) {
          try {
            await this.lockFd.close();
          } catch {
            // ignore secondary close errors
          } finally {
            this.lockFd = null;
          }
        }
        if (this.currentLockPath) {
          try {
            await unlink(this.currentLockPath);
          } catch {
            // ignore secondary unlink errors
          }
        }
        throw writeError;
      }

      console.error(`[ConversionLock] Acquired lock for: ${sourcePath}`);
      return true;
    } catch (error: any) {
      if (error.code === 'EEXIST') {
        console.error(`[ConversionLock] Lock already held for: ${sourcePath}`);
        return false;
      }
      // Other errors (permissions, etc.)
      console.error(`[ConversionLock] Failed to acquire lock:`, error);
      return false;
    }
  }

  /**
   * Release the lock
   */
  async releaseLock(): Promise<void> {
    if (this.lockFd !== null) {
      try {
        await this.lockFd.close();
        this.lockFd = null;
      } catch (error) {
        console.error('[ConversionLock] Error closing lock file descriptor:', error);
      }
    }

    if (this.currentLockPath !== null) {
      try {
        await unlink(this.currentLockPath);
        console.error(`[ConversionLock] Released lock: ${basename(this.currentLockPath)}`);
      } catch (error: any) {
        if (error.code !== 'ENOENT') {
          console.error('[ConversionLock] Error deleting lock file:', error);
        }
      }
      this.currentLockPath = null;
    }
  }

  /**
   * Clean up stale lock file (older than STALE_LOCK_AGE_MS)
   */
  private async cleanupStaleLock(): Promise<void> {
    if (this.currentLockPath === null) {
      return;
    }

    try {
      const stats = await stat(this.currentLockPath);
      const age = Date.now() - stats.mtime.getTime();

      if (age > ConversionLock.STALE_LOCK_AGE_MS) {
        try {
          await unlink(this.currentLockPath);
          console.error(`[ConversionLock] Cleaned up stale lock (${Math.round(age / 1000)}s old)`);
        } catch (error: any) {
          if (error.code !== 'ENOENT') {
            // Ignore race where another process already removed the stale lock
            console.error('[ConversionLock] Error deleting stale lock file:', error);
          }
        }
      }
    } catch (error: any) {
      if (error.code !== 'ENOENT') {
        // Ignore missing files, log other errors
        console.error('[ConversionLock] Error checking stale lock:', error);
      }
    }
  }

  /**
   * Execute a function with an exclusive lock
   * Returns null if lock cannot be acquired (another process holds it)
   */
  async withLock<T>(sourcePath: string, fn: () => Promise<T>): Promise<T | null> {
    const acquired = await this.acquireLock(sourcePath);
    if (!acquired) {
      console.warn(`[ConversionLock] Cannot acquire lock for ${sourcePath} - another process may be converting this file`);
      return null; // Lock not acquired, another process is converting
    }

    try {
      return await fn();
    } finally {
      await this.releaseLock();
    }
  }
}

export class ConvertedFileManager {
  private static readonly DEFAULT_MAX_SIZE_GB = 1;
  private static readonly CACHE_FILE = '.qsv-mcp-converted-cache.json';
  private static readonly CONVERSIONS_FILE = '.qsv-mcp-conversions.json';
  private static readonly STALE_CONVERSION_AGE_MS = 60 * 60 * 1000; // 1 hour

  private maxSizeBytes: number;
  private cacheFilePath: string;
  private conversionsFilePath: string;
  private workingDir: string;
  private metrics: ConversionMetrics;

  constructor(workingDir: string, maxSizeGB?: number) {
    // Validate and sanitize size limit
    const sizeGB = this.validateSizeLimit(
      maxSizeGB ?? config.convertedLifoSizeGB
    );

    this.maxSizeBytes = sizeGB * 1024 * 1024 * 1024;
    this.workingDir = workingDir;
    this.cacheFilePath = join(workingDir, ConvertedFileManager.CACHE_FILE);
    this.conversionsFilePath = join(workingDir, ConvertedFileManager.CONVERSIONS_FILE);

    // Initialize metrics
    this.metrics = {
      conversions: { total: 0, successful: 0, failed: 0 },
      cache: { hits: 0, misses: 0, evictions: 0, orphansRemoved: 0 },
      cleanup: { runs: 0, filesDeleted: 0, bytesFreed: 0, partialConversionsRemoved: 0 },
      errors: { conversionErrors: 0, cacheLoadErrors: 0, cacheSaveErrors: 0, deletionErrors: 0 },
    };
  }

  /**
   * Validate size limit configuration
   * Valid range: 0.1-100 GB
   * Recommended range: 0.5-100 GB
   */
  private validateSizeLimit(sizeGB: number): number {
    // Reject invalid values (non-numeric, negative, zero, infinity, NaN)
    if (!Number.isFinite(sizeGB) || sizeGB <= 0) {
      console.error(`[Converted File Manager] Invalid size limit: ${sizeGB}, using default ${ConvertedFileManager.DEFAULT_MAX_SIZE_GB} GB`);
      return ConvertedFileManager.DEFAULT_MAX_SIZE_GB;
    }

    // Reject out of bounds (< 0.1 GB or > 100 GB)
    if (sizeGB < 0.1 || sizeGB > 100) {
      console.error(`[Converted File Manager] Size limit out of valid range: ${sizeGB} GB (must be 0.1-100 GB), using default ${ConvertedFileManager.DEFAULT_MAX_SIZE_GB} GB`);
      return ConvertedFileManager.DEFAULT_MAX_SIZE_GB;
    }

    // Warn about unusual but valid values
    if (sizeGB < 0.5) {
      console.warn(`[Converted File Manager] Unusually small size limit: ${sizeGB} GB (recommended minimum: 0.5 GB)`);
    }

    return sizeGB;
  }

  /**
   * Validate and canonicalize file path
   * Prevents directory traversal attacks and normalizes paths
   * @param filePath The path to validate
   * @param mustExist If true, throws error if file doesn't exist (default: false)
   */
  private async validatePath(filePath: string, mustExist: boolean = false): Promise<string> {
    // Check for control characters (includes null bytes)
    if (/[\x00-\x1F\x7F]/.test(filePath)) {
      throw new Error('Invalid path: contains control characters');
    }

    // Check path length (reasonable limit)
    if (filePath.length > 4096) {
      throw new Error('Invalid path: exceeds maximum length (4096)');
    }

    // Canonicalize path to resolve symlinks and normalize
    try {
      const canonical = await realpath(filePath);
      return canonical;
    } catch (error: any) {
      if (error.code === 'ENOENT') {
        // File doesn't exist - normalize it anyway to prevent directory traversal
        if (mustExist) {
          throw new Error(`File does not exist: ${filePath}`);
        }

        // Use resolve() to normalize path and remove .. sequences
        // This prevents directory traversal attacks like ../../../../etc/passwd
        const normalized = resolve(filePath);

        // Additional validation: ensure normalized path doesn't escape working directory
        // Use path.relative() for robust cross-platform validation (handles Windows case-insensitivity)
        const workingDirResolved = resolve(this.workingDir);
        const relativePath = relative(workingDirResolved, normalized);

        // If relative path starts with "..", it's trying to escape the working directory
        if (relativePath.startsWith('..' + sep) || relativePath === '..') {
          throw new Error(`Path escapes working directory: ${filePath} -> ${normalized} (relative: ${relativePath})`);
        }

        return normalized;
      }
      throw error;
    }
  }

  /**
   * Set secure permissions on a file
   * Sets mode to 0o600 (owner read/write only)
   */
  private async setSecurePermissions(filePath: string): Promise<void> {
    try {
      await chmod(filePath, 0o600);
    } catch (error) {
      // Log but don't fail - permissions aren't critical on all platforms
      console.error(`[Converted File Manager] Warning: Failed to set secure permissions on ${filePath}:`, error);
    }
  }

  /**
   * Compute SHA-256 hash of first 4KB of file
   * Used for enhanced change detection on critical files
   */
  private async computeFileHash(filePath: string): Promise<string | null> {
    try {
      const handle = await open(filePath, 'r');
      try {
        const buffer = Buffer.alloc(4096);
        const { bytesRead } = await handle.read(buffer, 0, 4096, 0);

        if (bytesRead > 0) {
          const hash = createHash('sha256')
            .update(buffer.slice(0, bytesRead))
            .digest('hex');
          return `sha256:${hash.substring(0, 16)}`; // Use first 16 chars for space efficiency
        }
        return null;
      } finally {
        await handle.close();
      }
    } catch (error) {
      console.error('[Converted File Manager] Error computing file hash:', error);
      return null;
    }
  }

  /**
   * Check if source file has changed using enhanced detection
   * Uses multiple signals: inode > size+mtime > mtime-only
   *
   * Note: Hash comparison is intentionally NOT performed here as it's expensive.
   * The sourceHash field is stored for future use cases (e.g., manual verification,
   * debugging, or when called with an explicitly computed hash). Callers needing
   * hash-based validation should call computeFileHash() separately and compare.
   */
  private hasSourceChanged(entry: ConvertedFileEntry, sourceStats: Stats): boolean {
    // Priority 1: Inode comparison (Unix-like systems only, fast and reliable)
    // Skip on Windows where ino is always 0 (meaningless comparison)
    if (entry.sourceInode !== undefined && sourceStats.ino !== undefined && sourceStats.ino !== 0) {
      if (entry.sourceInode !== sourceStats.ino) {
        console.error('[Converted File Manager] Source file inode changed (file replaced)');
        return true; // Inode changed = file was replaced
      }
    }

    // Priority 2: Size + mtime comparison
    if (entry.sourceSize !== undefined) {
      const sizeChanged = entry.sourceSize !== sourceStats.size;
      const mtimeChanged = sourceStats.mtime.getTime() > entry.sourceTimestamp;

      if (sizeChanged) {
        console.error('[Converted File Manager] Source file size changed');
        return true;
      }

      if (mtimeChanged) {
        console.error('[Converted File Manager] Source file mtime changed');
        return true;
      }

      return false; // Size and mtime match
    }

    // Priority 3: Fallback to mtime-only (legacy behavior for old cache entries)
    if (sourceStats.mtime.getTime() > entry.sourceTimestamp) {
      console.error('[Converted File Manager] Source file mtime changed (fallback check)');
      return true;
    }

    return false; // No changes detected
  }

  /**
   * Load cache from disk with validation and migration
   */
  private async loadCache(): Promise<ConvertedFileCacheV1> {
    try {
      await access(this.cacheFilePath, constants.R_OK);
      const data = await readFile(this.cacheFilePath, 'utf-8');
      const cache = JSON.parse(data);

      // Validate and migrate if needed
      return await this.validateCache(cache);
    } catch (error: any) {
      if (error.code === 'ENOENT') {
        // Cache doesn't exist yet, return empty with initialized sequence
        return { version: 1, entries: [], totalSize: 0, nextSequence: 0 };
      }

      // JSON parse error or other corruption
      // Track cache load failures/corruption in metrics if available
      if (
        this.metrics &&
        this.metrics.errors &&
        typeof this.metrics.errors.cacheLoadErrors === 'number'
      ) {
        this.metrics.errors.cacheLoadErrors++;
      }
      console.error('[Converted File Manager] Cache load failed, attempting recovery:', error);
      return await this.recoverFromCorruption();
    }
  }

  /**
   * Validate cache structure and migrate from v0 to v1 if needed
   * Initializes sequence counter if missing
   * Persists any corrections made during validation
   */
  private async validateCache(cache: any): Promise<ConvertedFileCacheV1> {
    // Check if this is v0 format (no version field)
    if (!cache.version) {
      console.error('[Converted File Manager] Detected v0 cache format, migrating to v1...');
      return await this.migrateV0ToV1(cache as ConvertedFileCacheV0);
    }

    // Validate v1 format
    if (cache.version !== 1) {
      throw new Error(`Unknown cache version: ${cache.version}`);
    }

    if (!Array.isArray(cache.entries)) {
      throw new Error('Invalid cache: entries is not an array');
    }

    if (typeof cache.totalSize !== 'number') {
      throw new Error('Invalid cache: totalSize is not a number');
    }

    // Track whether any corrections were made that need to be persisted
    let needsPersist = false;

    // Initialize nextSequence if missing (for existing v1 caches)
    if (cache.nextSequence === undefined) {
      let maxSequence = -1;
      for (const entry of cache.entries) {
        if (entry.sequence !== undefined && entry.sequence > maxSequence) {
          maxSequence = entry.sequence;
        }
      }
      cache.nextSequence = maxSequence + 1;
      console.error(`[Converted File Manager] Initialized nextSequence to ${cache.nextSequence}`);
      needsPersist = true;
    }

    // Recalculate totalSize to ensure consistency
    // Allow small tolerance (100 bytes) for concurrent operations or filesystem metadata timing
    const calculatedSize = cache.entries.reduce((sum: number, e: ConvertedFileEntry) => sum + e.size, 0);
    const TOLERANCE_BYTES = 100;
    if (Math.abs(calculatedSize - cache.totalSize) > TOLERANCE_BYTES) {
      console.warn(`[Converted File Manager] totalSize mismatch (${cache.totalSize} vs ${calculatedSize}), fixing...`);
      cache.totalSize = calculatedSize;
      needsPersist = true;
    }

    // Persist corrections to disk if any were made
    if (needsPersist) {
      console.error('[Converted File Manager] Persisting cache validation fixes...');
      await this.saveCache(cache as ConvertedFileCacheV1);
    }

    return cache as ConvertedFileCacheV1;
  }

  /**
   * Migrate v0 cache format to v1
   */
  private async migrateV0ToV1(cacheV0: ConvertedFileCacheV0): Promise<ConvertedFileCacheV1> {
    // Backup v0 cache
    const backupPath = `${this.cacheFilePath}.v0.backup`;
    try {
      await writeFile(backupPath, JSON.stringify(cacheV0, null, 2));
      console.error(`[Converted File Manager] Backed up v0 cache to: ${backupPath}`);
    } catch (error) {
      console.error('[Converted File Manager] Failed to backup v0 cache:', error);
    }

    // Assign sequences to existing entries for stable sort
    // Use createdAt timestamps as base to ensure monotonicity across migrations
    // Sort entries by createdAt first to assign sequences in chronological order
    const sortedEntries = [...cacheV0.entries].sort((a, b) => a.createdAt - b.createdAt);

    let sequence = 0;
    const entriesWithSequence = sortedEntries.map(entry => ({
      ...entry,
      sequence: sequence++,
    }));

    // Create v1 cache
    const cacheV1: ConvertedFileCacheV1 = {
      version: 1,
      entries: entriesWithSequence,
      totalSize: cacheV0.totalSize,
      lastCleanup: Date.now(),
      nextSequence: sequence,
    };

    console.error('[Converted File Manager] Migration to v1 complete');
    return cacheV1;
  }

  /**
   * Recover from cache corruption by rebuilding from filesystem
   */
  private async recoverFromCorruption(): Promise<ConvertedFileCacheV1> {
    // Backup corrupted cache
    try {
      const corruptData = await readFile(this.cacheFilePath, 'utf-8').catch(() => null);
      if (corruptData) {
        const backupPath = `${this.cacheFilePath}.corrupt.${Date.now()}`;
        await writeFile(backupPath, corruptData);
        console.error(`[Converted File Manager] Backed up corrupted cache to: ${backupPath}`);
      }
    } catch (error) {
      console.error('[Converted File Manager] Failed to backup corrupted cache:', error);
    }

    // Scan filesystem for .converted.csv files
    console.error('[Converted File Manager] Rebuilding cache from filesystem...');
    const entries: ConvertedFileEntry[] = [];
    let totalSize = 0;

    try {
      const files = await readdir(this.workingDir);

      for (const file of files) {
        if (file.includes('.converted.') && file.endsWith('.csv')) {
          const convertedPath = join(this.workingDir, file);

          try {
            const stats = await stat(convertedPath);

            // Try to infer source path (remove .converted.{uuid}.csv suffix)
            // Use more robust parsing to handle edge cases like filenames containing ".converted."
            const fileBasename = basename(convertedPath);
            const fileDir = dirname(convertedPath);

            // Find the position of ".converted." marker
            const markerIndex = fileBasename.lastIndexOf('.converted.');
            if (markerIndex === -1) {
              // Malformed filename, skip this entry
              console.error(`[Converted File Manager] Malformed converted filename: ${fileBasename}`);
              continue;
            }

            // Extract source base name (everything before ".converted.")
            const sourceBasename = fileBasename.substring(0, markerIndex);
            const sourcePath = join(fileDir, sourceBasename);

            // Verify source exists
            const sourceStats = await stat(sourcePath).catch(() => null);

            if (sourceStats) {
              entries.push({
                convertedPath,
                sourcePath,
                sourceTimestamp: sourceStats.mtime.getTime(),
                size: stats.size,
                createdAt: stats.mtime.getTime(),
              });
              totalSize += stats.size;
            } else {
              // Source doesn't exist, orphaned converted file
              console.error(`[Converted File Manager] Found orphaned converted file: ${file}`);
            }
          } catch (error) {
            console.error(`[Converted File Manager] Error processing ${file}:`, error);
          }
        }
      }

      console.error(`[Converted File Manager] Recovered ${entries.length} entries from filesystem`);
    } catch (error) {
      console.error('[Converted File Manager] Error scanning filesystem:', error);
    }

    // Assign sequences to recovered entries
    // Sort by createdAt first to ensure monotonicity
    const sortedEntries = [...entries].sort((a, b) => a.createdAt - b.createdAt);

    let sequence = 0;
    const entriesWithSequence = sortedEntries.map(entry => ({
      ...entry,
      sequence: sequence++,
    }));

    return {
      version: 1,
      entries: entriesWithSequence,
      totalSize,
      lastCleanup: Date.now(),
      nextSequence: sequence,
    };
  }

  /**
   * Save cache to disk using atomic write (temp file + rename)
   * Uses UUID to prevent collisions
   * Sets secure permissions (0o600)
   */
  private async saveCache(cache: ConvertedFileCacheV1): Promise<void> {
    try {
      // Write to temporary file first with UUID for uniqueness
      const tempPath = `${this.cacheFilePath}.tmp.${process.pid}.${randomUUID()}`;
      await writeFile(tempPath, JSON.stringify(cache, null, 2));

      // Set secure permissions on temp file
      await this.setSecurePermissions(tempPath);

      // Atomic rename to actual cache file
      await rename(tempPath, this.cacheFilePath);

      // Ensure secure permissions on final file
      await this.setSecurePermissions(this.cacheFilePath);
    } catch (error) {
      this.metrics.errors.cacheSaveErrors++;
      console.error('[Converted File Manager] Failed to save cache:', error);
      throw error;
    }
  }

  /**
   * Recalculate total size from entries (for consistency checks)
   */
  private recalculateTotalSize(cache: ConvertedFileCacheV1): number {
    return cache.entries.reduce((sum, e) => sum + e.size, 0);
  }

  /**
   * Check if a converted file exists and is still valid (source hasn't changed)
   * Uses multi-factor change detection when cache entry is available
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

      // Try to find cache entry for enhanced change detection
      const cache = await this.loadCache();
      const entry = cache.entries.find(e => e.sourcePath === sourcePath && e.convertedPath === convertedPath);

      if (entry) {
        // Use enhanced change detection if cache entry exists
        if (this.hasSourceChanged(entry, sourceStats)) {
          console.error(`[Converted File Manager] Source modified (enhanced check), reconverting: ${sourcePath}`);
          this.metrics.cache.misses++;
          return null; // Source has changed, need to reconvert
        }
      } else {
        // Fallback to simple mtime check if no cache entry
        if (sourceStats.mtime.getTime() > convertedStats.mtime.getTime()) {
          console.error(`[Converted File Manager] Source modified (mtime check), reconverting: ${sourcePath}`);
          this.metrics.cache.misses++;
          return null; // Source has changed, need to reconvert
        }
      }

      console.error(`[Converted File Manager] Reusing existing converted file: ${convertedPath}`);
      this.metrics.cache.hits++;
      return convertedPath;

    } catch (error) {
      console.error('[Converted File Manager] Error checking converted file:', error);
      return null;
    }
  }

  /**
   * Register a conversion as in-progress to enable cleanup on failure
   * Tracks conversion metrics
   */
  async registerConversionStart(sourcePath: string, convertedPath: string): Promise<void> {
    try {
      const tracker = await this.loadConversionTracker();

      // Add this conversion
      tracker.conversions.push({
        sourcePath,
        convertedPath,
        startedAt: Date.now(),
        pid: process.pid,
      });

      await this.saveConversionTracker(tracker);

      // Track conversion start
      this.metrics.conversions.total++;
    } catch (error) {
      console.error('[Converted File Manager] Error registering conversion start:', error);
    }
  }

  /**
   * Mark a conversion as complete (remove from in-progress tracking)
   */
  async registerConversionComplete(sourcePath: string): Promise<void> {
    try {
      const tracker = await this.loadConversionTracker();

      // Remove this conversion
      tracker.conversions = tracker.conversions.filter(c => c.sourcePath !== sourcePath);

      await this.saveConversionTracker(tracker);
    } catch (error) {
      console.error('[Converted File Manager] Error registering conversion complete:', error);
    }
  }

  /**
   * Track a failed conversion
   */
  trackConversionFailure(): void {
    this.metrics.conversions.failed++;
  }

  /**
   * Load conversion tracker
   */
  private async loadConversionTracker(): Promise<ConversionTracker> {
    try {
      const data = await readFile(this.conversionsFilePath, 'utf-8');
      return JSON.parse(data);
    } catch {
      return { conversions: [] };
    }
  }

  /**
   * Save conversion tracker
   * Enhanced with secure permissions
   */
  private async saveConversionTracker(tracker: ConversionTracker): Promise<void> {
    try {
      const tempPath = `${this.conversionsFilePath}.tmp.${process.pid}.${randomUUID()}`;
      await writeFile(tempPath, JSON.stringify(tracker, null, 2));

      // Set secure permissions on temp file
      await this.setSecurePermissions(tempPath);

      // Atomic rename to actual tracker file
      await rename(tempPath, this.conversionsFilePath);

      // Ensure secure permissions on final file
      await this.setSecurePermissions(this.conversionsFilePath);
    } catch (error) {
      console.error('[Converted File Manager] Failed to save conversion tracker:', error);
    }
  }

  /**
   * Clean up partial conversions (files from failed conversions)
   */
  async cleanupPartialConversions(): Promise<void> {
    try {
      const tracker = await this.loadConversionTracker();
      const staleThreshold = Date.now() - ConvertedFileManager.STALE_CONVERSION_AGE_MS;
      const activeConversions: ConversionInProgress[] = [];

      for (const conversion of tracker.conversions) {
        const age = Date.now() - conversion.startedAt;

        // Check if conversion is stale
        if (conversion.startedAt < staleThreshold) {
          console.error(
            `[Converted File Manager] Cleaning up stale conversion (${Math.round(age / 1000)}s old): ${conversion.convertedPath}`
          );

          // Try to delete partial file
          try {
            await unlink(conversion.convertedPath);
            this.metrics.cleanup.partialConversionsRemoved++; // Track partial cleanup
          } catch (error: any) {
            if (error.code !== 'ENOENT') {
              console.error('[Converted File Manager] Failed to delete partial file:', error);
            } else {
              // File already gone
              this.metrics.cleanup.partialConversionsRemoved++; // Track partial cleanup
            }
          }
        } else {
          // Keep active conversions
          activeConversions.push(conversion);
        }
      }

      // Update tracker
      tracker.conversions = activeConversions;
      await this.saveConversionTracker(tracker);

    } catch (error) {
      console.error('[Converted File Manager] Error cleaning up partial conversions:', error);
    }
  }

  /**
   * Register a newly created converted file and enforce LIFO size limit
   * Uses file locking to prevent race conditions
   * Enhanced with multi-factor metadata
   */
  async registerConvertedFile(
    sourcePath: string,
    convertedPath: string,
  ): Promise<void> {
    const lock = new ConversionLock(this.cacheFilePath);

    const result = await lock.withLock(sourcePath, async () => {
      try {
        // Validate paths - both files must exist at registration time
        await this.validatePath(convertedPath, true);
        await this.validatePath(sourcePath, true);

        const cache = await this.loadCache();
        const convertedStats = await stat(convertedPath);
        const sourceStats = await stat(sourcePath);

        // Remove existing entry for this source if present
        cache.entries = cache.entries.filter(e => e.sourcePath !== sourcePath);

        // Compute hash for files under 10MB (stored for debugging/manual verification)
        // Note: Not used in automatic change detection due to performance cost
        const sourceHashResult = sourceStats.size < 10 * 1024 * 1024
          ? await this.computeFileHash(sourcePath)
          : null;

        // Assign sequence for stable LIFO sort
        const sequence = cache.nextSequence ?? 0;
        cache.nextSequence = sequence + 1;

        // Add new entry with enhanced metadata
        const newEntry: ConvertedFileEntry = {
          convertedPath,
          sourcePath,
          sourceTimestamp: sourceStats.mtime.getTime(),
          sourceSize: sourceStats.size,
          sourceInode: sourceStats.ino,
          sourceHash: sourceHashResult ?? undefined, // Convert null to undefined
          size: convertedStats.size,
          createdAt: Date.now(),
          sequence,
        };

        cache.entries.push(newEntry);

        // Recalculate total size for consistency
        cache.totalSize = this.recalculateTotalSize(cache);

        // Enforce size limit (LIFO - delete oldest first)
        await this.enforceSizeLimit(cache);

        await this.saveCache(cache);

        // Track successful conversion registration
        // Note: Conversion was already marked complete before calling this method
        this.metrics.conversions.successful++;

      } catch (error) {
        console.error('[Converted File Manager] Error registering converted file:', error);
        throw error;
      }
    });

    // Check if lock acquisition failed
    if (result === null) {
      console.warn(`[Converted File Manager] Failed to register ${convertedPath} - could not acquire lock (another process may be registering the same file)`);
      // Note: The file is still valid and usable, just not tracked in the cache yet.
      // The next cleanup cycle or successful registration attempt will add it.
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
   * Enhanced with robust error handling to prevent size tracking corruption
   * Enhanced with stable sort using sequence counter
   */
  private async enforceSizeLimit(cache: ConvertedFileCacheV1): Promise<void> {
    // Hysteresis to prevent cleanup thrashing
    // Decision tree:
    // 1. Size <= 100%: Don't clean (under limit)
    // 2. Size 100-110% AND cleaned <1hr ago: Don't clean (hysteresis band, prevent thrashing)
    // 3. Size 100-110% AND cleaned >=1hr ago: Clean (in band but stale, prevent indefinite growth)
    // 4. Size > 110%: Always clean (exceeded tolerance, ignore time)
    const hysteresisThreshold = this.maxSizeBytes * 1.1; // 10% over limit
    const timeSinceLastCleanup = cache.lastCleanup ? Date.now() - cache.lastCleanup : Infinity;
    const oneHourMs = 60 * 60 * 1000;

    // Case 1: Under strict limit - no cleanup needed
    if (cache.totalSize <= this.maxSizeBytes) {
      return;
    }

    // Case 2: Within hysteresis band AND recently cleaned - skip to prevent thrashing
    // Cases 3 & 4: Either exceeded hysteresis OR enough time passed - proceed with cleanup
    if (cache.totalSize <= hysteresisThreshold && timeSinceLastCleanup < oneHourMs) {
      return; // Hysteresis: Don't clean if in band and cleaned recently
    }

    // Reaching here means we should clean because:
    // - Over 110% (case 4), OR
    // - Between 100-110% but haven't cleaned in over an hour (case 3)

    // Create a sorted copy for cleanup iteration (don't mutate cache.entries directly)
    // This prevents inconsistency if concurrent operations save during cleanup
    const sortedEntries = [...cache.entries].sort((a, b) => {
      // Primary sort: createdAt timestamp (oldest first) for LIFO deletion
      const timeDiff = a.createdAt - b.createdAt;
      if (timeDiff !== 0) {
        return timeDiff;
      }

      // Secondary sort: sequence number for deterministic ordering when timestamps match
      const seqA = a.sequence ?? 0;
      const seqB = b.sequence ?? 0;
      return seqA - seqB;
    });

    const toDelete: ConvertedFileEntry[] = [];
    let currentSize = cache.totalSize;

    // Mark oldest files for deletion until we're under the limit
    for (const entry of sortedEntries) {
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

    // Track cleanup run
    this.metrics.cleanup.runs++;

    // Delete files with robust error handling
    for (const entry of toDelete) {
      let deleted = false;

      try {
        await unlink(entry.convertedPath);
        deleted = true;
        console.error(`[Converted File Manager] Deleted: ${entry.convertedPath} (${formatBytes(entry.size)})`);
      } catch (error: any) {
        if (error.code === 'ENOENT') {
          // File already gone, treat as deleted
          deleted = true;
        } else {
          console.error(`[Converted File Manager] Failed to delete ${entry.convertedPath}:`, error);
          this.metrics.errors.deletionErrors++; // Track deletion error
        }
      } finally {
        // Always update cache to reflect actual state
        if (deleted) {
          cache.entries = cache.entries.filter(e => e.convertedPath !== entry.convertedPath);
          cache.totalSize -= entry.size;

          // Track metrics
          this.metrics.cache.evictions++;
          this.metrics.cleanup.filesDeleted++;
          this.metrics.cleanup.bytesFreed += entry.size;
        } else {
          // File deletion failed but it still exists, keep it in cache
          // This prevents size tracking corruption
          console.error(`[Converted File Manager] Keeping entry in cache due to deletion failure: ${entry.convertedPath}`);
        }
      }
    }

    // Update last cleanup timestamp
    cache.lastCleanup = Date.now();

    // Final consistency check
    const recalculated = this.recalculateTotalSize(cache);
    const TOLERANCE_BYTES = 100; // Small tolerance for concurrent operations
    if (Math.abs(recalculated - cache.totalSize) > TOLERANCE_BYTES) {
      console.warn(`[Converted File Manager] Size discrepancy after cleanup, fixing (${cache.totalSize} -> ${recalculated})`);
      cache.totalSize = recalculated;
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
          this.metrics.cache.orphansRemoved++; // Track orphan removal
        }
      }

      if (validEntries.length !== cache.entries.length) {
        cache.entries = validEntries;
        cache.totalSize = this.recalculateTotalSize(cache);
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
            const tempPath = join(cacheDir, file);
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

      // Clean up partial conversions
      await this.cleanupPartialConversions();

    } catch (error) {
      console.error('[Converted File Manager] Error cleaning up orphaned entries:', error);
    }
  }

  /**
   * Get current metrics snapshot
   */
  getMetrics(): Readonly<ConversionMetrics> {
    return { ...this.metrics };
  }

  /**
   * Reset metrics counters
   */
  resetMetrics(): void {
    this.metrics = {
      conversions: { total: 0, successful: 0, failed: 0 },
      cache: { hits: 0, misses: 0, evictions: 0, orphansRemoved: 0 },
      cleanup: { runs: 0, filesDeleted: 0, bytesFreed: 0, partialConversionsRemoved: 0 },
      errors: { conversionErrors: 0, cacheLoadErrors: 0, cacheSaveErrors: 0, deletionErrors: 0 },
    };
  }
}
