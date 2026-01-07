# QSV Agent Skills - Design Document

## Executive Summary

Auto-generate Agent Skills from qsv's 67 command usage texts, creating a comprehensive, always-synchronized skill library for Claude Code and other AI agents. This leverages qsv's existing excellent documentation to make its data-wrangling capabilities discoverable and composable in agent workflows.

## Motivation

### Current State
- qsv has 67 commands with rich, structured usage documentation
- Each command has examples, parameter descriptions, and detailed explanations
- Usage text follows consistent docopt format
- Tests are linked from usage text for additional context

### Opportunity
- Transform static documentation into executable Agent Skills
- Make qsv's capabilities discoverable to AI agents
- Enable composition of complex data workflows
- Maintain single source of truth (usage text)

### Benefits
1. **Zero Documentation Debt**: Skills auto-update when usage text changes
2. **Discoverability**: Agents can search/recommend appropriate skills
3. **Composability**: Chain skills for complex workflows
4. **Consistency**: All 67 commands exposed uniformly
5. **Rich Context**: Examples become executable demonstrations

## Architecture

### Overview

```
┌─────────────────┐
│  qsv Commands   │
│  (src/cmd/*.rs) │
└────────┬────────┘
         │
         │ Extract USAGE
         ▼
┌─────────────────┐
│ Usage Parser    │
│ (Rust tool)     │
└────────┬────────┘
         │
         │ Generate
         ▼
┌─────────────────┐      ┌──────────────┐
│ Skill Registry  │─────▶│ skill.json   │
│ (.claude/skills)│      │ per command  │
└─────────────────┘      └──────────────┘
         │
         │ Load at runtime
         ▼
┌─────────────────┐
│  Claude Agent   │
│  (invokes qsv)  │
└─────────────────┘
```

### Components

#### 1. Usage Text Parser
**Input**: Raw USAGE string from each command
**Output**: Structured skill definition

**Parses**:
- Description (first paragraph)
- Examples (lines starting with `$`)
- Usage syntax (after "Usage:")
- Arguments (under "arguments:")
- Options/flags (under "options:")
- Links to tests

#### 2. Skill Definition Generator
**Input**: Parsed usage data
**Output**: Skill definition file (JSON/YAML)

**Generates**:
- Skill metadata (name, description, category)
- Parameter schema with types and validation
- Option flags with descriptions
- Example invocations
- Help text

#### 3. Skill Registry
**Location**: `.claude/skills/qsv/`
**Format**: One JSON file per command

**Structure**:
```json
{
  "name": "qsv-select",
  "version": "0.1.0",
  "description": "Select columns from CSV data efficiently",
  "category": "data-wrangling",
  "command": {
    "binary": "qsv",
    "subcommand": "select",
    "args": [...],
    "options": [...]
  },
  "examples": [...],
  "documentation": "https://github.com/dathere/qsv/blob/master/tests/test_select.rs"
}
```

## Skill Definition Format

### Core Schema

```typescript
interface QsvSkill {
  // Metadata
  name: string;                    // e.g., "qsv-select"
  version: string;                 // matches qsv version
  description: string;             // from usage text
  category: SkillCategory;         // inferred or tagged

  // Command invocation
  command: {
    binary: "qsv";
    subcommand: string;            // e.g., "select"
    args: Argument[];
    options: Option[];
  };

  // Documentation
  examples: Example[];
  usageText: string;               // original usage text
  testFile?: string;               // link to test file

  // Feature flags (from Cargo.toml)
  requiredFeatures?: string[];

  // Behavioral hints
  hints: {
    streamable: boolean;           // can work with stdin/stdout
    indexed?: boolean;             // benefits from index
    memory: "constant" | "proportional" | "full";  // 🤯 markers
  };
}

interface Argument {
  name: string;
  type: "string" | "file" | "number" | "regex";
  required: boolean;
  description: string;
  default?: string;
  validation?: {
    pattern?: string;
    min?: number;
    max?: number;
  };
}

interface Option {
  flag: string;                    // e.g., "--random"
  short?: string;                  // e.g., "-R"
  type?: "flag" | "string" | "number";
  description: string;
  default?: any;
  requires?: string[];             // dependent options
}

interface Example {
  description: string;
  command: string;
  expectedBehavior?: string;
}

type SkillCategory =
  | "selection"      // select, slice, take, sample
  | "filtering"      // search, searchset, grep
  | "transformation" // apply, rename, transpose
  | "aggregation"    // stats, frequency, count
  | "joining"        // join, joinp
  | "validation"     // schema, validate
  | "formatting"     // fmt, fixlengths, table
  | "conversion"     // to, input, excel
  | "analysis"       // stats, moarstats, correlation
  | "utility";       // index, cat, headers
```

### Example: `qsv-select` Skill

