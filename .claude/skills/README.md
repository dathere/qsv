# QSV Agent Skills

Complete TypeScript implementation for loading, executing, and composing qsv command pipelines with the Claude Agent SDK.

## Overview

This directory contains:

1. **66 Auto-generated Skill Definitions** - JSON files describing all qsv commands
2. **TypeScript Executor** - Complete implementation for running qsv skills
3. **Pipeline Composition API** - Fluent interface for chaining operations
4. **Working Examples** - Practical demonstrations of the system

Each skill file provides:
- **Command specification**: Binary, subcommand, arguments, and options
- **Rich descriptions**: Extracted from usage text
- **Examples**: Real usage examples from documentation (417 total)
- **Type information**: Inferred parameter types and validation
- **Performance hints**: Memory usage, streaming capability, indexing benefits
- **Links to tests**: For additional context and validation

## Quick Start

### Installation

```bash
cd .claude/skills
npm install
npm run build
```

### Run Examples

```bash
# Basic skill loading and execution
npm test

# Pipeline composition
npm run test-pipeline
```

## Generated Skills (66)

| Category | Count | Skills |
|----------|-------|--------|
| **utility** | 38 | cat, clipboard, count, edit, headers, index, input, lens, partition, pro, pseudo, reverse, sniff, split, template, etc. |
| **transformation** | 5 | apply, applydp, rename, replace, transpose |
| **aggregation** | 4 | frequency, moarstats, stats, count |
| **conversion** | 5 | excel, input, json, jsonl, to, tojsonl |
| **selection** | 3 | select, slice, sample |
| **filtering** | 2 | search, searchset |
| **formatting** | 3 | fmt, fixlengths, table |
| **joining** | 2 | join, joinp |
| **validation** | 3 | schema, safenames, validate |
| **analysis** | 1 | describegpt |

**Total Statistics:**
- **Skills**: 66 commands
- **Examples**: 417 usage examples
- **Options**: 837 command-line options
- **Arguments**: 60 positional arguments

## Project Structure

```
.claude/skills/
├── qsv/                    # 66 skill JSON definitions
│   ├── qsv-select.json
│   ├── qsv-stats.json
│   ├── qsv-moarstats.json
│   └── ... (63 more)
├── src/                    # TypeScript source
│   ├── types.ts           # Type definitions
│   ├── loader.ts          # Skill loading & search
│   ├── executor.ts        # qsv execution wrapper
│   ├── pipeline.ts        # Pipeline composition API
│   └── index.ts           # Public exports
├── examples/               # Working examples
│   ├── basic.js           # Skill loading & execution
│   └── pipeline.js        # Pipeline composition
├── dist/                   # Compiled JavaScript (gitignored)
├── package.json
├── tsconfig.json
├── README.md              # This file
└── SKILLS_README.md       # Complete API documentation
```

## Usage

### Basic Skill Execution

```typescript
import { SkillLoader, SkillExecutor } from './dist/index.js';

// Load all 66 skills
const loader = new SkillLoader();
await loader.loadAll();

// Search for skills
const dedupSkills = loader.search('duplicate');
// Returns: qsv-dedup, qsv-extdedup, qsv-diff, etc.

// Execute a skill
const executor = new SkillExecutor();
const skill = await loader.load('qsv-select');

const result = await executor.execute(skill, {
  args: { selection: '1-5' },
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
  .search('^[^@]+@', 'email')        // Validate emails
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
//   qsv slice --start 0 --end 100
```

## Skill Schema

Each skill JSON file follows this structure:

```json
{
  "name": "qsv-<command>",
  "version": "12.0.0",
  "description": "Command description from usage text",
  "category": "selection|filtering|transformation|aggregation|...",
  "command": {
    "binary": "qsv",
    "subcommand": "<command>",
    "args": [
      {
        "name": "argument_name",
        "type": "string|number|file|regex",
        "required": true|false,
        "description": "Argument description",
        "examples": []
      }
    ],
    "options": [
      {
        "flag": "--option",
        "short": "-o",
        "type": "flag|string|number",
        "description": "Option description",
        "default": "value"
      }
    ]
  },
  "examples": [
    {
      "description": "What this example does",
      "command": "qsv command example"
    }
  ],
  "hints": {
    "streamable": true,
    "indexed": false,
    "memory": "constant"
  },
  "test_file": "https://github.com/dathere/qsv/blob/master/tests/test_<command>.rs"
}
```

## Generation

Skills are auto-generated from qsv command USAGE text using `qsv-skill-gen`:

```bash
# Generate all 66 skills
cargo run --bin qsv-skill-gen --features all_features

# Output: .claude/skills/qsv/*.json
```

The generator:
1. Extracts `USAGE` static string from command source files
2. Parses description, examples, arguments, and options
3. Infers types from names and descriptions
4. Detects performance hints from emoji markers (🤯 📇 🏎️ 😣)
5. Generates structured JSON skill definitions

## Type Inference

The generator infers parameter types:

| Pattern | Inferred Type |
|---------|---------------|
| `<input>`, `<file>` | `file` |
| `<number>`, `<count>` | `number` |
| `<regex>`, `<pattern>` | `regex` |
| `<selection>`, `<column>` | `string` |
| Default | `string` |

## Performance Hints

Hints are extracted from usage text markers:

- 🤯 → `memory: "full"` (loads entire file)
- 📇 → `indexed: true` (benefits from index)
- 🏎️ → `parallel: true` (supports parallel execution)
- 😣 → `memory: "proportional"` (memory scales with cardinality)

## API Documentation

See [SKILLS_README.md](./SKILLS_README.md) for complete API documentation including:

- SkillLoader API (loading, searching, statistics)
- SkillExecutor API (execution, validation)
- QsvPipeline API (15 convenience methods)
- Type definitions
- Advanced examples

## Integration with Claude Agent SDK

```typescript
import { Agent } from '@anthropic-ai/agent-sdk';
import { SkillLoader } from './dist/index.js';

const loader = new SkillLoader();
await loader.loadAll();

const agent = new Agent({
  skills: Array.from((await loader.loadAll()).values())
});

// Agent can now discover and invoke qsv skills
await agent.chat("Remove duplicates from sales.csv");
// Agent automatically finds and invokes qsv-dedup
```

## Development

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run tests
npm test
npm run test-pipeline

# Rebuild skills (from qsv repo root)
cargo run --bin qsv-skill-gen --features all_features
```

## Documentation

- [Complete API Documentation](./SKILLS_README.md)
- [Design Document](../../docs/AGENT_SKILLS_DESIGN.md)
- [Integration Guide](../../docs/AGENT_SKILLS_INTEGRATION.md)
- [POC Summary](../../docs/AGENT_SKILLS_POC_SUMMARY.md)
- [Complete Summary](../../docs/AGENT_SKILLS_COMPLETE_SUMMARY.md)
- [qsv Commands](https://github.com/dathere/qsv#commands)

## Requirements

- Node.js >= 18.0.0
- qsv installed and in PATH (for execution)
- TypeScript 5.0+ (for development)

## License

MIT

---

**Generated**: 2026-01-02
**Generator**: `qsv-skill-gen` v12.0.0
**Skills**: 66/66 commands (100%)
**Status**: ✅ Complete and Production Ready
