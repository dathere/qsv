/**
 * Platform-specific qsv installation logic
 *
 * Downloads the qsv binary from GitHub Releases (latest), extracts only
 * qsvmcp, and installs it to a discoverable location.
 * Falls back to manual instructions for unsupported platforms.
 */

import { execFileSync } from "child_process";
import {
  accessSync,
  chmodSync,
  constants,
  copyFileSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  rmSync,
  writeFileSync,
} from "fs";
import { join } from "path";
import { tmpdir } from "os";

export interface InstallResult {
  success: boolean;
  method: "direct-download" | "manual";
  binaryPath?: string;
  output?: string;
  error?: string;
  instructions?: string;
}

/**
 * Asset filename suffixes for supported platforms.
 * These match the GitHub release asset naming convention:
 *   qsv-{version}-{suffix}.zip
 */
export const ASSET_SUFFIXES: Record<string, string> = {
  "darwin-arm64": "aarch64-apple-darwin",
  "win32-x64": "x86_64-pc-windows-msvc",
  "win32-arm64": "aarch64-pc-windows-msvc",
};

/** GitHub API URL for the latest qsv release */
const GITHUB_LATEST_RELEASE_URL = "https://api.github.com/repos/dathere/qsv/releases/latest";

/**
 * Get the asset filename suffix for the current platform, or null if unsupported.
 */
export function getAssetSuffix(): string | null {
  const key = `${process.platform}-${process.arch}`;
  return ASSET_SUFFIXES[key] ?? null;
}

/**
 * Fetch the download URL for the latest qsv release for the current platform.
 * Uses the GitHub Releases API to find the matching asset.
 */
export async function getDownloadUrl(): Promise<string | null> {
  const suffix = getAssetSuffix();
  if (!suffix) return null;

  const response = await fetch(GITHUB_LATEST_RELEASE_URL, {
    headers: { Accept: "application/vnd.github+json" },
  });
  if (!response.ok) return null;

  const release = (await response.json()) as {
    assets: Array<{ name: string; browser_download_url: string }>;
  };

  const asset = release.assets.find(
    (a) => a.name.endsWith(`${suffix}.zip`),
  );
  return asset?.browser_download_url ?? null;
}

/**
 * Get the target installation directory for the qsvmcp binary.
 *
 * macOS/Linux: /usr/local/bin if writable, else ~/.local/bin
 * Windows: %LOCALAPPDATA%\Programs\qsv\
 */
export function getInstallDir(): string {
  if (process.platform === "win32") {
    const localAppData = process.env.LOCALAPPDATA ?? join(process.env.USERPROFILE ?? "", "AppData", "Local");
    return join(localAppData, "Programs", "qsv");
  }

  // macOS / Linux: prefer /usr/local/bin if writable
  const systemDir = "/usr/local/bin";
  try {
    accessSync(systemDir, constants.W_OK);
    return systemDir;
  } catch {
    // Fall back to ~/.local/bin
    const home = process.env.HOME ?? "/tmp";
    return join(home, ".local", "bin");
  }
}

/**
 * Download and install the qsvmcp binary from the given URL.
 */
