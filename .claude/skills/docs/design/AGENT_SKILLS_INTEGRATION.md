# QSV Skills Integration with Claude Agent SDK

> **Note**: This document was written during the design phase. The actual
> integration uses the Model Context Protocol (MCP) rather than a hypothetical
> "Claude Agent SDK". See `README-MCP.md` for current implementation details.
>
> **Last Updated**: 2026-01-25 | **Version**: 14.2.0

## Overview

This document demonstrates how qsv-generated Agent Skills integrate with the Claude Agent SDK, showing practical examples, API design, and developer workflows.

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Claude Agent Runtime                      │
│                                                              │
│  ┌──────────────┐      ┌──────────────┐    ┌─────────────┐ │
│  │   Agent      │─────▶│ Skill Engine │───▶│  Executor   │ │
│  │  (Claude AI) │      └──────────────┘    └─────────────┘ │
│  └──────────────┘             │                    │        │
│                                │                    │        │
└────────────────────────────────┼────────────────────┼────────┘
                                 │                    │
                                 ▼                    ▼
                    ┌─────────────────────┐  ┌──────────────┐
                    │  Skill Registry     │  │ qsv Binary   │
                    │  (.claude/skills/)  │  │ (spawned)    │
                    │                     │  └──────────────┘
                    │  - qsv-select.json  │
                    │  - qsv-stats.json   │
                    │  - qsv-join.json    │
                    │  - ...              │
                    └─────────────────────┘
```

## Skill Discovery

### 1. Loading Skills at Runtime

```typescript
// skills/loader.ts
// Note: This example uses a hypothetical SDK API for illustration.
// The actual implementation uses MCP (Model Context Protocol).
// See src/mcp-server.ts for the real implementation.
import { readdir, readFile } from 'fs/promises';
import { join } from 'path';

export class QsvSkillLoader {
  async loadAll(): Promise<SkillRegistry> {
    const skillsDir = '.claude/skills/qsv';
    const registry = new SkillRegistry();

    // Load all skill JSON files
    const files = await readdir(skillsDir);
    for (const file of files.filter(f => f.endsWith('.json'))) {
      const skillPath = join(skillsDir, file);
      const skillDef = JSON.parse(await readFile(skillPath, 'utf-8'));

      // Register skill with executor
      registry.register(
        skillDef.name,
        new QsvSkillExecutor(skillDef)
      );
    }

    return registry;
  }

  async search(query: string): Promise<QsvSkill[]> {
    const allSkills = await this.loadAll();

    // Search by description, category, examples
    return allSkills.filter(skill =>
      skill.description.includes(query) ||
      skill.category.includes(query) ||
      skill.examples.some(ex => ex.description.includes(query))
    );
  }

  async getByCategory(category: string): Promise<QsvSkill[]> {
    const allSkills = await this.loadAll();
    return allSkills.filter(skill => skill.category === category);
  }
}
```

### 2. Agent Discovery Flow

```typescript
// Example: Agent searching for skills
import { Agent } from '@anthropic-ai/agent-sdk';

const agent = new Agent({
  apiKey: process.env.ANTHROPIC_API_KEY,
  skills: await QsvSkillLoader.loadAll()
});

// User asks: "How do I remove duplicate rows from my CSV?"
const response = await agent.chat({
  message: "How do I remove duplicate rows from my CSV?",
  context: {
    files: ["data.csv"]
  }
});

// Agent internally:
// 1. Searches skills: "duplicate rows" → finds qsv-dedup
// 2. Reads skill definition
// 3. Proposes solution with skill invocation
```

## Skill Execution

### 1. Basic Executor Implementation

```typescript
// skills/executor.ts
import { spawn } from 'child_process';
import { createReadStream, createWriteStream } from 'fs';
import { pipeline } from 'stream/promises';

export class QsvSkillExecutor {
  constructor(private skillDef: QsvSkill) {}

