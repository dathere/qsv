# QSV Agent Skills - Executor API

Complete Node.js/TypeScript implementation for loading and executing qsv skills.

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
//   total: 56,
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

## Examples

### Example 1: Data Cleaning

```typescript
const searchSkill = await loader.load('qsv-search');
const dedupSkill = await loader.load('qsv-dedup');
const selectSkill = await loader.load('qsv-select');

// Chain operations sequentially
const step1 = await executor.execute(searchSkill, {
  args: { regex: '^false$' },
  options: { select: 'test_account' },
  stdin: customerData
});

const step2 = await executor.execute(dedupSkill, {
  stdin: step1.output
});

const step3 = await executor.execute(selectSkill, {
  args: { selection: 'id,name,email,revenue' },
  stdin: step2.output
});

console.log(step3.output);
```

### Example 2: Analytics

```typescript
const selectSkill = await loader.load('qsv-select');
const statsSkill = await loader.load('qsv-stats');

const selected = await executor.execute(selectSkill, {
  args: { selection: 'revenue,age,signup_date' },
  stdin: customerData
});

const stats = await executor.execute(statsSkill, {
  options: { everything: true },
  stdin: selected.output
});

console.log(stats.output);
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
}
```

### SkillParams

```typescript
interface SkillParams {
  args?: Record<string, unknown>;     // Positional arguments
  options?: Record<string, unknown>;  // Command options
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

## Running Examples

```bash
# Basic skill usage
node examples/basic.js
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
| documentation | describegpt | AI-powered data documentation |
| utility | index, cat, headers | Utility operations |

## Performance Hints

Each skill includes performance metadata:

- **streamable**: Can process data as a stream (constant memory)
- **indexed**: Benefits from CSV index (faster random access)
- **memory**: Memory usage pattern (constant, proportional, full)

Use these hints to optimize execution performance:

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
# Should show 56 .json files
```

## License

MIT

## Links

- [qsv Repository](https://github.com/dathere/qsv)
- [Design Document](../design/AGENT_SKILLS_DESIGN.md)
- [Integration Guide](../design/AGENT_SKILLS_INTEGRATION.md)
- [Skill Generation POC](../design/AGENT_SKILLS_POC_SUMMARY.md)