```json
{
  "name": "qsv-select",
  "version": "12.0.0",
  "description": "Select columns from CSV data efficiently. Re-order, duplicate, reverse or drop columns. Reference by index, name, range, or regex.",
  "category": "selection",

  "command": {
    "binary": "qsv",
    "subcommand": "select",
    "args": [
      {
        "name": "selection",
        "type": "string",
        "required": true,
        "description": "Columns to select (index, name, range, regex). Use '!' to invert.",
        "examples": ["1,4", "Header1-Header4", "/^a/", "!1-2"]
      },
      {
        "name": "input",
        "type": "file",
        "required": false,
        "description": "Input CSV file (defaults to stdin)"
      }
    ],
    "options": [
      {
        "flag": "--random",
        "short": "-R",
        "type": "flag",
        "description": "Randomly shuffle the columns in the selection"
      },
      {
        "flag": "--seed",
        "type": "number",
        "description": "Seed for the random number generator",
        "requires": ["--random"]
      },
      {
        "flag": "--sort",
        "short": "-S",
        "type": "flag",
        "description": "Sort the selected columns lexicographically"
      },
      {
        "flag": "--output",
        "short": "-o",
        "type": "string",
        "description": "Write output to file instead of stdout"
      }
    ]
  },

  "examples": [
    {
      "description": "Select the first and fourth columns",
      "command": "qsv select 1,4 data.csv"
    },
    {
      "description": "Select columns using regex (starting with 'a')",
      "command": "qsv select /^a/ data.csv"
    },
    {
      "description": "Reverse column order",
      "command": "qsv select _-1 data.csv"
    },
    {
      "description": "Remove sensitive columns",
      "command": "qsv select '!/SSN|password|account_no/' data.csv"
    }
  ],

  "hints": {
    "streamable": true,
    "indexed": false,
    "memory": "constant"
  },

  "testFile": "https://github.com/dathere/qsv/blob/master/tests/test_select.rs"
}
```

## Usage Text Parsing Strategy

### Parsing Algorithm

```rust
struct UsageParser {
    // State machine for parsing different sections
}

impl UsageParser {
    fn parse(usage_text: &str) -> SkillDefinition {
        let sections = self.split_sections(usage_text);

        SkillDefinition {
            description: self.parse_description(&sections.description),
            examples: self.parse_examples(&sections.examples),
            command: self.parse_usage_line(&sections.usage),
            args: self.parse_arguments(&sections.arguments),
            options: self.parse_options(&sections.options),
        }
    }

    fn parse_examples(&self, text: &str) -> Vec<Example> {
        // Extract lines starting with "$" or "#"
        // "  $ qsv select 1,4" -> Example { command: "qsv select 1,4", ... }
        // "  # select columns starting with 'a'" -> description for next example
    }

    fn parse_usage_line(&self, text: &str) -> Command {
        // "qsv select [options] [--] <selection> [<input>]"
        // Extract: subcommand, required args, optional args
    }

    fn parse_arguments(&self, text: &str) -> Vec<Argument> {
        // "    <selection>            The columns to select..."
        // Extract: name, description, infer type from description/examples
    }

    fn parse_options(&self, text: &str) -> Vec<Option> {
        // "    -R, --random           Randomly shuffle..."
        // Extract: short flag, long flag, type, description
    }
}
```

### Type Inference

Infer parameter types from:
1. **Name patterns**: `<input>` → file, `<number>` → number
2. **Description keywords**: "file path", "regex", "number"
3. **Example values**: `/^a/` → regex, `1,4` → string
4. **Validation hints**: "(must be between X and Y)" → number with range

### Special Markers

Extract behavioral hints from usage text:
- 🤯 emoji → `memory: "full"`
- 📇 emoji → `hints.indexed: true`
- 🏎️ emoji → `hints.parallel: true`
- 😣 emoji → `memory: "proportional"`

## Generation Pipeline

### Build-Time Generation

```rust
// In build.rs or separate binary: qsv-skill-gen

fn main() {
    let output_dir = PathBuf::from(".claude/skills/qsv");
    fs::create_dir_all(&output_dir)?;

    // Iterate through all commands
    for entry in glob("src/cmd/*.rs")? {
        let cmd_path = entry?;
        let cmd_name = extract_command_name(&cmd_path);

        // Extract USAGE constant
        let usage_text = extract_usage_static(&cmd_path)?;

        // Parse into skill definition
        let skill = UsageParser::new().parse(usage_text, cmd_name);

        // Enhance with metadata
        let enhanced = enhance_skill(skill, &cmd_path)?;

        // Write skill file
        let skill_file = output_dir.join(format!("{}.json", cmd_name));
        write_skill_json(&skill_file, &enhanced)?;
    }

    // Generate index/registry
    generate_skill_index(&output_dir)?;
}

fn enhance_skill(skill: Skill, cmd_path: &Path) -> Result<Skill> {
    // Add feature requirements from main.rs registration
    let features = extract_feature_flags(cmd_path)?;

    // Add performance hints from emoji markers
    let hints = extract_performance_hints(&skill.usage_text)?;

    // Link to test file
    let test_file = find_test_file(&skill.name)?;

    Ok(Skill {
        required_features: features,
        hints,
        test_file,
        ..skill
    })
}
```

