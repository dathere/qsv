# QSV Agent Skills - Executor & Pipeline API

Complete Node.js/TypeScript implementation for loading and executing qsv skills with a fluent pipeline composition API.

## Installation

```bash
cd .claude/skills
npm install
```

## Requirements

- Node.js >= 18.0.0
- qsv installed and in PATH (for execution)
- TypeScript 5.0+ (for development)

## Quick Start

### Basic Usage

```typescript
// For TypeScript projects
import { SkillLoader, SkillExecutor } from '@qsv/agent-skills';

// For JavaScript projects (after building)
import { SkillLoader, SkillExecutor } from './dist/index.js';

// Load all skills
const loader = new SkillLoader();
await loader.loadAll();

// Execute a skill
const executor = new SkillExecutor();
const skill = await loader.load('qsv-select');

const result = await executor.execute(skill, {
  args: { selection: '1,4' },
  stdin: csvData
});

console.log(result.output);
```

### Pipeline Composition

```typescript
import { SkillLoader, QsvPipeline } from './dist/index.js';

const loader = new SkillLoader();
await loader.loadAll();

// Create a data cleaning pipeline
const pipeline = new QsvPipeline(loader)
  .select('!SSN,password')           // Remove sensitive columns
  .dedup()                            // Remove duplicates
  .filter('^[^@]+@', 'email')        // Validate emails
  .sortBy('revenue', { reverse: true }) // Sort descending
  .slice(0, 100);                     // Top 100

// Execute pipeline
const result = await pipeline.execute(csvData);
console.log(result.output.toString());

// Or generate shell script
const shell = await pipeline.toShellScript();
console.log(shell);
// Output:
// qsv select '!SSN,password' | \
//   qsv dedup | \
//   qsv search -s email '^[^@]+@' | \
//   qsv sort --reverse -s revenue | \
//   qsv slice --end 100
```

## API Documentation

### SkillLoader

Loads and manages skill definitions from JSON files.

#### Methods

##### `loadAll(): Promise<Map<string, QsvSkill>>`
Load all skills from the directory.

```typescript
const loader = new SkillLoader();
const skills = await loader.loadAll();
console.log(`Loaded ${skills.size} skills`);
```

##### `load(skillName: string): Promise<QsvSkill | null>`
Load a specific skill by name.

```typescript
const skill = await loader.load('qsv-select');
```

##### `search(query: string): QsvSkill[]`
Search skills by query (matches name, description, category, examples).

```typescript
const dedupSkills = loader.search('duplicate');
```

##### `getByCategory(category: SkillCategory): QsvSkill[]`
Get all skills in a category.

```typescript
const aggregation = loader.getByCategory('aggregation');
// Returns: stats, moarstats, frequency, count, etc.
```

##### `getStats()`
Get statistics about loaded skills.

```typescript
const stats = loader.getStats();
console.log(stats);
// {
//   total: 65,
//   byCategory: { selection: 5, aggregation: 8, ... },
//   totalExamples: 200,
//   totalOptions: 450,
//   totalArgs: 50
// }
```

### SkillExecutor

Executes qsv skills by spawning qsv processes.

#### Constructor

```typescript
const executor = new SkillExecutor('qsv'); // Optional: custom binary path
```

#### Methods

##### `execute(skill: QsvSkill, params: SkillParams): Promise<SkillResult>`
Execute a skill with given parameters.

```typescript
const result = await executor.execute(skill, {
  args: { selection: '1-5' },
  options: { output: 'result.csv' },
  stdin: csvData
});

console.log(result.success);        // true/false
console.log(result.output);         // CSV output
console.log(result.metadata.command); // Executed command
console.log(result.metadata.duration); // Execution time (ms)
```

### QsvPipeline

Fluent API for composing multi-step data processing pipelines.

#### Constructor

```typescript
const pipeline = new QsvPipeline(loader, executor?);
```

#### Core Methods

##### `add(skillName: string, params: SkillParams): this`
Add a generic skill step.

```typescript
pipeline.add('qsv-custom', { args: { ... }, options: { ... } });
```

##### `execute(input: string | Buffer): Promise<PipelineResult>`
Execute the pipeline.

```typescript
const result = await pipeline.execute(csvData);
console.log(result.output);         // Final output
console.log(result.totalDuration);  // Total execution time
console.log(result.steps);          // Array of step results
```

##### `toShellScript(): Promise<string>`
Generate equivalent shell script.

```typescript
const script = await pipeline.toShellScript();
console.log(script);
```

#### Convenience Methods

All methods return `this` for chaining.

**Selection:**
- `select(selection: string, options?): this`
- `slice(start?, end?, options?): this`
- `head(n: number): this`

**Filtering:**
- `search(pattern: string, column?, options?): this`
- `filter(pattern: string, column?, options?): this` (alias for search)

**Transformation:**
- `dedup(options?): this`
- `sortBy(column: string, options?): this`
- `rename(columns: string, newNames: string, options?): this`
- `apply(operations: string, column: string, options?): this`
- `transpose(options?): this`

**Aggregation:**
- `stats(options?): this`
- `moarstats(options?): this`
- `frequency(options?): this`

