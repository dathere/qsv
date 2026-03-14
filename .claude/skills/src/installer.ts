/**
 * Platform-specific qsv installation logic
 *
 * Downloads the qsv release archive from GitHub Releases (latest), extracts
 * both qsvmcp and qsv binaries, and installs them to a discoverable location.
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
import { homedir, tmpdir } from "os";

export interface InstallResult {
  success: boolean;
  method: "direct-download" | "manual";
  binaryPaths?: { qsvmcp: string; qsv?: string };
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

/** Timeout for GitHub API and download requests (60 seconds) */
const FETCH_TIMEOUT_MS = 60_000;

/**
 * Escape a string for use inside PowerShell single-quoted strings.
 * Single-quoted strings in PowerShell only interpret '' as a literal '.
 * We also reject characters that should never appear in temp paths as a
 * defense-in-depth measure against command injection.
 */
function psEscape(s: string): string {
  // Reject paths containing characters that could break out of single-quoted context
  // or be used for injection (backticks, $, semicolons, pipes, etc.)
  if (/[`$;|&{}<>]/.test(s)) {
    throw new Error(`Refusing to interpolate potentially unsafe path into PowerShell command: ${s}`);
  }
  return s.replace(/'/g, "''");
}

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
    headers: {
      Accept: "application/vnd.github+json",
      "User-Agent": "qsv-mcp-server",
    },
    signal: AbortSignal.timeout(FETCH_TIMEOUT_MS),
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
    const localAppData = process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local");
    return join(localAppData, "Programs", "qsv");
  }

  // macOS / Linux: prefer /usr/local/bin if writable
  const systemDir = "/usr/local/bin";
  try {
    accessSync(systemDir, constants.W_OK);
    return systemDir;
  } catch {
    // Fall back to ~/.local/bin
    return join(homedir(), ".local", "bin");
  }
}

/**
 * Find a binary by name in the extracted archive directory.
 * Returns the path if found, or null if not found.
 */
function findBinaryInArchive(tempDir: string, binaryName: string, isWindows: boolean): string | null {
  let extractedPath = join(tempDir, binaryName);
  if (existsSync(extractedPath)) return extractedPath;

  try {
    if (isWindows) {
      const findResult = execFileSync("powershell", [
        "-NoProfile",
        "-Command",
        `(Get-ChildItem -Path '${psEscape(tempDir)}' -Recurse -Filter '${binaryName}' | Select-Object -First 1).FullName`,
      ], { encoding: "utf8", timeout: 10_000 }).trim();
      if (findResult) return findResult;
    } else {
      const findResult = execFileSync("/usr/bin/find", [tempDir, "-name", binaryName, "-type", "f"], {
        encoding: "utf8",
        timeout: 10_000,
      }).trim().split("\n")[0];
      if (findResult) return findResult;
    }
  } catch {
    // Best-effort search — if find/powershell fails, fall through to return null
  }

  return null;
}

/**
 * Download and install the qsvmcp and qsv binaries from the given URL.
 */
async function downloadAndInstall(url: string): Promise<InstallResult> {
  const isWindows = process.platform === "win32";
  const tempDir = mkdtempSync(join(tmpdir(), "qsv-install-"));

  try {
    // 1. Download the zip file
    const response = await fetch(url, {
      headers: { "User-Agent": "qsv-mcp-server" },
      signal: AbortSignal.timeout(FETCH_TIMEOUT_MS),
    });
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

    // 2. Extract the archive
    if (isWindows) {
      execFileSync("powershell", [
        "-NoProfile",
        "-Command",
        `Expand-Archive -Path '${psEscape(zipPath)}' -DestinationPath '${psEscape(tempDir)}' -Force`,
      ], { encoding: "utf8", timeout: 60_000 });
    } else {
      execFileSync("/usr/bin/unzip", ["-o", zipPath, "-d", tempDir], {
        encoding: "utf8",
        timeout: 60_000,
      });
    }

    // 3. Find and install both binaries
    const qsvmcpName = isWindows ? "qsvmcp.exe" : "qsvmcp";
    const qsvName = isWindows ? "qsv.exe" : "qsv";

    const qsvmcpExtracted = findBinaryInArchive(tempDir, qsvmcpName, isWindows);
    if (!qsvmcpExtracted) {
      return {
        success: false,
        method: "direct-download",
        error: `Could not find ${qsvmcpName} in the downloaded archive`,
      };
    }

    const installDir = getInstallDir();
    mkdirSync(installDir, { recursive: true });

    const installedPaths: { qsvmcp: string; qsv?: string } = {
      qsvmcp: join(installDir, qsvmcpName),
    };

    // Install qsvmcp (required)
    copyFileSync(qsvmcpExtracted, installedPaths.qsvmcp);

    // Install qsv (optional — warn but don't fail if missing)
    const qsvExtracted = findBinaryInArchive(tempDir, qsvName, isWindows);
    let qsvWarning = "";
    if (qsvExtracted) {
      const qsvTarget = join(installDir, qsvName);
      copyFileSync(qsvExtracted, qsvTarget);
      installedPaths.qsv = qsvTarget;
    } else {
      qsvWarning = `\nNote: ${qsvName} was not found in the archive; only qsvmcp was installed.`;
    }

    // 4. Set permissions on Unix
    if (!isWindows) {
      chmodSync(installedPaths.qsvmcp, 0o755);
      if (installedPaths.qsv) chmodSync(installedPaths.qsv, 0o755);
    }

    // 5. On macOS: clear Gatekeeper quarantine flag
    if (process.platform === "darwin") {
      for (const p of [installedPaths.qsvmcp, installedPaths.qsv]) {
        if (!p) continue;
        try {
          execFileSync("xattr", ["-d", "com.apple.quarantine", p], {
            encoding: "utf8",
            timeout: 10_000,
          });
        } catch {
          // xattr removal is best-effort — may fail if no quarantine flag is set
        }
      }
    }

    const installed = installedPaths.qsv
      ? `qsvmcp and qsv`
      : `qsvmcp`;

    return {
      success: true,
      method: "direct-download",
      binaryPaths: installedPaths,
      output: `Successfully installed ${installed} to ${installDir}${qsvWarning}`,
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
        `2. Download the macOS ARM64 binary (qsv-*-aarch64-apple-darwin.zip)\n` +
        `3. Extract and move to /usr/local/bin/\n` +
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsvmcp qsv /usr/local/bin/\n\`\`\``;
      break;

    case "win32":
      instructions =
        `## Install qsv on Windows\n\n` +
        `**Download pre-built binary:**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the Windows binary (qsv-*-x86_64-pc-windows-msvc.zip)\n` +
        `3. Extract and add qsvmcp.exe and qsv.exe to your PATH`;
      break;

    default: // linux and others
      instructions =
        `## Install qsv on Linux\n\n` +
        `**Download pre-built binary:**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the Linux binary (qsv-*-x86_64-unknown-linux-gnu.zip)\n` +
        `3. Extract and install:\n` +
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsvmcp qsv /usr/local/bin/\n\`\`\`\n\n` +
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
