---
name: infer-ontology
version: 18.0.0
license: MIT
allowed-tools:
  # Discovery
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  # Analysis
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_moarstats
  - mcp__qsv__qsv_frequency
  # Exploration
  - mcp__qsv__qsv_slice
  # Transform & Query
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_joinp
  - mcp__qsv__qsv_command
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
description: Infer a semantic ontology from all files in the working directory - entities, attributes, relationships, domain taxonomy, and cross-file join paths. Outputs ONTOLOGY.md.
---

# Infer Ontology

Scan all files in the current working directory, profile each one, then synthesize a semantic ontology describing the entities, their attributes, the relationships between files, and the domain taxonomy.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Steps

### Phase 1: Discovery

1. **Sync working directory**: Call `qsv_get_working_dir` to confirm the current path. If needed, call `qsv_set_working_dir`.

2. **List files**: Call `qsv_list_files` to get all files in the working directory. Classify each file:

   **Tabular** (handled natively by qsv MCP Server — auto-converted to CSV if needed):
   - CSV/TSV/SSV/TAB (`.csv`, `.tsv`, `.ssv`, `.tab` and `.sz` compressed variants)
   - Excel (`.xlsx`, `.xls`, `.xlsm`, `.xlsb`, `.ods`)
   - JSON/JSONL (`.json`, `.jsonl`, `.ndjson`)
   - Parquet (`.parquet`, `.pq`, `.pqt`)

   **Non-tabular** (best-effort extraction):
   - Markdown (`.md`), Text (`.txt`), README files — read for domain context
   - JSON config/schema files — extract field names and types
   - Data dictionaries, codebooks — extract controlled vocabularies
   - Other files — note their presence but skip deep analysis

### Phase 2: Profile Each Tabular File

3. **Run data-profile on each tabular file**: For every tabular file discovered in step 2, execute the full `/data-profile` workflow (steps 1-11). This produces for each file:
   - Format metadata (delimiter, encoding, row count, column count)
   - Full statistics with cardinality (`.stats.csv` cache)
   - Advanced statistics (kurtosis, Gini, entropy via moarstats)
   - Frequency distributions (top 10 values per column)
   - PII/PHI screening results
   - Injection screening results
   - Data preview (first 5 rows)
   - Data Dictionary with Label/Description per column, Dataset Description, and Tags

   Run profiles sequentially to avoid overwhelming the MCP server. After each profile completes, retain the key outputs (stats, frequencies, Data Dictionary, Tags) for cross-file analysis.

4. **Extract context from non-tabular files**: For each non-tabular file:
   - Read the file content (if text-based and reasonably sized)
   - Extract any domain terminology, field definitions, business rules, or relationship descriptions
   - Note any schema definitions, data dictionaries, or codebooks that describe the tabular files

### Phase 3: Cross-File Relationship Detection

5. **Identify shared columns**: Compare column names across all profiled files. Flag columns that appear in multiple files (exact name match or close variants like `customer_id` / `cust_id` / `customerid`).

6. **Validate join candidates**: For each pair of files sharing column names:
   - Compare data types from stats (both should be the same type)
   - Compare cardinality: a foreign key column typically has cardinality ≤ the primary key's cardinality
   - Compare value ranges (min/max) — overlapping ranges suggest a real relationship
   - Compare frequency distributions — if top values in one appear in the other, the relationship is likely valid
   - Use `qsv_sqlp` to test overlap when needed:
     ```
     SELECT COUNT(DISTINCT a.col) as overlap
     FROM read_csv('file1.csv') a
     INNER JOIN read_csv('file2.csv') b ON a.col = b.col
     ```

7. **Detect relationship types**: Classify each validated relationship:
   - **One-to-One**: both sides have cardinality ≈ row count and overlap is high
   - **One-to-Many**: one side has cardinality ≈ row count (primary key), the other has cardinality < row count (foreign key)
   - **Many-to-Many**: both sides have cardinality < row count — suggests a junction/bridge table may exist
   - **Hierarchical**: a column's values are a subset of another column in the same or different file (parent-child)
   - **Temporal**: date/time columns across files that suggest event sequences or time-series relationships

### Phase 4: Ontology Synthesis

