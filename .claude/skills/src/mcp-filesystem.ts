/**
 * MCP Filesystem Resource Provider
 *
 * Exposes local tabular data files as browsable MCP resources, allowing
 * Claude Desktop to work with local files without uploading them.
 */

import { readdir, stat, readFile, realpath, access } from "fs/promises";
import { realpathSync } from "fs";
import { join, resolve, relative, basename, dirname, extname, isAbsolute } from "path";
import { homedir } from "os";
import type {
  McpResource,
  FileMetadata,
} from "./types.js";
import { formatBytes } from "./utils.js";
import { config } from "./config.js";
import { runQsvSimple } from "./executor.js";

/**
 * Expand tilde (~) in paths to the user's home directory
 * Handles: ~ (home dir), ~/path or ~\path (relative to home)
 */
function expandTilde(path: string): string {
  if (!path) return path;

  // Handle ~/ or ~\ (Windows users might use backslash)
  if (path.startsWith("~/") || path.startsWith("~\\")) {
    return join(homedir(), path.slice(2));
  }

  // Handle ~ alone (just home directory)
  if (path === "~") {
    return homedir();
  }

  return path;
}

/**
 * Cache expiration time in milliseconds (1 minute)
 */
const METADATA_CACHE_TTL_MS = 60000;

/**
 * Maximum number of entries in the metadata cache.
 * When exceeded, the oldest entries (by cachedAt) are evicted.
 */
const METADATA_CACHE_MAX_SIZE = 1000;

export interface FilesystemConfig {
  /**
   * Working directory for relative paths (defaults to process.cwd())
   */
  workingDirectory?: string;

  /**
   * Additional allowed directories for file access
   * Paths outside these directories will be rejected for security
   */
  allowedDirectories?: string[];

  /**
   * File extensions to include in listings
   */
  allowedExtensions?: string[];

  /**
   * Maximum file size to preview in bytes (default: 1MB)
   */
  maxPreviewSize?: number;

  /**
   * Number of preview lines to show (default: 20)
   */
  previewLines?: number;

  /**
   * Path to qsv binary (default: 'qsv')
   */
  qsvBinPath?: string;

  /**
   * Plugin mode (Claude Code or Cowork)
   * When true, directories are auto-added to allowedDirs instead of
   * being rejected, because the host environment provides filesystem isolation
   */
  pluginMode?: boolean;
}

export class FilesystemResourceProvider {
  // Static format detection sets (performance optimization)
  private static readonly EXCEL_FORMATS = new Set([
    ".xls",
    ".xlsx",
    ".xlsm",
    ".xlsb",
    // Includes .ods as it is also handled via `qsv excel`
    ".ods",
  ]);
  private static readonly JSONL_FORMATS = new Set([".jsonl", ".ndjson"]);
  private static readonly PARQUET_FORMATS = new Set([".parquet", ".pq"]);

  private workingDir: string;
  private allowedDirs: string[];
  private allowedExtensions: Set<string>;
  private maxPreviewSize: number;
  private previewLines: number;
  private qsvBinPath: string;
  private pluginMode: boolean;
  private metadataCache: Map<string, FileMetadata>;
  private metadataCachePromises: Map<string, Promise<FileMetadata | null>>;

  constructor(config: FilesystemConfig = {}) {
    try {
      this.workingDir = realpathSync(resolve(config.workingDirectory || process.cwd()));
    } catch {
      this.workingDir = resolve(config.workingDirectory || process.cwd());
    }

    this.allowedDirs = [
      this.workingDir,
      ...(config.allowedDirectories || []).map((d) => {
        try {
          return realpathSync(resolve(d));
        } catch {
          return resolve(d);
        }
      }),
    ];
    this.allowedExtensions = new Set(
      config.allowedExtensions || [
        // Native CSV formats
        ".csv",
        ".tsv",
        ".tab",
        ".ssv",
        // Snappy-compressed formats
        ".csv.sz",
        ".tsv.sz",
        ".tab.sz",
        ".ssv.sz",
        // Excel formats (require conversion via qsv excel)
        ".xls",
        ".xlsx",
        ".xlsm",
        ".xlsb",
        // OpenDocument Spreadsheet (require conversion via qsv excel)
        ".ods",
        // JSONL/NDJSON (require conversion via qsv jsonl)
        ".jsonl",
        ".ndjson",
        // Parquet formats (require conversion via qsv sqlp)
        ".parquet",
        ".pq",
      ],
    );
    this.maxPreviewSize = config.maxPreviewSize || 1024 * 1024; // 1MB
    this.previewLines = config.previewLines || 20;
    this.qsvBinPath = config.qsvBinPath || "qsv";
    this.pluginMode = config.pluginMode || false;
    this.metadataCache = new Map();
    this.metadataCachePromises = new Map();

    console.error(`Filesystem provider initialized:`);
    console.error(`  Working directory: ${this.workingDir}`);
    console.error(`  Allowed directories: ${this.allowedDirs.join(", ")}`);
    console.error(`  QSV binary: ${this.qsvBinPath}`);
    if (this.pluginMode) {
      console.error(`  Plugin mode: enabled (relaxed directory security)`);
    }
  }

