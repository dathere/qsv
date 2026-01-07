/**
 * Update Checker for QSV MCP Server
 *
 * Provides mechanisms for:
 * 1. Detecting qsv binary version changes
 * 2. Checking if skill definitions are stale
 * 3. Checking for MCP server updates
 * 4. Auto-regenerating skills when needed
 */

import { spawn } from 'child_process';
import { readFileSync, writeFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

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
}

const DEFAULT_CONFIG: UpdateConfig = {
  autoRegenerateSkills: false, // Conservative default
  checkForUpdatesOnStartup: true,
  notifyOnUpdatesAvailable: true,
  githubRepo: 'dathere/qsv'
};

export class UpdateChecker {
  private qsvBinaryPath: string;
  private skillsDir: string;
  private versionFilePath: string;
  private config: UpdateConfig;

  constructor(qsvBinaryPath: string = 'qsv', skillsDir?: string, config?: Partial<UpdateConfig>) {
    this.qsvBinaryPath = qsvBinaryPath;
    this.skillsDir = skillsDir || join(__dirname, '../qsv');
    this.versionFilePath = join(dirname(this.skillsDir), '.qsv-mcp-versions.json');
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  /**
   * Get current qsv binary version
   */
  async getQsvBinaryVersion(): Promise<string> {
    return new Promise((resolve, reject) => {
      const child = spawn(this.qsvBinaryPath, ['--version']);
      let output = '';

      child.stdout.on('data', (data) => {
        output += data.toString();
      });

      child.on('close', (code) => {
        if (code !== 0) {
          reject(new Error(`qsv --version exited with code ${code}`));
          return;
        }

        // Parse version from output like "qsv 0.132.0"
        const match = output.match(/qsv\s+(\d+\.\d+\.\d+)/);
        if (match) {
          resolve(match[1]);
        } else {
          reject(new Error(`Could not parse qsv version from: ${output}`));
        }
      });

      child.on('error', (error) => {
        reject(new Error(`Failed to execute qsv: ${error.message}`));
      });
    });
  }

  /**
   * Get version that skills were generated with (from skill JSON files)
   */
  getSkillsVersion(): string {
    try {
      // Read version from any skill file (they all have the same version)
      const sampleSkillPath = join(this.skillsDir, 'qsv-stats.json');
      if (!existsSync(sampleSkillPath)) {
        return 'unknown';
      }

      const skill = JSON.parse(readFileSync(sampleSkillPath, 'utf-8'));
      return skill.version || 'unknown';
    } catch (error) {
      console.error('[UpdateChecker] Failed to read skills version:', error);
      return 'unknown';
    }
  }

  /**
   * Get MCP server version (from package.json)
   */
  getMcpServerVersion(): string {
    try {
      const packageJsonPath = join(__dirname, '../package.json');
      const fallbackPath = join(__dirname, '../../package.json');

      const path = existsSync(packageJsonPath) ? packageJsonPath : fallbackPath;
      const packageJson = JSON.parse(readFileSync(path, 'utf-8'));
      return packageJson.version || 'unknown';
    } catch (error) {
      console.error('[UpdateChecker] Failed to read MCP server version:', error);
      return 'unknown';
    }
  }

  /**
   * Load stored version information
   */
  loadVersionInfo(): VersionInfo | null {
    try {
      if (!existsSync(this.versionFilePath)) {
        return null;
      }
      return JSON.parse(readFileSync(this.versionFilePath, 'utf-8'));
    } catch (error) {
      console.error('[UpdateChecker] Failed to load version info:', error);
      return null;
    }
  }

  /**
   * Save version information
   */
  saveVersionInfo(info: VersionInfo): void {
    try {
      writeFileSync(this.versionFilePath, JSON.stringify(info, null, 2), 'utf-8');
    } catch (error) {
      console.error('[UpdateChecker] Failed to save version info:', error);
    }
  }

  /**
   * Check for available updates from GitHub releases
   */
  async checkGitHubReleases(): Promise<string | null> {
    try {
      // Use Node.js built-in fetch (available in Node 18+)
      const response = await fetch(
        `https://api.github.com/repos/${this.config.githubRepo}/releases/latest`,
        {
          headers: {
            'Accept': 'application/vnd.github.v3+json',
            'User-Agent': 'qsv-mcp-server'
          }
        }
      );

      if (!response.ok) {
        console.error('[UpdateChecker] GitHub API returned:', response.status);
        return null;
      }

      const data = await response.json() as { tag_name: string };
      // Tag format is typically "v0.132.0" or "0.132.0"
      const version = data.tag_name.replace(/^v/, '');
      return version;
    } catch (error) {
      console.error('[UpdateChecker] Failed to check GitHub releases:', error);
      return null;
    }
  }

  /**
   * Compare semantic versions (simple implementation)
   */
  private compareVersions(v1: string, v2: string): number {
    const parts1 = v1.split('.').map(Number);
    const parts2 = v2.split('.').map(Number);

    for (let i = 0; i < Math.max(parts1.length, parts2.length); i++) {
      const part1 = parts1[i] || 0;
      const part2 = parts2[i] || 0;

      if (part1 > part2) return 1;
      if (part1 < part2) return -1;
    }

    return 0;
  }

  /**
   * Perform comprehensive update check
   */
  async checkForUpdates(): Promise<UpdateCheckResult> {
    const recommendations: string[] = [];

    // Get current versions
    const currentQsvVersion = await this.getQsvBinaryVersion();
    const skillsVersion = this.getSkillsVersion();
    const mcpServerVersion = this.getMcpServerVersion();

    // Check if skills are outdated
    const skillsOutdated = currentQsvVersion !== skillsVersion &&
                           skillsVersion !== 'unknown' &&
                           currentQsvVersion !== 'unknown';

    if (skillsOutdated) {
      const comparison = this.compareVersions(currentQsvVersion, skillsVersion);
      if (comparison > 0) {
        recommendations.push(
          `⚠️  qsv binary (${currentQsvVersion}) is newer than skills (${skillsVersion})`
        );
        recommendations.push(
          `   Run: cargo run --bin qsv-skill-gen --features all_features`
        );
        recommendations.push(
          `   Then restart the MCP server`
        );
      } else if (comparison < 0) {
        recommendations.push(
          `ℹ️  qsv binary (${currentQsvVersion}) is older than skills (${skillsVersion})`
        );
        recommendations.push(
          `   Consider updating qsv: qsv --update`
        );
      }
    }

    // Check for latest qsv release on GitHub
    let latestQsvVersion: string | null = null;
    try {
      latestQsvVersion = await this.checkGitHubReleases();
      if (latestQsvVersion && this.compareVersions(latestQsvVersion, currentQsvVersion) > 0) {
        recommendations.push(
          `🆕 New qsv release available: ${latestQsvVersion} (you have ${currentQsvVersion})`
        );
        recommendations.push(
          `   Update with: qsv --update`
        );
      }
    } catch (error) {
      // Non-critical error, continue
    }

    // Save current state
    this.saveVersionInfo({
      qsvBinaryVersion: currentQsvVersion,
      skillsGeneratedWithVersion: skillsVersion,
      mcpServerVersion: mcpServerVersion,
      lastChecked: new Date().toISOString()
    });

    return {
      qsvBinaryOutdated: latestQsvVersion ? this.compareVersions(latestQsvVersion, currentQsvVersion) > 0 : false,
      skillsOutdated,
      mcpServerOutdated: false, // MCP server updates handled via npm
      currentQsvVersion,
      skillsVersion,
      mcpServerVersion,
      latestMcpServerVersion: undefined,
      recommendations
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

    console.error('[UpdateChecker] Auto-regenerating skills...');

    return new Promise((resolve) => {
      // Find the qsv repository root (should be ../.. from .claude/skills)
      const repoRoot = join(this.skillsDir, '../..');

      const child = spawn(
        'cargo',
        ['run', '--bin', 'qsv-skill-gen', '--features', 'all_features'],
        {
          cwd: repoRoot,
          stdio: 'inherit'
        }
      );

      child.on('close', (code) => {
        if (code === 0) {
          console.error('[UpdateChecker] ✅ Skills regenerated successfully');
          resolve(true);
        } else {
          console.error('[UpdateChecker] ❌ Failed to regenerate skills (exit code:', code, ')');
          resolve(false);
        }
      });

      child.on('error', (error) => {
        console.error('[UpdateChecker] ❌ Failed to spawn cargo:', error);
        resolve(false);
      });
    });
  }

  /**
   * Quick check - only compares local versions (no network calls)
   */
  async quickCheck(): Promise<{ skillsOutdated: boolean; versions: VersionInfo }> {
    const currentQsvVersion = await this.getQsvBinaryVersion();
    const skillsVersion = this.getSkillsVersion();
    const mcpServerVersion = this.getMcpServerVersion();

    const skillsOutdated = currentQsvVersion !== skillsVersion &&
                           skillsVersion !== 'unknown' &&
                           currentQsvVersion !== 'unknown';

    return {
      skillsOutdated,
      versions: {
        qsvBinaryVersion: currentQsvVersion,
        skillsGeneratedWithVersion: skillsVersion,
        mcpServerVersion,
        lastChecked: new Date().toISOString()
      }
    };
  }
}

/**
 * Environment variable configuration
 */
export function getUpdateConfigFromEnv(): Partial<UpdateConfig> {
  return {
    autoRegenerateSkills: process.env.QSV_MCP_AUTO_REGENERATE_SKILLS === 'true',
    checkForUpdatesOnStartup: process.env.QSV_MCP_CHECK_UPDATES_ON_STARTUP !== 'false',
    notifyOnUpdatesAvailable: process.env.QSV_MCP_NOTIFY_UPDATES !== 'false',
    githubRepo: process.env.QSV_MCP_GITHUB_REPO || 'dathere/qsv'
  };
}
