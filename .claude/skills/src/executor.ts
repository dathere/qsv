/**
 * QSV Skill Executor
 * Executes qsv skills by spawning qsv processes
 */

import { spawn } from "child_process";
import type { QsvSkill, SkillParams, SkillResult } from "./types.js";
import { config } from "./config.js";

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

export class SkillExecutor {
  private qsvBinary: string;
  private workingDirectory: string;

  constructor(qsvBinary: string = "qsv", workingDirectory?: string) {
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
   * Build command string for shell scripts (skips input validation)
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
    if (skill.command.subcommand === "stats") {
      // Check if --stats-jsonl is already in options
      const hasStatsJsonl =
        params.options &&
        (params.options["stats-jsonl"] === true ||
          params.options["stats_jsonl"] === true ||
          params.options["--stats-jsonl"] === true);

      // If not present, add it to params.options
      if (!hasStatsJsonl) {
        if (!params.options) {
          params.options = {};
        }
        params.options["stats-jsonl"] = true;
      }
    }

    // Add options/flags first
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        // Handle keys that may already include the -- prefix
        const normalizedKey = key.startsWith("--")
          ? key.substring(2)
          : key.startsWith("-")
            ? key.substring(1)
            : key;

        // --help is universally available for all qsv commands, even if not in skill definition
        // Note: mcp-tools.ts normalizes all help requests to options['help'] = true
        if (normalizedKey === "help") {
          if (value) args.push("--help");
          continue;
        }

        // Find option definition
        const option = skill.command.options.find(
          (o) =>
            o.flag === key ||
            o.short === key ||
            o.flag === `--${normalizedKey}` ||
            o.short === `-${normalizedKey}` ||
            o.flag.replace("--", "") === normalizedKey,
        );

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
   * Run qsv command with timeout handling
   */
  private runQsv(
    args: string[],
    params: SkillParams,
    timeoutMs: number,
  ): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
  }> {
    return new Promise((resolve, reject) => {
      // Log the full command for debugging
      const fullCommand = `${this.qsvBinary} ${args.join(" ")}`;
      console.error(`[Executor] Running command: ${fullCommand}`);
      console.error(`[Executor] Binary path: ${this.qsvBinary}`);
      console.error(`[Executor] Working directory: ${this.workingDirectory}`);
      console.error(`[Executor] Args:`, JSON.stringify(args));
      console.error(`[Executor] Timeout: ${timeoutMs}ms`);

      const proc = spawn(this.qsvBinary, args, {
        stdio: ["pipe", "pipe", "pipe"],
        cwd: this.workingDirectory,
      });

      let stdout = "";
      let stderr = "";
      let stdoutTruncated = false;
      let timedOut = false;
      let processExited = false;
      let timer: ReturnType<typeof setTimeout> | null = null;
      let killTimer: ReturnType<typeof setTimeout> | null = null;
      const MAX_STDOUT_SIZE = 50 * 1024 * 1024; // 50MB limit to prevent memory issues

      // Set up timeout handler
      timer = setTimeout(() => {
        timedOut = true;
        console.error(
          `[Executor] Process timed out after ${timeoutMs}ms, sending SIGTERM`,
        );
        proc.kill("SIGTERM");

        // Give process a moment to terminate gracefully, then send SIGKILL
        killTimer = setTimeout(() => {
          // Only send SIGKILL if process hasn't exited yet
          // Check both our flag (set on 'close') and proc.exitCode (set on 'exit')
          // since 'exit' fires before 'close' when process terminates
          if (!processExited && proc.exitCode === null) {
            console.error(`[Executor] Process did not terminate, sending SIGKILL`);
            proc.kill("SIGKILL");
          }
        }, 1000);
      }, timeoutMs);

      // Handle input
      if (params.stdin) {
        proc.stdin.write(params.stdin);
        proc.stdin.end();
      } else {
        proc.stdin.end();
      }

      // Collect output with size limit
      proc.stdout.on("data", (chunk) => {
        const chunkStr = chunk.toString();

        // Check if adding this chunk would exceed the limit
        if (stdout.length + chunkStr.length > MAX_STDOUT_SIZE) {
          if (!stdoutTruncated) {
            stdoutTruncated = true;
            console.error(
              `[Executor] WARNING: stdout exceeded ${MAX_STDOUT_SIZE / 1024 / 1024}MB limit, truncating output. Consider using --output to write to a file instead.`,
            );
            stdout +=
              "\n\n[OUTPUT TRUNCATED - Result too large for display. Use --output option to write to a file.]\n";
          }
          // Stop accumulating to prevent memory issues
          return;
        }

        stdout += chunkStr;
      });

      proc.stderr.on("data", (chunk) => {
        const data = chunk.toString();
        stderr += data;
        console.error(`[Executor] stderr: ${data}`);
      });

      proc.on("close", (exitCode) => {
        // Mark process as exited (used by SIGKILL escalation logic)
        processExited = true;

        // Clear timers
        if (timer) {
          clearTimeout(timer);
          timer = null;
        }
        if (killTimer) {
          clearTimeout(killTimer);
          killTimer = null;
        }

        console.error(`[Executor] Process exited with code: ${exitCode}`);
        console.error(`[Executor] stdout length: ${stdout.length}`);
        console.error(`[Executor] stderr length: ${stderr.length}`);

        // If process was terminated due to timeout, return exit code 124
        if (timedOut) {
          resolve({
            exitCode: 124, // Standard timeout exit code (like GNU timeout)
            stdout,
            stderr:
              stderr +
              `\n[TIMEOUT] Process exceeded ${timeoutMs}ms timeout and was terminated.`,
          });
          return;
        }

        resolve({
          exitCode: exitCode || 0,
          stdout,
          stderr,
        });
      });

      proc.on("error", (err) => {
        // Mark process as exited
        processExited = true;

        // Clear timers
        if (timer) {
          clearTimeout(timer);
          timer = null;
        }
        if (killTimer) {
          clearTimeout(killTimer);
          killTimer = null;
        }

        // Skip reject if we already handled timeout
        if (timedOut) {
          return;
        }

        console.error(`[Executor] Process error:`, err);
        reject(err);
      });
    });
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
      if (value && !this.validateType(value, arg.type)) {
        throw new Error(
          `Invalid type for ${arg.name}: expected ${arg.type}, got ${typeof value}`,
        );
      }
    }

    // Validate option types
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        // Handle keys that may already include the -- prefix
        const normalizedKey = key.startsWith("--")
          ? key.substring(2)
          : key.startsWith("-")
            ? key.substring(1)
            : key;

        const option = skill.command.options.find(
          (o) =>
            o.flag === key ||
            o.short === key ||
            o.flag === `--${normalizedKey}` ||
            o.short === `-${normalizedKey}` ||
            o.flag.replace("--", "") === normalizedKey,
        );

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
  private validateType(value: any, expectedType: string): boolean {
    switch (expectedType) {
      case "number":
        return typeof value === "number" && !isNaN(value);
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
