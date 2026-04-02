---
name: data-profile
description: Profile a CSV/TSV/Excel file - detect format, compute statistics, show value distributions
user-invocable: true
argument-hint: "<file>"
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
  # Documentation
  - mcp__qsv__qsv_describegpt
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
---

# Data Profile

Profile the given tabular data file to understand its structure, types, and distributions.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Steps

1. **Index**: Run `qsv_index` on the file for fast random access in subsequent steps.

2. **Detect format**: Run `qsv_sniff` on the file to detect delimiter, encoding, preamble, and row count estimate.

3. **Count rows**: Run `qsv_count` to get the exact row count.

4. **Get headers**: Run `qsv_headers` to list all column names and positions.

5. **Compute statistics**: Run `qsv_stats` with `cardinality: true` and `stats_jsonl: true` to generate full column statistics and cache them. Include `--everything` for comprehensive stats (mean, median, mode, stddev, quartiles, etc.). Basic moarstats auto-runs to enrich the cache with ~18 additional columns.

6. **Advanced statistics**: Run `qsv_moarstats` with `advanced: true` (omit `output_file` — it updates the stats cache in-place by default). This enriches the stats cache with:
   - **Distribution shape**: kurtosis, bimodality coefficient, Jarque-Bera test (normality), skewness measures (pearson_skewness)
   - **Inequality/diversity**: Gini coefficient, Atkinson index, Theil index, Shannon entropy, normalized entropy, Simpson's diversity index
   - **Robust central tendency**: winsorized/trimmed means (with stddev, variance, CV, range, stddev ratio)
   - **Derived ratios**: median_mean_ratio, range_stddev_ratio, quartile_coefficient_dispersion, mad_stddev_ratio, iqr_range_ratio, robust_cv
   - **Outlier statistics**: counts by severity (extreme/mild, lower/upper), outlier mean/stddev/range, impact ratio, fence z-scores
   - **Other**: trimean, midhinge, mode_zscore, min/max z-scores, relative standard error, mean absolute deviation, xsd_type

7. **Show distributions**: Run `qsv_frequency` with `limit: 10` to show top value distributions for each column. For high-cardinality columns (cardinality close to row count), note them as likely unique identifiers.

8. **Optional: Bivariate correlations** (if multiple numeric columns): Run `qsv_moarstats` with `bivariate: true` to compute pairwise Pearson/Spearman/Kendall correlations, covariance, and mutual information. Output goes to `<FILESTEM>.stats.bivariate.csv`. Reveals hidden relationships between columns.

9. **Optional: Robust statistics** (if data is messy/heavy-tailed and < 100K rows): Run `qsv_command` with `command: "pragmastat"` for Hodges-Lehmann center and Shamos spread — robust estimators that tolerate up to 29% corrupted data. Especially useful when mean/stddev are misleading due to outliers. **Warning:** pragmastat computes median-of-pairwise statistics (O(n²) complexity) and becomes very slow on large datasets. For files > 100K rows, use `--subsample 10000` for ~100x speedup, or combine `--subsample 10000 --no-bounds` for ~200x speedup.

10. **Screen for PII/PHI**: Run `qsv_command` with `command: "searchset"` and `args: ["--flag", "pii_match", "${CLAUDE_PLUGIN_ROOT}/resources/pii-regexes.txt"]` to scan for sensitive data patterns (SSN, credit cards, email, phone, IBAN). Report any columns with matches.

11. **Screen for injection**: Run `qsv_command` with `command: "searchset"` and `args: ["--flag", "injection_match", "${CLAUDE_PLUGIN_ROOT}/resources/injection-regexes.txt"]` to scan for CSV/formula injection and SQL injection payloads. Report any columns with matches.

12. **Preview data**: Run `qsv_slice` with `len: 5` to show the first 5 rows as a sample.

