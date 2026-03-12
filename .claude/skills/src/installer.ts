/**
 * Platform-specific qsv installation logic
 *
 * Detects the available package manager and installs qsv, or returns
 * manual installation instructions when no supported package manager is found.
 */

import { execFile as execFileCb, execFileSync } from "child_process";
import { promisify } from "util";

const execFile = promisify(execFileCb);

/** Timeout for package manager install commands (5 minutes) */
const INSTALL_TIMEOUT_MS = 5 * 60 * 1000;

export interface InstallResult {
  success: boolean;
  method: "homebrew" | "scoop" | "manual";
  binaryPath?: string;
  output?: string;
  error?: string;
  instructions?: string;
}

type PackageManager = "homebrew" | "scoop" | "none";

/**
 * Detect available package manager for the current platform.
 */
export function detectPackageManager(): PackageManager {
  const command = process.platform === "win32" ? "where" : "which";

  if (process.platform === "darwin" || process.platform === "linux") {
    try {
      execFileSync(command, ["brew"], {
        encoding: "utf8",
        stdio: ["ignore", "pipe", "ignore"],
      });
      return "homebrew";
    } catch {
      // brew not found
    }
  }

  if (process.platform === "win32") {
    try {
      execFileSync(command, ["scoop"], {
        encoding: "utf8",
        stdio: ["ignore", "pipe", "ignore"],
      });
      return "scoop";
    } catch {
      // scoop not found
    }
  }

  return "none";
}

/**
 * Install qsv via Homebrew (macOS/Linux).
 */
async function installViaHomebrew(): Promise<InstallResult> {
  try {
    const { stdout, stderr } = await execFile("brew", ["install", "qsv"], {
      encoding: "utf8",
      timeout: INSTALL_TIMEOUT_MS,
    });

    const output = (stdout + "\n" + stderr).trim();

    // "already installed" is a success case
    if (output.includes("already installed")) {
      return {
        success: true,
        method: "homebrew",
        output,
      };
    }

    return {
      success: true,
      method: "homebrew",
      output,
    };
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);

    // Treat "already installed" errors as success
    if (message.includes("already installed")) {
      return {
        success: true,
        method: "homebrew",
        output: message,
      };
    }

    return {
      success: false,
      method: "homebrew",
      error: message,
    };
  }
}

/**
 * Install qsv via Scoop (Windows).
 */
async function installViaScoop(): Promise<InstallResult> {
  try {
    const { stdout, stderr } = await execFile("scoop", ["install", "qsv"], {
      encoding: "utf8",
      timeout: INSTALL_TIMEOUT_MS,
    });

    const output = (stdout + "\n" + stderr).trim();

    return {
      success: true,
      method: "scoop",
      output,
    };
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);

    // Treat "already installed" as success
    if (message.includes("already installed") || message.includes("is already installed")) {
      return {
        success: true,
        method: "scoop",
        output: message,
      };
    }

    return {
      success: false,
      method: "scoop",
      error: message,
    };
  }
}

/**
 * Return manual installation instructions when no supported package manager is available.
 */
export function getManualInstructions(platform: NodeJS.Platform): InstallResult {
  const baseUrl = "https://github.com/dathere/qsv/releases/latest";

  let instructions: string;

  switch (platform) {
    case "darwin":
      instructions =
        `## Install qsv on macOS\n\n` +
        `**Option 1: Install Homebrew first, then qsv**\n` +
        `\`\`\`\n/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"\nbrew install qsv\n\`\`\`\n\n` +
        `**Option 2: Download pre-built binary**\n` +
        `1. Go to ${baseUrl}\n` +
        `2. Download the macOS binary (qsv-*-apple-darwin.zip)\n` +
        `3. Extract and move to /usr/local/bin/\n` +
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsv /usr/local/bin/\n\`\`\``;
      break;

    case "win32":
      instructions =
        `## Install qsv on Windows\n\n` +
        `**Option 1: Install Scoop first, then qsv**\n` +
        `\`\`\`\nSet-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser\nirm get.scoop.sh | iex\nscoop install qsv\n\`\`\`\n\n` +
        `**Option 2: Download pre-built binary**\n` +
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
        `\`\`\`\nunzip qsv-*.zip\nsudo mv qsv /usr/local/bin/\n\`\`\`\n\n` +
        `**Or install via Homebrew on Linux:**\n` +
        `\`\`\`\nbrew install qsv\n\`\`\`\n\n` +
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
 * On macOS: tries Homebrew, falls back to manual instructions.
 * On Windows: tries Scoop, falls back to manual instructions.
 * On Linux: tries Homebrew when available, otherwise returns manual instructions.
 */
export async function installQsv(): Promise<InstallResult> {
  const pm = detectPackageManager();

  switch (pm) {
    case "homebrew":
      return await installViaHomebrew();

    case "scoop":
      return await installViaScoop();

    case "none":
      return getManualInstructions(process.platform);
  }
}
