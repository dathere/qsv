/**
 * Working directory management: elicitation, roots sync, and directory discovery.
 *
 * Extracted from mcp-server.ts to reduce module size and make each concern
 * independently testable.
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { type ElicitResult } from "@modelcontextprotocol/sdk/types.js";
import { basename } from "node:path";
import { stat as fsStat } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import type { FilesystemResourceProvider } from "./mcp-filesystem.js";
import { getErrorMessage } from "./utils.js";

/**
 * Manages working directory state: confirmation status, elicitation,
 * roots sync, and directory discovery.
 */
export class WorkingDirManager {
  private workingDirConfirmed = false;
  private manuallySetWorkingDir = false;
  private elicitationPromise: Promise<void> | null = null;
  private syncingRoots = false;
  private pendingRootsSync = false;
  private rootsSyncRetries = 0;
  private static readonly MAX_ROOTS_SYNC_RETRIES = 3;

  constructor(
    private server: Server,
    private filesystemProvider: FilesystemResourceProvider,
    private onDirectoryChanged: (directory: string) => string,
  ) {}

  /** Whether the working directory has been confirmed (via roots, manual set, or elicitation). */
  get isConfirmed(): boolean {
    return this.workingDirConfirmed;
  }

  /** Mark as confirmed (e.g. after successful set_working_dir call). */
  confirmDirectory(): void {
    this.workingDirConfirmed = true;
  }

  /** Mark that the directory was set manually (blocks roots auto-sync). */
  markManuallySet(): void {
    this.manuallySetWorkingDir = true;
    this.workingDirConfirmed = true;
  }

  /** Clear the manual flag so roots auto-sync works again. */
  clearManuallySet(): void {
    this.manuallySetWorkingDir = false;
  }

  /**
   * Ensure the working directory is confirmed before a data tool runs.
   * If not yet confirmed, triggers elicitation (or awaits an in-progress one).
   * Should be called for non-exempt tool invocations.
   */
  async ensureConfirmedForTool(): Promise<void> {
    if (this.workingDirConfirmed) return;

    if (this.elicitationPromise) {
      try {
        await this.elicitationPromise;
      } catch {
        this.workingDirConfirmed = true;
        console.error("[Elicitation] Concurrent wait failed; using default");
      }
      return;
    }

    const promise = (async () => {
      try {
        const elicitResult = await this.elicitWorkingDirectory();
        if (elicitResult.directory) {
          try {
            this.onDirectoryChanged(elicitResult.directory);
            this.manuallySetWorkingDir = true;
            this.workingDirConfirmed = true;
            console.error(
              `[Elicitation] Working directory set to: ${elicitResult.directory}`,
            );
          } catch (err) {
            console.error(
              `[Elicitation] Failed to set working directory to ${elicitResult.directory}: ${getErrorMessage(err)}`,
            );
            this.workingDirConfirmed = true;
            console.error("[Elicitation] Working directory not applied; using default instead");
          }
        } else {
          this.workingDirConfirmed = true;
          console.error("[Elicitation] Working directory not selected; using default");
        }
      } finally {
        this.elicitationPromise = null;
      }
    })();
    this.elicitationPromise = promise;
    await promise;
  }

  /**
   * Auto-sync working directory from MCP client roots.
   * Used when the client communicates its root directory (e.g., Claude Cowork's "Work in a folder").
   */
  async syncWorkingDirFromRoots(): Promise<void> {
    if (this.manuallySetWorkingDir) {
      console.error("[Roots] Skipping auto-sync; working directory was manually set via qsv_set_working_dir");
      return;
    }
    if (this.syncingRoots) {
      console.error("[Roots] Sync already in progress, will re-sync when done");
      this.pendingRootsSync = true;
      return;
    }
    this.syncingRoots = true;
    let syncSucceeded = false;
    try {
      const { roots } = await this.server.listRoots();
      const fileRoot = roots.find(r => r.uri.startsWith("file://"));
      if (!fileRoot) {
        if (roots.length > 0) {
          console.error(`[Roots] ${roots.length} root(s) found but none use file:// URI; working directory not auto-set`);
        }
        return;
      }
      const rootPath = fileURLToPath(fileRoot.uri);
      try {
        const s = await fsStat(rootPath);
        if (!s.isDirectory()) {
          console.error(`[Roots] Skipping root path (not a directory): ${rootPath}`);
          return;
        }
      } catch {
        console.error(`[Roots] Skipping non-existent or inaccessible root path: ${rootPath}`);
        return;
      }
      const resolved = this.onDirectoryChanged(rootPath);
      this.workingDirConfirmed = true;
      console.error(`[Roots] Auto-set working directory to: ${resolved}`);
      if (roots.length > 1) {
        console.error(`[Roots] Note: ${roots.length - 1} additional root(s) ignored; only the first file:// root is used`);
      }
      syncSucceeded = true;
    } catch (error: unknown) {
      if (error instanceof Error) {
        const rawCode = "code" in error ? (error as Record<string, unknown>).code : undefined;
        const errorCode = typeof rawCode === "string" ? Number(rawCode) : rawCode;
        if (errorCode === -32601) {
          console.error(`[Roots] Client does not support roots (code -32601)`);
          return;
        }
        const msg = error.message.toLowerCase();
        if (msg.includes("not supported") || msg.includes("method not found")) {
          console.error(`[Roots] Client does not support roots: ${error.message}`);
          return;
        }
        console.error(`[Roots] Failed to sync working directory: ${error.message}`);
      } else {
        console.error(`[Roots] Failed to sync working directory: ${String(error)}`);
      }
    } finally {
      this.syncingRoots = false;
      if (this.pendingRootsSync) {
        this.pendingRootsSync = false;
        if (this.rootsSyncRetries < WorkingDirManager.MAX_ROOTS_SYNC_RETRIES) {
          this.rootsSyncRetries++;
          console.error(`[Roots] Running pending re-sync (attempt ${this.rootsSyncRetries}/${WorkingDirManager.MAX_ROOTS_SYNC_RETRIES})`);
          await this.syncWorkingDirFromRoots();
        } else {
          console.error(`[Roots] Max re-sync retries (${WorkingDirManager.MAX_ROOTS_SYNC_RETRIES}) reached, skipping pending sync`);
          this.rootsSyncRetries = 0;
        }
      } else if (syncSucceeded) {
        this.rootsSyncRetries = 0;
      }
    }
  }

