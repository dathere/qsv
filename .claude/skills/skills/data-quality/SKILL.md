# Data Quality Assessment with qsv

For the full step-by-step profiling workflow, use the `/data-profile` command. This skill provides quick-reference guidance for quality assessment and remediation decisions.

## Quality Dimensions (Quick Reference)

| Dimension | Key Question | Primary Check | Red Flag |
|-----------|-------------|---------------|----------|
| **Completeness** | Missing values? | `stats` — `nullcount`, `sparsity` | Sparsity > 0.5 |
| **Uniqueness** | Unwanted duplicates? | `stats --cardinality` vs row count | Key column cardinality < row count |
| **Validity** | Correct formats/types? | `stats` — `type`; `validate schema.json` | String type on numeric column |
| **Consistency** | Uniform formats? | `frequency` — case variants; `sniff` — encoding | Same value in different cases |
| **Accuracy** | Plausible values? | `stats` — min/max/stddev | Values > 3 stddev from mean |

## Remediation Decision Tree

When a quality issue is found, choose the right fix:

| Problem | Severity | Fix Command | When to Skip |
|---------|----------|-------------|-------------|
| Ragged rows | High | `fixlengths` | Never — breaks downstream tools |
| Wrong encoding | High | `input` | File is already UTF-8 (check with `sniff`) |
| Unsafe column names | Medium | `safenames` | Headers already safe (no spaces/special chars) |
| Leading/trailing whitespace | Medium | `sqlp` with `TRIM(col)` | Stats show no difference between `min`/`max` lengths and trimmed values |
| Duplicate rows | Medium | `dedup` (or `extdedup` for >1GB) | `stats --cardinality` on key columns shows all unique |
| Inconsistent case | Low | `sqlp` with `UPPER(col)` or `LOWER(col)` | `frequency` shows no case variants |
| Empty values | Low | `sqlp` with `COALESCE(NULLIF(col, ''), 'N/A')` | Nulls are semantically meaningful |
| Invalid rows | Low | `validate schema.json` + filter | No schema available |

## Fix Ordering

Always apply fixes in this order to avoid cascading issues:

```
1. input          (encoding — must be UTF-8 before anything else)
2. safenames      (headers — fixes names before column references)
3. fixlengths     (structure — ensures consistent field counts)
4. sqlp with TRIM()    (whitespace — clean values before dedup)
5. dedup          (duplicates — remove after trimming so "foo " and "foo" match)
6. validate       (validation — check against schema last)
```

## Stats Cache as Quality Dashboard

After running `stats --cardinality --stats-jsonl` (basic moarstats auto-runs), read the `.stats.csv` cache to assess quality in one pass:

| Cache Column | Quality Signal |
|-------------|----------------|
| `nullcount` | Completeness — 0 is ideal |
| `sparsity` | Completeness — ratio of nulls (0.0–1.0) |
| `cardinality` | Uniqueness — compare to row count |
| `type` | Validity — check expected types |
| `min` / `max` | Accuracy — plausible range? |
| `mean` / `stddev` | Accuracy — outlier detection (>3σ) |
| `outliers_total_cnt` | Accuracy — from moarstats; outlier count per column |
| `mode` | Consistency — dominant value expected? |

### Advanced Stats (via `moarstats --advanced`)

Run `moarstats --advanced` to enrich the cache with distribution shape metrics:

| Cache Column | Quality Signal |
|-------------|----------------|
| `kurtosis` | >3 heavy tails (outlier-prone), <3 light tails |
| `bimodality_coefficient` | >=0.555 suggests bimodal distribution (possible mixed populations) |
| `gini_coefficient` | Near 1 = extreme concentration; near 0 = uniform |
| `shannon_entropy` | Low = concentrated values; high = diverse |
| `winsorized_mean` | Compare to `mean` — large difference signals outlier influence |
