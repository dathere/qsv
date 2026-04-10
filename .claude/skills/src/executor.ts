/**
 * QSV Skill Executor
 * Executes qsv skills by spawning qsv processes
 */

import { spawn, type ChildProcess } from "child_process";
import type { QsvSkill, Option, SkillParams, SkillResult } from "./types.js";
import { config } from "./config.js";
import { compareVersions } from "./utils.js";
import { spawnWithTimeout } from "./spawn-utils.js";

/** Minimum qsv binary version that supports --frequency-jsonl */
const FREQUENCY_JSONL_MIN_VERSION = "16.1.0";

/**
 * Check if a skill has subcommands by examining its first argument
 *
 * Commands with subcommands have "subcommand" as the first argument name
 * with an enum of valid subcommand values
 */
function hasSubcommands(skill: QsvSkill): boolean {
  const firstArg = skill.command.args[0];
  return firstArg?.name === "subcommand" && "enum" in firstArg;
}

/**
 * Get the subcommand value from parameters
 *
 * For commands with subcommands, the first argument is always named "subcommand"
 * and the user must provide a value from the enum (unless it's optional)
 */
function getSubcommand(skill: QsvSkill, params: SkillParams): string | null {
  const firstArg = skill.command.args[0];

  if (firstArg.name !== "subcommand") {
    throw new Error(
      `Internal error: expected first arg to be 'subcommand', got '${firstArg.name}'`,
    );
  }

  // Get the subcommand value from params
  const subcommandValue = params.args?.subcommand;

  if (!subcommandValue) {
    // If subcommand is optional, return null (don't add to args)
    if (!firstArg.required) {
      return null;
    }

    // Otherwise, throw error for missing required subcommand
    const validSubcommands =
      "enum" in firstArg && Array.isArray((firstArg as { enum?: unknown }).enum)
        ? (firstArg as { enum: string[] }).enum
        : [];
    throw new Error(
      `Missing required subcommand for ${skill.command.subcommand}. ` +
        `Valid subcommands: ${validSubcommands.join(", ")}`,
    );
  }

  const subcommand = String(subcommandValue);

  // Don't validate against enum - let qsv itself validate the subcommand
  // The enum is for documentation/UI purposes only

  return subcommand;
}

/**
 * Normalize an option key by stripping leading dashes
 */
function normalizeOptionKey(key: string): string {
  if (key.startsWith("--") && key.length > 2) return key.substring(2);
  if (key.startsWith("-") && key.length > 1) return key.substring(1);
  return key;
}

/**
 * Find an option definition in a skill's options list
 */
function findOptionDef(skill: QsvSkill, key: string): Option | undefined {
  const normalizedKey = normalizeOptionKey(key);
  return skill.command.options.find(
    (o) =>
      o.flag === key ||
      o.short === key ||
      o.flag === `--${normalizedKey}` ||
      o.short === `-${normalizedKey}` ||
      o.flag.replace("--", "") === normalizedKey,
  );
}

/**
 * Lightweight qsv process runner for simple operations (indexing, conversion, metadata).
 * Returns stdout on success, throws on failure or timeout.
 */
