/**
 * QSV Pipeline Composition API
 * Provides a fluent interface for chaining qsv skills
 */

import type { SkillLoader } from "./loader.js";
import { SkillExecutor } from "./executor.js";
import type {
  SkillParams,
  PipelineStep,
  PipelineResult,
  SkillResult,
} from "./types.js";

export class QsvPipeline {
  private steps: PipelineStep[] = [];
  private loader: SkillLoader;
  private executor: SkillExecutor;

  constructor(loader: SkillLoader, executor?: SkillExecutor) {
    this.loader = loader;
    this.executor = executor || new SkillExecutor();
  }

  /**
   * Add a generic skill step
   */
  add(skillName: string, params: SkillParams): this {
    this.steps.push({ skillName, params });
    return this;
  }

  /**
   * Select columns
   */
  select(selection: string, options?: Record<string, any>): this {
    return this.add("qsv-select", {
      args: { selection },
      options,
    });
  }

  /**
   * Remove duplicate rows
   */
  dedup(options?: Record<string, any>): this {
    return this.add("qsv-dedup", {
      args: {},
      options,
    });
  }

  /**
   * Compute statistics
   */
  stats(options?: Record<string, any>): this {
    return this.add("qsv-stats", {
      args: {},
      options,
    });
  }

  /**
   * Compute extended statistics
   */
  moarstats(options?: Record<string, any>): this {
    return this.add("qsv-moarstats", {
      args: {},
      options,
    });
  }

  /**
   * Sort by column
   */
  sortBy(column: string, options?: Record<string, any>): this {
    return this.add("qsv-sort", {
      args: {},
      options: { ...options, select: column },
    });
  }

  /**
   * Search/filter rows
   */
  search(
    pattern: string,
    column?: string,
    options?: Record<string, any>,
  ): this {
    return this.add("qsv-search", {
      args: { regex: pattern },
      options: column ? { ...options, select: column } : options,
    });
  }

  /**
   * Filter rows (alias for search)
   */
  filter(
    pattern: string,
    column?: string,
    options?: Record<string, any>,
  ): this {
    return this.search(pattern, column, options);
  }

  /**
   * Frequency distribution
   */
  frequency(options?: Record<string, any>): this {
    return this.add("qsv-frequency", {
      args: {},
      options,
    });
  }

  /**
   * Join with another CSV
   */
  join(columns: string, file: string, options?: Record<string, any>): this {
    return this.add("qsv-join", {
      args: { columns1: columns, input1: file },
      options,
    });
  }

  /**
   * Rename columns
   */
  rename(
    columns: string,
    newNames: string,
    options?: Record<string, any>,
  ): this {
    return this.add("qsv-rename", {
      args: {},
      options: { ...options, rename: `${columns}:${newNames}` },
    });
  }

  /**
   * Apply transformations
   */
  apply(
    operations: string,
    column: string,
    options?: Record<string, any>,
  ): this {
    return this.add("qsv-apply", {
      args: { operations, column },
      options,
    });
  }

  /**
   * Slice rows
   */
  slice(start?: number, end?: number, options?: Record<string, any>): this {
    const sliceOptions = { ...options };
    if (start !== undefined) sliceOptions.start = start;
    if (end !== undefined) sliceOptions.end = end;

    return this.add("qsv-slice", {
      args: {},
      options: sliceOptions,
    });
  }

  /**
   * Take first N rows (alias for slice)
   */
  head(n: number): this {
    return this.slice(0, n);
  }

  /**
   * Transpose the CSV
   */
  transpose(options?: Record<string, any>): this {
    return this.add("qsv-transpose", {
      args: {},
      options,
    });
  }

  /**
   * Execute the pipeline
   */
  async execute(input: string | Buffer): Promise<PipelineResult> {
    const results: SkillResult[] = [];
    let currentData: Buffer =
      typeof input === "string" ? Buffer.from(input) : input;

    for (const step of this.steps) {
      const skill = await this.loader.load(step.skillName);

      if (!skill) {
        throw new Error(`Skill not found: ${step.skillName}`);
      }

      // Pass output from previous step as input
      const result = await this.executor.execute(skill, {
        ...step.params,
        stdin: currentData,
      });

      if (!result.success) {
        throw new Error(
          `Pipeline failed at step ${step.skillName}: ${result.stderr}`,
        );
      }

      currentData = Buffer.from(result.output);
      results.push(result);
    }

    return {
      output: currentData,
      steps: results,
      totalDuration: results.reduce((sum, r) => sum + r.metadata.duration, 0),
    };
  }

  /**
   * Generate equivalent shell script
   */
  async toShellScript(): Promise<string> {
    const commands: string[] = [];

    for (const step of this.steps) {
      const skill = await this.loader.load(step.skillName);

      if (!skill) {
        throw new Error(`Skill not found: ${step.skillName}`);
      }

      const executor = new SkillExecutor();
      const command = executor.buildCommand(skill, step.params);
      commands.push(command);
    }

    return commands.join(" | \\\n  ");
  }

  /**
   * Clear all steps
   */
  clear(): this {
    this.steps = [];
    return this;
  }

  /**
   * Get current steps
   */
  getSteps(): PipelineStep[] {
    return [...this.steps];
  }
}
