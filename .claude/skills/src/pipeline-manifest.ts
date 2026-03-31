/**
 * Pipeline Manifest — records every qsv tool invocation for reproducibility.
 *
 * Accumulates steps in-memory during a session, hashes files via `b3sum` CLI,
 * and serializes to `pipeline.json` + `pipeline.sh` at session end.
 * An incremental `.qsv-pipeline-steps.jsonl` provides crash resilience.
 */

import { execFile, execFileSync } from "node:child_process";
import { appendFileSync, statSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

// ── Types ────────────────────────────────────────────────────────────────────

export type StepKind = "exploratory" | "transformative" | "meta";

export interface FileHash {
  file: string;
  blake3: string | null;
  size_bytes: number;
}

export interface PipelineStep {
  step: number;
  invocation_id: string;
  tool: string;
  command: string;
  args: Record<string, unknown>;
  reason: string | null;
  timestamp: string;
  duration_ms: number;
  success: boolean;
  kind: StepKind;
  deterministic: boolean;
  seed?: number;
  input: FileHash | null;
  output: FileHash | null;
  additional_inputs: Array<FileHash & { param: string }>;
  error_message?: string;
  web_sources?: string[];
}

export interface PipelineManifestJson {
  version: string;
  session: {
    id: string;
    started_at: string;
    ended_at: string;
    qsv_version: string;
    mcp_server_version: string;
    working_directory: string;
  };
  steps: PipelineStep[];
  file_inventory: Record<
    string,
    {
      blake3: string | null;
      size_bytes: number;
      first_seen_step: number;
      role: "input" | "output" | "intermediate";
    }
  >;
}

// ── Constants ────────────────────────────────────────────────────────────────

const MANIFEST_VERSION = "1.0.0";
const JSONL_FILENAME = ".qsv-pipeline-steps.jsonl";
const B3SUM_TIMEOUT_MS = 30_000;
const MAX_HASHABLE_SIZE = 10 * 1024 * 1024 * 1024; // 10 GB

/** Tools that are metadata/infrastructure — not data operations. */
const META_TOOLS = new Set([
  "qsv_config",
  "qsv_set_working_dir",
  "qsv_get_working_dir",
  "qsv_browse_directory",
  "qsv_list_files",
  "qsv_log",
  "qsv_search_tools",
  "qsv_setup",
]);

/** Tools that inspect/profile data without transforming it. */
const EXPLORATORY_TOOLS = new Set([
  "qsv_stats",
  "qsv_frequency",
  "qsv_count",
  "qsv_headers",
  "qsv_sniff",
  "qsv_schema",
  "qsv_describegpt",
  "qsv_moarstats",
  "qsv_pragmastat",
]);

/** Commands that may produce non-deterministic output. */
const NON_DETERMINISTIC_COMMANDS = new Set(["sample", "sort", "sortcheck"]);

// ── PipelineManifest Class ───────────────────────────────────────────────────

export class PipelineManifest {
  private sessionId: string;
  private startedAt: string;
  private workingDir: string;
  private qsvVersion: string;
  private mcpServerVersion: string;
  private steps: PipelineStep[] = [];
  private stepCounter = 0;
  private b3sumAvailable: boolean;
  private hashCache = new Map<string, { blake3: string; mtimeMs: number }>();
  private pendingWebSources: string[] = [];

  constructor(
    sessionId: string,
    workingDir: string,
    qsvVersion: string,
    mcpServerVersion: string,
  ) {
    this.sessionId = sessionId;
    this.startedAt = new Date().toISOString();
    this.workingDir = workingDir;
    this.qsvVersion = qsvVersion;
    this.mcpServerVersion = mcpServerVersion;
    this.b3sumAvailable = detectB3sum();
  }

  /** Update the working directory (called when user changes it mid-session). */
  updateWorkingDir(newDir: string): void {
    this.workingDir = newDir;
  }

  /** Check if b3sum is available. */
  isB3sumAvailable(): boolean {
    return this.b3sumAvailable;
  }

  /** Get collected steps (for testing). */
  getSteps(): readonly PipelineStep[] {
    return this.steps;
  }

  /** Attach web source URLs that will be associated with the next recorded step. */
  addWebSource(url: string): void {
    this.pendingWebSources.push(url);
  }

  /**
   * Record a pipeline step after a tool call completes.
   * Hashes input/output files incrementally and appends to JSONL for crash resilience.
   */
  async recordStep(params: {
    invocationId: string;
    toolName: string;
    toolArgs: Record<string, unknown>;
    reason: string | null;
    commandLine: string | null;
    inputFile: string | null;
    outputFile: string | null;
    additionalInputFiles: Array<{ file: string; param: string }>;
    durationMs: number;
    success: boolean;
    errorMessage?: string;
    category?: string;
  }): Promise<void> {
    this.stepCounter++;

    const kind = classifyKind(params.toolName);
    const { deterministic, seed } = isDeterministic(params.toolName, params.toolArgs);

    // Hash input/output files
    const inputHash = params.inputFile
      ? await this.hashFile(params.inputFile)
      : null;
    const outputHash = params.outputFile
      ? await this.hashFile(params.outputFile)
      : null;

    // Hash additional inputs (e.g., join second table)
    const additionalInputs: Array<FileHash & { param: string }> = [];
    for (const { file, param } of params.additionalInputFiles) {
      const hash = await this.hashFile(file);
      if (hash) {
        additionalInputs.push({ ...hash, param });
      }
    }

    // Drain pending web sources
    const webSources =
      this.pendingWebSources.length > 0
        ? [...this.pendingWebSources]
        : undefined;
    this.pendingWebSources = [];

    const step: PipelineStep = {
      step: this.stepCounter,
      invocation_id: params.invocationId,
      tool: params.toolName,
      command: params.commandLine ?? "",
      args: params.toolArgs,
      reason: params.reason,
      timestamp: new Date().toISOString(),
      duration_ms: params.durationMs,
      success: params.success,
      kind,
      deterministic,
      ...(seed !== undefined && { seed }),
      input: inputHash,
      output: outputHash,
      additional_inputs: additionalInputs,
      ...(params.errorMessage && { error_message: params.errorMessage }),
      ...(webSources && { web_sources: webSources }),
    };

    this.steps.push(step);

    // Append to incremental JSONL (sync for atomicity under concurrent calls)
    try {
      appendFileSync(
        join(this.workingDir, JSONL_FILENAME),
        JSON.stringify(step) + "\n",
        "utf-8",
      );
    } catch (err) {
      console.error(
        `[PipelineManifest] Failed to write JSONL: ${err instanceof Error ? err.message : err}`,
      );
    }
  }

  /**
   * Hash a file using b3sum CLI.
   * Caches by path + mtime to avoid redundant hashing.
   */
  async hashFile(filePath: string): Promise<FileHash | null> {
    let fileStats;
    try {
      fileStats = statSync(filePath);
    } catch {
      return null;
    }

    if (!fileStats.isFile()) return null;

    const result: FileHash = {
      file: filePath,
      blake3: null,
      size_bytes: fileStats.size,
    };

    if (!this.b3sumAvailable) return result;
    if (fileStats.size > MAX_HASHABLE_SIZE) return result;

    // Check cache
    const cached = this.hashCache.get(filePath);
    if (cached && cached.mtimeMs === fileStats.mtimeMs) {
      return { ...result, blake3: cached.blake3 };
    }

    try {
      const { stdout } = await execFileAsync(
        "b3sum",
        ["--no-names", filePath],
        { timeout: B3SUM_TIMEOUT_MS },
      );
      const hash = stdout.trim();
      this.hashCache.set(filePath, { blake3: hash, mtimeMs: fileStats.mtimeMs });
      return { ...result, blake3: hash };
    } catch (err) {
      console.error(
        `[PipelineManifest] b3sum failed for ${filePath}: ${err instanceof Error ? err.message : err}`,
      );
      return result;
    }
  }

  /**
   * Build the file inventory from all recorded steps.
   */
  private buildFileInventory(): PipelineManifestJson["file_inventory"] {
    const inventory: PipelineManifestJson["file_inventory"] = {};

    for (const step of this.steps) {
      // Input file
      if (step.input?.file && !inventory[step.input.file]) {
        inventory[step.input.file] = {
          blake3: step.input.blake3,
          size_bytes: step.input.size_bytes,
          first_seen_step: step.step,
          role: "input",
        };
      }

      // Output file
      if (step.output?.file) {
        if (!inventory[step.output.file]) {
          inventory[step.output.file] = {
            blake3: step.output.blake3,
            size_bytes: step.output.size_bytes,
            first_seen_step: step.step,
            role: "output",
          };
        }
        // If a file was previously seen as an output and is now an input,
        // mark it as intermediate
        if (step.input?.file && inventory[step.input.file]?.role === "output") {
          inventory[step.input.file].role = "intermediate";
        }
      }

      // Additional inputs
      for (const ai of step.additional_inputs) {
        if (!inventory[ai.file]) {
          inventory[ai.file] = {
            blake3: ai.blake3,
            size_bytes: ai.size_bytes,
            first_seen_step: step.step,
            role: "input",
          };
        }
      }
    }

    return inventory;
  }

  /**
   * Generate a bash replay script from transformative steps.
   */
  generateReplayScript(): string {
    const lines = [
      "#!/usr/bin/env bash",
      "set -euo pipefail",
      "",
      "# Pipeline replay script generated by qsv MCP Server",
      `# Session: ${this.sessionId}`,
      `# Date: ${new Date().toISOString()}`,
      "#",
      "# This script replays the transformative (data-modifying) steps from the session.",
      "# Exploratory steps (stats, frequency, etc.) are omitted.",
      "",
    ];

    let hasSteps = false;
    for (const step of this.steps) {
      if (step.kind !== "transformative") continue;
      if (!step.command) continue;

      hasSteps = true;

      if (!step.success) {
        lines.push(`# SKIPPED (failed in original session): ${step.command}`);
        lines.push("");
        continue;
      }
      if (!step.deterministic) {
        lines.push(
          "# WARNING: non-deterministic — output may differ from original session",
        );
      }
      if (step.reason) {
        lines.push(`# ${step.reason}`);
      }
      lines.push(step.command);
      lines.push("");
    }

    if (!hasSteps) {
      lines.push("# No transformative steps were recorded in this session.");
      lines.push("");
    }

    return lines.join("\n");
  }

  /**
   * Finalize the manifest: write pipeline.json and pipeline.sh, clean up JSONL.
   */
  finalize(endTime?: string): { jsonPath: string; shPath: string } | null {
    if (this.steps.length === 0) {
      // Clean up empty JSONL if it exists
      try {
        unlinkSync(join(this.workingDir, JSONL_FILENAME));
      } catch {
        // ignore
      }
      return null;
    }

    const ended = endTime ?? new Date().toISOString();
    const manifest: PipelineManifestJson = {
      version: MANIFEST_VERSION,
      session: {
        id: this.sessionId,
        started_at: this.startedAt,
        ended_at: ended,
        qsv_version: this.qsvVersion,
        mcp_server_version: this.mcpServerVersion,
        working_directory: this.workingDir,
      },
      steps: this.steps,
      file_inventory: this.buildFileInventory(),
    };

    const jsonPath = join(this.workingDir, "pipeline.json");
    const shPath = join(this.workingDir, "pipeline.sh");

    try {
      writeFileSync(jsonPath, JSON.stringify(manifest, null, 2) + "\n", "utf-8");
      console.error(`[PipelineManifest] Wrote ${jsonPath}`);
    } catch (err) {
      console.error(
        `[PipelineManifest] Failed to write pipeline.json: ${err instanceof Error ? err.message : err}`,
      );
      return null;
    }

    try {
      writeFileSync(shPath, this.generateReplayScript(), { encoding: "utf-8", mode: 0o755 });
      console.error(`[PipelineManifest] Wrote ${shPath}`);
    } catch (err) {
      console.error(
        `[PipelineManifest] Failed to write pipeline.sh: ${err instanceof Error ? err.message : err}`,
      );
    }

    // Clean up incremental JSONL
    try {
      unlinkSync(join(this.workingDir, JSONL_FILENAME));
    } catch {
      // ignore — may not exist
    }

    return { jsonPath, shPath };
  }
}

// ── Exported Utility Functions ───────────────────────────────────────────────

/**
 * Classify a tool invocation as meta, exploratory, or transformative.
 */
export function classifyKind(toolName: string): StepKind {
  if (META_TOOLS.has(toolName)) return "meta";
  if (EXPLORATORY_TOOLS.has(toolName)) return "exploratory";
  return "transformative";
}

/**
 * Determine if a tool invocation is deterministic.
 * `sample` and `sort --random` without `--seed` are non-deterministic.
 */
export function isDeterministic(
  toolName: string,
  args: Record<string, unknown>,
): { deterministic: boolean; seed?: number } {
  const commandName = toolName.replace(/^qsv_/, "");

  if (!NON_DETERMINISTIC_COMMANDS.has(commandName)) {
    return { deterministic: true };
  }

  // sample is always non-deterministic unless --seed is set
  if (commandName === "sample") {
    const seed = extractSeed(args);
    if (seed !== undefined) {
      return { deterministic: true, seed };
    }
    return { deterministic: false };
  }

  // sort is only non-deterministic with --random
  if (commandName === "sort") {
    const hasRandom =
      args.random === true ||
      args["--random"] === true;
    if (!hasRandom) return { deterministic: true };

    const seed = extractSeed(args);
    if (seed !== undefined) {
      return { deterministic: true, seed };
    }
    return { deterministic: false };
  }

  return { deterministic: true };
}

/**
 * Detect whether `b3sum` is available on PATH.
 */
export function detectB3sum(): boolean {
  try {
    execFileSync("b3sum", ["--version"], { timeout: 5_000, stdio: "ignore" });
    console.error("[PipelineManifest] b3sum detected — BLAKE3 hashing enabled");
    return true;
  } catch {
    console.error(
      "[PipelineManifest] b3sum not found — BLAKE3 hashing disabled (install: cargo install b3sum)",
    );
    return false;
  }
}

/**
 * Build a pipeline manifest from JSONL steps file (crash-recovery path).
 * Used by the SessionEnd hook when the MCP server didn't finalize cleanly.
 */
export function buildManifestFromJsonl(
  jsonlPath: string,
  sessionId: string,
  workingDir: string,
): PipelineManifestJson | null {
  const { readFileSync, existsSync } = require("node:fs") as typeof import("node:fs");

  if (!existsSync(jsonlPath)) return null;

  let content: string;
  try {
    content = readFileSync(jsonlPath, "utf-8");
  } catch {
    return null;
  }

  const steps: PipelineStep[] = [];
  for (const line of content.split("\n").filter(Boolean)) {
    try {
      steps.push(JSON.parse(line));
    } catch {
      continue;
    }
  }

  if (steps.length === 0) return null;

  // Build file inventory
  const inventory: PipelineManifestJson["file_inventory"] = {};
  for (const step of steps) {
    if (step.input?.file && !inventory[step.input.file]) {
      inventory[step.input.file] = {
        blake3: step.input.blake3,
        size_bytes: step.input.size_bytes,
        first_seen_step: step.step,
        role: "input",
      };
    }
    if (step.output?.file) {
      if (!inventory[step.output.file]) {
        inventory[step.output.file] = {
          blake3: step.output.blake3,
          size_bytes: step.output.size_bytes,
          first_seen_step: step.step,
          role: "output",
        };
      }
      if (step.input?.file && inventory[step.input.file]?.role === "output") {
        inventory[step.input.file].role = "intermediate";
      }
    }
    for (const ai of step.additional_inputs ?? []) {
      if (!inventory[ai.file]) {
        inventory[ai.file] = {
          blake3: ai.blake3,
          size_bytes: ai.size_bytes,
          first_seen_step: step.step,
          role: "input",
        };
      }
    }
  }

  const firstTs = steps[0]?.timestamp ?? new Date().toISOString();
  const lastTs = steps[steps.length - 1]?.timestamp ?? new Date().toISOString();

  return {
    version: MANIFEST_VERSION,
    session: {
      id: sessionId,
      started_at: firstTs,
      ended_at: lastTs,
      qsv_version: "unknown",
      mcp_server_version: "unknown",
      working_directory: workingDir,
    },
    steps,
    file_inventory: inventory,
  };
}

/**
 * Generate a replay script from steps (used by the SessionEnd fallback path).
 */
export function generateReplayScriptFromSteps(
  steps: PipelineStep[],
  sessionId: string,
): string {
  const lines = [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    "",
    "# Pipeline replay script generated by qsv MCP Server (crash-recovery)",
    `# Session: ${sessionId}`,
    `# Date: ${new Date().toISOString()}`,
    "",
  ];

  let hasSteps = false;
  for (const step of steps) {
    if (step.kind !== "transformative") continue;
    if (!step.command) continue;
    hasSteps = true;

    if (!step.success) {
      lines.push(`# SKIPPED (failed): ${step.command}`);
      lines.push("");
      continue;
    }
    if (!step.deterministic) {
      lines.push("# WARNING: non-deterministic — output may differ");
    }
    if (step.reason) {
      lines.push(`# ${step.reason}`);
    }
    lines.push(step.command);
    lines.push("");
  }

  if (!hasSteps) {
    lines.push("# No transformative steps recorded.");
    lines.push("");
  }

  return lines.join("\n");
}

// ── Helpers ──────────────────────────────────────────────────────────────────

function extractSeed(args: Record<string, unknown>): number | undefined {
  const seed = args.seed ?? args["--seed"];
  if (seed !== undefined && seed !== null) {
    const n = Number(seed);
    if (!isNaN(n)) return n;
  }
  return undefined;
}