async function downloadAndInstall(url: string): Promise<InstallResult> {
  const isWindows = process.platform === "win32";
  const binaryName = isWindows ? "qsvmcp.exe" : "qsvmcp";
  const tempDir = mkdtempSync(join(tmpdir(), "qsv-install-"));

  try {
    // 1. Download the zip file
    const response = await fetch(url);
    if (!response.ok) {
      return {
        success: false,
        method: "direct-download",
        error: `Download failed: HTTP ${response.status} ${response.statusText}`,
      };
    }

    const zipPath = join(tempDir, "qsv.zip");
    const buffer = Buffer.from(await response.arrayBuffer());
    writeFileSync(zipPath, buffer);

    // 2. Extract only the qsvmcp binary
    if (isWindows) {
      execFileSync("powershell", [
        "-NoProfile",
        "-Command",
        `Expand-Archive -Path '${zipPath}' -DestinationPath '${tempDir}' -Force`,
      ], { encoding: "utf8", timeout: 60_000 });
    } else {
      // Extract entire archive, then search for the binary (handles nested directories)
      execFileSync("/usr/bin/unzip", ["-o", zipPath, "-d", tempDir], {
        encoding: "utf8",
        timeout: 60_000,
      });
    }

    // Find the extracted binary (may be nested in a subdirectory)
    let extractedPath = join(tempDir, binaryName);
    if (!existsSync(extractedPath)) {
      if (isWindows) {
        const findResult = execFileSync("powershell", [
          "-NoProfile",
          "-Command",
          `(Get-ChildItem -Path '${tempDir}' -Recurse -Filter '${binaryName}' | Select-Object -First 1).FullName`,
        ], { encoding: "utf8", timeout: 10_000 }).trim();
        if (findResult) {
          extractedPath = findResult;
        }
      } else {
        // Use find to locate the binary in nested directories
        const findResult = execFileSync("/usr/bin/find", [tempDir, "-name", binaryName, "-type", "f"], {
          encoding: "utf8",
          timeout: 10_000,
        }).trim().split("\n")[0];
        if (findResult) {
          extractedPath = findResult;
        }
      }
    }

    if (!existsSync(extractedPath)) {
      return {
        success: false,
        method: "direct-download",
        error: `Could not find ${binaryName} in the downloaded archive`,
      };
    }

    // 3. Install to target directory
    const installDir = getInstallDir();
    mkdirSync(installDir, { recursive: true });
    const targetPath = join(installDir, binaryName);
    copyFileSync(extractedPath, targetPath);

    // 4. Set permissions on Unix
    if (!isWindows) {
      chmodSync(targetPath, 0o755);
    }

    // 5. On macOS: clear Gatekeeper quarantine flag
    if (process.platform === "darwin") {
      try {
        execFileSync("xattr", ["-d", "com.apple.quarantine", targetPath], {
          encoding: "utf8",
          timeout: 10_000,
        });
      } catch {
        // xattr removal is best-effort — may fail if no quarantine flag is set
      }
    }

    return {
      success: true,
      method: "direct-download",
      binaryPath: targetPath,
      output: `Successfully installed ${binaryName} to ${targetPath}`,
    };
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    return {
      success: false,
      method: "direct-download",
      error: `Installation failed: ${message}`,
    };
  } finally {
    try {
      rmSync(tempDir, { recursive: true, force: true });
    } catch {
      // Ignore cleanup errors
    }
  }
}

/**
 * Return manual installation instructions when direct download is not available.
 */
export function getManualInstructions(platform: NodeJS.Platform): InstallResult {
  const baseUrl = "https://github.com/dathere/qsv/releases/latest";

  let instructions: string;

  switch (platform) {
    case "darwin":
      instructions =
        `## Install qsv on macOS\n\n` +
        `**Download pre-built binary:**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the macOS binary (qsv-*-apple-darwin.zip)\n` +
        `3. Extract and move to /usr/local/bin/\n` +
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsvmcp /usr/local/bin/\n\`\`\``;
      break;

    case "win32":
      instructions =
        `## Install qsv on Windows\n\n` +
        `**Download pre-built binary:**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the Windows binary (qsv-*-x86_64-pc-windows-msvc.zip)\n` +
        `3. Extract and add to your PATH`;
      break;

    default: // linux and others
      instructions =
        `## Install qsv on Linux\n\n` +
        `**Download pre-built binary:**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the Linux binary (qsv-*-x86_64-unknown-linux-gnu.zip)\n` +
        `3. Extract and install:\n` +
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsvmcp /usr/local/bin/\n\`\`\`\n\n` +
        `For more options, visit ${baseUrl}`;
      break;
  }

  return {
    success: false,
    method: "manual",
    instructions,
  };
}

/**
 * Main entry point: install qsv using the best available method.
 *
 * On supported platforms (macOS, Linux, Windows): downloads from GitHub Releases.
 * On other platforms: returns manual installation instructions.
 */
export async function installQsv(): Promise<InstallResult> {
  const url = await getDownloadUrl();

  if (url) {
    return await downloadAndInstall(url);
  }

  return getManualInstructions(process.platform);
}
