---
name: data-viz
description: Create publication-quality visualizations from CSV/TSV/Excel data using Python
user-invocable: true
argument-hint: "<file> [chart type]"
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
  - mcp__qsv__qsv_command
  # Workspace
  - mcp__qsv__qsv_list_files
  - mcp__qsv__qsv_search_tools
  - mcp__qsv__qsv_get_working_dir
  - mcp__qsv__qsv_set_working_dir
---

# Data Viz

Create publication-quality data visualizations from tabular data files. Uses qsv to profile and prepare data, then generates Python charts with best practices for clarity, accuracy, and design.

> **Cowork note:** If relative paths don't resolve, call `qsv_get_working_dir` and `qsv_set_working_dir` to sync the working directory.

## Steps

### 1. Understand the Request

Determine:
- **Data source**: CSV/TSV/Excel file, or query results from a prior analysis
- **Chart type**: Explicitly requested or needs to be recommended
- **Purpose**: Exploration, presentation, report, dashboard component
- **Audience**: Technical team, executives, external stakeholders

### 2. Profile the Data with qsv

a. **Index and detect**: Run `qsv_index`, then `qsv_sniff` to detect format and encoding.

b. **Understand structure**: Run `qsv_headers` and `qsv_count` to get column names and row count.

c. **Profile columns**: Run `qsv_stats` with `cardinality: true, stats_jsonl: true` to understand types, ranges, and distributions. Read `.stats.csv` to inform chart design:
   - `type` → choose appropriate axis type (numeric, categorical, date)
   - `min`/`max` → set axis ranges
   - `cardinality` → determine if column is categorical (low) or continuous (high)
   - `nullcount` → note missing data that could affect the chart

d. **Check distributions**: Run `qsv_frequency` with `limit: 20` on columns you plan to plot — this reveals the actual values and whether grouping or filtering is needed.

e. **Preview data**: Run `qsv_slice` with `len: 5` to see actual values and formats.

### 3. Prepare the Data

Use qsv to prepare visualization-ready data:

- **Filter**: `qsv_search` or `qsv_sqlp` to subset rows
- **Aggregate**: `qsv_sqlp` for GROUP BY, window functions, computed columns
- **Select columns**: `qsv_select` to keep only what's needed
- **Sort**: `qsv_sqlp` with ORDER BY for ordered categories or time series

Export the prepared data to a CSV file for Python to read.

### 4. Select Chart Type

If the user didn't specify, recommend based on data and question:

| Data Relationship | Recommended Chart | How qsv Helps Choose |
|-------------------|-------------------|---------------------|
| Trend over time | Line chart | `stats` shows Date/DateTime type |
| Comparison across categories | Bar chart (horizontal if many) | `frequency` shows category counts; `cardinality` < 20 |
| Part-to-whole composition | Stacked bar or area chart | `frequency` shows proportions; avoid pie unless < 6 categories |
| Distribution of values | Histogram or box plot | `stats` shows min/max/mean/stddev; `moarstats` shows kurtosis |
| Correlation between two variables | Scatter plot | `stats` shows two numeric columns |
| Ranking | Horizontal bar chart | `frequency` with `--limit` for top-N |
| Matrix of relationships | Heatmap | Two categorical columns with low `cardinality` |
| Two-variable comparison over time | Dual-axis line or grouped bar | Two numeric columns + one Date column |

### 5. Generate the Visualization

Write Python code using matplotlib + seaborn (default) or plotly (if interactive requested):

```python
import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd

# Load the prepared CSV
df = pd.read_csv('prepared_data.csv')

# Set professional style
plt.style.use('seaborn-v0_8-whitegrid')
sns.set_palette("husl")

# Create figure with appropriate size
fig, ax = plt.subplots(figsize=(10, 6))

# [chart-specific code]

# Always include:
ax.set_title('Clear, Descriptive Title', fontsize=14, fontweight='bold')
ax.set_xlabel('X-Axis Label', fontsize=11)
ax.set_ylabel('Y-Axis Label', fontsize=11)

# Format numbers appropriately
# - Percentages: '45.2%' not '0.452'
# - Currency: '$1.2M' not '1200000'
# - Large numbers: '2.3K' or '1.5M' not '2300' or '1500000'

# Remove chart junk
ax.spines['top'].set_visible(False)
ax.spines['right'].set_visible(False)

plt.tight_layout()
plt.savefig('chart_name.png', dpi=150, bbox_inches='tight')
plt.show()
```

### 6. Apply Design Best Practices

**Color:**
- Use a consistent, colorblind-friendly palette
- Highlight the key data point or trend with a contrasting color
- Grey out less important reference data

**Typography:**
- Descriptive title that states the insight, not just the metric (e.g., "Revenue grew 23% YoY" not "Revenue by Month")
- Readable axis labels (not rotated 90 degrees if avoidable)
- Data labels on key points when they add clarity

**Layout:**
- Appropriate whitespace and margins
- Legend placement that doesn't obscure data
- Sort categories by value (not alphabetically) unless there's a natural order

**Accuracy:**
- Y-axis starts at zero for bar charts
- No misleading axis breaks without clear notation
- Consistent scales when comparing panels
- Appropriate precision (don't show 10 decimal places)

### 7. Save and Present

1. Save the chart as PNG with descriptive name
2. Display the chart to the user
3. Provide the Python code so they can modify it
4. Suggest variations (different chart type, different grouping, zoomed time range)

## Data Preparation Recipes

### Time Series
```
qsv_sqlp: SELECT date_col, SUM(value) as total
  FROM data GROUP BY date_col ORDER BY date_col
```

### Top-N Categories
```
qsv_frequency: --select category_col --limit 10
```
Or for aggregated values:
```
qsv_sqlp: SELECT category, SUM(amount) as total
  FROM data GROUP BY category ORDER BY total DESC LIMIT 10
```

### Distribution
```
qsv_stats: Check min, max, mean, stddev, cardinality
qsv_moarstats: --advanced for kurtosis, bimodality
qsv_sqlp: SELECT FLOOR(value/10)*10 as bin, COUNT(*) as cnt
  FROM data GROUP BY bin ORDER BY bin
```

### Correlation
```
qsv_select: Pick the two numeric columns
qsv_stats: Verify both are numeric types with reasonable ranges
```

### Comparison Across Groups
```
qsv_sqlp: SELECT group_col, AVG(metric) as avg_metric, COUNT(*) as n
  FROM data GROUP BY group_col ORDER BY avg_metric DESC
```

## Notes

- Always profile with qsv first — `stats` and `frequency` reveal the right chart type and catch data issues before plotting
- For large files, use `qsv_sqlp` to aggregate before passing to Python — don't load millions of rows into pandas
- If interactive charts are requested (hover, zoom), use plotly instead of matplotlib
- Specify "presentation" for larger fonts and higher contrast
- Multiple charts can be created at once (e.g., "create a 2x2 grid")
- Charts are saved to the current directory as PNG files
- For data quality issues discovered during profiling, recommend `/data-clean` before visualizing