export async function runQsvSimple(
  binPath: string,
  args: string[],
  options?: {
    timeoutMs?: number;
    cwd?: string;
    captureStdout?: boolean;
    /** Called with the spawned ChildProcess for external tracking (e.g., shutdown kill). */
    onSpawn?: (proc: ChildProcess) => void;
    /** Called when the process exits for external cleanup. */
    onExit?: (proc: ChildProcess) => void;
  },
): Promise<string> {
  const timeoutMs = options?.timeoutMs ?? 60_000;
  const captureStdout = options?.captureStdout ?? false;

  return new Promise((resolve, reject) => {
    const proc = spawn(binPath, args, {
      stdio: ["ignore", captureStdout ? "pipe" : "ignore", "pipe"],
      cwd: options?.cwd,
    });

    options?.onSpawn?.(proc);

    let stdout = "";
    let stderr = "";
    let finalized = false;

    // Single finalization path: clears timer, calls onExit exactly once,
    // then resolves or rejects the promise.
    const finalize = (err?: Error, code?: number | null) => {
      if (finalized) return;
      finalized = true;
      clearTimeout(timer);
      options?.onExit?.(proc);

      if (err) {
        reject(err);
      } else if (code === 0) {
        resolve(stdout);
      } else {
        reject(new Error(`Command failed with exit code ${code}: ${stderr}`));
      }
    };

    const timer = setTimeout(() => {
      proc.kill("SIGTERM");
      finalize(new Error(`qsv ${args[0]} timed out after ${timeoutMs}ms`));
    }, timeoutMs);

    if (captureStdout) {
      proc.stdout!.on("data", (chunk) => { stdout += chunk.toString(); });
    }
    const MAX_STDERR_SIZE = 50 * 1024 * 1024; // 50MB limit
    proc.stderr!.on("data", (chunk) => {
      const data = chunk.toString();
      if (stderr.length + data.length <= MAX_STDERR_SIZE) {
        stderr += data;
      }
    });

    proc.on("close", (code) => {
      finalize(undefined, code);
    });

    proc.on("error", (err) => {
      finalize(err);
    });
  });
}

export class SkillExecutor {
  private qsvBinary: string;
  private workingDirectory: string;

  constructor(qsvBinary: string = config.qsvBinPath, workingDirectory?: string) {
    this.qsvBinary = qsvBinary;
    this.workingDirectory = workingDirectory || process.cwd();
  }

  /**
   * Set the working directory for qsv process spawning.
   * This ensures qsv uses the correct directory for resolving relative paths
   * and writing secondary output files.
   */
  setWorkingDirectory(dir: string): void {
    this.workingDirectory = dir;
  }

  /**
   * Get the current working directory
   */
  getWorkingDirectory(): string {
    return this.workingDirectory;
  }

  /**
   * Execute a skill with given parameters
   */
  async execute(skill: QsvSkill, params: SkillParams): Promise<SkillResult> {
    // Skip validation when --help is requested (no input file needed for help)
    // Note: mcp-tools.ts normalizes all help requests to options['help'] = true
    const isHelpRequest = params.options?.["help"] === true;

    if (!isHelpRequest) {
      // Validate parameters only when not requesting help
      this.validateParams(skill, params);
    }

    // Build command arguments
    const args = this.buildArgs(skill, params);

    // Get timeout from params, then config (default 10 minutes), then fallback
    const rawTimeout = params.timeoutMs ?? config.operationTimeoutMs ?? 10 * 60 * 1000;
    // Validate timeout: must be positive number, clamp to sane range (1s - 30min)
    const timeoutMs = Math.max(1000, Math.min(30 * 60 * 1000, Number(rawTimeout) || 10 * 60 * 1000));

    // Execute qsv command
    const startTime = Date.now();
    const result = await this.runQsv(args, params, timeoutMs);

    return {
      success: result.exitCode === 0,
      output: result.stdout,
      stderr: result.stderr,
      metadata: {
        command: `qsv ${args.join(" ")}`,
        duration: Date.now() - startTime,
        rowsProcessed: this.extractRowCount(result.stderr),
        exitCode: result.exitCode,
      },
    };
  }

  /**
   * Build command string for display/logging only (NOT shell-safe).
   * Args are joined with spaces without quoting or escaping.
   * For actual execution, use execute() which passes args as an array to spawn().
   */
  buildCommand(skill: QsvSkill, params: SkillParams): string {
    const args = this.buildArgs(skill, params, true);
    return `qsv ${args.join(" ")}`;
  }

