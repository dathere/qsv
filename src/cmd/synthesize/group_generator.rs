//! Joint, multi-column generation for relationship groups.
//!
//! A `GroupGenerator` emits a *tuple* of values per row so that several columns
//! are filled coherently, preserving inter-column structure that the
//! independent per-column `ColumnGenerator`s cannot. Ungrouped columns keep
//! using `ColumnGenerator`; only columns named by a declared `relationship` are
//! routed through a `GroupGenerator` (see `relationships.rs`).

use rand::{RngExt, rngs::StdRng};

use super::generator::draw_null;

/// The value domain an `ordered` relationship group is generated in. Date
/// values are carried as whole days since the UNIX epoch, datetimes as seconds
/// — matching `generator::parse_epoch`. `Numeric` covers both Integer and Float
/// members; per-member integer-vs-float formatting is tracked separately (see
/// `Ordered::is_int`) so a mixed group never drifts an Integer column to a
/// float string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OrderedKind {
    Numeric,
    Date,
    DateTime,
}

/// A joint, multi-column generator for one relationship group.
pub(crate) enum GroupGenerator {
    /// Frequency-weighted sampling of whole observed tuples. Every emitted
    /// combination is one that actually occurs in the source, so functional
    /// dependencies (city → state → zip, category → subcategory, …) are
    /// preserved exactly. Nulls are reproduced verbatim as part of the tuple.
    Joint {
        /// Output column indices this group fills, ascending. Tuple values are
        /// stored in the same order.
        col_indices: Vec<usize>,
        /// Distinct source tuples; `tuples[t][k]` is the value for
        /// `col_indices[k]`. Guaranteed non-empty by the builder.
        tuples:      Vec<Vec<String>>,
        /// Normalized cumulative weights, ascending, last element == 1.0.
        cumulative:  Vec<f64>,
    },
    /// Anchor + learned-gap generation for a monotonic chain of columns. The
    /// anchor (`member_cols[0]`) is drawn from its own quartile distribution;
    /// each later member is the previous member plus a non-negative gap drawn
    /// from the gap distribution learned from the source. Because every gap is
    /// >= 0, the declared order holds in every output row.
    Ordered {
        /// Output column index per member, in member (low-to-high) order;
        /// `member_cols[0]` is the anchor.
        member_cols:    Vec<usize>,
        kind:           OrderedKind,
        /// Per-member flag (`Numeric` groups only): format the member as an
        /// integer when true, a float when false. Lets a mixed Integer/Float
        /// group keep each column's declared type. Ignored for Date/DateTime.
        is_int:         Vec<bool>,
        /// describegpt-inferred chrono strftime format for date members.
        date_format:    Option<String>,
        /// Anchor quartile buckets, in the numeric domain.
        anchor_buckets: Vec<(f64, f64)>,
        /// Consecutive-gap quartile buckets; `gap_buckets[i]` is the gap from
        /// `member_cols[i]` to `member_cols[i + 1]`, clamped to >= 0. Length is
        /// `member_cols.len() - 1`.
        gap_buckets:    Vec<Vec<(f64, f64)>>,
        /// Null ratio of the anchor column — when the anchor draws null, the
        /// whole chain is emitted null.
        null_ratio:     f64,
    },
    /// Gaussian-copula generation for a set of correlated numeric columns. Each
    /// column keeps its own quartile-bucket marginal exactly; the copula only
    /// couples them, reproducing the source's rank (Spearman) correlation.
    Correlated {
        /// Output column index per member, in member declared order.
        col_indices: Vec<usize>,
        /// Row-normalized lower-triangular Cholesky factor of the correlation
        /// matrix; `cholesky_l[i][j]` is used for `j <= i`.
        cholesky_l:  Vec<Vec<f64>>,
        /// Per-member quartile buckets — a piecewise-uniform inverse CDF that
        /// maps a uniform draw back onto the column's own distribution.
        marginals:   Vec<Vec<(f64, f64)>>,
        /// Per-member flag: round the emitted value to an integer.
        is_int:      Vec<bool>,
        /// Per-member null ratio. Each member draws null independently, so its
        /// marginal null ratio is preserved and nulls do not all co-occur.
        null_ratios: Vec<f64>,
    },
}

impl GroupGenerator {
    /// The output column indices this group is responsible for.
    pub(crate) fn col_indices(&self) -> &[usize] {
        match self {
            GroupGenerator::Ordered { member_cols, .. } => member_cols,
            GroupGenerator::Joint { col_indices, .. }
            | GroupGenerator::Correlated { col_indices, .. } => col_indices,
        }
    }

