# moarstats

> Add up to an additional 73 statistical measures, including extended outlier, robust & bivariate statistics to an existing stats CSV file. ([example](../moarstats/NYC_311_SR_2010-2020-sample-1M.stats.csv)).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/moarstats.rs](https://github.com/dathere/qsv/blob/master/src/cmd/moarstats.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🏎️](TableOfContents.md#legend "multithreaded and/or faster when an index (📇) is available.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Moarstats Options](#moarstats-options) | [Bivariate Statistics Options](#bivariate-statistics-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Add dozens of additional statistics, including extended outlier, robust & bivariate
statistics to an existing stats CSV file. It also maps the field type to the most specific
W3C XML Schema Definition (XSD) datatype (<https://www.w3.org/TR/xmlschema-2/>).

> [!IMPORTANT]
> The `moarstats` command is designed to be run AFTER the `stats` command, as it relies
> on the baseline statistics computed by `stats` to calculate "moar" statistics.

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" (<https://www.dictionary.com/culture/slang/moar>) statistics that can be
derived from existing stats columns and by scanning the original CSV file.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following 40 additional univariate statistics:  
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
11. Trimean: (Q1 + 2*median + Q3) / 4
Tukey's trimean - a robust estimator of central tendency combining the median
with the midhinge. More robust than mean, more efficient than median alone.
<https://en.wikipedia.org/wiki/Trimean>
12. Midhinge: (Q1 + Q3) / 2
Midpoint of the middle 50% of data. A robust central tendency measure
that complements the mean and median.
<https://en.wikipedia.org/wiki/Midhinge>
13. Robust CV: MAD / |median|
Robust Coefficient of Variation using MAD and the magnitude of the median.
Always non-negative. Resistant to outliers, useful for comparing variability.
<https://en.wikipedia.org/wiki/Robust_measures_of_scale>
14. Normalized MAD: MAD / 0.6745 (≈ 1.4826 * MAD)
Robust estimate of the standard deviation, consistent with stddev for normal data.
<https://en.wikipedia.org/wiki/Median_absolute_deviation>
15. Normalized IQR: IQR / 1.349
Robust estimate of the standard deviation, consistent with stddev for normal data.
16. Robust Z-Score of Min: 0.6745 * (min - median) / MAD
Iglewicz-Hoaglin modified z-score of the minimum.
Values beyond ±3.5 are commonly flagged as potential outliers.
17. Robust Z-Score of Max: 0.6745 * (max - median) / MAD
Iglewicz-Hoaglin modified z-score of the maximum.
18. Kelly Skewness: (P90 + P10 - 2*median) / (P90 - P10)
Percentile-based skewness. Less sensitive to extreme tails than moment skewness,
and uses more of the distribution than quartile skewness.
Requires the 10th & 90th percentiles (included in the default percentile list).
19. P90/P10 Ratio: P90 / P10
Common spread/inequality ratio. Only computed when P10 > 0.
20. Zero Share: n_zero / (n_zero + n_positive + n_negative)
Share of non-null numeric values that are exactly zero (zero-inflation).
21. Berger-Parker Dominance: mode_occurrences / row count
Share of rows taken by the most frequent value (NULL counts as a value, as in mode).
Works for all field types. Not meaningful on weighted stats.
<https://en.wikipedia.org/wiki/Diversity_index#Berger%E2%80%93Parker_index>
22. Moment Skewness: adjusted Fisher-Pearson standardized moment coefficient (G1).
Moment-based counterpart to the quantile-based `skewness` from stats.
Positive values indicate right skew, negative values indicate left skew.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Skewness#Sample_skewness>
23. Kurtosis: Measures the "tailedness" of the distribution (sample excess kurtosis, G2).
Positive values indicate heavy tails, negative values indicate light tails.
Values near 0 indicate a normal distribution.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Kurtosis>
24. Bimodality Coefficient: Measures whether a distribution has two modes (peaks) or is unimodal.
BC > 0.555 (the value for a uniform distribution) suggests bimodal/multimodal.
Computed as (G1² + 1) / (G2 + 3(n-1)² / ((n-2)(n-3))), using moment skewness and kurtosis.
Heavily skewed unimodal data can also exceed 0.555.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Bimodality>
25. Jarque-Bera Test: (n/6) * (S² + K²/4)
Standard test for normality, where S and K are the (biased) moment skewness and
excess kurtosis.
Also computes jarque_bera_pvalue (from chi-squared distribution with 2 df).
Low p-values (< 0.05) indicate the data is NOT normally distributed.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Jarque%E2%80%93Bera_test>
26. Gini Coefficient: Measures inequality/dispersion in the distribution.
Values range from 0 (perfect equality) to 1 (maximum inequality).
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Gini_coefficient>
27. Atkinson Index: Measures inequality in the distribution with a sensitivity parameter.
Values range from 0 (perfect equality) to 1 (maximum inequality).
The Atkinson Index is a more general form of the Gini coefficient that allows for
different sensitivity to inequality. Sensitivity is configurable via --epsilon.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Atkinson_index>
28. Theil Index: (1/n) * Σ((x_i / mean) * ln(x_i / mean))
Measures inequality/concentration. Unlike Gini, it is decomposable into
within-group and between-group components. Only computed for positive values.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Theil_index>
29. Mean Absolute Deviation (from mean): (1/n) * Σ|x_i - mean|
Average absolute distance from the mean. Different from MAD (which uses median).
Less robust but more statistically efficient than MAD.
Requires --advanced flag.
30. Hoover Index: Σ|x_i - mean| / (2 * Σx_i)
Share of the total that would have to be redistributed to reach equality (0 to 1).
Only computed for non-negative numeric data. Requires --advanced flag.
<https://en.wikipedia.org/wiki/Hoover_index>
31. L-CV: λ2 / λ1
Coefficient of L-variation - a robust, bounded analogue of the CV based on
L-moments (linear combinations of order statistics). Only computed for numeric data
with a positive mean. Requires --advanced flag.
<https://en.wikipedia.org/wiki/L-moment>
32. L-Skewness: τ3 = λ3 / λ2
Robust analogue of skewness, bounded in [-1, 1]; exists whenever the mean does.
Requires --advanced flag.
33. L-Kurtosis: τ4 = λ4 / λ2
Robust analogue of kurtosis, bounded in [-1/4, 1]. About 0.1226 for a normal
distribution. Requires --advanced flag.
34. Lag-1 Autocorrelation: Σ(x_t - mean)(x_{t+1} - mean) / Σ(x_t - mean)²
Correlation between consecutive non-null values in file order. Values near 1 indicate
a trend, drift or clustered/sorted data; near 0 no serial dependence; negative values
alternation. Requires --advanced flag.
<https://en.wikipedia.org/wiki/Autocorrelation>
35. Benford MAD: mean absolute deviation between the first-significant-digit
proportions and Benford's law. Nigrini's thresholds: < 0.006 close conformity,
< 0.012 acceptable, < 0.015 marginal, otherwise nonconformity - a data quality or
fabrication signal. Only computed for numeric data with at least 100 non-zero values
spanning at least two orders of magnitude. Requires --advanced flag.
<https://en.wikipedia.org/wiki/Benford%27s_law>
36. Shannon Entropy: Measures the information content/uncertainty in the distribution.
Higher values indicate more diversity, lower values indicate more concentration.
Values range from 0 (all values identical) to log2(n) where n is the number of unique values.
Requires --advanced flag.
<https://en.wikipedia.org/wiki/Entropy_(information_theory)>
37. Normalized Entropy: Normalized version of Shannon Entropy scaled to [0, 1].
Values range from 0 (all values identical) to 1 (all values equally distributed).
Computed as shannon_entropy / log2(cardinality).
Requires shannon_entropy (from --advanced flag) and cardinality (from base stats).
38. Simpson's Diversity Index: 1 - Σ(p_i²)
Probability that two randomly chosen values are different.
Ranges from 0 (all identical) to 1 (all unique). More intuitive than entropy.
Requires --advanced flag (computed alongside entropy from frequency data).
<https://en.wikipedia.org/wiki/Diversity_index#Simpson_index>
39. Winsorized Mean: Replaces values below/above thresholds with threshold values, then computes mean.
All values are included in the calculation, but extreme values are capped at thresholds.
<https://en.wikipedia.org/wiki/Winsorized_mean>
Also computes (<PCT> is the threshold suffix of the mean column, e.g. 25pct or 5pct):  
winsorized_stddev_<PCT>, winsorized_variance_<PCT>, winsorized_cv_<PCT>,
winsorized_range_<PCT>, and winsorized_<PCT>_stddev_ratio
(winsorized stddev / overall stddev). Note the ratio column interpolates <PCT>
before _stddev_ratio, unlike the others.
40. Trimmed Mean: Excludes values outside thresholds, then computes mean.
Only values within thresholds are included in the calculation.
<https://en.wikipedia.org/wiki/Truncated_mean>
Also computes (<PCT> is the threshold suffix of the mean column, e.g. 25pct or 5pct):  
trimmed_stddev_<PCT>, trimmed_variance_<PCT>, trimmed_cv_<PCT>, trimmed_range_<PCT>,
and trimmed_<PCT>_stddev_ratio (trimmed stddev / overall stddev). Note the ratio
column interpolates <PCT> before _stddev_ratio, unlike the others.
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

```text
inner fences at Q1/Q3 ± 1.5*IQR, outer fences at Q1/Q3 ± 3.0*IQR.
```

These univariate statistics are only computed for numeric and date/datetime columns
where the required base univariate statistics (mean, median, stddev, etc.) are available.
Univariate outlier statistics additionally require that quartiles (and thus fences) were
computed when generating the stats CSV.
Winsorized/trimmed means require either Q1/Q3 or percentiles to be available.
Kurtosis, Gini & Atkinson Index require reading the original CSV file to collect
all values for computation.

BIVARIATE STATISTICS:  

The `moarstats` command also computes the following 9 bivariate statistics:  
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
7. Theil's U (uncertainty coefficient)
Directed measure of how much knowing one field reduces uncertainty about the other.
Asymmetric, so two columns are emitted: u_field2_given_field1 and u_field1_given_field2.
Values range from 0 (no reduction) to 1 (fully determined).
Selected with `u` in --bivariate-stats (or via "all").
<https://en.wikipedia.org/wiki/Uncertainty_coefficient>
8. Cramér's V: sqrt(χ² / (n * (min(r, c) - 1)))
Chi-squared based association between two fields of any type, from the same
joint frequency table as mutual information. Ranges from 0 (independent) to 1.
Selected with `cramersv` in --bivariate-stats (or via "all").
<https://en.wikipedia.org/wiki/Cram%C3%A9r%27s_V>
9. Linear Regression: ordinary least-squares fit of field2 on field1
Emits regression_slope, regression_intercept and r_squared (the coefficient of
determination) for numeric/date field pairs, from the same streaming state as
Pearson's correlation. Selected with `regression` in --bivariate-stats (or via "all").
<https://en.wikipedia.org/wiki/Simple_linear_regression>

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

Non-finite numeric tokens ("NaN", "Infinity", "-Infinity", and their case variants) are
excluded from moarstats computations — the parser in moarstats filters them out before they
reach correlation, variance and mean calculations, preventing a single bad cell from silently
poisoning the results. Note that the baseline `stats` command may still count these tokens
as Float observations, so the `type`/`null_count` columns in `<FILESTEM>.stats.csv` are not
affected by this filter.


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

See also <https://github.com/dathere/qsv/wiki/Aggregation-and-Statistics#moarstats>

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
| &nbsp;`‑‑advanced`&nbsp; | flag | Compute Moment Skewness, Kurtosis, Shannon Entropy, Bimodality Coefficient, Jarque-Bera, Gini Coefficient, Atkinson Index, Theil Index, Mean Absolute Deviation, Hoover Index, L-moment ratios, Lag-1 Autocorrelation, Benford MAD and Simpson's Diversity Index. These advanced statistics computations require reading the original CSV file to collect all values for computation and are computationally expensive. Further, Entropy computation requires the frequency command to be run with --limit 0 to collect all frequencies. An index will be auto-created for the original CSV file if it doesn't already exist to enable parallel processing. |  |
| &nbsp;`‑e,`<br>`‑‑epsilon`&nbsp; | float | The Atkinson Index Inequality Aversion parameter. Epsilon controls the sensitivity of the Atkinson Index to inequality. The higher the epsilon, the more sensitive the index is to inequality. Typical values are 0.5 (standard in economic research), 1.0 (natural boundary), or 2.0 (useful for poverty analysis). | `1.0` |
| &nbsp;`‑‑stats‑options`&nbsp; | string | Options to pass to the stats command if baseline stats need to be generated. The options are passed as a single string that will be split by whitespace. | `--infer-dates --infer-boolean --cardinality --mode --mad --quartiles --percentiles --force --stats-jsonl` |
| &nbsp;`‑‑round`&nbsp; | integer | Round statistics to <n> decimal places. Rounding follows Midpoint Nearest Even (Bankers Rounding) rule. | `4` |
| &nbsp;`‑‑use‑percentiles`&nbsp; | flag | Use percentiles instead of Q1/Q3 for winsorization/trimming. Requires percentiles to be computed in the stats CSV. |  |
| &nbsp;`‑‑pct‑thresholds`&nbsp; | string | Comma-separated percentile pair (e.g., "10,90") to use for winsorization/trimming when --use-percentiles is set. Both values must truncate to whole percentiles between 1 and 100, and lower < upper. The thresholds are automatically merged into the --percentile-list of the stats run, and an existing stats CSV computed with a percentile list that lacks them is recomputed. | `5,95` |
| &nbsp;`‑‑xsd‑gdate‑scan`&nbsp; | string | Gregorian XSD date type detection mode. "quick": Fast detection using min/max values. Produces types with ?? suffix (less confident). "thorough": Comprehensive detection checking all percentile values. Slower but ensures all values match the pattern. Produces types with ? suffix (more confident). | `quick` |

<a name="bivariate-statistics-options"></a>

## Bivariate Statistics Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑B,`<br>`‑‑bivariate`&nbsp; | flag | Enable bivariate statistics computation. Requires indexed CSV file (index will be auto-created if missing). Computes pairwise correlations, covariances, mutual information, and normalized mutual information between columns. The bivariate statistics |  |
| &nbsp;`‑S,`<br>`‑‑bivariate‑stats`&nbsp; | string | Comma-separated list of bivariate statistics to compute. Options: pearson, spearman, kendall, covariance, mi (mutual information), nmi (normalized mutual information), u (Theil's directed uncertainty coefficient; emits u_field2_given_field1 and u_field1_given_field2), cramersv (Cramér's V) and regression (slope, intercept & r_squared). Use "all" to compute all statistics or "fast" to compute only pearson & covariance, which is much faster as it doesn't require storing all values and uses streaming algorithms. | `fast` |
| &nbsp;`‑C,`<br>`‑‑cardinality‑threshold`&nbsp; | integer | Skip mutual information (mi/nmi/u) and cramersv for field pairs where either field's cardinality exceeds this threshold. Such pairs also skip building their joint-frequency table, which is the dominant memory cost of --bivariate-stats all. Defaults to half the row count, floored at 1000, so it stays inert on small inputs and scales with large ones. Mutual information between near-unique columns saturates at log(n) and is noise regardless of how efficiently it is computed. |  |
| &nbsp;`‑‑bivariate‑batch`&nbsp; | integer | Process at most <n> field pairs per pass over the input, bounding peak memory at the cost of extra passes. Peak memory is otherwise O(columns^2) regardless of row count - a 160-column, 100k-row (60 MB) input needs ~21 GiB with mi/nmi/u enabled. Extra passes are cheap, so prefer the largest <n> that fits. Only applies to indexed input with >= 10,000 rows. Set to 0 to process all pairs in one pass. | `0` |
| &nbsp;`‑J,`<br>`‑‑join‑inputs`&nbsp; | string | Additional datasets to join. Comma-separated list of CSV files to join with the primary input. e.g.: --join-inputs customers.csv,products.csv |  |
| &nbsp;`‑K,`<br>`‑‑join‑keys`&nbsp; | string | Join keys for each dataset. Comma-separated list of join key column names, one per dataset. Must specify same number of keys as datasets (primary + addl). e.g.: --join-keys customer_id,customer_id,product_id |  |
| &nbsp;`‑T,`<br>`‑‑join‑type`&nbsp; | string | Join type when using --join-inputs. Valid values: inner, left, right, full | `inner` |
| &nbsp;`‑p,`<br>`‑‑progressbar`&nbsp; | flag | Show progress bars when computing bivariate statistics. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑force`&nbsp; | flag | Force recomputing stats even if valid precomputed stats cache exists. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | The number of jobs to run in parallel. This works only when the given CSV has an index. Note that a file handle is opened for each job. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of overwriting the stats CSV file. |  |

---
**Source:** [`src/cmd/moarstats.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/moarstats.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
