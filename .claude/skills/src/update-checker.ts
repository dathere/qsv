/**
 * Update Checker for QSV MCP Server
 *
 * Provides mechanisms for:
 * 1. Detecting qsv binary version changes
 * 2. Checking if skill definitions are stale
 * 3. Checking for MCP server updates
 * 4. Auto-regenerating skills when needed
 */

import { spawn } from "child_process";
import { readFileSync, writeFileSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { compareVersions } from "./utils.js";
import { VERSION } from "./version.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

export interface VersionInfo {
  qsvBinaryVersion: string;
  skillsGeneratedWithVersion: string;
  mcpServerVersion: string;
  lastChecked: string;
}

export interface UpdateCheckResult {
  qsvBinaryOutdated: boolean;
  skillsOutdated: boolean;
  mcpServerOutdated: boolean;
  currentQsvVersion: string;
  skillsVersion: string;
  mcpServerVersion: string;
  latestMcpServerVersion?: string;
  recommendations: string[];
}

export interface UpdateConfig {
  autoRegenerateSkills: boolean;
  checkForUpdatesOnStartup: boolean;
  notifyOnUpdatesAvailable: boolean;
  githubRepo: string;
  isExtensionMode?: boolean; // Desktop extension mode - skip MCP server version checks
}

const DEFAULT_CONFIG: UpdateConfig = {
  autoRegenerateSkills: false, // Conservative default
  checkForUpdatesOnStartup: true,
  notifyOnUpdatesAvailable: true,
  githubRepo: "dathere/qsv",
  isExtensionMode: false,
};

export class UpdateChecker {
  private qsvBinaryPath: string;
  private skillsDir: string;
  private versionFilePath: string;
  private config: UpdateConfig;

  constructor(
    qsvBinaryPath: string = "qsv",
    skillsDir?: string,
    config?: Partial<UpdateConfig>,
  ) {
    this.qsvBinaryPath = qsvBinaryPath;
    this.skillsDir = skillsDir || join(__dirname, "../qsv");
    this.versionFilePath = join(
      dirname(this.skillsDir),
      ".qsv-mcp-versions.json",
    );
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Get current qsv binary version
   */
  async getQsvBinaryVersion(): Promise<string> {
    const SPAWN_TIMEOUT_MS = 30_000;
    return new Promise((resolve, reject) => {
      const child = spawn(this.qsvBinaryPath, ["--version"]);
      let output = "";
      let settled = false;

      const timer = setTimeout(() => {
        if (!settled) {
          settled = true;
          child.kill("SIGTERM");
          reject(new Error(`qsv --version timed out after ${SPAWN_TIMEOUT_MS}ms`));
        }
      }, SPAWN_TIMEOUT_MS);

      child.stdout.on("data", (data) => {
        output += data.toString();
      });

      child.on("close", (code) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        if (code !== 0) {
          reject(new Error(`qsv --version exited with code ${code}`));
          return;
        }

        // Parse version from output like "qsv 0.132.0", "qsvmcp 16.1.0-mimalloc", etc.
        // Handle binary variants (qsv, qsvmcp, qsvlite, qsvdp), extra text, and pre-release tags
        const match = output.match(/qsv\w*\s+(\d+\.\d+\.\d+)(?:-[\w.]+)?/);
        if (match) {
          // Only use the main version number (ignore pre-release tags for now)
          resolve(match[1]);
        } else {
          reject(new Error(`Could not parse qsv version from: ${output}`));
        }
      });

      child.on("error", (error) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        reject(new Error(`Failed to execute qsv: ${error.message}`));
      });
    });
  }

  /**
   * Get version that skills were generated with (from skill JSON files)
   */
  getSkillsVersion(): string {
    // Try multiple skill files as fallbacks for resilience
    const skillFilesToTry = [
      "qsv-stats.json",
      "qsv-select.json",
      "qsv-count.json",
      "qsv-search.json",
    ];

    for (const skillFile of skillFilesToTry) {
      try {
        const skillPath = join(this.skillsDir, skillFile);
        if (!existsSync(skillPath)) {
          continue;
        }

        const skill = JSON.parse(readFileSync(skillPath, "utf-8"));
        if (skill.version) {
          return skill.version;
        }
      } catch (error: unknown) {
        // Try next file
        continue;
      }
    }

    console.warn(
      "[UpdateChecker] Could not determine skills version from any skill file",
    );
    return "unknown";
  }

  /**
   * Get MCP server version (delegates to version.ts)
   */
  getMcpServerVersion(): string {
    return VERSION;
  }

  /**
   * Load stored version information
   */
  loadVersionInfo(): VersionInfo | null {
    try {
      if (!existsSync(this.versionFilePath)) {
        return null;
      }
      return JSON.parse(readFileSync(this.versionFilePath, "utf-8"));
    } catch (error: unknown) {
      console.error("[UpdateChecker] Failed to load version info:", error);
      return null;
    }
  }

  /**
   * Save version information
   */
  saveVersionInfo(info: VersionInfo): void {
    try {
      writeFileSync(
        this.versionFilePath,
        JSON.stringify(info, null, 2),
        "utf-8",
      );
    } catch (error) {
      console.error("[UpdateChecker] Failed to save version info:", error);
      console.warn(
        "[UpdateChecker] WARNING: Version info could not be persisted at",
        this.versionFilePath,
        "- update checks may be repeated or version tracking may be inaccurate.",
      );
    }
  }

  /**
   * Check for available updates from GitHub releases
   */
  async checkGitHubReleases(signal?: AbortSignal): Promise<string | null> {
    try {
      // Use Node.js built-in fetch (available in Node 18+)
      const response = await fetch(
        `https://api.github.com/repos/${this.config.githubRepo}/releases/latest`,
        {
          headers: {
            Accept: "application/vnd.github.v3+json",
            "User-Agent": "qsv-mcp-server",
          },
          signal,
        },
      );

      if (!response.ok) {
        console.error("[UpdateChecker] GitHub API returned:", response.status);
        return null;
      }

      const data: unknown = await response.json();
      if (
        !data ||
        typeof data !== "object" ||
        data === null ||
        typeof (data as { tag_name?: unknown }).tag_name !== "string"
      ) {
        console.error(
          "[UpdateChecker] GitHub API response missing valid tag_name field",
        );
        return null;
      }
      const tagName = (data as { tag_name: string }).tag_name;
      // Tag format is typically "v0.132.0" or "0.132.0"
      const version = tagName.replace(/^v/, "");
      return version;
    } catch (error: unknown) {
      console.error("[UpdateChecker] Failed to check GitHub releases:", error);
      return null;
    }
  }

  /**
   * Perform comprehensive update check
   */
  async checkForUpdates(signal?: AbortSignal): Promise<UpdateCheckResult> {
    const recommendations: string[] = [];

    // Get current versions
    const currentQsvVersion = await this.getQsvBinaryVersion();
    const skillsVersion = this.getSkillsVersion();
    // Skip MCP server version check in extension mode (managed by Claude Desktop)
    const mcpServerVersion = this.config.isExtensionMode
      ? "extension"
      : this.getMcpServerVersion();

    // Check if skills are outdated
    const skillsOutdated =
      currentQsvVersion !== skillsVersion &&
      skillsVersion !== "unknown" &&
      currentQsvVersion !== "unknown";

    if (skillsOutdated) {
      // Note: compareVersions returns NaN for unparseable versions.
      // NaN > 0 and NaN < 0 are both false, so unparseable versions
      // are safely ignored (no recommendations emitted).
      const comparison = compareVersions(currentQsvVersion, skillsVersion);
      if (comparison > 0) {
        recommendations.push(
          `⚠️  qsv binary (${currentQsvVersion}) is newer than skills (${skillsVersion})`,
        );
        recommendations.push(`   Run: qsv --update-mcp-skills`);
        recommendations.push(`   Then restart the MCP server`);
      } else if (comparison < 0) {
        recommendations.push(
          `ℹ️  qsv binary (${currentQsvVersion}) is older than skills (${skillsVersion})`,
        );
        recommendations.push(`   Consider updating qsv: qsv --update`);
      }
    }

    // Check for latest qsv release on GitHub
    let latestQsvVersion: string | null = null;
    try {
      latestQsvVersion = await this.checkGitHubReleases(signal);
      if (
        latestQsvVersion &&
        compareVersions(latestQsvVersion, currentQsvVersion) > 0
      ) {
        recommendations.push(
          `🆕 New qsv release available: ${latestQsvVersion} (you have ${currentQsvVersion})`,
        );
        recommendations.push(`   Update with: qsv --update`);
      }
    } catch (error: unknown) {
      // Non-critical error, continue
      console.warn("[UpdateChecker] GitHub release check failed:", error);
    }

    // Save current state
    this.saveVersionInfo({
      qsvBinaryVersion: currentQsvVersion,
      skillsGeneratedWithVersion: skillsVersion,
      mcpServerVersion: mcpServerVersion,
      lastChecked: new Date().toISOString(),
    });

    return {
      qsvBinaryOutdated: latestQsvVersion
        ? compareVersions(latestQsvVersion, currentQsvVersion) > 0
        : false,
      skillsOutdated,
      mcpServerOutdated: false, // MCP server updates handled via npm
      currentQsvVersion,
      skillsVersion,
      mcpServerVersion,
      latestMcpServerVersion: undefined,
      recommendations,
    };
  }

  /**
   * Attempt to auto-regenerate skills
   * Returns true if successful, false otherwise
   */
  async autoRegenerateSkills(): Promise<boolean> {
    if (!this.config.autoRegenerateSkills) {
      return false;
    }

    console.error("[UpdateChecker] Auto-regenerating skills...");

    const REGEN_TIMEOUT_MS = 60_000;
    return new Promise((resolve) => {
      // Use qsv binary directly with --update-mcp-skills flag
      // This is much simpler and doesn't require Rust toolchain
      const child = spawn(this.qsvBinaryPath, ["--update-mcp-skills"], {
        stdio: "inherit",
      });
      let settled = false;

      const timer = setTimeout(() => {
        if (!settled) {
          settled = true;
          child.kill("SIGTERM");
          console.error(`[UpdateChecker] ❌ Skills regeneration timed out after ${REGEN_TIMEOUT_MS}ms`);
          resolve(false);
        }
      }, REGEN_TIMEOUT_MS);

      child.on("close", (code) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        if (code === 0) {
          console.error("[UpdateChecker] ✅ Skills regenerated successfully");
          resolve(true);
        } else {
          console.error(
            "[UpdateChecker] ❌ Failed to regenerate skills (exit code:",
            code,
            ")",
          );
          resolve(false);
        }
      });

      child.on("error", (error) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        console.error("[UpdateChecker] ❌ Failed to spawn qsv:", error);
        resolve(false);
      });
    });
  }

  /**
   * Quick check - only compares local versions (no network calls)
   */
  async quickCheck(): Promise<{
    skillsOutdated: boolean;
    versions: VersionInfo;
  }> {
    const currentQsvVersion = await this.getQsvBinaryVersion();
    const skillsVersion = this.getSkillsVersion();
    // Skip MCP server version check in extension mode (managed by Claude Desktop)
    const mcpServerVersion = this.config.isExtensionMode
      ? "extension"
      : this.getMcpServerVersion();

    const skillsOutdated =
      currentQsvVersion !== skillsVersion &&
      skillsVersion !== "unknown" &&
      currentQsvVersion !== "unknown";

    return {
      skillsOutdated,
      versions: {
        qsvBinaryVersion: currentQsvVersion,
        skillsGeneratedWithVersion: skillsVersion,
        mcpServerVersion,
        lastChecked: new Date().toISOString(),
      },
    };
  }
}

/**
 * Environment variable configuration
 */
export function getUpdateConfigFromEnv(): Partial<UpdateConfig> {
  return {
    autoRegenerateSkills: process.env.QSV_MCP_AUTO_REGENERATE_SKILLS === "true",
    checkForUpdatesOnStartup:
      process.env.QSV_MCP_CHECK_UPDATES_ON_STARTUP !== "false",
    notifyOnUpdatesAvailable: process.env.QSV_MCP_NOTIFY_UPDATES !== "false",
    githubRepo: process.env.QSV_MCP_GITHUB_REPO || "dathere/qsv",
    isExtensionMode: process.env.MCPB_EXTENSION_MODE === "true",
  };
}
