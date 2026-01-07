# QSV Agent Skills

Complete TypeScript implementation for loading, executing, and composing qsv command pipelines with the Claude Agent SDK and Claude Desktop (MCP).

## 🎯 NEW: Work with Local Tabular Data Files Without Uploading!

The QSV MCP Server now supports **direct access to local tabular data files** (CSV, Excel, JSONL, etc.). No more uploading files to Claude Desktop!

**Quick Start**: See [QUICK_START_LOCAL_FILES.md](./QUICK_START_LOCAL_FILES.md)
**Full Guide**: See [FILESYSTEM_USAGE.md](./FILESYSTEM_USAGE.md)

### Key Features:
- ✅ Browse tabular data files in your directories (CSV, Excel, JSONL, etc.)
- ✅ Process files without uploading
- ✅ No input file size limits (streams large files efficiently)
- ✅ Smart output handling (auto-saves large results > 850KB to disk)
- ✅ Instant access
- ✅ Secure path validation

## Overview

This directory contains:

1. **66 Auto-generated Skill Definitions** - JSON files describing all qsv commands (parsed with qsv-docopt)
2. **TypeScript Executor** - Complete implementation for running qsv skills
3. **Pipeline Composition API** - Fluent interface for chaining operations
4. **MCP Server with Filesystem Access** - Model Context Protocol server for Claude Desktop integration
5. **Working Demos** - Practical demonstrations of the system

Each skill file provides:
- **Command specification**: Binary, subcommand, arguments, and options (parsed with qsv-docopt)
- **Rich descriptions**: Extracted from usage text
- **Usage examples**: Real usage examples from documentation (417 total)
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

# MCP server (for Claude Desktop)
npm run mcp:install
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
- **Usage Examples**: 417 from documentation
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
│   ├── loader.ts          # Skill loading
│   ├── executor.ts        # qsv execution wrapper
│   ├── pipeline.ts        # Pipeline composition API
│   ├── mcp-server.ts      # MCP server implementation
│   ├── mcp-tools.ts       # MCP tool definitions
│   ├── mcp-filesystem.ts  # Filesystem resource provider
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
├── FILESYSTEM_USAGE.md     # Local file access guide
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

### Skill Definitions

Skills are auto-generated from qsv command USAGE text using the `--update-mcp-skills` flag:

```bash
# Generate all 66 skills
qsv --update-mcp-skills

# Output: .claude/skills/qsv/*.json
```

The generator uses **qsv-docopt Parser** (the same parser qsv uses at runtime) for robust parsing:
1. Extracts `USAGE` static string from command source files
2. **Parses with qsv-docopt** for accurate argument/option detection
3. Extracts descriptions from USAGE text
4. Infers types from names and descriptions
5. Detects performance hints from emoji markers (🤯 📇 🏎️ 😣)
6. Generates structured JSON skill definitions

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
      "args": ["/path/to/qsv/.claude/skills/dist/mcp-server.js"],
      "env": {
        "QSV_MCP_BIN_PATH": "/usr/local/bin/qsv",
        "QSV_MCP_WORKING_DIR": "/Users/your-username/Downloads",
        "QSV_MCP_ALLOWED_DIRS": "/Users/your-username/Downloads:/Users/your-username/Documents"
      }
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

- **25 MCP Tools**: 20 common commands + generic fallback + pipeline tool + 3 filesystem tools
- **Local File Access**: Browse and process tabular data files (CSV, Excel, JSONL, etc.) directly from your filesystem
- **File-Based Processing**: Works with your local files without uploading
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
npm run mcp:install                   # Install MCP server for Claude Desktop

# Regenerate skills
qsv --update-mcp-skills
```

## Documentation

- [MCP Server Guide](./README-MCP.md) - Claude Desktop integration
- [Filesystem Usage Guide](./FILESYSTEM_USAGE.md) - Local file access
- [Auto-Update Guide](./AUTO_UPDATE.md) - Keep skills in sync with qsv releases
- [Complete API Documentation](./SKILLS_README.md)
- [Design Document](./docs/design/AGENT_SKILLS_DESIGN.md)
- [Integration Guide](./docs/design/AGENT_SKILLS_INTEGRATION.md)
- [POC Summary](./docs/design/AGENT_SKILLS_POC_SUMMARY.md)
- [Complete Summary](./docs/design/AGENT_SKILLS_COMPLETE_SUMMARY.md)
- [qsv Commands](https://github.com/dathere/qsv#commands)

## Requirements

- Node.js >= 18.0.0
- qsv installed and in PATH (for execution)
- TypeScript 5.0+ (for development)

## License

MIT

---

**Updated**: 2026-01-07
**Version**: 13.0.0
**Generator**: `qsv --update-mcp-skills`
**Skills**: 66/66 commands (100%)
**Usage Examples**: 417 from documentation
**Parsing**: qsv-docopt (robust, accurate)
**Features**: MCP server, filesystem access, pipeline composition, type-safe execution
**Status**: ✅ Production Ready