  /**
   * Get the working directory
   */
  getWorkingDirectory(): string {
    return this.workingDir;
  }

  /**
   * Check if a path is within allowed directories.
   * In plugin mode, auto-adds the directory if not already allowed.
   * Returns true if path is allowed (or was auto-added).
   */
  private isPathAllowed(targetPath: string, isDirectory = false): boolean {
    const allowed = this.allowedDirs.some((allowedDir) => {
      // Direct equality check (both should be canonicalized by now)
      if (targetPath === allowedDir) return true;

      const rel = relative(allowedDir, targetPath);
      if (rel === "") return true;

      // Ensure relative path stays within the allowed directory:
      // 1. Not an absolute path (cross-drive escape on Windows)
      // 2. Doesn't escape to parent ('..')
      // 3. Belt-and-suspenders: reject leading separators (already caught by isAbsolute)
      const isContainedSubpath =
        !isAbsolute(rel) &&
        !rel.startsWith("..") &&
        !rel.startsWith("/") &&
        !rel.startsWith("\\");

      if (isContainedSubpath) return true;

      // Case-insensitive fallback for platforms with case-insensitive filesystems
      // (macOS APFS default, Windows NTFS). Handles /private/var symlink
      // discrepancies on macOS and case-preserving but case-insensitive paths.
      if (process.platform === "darwin" || process.platform === "win32") {
        const lowerAllowed = allowedDir.toLowerCase();
        const lowerTarget = targetPath.toLowerCase();

        const sep = process.platform === "win32" ? "\\" : "/";
        const prefix = lowerAllowed.endsWith(sep) ? lowerAllowed : lowerAllowed + sep;
        if (lowerTarget === lowerAllowed || lowerTarget.startsWith(prefix)) return true;
      }

      return false;
    });

    if (!allowed && this.pluginMode) {
      // In plugin mode, auto-add the directory.
      // If targetPath is itself a directory (from setWorkingDirectory), add it directly.
      // If targetPath is a file (from resolvePath), add its containing directory.
      const dirToAdd = isDirectory ? targetPath : dirname(targetPath);
      if (!this.allowedDirs.includes(dirToAdd)) {
        this.allowedDirs.push(dirToAdd);
      }
      console.error(
        `[Plugin Mode] Auto-added directory to allowedDirs: ${dirToAdd}`,
      );
      return true;
    }

    return allowed;
  }

  /**
   * Set a new working directory
   * Only allows directories within existing allowed directories for security
   */
  setWorkingDirectory(dir: string): void {
    const expanded = expandTilde(dir);
    let newDir = resolve(expanded);
    try {
      newDir = realpathSync(newDir);
    } catch {
      // Keep as-is if directory doesn't exist or inaccessible
    }

    if (!this.isPathAllowed(newDir, true)) {
      throw new Error(
        `Cannot set working directory to ${dir}: outside allowed directories`,
      );
    }

    // In plugin mode, also add the original (pre-realpath) resolved path to handle symlinks
    if (this.pluginMode) {
      const originalResolved = resolve(expanded);
      if (originalResolved !== newDir && !this.allowedDirs.includes(originalResolved)) {
        this.allowedDirs.push(originalResolved);
      }
    }

    this.workingDir = newDir;
    console.error(`Working directory changed to: ${this.workingDir}`);
  }