13. **Document**: Generate a Data Dictionary, Dataset Description, and Tags as JSON.

    **13a) Primary — use `describegpt`**: Run `qsv_describegpt` with `all: true, format: "JSON"` and `output: "<filestem>.describegpt.json"`. If the user provided a Tag Vocabulary file, also pass `tag_vocab: "<vocab_file>"`. This produces a structured JSON file with three top-level objects: `Dictionary`, `Description`, and `Tags`. Each of these contains a `response` (the main content), optional `reasoning`, and `token_usage` metadata. The data dictionary itself is under `Dictionary.response.fields`, as an array of field descriptors with keys like `name`, `null_count`, `cardinality`, `min`, `max`, `mean`, and `stddev`. Present the results to the user. When MCP sampling is unavailable but the tool still returns prompts, follow those prompts by issuing a follow-up call with `_llm_responses` instead of using the manual fallback.

    **13b) Fallback — manual generation**: If `describegpt` encounters a tool error or times out, or if following its prompts via `_llm_responses` is not possible, fall back to generating the same artifacts manually from the statistics (steps 5-6) and frequency distributions (step 7). Save the result as `<filestem>.profile.json` using the same canonical structure as `describegpt`, for example:

    ```json
    {
      "Dictionary": {
        "response": {
          "fields": [
            {
              "name": "column_name",
              "type": "Integer",
              "label": "Column Name",
              "description": "1-5 sentence description informed by type, stats, and frequency distribution",
              "null_count": 0,
              "cardinality": 100,
              "min": "0",
              "max": "999",
              "mean": "450.5",
              "stddev": "120.3"
            }
          ]
        },
        "reasoning": "",
        "token_usage": {}
      },
      "Description": {
        "response": "3-10 sentences describing the dataset: what it represents, scope, key characteristics, quality issues, and potential use cases.",
        "reasoning": "",
        "token_usage": {}
      },
      "Tags": {
        "response": ["tag1", "tag2", "tag3"],
        "reasoning": "",
        "token_usage": {}
      }
    }
    ```

    For the fallback dictionary entries (under `Dictionary.response.fields`):
    - `label`: Human-readable version of the field name (e.g., `customer_id` → `Customer ID`)
    - `description`: 1-5 sentence description informed by type, statistics, and frequency distribution
    - Include key stats fields (`null_count`, `cardinality`, `min`, `max`, `mean`, `sortiness`, `stddev`, `variance`, `cv`, `sparsity`) where applicable

    For the fallback tags (under `Tags.response`): Infer 5-15 semantic tags based on column names, data types, value distributions, and domain characteristics. If a controlled Tag Vocabulary is provided, constrain choices to that vocabulary only.

## Quality Dimensions

When profiling, assess these quality dimensions:

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

### 6. Column Name Quality
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Unsafe names | `safenames --verify` | Spaces, special chars, reserved words |
| Duplicate headers | `headers` | Same name appearing twice |
| Naming consistency | `headers` | Mixed conventions (camelCase vs snake_case) |

**Red flag**: Column names with spaces or special characters break downstream tools and SQL queries.

### 7. Conformity
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Standard codes | `searchset` with domain regex file | Values not matching ISO country, state, zip patterns |
| Format adherence | `search --flag` with expected pattern | Phone numbers, emails, URLs not matching standard format |
| Controlled vocabularies | `frequency` | Unexpected values outside known valid set |

**Red flag**: A "country" column with free-text entries instead of ISO 3166 codes, or a "state" column mixing abbreviations and full names.

### 8. Referential Integrity
| Check | Command | What to Look For |
|-------|---------|-----------------|
| Orphaned foreign keys | `joinp --left-anti` | Rows in child file with no match in parent |
| Missing references | `joinp --left-anti` (reversed) | Parent records with no children (if expected) |
| Key overlap | `sqlp` | Cross-file key comparison via SQL |

**Red flag**: An orders file referencing customer IDs that don't exist in the customers file. Only applicable when profiling related files together.

### 9. PII/PHI Screening
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

### 10. Injection Safety
**Question**: Does the data contain CSV/formula injection or SQL injection payloads?

Malicious cell values can execute code when opened in spreadsheet applications (Excel, Google Sheets) or cause damage when loaded into databases without parameterized queries.

Use `searchset` with the bundled injection regex file to scan all columns:

