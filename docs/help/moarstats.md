# moarstats

> Add dozens of additional statistics, including extended outlier, robust & bivariate statistics to an existing stats CSV file. ([example](../moarstats/NYC_311_SR_2010-2020-sample-1M.stats.csv)).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/moarstats.rs](https://github.com/dathere/qsv/blob/master/src/cmd/moarstats.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="multithreaded and/or faster when an index (📇) is available.">🏎️</abbr>

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Moarstats Options](#moarstats-options) | [Bivariate Statistics Options](#bivariate-statistics-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Add dozens of additional statistics, including extended outlier, robust & bivariate
statistics to an existing stats CSV file. It also maps the field type to the most specific
W3C XML Schema Definition (XSD) datatype (<https://www.w3.org/TR/xmlschema-2/>).

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" (<https://www.dictionary.com/culture/slang/moar>) statistics that can be
derived from existing stats columns and by scanning the original CSV file.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following 18 additional univariate statistics:
1. Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
Measures asymmetry of the distribution.
Positive values indicate right skew, negative values indicate left skew.
<https://en.wikipedia.org/wiki/Skewness>
2. Range to Standard Deviation Ratio: range / stddev
Normalizes the spread of data.
Higher values indicate more extreme outliers relative to the variability.
3. Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
Measures relative variability using quartiles.
Useful for comparing dispersion across different scales.
<https://en.wikipedia.org/wiki/Quartile_coefficient_of_dispersion>
4. Z-Score of Mode: (mode - mean) / stddev
Indicates how typical the mode is relative to the distribution.
Values near 0 suggest the mode is near the mean.
5. Relative Standard Error: sem / mean
Measures precision of the mean estimate relative to its magnitude.
Lower values indicate more reliable estimates.
6. Z-Score of Min: (min - mean) / stddev
Shows how extreme the minimum value is.
Large negative values indicate outliers or heavy left tail.
7. Z-Score of Max: (max - mean) / stddev
Shows how extreme the maximum value is.
Large positive values indicate outliers or heavy right tail.
8. Median-to-Mean Ratio: median / mean
Indicates skewness direction.
Ratio < 1 suggests right skew, > 1 suggests left skew, = 1 suggests symmetry.
9. IQR-to-Range Ratio: iqr / range
Measures concentration of data.
Higher values (closer to 1) indicate more data concentrated in the middle 50%.
10. MAD-to-StdDev Ratio: mad / stddev
Compares robust vs non-robust spread measures.
Higher values suggest presence of outliers affecting stddev.
11. Kurtosis: Measures the "tailedness" of the distribution (excess kurtosis).
Positive values indicate heavy tails, negative values indicate light tails.
Values near 0 indicate a normal distribution.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Kurtosis>
12. Bimodality Coefficient: Measures whether a distribution has two modes (peaks) or is unimodal.
BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal.
Computed as (skewness² + 1) / (kurtosis + 3).
Requires --advanced flag (needs skewness from base stats and kurtosis from --advanced flag).
<https://en.wikipedia.org/wiki/Bimodality>
13. Gini Coefficient: Measures inequality/dispersion in the distribution.
Values range from 0 (perfect equality) to 1 (maximum inequality).
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Gini_coefficient>
14. Atkinson Index: Measures inequality in the distribution with a sensitivity parameter.
Values range from 0 (perfect equality) to 1 (maximum inequality).
The Atkinson Index is a more general form of the Gini coefficient that allows for
different sensitivity to inequality. Sensitivity is configurable via --epsilon.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Atkinson_index>
15. Shannon Entropy: Measures the information content/uncertainty in the distribution.
Higher values indicate more diversity, lower values indicate more concentration.
Values range from 0 (all values identical) to log2(n) where n is the number of unique values.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Entropy_(information_theory)>
16. Normalized Entropy: Normalized version of Shannon Entropy scaled to [0, 1].
Values range from 0 (all values identical) to 1 (all values equally distributed).
Computed as shannon_entropy / log2(cardinality).
Requires shannon_entropy (from --advanced flag) and cardinality (from base stats).
17. Winsorized Mean: Replaces values below/above thresholds with threshold values, then computes mean.
All values are included in the calculation, but extreme values are capped at thresholds.
<https://en.wikipedia.org/wiki/Winsorized_mean>
Also computes: winsorized_stddev, winsorized_variance, winsorized_cv, winsorized_range,
and winsorized_stddev_ratio (winsorized_stddev / overall_stddev).
18. Trimmed Mean: Excludes values outside thresholds, then computes mean.
Only values within thresholds are included in the calculation.
<https://en.wikipedia.org/wiki/Truncated_mean>
Also computes: trimmed_stddev, trimmed_variance, trimmed_cv, trimmed_range,
and trimmed_stddev_ratio (trimmed_stddev / overall_stddev).
By default, uses Q1 and Q3 as thresholds (25% winsorization/trimming).
With --use-percentiles, uses configurable percentiles (e.g., 5th/95th) as thresholds
with --pct-thresholds.

In addition, it computes the following univariate outlier statistics (24 outlier statistics total).
<https://en.wikipedia.org/wiki/Outlier>
(requires --quartiles or --everything in stats):

Outlier Counts (7 statistics):
- outliers_extreme_lower_cnt: Count of values below the lower outer fence
- outliers_mild_lower_cnt: Count of values between lower outer and inner fences
- outliers_normal_cnt: Count of values between inner fences (non-outliers)
- outliers_mild_upper_cnt: Count of values between upper inner and outer fences
- outliers_extreme_upper_cnt: Count of values above the upper outer fence
- outliers_total_cnt: Total count of all outliers (sum of extreme and mild outliers)
- outliers_percentage: Percentage of values that are outliers

Outlier Descriptive Statistics (6 statistics):
- outliers_mean: Mean value of outliers
- non_outliers_mean: Mean value of non-outliers
- outliers_to_normal_mean_ratio: Ratio of outlier mean to non-outlier mean
- outliers_min: Minimum value among outliers
- outliers_max: Maximum value among outliers
- outliers_range: Range of outlier values (max - min)

Outlier Variance/Spread Statistics (7 statistics):
- outliers_stddev: Standard deviation of outlier values
- outliers_variance: Variance of outlier values
- non_outliers_stddev: Standard deviation of non-outlier values
- non_outliers_variance: Variance of non-outlier values
- outliers_cv: Coefficient of variation for outliers (stddev / mean)
- non_outliers_cv: Coefficient of variation for non-outliers (stddev / mean)
- outliers_normal_stddev_ratio: Ratio of outlier stddev to non-outlier stddev

Outlier Impact Statistics (2 statistics):
- outlier_impact: Difference between overall mean and non-outlier mean
- outlier_impact_ratio: Relative impact (outlier_impact / non_outlier_mean)

Outlier Boundary Statistics (2 statistics):
- lower_outer_fence_zscore: Z-score of the lower outer fence boundary
- upper_outer_fence_zscore: Z-score of the upper outer fence boundary

These outlier statistics require reading the original CSV file and comparing each
value against the fence thresholds.
Fences are computed using the IQR method:
inner fences at Q1/Q3 ± 1.5*IQR, outer fences at Q1/Q3 ± 3.0*IQR.

These univariate statistics are only computed for numeric and date/datetime columns
where the required base univariate statistics (mean, median, stddev, etc.) are available.
Univariate outlier statistics additionally require that quartiles (and thus fences) were
computed when generating the stats CSV.
Winsorized/trimmed means require either Q1/Q3 or percentiles to be available.
Kurtosis, Gini & Atkinson Index require reading the original CSV file to collect
all values for computation.

BIVARIATE STATISTICS:

The `moarstats` command also computes the following 6 bivariate statistics:
1. Pearson's correlation
Measures linear correlation between two numeric/date fields.
Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
0 indicates no linear correlation.
<https://en.wikipedia.org/wiki/Pearson_correlation_coefficient>
2. Spearman's rank correlation
Measures monotonic correlation between two numeric/date fields.
Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
0 indicates no monotonic correlation.
<https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient>
3. Kendall's tau
Measures monotonic correlation between two numeric/date fields.
Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
0 indicates no monotonic correlation.
<https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient>
4. Covariance
Measures the linear relationship between two numeric/date fields.
Values range from negative infinity to positive infinity.
0 indicates no linear relationship.
<https://en.wikipedia.org/wiki/Covariance>
5. Mutual Information
Measures the amount of information obtained about one field by observing another.
Values range from 0 (independent) to positive infinity.
<https://en.wikipedia.org/wiki/Mutual_information>
6. Normalized Mutual Information
Normalized version of mutual information, scaled by the geometric mean of individual entropies.
Values range from 0 (independent) to 1 (perfectly dependent).
<https://en.wikipedia.org/wiki/Mutual_information#Normalized_variants>

These bivariate statistics are computed when the `--bivariate` flag is used
and require an indexed CSV file (index will be auto-created if missing).
Bivariate statistics are output to a separate file: `<FILESTEM>.stats.bivariate.csv`.

Bivariate statistics require reading the entire CSV file and are computationally VERY expensive.
For large files (>= 10k records), parallel chunked processing is used when an index is available.
For smaller files or when no index exists, sequential processing is used.

MULTI-DATASET BIVARIATE STATISTICS:

When using the `--join-inputs` flag, multiple datasets can be joined internally before
computing bivariate statistics. This allows analyzing bivariate statistics across datasets
that share common join keys. The joined dataset is saved as a temporary file that is
automatically deleted after computing the bivariate statistics.
The bivariate statistics are saved to `<FILESTEM>.stats.bivariate.joined.csv`.


<a name="examples"></a>

## Examples [↩](#nav)

> Add moar stats to existing stats file

```console
qsv moarstats data.csv
```

> Generate baseline stats first with custom options, then add moar stats

```console
qsv moarstats data.csv --stats-options "--everything --infer-dates"
```

> Compute bivariate statistics between fields

```console
qsv moarstats data.csv --bivariate
```

> Compute even more bivariate statistics

```console
qsv moarstats data.csv --bivariate --bivariate-stats pearson,spearman,kendall,mi,nmi,covariance
```

> Join multiple datasets and compute bivariate statistics

```console
qsv moarstats data.csv --bivariate --join-inputs customers.csv,products.csv --join-keys cust_id,prod_id
```

> Join multiple datasets and compute bivariate statistics with different join type

```console
qsv moarstats data.csv --bivariate --join-inputs customers.csv,products.csv --join-keys cust_id,prod_id --join-type left
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_moarstats.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv moarstats [options] [<input>]
qsv moarstats --help
```

<a name="moarstats-options"></a>

## Moarstats Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--advanced`&nbsp; | flag | Compute Kurtosis, Shannon Entropy, Bimodality Coefficient, Gini Coefficient and Atkinson Index. These advanced statistics computations require reading the original CSV file to collect all values for computation and are computationally expensive. Further, Entropy computation requires the frequency command to be run with --limit 0 to collect all frequencies. An index will be auto-created for the original CSV file if it doesn't already exist to enable parallel processing. |  |
| &nbsp;`-e,`<br>`--epsilon`&nbsp; | string | The Atkinson Index Inequality Aversion parameter. Epsilon controls the sensitivity of the Atkinson Index to inequality. The higher the epsilon, the more sensitive the index is to inequality. Typical values are 0.5 (standard in economic research), 1.0 (natural boundary), or 2.0 (useful for poverty analysis). | `1.0` |
| &nbsp;`--stats-options`&nbsp; | string | Options to pass to the stats command if baseline stats need to be generated. The options are passed as a single string that will be split by whitespace. | `--infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl` |
| &nbsp;`--round`&nbsp; | string | Round statistics to <n> decimal places. Rounding follows Midpoint Nearest Even (Bankers Rounding) rule. | `4` |
| &nbsp;`--use-percentiles`&nbsp; | flag | Use percentiles instead of Q1/Q3 for winsorization/trimming. Requires percentiles to be computed in the stats CSV. |  |
| &nbsp;`--pct-thresholds`&nbsp; | string | Comma-separated percentile pair (e.g., "10,90") to use for winsorization/trimming when --use-percentiles is set. Both values must be between 0 and 100, and lower < upper. | `5,95` |
| &nbsp;`--xsd-gdate-scan`&nbsp; | string | Gregorian XSD date type detection mode. "quick": Fast detection using min/max values. Produces types with ?? suffix (less confident). "thorough": Comprehensive detection checking all percentile values. Slower but ensures all values match the pattern. Produces types with ? suffix (more confident). | `quick` |

<a name="bivariate-statistics-options"></a>

## Bivariate Statistics Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-B,`<br>`--bivariate`&nbsp; | flag | Enable bivariate statistics computation. Requires indexed CSV file (index will be auto-created if missing). Computes pairwise correlations, covariances, mutual information, and normalized mutual information between columns. The bivariate statistics |  |
| &nbsp;`-S,`<br>`--bivariate-stats`&nbsp; | string | Comma-separated list of bivariate statistics to compute. Options: pearson, spearman, kendall, covariance, mi (mutual information), nmi (normalized mutual information) Use "all" to compute all statistics or "fast" to compute only pearson & covariance, which is much faster as it doesn't require storing all values and uses streaming algorithms. | `fast` |
| &nbsp;`-C,`<br>`--cardinality-threshold`&nbsp; | string | Skip mutual information computation for field pairs where either field has cardinality exceeding this threshold. Helps avoid expensive computations for high-cardinality fields. | `1000000` |
| &nbsp;`-J,`<br>`--join-inputs`&nbsp; | string | Additional datasets to join. Comma-separated list of CSV files to join with the primary input. e.g.: --join-inputs customers.csv,products.csv |  |
| &nbsp;`-K,`<br>`--join-keys`&nbsp; | string | Join keys for each dataset. Comma-separated list of join key column names, one per dataset. Must specify same number of keys as datasets (primary + addl). e.g.: --join-keys customer_id,customer_id,product_id |  |
| &nbsp;`-T,`<br>`--join-type`&nbsp; | string | Join type when using --join-inputs. Valid values: inner, left, right, full | `inner` |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars when computing bivariate statistics. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--force`&nbsp; | flag | Force recomputing stats even if valid precomputed stats cache exists. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. This works only when the given CSV has an index. Note that a file handle is opened for each job. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of overwriting the stats CSV file. |  |

---
**Source:** [`src/cmd/moarstats.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/moarstats.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
