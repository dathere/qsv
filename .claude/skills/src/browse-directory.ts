/**
 * Directory scanning logic for the qsv_browse_directory tool.
 *
 * Extracted from QsvMcpServer.handleBrowseDirectory so it can be
 * unit-tested independently of the full server instance.
 */

import { dirname, extname, join } from "node:path";
import { readdir, stat as fsStat } from "node:fs/promises";

/** Tabular file extensions recognised by the directory scanner. */
export const TABULAR_EXTS = new Set([
  ".csv", ".tsv", ".tab", ".ssv", ".parquet", ".pq", ".pqt",
  ".jsonl", ".ndjson", ".json", ".xlsx", ".xls",
  ".xlsm", ".xlsb", ".ods",
]);

export interface SubdirectoryInfo {
  name: string;
  path: string;
  tabularFileCount: number;
  subdirCount: number;
}

export interface BrowseResult {
  currentPath: string;
  parent: string | null;
  subdirectories: SubdirectoryInfo[];
  tabularFileCount: number;
}

/**
 * Scan a directory and return its visible subdirectories (with tabular file
 * and sub-directory counts) plus the number of tabular files at the top level.
 *
 * Hidden entries (names starting with ".") are skipped.
 *
 * Throws if `targetDir` does not exist, is not accessible, or is not a directory.
 */
export async function scanDirectory(targetDir: string): Promise<BrowseResult> {
  const dirStat = await fsStat(targetDir);
  if (!dirStat.isDirectory()) {
    throw new Error(`"${targetDir}" is not a directory.`);
  }

  const entries = await readdir(targetDir, { withFileTypes: true });
  const subdirectories: SubdirectoryInfo[] = [];
  let tabularFileCount = 0;

  for (const entry of entries) {
    if (entry.name.startsWith(".")) continue;

    if (entry.isDirectory()) {
      let subTabular = 0;
      let subDirs = 0;
      try {
        const subEntries = await readdir(join(targetDir, entry.name), { withFileTypes: true });
        for (const sub of subEntries) {
          if (sub.name.startsWith(".")) continue;
          if (sub.isDirectory()) subDirs++;
          else if (TABULAR_EXTS.has(extname(sub.name).toLowerCase())) subTabular++;
        }
      } catch {
        // Permission denied or other error — just skip counts
      }

      subdirectories.push({
        name: entry.name,
        path: join(targetDir, entry.name),
        tabularFileCount: subTabular,
        subdirCount: subDirs,
      });
    } else if (TABULAR_EXTS.has(extname(entry.name).toLowerCase())) {
      tabularFileCount++;
    }
  }

  subdirectories.sort((a, b) => a.name.localeCompare(b.name, undefined, { sensitivity: "base" }));

  const parent = dirname(targetDir);
  const hasParent = parent !== targetDir;

  return {
    currentPath: targetDir,
    parent: hasParent ? parent : null,
    subdirectories,
    tabularFileCount,
  };
}
