/**
 * MCP Filesystem Resource Provider
 *
 * Exposes local CSV files as browsable MCP resources, allowing
 * Claude Desktop to work with local files without uploading them.
 */

import { readdir, stat, readFile } from 'fs/promises';
import { join, resolve, relative, basename, extname } from 'path';
import type { McpResource, McpResourceContent } from './types.js';

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
   * File extensions to include in listings (defaults to CSV-related)
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
}

export class FilesystemResourceProvider {
  private workingDir: string;
  private allowedDirs: string[];
  private allowedExtensions: Set<string>;
  private maxPreviewSize: number;
  private previewLines: number;

  constructor(config: FilesystemConfig = {}) {
    this.workingDir = resolve(config.workingDirectory || process.cwd());
    this.allowedDirs = [
      this.workingDir,
      ...(config.allowedDirectories || []).map(d => resolve(d)),
    ];
    this.allowedExtensions = new Set(
      config.allowedExtensions || ['.csv', '.tsv', '.tab', '.ssv', '.txt', '.sz'],
    );
    this.maxPreviewSize = config.maxPreviewSize || 1024 * 1024; // 1MB
    this.previewLines = config.previewLines || 20;

    console.error(`Filesystem provider initialized:`);
    console.error(`  Working directory: ${this.workingDir}`);
    console.error(`  Allowed directories: ${this.allowedDirs.join(', ')}`);
  }

  /**
   * Get the working directory
   */
  getWorkingDirectory(): string {
    return this.workingDir;
  }

  /**
   * Set a new working directory
   */
  setWorkingDirectory(dir: string): void {
    this.workingDir = resolve(dir);
    if (!this.allowedDirs.includes(this.workingDir)) {
      this.allowedDirs.push(this.workingDir);
    }
    console.error(`Working directory changed to: ${this.workingDir}`);
  }

  /**
   * Resolve a path (absolute or relative to working directory)
   */
  resolvePath(path: string): string {
    if (!path) {
      return this.workingDir;
    }

    const resolved = resolve(this.workingDir, path);

    // Security check: ensure path is within allowed directories
    const isAllowed = this.allowedDirs.some(allowedDir => {
      const rel = relative(allowedDir, resolved);
      return !rel.startsWith('..') && !resolve(allowedDir, rel).startsWith('..');
    });

    if (!isAllowed) {
      throw new Error(
        `Access denied: ${path} is outside allowed directories. ` +
        `Allowed: ${this.allowedDirs.join(', ')}`,
      );
    }

    return resolved;
  }

  /**
   * List CSV files in a directory as MCP resources
   */
  async listFiles(
    directory?: string,
    recursive: boolean = false,
  ): Promise<{ resources: McpResource[] }> {
    const dir = this.resolvePath(directory || '.');
    const resources: McpResource[] = [];

    try {
      await this.scanDirectory(dir, resources, recursive);

      console.error(`Found ${resources.length} CSV files in ${dir}`);

      return { resources };
    } catch (error) {
      console.error(`Error listing files in ${dir}:`, error);
      throw error;
    }
  }

  /**
   * Recursively scan directory for CSV files
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
          if (recursive && !entry.name.startsWith('.')) {
            await this.scanDirectory(fullPath, resources, recursive);
          }
        } else if (entry.isFile()) {
          const ext = extname(entry.name).toLowerCase();
          if (this.allowedExtensions.has(ext)) {
            const relativePath = relative(this.workingDir, fullPath);
            const uri = `file:///${fullPath.replace(/\\/g, '/')}`;

            resources.push({
              uri,
              name: relativePath,
              description: `CSV file: ${entry.name}`,
              mimeType: this.getMimeType(ext),
            });
          }
        }
      }
    } catch (error) {
      console.error(`Error scanning directory ${dir}:`, error);
      // Don't throw - just skip inaccessible directories
    }
  }

  /**
   * Get file info and preview as MCP resource content
   */
  async getFileContent(uri: string): Promise<McpResourceContent | null> {
    try {
      // Parse file:/// URI
      const filePath = uri.replace(/^file:\/\/\//, '');
      const resolved = this.resolvePath(filePath);

      // Get file stats
      const stats = await stat(resolved);

      if (!stats.isFile()) {
        throw new Error(`Not a file: ${filePath}`);
      }

      const ext = extname(resolved).toLowerCase();
      const mimeType = this.getMimeType(ext);

      // Generate preview if file is small enough
      let preview = '';
      if (stats.size <= this.maxPreviewSize) {
        const content = await readFile(resolved, 'utf-8');
        const lines = content.split('\n').slice(0, this.previewLines);
        preview = lines.join('\n');

        if (content.split('\n').length > this.previewLines) {
          preview += `\n... (${content.split('\n').length - this.previewLines} more lines)`;
        }
      } else {
        preview = `File too large for preview (${this.formatBytes(stats.size)})`;
      }

      const relativePath = relative(this.workingDir, resolved);
      const absolutePath = resolved;

      const info = {
        file: {
          name: basename(resolved),
          path: relativePath,
          absolutePath,
          size: stats.size,
          sizeFormatted: this.formatBytes(stats.size),
          modified: stats.mtime.toISOString(),
          extension: ext,
        },
        preview,
        usage: {
          description: 'Use this file path in qsv commands',
          examples: [
            `qsv_stats with input_file: "${relativePath}"`,
            `qsv_headers with input_file: "${absolutePath}"`,
            `qsv_frequency with input_file: "${relativePath}" and column name`,
          ],
        },
      };

      return {
        uri,
        mimeType,
        text: JSON.stringify(info, null, 2),
      };
    } catch (error) {
      console.error(`Error reading file ${uri}:`, error);
      return null;
    }
  }

  /**
   * Get MIME type for file extension
   */
  private getMimeType(ext: string): string {
    switch (ext.toLowerCase()) {
      case '.csv':
        return 'text/csv';
      case '.tsv':
      case '.tab':
        return 'text/tab-separated-values';
      case '.txt':
        return 'text/plain';
      case '.sz':
        return 'application/x-snappy-framed';
      default:
        return 'text/plain';
    }
  }

  /**
   * Format bytes to human-readable string
   */
  private formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
  }
}
