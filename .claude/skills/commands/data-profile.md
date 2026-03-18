---
allowed-tools:
  - mcp__qsv__qsv_sniff
  - mcp__qsv__qsv_count
  - mcp__qsv__qsv_headers
  - mcp__qsv__qsv_index
  - mcp__qsv__qsv_stats
  - mcp__qsv__qsv_moarstats
  - mcp__qsv__qsv_frequency
  - mcp__qsv__qsv_slice
  - mcp__qsv__qsv_command
  - mcp__qsv__qsv_describegpt
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
argument-hint: "<file>"
description: Profile a CSV/TSV/Excel file - detect format, compute statistics, show value distributions
---

# Data Profile

Profile the given tabular data file to understand its structure, types, and distributions.

## Cowork Setup

If running in Claude Code or Cowork, first call `qsv_get_working_dir` to check qsv's current working directory. If it differs from your workspace root (the directory where relative paths should resolve), call `qsv_set_working_dir` to sync it.

## Steps

1. **Index**: Run `qsv_index` on the file for fast random access in subsequent steps.

2. **Detect format**: Run `qsv_sniff` on the file to detect delimiter, encoding, preamble, and row count estimate.

3. **Count rows**: Run `qsv_count` to get the exact row count.

4. **Get headers**: Run `qsv_headers` to list all column names and positions.

5. **Compute statistics**: Run `qsv_stats` with `cardinality: true` and `stats_jsonl: true` to generate full column statistics and cache them. Include `--everything` for comprehensive stats (mean, median, mode, stddev, quartiles, etc.). Basic moarstats auto-runs to enrich the cache with ~18 additional columns.

6. **Advanced statistics**: Run `qsv_moarstats` with `advanced: true` (omit `output_file` — it updates the stats cache in-place by default). This adds kurtosis, bimodality coefficient, Gini coefficient, Shannon entropy, and winsorized/trimmed means to the stats cache — essential for understanding distribution shape and inequality.

7. **Show distributions**: Run `qsv_frequency` with `limit: 10` to show top value distributions for each column. For high-cardinality columns (cardinality close to row count), note them as likely unique identifiers.

8. **Screen for PII/PHI**: Run `qsv_command` with `command: "searchset"` and `args: ["--flag", "pii_match", "${CLAUDE_PLUGIN_ROOT}/resources/pii-regexes.txt"]` to scan for sensitive data patterns (SSN, credit cards, email, phone, IBAN). Report any columns with matches.

9. **Preview data**: Run `qsv_slice` with `len: 5` to show the first 5 rows as a sample.

10. **Document**: Run `qsv_describegpt` with `all: true` to generate a Data Dictionary, Description, and Tags. Output defaults to `<filestem>.describegpt.md`. This step leverages the stats cache created in step 5. Uses the connected LLM via MCP sampling — no API key needed.

## Quality Dimensions

When profiling, assess these five dimensions:

### 1. Completeness
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Null counts | `stats --cardinality --stats-jsonl` | `nullcount` column > 0 |
| Empty strings | `frequency --limit 10` | Empty string in top values |
| Sparsity | `stats` | `sparsity` field (ratio of nulls) |

**Red flag**: Sparsity > 0.5 means more than half the values are null.

### 2. Uniqueness
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Duplicate rows | `dedup --dupes-output dupes.csv` | Non-empty dupes file |
| Cardinality | `stats --cardinality` | `cardinality` vs row count |
| Unique ratio | `stats` | If cardinality = row count, column is unique |

**Red flag**: Key columns (ID, email) with cardinality < row count.

### 3. Validity
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Schema validation | `validate schema.json` | Validation error count |
| Data types | `stats` | `type` column (String, Integer, Float, Date, etc.) |
| Format patterns | `search --flag` | Rows not matching expected regex |
| Value ranges | `stats` | `min`, `max` outside expected range |

**Red flag**: Type column shows "String" for what should be numeric data.

### 4. Consistency
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Date formats | `stats` | Mixed date types in same column |
| Case consistency | `frequency` | "NYC" vs "nyc" vs "Nyc" as separate values |
| Encoding | `sniff` | Non-UTF-8 encoding detected |
| Delimiters | `sniff` | Unexpected delimiter or quoting |
| Row lengths | `fixlengths` | Pads short rows to match longest row; compare count before/after to detect ragged rows |

**Red flag**: Frequency shows same value in different cases/formats.

