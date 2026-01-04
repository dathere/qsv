# QSV Agent Skills

Complete TypeScript implementation for loading, executing, and composing qsv command pipelines with the Claude Agent SDK and Claude Desktop (MCP).

## 🎯 NEW: Work with Local CSV Files Without Uploading!

The QSV MCP Server now supports **direct access to local CSV files**. No more uploading files to Claude Desktop!

**Quick Start**: See [QUICK_START_LOCAL_FILES.md](./QUICK_START_LOCAL_FILES.md)
**Full Guide**: See [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)

### Key Features:
- ✅ Browse CSV files in your directories
- ✅ Process files without uploading
- ✅ No file size limits
- ✅ Instant access
- ✅ Secure path validation

## Overview

This directory contains:

1. **66 Auto-generated Skill Definitions** - JSON files describing all qsv commands (parsed with qsv-docopt)
2. **1,279 Test-Based Examples** - Real examples extracted from CI tests with full I/O data
3. **TypeScript Executor** - Complete implementation for running qsv skills
4. **Pipeline Composition API** - Fluent interface for chaining operations
5. **MCP Server with Filesystem Access** - Model Context Protocol server for Claude Desktop integration
6. **Working Demos** - Practical demonstrations of the system

Each skill file provides:
- **Command specification**: Binary, subcommand, arguments, and options (parsed with qsv-docopt)
- **Rich descriptions**: Extracted from usage text
- **USAGE examples**: Real usage examples from documentation (417 total)
- **Test examples reference**: Pointer to load-as-needed test examples (1,279 total from 54 skills)
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

# Test-based examples (load-as-needed)
node examples/test-examples-demo.js
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
- **USAGE Examples**: 417 from documentation
- **Test Examples**: 1,279 from CI tests (54 skills, 82% coverage)
- **Total Examples**: 1,696
- **Options**: 837 command-line options
- **Arguments**: 60 positional arguments

## Project Structure

```
.claude/skills/
├── qsv/                    # 66 skill JSON definitions
│   ├── qsv-select.json     # With examples_ref pointer
│   ├── qsv-stats.json
│   ├── qsv-moarstats.json
│   └── ... (63 more)
├── examples/               # Test-based examples (load-as-needed)
│   ├── qsv-select-examples.json  # 40 examples with I/O data
│   ├── qsv-stats-examples.json   # 96 examples
│   ├── qsv-dedup-examples.json   # 11 examples
│   ├── ... (51 more)
│   ├── basic.js                  # Demo: basic usage
│   ├── pipeline.js               # Demo: pipeline composition
│   └── test-examples-demo.js     # Demo: test examples
├── src/                    # TypeScript source
│   ├── types.ts           # Type definitions (includes TestExample + MCP types)
│   ├── loader.ts          # Skill loading & loadTestExamples()
│   ├── executor.ts        # qsv execution wrapper
│   ├── pipeline.ts        # Pipeline composition API
│   ├── mcp-server.ts      # MCP server implementation
│   ├── mcp-tools.ts       # MCP tool definitions
│   ├── mcp-resources.ts   # MCP resource provider
│   ├── mcp-pipeline.ts    # MCP pipeline tool
│   └── index.ts           # Public exports
├── scripts/
│   └── install-mcp.js     # MCP installation helper
├── dist/                   # Compiled JavaScript (gitignored)
├── package.json
├── tsconfig.json
├── mcp-config.json         # Claude Desktop config template
├── README.md               # This file
├── README-MCP.md           # MCP server documentation
└── SKILLS_README.md        # Complete API documentation
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

### Load Test-Based Examples (On-Demand)

```typescript
import { SkillLoader } from './dist/index.js';

const loader = new SkillLoader();
await loader.loadAll();

// Load test examples for a specific skill
const dedupExamples = await loader.loadTestExamples('qsv-dedup');

if (dedupExamples) {
  console.log(`Loaded ${dedupExamples.examples.length} test examples`);

  // Access first example
  const example = dedupExamples.examples[0];
  console.log(`Name: ${example.name}`);
  console.log(`Description: ${example.description}`);
  console.log(`Command: ${example.command}`);

  // Get input CSV data
  if (example.input?.data) {
    const inputCSV = example.input.data
      .map(row => row.join(','))
      .join('\n');
    console.log('Input:', inputCSV);
  }

  // Get expected output
  if (example.expected?.data) {
    const expectedCSV = example.expected.data
      .map(row => row.join(','))
      .join('\n');
    console.log('Expected:', expectedCSV);
  }

  // Filter by tags
  const regressionTests = dedupExamples.examples.filter(ex =>
    ex.tags && ex.tags.includes('regression')
  );
}
```

**Available Tags**: basic, regression, error-handling, case-sensitivity, unicode, no-headers, custom-delimiter

**Demo**: Run `node examples/test-examples-demo.js` to see test examples in action

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
  "test_file": "https://github.com/dathere/qsv/blob/master/tests/test_<command>.rs",
  "examples_ref": "examples/qsv-<command>-examples.json"
}
```