    /// Emit one synthetic value per member column. Returns `(column index,
    /// value)` pairs so the caller can place each value in the right output
    /// slot regardless of the order groups are scheduled in.
    pub(crate) fn next(&self, rng: &mut StdRng) -> Vec<(usize, String)> {
        match self {
            GroupGenerator::Joint {
                col_indices,
                tuples,
                cumulative,
            } => {
                // Same weighted draw as `ColumnGenerator::FrequencyWeighted`,
                // but the indexed vector holds whole tuples, so an entire row
                // slice is reproduced coherently.
                let r = rng.random_range(0.0..1.0);
                let idx = cumulative.partition_point(|&c| c < r).min(tuples.len() - 1);
                col_indices
                    .iter()
                    .copied()
                    .zip(tuples[idx].iter().cloned())
                    .collect()
            },

            GroupGenerator::Ordered {
                member_cols,
                kind,
                is_int,
                date_format,
                anchor_buckets,
                gap_buckets,
                null_ratio,
            } => {
                // A null anchor nulls the whole chain — a monotonic chain
                // `m[i] = m[i-1] + gap` cannot have a non-null member sitting
                // after a null predecessor.
                if draw_null(*null_ratio, rng) {
                    return member_cols.iter().map(|&c| (c, String::new())).collect();
                }
                let mut value = sample_bucket(anchor_buckets, rng);
                let mut out = Vec::with_capacity(member_cols.len());
                out.push((
                    member_cols[0],
                    format_ordered(value, *kind, is_int[0], date_format.as_ref()),
                ));
                for (i, gaps) in gap_buckets.iter().enumerate() {
                    // Gap buckets are clamped to >= 0, so `value` is
                    // non-decreasing along the chain.
                    value += sample_bucket(gaps, rng);
                    out.push((
                        member_cols[i + 1],
                        format_ordered(value, *kind, is_int[i + 1], date_format.as_ref()),
                    ));
                }
                out
            },

            GroupGenerator::Correlated {
                col_indices,
                cholesky_l,
                marginals,
                is_int,
                null_ratios,
            } => {
                let k = col_indices.len();
                // k iid standard normals.
                let z: Vec<f64> = (0..k).map(|_| standard_normal(rng)).collect();
                // x = L z gives correlated standard normals; mapping each
                // through Phi yields correlated uniforms, then each column's
                // own quantile function restores its exact marginal.
                let mut out = Vec::with_capacity(k);
                for i in 0..k {
                    let mut x = 0.0;
                    for (j, &z_j) in z.iter().enumerate().take(i + 1) {
                        x += cholesky_l[i][j] * z_j;
                    }
                    let value = quantile_value(&marginals[i], normal_cdf(x));
                    // Each member draws null independently (after the copula
                    // values are computed, so the RNG sequence stays fixed) —
                    // its marginal null ratio is preserved and nulls do not
                    // all co-occur.
                    let text = if draw_null(null_ratios[i], rng) {
                        String::new()
                    } else if is_int[i] {
                        #[allow(clippy::cast_possible_truncation)]
                        {
                            (value.round() as i64).to_string()
                        }
                    } else {
                        value.to_string()
                    };
                    out.push((col_indices[i], text));
                }
                out
            },
        }
    }
}

/// Uniformly draw a value from a randomly chosen quartile bucket.
fn sample_bucket(buckets: &[(f64, f64)], rng: &mut StdRng) -> f64 {
    let (lo, hi) = buckets[rng.random_range(0..buckets.len())];
    if lo < hi {
        rng.random_range(lo..hi)
    } else {
        lo
    }
}

/// Format a numeric-domain value back into the member column's textual form.
/// `is_int` selects integer vs float rendering for a `Numeric` member; it is
/// ignored for Date/DateTime.
fn format_ordered(
    value: f64,
    kind: OrderedKind,
    is_int: bool,
    date_format: Option<&String>,
) -> String {
    #[allow(clippy::cast_possible_truncation)]
    match kind {
        OrderedKind::Numeric => {
            if is_int {
                (value.round() as i64).to_string()
            } else {
                value.to_string()
            }
        },
        OrderedKind::Date | OrderedKind::DateTime => {
            let is_datetime = kind == OrderedKind::DateTime;
            // Date values are whole days since the epoch; datetimes are seconds.
            let seconds = if is_datetime {
                value.round() as i64
            } else {
                (value.round() as i64) * 86_400
            };
            match chrono::DateTime::from_timestamp(seconds, 0) {
                Some(dt) => match date_format {
                    Some(fmt) => dt.format(fmt.as_str()).to_string(),
                    None if is_datetime => dt.to_rfc3339(),
                    None => dt.format("%Y-%m-%d").to_string(),
                },
                None => String::new(),
            }
        },
    }
}

/// One draw from the standard normal distribution via the Box-Muller transform
/// (cosine variant — same approach as `generator::sample_target_length`, no
/// extra dependency).
fn standard_normal(rng: &mut StdRng) -> f64 {
    // `u1 >= f64::EPSILON` guards `ln(0) = -inf`.
    let u1: f64 = rng.random_range(f64::EPSILON..1.0);
    let u2: f64 = rng.random_range(0.0..1.0);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// The error function, via the Abramowitz & Stegun 7.1.26 polynomial
/// approximation (max absolute error ~1.5e-7 — far tighter than synthesis
/// needs).
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = ((((1.061_405_429 * t - 1.453_152_027) * t + 1.421_413_741) * t - 0.284_496_736)
        * t
        + 0.254_829_592)
        * t;
    sign * (1.0 - poly * (-x * x).exp())
}

/// The standard-normal cumulative distribution function, `Phi(x)`.
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Map a uniform `u` in `[0, 1)` back onto a column's distribution, treating
/// its quartile buckets as a piecewise-uniform inverse CDF. Each of the `n`
/// buckets carries probability `1/n`; within a bucket the mapping is linear.
/// This reproduces exactly the marginal that `ColumnGenerator::NumericQuantile`
/// produces, so the copula only couples columns — it never distorts a marginal.
fn quantile_value(buckets: &[(f64, f64)], u: f64) -> f64 {
    let nbuckets = buckets.len();
    let u = u.clamp(0.0, 1.0 - f64::EPSILON);
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let scaled = u * nbuckets as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let idx = (scaled as usize).min(nbuckets - 1);
    #[allow(clippy::cast_precision_loss)]
    let frac = scaled - idx as f64;
    let (lo, hi) = buckets[idx];
    lo + (hi - lo) * frac
}