```
qsv_command command: "searchset", input_file: "<file>", args: ["--flag", "injection_match", "${CLAUDE_PLUGIN_ROOT}/resources/injection-regexes.txt"]
```

The bundled `${CLAUDE_PLUGIN_ROOT}/resources/injection-regexes.txt` detects:

**CSV/Formula Injection**:
| Pattern | Example | Risk |
|---------|---------|------|
| Starts with `=` | `=CMD("calc")` | Arbitrary command execution in Excel |
| Starts with `+` + function | `+CMD("calc")` | Same as `=` in many spreadsheet apps (positive numbers/phone numbers excluded) |
| Starts with `-` + function | `-SUM(A1:A10)` | Formula execution (negative numbers excluded) |
| Starts with `@` | `@SUM(A1:A10)` | Excel function prefix |
| Starts with tab/CR | `\t=CMD(...)` | Bypasses naive prefix checks |

**SQL Injection**:
| Pattern | Example | Risk |
|---------|---------|------|
| SELECT...FROM | `'; SELECT * FROM users--` | Data exfiltration |
| UNION SELECT | `' UNION SELECT password FROM users--` | Query hijacking |
| DROP TABLE/DATABASE | `'; DROP TABLE users--` | Data destruction |
| INSERT INTO | `'; INSERT INTO users VALUES(...)--` | Data tampering |
| DELETE FROM | `'; DELETE FROM orders--` | Data deletion |
| UPDATE SET | `'; UPDATE users SET role='admin'--` | Data modification |
| Tautology | `' OR 1=1--` | Authentication bypass |
| Stacked queries | `'; DELETE FROM orders--` | Arbitrary SQL execution |

**Red flag**: Any matches indicate potential injection payloads — sanitize cells before sharing the file or loading into a database. For formula injection, prefix dangerous cells with a single quote (`'`) or strip leading `=+-@` characters.

## Report Format

Present a summary with:
- **File info**: format, delimiter, encoding, row count, column count
- **Column overview**: table with name, type, nulls, cardinality, min, max, mean (where applicable)
- **Key observations**: unique identifiers, high-null columns, type mismatches, notable distributions
- **Data quality flags**: any issues found (high sparsity, mixed types, ragged rows)
- **Data Dictionary, Description & Tags**: JSON documentation generated via `describegpt` (step 13a), or manually from stats cache and frequency distributions as fallback (step 13b)

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
- [ ] **Column names** safe and consistent (safenames --verify)
- [ ] **Conformity** to domain standards checked where applicable (searchset)
- [ ] **Referential integrity** verified across related files if provided (joinp --left-anti)
- [ ] **PII/PHI patterns** detected via searchset (privacy)
- [ ] **Injection payloads** scanned for CSV/formula and SQL injection patterns (searchset)
- [ ] **Data Dictionary** with Label and Description per column, dataset Description, and Tags — via `describegpt --format JSON` (step 13a) or manual fallback (step 13b)

## Common Data Quality Fixes

| Problem | Fix Command |
|---------|-------------|
| Inconsistent case | `sqlp` with `UPPER(col)` or `LOWER(col)` |
| Leading/trailing whitespace | `sqlp` with `TRIM(col)` |
| Duplicate rows | `dedup` |
| Ragged rows | `fixlengths` |
| Unsafe column names | `safenames` |
| Non-conforming values | `searchset` + `search --flag` to identify, `sqlp` to fix |
| Orphaned foreign keys | `joinp --left-anti` to find, then remove or fix references |
| Injection payloads | `searchset` to detect + `sqlp` to sanitize (prefix with `'` or strip leading `=+-@`) |
| Wrong encoding | `input` (normalizes to UTF-8) |
| Empty values | `sqlp` with `COALESCE(NULLIF(col, ''), 'N/A')` |
| Invalid rows | `validate schema.json` + filter |

## Notes

- For Excel/JSONL files, the MCP server auto-converts to CSV first
- The stats cache created in step 5 accelerates subsequent commands (frequency, schema, sqlp, joinp)
- If the file has no headers, mention this and use column indices
