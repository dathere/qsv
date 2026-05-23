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
 * Strict semver pattern accepted as a minimum-version floor.
 * Matches MAJOR.MINOR.PATCH with optional pre-release (`-…`) and build
 * (`+…`) metadata, matching what compareVersions() in src/utils.ts already
 * strips and compares. Strings that don't match (e.g. "v20.1.0", "20.1",
 * "20.1.0-") are rejected upstream so a malformed manifest can't silently
 * relax the version-floor check via NaN-becomes-0 coercion in
 * compareVersions().
 */
const SEMVER_PATTERN = /^\d+\.\d+\.\d+(?:-[\w.-]+)?(?:\+[\w.-]+)?$/;

/**
 * Read the minimum required qsv binary version from manifest.json's
 * `_meta["com.dathere.qsv"].minimum_qsv_version` field. This is the single
 * source of truth for the minimum-version floor enforced by both the MCP
 * server (src/config.ts) and the SessionStart hook (scripts/cowork-setup.cjs).
 *
 * Returns null when the manifest is missing, malformed, or contains a value
 * that doesn't parse as strict semver (MAJOR.MINOR.PATCH with optional
 * pre-release/build metadata). Callers must handle null (typically by
 * falling back to "0.0.0" so they don't crash a SessionStart hook on a
 * packaging error).
 * Exported for testing.
 */
export function readMinimumQsvVersionFromManifest(projectRoot: string): string | null {
  try {
    const manifestPath = join(projectRoot, "manifest.json");
    if (!existsSync(manifestPath)) return null;
    const parsed: unknown = JSON.parse(readFileSync(manifestPath, "utf-8"));
    if (typeof parsed !== "object" || parsed === null) return null;
    const meta = (parsed as { _meta?: unknown })._meta;
    if (typeof meta !== "object" || meta === null) return null;
    const qsvMeta = (meta as Record<string, unknown>)["com.dathere.qsv"];
    if (typeof qsvMeta !== "object" || qsvMeta === null) return null;
    const v = (qsvMeta as { minimum_qsv_version?: unknown }).minimum_qsv_version;
    if (typeof v !== "string" || v.length === 0) return null;
    if (!SEMVER_PATTERN.test(v)) return null;
    return v;
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