  /**
   * Resolve a path (absolute or relative to working directory)
   * Canonicalizes the path to resolve symlinks and validates against allowed directories
   */
  async resolvePath(path: string): Promise<string> {
    if (!path) {
      return this.workingDir;
    }

    const expanded = expandTilde(path);
    const resolved = resolve(this.workingDir, expanded);

    // Canonicalize the path to resolve symlinks
    let canonical: string;
    try {
      canonical = await realpath(resolved);
    } catch (error: unknown) {
      // If file doesn't exist yet (e.g., output file), use resolved path
      // but still validate the parent directory exists and is allowed
      const parentDir = join(resolved, "..");
      try {
        canonical = await realpath(parentDir);
        canonical = join(canonical, basename(resolved));
      } catch {
        throw new Error(
          `Path does not exist and parent directory is inaccessible: ${path}`,
        );
      }
    }

    // Security check: ensure canonical path is within allowed directories
    if (!this.isPathAllowed(canonical)) {
      throw new Error(`Access denied: ${path} is outside allowed directories`);
    }

    return canonical;
  }

  /**
   * List tabular data files in a directory as MCP resources
   */
  async listFiles(
    directory?: string,
    recursive: boolean = false,
  ): Promise<{ resources: McpResource[] }> {
    // If directory is undefined or empty, use working directory directly
    const dir = directory ? await this.resolvePath(directory) : this.workingDir;
    const resources: McpResource[] = [];

    try {
      await this.scanDirectory(dir, resources, recursive);

      // Enforce file listing limit
      if (resources.length > config.maxFilesPerListing) {
        const limited = resources.slice(0, config.maxFilesPerListing);
        console.error(
          `Found ${resources.length} tabular data files in ${dir}, ` +
          `but limit is ${config.maxFilesPerListing}. Returning first ${config.maxFilesPerListing} files.`,
        );
        return { resources: limited };
      }

      console.error(`Found ${resources.length} tabular data files in ${dir}`);

      return { resources };
    } catch (error: unknown) {
      console.error(
        `Error listing files in ${directory || "."}: ${error instanceof Error ? error.message : String(error)}`,
      );
      throw error;
    }
  }

  /**
   * Get file extension, handling double extensions like .csv.sz
   */
  private getFileExtension(filename: string): string | null {
    const lower = filename.toLowerCase();

    // Check for double extensions first (e.g., .csv.sz)
    if (lower.endsWith(".csv.sz")) return ".csv.sz";
    if (lower.endsWith(".tsv.sz")) return ".tsv.sz";
    if (lower.endsWith(".tab.sz")) return ".tab.sz";
    if (lower.endsWith(".ssv.sz")) return ".ssv.sz";

    // Check for single extensions
    const ext = extname(filename).toLowerCase();
    return ext || null;
  }

  /**
   * Check if a filename matches temporary/converted file patterns
   * These files should be excluded from the MCP resource list
   */
  private isTemporaryFile(filename: string): boolean {
    // Converted files: *.converted.{hex16}.csv (16-char hex from randomUUID without hyphens)
    if (/\.converted\.[a-f0-9]{16}\.csv$/i.test(filename)) {
      return true;
    }
    // Temporary output files: qsv-output-{uuid}.csv
    if (/^qsv-output-[a-f0-9-]+\.csv$/i.test(filename)) {
      return true;
    }
    // Temp files with .tmp. pattern
    if (filename.includes(".tmp.")) {
      return true;
    }
    return false;
  }

  /**
   * Check if a file format requires conversion to CSV
   */
  needsConversion(filePath: string): boolean {
    const ext = this.getFileExtension(basename(filePath));
    if (!ext) return false;

    return (
      FilesystemResourceProvider.EXCEL_FORMATS.has(ext) ||
      FilesystemResourceProvider.JSONL_FORMATS.has(ext) ||
      FilesystemResourceProvider.PARQUET_FORMATS.has(ext)
    );
  }

  /**
   * Get the conversion command for a file format
   */
  getConversionCommand(filePath: string): string | null {
    const ext = this.getFileExtension(basename(filePath));
    if (!ext) return null;

    if (FilesystemResourceProvider.EXCEL_FORMATS.has(ext)) return "excel";
    if (FilesystemResourceProvider.JSONL_FORMATS.has(ext)) return "jsonl";
    if (FilesystemResourceProvider.PARQUET_FORMATS.has(ext)) return "parquet";

    return null;
  }