  async execute(params: SkillParams): Promise<SkillResult> {
    // Validate parameters
    this.validateParams(params);

    // Build command arguments
    const args = this.buildArgs(params);

    // Execute qsv command
    const result = await this.runQsv(args, params);

    return {
      success: result.exitCode === 0,
      output: result.output,
      stderr: result.stderr,
      metadata: {
        command: `qsv ${args.join(' ')}`,
        duration: result.duration,
        rowsProcessed: this.extractRowCount(result.stderr)
      }
    };
  }

  private buildArgs(params: SkillParams): string[] {
    const args = [this.skillDef.command.subcommand];

    // Add options/flags
    for (const [key, value] of Object.entries(params.options || {})) {
      const option = this.skillDef.command.options.find(o =>
        o.flag === `--${key}` || o.short === `-${key}`
      );

      if (!option) continue;

      if (option.type === 'flag') {
        if (value) args.push(option.flag);
      } else {
        args.push(option.flag, String(value));
      }
    }

    // Add positional arguments
    for (const arg of this.skillDef.command.args) {
      const value = params.args[arg.name];
      if (value !== undefined) {
        args.push(String(value));
      } else if (arg.required) {
        throw new Error(`Missing required argument: ${arg.name}`);
      }
    }

    return args;
  }

  private async runQsv(
    args: string[],
    params: SkillParams
  ): Promise<ExecutionResult> {
    const startTime = Date.now();

    return new Promise((resolve, reject) => {
      const proc = spawn('qsv', args, {
        stdio: ['pipe', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      // Handle input (stdin or file)
      if (params.stdin) {
        proc.stdin.write(params.stdin);
        proc.stdin.end();
      } else if (params.inputFile) {
        const input = createReadStream(params.inputFile);
        input.pipe(proc.stdin);
      }

      // Collect output
      proc.stdout.on('data', chunk => stdout += chunk);
      proc.stderr.on('data', chunk => stderr += chunk);

      proc.on('close', exitCode => {
        resolve({
          exitCode,
          output: stdout,
          stderr,
          duration: Date.now() - startTime
        });
      });

      proc.on('error', reject);
    });
  }

  private validateParams(params: SkillParams): void {
    // Validate required arguments
    for (const arg of this.skillDef.command.args) {
      if (arg.required && !(arg.name in params.args)) {
        throw new Error(`Missing required argument: ${arg.name}`);
      }

      // Type validation
      const value = params.args[arg.name];
      if (value && !this.validateType(value, arg.type)) {
        throw new Error(
          `Invalid type for ${arg.name}: expected ${arg.type}`
        );
      }

      // Custom validation (regex, range, etc.)
      if (arg.validation) {
        this.validateConstraints(value, arg.validation);
      }
    }

    // Validate option dependencies
    for (const option of this.skillDef.command.options) {
      if (option.requires && params.options?.[option.flag]) {
        for (const required of option.requires) {
          if (!params.options?.[required]) {
            throw new Error(
              `${option.flag} requires ${required}`
            );
          }
        }
      }
    }
  }

  private validateType(value: any, type: string): boolean {
    switch (type) {
      case 'number':
        return typeof value === 'number' && !isNaN(value);
      case 'string':
        return typeof value === 'string';
      case 'file':
        return typeof value === 'string'; // TODO: check file exists
      case 'regex':
        try {
          new RegExp(value);
          return true;
        } catch {
          return false;
        }
      default:
        return true;
    }
  }
}
```

### 2. Example: Single Skill Invocation

```typescript
// examples/basic-execution.ts
import { QsvSkillLoader } from './skills/loader';

async function example1() {
  const loader = new QsvSkillLoader();
  const selectSkill = await loader.load('qsv-select');

  // Execute: qsv select 1,4 data.csv -o output.csv
  const result = await selectSkill.execute({
    args: {
      selection: '1,4',
      input: 'data.csv'
    },
    options: {
      output: 'output.csv'
    }
  });

  console.log('Success:', result.success);
  console.log('Command:', result.metadata.command);
  console.log('Duration:', result.metadata.duration, 'ms');
}
```

## Skill Composition

### 1. Pipeline Builder

```typescript
// skills/pipeline.ts
export class QsvPipeline {
  private steps: PipelineStep[] = [];

  constructor(private loader: QsvSkillLoader) {}

  // Fluent API for building pipelines
  add(skillName: string, params: SkillParams): this {
    this.steps.push({ skillName, params });
    return this;
  }

  select(selection: string): this {
    return this.add('qsv-select', {
      args: { selection }
    });
  }

  dedup(): this {
    return this.add('qsv-dedup', { args: {} });
  }

  stats(options?: { everything?: boolean }): this {
    return this.add('qsv-stats', {
      args: {},
      options
    });
  }

  sortBy(column: string): this {
    return this.add('qsv-sort', {
      args: { column }
    });
  }

  filter(pattern: string, column?: string): this {
    return this.add('qsv-search', {
      args: { regex: pattern },
      options: column ? { select: column } : {}
    });
  }

  // Execute pipeline
  async execute(input: string | Buffer): Promise<PipelineResult> {
    let currentData = typeof input === 'string'
      ? await readFile(input)
      : input;

    const results: SkillResult[] = [];

    for (const step of this.steps) {
      const skill = await this.loader.load(step.skillName);

      // Pass output from previous step as input
      const result = await skill.execute({
        ...step.params,
        stdin: currentData
      });

      if (!result.success) {
        throw new Error(
          `Pipeline failed at step ${step.skillName}: ${result.stderr}`
        );
      }

      currentData = Buffer.from(result.output);
      results.push(result);
    }

    return {
      output: currentData,
      steps: results,
      totalDuration: results.reduce((sum, r) => sum + r.metadata.duration, 0)
    };
  }

  // Generate shell script equivalent
  toShellScript(): string {
    const commands = this.steps.map(step => {
      const skill = this.loader.loadSync(step.skillName);
      const executor = new QsvSkillExecutor(skill);
      const args = executor.buildArgs(step.params);
      return `qsv ${args.join(' ')}`;
    });

    return commands.join(' | \\\n  ');
  }
}
```

### 2. Example: Data Cleaning Pipeline

```typescript
// examples/pipeline-composition.ts
async function cleanDataPipeline() {
  const loader = new QsvSkillLoader();
  const pipeline = new QsvPipeline(loader);

  // Build a data cleaning pipeline
  const result = await pipeline
    .select('!SSN,password,account_no')  // Remove sensitive columns
    .dedup()                              // Remove duplicates
    .filter('^[^@]+@[^@]+\\.[^@]+$', 'email')  // Filter invalid emails
    .sortBy('date')                       // Sort by date
    .execute('raw_data.csv');

  console.log('Pipeline completed in', result.totalDuration, 'ms');
  console.log('Steps executed:');
  result.steps.forEach((step, i) => {
    console.log(`  ${i + 1}. ${step.metadata.command} (${step.metadata.duration}ms)`);
  });

  // Write output
  await writeFile('cleaned_data.csv', result.output);

  // Print equivalent shell command
  console.log('\nEquivalent shell command:');
  console.log(pipeline.toShellScript());
}

// Output:
// Pipeline completed in 1234 ms
// Steps executed:
//   1. qsv select !SSN,password,account_no (123ms)
//   2. qsv dedup (456ms)
//   3. qsv search -s email '^[^@]+@[^@]+\.[^@]+$' (345ms)
//   4. qsv sort -s date (310ms)
//
// Equivalent shell command:
// qsv select '!SSN,password,account_no' | \
//   qsv dedup | \
//   qsv search -s email '^[^@]+@[^@]+\.[^@]+$' | \
//   qsv sort -s date
```

## Agent Integration Examples

### 1. Conversational Data Analysis

```typescript
// examples/conversational-agent.ts
import { Agent } from '@anthropic-ai/agent-sdk';

const agent = new Agent({
  apiKey: process.env.ANTHROPIC_API_KEY,
  skills: await new QsvSkillLoader().loadAll()
});

// Multi-turn conversation
const session = agent.createSession();

await session.message("I have a sales CSV with duplicate entries");
// Agent: "I can help you remove duplicates. What file is it?"

await session.message("sales_2024.csv");
// Agent: "I'll remove duplicates from sales_2024.csv"
// [Invokes qsv-dedup skill]
// Agent: "Done! Removed 145 duplicate rows. The cleaned data is in sales_2024_deduped.csv"

await session.message("Now show me statistics for the revenue column");
// Agent: "I'll compute statistics for the revenue column"
// [Invokes qsv-stats with --select revenue]
// Agent: "Here are the statistics for revenue:
//         Mean: $12,345.67
//         Median: $10,234.00
//         StdDev: $5,432.10
//         Min: $100.00
//         Max: $95,000.00"

await session.message("Which products had revenue over $50,000?");
// Agent: "I'll filter for high-revenue products"
// [Invokes qsv-search or qsv-filter]
// Agent: "Found 23 products with revenue over $50,000:
//         1. Product A: $95,000
//         2. Product B: $87,500
//         ..."
```

### 2. Automated Data Quality Workflow

```typescript
// examples/data-quality-agent.ts
class DataQualityAgent {
  constructor(private agent: Agent) {}

  async assessQuality(csvFile: string): Promise<QualityReport> {
    // 1. Get basic stats
    const statsResult = await this.agent.invokeSkill('qsv-stats', {
      args: { input: csvFile },
      options: { everything: true }
    });

    // 2. Validate schema
    const schemaResult = await this.agent.invokeSkill('qsv-schema', {
      args: { input: csvFile }
    });

    // 3. Check for duplicates
    const dedupResult = await this.agent.invokeSkill('qsv-dedup', {
      args: { input: csvFile },
      options: { dupes_output: 'duplicates.csv' }
    });

    // 4. Analyze cardinality
    const freqResult = await this.agent.invokeSkill('qsv-frequency', {
      args: { input: csvFile },
      options: { limit: 10 }
    });

    // Agent analyzes results and generates report
    const report = await this.agent.chat({
      message: `Analyze this data quality:
        - Stats: ${statsResult.output}
        - Schema: ${schemaResult.output}
        - Duplicates: ${dedupResult.metadata.rowsProcessed} found
        - Frequency distribution: ${freqResult.output}

        Provide a quality score (0-100) and recommendations.`,
      context: { csvFile }
    });

    return JSON.parse(report);
  }
}

// Usage
const agent = new Agent({
  skills: await new QsvSkillLoader().loadAll()
});
const qualityAgent = new DataQualityAgent(agent);

const report = await qualityAgent.assessQuality('customer_data.csv');
console.log('Quality Score:', report.score);
console.log('Issues Found:', report.issues);
console.log('Recommendations:', report.recommendations);
```

### 3. Smart Data Transformation

```typescript
// examples/smart-transform.ts
async function smartTransform() {
  const agent = new Agent({
    skills: await new QsvSkillLoader().loadAll()
  });

  // User provides natural language transformation request
  const request = `
    I need to transform my sales data:
    1. Remove test accounts (username contains 'test')
    2. Convert all dates to ISO format
    3. Add a 'revenue_tier' column: <1000=low, 1000-5000=medium, >5000=high
    4. Sort by revenue descending
    5. Export to Excel
  `;

  // Agent plans and executes
  const plan = await agent.plan({
    goal: request,
    input: 'sales.csv',
    constraints: {
      preserveOriginal: true,
      validateOutput: true
    }
  });

  console.log('Execution Plan:');
  plan.steps.forEach((step, i) => {
    console.log(`${i + 1}. ${step.skill}: ${step.description}`);
  });

  // Execute plan with user confirmation
  const confirmed = await confirmPlan(plan);
  if (confirmed) {
    const result = await agent.executePlan(plan);
    console.log('Transformation complete:', result.output);
  }
}

// Agent generates plan:
// 1. qsv-search: Filter out test accounts
// 2. qsv-datefmt: Convert dates to ISO format
// 3. qsv-apply: Add revenue_tier column with lua script
// 4. qsv-sort: Sort by revenue descending
// 5. qsv-excel: Export to Excel format
```

## Advanced Features

### 1. Caching & Optimization

```typescript
// skills/cache.ts
export class SkillCache {
  private cache = new Map<string, CachedResult>();

  async executeWithCache(
    skill: QsvSkill,
    params: SkillParams
  ): Promise<SkillResult> {
    const cacheKey = this.generateKey(skill, params);

    // Check cache
    const cached = this.cache.get(cacheKey);
    if (cached && !this.isExpired(cached)) {
      console.log('Cache hit:', skill.name);
      return cached.result;
    }

    // Execute and cache
    const result = await skill.execute(params);
    this.cache.set(cacheKey, {
      result,
      timestamp: Date.now(),
      inputHash: await this.hashInput(params)
    });

    return result;
  }

  private generateKey(skill: QsvSkill, params: SkillParams): string {
    return `${skill.name}:${JSON.stringify(params)}`;
  }

  private isExpired(cached: CachedResult): boolean {
    const maxAge = 5 * 60 * 1000; // 5 minutes
    return Date.now() - cached.timestamp > maxAge;
  }
}
```

### 2. Streaming Support

```typescript
// skills/streaming.ts
export class StreamingExecutor {
  async executeStream(
    skill: QsvSkill,
    params: SkillParams,
    onChunk: (chunk: string) => void
  ): Promise<void> {
    const args = this.buildArgs(params);
    const proc = spawn('qsv', args);

    // Stream output as it arrives
    proc.stdout.on('data', chunk => {
      onChunk(chunk.toString());
    });

    return new Promise((resolve, reject) => {
      proc.on('close', code => {
        code === 0 ? resolve() : reject(new Error(`Exit code: ${code}`));
      });
    });
  }
}

// Usage: Real-time progress for large files
const executor = new StreamingExecutor();
let rowsProcessed = 0;

await executor.executeStream(
  selectSkill,
  { args: { selection: '1-5', input: 'huge.csv' } },
  (chunk) => {
    rowsProcessed += chunk.split('\n').length - 1;
    console.log(`Processed ${rowsProcessed} rows...`);
  }
);
```

### 3. Parallel Execution

```typescript
// skills/parallel.ts
export class ParallelPipeline {
  async executeFanOut(
    input: string,
    branches: PipelineStep[][]
  ): Promise<SkillResult[]> {
    // Execute multiple pipelines in parallel on same input
    const results = await Promise.all(
      branches.map(branch =>
        this.executeBranch(input, branch)
      )
    );

    return results;
  }

  async executeFanIn(
    inputs: string[],
    mergeSkill: string
  ): Promise<SkillResult> {
    // Merge multiple inputs using skill (e.g., qsv-cat)
    const skill = await this.loader.load(mergeSkill);
    return skill.execute({
      args: { inputs }
    });
  }
}

// Example: Parallel analysis
const parallel = new ParallelPipeline();

const [statsResult, freqResult, schemaResult] = await parallel.executeFanOut(
  'data.csv',
  [
    [{ skill: 'qsv-stats', params: { options: { everything: true }}}],
    [{ skill: 'qsv-frequency', params: {}}],
    [{ skill: 'qsv-schema', params: {}}]
  ]
);
```

## Error Handling

### 1. Graceful Degradation

```typescript
// skills/errors.ts
export class RobustExecutor {
  async executeWithRetry(
    skill: QsvSkill,
    params: SkillParams,
    options: RetryOptions = {}
  ): Promise<SkillResult> {
    const maxRetries = options.maxRetries || 3;
    const backoff = options.backoff || 1000;

    for (let attempt = 0; attempt < maxRetries; attempt++) {
      try {
        return await skill.execute(params);
      } catch (error) {
        if (attempt === maxRetries - 1) throw error;

        // Wait before retry (exponential backoff)
        await new Promise(resolve =>
          setTimeout(resolve, backoff * Math.pow(2, attempt))
        );

        console.log(`Retry ${attempt + 1}/${maxRetries}...`);
      }
    }
  }

  async executeWithFallback(
    primarySkill: QsvSkill,
    fallbackSkill: QsvSkill,
    params: SkillParams
  ): Promise<SkillResult> {
    try {
      return await primarySkill.execute(params);
    } catch (error) {
      console.warn('Primary skill failed, trying fallback:', error);
      return await fallbackSkill.execute(params);
    }
  }
}

// Example: Try Polars-powered join, fallback to regular join
const executor = new RobustExecutor();
const result = await executor.executeWithFallback(
  await loader.load('qsv-joinp'),  // Polars version
  await loader.load('qsv-join'),   // Regular version
  joinParams
);
```

### 2. Validation & User Feedback

```typescript
// skills/validation.ts
export class ValidatingExecutor {
  async executeWithValidation(
    skill: QsvSkill,
    params: SkillParams
  ): Promise<SkillResult> {
    // Pre-execution validation
    const validation = await this.validatePreConditions(skill, params);
    if (!validation.valid) {
      throw new ValidationError(validation.errors);
    }

    // Execute
    const result = await skill.execute(params);

    // Post-execution validation
    if (!this.validateResult(result)) {
      throw new Error('Result validation failed');
    }

    return result;
  }

  private async validatePreConditions(
    skill: QsvSkill,
    params: SkillParams
  ): Promise<ValidationResult> {
    const errors: string[] = [];

    // Check file exists
    if (params.args.input) {
      if (!await fileExists(params.args.input)) {
        errors.push(`Input file not found: ${params.args.input}`);
      }
    }

    // Check feature flags
    if (skill.requiredFeatures) {
      const available = await this.checkFeatures();
      const missing = skill.requiredFeatures.filter(f => !available.includes(f));
      if (missing.length > 0) {
        errors.push(`Missing required features: ${missing.join(', ')}`);
      }
    }

    // Check memory requirements
    if (skill.hints.memory === 'full') {
      const fileSize = await getFileSize(params.args.input);
      const availableMemory = getAvailableMemory();
      if (fileSize > availableMemory * 0.8) {
        errors.push('File too large for available memory');
      }
    }

    return {
      valid: errors.length === 0,
      errors
    };
  }
}
```

## Testing Integration

### 1. Skill Testing Framework

```typescript
// test/skill-test-runner.ts
export class SkillTestRunner {
  async testAllSkills(): Promise<TestReport> {
    const loader = new QsvSkillLoader();
    const skills = await loader.loadAll();
    const results: TestResult[] = [];

    for (const skill of skills) {
      const result = await this.testSkill(skill);
      results.push(result);
    }

    return { results, summary: this.summarize(results) };
  }

  async testSkill(skill: QsvSkill): Promise<TestResult> {
    const tests: TestCase[] = [];

    // Test each example
    for (const example of skill.examples) {
      tests.push(await this.testExample(skill, example));
    }

    // Test parameter validation
    tests.push(await this.testValidation(skill));

    return {
      skillName: skill.name,
      tests,
      passed: tests.every(t => t.passed)
    };
  }

  private async testExample(
    skill: QsvSkill,
    example: Example
  ): Promise<TestCase> {
    try {
      // Parse example command
      const params = this.parseExample(example.command);

      // Create test data if needed
      const testData = await this.createTestData(skill);

      // Execute
      const result = await skill.execute({
        ...params,
        args: { ...params.args, input: testData }
      });

      return {
        name: example.description,
        passed: result.success,
        error: result.success ? null : result.stderr
      };
    } catch (error) {
      return {
        name: example.description,
        passed: false,
        error: error.message
      };
    }
  }
}
```

## Performance Monitoring

```typescript
// skills/monitoring.ts
export class SkillMonitor {
  private metrics = new Map<string, SkillMetrics>();

  recordExecution(
    skillName: string,
    duration: number,
    inputSize: number,
    success: boolean
  ): void {
    const metrics = this.metrics.get(skillName) || {
      executions: 0,
      totalDuration: 0,
      totalInputSize: 0,
      failures: 0
    };

    metrics.executions++;
    metrics.totalDuration += duration;
    metrics.totalInputSize += inputSize;
    if (!success) metrics.failures++;

    this.metrics.set(skillName, metrics);
  }

  getReport(skillName: string): PerformanceReport {
    const metrics = this.metrics.get(skillName);
    if (!metrics) return null;

    return {
      averageDuration: metrics.totalDuration / metrics.executions,
      averageInputSize: metrics.totalInputSize / metrics.executions,
      successRate: (metrics.executions - metrics.failures) / metrics.executions,
      throughput: metrics.totalInputSize / metrics.totalDuration // bytes/ms
    };
  }

  async suggestOptimizations(
    pipeline: PipelineStep[]
  ): Promise<Optimization[]> {
    // Analyze pipeline for optimization opportunities
    const suggestions: Optimization[] = [];

    // 1. Suggest indexing
    if (this.shouldIndex(pipeline)) {
      suggestions.push({
        type: 'index',
        description: 'Add index for faster repeated access',
        command: 'qsv index input.csv'
      });
    }

    // 2. Suggest reordering
    const reordered = this.optimizeOrder(pipeline);
    if (reordered !== pipeline) {
      suggestions.push({
        type: 'reorder',
        description: 'Reorder operations for better performance',
        newPipeline: reordered
      });
    }

    // 3. Suggest parallelization
    const parallel = this.findParallelizable(pipeline);
    if (parallel.length > 0) {
      suggestions.push({
        type: 'parallel',
        description: 'Execute these steps in parallel',
        steps: parallel
      });
    }

    return suggestions;
  }
}
```

## Developer Experience

### CLI Tool for Testing Skills

```bash
# Install CLI
npm install -g @qsv/agent-skills

# Test a skill
qsv-skill test qsv-select --example 0

# Test all skills
qsv-skill test-all

# Generate skill from usage text
qsv-skill generate src/cmd/newcommand.rs

# Validate skill definition
qsv-skill validate .claude/skills/qsv/qsv-select.json

# Interactive skill explorer
qsv-skill explore
```

### VS Code Extension

```typescript
// Extension provides:
// - Skill autocomplete in code
// - Inline documentation from skill definitions
// - Quick actions to test skills
// - Pipeline visualization
```

## Summary

This integration approach provides:

1. **Seamless Discovery**: Load all skills from JSON files
2. **Type-Safe Execution**: Validate parameters before execution
3. **Composable Pipelines**: Chain skills with fluent API
4. **Error Handling**: Retries, fallbacks, validation
5. **Performance Monitoring**: Track metrics and suggest optimizations
6. **Developer Tools**: CLI, testing framework, VS Code extension

The auto-generated skills from qsv usage text become first-class citizens in the Agent SDK ecosystem, enabling powerful data workflows with minimal manual coding.

---

**Next Steps**: Implement proof-of-concept parser and generate first 5 skills to validate this integration design.

---

## Current Implementation (v14.2.0)

The design concepts in this document were implemented using the **Model Context
Protocol (MCP)** rather than a standalone SDK. Key differences:

| Design Concept | Actual Implementation |
|----------------|----------------------|
| `@anthropic-ai/agent-sdk` | `@modelcontextprotocol/sdk` |
| SkillRegistry | MCP ListTools/CallTool handlers |
| Agent.invokeSkill() | MCP tool invocation |
| Pipeline builder | `mcp-pipeline.ts` |

### New Features in v14.2.0

- **Tool Search**: `qsv_search_tools` for discovering commands by keyword/category
- **Expose-All-Tools Mode**: Auto-detection for tool-search-capable clients
- **Client Detection**: Identifies Claude Desktop, Code, Cowork
- **Guidance Enhancement**: USE WHEN, COMMON PATTERNS, CAUTION hints
- **MCPB Packaging**: Desktop extension bundle for easy installation

See `CLAUDE.md` and `README-MCP.md` for current documentation.
