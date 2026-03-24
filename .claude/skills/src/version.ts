/**
 * Version Management
 *
 * Exports the version from package.json and validates it matches manifest.json.
 * Both files must stay in sync — package.json is the npm/build version,
 * manifest.json is the Claude Desktop extension/plugin version.
 */

import { readFileSync, existsSync } from "fs";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Resolve the project root directory (parent of dist/).
 * Handles both production (dist/version.js) and test (dist/src/version.js) layouts.
 * Exported for testing.
 */
export function resolveProjectRoot(): string {
  const productionRoot = join(__dirname, "..");
  const testRoot = join(__dirname, "../..");

  if (existsSync(join(productionRoot, "package.json"))) {
    return productionRoot;
  }
  if (existsSync(join(testRoot, "package.json"))) {
    return testRoot;
  }
  return productionRoot; // fallback
}

/**
 * Read a version string from a JSON file.
 * Returns null if the file doesn't exist, can't be parsed, or has no version field.
 * Exported for testing.
 */
export function readVersionFromJson(filePath: string): string | null {
  try {
    if (!existsSync(filePath)) return null;
    const parsed: unknown = JSON.parse(readFileSync(filePath, "utf-8"));
    if (typeof parsed === "object" && parsed !== null && "version" in parsed) {
      const { version } = parsed as { version: unknown };
      if (typeof version === "string" && version.length > 0) return version;
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Get version from package.json and validate it matches manifest.json.
 * Logs a warning at startup if the versions diverge.
 */
function getVersion(): string {
  const projectRoot = resolveProjectRoot();
  const packageJsonPath = join(projectRoot, "package.json");
  const manifestJsonPath = join(projectRoot, "manifest.json");

  const packageVersion = readVersionFromJson(packageJsonPath);
  if (!packageVersion) {
    console.error("[Version] Failed to read version from package.json");
    return "0.0.0";
  }

  // Validate manifest.json version matches package.json.
  // A missing manifest.json is normal in dev/test layouts (dist/src/version.js
  // resolves to the repo root where manifest.json lives one level up from the
  // test build output). Only warn when it's genuinely unexpected.
  const manifestVersion = readVersionFromJson(manifestJsonPath);
  if (!manifestVersion) {
    // Suppress noise during test runs; in production (dist/version.js next to
    // manifest.json) a missing manifest is a real packaging error worth logging.
    if (!process.env.NODE_TEST) {
      console.error("[Version] manifest.json not found or has no version field");
    }
  } else if (manifestVersion !== packageVersion) {
    console.error(
      `[Version] ⚠️  VERSION MISMATCH: package.json=${packageVersion}, manifest.json=${manifestVersion}. ` +
      `These must be kept in sync. Update the lagging file before publishing.`,
    );
  }

  return packageVersion;
}

export const VERSION = getVersion();
