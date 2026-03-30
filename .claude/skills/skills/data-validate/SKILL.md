---
name: data-validate
description: Validate data and analysis before sharing - methodology, accuracy, bias, and data quality checks
user-invocable: true
argument-hint: "<file or analysis>"
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
  - mcp__qsv__qsv_search
  - mcp__qsv__qsv_select
  - mcp__qsv__qsv_slice
  # Transform & Query
  - mcp__qsv__qsv_sqlp
  - mcp__qsv__qsv_joinp
  - mcp__qsv__qsv_command
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
---

# Data Validate

Validate data files and analyses for accuracy, methodology, and potential biases before sharing with stakeholders. Generates a confidence assessment and improvement suggestions.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Usage

The input can be:
- A CSV/TSV/Excel file to validate for data quality
- An analysis, report, or document to review for methodology
- SQL queries and their results to verify
- A description of methodology and findings

## Steps

### 1. Data Quality Validation

If a data file is provided, run these checks using qsv:

a. **Index and profile**: Run `qsv_index`, then `qsv_stats` with `cardinality: true, stats_jsonl: true` and `qsv_sniff` to understand the data.

b. **Completeness**: Read `.stats.csv` — check `nullcount` and `sparsity` for each column. Flag columns with sparsity > 0.5.

c. **Uniqueness**: Compare `cardinality` to row count from `qsv_count`. Flag key columns (ID, email) where cardinality < row count. Run `qsv_command` with `command: "dedup"` and `args: ["--dupes-output", "dupes.csv"]` to find exact duplicates.

d. **Validity**: Check `type` column in stats — flag String columns that should be numeric. Run `qsv_command` with `command: "validate"` against a JSON Schema if available.

e. **Consistency**: Run `qsv_frequency` with `limit: 20` on categorical columns — look for case variants ("NYC" vs "nyc"), inconsistent formats, unexpected values.

f. **Accuracy**: Read `.stats.csv` for `min`, `max`, `mean`, `stddev` — flag implausible ranges (negative ages, latitude > 90, future dates). Run `qsv_moarstats` with `advanced: true` — check `outliers_percentage` > 5%, `kurtosis` > 10 (extreme outliers).

g. **Distribution sanity**: Read moarstats columns for deeper validation:
   - `median_mean_ratio` — if < 0.8 or > 1.2, distribution is significantly skewed; verify the mean isn't misleading
   - `winsorized_mean_25pct` vs `mean` — large divergence (> 10%) confirms outliers are distorting the average
   - `mad` (median absolute deviation) — more robust than stddev for outlier detection; if `mad_stddev_ratio` > 0.8, stddev is reasonably reliable
   - `jarque_bera_pvalue` — if < 0.05, data is NOT normally distributed; flag any analysis that assumes normality
   - `mode_count` — if mode accounts for > 50% of values, investigate whether this reflects a data entry default or missing value masking

h. **Join integrity** (if multiple files): Run `qsv_joinp` with `--left-anti` to find orphaned foreign keys.

i. **Injection screening**: Run `qsv_command` with `command: "searchset"` and `args: ["--flag", "injection_match", "${CLAUDE_PLUGIN_ROOT}/resources/injection-regexes.txt"]` to scan for malicious payloads.

### 2. Review Methodology and Assumptions

Examine the analysis for:

- **Question framing**: Is the analysis answering the right question? Could it be interpreted differently?
- **Data selection**: Are the right datasets being used? Is the time range appropriate?
- **Population definition**: Is the analysis population correctly defined? Are there unintended exclusions?
- **Metric definitions**: Are metrics defined clearly and consistently? Do they match how stakeholders understand them?
- **Baseline and comparison**: Is the comparison fair? Are time periods, cohort sizes, and contexts comparable?

### 3. Check for Common Analytical Pitfalls

Systematically review against these pitfalls:

