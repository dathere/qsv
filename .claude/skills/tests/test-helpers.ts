/**
 * Shared test helpers for qsv MCP Server tests
 *
 * Consolidates common patterns found across test files:
 * - Test directory creation/cleanup
 * - CSV file creation
 * - qsv availability check
 */

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
