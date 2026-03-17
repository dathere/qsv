# Data Quality Assessment with qsv

For the full quality assessment workflow and step-by-step profiling, use the `/data-profile` command. This skill provides a quick-reference summary of the five quality dimensions and common fixes.

## Quality Dimensions (Quick Reference)

| Dimension | Key Question | Primary Check |
|-----------|-------------|---------------|
| **Completeness** | Missing values? | `stats` — `nullcount`, `sparsity` > 0.5 |
| **Uniqueness** | Unwanted duplicates? | `stats --cardinality` — cardinality vs row count |
| **Validity** | Correct formats/types? | `stats` — `type` column; `validate schema.json` |
| **Consistency** | Uniform formats? | `frequency` — case variants; `sniff` — encoding |
| **Accuracy** | Plausible values? | `stats` — min/max/stddev; `frequency --limit 20` |

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