8. **Define entities**: Each tabular file represents one entity (or multiple if it's a denormalized/wide table). For each entity:
   - **Name**: Derive from the filename (e.g., `customers.csv` → `Customer`)
   - **Description**: Use the Dataset Description from the data-profile
   - **Primary key**: The column with cardinality = row count and no nulls (if one exists)
   - **Attributes**: All columns, with their Label, Description, type, and nullability from the Data Dictionary

9. **Define relationships**: From the cross-file analysis (steps 5-7), document each relationship:
   - **Source entity** → **Target entity**
   - **Join columns** (source.column → target.column)
   - **Relationship type** (one-to-one, one-to-many, many-to-many)
   - **Overlap strength** (percentage of source values found in target)
   - **Join path** (the SQL or joinp expression to connect them)

10. **Infer domain taxonomy**: Using all collected information (column names, value distributions, Tags from each file, non-tabular file context):
    - **Domain**: The overarching subject area (e.g., "Healthcare", "E-commerce", "Finance")
    - **Subdomains**: More specific topic areas (e.g., "Patient Records", "Claims Processing")
    - **Entity hierarchy**: Group entities into logical categories
    - **Controlled vocabularies**: Columns with low cardinality that serve as classification dimensions (e.g., status codes, categories, types)
    - **Temporal scope**: Date ranges across the dataset collection

11. **Assess data quality across the collection**: Summarize cross-cutting quality concerns:
    - Inconsistent column naming conventions across files
    - Orphaned foreign keys (references to entities not present in any file)
    - PII/PHI exposure across the collection
    - Completeness gaps (entities with high null rates in key columns)
    - Type inconsistencies (same column name, different types across files)

### Phase 5: Output

12. **Write ONTOLOGY.md**: Generate the ontology document in the working directory using the template below.

## ONTOLOGY.md Template

````markdown
# Ontology: {Domain Name}

> Auto-generated ontology inferred from {N} files in `{working_directory}`.
> Generated: {date}

## Overview

{3-5 sentence summary of the dataset collection: what domain it covers, how many
entities, total rows across all files, key relationships, and overall data quality.}

## Domain Taxonomy

**Domain**: {Primary domain}
**Subdomains**: {Comma-separated list}
**Temporal scope**: {Earliest date} to {Latest date} (if applicable)
**Tags**: {Merged and deduplicated tags from all file profiles}

## Entities

### {Entity Name} (`{filename}`)

{Dataset Description from data-profile}

| Field | Type | Label | Description | Nullable | Cardinality | Key |
|-------|------|-------|-------------|----------|-------------|-----|
| ... | ... | ... | ... | ... | ... | PK/FK/— |

**Quality notes**: {Any quality flags from profiling — PII, high nulls, injection, etc.}

{Repeat for each entity}

## Relationships

| Source | Target | Source Column | Target Column | Type | Overlap | Join Expression |
|--------|--------|---------------|---------------|------|---------|-----------------|
| ... | ... | ... | ... | 1:N | 98.5% | `joinp --left file1.csv file2.csv --columns col` |

### Relationship Diagram

{ASCII or text-based entity-relationship diagram showing entities as boxes
and relationships as labeled arrows. Example:}

```
[Customer] 1──→N [Order] N←──1 [Product]
    │                              │
    └──────── N:M ─────────────────┘
              (via OrderItem)
```

## Controlled Vocabularies

{For columns with low cardinality that serve as classification dimensions}

### {Column Name} (`{filename}`)

| Value | Frequency | Description |
|-------|-----------|-------------|
| ... | ... | ... |

## Cross-Collection Quality Summary

| Dimension | Status | Details |
|-----------|--------|---------|
| Naming consistency | {OK/Warning} | {Details} |
| Referential integrity | {OK/Warning} | {Orphaned keys found in ...} |
| PII/PHI exposure | {OK/Warning} | {Columns flagged in ...} |
| Type consistency | {OK/Warning} | {Mismatched types for ...} |
| Completeness | {OK/Warning} | {High-null columns in ...} |

## Non-Tabular Context

{Summary of information extracted from non-tabular files that informed the ontology}

| File | Type | Contribution |
|------|------|-------------|
| ... | ... | {What domain context or definitions it provided} |
````

## Notes

- For Excel workbooks with multiple sheets, each sheet is treated as a separate entity
- Parquet files are queried in-place via `sqlp` with `read_parquet()` rather than converted to CSV
- The cross-file relationship detection uses column name matching and cardinality comparison as heuristics — always verify inferred relationships against domain knowledge
- For large directories (>20 tabular files), consider profiling in batches to manage context
- If a controlled Tag Vocabulary is provided, all Tags across the ontology are constrained to that vocabulary
