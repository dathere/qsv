---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_describegpt
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file> [--dictionary|--description|--tags|--all]"
description: Generate AI-powered Data Dictionary, Description, and Tags for a CSV/TSV/Excel file
---

# Data Describe

Generate AI-powered documentation for a tabular data file using `describegpt`. Produces a Data Dictionary (column labels, descriptions, types), a natural-language Description of the dataset, and semantic Tags — all via the connected LLM (no API key needed in MCP mode).

## Cowork Setup

If running in Claude Code or Cowork, first call `qsv_get_working_dir` to check qsv's current working directory. If it differs from your workspace root (the directory where relative paths should resolve), call `qsv_set_working_dir` to sync it.

## Steps

1. **Index**: Run `qsv_index` on the file for fast random access.

2. **Profile**: Run `qsv_stats` with `cardinality: true, stats_jsonl: true` to generate the stats cache. describegpt reads this cache for column metadata, so it must exist first.

3. **Describe**: Run `qsv_describegpt` with the requested options (default: `all: true`). Output defaults to `<filestem>.describegpt.md`.

4. **Present**: Display the generated Data Dictionary table, Description, and Tags to the user.

## Options

| Option | Effect |
|--------|--------|
| `--all` (default) | Generate Dictionary + Description + Tags in one pass |
| `--dictionary` | Data Dictionary only — column labels, descriptions, types |
| `--description` | Natural-language dataset Description only |
| `--tags` | Semantic Tags only |
| `--format` | Output format: `Markdown` (default), `JSON`, `TSV`, `TOON` |
| `--language` | Generate output in a non-English language (e.g. `Spanish`, `French`) |
| `--addl-cols-list` | Enrich the dictionary with extra columns (e.g. `"everything"`, `"moar!"`) |
| `--tag-vocab` | Constrain tags to a controlled vocabulary (comma-separated) |
| `--num-tags` | Number of tags to generate (default: 5) |
| `--num-examples` | Number of example values per column in the dictionary |
| `--enum-threshold` | Max cardinality to treat a column as an enum in the dictionary |

## Notes

- No API key needed in MCP mode — uses the connected LLM automatically via MCP sampling
- The stats cache must exist first for best results (step 2 creates it)
- Output defaults to `<filestem>.describegpt.md`
- For Excel/JSONL files, the MCP server auto-converts to CSV first
- Use `--format JSON` when you need machine-readable output for downstream processing
- Use `--language` to generate documentation in the user's preferred language