  /**
   * Discover well-known directories that exist on the user's system.
   * Used by both the elicitation form and the fallback suggestion list.
   */
  async discoverDirectories(): Promise<Array<{ path: string; label: string }>> {
    const allowedDirs = this.filesystemProvider.getAllowedDirectories();
    const allowedCandidates: Array<{ path: string; label: string }> = allowedDirs.map(dir => ({
      path: dir,
      label: basename(dir) || dir,
    }));

    const currentDir = this.filesystemProvider.getWorkingDirectory();
    const allCandidates = [
      ...allowedCandidates,
      { path: currentDir, label: "Current Directory" },
    ];

    const results = await Promise.allSettled(
      allCandidates.map(async (candidate) => {
        const s = await fsStat(candidate.path);
        return s.isDirectory() ? candidate : null;
      }),
    );

    const seen = new Set<string>();
    const candidates: Array<{ path: string; label: string }> = [];
    for (const result of results) {
      if (result.status === "fulfilled" && result.value) {
        const { path } = result.value;
        if (!seen.has(path)) {
          seen.add(path);
          candidates.push(result.value);
        }
      }
    }
    return candidates;
  }

  /**
   * Build a directory suggestion list for when elicitation is not available.
   */
  async buildDirectorySuggestions(): Promise<string> {
    const candidates = await this.discoverDirectories();
    const currentDir = this.filesystemProvider.getWorkingDirectory();
    const suggestions = candidates.map((c) => `  - ${c.label}: ${c.path}`).join("\n");
    return (
      `No directory specified. Current working directory: ${currentDir}\n\n` +
      `Available directories:\n${suggestions}\n\n` +
      `Call qsv_set_working_dir with one of these paths (e.g. directory: "${candidates[0]?.path || currentDir}"), ` +
      `or provide any other accessible directory path.`
    );
  }

  /**
   * Present an interactive directory picker via MCP elicitation (form mode).
   * When elicitation fails, falls back to text suggestions.
   */
  async elicitWorkingDirectory(): Promise<{ directory?: string; fallback?: string }> {
    const capabilities = this.server.getClientCapabilities();
    if (capabilities && !capabilities.elicitation) {
      return { fallback: await this.buildDirectorySuggestions() };
    }

    const candidates = await this.discoverDirectories();
    const enumValues = candidates.map((c) => c.path);
    const enumLabels = candidates.map((c) => ({
      const: c.path,
      title: `${c.label} — ${c.path}`,
    }));

    try {
      const result: ElicitResult = await this.server.elicitInput({
        mode: "form",
        message: "Select a working directory for qsv file operations:",
        requestedSchema: {
          type: "object",
          properties: {
            selected_directory: {
              type: "string",
              title: "Directory",
              description: "Choose from common directories",
              enum: enumValues,
              oneOf: enumLabels,
            },
            custom_path: {
              type: "string",
              title: "Custom Path (optional)",
              description: "Or type a custom directory path (overrides selection above)",
            },
          },
        },
      });

      if (result.action === "accept" && result.content) {
        const customPath =
          typeof result.content.custom_path === "string"
            ? result.content.custom_path.trim()
            : "";
        const selectedDir =
          typeof result.content.selected_directory === "string"
            ? result.content.selected_directory.trim()
            : "";
        const chosenDir = customPath || selectedDir;

        if (chosenDir) {
          try {
            const stat = await fsStat(chosenDir);
            if (!stat.isDirectory()) {
              return {
                fallback: `"${chosenDir}" is not a directory. Please call qsv_set_working_dir with a valid directory path.`,
              };
            }
          } catch {
            return {
              fallback: `Directory "${chosenDir}" does not exist or is not accessible. Please call qsv_set_working_dir with a valid directory path.`,
            };
          }
          return { directory: chosenDir };
        }

        return {
          fallback: "No directory was selected. Please call qsv_set_working_dir with a directory path.",
        };
      }

      if (result.action === "decline") {
        return {
          fallback: "Directory selection was declined. The working directory remains unchanged. You can call qsv_set_working_dir with an explicit path.",
        };
      }

      return {
        fallback: "Directory selection was cancelled. The working directory remains unchanged.",
      };
    } catch (error: unknown) {
      console.error("[Elicitation] Failed to elicit working directory:", getErrorMessage(error));
      return { fallback: await this.buildDirectorySuggestions() };
    }
  }
}