  /**
   * Build command line arguments from skill definition and params
   */
  private buildArgs(
    skill: QsvSkill,
    params: SkillParams,
    forShellScript = false,
  ): string[] {
    const args: string[] = [skill.command.subcommand];

    // Check if this is a help request
    // Note: mcp-tools.ts normalizes all help requests to options['help'] = true
    const isHelpRequest = params.options?.["help"] === true;

    // Handle commands with subcommands
    // Commands with subcommands have "subcommand" as the first argument with an enum
    if (hasSubcommands(skill)) {
      const subcommand = getSubcommand(skill, params);
      if (subcommand) {
        args.push(subcommand);
        console.error(
          `[Executor] Added ${skill.command.subcommand} subcommand: ${subcommand}`,
        );
      } else {
        console.error(
          `[Executor] No subcommand provided for ${skill.command.subcommand} (optional)`,
        );
      }
    }

    // For stats command, always ensure --stats-jsonl flag is set
    // This creates the stats cache that other "smart" commands use
    // Skip when reading from stdin (e.g. pipelines) since cache requires a file path
    // Note: --stats-jsonl has been available since qsv 10.0.0, so no version guard needed
    if (skill.command.subcommand === "stats" && !params.stdin) {
      if (!params.options) {
        params.options = {};
      }
      // Check both key formats to avoid duplicate flags
      // (buildSkillExecParams uses "--stats-jsonl", auto-add uses "stats-jsonl")
      if (!params.options["stats-jsonl"] && !params.options["--stats-jsonl"]) {
        params.options["stats-jsonl"] = true;
      }
    }

    // For frequency command, always ensure --frequency-jsonl flag is set
    // This creates the frequency cache for reuse
    // Skip when reading from stdin (e.g. pipelines) since cache requires a file path
    // Only enable if the binary version supports the flag (introduced in 16.0.0)
    if (
      skill.command.subcommand === "frequency" &&
      !params.stdin &&
      findOptionDef(skill, "frequency-jsonl")
    ) {
      const binaryVersion = config.qsvValidation.version;
      if (
        binaryVersion &&
        compareVersions(binaryVersion, FREQUENCY_JSONL_MIN_VERSION) >= 0
      ) {
        if (!params.options) {
          params.options = {};
        }
        // Check both key formats to avoid duplicate flags
        if (
          !params.options["frequency-jsonl"] &&
          !params.options["--frequency-jsonl"]
        ) {
          params.options["frequency-jsonl"] = true;
        }
      }
    }

    // Add options/flags first
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        const normalizedKey = normalizeOptionKey(key);

        // --help is universally available for all qsv commands, even if not in skill definition
        // Note: mcp-tools.ts normalizes all help requests to options['help'] = true
        if (normalizedKey === "help") {
          if (value) args.push("--help");
          continue;
        }

        // Find option definition
        const option = findOptionDef(skill, key);
        if (!option) continue;

        if (option.type === "flag") {
          // Boolean flag
          if (value) args.push(option.flag);
        } else {
          // Option with value
          args.push(option.flag, String(value));
        }
      }
    }

    // Add positional arguments
    // For commands with subcommands, skip the first argument (it's the subcommand itself)
    const startIndex = hasSubcommands(skill) ? 1 : 0;

    for (let i = startIndex; i < skill.command.args.length; i++) {
      const arg = skill.command.args[i];
      const value = params.args?.[arg.name];

      if (value !== undefined) {
        args.push(String(value));
      } else if (arg.required && !forShellScript && !isHelpRequest) {
        // Skip input validation if stdin is provided or if --help is requested
        if (arg.name === "input" && params.stdin) {
          continue;
        }
        throw new Error(`Missing required argument: ${arg.name}`);
      }
    }

    return args;
  }

  /**
   * Signal-to-exit-code mapping (128 + signal number).
   * SIGTERM=15 → 143, SIGKILL=9 → 137, SIGINT=2 → 130
   */
  private static readonly SIGNAL_EXIT_CODES: Record<string, number> = {
    SIGTERM: 143,
    SIGKILL: 137,
    SIGINT: 130,
    SIGHUP: 129,
    SIGQUIT: 131,
  };

  /**
   * Run qsv command with timeout handling
   */
  private async runQsv(
    args: string[],
    params: SkillParams,
    timeoutMs: number,
  ): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
  }> {
    const debug = config.mcpLogLevel === "debug";
    if (debug) {
      console.error(`[Executor] Running command: ${this.qsvBinary} ${args.join(" ")}`);
      console.error(`[Executor] Working directory: ${this.workingDirectory}`);
      console.error(`[Executor] Timeout: ${timeoutMs}ms`);
    }

    const result = await spawnWithTimeout({
      binary: this.qsvBinary,
      args,
      cwd: this.workingDirectory,
      timeoutMs,
      stdin: params.stdin,
      stdoutTruncationMsg:
        "\n\n[OUTPUT TRUNCATED - Result too large for display. Use --output option to write to a file.]\n",
    });

    if (debug) {
      console.error(`[Executor] Process exited: code=${result.exitCode}, signal=${result.signal}`);
      console.error(`[Executor] stdout length: ${result.stdout.length}, stderr length: ${result.stderr.length}`);
    }

    if (result.timedOut) {
      return {
        exitCode: 124, // Standard timeout exit code (like GNU timeout)
        stdout: result.stdout,
        stderr: result.stderr + `\n[TIMEOUT] Process exceeded ${timeoutMs}ms timeout and was terminated.`,
      };
    }

    // Handle signal termination: map signals to conventional exit codes
    if (result.exitCode === null && result.signal) {
      const signalExitCode = SkillExecutor.SIGNAL_EXIT_CODES[result.signal] ?? 128;
      return {
        exitCode: signalExitCode,
        stdout: result.stdout,
        stderr: result.stderr + `\n[SIGNAL] Process was terminated by signal: ${result.signal}`,
      };
    }

    return {
      exitCode: result.exitCode ?? 0,
      stdout: result.stdout,
      stderr: result.stderr,
    };
  }

  /**
   * Validate parameters against skill definition
   */
  private validateParams(skill: QsvSkill, params: SkillParams): void {
    // Validate required arguments
    for (const arg of skill.command.args) {
      // Skip validation for 'subcommand' argument - handled separately in buildArgs
      if (arg.name === "subcommand" && hasSubcommands(skill)) {
        continue;
      }

      if (arg.required && !params.args?.[arg.name]) {
        // Skip input validation if stdin is provided
        if (arg.name === "input" && params.stdin) {
          continue;
        }
        throw new Error(`Missing required argument: ${arg.name}`);
      }

      // Type validation
      const value = params.args?.[arg.name];
      if (value !== undefined && value !== null && !this.validateType(value, arg.type)) {
        throw new Error(
          `Invalid type for ${arg.name}: expected ${arg.type}, got ${typeof value}`,
        );
      }
    }

    // Validate option types
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        const option = findOptionDef(skill, key);

        if (!option) {
          console.warn(`Unknown option: ${key}`);
          continue;
        }

        if (option.type !== "flag" && !this.validateType(value, option.type)) {
          throw new Error(
            `Invalid type for option ${key}: expected ${option.type}`,
          );
        }
      }
    }
  }

  /**
   * Validate value type
   */
  private validateType(value: unknown, expectedType: string): boolean {
    switch (expectedType) {
      case "number":
        return typeof value === "number" && Number.isFinite(value);
      case "string":
      case "file":
      case "regex":
        return typeof value === "string";
      case "flag":
        return typeof value === "boolean";
      default:
        return true;
    }
  }

  /**
   * Extract row count from stderr (qsv outputs progress/stats there)
   */
  private extractRowCount(stderr: string): number | undefined {
    // Look for patterns like "Processed 1000 rows"
    const match = stderr.match(/(\d+)\s+rows?/i);
    return match ? parseInt(match[1], 10) : undefined;
  }
}