  /**
   * Get CSV metadata for a file (row count, columns, etc.)
   * Results are cached for 60 seconds to avoid repeated qsv calls
   * Uses promise-based deduplication to prevent concurrent qsv calls for the same file
   */
  async getFileMetadata(filePath: string): Promise<FileMetadata | null> {
    const cacheKey = filePath;

    // Check cache first
    const cached = this.metadataCache.get(cacheKey);
    if (cached) {
      const age = Date.now() - cached.cachedAt;
      // Cache for 60 seconds
      if (age < METADATA_CACHE_TTL_MS) {
        console.error(
          `[Filesystem] Using cached metadata for ${basename(filePath)} (age: ${Math.round(age / 1000)}s)`,
        );
        return cached;
      }
      // Cache expired, remove it
      this.metadataCache.delete(cacheKey);
    }

    // Check if request already in progress (race condition prevention)
    const inProgress = this.metadataCachePromises.get(cacheKey);
    if (inProgress) {
      console.error(
        `[Filesystem] Deduplicating concurrent metadata request for ${basename(filePath)}`,
      );
      return inProgress;
    }

    // Start new request and track it
    const promise = this.fetchMetadata(filePath);
    this.metadataCachePromises.set(cacheKey, promise);

    try {
      const result = await promise;
      this.metadataCachePromises.delete(cacheKey);
      return result;
    } catch (error: unknown) {
      this.metadataCachePromises.delete(cacheKey);
      throw error;
    }
  }

  /**
   * Fetch metadata from qsv (internal helper, not cached)
   */
  private async fetchMetadata(filePath: string): Promise<FileMetadata | null> {
    const metadata: FileMetadata = {
      rowCount: null,
      columnCount: null,
      columnNames: [],
      hasStatsCache: false,
      cachedAt: Date.now(),
    };

    try {
      // Check for stats cache file
      const statsFile = `${filePath}.stats.csv`;
      try {
        await access(statsFile);
        metadata.hasStatsCache = true;
        console.error(
          `[Filesystem] Stats cache found for ${basename(filePath)}`,
        );
      } catch {
        // Stats cache doesn't exist - not an error
      }

      // Get row count using qsv count (fast - uses index if available)
      try {
        const countResult = await this.runQsvCommand(["count", filePath]);
        const rowCount = parseInt(countResult.trim(), 10);
        if (!isNaN(rowCount)) {
          metadata.rowCount = rowCount;
          console.error(
            `[Filesystem] Row count for ${basename(filePath)}: ${rowCount}`,
          );
        }
      } catch (error: unknown) {
        console.error(
          `[Filesystem] Failed to get row count for ${basename(filePath)}:`,
          error,
        );
      }

      // Get column names using qsv headers --just-names (fast)
      try {
        const headersResult = await this.runQsvCommand([
          "headers",
          "--just-names",
          filePath,
        ]);
        const columnNames = headersResult
          .trim()
          .split("\n")
          .filter((name) => name.length > 0);
        if (columnNames.length > 0) {
          metadata.columnNames = columnNames;
          metadata.columnCount = columnNames.length;
          console.error(
            `[Filesystem] Column count for ${basename(filePath)}: ${columnNames.length}`,
          );
        }
      } catch (error: unknown) {
        console.error(
          `[Filesystem] Failed to get column names for ${basename(filePath)}:`,
          error,
        );
      }

      // Evict oldest entries if cache is full
      if (this.metadataCache.size >= METADATA_CACHE_MAX_SIZE) {
        let oldestKey: string | undefined;
        let oldestTime = Infinity;
        for (const [key, entry] of this.metadataCache) {
          if (entry.cachedAt < oldestTime) {
            oldestTime = entry.cachedAt;
            oldestKey = key;
          }
        }
        if (oldestKey) this.metadataCache.delete(oldestKey);
      }

      // Cache the result
      this.metadataCache.set(filePath, metadata);

      return metadata;
    } catch (error) {
      console.error(
        `[Filesystem] Error getting metadata for ${basename(filePath)}:`,
        error,
      );
      return null;
    }
  }

  /**
   * Execute a qsv command and return its stdout output
   *
   * @param args - Command arguments to pass to qsv binary
   * @returns Promise resolving to the command's stdout output
   * @throws Error if the command fails, times out, or exits with non-zero code
   *
   * @remarks
   * This method spawns the qsv binary specified in config and accumulates
   * stdout/stderr output in memory. For commands that may produce large
   * outputs, consider adding size limits or streaming mechanisms.
   *
   * Includes a 60-second timeout to prevent indefinite hangs if qsv becomes
   * unresponsive. The process is killed if the timeout is exceeded.
   */
  private async runQsvCommand(args: string[]): Promise<string> {
    return runQsvSimple(this.qsvBinPath, args, {
      timeoutMs: 60_000, // 60 seconds for metadata operations
      captureStdout: true,
    });
  }

