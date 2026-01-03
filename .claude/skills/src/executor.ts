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

    // Add options/flags first
    if (params.options) {
      for (const [key, value] of Object.entries(params.options)) {
        // Find option definition
        const option = skill.command.options.find(o =>
          o.flag === `--${key}` || o.short === `-${key}` || o.flag.replace('--', '') === key
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
      const proc = spawn(this.qsvBinary, args, {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      // Handle input
      if (params.stdin) {
        proc.stdin.write(params.stdin);
        proc.stdin.end();
      } else {
        proc.stdin.end();
      }

      // Collect output
      proc.stdout.on('data', chunk => {
        stdout += chunk.toString();
      });

      proc.stderr.on('data', chunk => {
        stderr += chunk.toString();
      });

      proc.on('close', exitCode => {
        resolve({
          exitCode: exitCode || 0,
          stdout,
          stderr
        });
      });

      proc.on('error', reject);
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
        const option = skill.command.options.find(o =>
          o.flag === `--${key}` || o.short === `-${key}` || o.flag.replace('--', '') === key
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
