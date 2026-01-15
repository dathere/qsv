/**
 * QSV Skill Executor
 * Executes qsv skills by spawning qsv processes
 */

import { spawn } from 'child_process';
import type { QsvSkill, SkillParams, SkillResult } from './types.js';

export class SkillExecutor {
  private qsvBinary: string;

  constructor(qsvBinary: string = 'qsv') {
    this.qsvBinary = qsvBinary;
  }

  /**
   * Execute a skill with given parameters
   */
  async execute(skill: QsvSkill, params: SkillParams): Promise<SkillResult> {
    // Validate parameters
    this.validateParams(skill, params);

    // Build command arguments
    const args = this.buildArgs(skill, params);

    // Execute qsv command
    const startTime = Date.now();
    const result = await this.runQsv(args, params);

    return {
      success: result.exitCode === 0,
      output: result.stdout,
      stderr: result.stderr,
      metadata: {
        command: `qsv ${args.join(' ')}`,
        duration: Date.now() - startTime,
        rowsProcessed: this.extractRowCount(result.stderr),
        exitCode: result.exitCode
      }
    };
  }

  /**
   * Build command string for shell scripts (skips input validation)
   */
  buildCommand(skill: QsvSkill, params: SkillParams): string {
    const args = this.buildArgs(skill, params, true);
    return `qsv ${args.join(' ')}`;
  }