  /**
   * Recursively scan directory for tabular data files
   */
  private async scanDirectory(
    dir: string,
    resources: McpResource[],
    recursive: boolean,
  ): Promise<void> {
    try {
      const entries = await readdir(dir, { withFileTypes: true });

      for (const entry of entries) {
        const fullPath = join(dir, entry.name);

        if (entry.isDirectory()) {
          if (recursive && !entry.name.startsWith(".")) {
            // Validate subdirectory is within allowed directories before recursing
            // This prevents following symlinks to unauthorized locations
            try {
              await this.resolvePath(relative(this.workingDir, fullPath));
              await this.scanDirectory(fullPath, resources, recursive);
            } catch (error: unknown) {
              // Subdirectory is outside allowed directories or inaccessible
              console.error(`Skipping unauthorized directory: ${fullPath}`);
            }
          }
        } else if (entry.isFile()) {
          const ext = this.getFileExtension(entry.name);
          if (ext && this.allowedExtensions.has(ext)) {
            // Skip temporary/converted files to avoid cluttering the resource list
            if (this.isTemporaryFile(entry.name)) {
              continue;
            }
            const relativePath = relative(this.workingDir, fullPath);
            const uri = this.pathToFileUri(fullPath);

            // Get file metadata
            let description = entry.name;
            try {
              const fileStats = await stat(fullPath);
              const size = formatBytes(fileStats.size);
              const date = fileStats.mtime.toISOString().split("T")[0]; // YYYY-MM-DD
              description = `${entry.name} (${size} ${date})`;
            } catch {
              // If stat fails, use basic description
              description = entry.name;
            }

            resources.push({
              uri,
              name: relativePath,
              description,
              mimeType: this.getMimeType(ext),
            });
          }
        }
      }
    } catch (error: unknown) {
      console.error(`Error scanning directory ${dir}:`, error);
      // Don't throw - just skip inaccessible directories
    }
  }

  /**
   * Convert a filesystem path to a file:/// URI
   * Handles both Windows and Unix paths correctly
   * Returns RFC 8089 compliant file URIs with three slashes
   */
  private pathToFileUri(filePath: string): string {
    // Normalize path separators to forward slashes
    let normalized = filePath.replace(/\\/g, "/");

    // On Windows, convert C:/path to /C:/path
    if (process.platform === "win32" && /^[a-zA-Z]:/.test(normalized)) {
      normalized = "/" + normalized;
    }

    // URL encode each path segment to properly handle special characters
    // Split by /, encode each segment, then rejoin to preserve path structure
    const segments = normalized.split("/");
    const encodedSegments = segments.map((segment) =>
      segment ? encodeURIComponent(segment) : segment,
    );
    const encoded = encodedSegments.join("/");

    // RFC 8089: file URIs use three slashes total (file:// + leading /)
    // Since encoded already starts with /, we use file:// prefix
    return `file://${encoded}`;
  }

  /**
   * Get MIME type for file extension
   */
  private getMimeType(ext: string): string {
    switch (ext.toLowerCase()) {
      // Native CSV formats
      case ".csv":
        return "text/csv";
      case ".tsv":
      case ".tab":
        return "text/tab-separated-values";
      case ".ssv":
        return "text/csv"; // Semicolon-separated

      // Snappy-compressed formats
      case ".csv.sz":
      case ".tsv.sz":
      case ".tab.sz":
      case ".ssv.sz":
        return "application/x-snappy-framed";

      // Excel formats
      case ".xls":
        return "application/vnd.ms-excel";
      case ".xlsx":
        return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
      case ".xlsm":
        return "application/vnd.ms-excel.sheet.macroEnabled.12";
      case ".xlsb":
        return "application/vnd.ms-excel.sheet.binary.macroEnabled.12";

      // OpenDocument Spreadsheet
      case ".ods":
        return "application/vnd.oasis.opendocument.spreadsheet";

      // JSONL/NDJSON
      case ".jsonl":
      case ".ndjson":
        return "application/x-ndjson";

      // Parquet formats
      case ".parquet":
      case ".pq":
        return "application/vnd.apache.parquet";

      default:
        return "application/octet-stream";
    }
  }
}
