# pragmastat

> Compute pragmatic statistics using the [Pragmastat](https://pragmastat.dev/) library. Uses the stats cache to auto-filter non-numeric columns and support Date/DateTime columns.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/pragmastat.rs](https://github.com/dathere/qsv/blob/master/src/cmd/pragmastat.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🤯](TableOfContents.md#legend "loads entire CSV into memory, though `dedup`, `stats` & `transpose` have \"streaming\" modes as well.")[🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Pragmastat Options](#pragmastat-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Pragmatic statistical toolkit.

Compute robust, median-of-pairwise statistics from the Pragmastat library.
Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

This is a "smart" command that uses the stats cache to work smarter & faster.
When a stats cache is available, non-numeric columns are automatically filtered out
(unless --select is explicitly provided) and Date/DateTime columns are supported.

By default, one-sample mode appends 7 ps_* columns to the .stats.csv cache file
(like moarstats). Use --standalone for the old standalone CSV output. Two-sample,
compare1, and compare2 modes always produce standalone output.

Input handling
* Only finite numeric values are used; non-numeric/NaN/Inf are ignored.
* Date/DateTime columns are supported when a stats cache is available
(run "qsv stats -E --infer-dates --stats-jsonl" first). Dates are converted to epoch
milliseconds for analysis, then center/bounds are formatted as dates and spread/shift
as days.
* Each column is treated as its own sample (two-sample compares columns, not rows).
* Non-numeric columns appear with n=0 and empty estimator cells.
* NOTE: This command loads all numeric values into memory.

ONE-SAMPLE OUTPUT (default, per selected column)
field, n, center, spread, center_lower, center_upper, spread_lower, spread_upper

center             Robust location; median of pairwise averages (Hodges-Lehmann).
Like the mean but stable with outliers; tolerates up to 29% corrupted data.
spread             Robust dispersion; median of pairwise absolute differences (Shamos).
Same units as data; also tolerates up to 29% corrupted data.
center_lower/upper Bounds for center with error rate = misrate (exact under weak symmetry).
Use 1e-3 for everyday analysis or 1e-6 for critical decisions.
spread_lower/upper Bounds for spread with error rate = misrate (randomized).

TWO-SAMPLE OUTPUT (--twosample, per unordered column pair)
field_x, field_y, n_x, n_y, shift, ratio, disparity,
shift_lower, shift_upper, ratio_lower, ratio_upper, disparity_lower, disparity_upper

shift                 Robust difference in location; median of pairwise differences.
Negative => first column tends to be lower.
ratio                 Robust multiplicative ratio; exp(shift(log x, log y)).
Use for positive-valued quantities (latency, price, concentration).
disparity             Robust effect size = shift / (average spread of x and y).
shift_lower/upper     Bounds for shift (exact; ties may be conservative).
If bounds exclude 0, the shift is reliable.
ratio_lower/upper     Bounds for ratio (exact; requires all values > 0).
If bounds exclude 1, the ratio is reliable.
disparity_lower/upper Bounds for disparity (randomized, Bonferroni combination).
If bounds exclude 0, the disparity is reliable.

When values are blank
* Column has no numeric data (n=0).
* Positivity required: ratio, ratio_* need all values > 0.
* Sparity required: spread/spread_*/disparity/disparity_* need real variability (not tie-dominant).
* Bounds require enough data for requested misrate; try higher misrate or more data.

### Misrate Parameter

misrate is the probability that bounds miss the true value (lower => wider bounds).
1e-3    Everyday analysis [default]
1e-6    Critical decisions

COMPARE1 OUTPUT (--compare1, one-sample confirmatory analysis)
field, n, metric, threshold, estimate, lower, upper, verdict

Tests one-sample estimates (center/spread) against user-defined thresholds.
Each threshold produces one row per column with a verdict:  
less          Estimate is statistically less than the threshold.
greater       Estimate is statistically greater than the threshold.
inconclusive  Not enough evidence to decide (interval contains threshold).

COMPARE2 OUTPUT (--compare2, two-sample confirmatory analysis)
field_x, field_y, n_x, n_y, metric, threshold, estimate, lower, upper, verdict

Tests two-sample estimates (shift/ratio/disparity) against user-defined thresholds.
Each threshold produces one row per column pair with the same verdict semantics.

### Threshold Format

Both compare flags accept a comma-separated list of metric:value pairs.
compare1 center:42.0             Single threshold
compare1 center:42.0,spread:0.5  Multiple thresholds
compare2 shift:0,disparity:0.8   Two-sample thresholds

Valid metrics for compare1: center, spread
Valid metrics for compare2: shift, ratio, disparity


<a name="examples"></a>

## Examples [↩](#nav)

> Append pragmastat columns to stats cache (default one-sample behavior)

```console
qsv pragmastat data.csv
```

> Standalone one-sample output (old behavior)

```console
qsv pragmastat --standalone data.csv
```

> One-sample statistics with selected columns

```console
qsv pragmastat --select latency_ms,price data.csv
```

> Two-sample statistics with selected columns

```console
qsv pragmastat --twosample --select latency_ms,price data.csv
```

> One-sample statistics with very tight bounds (lower misrate)

```console
qsv pragmastat --misrate 1e-6 data.csv
```

> Compare one-sample center against a threshold

```console
qsv pragmastat --compare1 center:42.0 --select latitude data.csv
```

> Compare one-sample center and spread against thresholds

```console
qsv pragmastat --compare1 center:42.0,spread:0.5 --select latitude data.csv
```

> Compare two-sample shift and disparity against thresholds

```console
qsv pragmastat --compare2 shift:0,disparity:0.8 --select latency_ms,price data.csv
```

> Fast exploratory analysis with subsampling (~100x speedup on large datasets)

```console
qsv pragmastat --standalone --subsample 10000 data.csv
```

> Reproducible subsampling with a specific seed

```console
qsv pragmastat --standalone --subsample 10000 --seed 123 data.csv
```

> Skip confidence bounds for ~2x speedup

```console
qsv pragmastat --standalone --no-bounds data.csv
```

> Combined: ~200x speedup for large datasets

```console
qsv pragmastat --standalone --subsample 10000 --no-bounds data.csv
```

Full Pragmastat manual:  
<https://github.com/AndreyAkinshin/pragmastat/releases/download/v12.0.0/pragmastat-v12.0.0.pdf>
<https://pragmastat.dev/> (latest version)

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv pragmastat [options] [<input>]
qsv pragmastat --help
```

<a name="pragmastat-options"></a>

## Pragmastat Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑t,`<br>`‑‑twosample`&nbsp; | flag | Compute two-sample estimators for all column pairs. |  |
| &nbsp;`‑‑compare1`&nbsp; | string | One-sample confirmatory analysis. Test center/spread against thresholds. Format: metric:value[,metric:value,...]. Mutually exclusive with --twosample and --compare2. |  |
| &nbsp;`‑‑compare2`&nbsp; | string | Two-sample confirmatory analysis. Test shift/ratio/disparity against thresholds. Format: metric:value[,metric:value,...]. Mutually exclusive with --twosample and --compare1. |  |
| &nbsp;`‑s,`<br>`‑‑select`&nbsp; | string | Select columns for analysis. Uses qsv's column selection syntax. Non-numeric columns appear with n=0. In two-sample mode, all pairs of selected columns are computed. |  |
| &nbsp;`‑m,`<br>`‑‑misrate`&nbsp; | string | Probability that bounds fail to contain the true parameter. Lower values produce wider bounds. Must be achievable for the given sample size. | `0.001` |
| &nbsp;`‑‑standalone`&nbsp; | flag | Output one-sample results as standalone CSV instead of appending to the stats cache. |  |
| &nbsp;`‑‑stats‑options`&nbsp; | string | Options to pass to the stats command if baseline stats need to be generated. The options are passed as a single string that will be split by whitespace. | `--infer-dates --infer-boolean --mad --quartiles --force --stats-jsonl` |
| &nbsp;`‑‑round`&nbsp; | string | Round statistics to <n> decimal places. Rounding follows Midpoint Nearest Even (Bankers Rounding) rule. | `4` |
| &nbsp;`‑‑force`&nbsp; | flag | Force recomputing ps_* columns even if they already exist in the stats cache. |  |
| &nbsp;`‑‑subsample`&nbsp; | string | Randomly subsample N values per column before computing. Speeds up large datasets while maintaining statistical robustness. Recommended: 10000-50000 for exploratory analysis. |  |
| &nbsp;`‑‑seed`&nbsp; | string | Seed for reproducible subsampling. If not specified, defaults to 42 when --subsample is used. |  |
| &nbsp;`‑‑no‑bounds`&nbsp; | flag | Skip confidence bounds computation (~2x faster). Incompatible with --compare1/--compare2. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading/writing CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will not be treated as headers. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`‑‑memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. Not valid for stdin. |  |

---
**Source:** [`src/cmd/pragmastat.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/pragmastat.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