### CI/CD Integration

```yaml
# .github/workflows/generate-skills.yml
name: Generate Agent Skills

on:
  push:
    paths:
      - 'src/cmd/*.rs'
      - 'src/main.rs'

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Generate Skills
        run: cargo run --bin qsv-skill-gen

      - name: Commit Generated Skills
        run: |
          git add .claude/skills/
          git commit -m "chore: regenerate agent skills" || exit 0
          git push
```

## Integration with Claude Agent SDK

### Skill Invocation

```typescript
// Agent can invoke skills like this:
const result = await agent.invokeSkill('qsv-select', {
  selection: '1,4',
  input: 'data.csv',
  options: { output: 'selected.csv' }
});
```

### Skill Composition

```typescript
// Chain multiple qsv skills
const workflow = agent.composeSkills([
  { skill: 'qsv-select', args: { selection: '!SSN,password' } },
  { skill: 'qsv-dedup', args: {} },
  { skill: 'qsv-stats', args: { everything: true } }
]);

await workflow.execute('input.csv');
```

### Skill Discovery

```typescript
// Agent searches for relevant skills
const skills = await agent.searchSkills({
  query: "remove duplicate rows",
  category: "transformation"
});
// Returns: [qsv-dedup, qsv-sort, ...]
```

## Use Cases

### 1. Data Cleaning Workflow

**User Request**: "Clean this CSV by removing duplicates, invalid emails, and sorting by date"

**Agent Plan**:
```
1. qsv-dedup: Remove duplicate rows
2. qsv-search: Filter out invalid emails using regex
3. qsv-sort: Sort by date column
```

**Generated Commands**:
```bash
qsv dedup input.csv | \
qsv search -v -s email '^[^@]+@[^@]+\.[^@]+$' | \
qsv sort -s date > cleaned.csv
```

### 2. Schema Validation

**User Request**: "Validate this CSV against my schema and report errors"

**Agent Plan**:
```
1. qsv-schema: Infer or validate against schema
2. qsv-validate: Check data quality
```

### 3. Data Analysis Pipeline

**User Request**: "Analyze sales data: compute stats, find correlations, generate summary"

**Agent Plan**:
```
1. qsv-select: Extract relevant columns
2. qsv-moarstats: Compute comprehensive statistics
3. qsv-frequency: Analyze categorical distributions
4. qsv-correlation: Find correlations between metrics
```

## Categorization Strategy

### Automatic Categorization

Categorize commands based on:
1. **Command name patterns**: stats → analysis, join → joining
2. **Usage text keywords**: "filter", "transform", "aggregate"
3. **Manual overrides**: `.claude/skills/categories.toml`

```toml
# .claude/skills/categories.toml
[categories]
selection = ["select", "slice", "take", "sample", "head", "tail"]
filtering = ["search", "searchset", "grep", "filter"]
transformation = ["apply", "rename", "transpose", "reverse", "datefmt"]
aggregation = ["stats", "moarstats", "frequency", "count", "groupby"]
joining = ["join", "joinp"]
validation = ["schema", "validate", "safenames"]
formatting = ["fmt", "fixlengths", "table", "align"]
conversion = ["to", "input", "excel", "lua", "foreach", "python"]
analysis = ["stats", "moarstats", "correlation", "describegpt"]
utility = ["index", "cat", "headers", "split", "partition"]
```

## Versioning and Compatibility

### Skill Version = qsv Version

```json
{
  "name": "qsv-select",
  "version": "12.0.0",  // matches qsv version
  "minQsvVersion": "12.0.0",
  "compatibility": {
    "features": ["all_features"],  // or specific feature flags
    "os": ["linux", "macos", "windows"]
  }
}
```

### Deprecation Handling

When command behavior changes:
```json
{
  "name": "qsv-oldcommand",
  "deprecated": true,
  "deprecatedSince": "11.0.0",
  "replacedBy": "qsv-newcommand",
  "migrationGuide": "Use qsv-newcommand with --new-flag instead"
}
```

## Implementation Phases

### Phase 1: Proof of Concept (Week 1)
- [ ] Build basic usage text parser
- [ ] Generate skills for 3-5 representative commands
- [ ] Create skill schema/format
- [ ] Test manual invocation

**Deliverable**: Working parser + 5 skill files