  /**
   * Build command line arguments from skill definition and params
   */
  private buildArgs(skill: QsvSkill, params: SkillParams, forShellScript = false): string[] {
    const args: string[] = [skill.command.subcommand];

    // Special handling for 'apply' command which has subcommands/modes
    // qsv apply has four modes: operations, emptyreplace, dynfmt, calcconv
    // The mode needs to be inserted after 'apply' but before the arguments
    if (skill.command.subcommand === 'apply') {
      // Determine which apply mode to use based on parameters
      let applyMode: string | undefined;

      if (params.args?.operations) {
        // operations mode: qsv apply operations <operations> [options] <column> [<input>]
        applyMode = 'operations';
      } else if (params.options?.['replacement'] || params.options?.['--replacement']) {
        // emptyreplace mode: qsv apply emptyreplace --replacement=<string> [options] <column> [<input>]
        applyMode = 'emptyreplace';
      } else if (params.options?.['formatstr'] || params.options?.['--formatstr']) {
        // dynfmt or calcconv mode - both use --formatstr
        // Default to operations if we have an operations arg, otherwise dynfmt
        // User can override by passing the mode explicitly
        applyMode = 'dynfmt';
      }

      // Add the mode as the next argument after 'apply'
      if (applyMode) {
        args.push(applyMode);
        console.error(`[Executor] Added apply mode: ${applyMode}`);
      } else {
        console.error('[Executor] WARNING: apply command called without clear mode, defaulting to operations');
        args.push('operations');
      }
    }

    // For stats command, always ensure --stats-jsonl flag is set
    // This creates the stats cache that other "smart" commands use
    if (skill.command.subcommand === 'stats') {
      // Check if --stats-jsonl is already in options
      const hasStatsJsonl = params.options && (
        params.options['stats-jsonl'] === true ||
        params.options['stats_jsonl'] === true ||
        params.options['--stats-jsonl'] === true
      );

      // If not present, add it to params.options
      if (!hasStatsJsonl) {
        if (!params.options) {
          params.options = {};
        }
        params.options['stats-jsonl'] = true;
      }
    }

    // Add options/flags first
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        // Handle keys that may already include the -- prefix
        const normalizedKey = key.startsWith('--') ? key.substring(2) : key.startsWith('-') ? key.substring(1) : key;

        // Find option definition
        const option = skill.command.options.find(o =>
          o.flag === key ||
          o.short === key ||
          o.flag === `--${normalizedKey}` ||
          o.short === `-${normalizedKey}` ||
          o.flag.replace('--', '') === normalizedKey
        );

        if (!option) continue;

        if (option.type === 'flag') {
          // Boolean flag
          if (value) args.push(option.flag);
        } else {
          // Option with value
          args.push(option.flag, String(value));
        }
      }
    }

    // Add positional arguments
    for (const arg of skill.command.args) {
      const value = params.args?.[arg.name];

      if (value !== undefined) {
        args.push(String(value));
      } else if (arg.required && !forShellScript) {
        // Skip input validation if stdin is provided
        if (arg.name === 'input' && params.stdin) {
          continue;
        }
        throw new Error(`Missing required argument: ${arg.name}`);
      }
    }

    return args;
  }

  /**
   * Run qsv command
   */
  private runQsv(args: string[], params: SkillParams): Promise<{
    exitCode: number;
    stdout: string;
    stderr: string;
  }> {
    return new Promise((resolve, reject) => {
      // Log the full command for debugging
      const fullCommand = `${this.qsvBinary} ${args.join(' ')}`;
      console.error(`[Executor] Running command: ${fullCommand}`);
      console.error(`[Executor] Binary path: ${this.qsvBinary}`);
      console.error(`[Executor] Args:`, JSON.stringify(args));

      const proc = spawn(this.qsvBinary, args, {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';
      let stdoutTruncated = false;
      const MAX_STDOUT_SIZE = 50 * 1024 * 1024; // 50MB limit to prevent memory issues

      // Handle input
      if (params.stdin) {
        proc.stdin.write(params.stdin);
        proc.stdin.end();
      } else {
        proc.stdin.end();
      }

      // Collect output with size limit
      proc.stdout.on('data', chunk => {
        const chunkStr = chunk.toString();

        // Check if adding this chunk would exceed the limit
        if (stdout.length + chunkStr.length > MAX_STDOUT_SIZE) {
          if (!stdoutTruncated) {
            stdoutTruncated = true;
            console.error(`[Executor] WARNING: stdout exceeded ${MAX_STDOUT_SIZE / 1024 / 1024}MB limit, truncating output. Consider using --output to write to a file instead.`);
            stdout += '\n\n[OUTPUT TRUNCATED - Result too large for display. Use --output option to write to a file.]\n';
          }
          // Stop accumulating to prevent memory issues
          return;
        }

        stdout += chunkStr;
      });

      proc.stderr.on('data', chunk => {
        const data = chunk.toString();
        stderr += data;
        console.error(`[Executor] stderr: ${data}`);
      });

      proc.on('close', exitCode => {
        console.error(`[Executor] Process exited with code: ${exitCode}`);
        console.error(`[Executor] stdout length: ${stdout.length}`);
        console.error(`[Executor] stderr length: ${stderr.length}`);
        resolve({
          exitCode: exitCode || 0,
          stdout,
          stderr
        });
      });

      proc.on('error', (err) => {
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
      if (arg.required && !params.args?.[arg.name]) {
        // Skip input validation if stdin is provided
        if (arg.name === 'input' && params.stdin) {
          continue;
        }
        throw new Error(`Missing required argument: ${arg.name}`);
      }

      // Type validation
      const value = params.args?.[arg.name];
      if (value && !this.validateType(value, arg.type)) {
        throw new Error(
          `Invalid type for ${arg.name}: expected ${arg.type}, got ${typeof value}`
        );
      }
    }

    // Validate option types
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        // Handle keys that may already include the -- prefix
        const normalizedKey = key.startsWith('--') ? key.substring(2) : key.startsWith('-') ? key.substring(1) : key;

        const option = skill.command.options.find(o =>
          o.flag === key ||
          o.short === key ||
          o.flag === `--${normalizedKey}` ||
          o.short === `-${normalizedKey}` ||
          o.flag.replace('--', '') === normalizedKey
        );

        if (!option) {
          console.warn(`Unknown option: ${key}`);
          continue;
        }

        if (option.type !== 'flag' && !this.validateType(value, option.type)) {
          throw new Error(
            `Invalid type for option ${key}: expected ${option.type}`
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
      case 'number':
        return typeof value === 'number' && !isNaN(value);
      case 'string':
      case 'file':
      case 'regex':
        return typeof value === 'string';
      case 'flag':
        return typeof value === 'boolean';
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