### 5. Accuracy
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Statistical outliers | `stats` | `mean`, `stddev` - values > 3 stddev from mean |
| Outlier counts | `moarstats` | `outliers_total_cnt`, `outliers_percentage` > 5% |
| Distribution shape | `moarstats --advanced` | `kurtosis` > 3 (heavy tails), `bimodality_coefficient` >= 0.555 (bimodal) |
| Inequality | `moarstats --advanced` | `gini_coefficient` near 1 (extreme concentration) |
| Value distributions | `frequency --limit 20` | Unexpected dominant values |
| Range checks | `stats` | `min`/`max` outside plausible range |
| Cross-field checks | `sqlp` | SQL WHERE clauses for business rules |

**Red flag**: Latitude > 90 or < -90, negative ages, future birth dates, kurtosis > 10 (extreme outliers).

### 6. PII/PHI Screening
**Question**: Does the data contain personally identifiable or protected health information?

Use `searchset` with a regex file to scan all columns for sensitive patterns:

```
qsv_command command: "searchset", input_file: "<file>", args: ["--flag", "pii_match", "${CLAUDE_PLUGIN_ROOT}/resources/pii-regexes.txt"]
```

The bundled `${CLAUDE_PLUGIN_ROOT}/resources/pii-regexes.txt` detects:
| Pattern | Example |
|---------|---------|
| SSN | `123-45-6789` |
| Mastercard | `5100 1234 5678 9012` |
| Visa | `4111 1111 1111 1111` |
| American Express | `371449635398431` |
| IBAN | `GB29NWBK60161331926819` |
| Email | `user@example.com` |
| US Phone | `+1 (555) 123-4567` |

For PHI screening, use the bundled `${CLAUDE_PLUGIN_ROOT}/resources/phi-regexes.txt`:

```
qsv_command command: "searchset", input_file: "<file>", args: ["--flag", "phi_match", "${CLAUDE_PLUGIN_ROOT}/resources/phi-regexes.txt"]
```

The bundled `${CLAUDE_PLUGIN_ROOT}/resources/phi-regexes.txt` detects:
| Pattern | Example |
|---------|---------|
| MRN (Medical Record Number) | `MRN123456` |
| DEA Number | `AB1234563` |
| NPI (National Provider Identifier) | `1234567890` (broad — verify with Luhn check) |
| ICD-10-CM Diagnosis Code | `J45.20`, `E11.9` |
| NDC (National Drug Code) | `0002-3456-78` |

For additional PHI patterns (e.g., MBI, state license numbers), create a custom regex file and pass it to `searchset` the same way.

**Red flag**: Any matches indicate PII/PHI exposure — flag columns for masking or removal before sharing.

## Report Format

Present a summary with:
- **File info**: format, delimiter, encoding, row count, column count
- **Column overview**: table with name, type, nulls, cardinality, min, max, mean (where applicable)
- **Key observations**: unique identifiers, high-null columns, type mismatches, notable distributions
- **Data quality flags**: any issues found (high sparsity, mixed types, ragged rows)
- **Data Dictionary, Description & Tags** (optional): AI-generated documentation from describegpt (step 10)

### Quality Report Checklist

- [ ] **Row count** and **column count**
- [ ] **Null/empty counts** per column (completeness)
- [ ] **Cardinality** per column (uniqueness assessment)
- [ ] **Data types** inferred per column (validity)
- [ ] **Min/max/mean** for numeric columns (range plausibility)
- [ ] **Outlier counts** and **distribution shape** (kurtosis, bimodality) from moarstats --advanced
- [ ] **Top frequency values** for categorical columns (distribution)
- [ ] **Duplicate rows** detected (uniqueness)
- [ ] **Schema violations** if schema provided (validity)
- [ ] **Encoding and delimiter** detected (consistency)
- [ ] **PII/PHI patterns** detected via searchset (privacy)
- [ ] **Data Dictionary** generated with column labels, descriptions, and types (describegpt)

## Common Data Quality Fixes

| Problem | Fix Command |
|---------|-------------|
| Inconsistent case | `sqlp` with `UPPER(col)` or `LOWER(col)` |
| Leading/trailing whitespace | `sqlp` with `TRIM(col)` |
| Duplicate rows | `dedup` |
| Ragged rows | `fixlengths` |
| Unsafe column names | `safenames` |
| Wrong encoding | `input` (normalizes to UTF-8) |
| Empty values | `sqlp` with `COALESCE(NULLIF(col, ''), 'N/A')` |
| Invalid rows | `validate schema.json` + filter |

## Notes

- For Excel/JSONL files, the MCP server auto-converts to CSV first
- The stats cache created in step 5 accelerates subsequent commands (frequency, schema, sqlp, joinp)
- If the file has no headers, mention this and use column indices
