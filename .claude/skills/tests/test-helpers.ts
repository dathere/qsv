/**
 * Shared test helpers for qsv MCP Server tests
 *
 * Consolidates common patterns found across test files:
 * - Test directory creation/cleanup
 * - CSV file creation
 * - qsv availability check
 */

import { execFileSync } from "child_process";
import { writeFile, mkdtemp, rm, realpath } from "fs/promises";
import { join } from "path";
import { tmpdir } from "os";
import { config } from "../src/config.js";

/**
 * Whether a valid qsv binary is available for integration tests.
 * Tests that require qsv should skip when this is false.
 *
 * Usage: test("name", { skip: !QSV_AVAILABLE }, async () => { ... })
 */
export const QSV_AVAILABLE: boolean = config.qsvValidation.valid;

/**
 * Whether the detected qsv binary supports `to parquet`.
 * Checks both that the `to` command exists AND that it supports the `parquet`
 * subcommand (added after qsv 18.0.0). Older releases have `to` but only for
 * PostgreSQL/SQLite/XLSX/ODS/DataPackage — not Parquet.
 *
 * Usage: test("name", { skip: !TO_PARQUET_AVAILABLE }, async () => { ... })
 */
export const TO_PARQUET_AVAILABLE: boolean = (() => {
  if (!QSV_AVAILABLE || !config.qsvValidation.availableCommands?.includes("to")) {
    return false;
  }
  try {
    const help = execFileSync(config.qsvBinPath, ["to", "--help"], {
      encoding: "utf8",
      timeout: 5000,
    });
    return /\bparquet\b/i.test(help);
  } catch {
    return false;
  }
})();

/**
 * Create a temporary test directory with a unique name.
 * Uses mkdtemp for guaranteed uniqueness and realpath for
 * Windows 8.3 short path compatibility.
 */
export async function createTestDir(prefix = "qsv-test"): Promise<string> {
  const dir = await mkdtemp(join(tmpdir(), `${prefix}-`));
  return realpath(dir);
}

/**
 * Clean up a test directory, ignoring errors.
 */
export async function cleanupTestDir(dir: string): Promise<void> {
  try {
    await rm(dir, { recursive: true, force: true });
  } catch {
    // Ignore cleanup errors
  }
}

/**
 * Create a test CSV file with the given content.
 * Returns the absolute path to the created file.
 */
export async function createTestCSV(
  dir: string,
  filename: string,
  content: string,
): Promise<string> {
  const filepath = join(dir, filename);
  await writeFile(filepath, content, "utf8");
  return filepath;
}
