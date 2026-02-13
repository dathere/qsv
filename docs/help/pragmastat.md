# pragmastat

> Compute pragmatic statistics using the Pragmastat library.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/pragmastat.rs](https://github.com/dathere/qsv/blob/master/src/cmd/pragmastat.rs)** | 🤯

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Pragmastat Options](#pragmastat-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Pragmatic statistical toolkit.

Compute robust, median-of-pairwise statistics from the Pragmastat library.
Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

Input handling
* Only finite numeric values are used; non-numeric/NaN/Inf are ignored.
* Each column is treated as its own sample (two-sample compares columns, not rows).
* Non-numeric columns appear with n=0 and empty estimator cells.
* NOTE: This command loads all numeric values into memory.

ONE-SAMPLE OUTPUT (default, per selected column)
field, n, center, spread, rel_spread, center_lower, center_upper

center       Robust location; median of pairwise averages (Hodges-Lehmann).
Like the mean but stable with outliers; tolerates up to 29% corrupted data.
spread       Robust dispersion; median of pairwise absolute differences (Shamos).
Same units as data; also tolerates up to 29% corrupted data.
rel_spread   Relative dispersion = spread / center (robust coefficient of variation).
Dimensionless; compares variability across scales.

center_lower/center_upper
Bounds for center with error rate = misrate (exact under weak symmetry).
Use 1e-3 for everyday analysis or 1e-6 for critical decisions.

TWO-SAMPLE OUTPUT (--twosample, per unordered column pair)
field_x, field_y, n_x, n_y, shift, ratio, avg_spread, disparity,
shift_lower, shift_upper, ratio_lower, ratio_upper

shift        Robust difference in location; median of pairwise differences.
Negative => first column tends to be lower.
ratio        Robust multiplicative ratio; exp(shift(log x, log y)).
Use for positive-valued quantities (latency, price, concentration).
avg_spread   Pooled robust dispersion (weighted by sample sizes).
Note: pooled scale, not Spread(x union y).
disparity    Effect size = shift / avg_spread (robust Cohen's d).

shift_lower/shift_upper, ratio_lower/ratio_upper
Bounds for shift/ratio with error rate = misrate (ties may be conservative).
If bounds exclude 0 (shift) or 1 (ratio), the difference is reliable.

When values are blank
* Column has no numeric data (n=0).
* Positivity required: rel_spread, ratio, ratio_* need all values > 0.
* Sparity required: spread/avg_spread/disparity need real variability (not tie-dominant).
* Bounds require enough data for requested misrate; try higher misrate or more data.

### Misrate Parameter

misrate is the probability that bounds miss the true value (lower => wider bounds).
1e-3    Everyday analysis [default]
1e-6    Critical decisions


<a name="examples"></a>

## Examples [↩](#nav)

> Basic one-sample statistics

```console
qsv pragmastat data.csv
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

Full Pragmastat manual:
<https://github.com/AndreyAkinshin/pragmastat/releases/download/v8.0.0/pragmastat-v8.0.0.pdf>
<https://pragmastat.dev/> (latest version)

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv pragmastat [options] [<input>]
qsv pragmastat --help
```

<a name="pragmastat-options"></a>

## Pragmastat Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-t,`<br>`--twosample`&nbsp; | flag | Compute two-sample estimators for all column pairs. |  |
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select columns for analysis. Uses qsv's column selection syntax. Non-numeric columns appear with n=0. In two-sample mode, all pairs of selected columns are computed. |  |
| &nbsp;`-m,`<br>`--misrate`&nbsp; | string | Probability that bounds fail to contain the true parameter. Lower values produce wider bounds. Must be achievable for the given sample size. | `0.001` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading/writing CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be treated as headers. |  |

---
**Source:** [`src/cmd/pragmastat.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/pragmastat.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