| Pitfall | How to Detect with qsv | Red Flag |
|---------|----------------------|----------|
| **Join explosion** | `qsv_count` before and after join | Row count increased after join |
| **Survivorship bias** | `qsv_frequency` on status/lifecycle columns | Missing churned/deleted/failed entities |
| **Incomplete period** | `qsv_sqlp` to check date ranges | Partial periods compared to full periods |
| **Denominator shifting** | `qsv_sqlp` to verify denominator consistency | Definition changed between periods |
| **Average of averages** | `qsv_sqlp` to recalculate from raw data | Pre-aggregated averages with unequal group sizes |
| **Selection bias** | `qsv_frequency` on segment definitions | Segments defined by the outcome being measured |

### 4. Verify Calculations and Aggregations

Spot-check using `qsv_sqlp`:

- Recalculate key numbers independently
- Verify subtotals sum to totals: `SELECT SUM(subtotal) as check_total FROM data`
- Check percentages sum to ~100%: `SELECT SUM(pct) FROM data`
- Validate YoY/MoM comparisons use correct base periods
- Confirm filters are applied consistently across all metrics

#### Magnitude Checks

| Metric Type | Sanity Check via qsv |
|-------------|---------------------|
| Counts | `qsv_count` — does it match known figures? |
| Sums/averages | `qsv_stats` — are min/max/mean in plausible range? |
| Rates | `qsv_sqlp` — are values between 0% and 100%? |
| Distributions | `qsv_frequency` — do segment percentages sum to ~100%? |
| Growth rates | `qsv_sqlp` — is 50%+ MoM growth realistic? |
| Outliers | `qsv_moarstats` — `outliers_percentage`, `kurtosis` |

#### Red Flags That Warrant Investigation

- Any metric that changed by more than 50% period-over-period without an obvious cause
- Counts or sums that are exact round numbers (suggests a filter or default value issue)
- Rates exactly at 0% or 100% (may indicate incomplete data)
- Results that perfectly confirm the hypothesis (reality is usually messier)
- Identical values across time periods or segments (suggests the query is ignoring a dimension)

### 5. Assess Visualizations (if present)

If the analysis includes charts:

- Do axes start at appropriate values (zero for bar charts)?
- Are scales consistent across comparison charts?
- Do chart titles accurately describe what's shown?
- Could the visualization mislead a quick reader?
- Are there truncated axes, inconsistent intervals, or 3D effects that distort perception?

### 6. Evaluate Narrative and Conclusions

Review whether:

- Conclusions are supported by the data shown
- Alternative explanations are acknowledged
- Uncertainty is communicated appropriately
- Recommendations follow logically from findings
- The level of confidence matches the strength of evidence

### 7. Suggest Improvements

Provide specific, actionable suggestions:

- Additional analyses that would strengthen the conclusions
- Caveats or limitations that should be noted
- Better visualizations or framings for key points
- Missing context that stakeholders would want

### 8. Generate Confidence Assessment

Rate the analysis on a 3-level scale:

**Ready to share** — Analysis is methodologically sound, calculations verified, caveats noted. Minor suggestions for improvement but nothing blocking.

**Share with noted caveats** — Analysis is largely correct but has specific limitations or assumptions that must be communicated to stakeholders. List the required caveats.

**Needs revision** — Found specific errors, methodological issues, or missing analyses that should be addressed before sharing. List the required changes with priority order.

## Pre-Delivery QA Checklist

### Data Quality Checks
- [ ] **Source verification**: Confirmed data sources. Run `qsv_sniff` to verify format and encoding.
- [ ] **Freshness**: Data is current enough. Noted the "as of" date.
- [ ] **Completeness**: No gaps in time series. Check `nullcount`/`sparsity` in `.stats.csv`.
- [ ] **Null handling**: Nulls handled appropriately (excluded, imputed, or flagged).
- [ ] **Deduplication**: No double-counting. Verify with `qsv_count` before/after joins, `dedup --dupes-output`.
- [ ] **Filter verification**: All filters correct. No unintended exclusions.