### Phase 2: Full Generation (Week 2)
- [ ] Parse all 67 commands
- [ ] Implement type inference
- [ ] Extract performance hints
- [ ] Generate skill index/registry
- [ ] Add feature flag detection

**Deliverable**: Complete skill library

### Phase 3: Integration (Week 3)
- [ ] Create skill invocation wrapper
- [ ] Implement skill composition
- [ ] Add error handling and validation
- [ ] Write documentation

**Deliverable**: Usable skill system

### Phase 4: Enhancement (Week 4)
- [ ] Add skill discovery/search
- [ ] Implement caching for common operations
- [ ] Create skill recommendation system
- [ ] Build CI/CD pipeline

**Deliverable**: Production-ready system

### Phase 5: Advanced Features (Future)
- [ ] Interactive skill builder UI
- [ ] Skill performance profiling
- [ ] Auto-optimization suggestions
- [ ] Multi-language bindings (Python SDK)

## Maintenance Strategy

### Keeping Skills Synchronized

1. **Pre-commit Hook**: Warn if usage text changed without skill regeneration
2. **CI Check**: Fail if skills are out of sync
3. **Automated PRs**: Bot generates skill updates when commands change

```bash
# .githooks/pre-commit
#!/bin/bash
if git diff --cached --name-only | grep -q "src/cmd/"; then
    echo "Command files changed. Regenerating skills..."
    cargo run --bin qsv-skill-gen
    git add .claude/skills/
fi
```

### Testing Strategy

```rust
#[test]
fn test_all_commands_have_skills() {
    let commands = discover_all_commands();
    let skills = load_all_skills();

    for cmd in commands {
        assert!(skills.contains_key(&cmd),
                "Missing skill for command: {}", cmd);
    }
}

#[test]
fn test_skill_examples_execute() {
    for skill in load_all_skills() {
        for example in &skill.examples {
            // Execute example command
            let output = Command::new("qsv")
                .args(parse_command(&example.command))
                .output()?;

            assert!(output.status.success(),
                    "Example failed: {}", example.command);
        }
    }
}
```

## Open Questions

1. **Skill Naming**: Prefix with `qsv-` or use plain names?
2. **Parameter Validation**: How strict? Client-side or server-side?
3. **Streaming Results**: How to handle large CSV outputs in skill responses?
4. **Skill Chaining**: Automatic pipeline optimization?
5. **Error Messages**: How to surface qsv errors to agent context?
6. **Credentials**: How to handle skills that need API keys (e.g., geocode)?

## Success Metrics

- **Coverage**: 100% of qsv commands have skills
- **Sync**: Skills auto-update within 1 hour of command changes
- **Discoverability**: Agent finds correct skill in <3 suggestions
- **Composition**: Successfully chain 3+ skills in a workflow
- **Maintenance**: Zero manual skill updates required

## Alternatives Considered

### 1. Manual Skill Creation
**Pros**: Fine-grained control, optimized descriptions
**Cons**: 67 skills to maintain, documentation drift

### 2. Runtime Parsing
**Pros**: Always in sync, no build step
**Cons**: Performance overhead, parsing errors at runtime

### 3. LLM-Generated Skills
**Pros**: Rich descriptions, inferred use cases
**Cons**: Non-deterministic, requires review, API costs

**Decision**: Auto-generation from usage text (this proposal) balances automation, accuracy, and maintainability.

## Appendix

### Example: Complex Skill with Dependencies

```json
{
  "name": "qsv-joinp",
  "description": "Join two CSV files using Polars for high performance",
  "category": "joining",
  "command": {
    "binary": "qsv",
    "subcommand": "joinp",
    "args": [
      {
        "name": "columns1",
        "type": "string",
        "required": true,
        "description": "Columns from first CSV to join on"
      },
      {
        "name": "input1",
        "type": "file",
        "required": true
      },
      {
        "name": "columns2",
        "type": "string",
        "required": true
      },
      {
        "name": "input2",
        "type": "file",
        "required": true
      }
    ],
    "options": [
      {
        "flag": "--left",
        "type": "flag",
        "description": "Perform left join"
      },
      {
        "flag": "--full",
        "type": "flag",
        "description": "Perform full outer join"
      }
    ]
  },
  "requiredFeatures": ["polars"],
  "hints": {
    "streamable": false,
    "indexed": false,
    "memory": "full",
    "parallel": true
  }
}
```

### Related Documentation

- [Claude Agent SDK](https://github.com/anthropics/agent-sdk)
- [qsv Command Reference](https://github.com/dathere/qsv#commands)
- [Docopt Specification](http://docopt.org/)

---

**Document Version**: 1.0
**Last Updated**: 2026-01-02
**Authors**: Joel (human), Claude Sonnet 4.5 (AI)
