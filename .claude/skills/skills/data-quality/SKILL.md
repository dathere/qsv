# Data Quality Assessment with qsv

## Quality Dimensions

### 1. Completeness
**Question**: Are there missing values?

| Check | Command | What to Look For |
|-------|---------|-----------------|
| Null counts | `stats --cardinality --stats-jsonl` | `nullcount` column > 0 |
| Empty strings | `frequency --limit 10` | Empty string in top values |
| Sparsity | `stats` | `sparsity` field (ratio of nulls) |

**Red flag**: Sparsity > 0.5 means more than half the values are null.

### 2. Uniqueness
**Question**: Are there unwanted duplicates?

| Check | Command | What to Look For |
|-------|---------|-----------------|
| Duplicate rows | `dedup --dupes-output dupes.csv` | Non-empty dupes file |
| Cardinality | `stats --cardinality` | `cardinality` vs row count |
| Unique ratio | `stats` | If cardinality = row count, column is unique |

**Red flag**: Key columns (ID, email) with cardinality < row count.

### 3. Validity
**Question**: Do values match expected formats?

| Check | Command | What to Look For |
|-------|---------|-----------------|
| Schema validation | `validate schema.json` | Validation error count |
| Data types | `stats` | `type` column (String, Integer, Float, Date, etc.) |
| Format patterns | `search --flag` | Rows not matching expected regex |
| Value ranges | `stats` | `min`, `max` outside expected range |

**Red flag**: Type column shows "String" for what should be numeric data.

### 4. Consistency
**Question**: Are formats consistent across the dataset?

| Check | Command | What to Look For |
|-------|---------|-----------------|
| Date formats | `stats` | Mixed date types in same column |
| Case consistency | `frequency` | "NYC" vs "nyc" vs "Nyc" as separate values |
| Encoding | `sniff` | Non-UTF-8 encoding detected |
| Delimiters | `sniff` | Unexpected delimiter or quoting |
| Row lengths | `fixlengths` | Pads short rows to match longest row; compare count before/after to detect ragged rows |

**Red flag**: Frequency shows same value in different cases/formats.

### 5. Accuracy
**Question**: Are values plausible?

| Check | Command | What to Look For |
|-------|---------|-----------------|
| Statistical outliers | `stats` | `mean`, `stddev` - values > 3 stddev from mean |
| Value distributions | `frequency --limit 20` | Unexpected dominant values |
| Range checks | `stats` | `min`/`max` outside plausible range |
| Cross-field checks | `sqlp` | SQL WHERE clauses for business rules |

**Red flag**: Latitude > 90 or < -90, negative ages, future birth dates.

## Quality Assessment Workflow

```
1. sniff           -> Detect format, encoding, preamble issues
2. count           -> Establish baseline row count
3. headers         -> Verify expected columns exist
4. stats --cardinality --stats-jsonl -> Full statistical profile
5. frequency       -> Value distribution for categorical columns
6. validate        -> Schema validation (if schema available)
7. fixlengths      -> Pad short rows to uniform length (compare count before/after to detect ragged rows)
```

## Quality Report Checklist

After profiling, report on:

- [ ] **Row count** and **column count**
- [ ] **Null/empty counts** per column (completeness)
- [ ] **Cardinality** per column (uniqueness assessment)
- [ ] **Data types** inferred per column (validity)
- [ ] **Min/max/mean** for numeric columns (range plausibility)
- [ ] **Top frequency values** for categorical columns (distribution)
- [ ] **Duplicate rows** detected (uniqueness)
- [ ] **Schema violations** if schema provided (validity)
- [ ] **Encoding and delimiter** detected (consistency)

## Common Data Quality Fixes

| Problem | Fix Command |
|---------|-------------|
| Inconsistent case | `apply operations upper/lower col` |
| Leading/trailing whitespace | `apply operations trim col` |
| Duplicate rows | `dedup` |
| Ragged rows | `fixlengths` |
| Unsafe column names | `safenames` |
| Wrong encoding | `input` (normalizes to UTF-8) |
| Empty values | `apply emptyreplace col --replacement "N/A"` |
| Invalid rows | `validate schema.json` + filter |