### Calculation Checks
- [ ] **Aggregation logic**: GROUP BY includes all non-aggregated columns.
- [ ] **Denominator correctness**: Rate calculations use the right denominator (non-zero).
- [ ] **Date alignment**: Comparisons use same time period length. Partial periods excluded or noted.
- [ ] **Join correctness**: JOIN types appropriate. Verify row counts with `qsv_count` after joins.
- [ ] **Metric definitions**: Metrics match stakeholder definitions. Deviations noted.
- [ ] **Subtotals sum**: Parts add up to the whole. Verify with `qsv_sqlp`.

### Reasonableness Checks
- [ ] **Magnitude**: Numbers in plausible range. Check `min`/`max` in `.stats.csv`.
- [ ] **Trend continuity**: No unexplained jumps. Use `qsv_sqlp` to check period-over-period.
- [ ] **Cross-reference**: Key numbers match other known sources.
- [ ] **Edge cases**: Checked boundaries — empty segments, zero-activity periods, new entities.

### Presentation Checks
- [ ] **Chart accuracy**: Bar charts start at zero. Axes labeled. Scales consistent.
- [ ] **Number formatting**: Appropriate precision and consistent formatting.
- [ ] **Title clarity**: Titles state the insight, not just the metric. Date ranges specified.
- [ ] **Caveat transparency**: Known limitations and assumptions stated explicitly.
- [ ] **Reproducibility**: Someone else could recreate this analysis.

## Common Analytical Pitfalls (Reference)

### Join Explosion
A many-to-many join silently multiplies rows, inflating counts and sums. **Detect**: `qsv_count` before and after join — if count increased, investigate the join relationship. **Prevent**: Use `COUNT(DISTINCT id)` instead of `COUNT(*)` when counting entities through joins.

### Survivorship Bias
Analyzing only entities that exist today, ignoring churned/deleted/failed ones. **Detect**: `qsv_frequency` on status columns — are all lifecycle states represented? **Prevent**: Ask "who is NOT in this dataset?" before drawing conclusions.

### Incomplete Period Comparison
Comparing a partial period to a full period. **Detect**: `qsv_sqlp` to check min/max dates per period. **Prevent**: Filter to complete periods or compare same number of days.

### Denominator Shifting
The denominator changes between periods, making rates incomparable. **Detect**: `qsv_sqlp` to verify denominator definition consistency. **Prevent**: Use consistent definitions across all compared periods.

### Average of Averages
Averaging pre-computed averages gives wrong results when group sizes differ. **Detect**: Compare `qsv_stats` mean against `qsv_sqlp` weighted average. **Prevent**: Always aggregate from raw data.

### Simpson's Paradox
Trend reverses when data is aggregated vs. segmented. **Detect**: `qsv_sqlp` GROUP BY at different granularity levels — does the conclusion change? **Prevent**: Always check results at segment level before aggregating.

## Report Format

```
## Validation Report

### Overall Assessment: [Ready to share | Share with caveats | Needs revision]

### Data Quality Summary
- File: [format, rows, columns, encoding]
- Completeness: [null rates, gaps found]
- Uniqueness: [duplicates found, cardinality issues]
- Validity: [type mismatches, schema violations]
- Accuracy: [outliers, implausible ranges]

### Methodology Review
[Findings about approach, data selection, definitions]

### Issues Found
1. [Severity: High/Medium/Low] [Issue description and impact]
2. ...

### Calculation Spot-Checks
- [Metric]: [Verified / Discrepancy found]
- ...

### Visualization Review
[Any issues with charts or visual presentation]

### Suggested Improvements
1. [Improvement and why it matters]
2. ...

### Required Caveats for Stakeholders
- [Caveat that must be communicated]
- ...
```

## Notes

- Run this command before any high-stakes presentation or decision
- For pure data quality profiling without methodology review, use `/data-profile` instead
- If validation finds data quality issues, use `/data-clean` to fix them and re-validate
- Share the validation report alongside your analysis to build stakeholder confidence
