/**
 * Version Management
 *
 * Exports the version from package.json to ensure consistency across the codebase.
 */

import { readFileSync, existsSync } from "fs";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * Get version from package.json
 */
function getVersion(): string {
  try {
    // Try multiple possible locations for package.json
    // 1. When built for production: dist/version.js -> ../package.json
    // 2. When built for tests: dist/src/version.js -> ../../package.json
    const productionPath = join(__dirname, "../package.json");
    const testPath = join(__dirname, "../../package.json");

    let packageJsonPath = productionPath;
    if (!existsSync(productionPath) && existsSync(testPath)) {
      packageJsonPath = testPath;
    }

    const parsed: unknown = JSON.parse(readFileSync(packageJsonPath, "utf-8"));
    if (typeof parsed === "object" && parsed !== null && "version" in parsed) {
      const { version } = parsed as { version: unknown };
      if (typeof version === "string" && version.length > 0) return version;
    }
    return "0.0.0";
  } catch (error) {
    console.error("[Version] Failed to read version from package.json:", error);
    return "0.0.0";
  }
}

export const VERSION = getVersion();