## Generation

### Skill Definitions

Skills are auto-generated from qsv command USAGE text using `qsv-skill-gen`:

```bash
# Generate all 66 skills
cargo run --bin qsv-skill-gen --features all_features

# Output: .claude/skills/qsv/*.json
```

The generator uses **qsv-docopt Parser** (the same parser qsv uses at runtime) for robust parsing:
1. Extracts `USAGE` static string from command source files
2. **Parses with qsv-docopt** for accurate argument/option detection
3. Extracts descriptions from USAGE text
4. Infers types from names and descriptions
5. Detects performance hints from emoji markers (🤯 📇 🏎️ 😣)
6. Generates structured JSON skill definitions with `examples_ref` pointer

### Test-Based Examples (Load-as-Needed)

Rich examples are extracted from CI test files using `qsv-test-examples-gen`:

```bash
# Extract examples from test files
cargo run --bin qsv-test-examples-gen --features all_features

# Output: .claude/skills/examples/*.json
# Result: 1,279 examples from 54 test files
```

The test examples generator:
1. Finds all `#[test]` functions in `tests/test_*.rs` using UTF-8-safe brace counting
2. Extracts input data from `wrk.create()` calls
3. Parses commands from both `wrk.command()` and `cmd.arg()` patterns (string literals only)
4. Captures expected output from assertions
5. Infers tags (regression, basic, error-handling, etc.)
6. Applies proper shell quoting to command strings (spaces, quotes, special chars)
7. Deduplicates test names by appending counters to duplicates
8. Generates JSON files with real input/output data

**Benefits**:
- ✅ Real, tested examples from CI suite
- ✅ Full input CSV data and expected outputs
- ✅ Shell-safe command strings with proper quoting
- ✅ Unique test names with automatic deduplication
- ✅ Tagged for easy filtering
- ✅ Load-as-needed architecture (keeps skill files lightweight)
- ✅ 1,279 examples across 54 skills (82% coverage)

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

## Integration with Claude Desktop (MCP Server)

The QSV MCP Server exposes all 66 qsv commands to Claude Desktop through the Model Context Protocol.

### Quick Start

```bash
# Install and configure
cd .claude/skills
npm install
npm run mcp:install
```

This will:
1. Build the MCP server
2. Update Claude Desktop configuration
3. Enable qsv tools in Claude Desktop

### Manual Configuration

Edit `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "qsv": {
      "command": "node",
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"]
    }
  }
}
```

Restart Claude Desktop to load the server.

### Usage

Once configured, use natural language in Claude Desktop:

```
"Select columns 1-5 from data.csv"
"Calculate statistics for the price column in sales.csv"
"Remove duplicates from data.csv and sort by revenue"
"Show me an example of joining two CSV files"
```

Claude will automatically:
- Select the appropriate qsv tool
- Execute the command
- Return results or explanations

### What's Available

- **22 MCP Tools**: 20 common commands + generic fallback + pipeline tool
- **1,279 Example Resources**: Real test examples with input/output data
- **File-Based Processing**: Works with your local CSV files
- **Natural Language Interface**: No command syntax needed

For complete MCP documentation, see [README-MCP.md](./README-MCP.md).

## Development

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Run demos
npm test                              # Basic skill usage
npm run test-pipeline                 # Pipeline composition
node examples/test-examples-demo.js   # Test examples

# Regenerate skills (from qsv repo root)
cargo run --bin qsv-skill-gen --features all_features

# Regenerate test examples (from qsv repo root)
cargo run --bin qsv-test-examples-gen --features all_features
```

## Documentation

- [MCP Server Guide](./README-MCP.md) - Claude Desktop integration
- [Complete API Documentation](./SKILLS_README.md)
- [Test-Based Examples Guide](../../docs/AGENT_SKILLS_TEST_EXAMPLES.md)
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

**Generated**: 2026-01-03
**Generators**: `qsv-skill-gen` + `qsv-test-examples-gen` v12.0.0
**Skills**: 66/66 commands (100%)
**USAGE Examples**: 417 from documentation
**Test Examples**: 1,279 from CI tests (54 skills)
**Total Examples**: 1,696
**Parsing**: qsv-docopt (robust, accurate)
**Features**: Shell-safe quoting, UTF-8 support, complete command strings, automatic deduplication
**Status**: ✅ Complete and Production Ready