**Joining:**
- `join(columns: string, file: string, options?): this`

## Examples

### Example 1: Data Cleaning

```typescript
const result = await new QsvPipeline(loader)
  .search('^false$', 'test_account')    // Remove test accounts
  .dedup()                               // Remove duplicates
  .filter('^[^@]+@[^@]+\\.[^@]+$', 'email') // Valid emails only
  .select('id,name,email,revenue')      // Select columns
  .sortBy('revenue', { reverse: true }) // Sort by revenue desc
  .execute(customerData);

console.log(result.output.toString());
```

### Example 2: Analytics

```typescript
const stats = await new QsvPipeline(loader)
  .select('revenue,age,signup_date')
  .stats({ everything: true })
  .execute(customerData);

console.log(stats.output.toString());
```

### Example 3: Data Transformation

```typescript
const transformed = await new QsvPipeline(loader)
  .select('!internal_id,debug_flag')
  .apply('upper', 'name')
  .apply('trim', '*')
  .sortBy('created_at')
  .execute(rawData);
```

### Example 4: Generate Shell Script

```typescript
const pipeline = new QsvPipeline(loader)
  .select('1-10')
  .dedup()
  .stats({ everything: true });

const shell = await pipeline.toShellScript();
console.log(shell);
// qsv select 1-10 | \
//   qsv dedup | \
//   qsv stats --everything
```

## Type Definitions

### QsvSkill

```typescript
interface QsvSkill {
  name: string;
  version: string;
  description: string;
  category: SkillCategory;
  command: CommandSpec;
  examples: Example[];
  hints?: BehavioralHints;
  test_file?: string;
}
```

### SkillParams

```typescript
interface SkillParams {
  args?: Record<string, any>;     // Positional arguments
  options?: Record<string, any>;  // Command options
  stdin?: string | Buffer;        // Input data
  inputFile?: string;             // Input file path
}
```

### SkillResult

```typescript
interface SkillResult {
  success: boolean;
  output: string;
  stderr: string;
  metadata: {
    command: string;      // Executed command
    duration: number;     // Execution time (ms)
    rowsProcessed?: number;
    exitCode: number;
  };
}
```

### PipelineResult

```typescript
interface PipelineResult {
  output: Buffer;           // Final output
  steps: SkillResult[];     // Results from each step
  totalDuration: number;    // Total execution time
}
```

## Running Examples

```bash
# Basic skill usage
node examples/basic.js

# Pipeline composition
node examples/pipeline.js
```

## Development

```bash
# Build TypeScript
npm run build

# Run tests
npm test
```

## Categories

Skills are organized by function:

| Category | Skills | Description |
|----------|--------|-------------|
| selection | select, slice, sample | Column/row selection |
| filtering | search, searchset, grep | Row filtering |
| transformation | apply, rename, transpose | Data transformation |
| aggregation | stats, moarstats, frequency, count | Statistical analysis |
| joining | join, joinp | Combining datasets |
| validation | schema, validate, safenames | Data validation |
| formatting | fmt, fixlengths, table | Output formatting |
| conversion | to, input, excel | Format conversion |
| analysis | correlation, describegpt | Advanced analysis |
| utility | index, cat, headers | Utility operations |

## Performance Hints

Each skill includes performance metadata:

- **streamable**: Can process data as a stream (constant memory)
- **indexed**: Benefits from CSV index (faster random access)
- **memory**: Memory usage pattern (constant, proportional, full)

Use these hints to optimize pipeline performance:

```typescript
const skill = await loader.load('qsv-stats');
console.log(skill.hints);
// { streamable: true, memory: 'constant' }

// For large files, prefer streamable skills
if (skill.hints?.streamable) {
  console.log('✅ Efficient for large files');
}
```

## Integration with Claude Agent SDK

The skills are designed to integrate seamlessly with the Claude Agent SDK:

```typescript
import { Agent } from '@anthropic-ai/agent-sdk';
import { SkillLoader } from '@qsv/agent-skills';

const loader = new SkillLoader();
await loader.loadAll();

const agent = new Agent({
  skills: Array.from((await loader.loadAll()).values())
});

// Agent can now discover and invoke qsv skills
await agent.chat("Remove duplicates from sales.csv");
// Agent automatically finds and invokes qsv-dedup
```

## Troubleshooting

### qsv command not found

Ensure qsv is installed and in your PATH:

```bash
which qsv
# or
qsv --version
```

Install from: https://github.com/dathere/qsv

### TypeScript compilation errors

Ensure you have TypeScript 5.0+:

```bash
npm install -g typescript@latest
tsc --version
```

### Skill not found

Check the skills directory:

```bash
ls -la .claude/skills/qsv/
# Should show 65 .json files
```

## License

MIT

## Links

- [qsv Repository](https://github.com/dathere/qsv)
- [Design Document](../../docs/AGENT_SKILLS_DESIGN.md)
- [Integration Guide](../../docs/AGENT_SKILLS_INTEGRATION.md)
- [Skill Generation POC](../../docs/AGENT_SKILLS_POC_SUMMARY.md)
