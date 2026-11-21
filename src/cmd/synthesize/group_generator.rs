//! Joint, multi-column generation for relationship groups.
//!
//! A `GroupGenerator` emits a *tuple* of values per row so that several columns
//! are filled coherently, preserving inter-column structure that the
//! independent per-column `ColumnGenerator`s cannot. Ungrouped columns keep
//! using `ColumnGenerator`; only columns named by a declared `relationship` are
//! routed through a `GroupGenerator` (see `relationships.rs`).

use rand::{RngExt, rngs::StdRng};
use serde::{Deserialize, Serialize};

use super::generator::{draw_null, format_number, round_at_least};

/// The value domain an `ordered` relationship group is generated in. Date
/// values are carried as whole days since the UNIX epoch, datetimes as seconds
/// — matching `generator::parse_epoch`. `Numeric` covers both Integer and Float
/// members; per-member integer-vs-float formatting is tracked separately (see
/// `Ordered::is_int`) so a mixed group never drifts an Integer column to a
/// float string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum OrderedKind {
    Numeric,
    Date,
    DateTime,
}

/// A joint, multi-column generator for one relationship group.
#[derive(Serialize, Deserialize)]
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
    /// anchor (`member_cols[0]`) is drawn from its own quantile distribution;
    /// each later member is the previous member plus a non-negative gap drawn
    /// from the gap distribution learned from the source. Because every gap is
    /// >= 0, the declared order holds in every output row.
    Ordered {
        /// Output column index per member, in member (low-to-high) order;
        /// `member_cols[0]` is the anchor.
        member_cols:      Vec<usize>,
        kind:             OrderedKind,
        /// Per-member flag (`Numeric` groups only): format the member as an
        /// integer when true, a float when false. Lets a mixed Integer/Float
        /// group keep each column's declared type. Ignored for Date/DateTime.
        is_int:           Vec<bool>,
        /// Per-member decimal places a Float member is rounded to (its source
        /// `max_precision`); `None` for Integer, unrounded and date members.
        decimals:         Vec<Option<u32>>,
        /// describegpt-inferred chrono strftime format for date members.
        date_format:      Option<String>,
        /// Anchor quantile buckets, in the numeric domain.
        anchor_buckets:   Vec<(f64, f64)>,
        /// Consecutive-gap quantile buckets; `gap_buckets[i]` is the gap from
        /// `member_cols[i]` to `member_cols[i + 1]`, clamped to >= 0. Length is
        /// `member_cols.len() - 1`.
        gap_buckets:      Vec<Vec<(f64, f64)>>,
        /// Per blank pattern, per-member `[lo, hi]` in the same domain as the
        /// buckets, with each `hi` capped by every later member the pattern
        /// leaves filled; a blank member's range is otherwise unbounded. Each
        /// follower is clamped into its range (then kept >= its predecessor,
        /// which the cap keeps in range), so anchor + gap can't reach a
        /// follower's real extremes once `--min-count` has trimmed them.
        pattern_ranges:   Vec<Vec<(f64, f64)>>,
        /// Share of source rows where every member is blank; such a row is
        /// emitted blank without drawing any value.
        null_ratio:       f64,
        /// Which members are blank in the other rows, as seen in the source:
        /// `blank_patterns[p][k]` is true when member `k` is blank. The chain
        /// is drawn whole, its blank members unprinted, so the members left
        /// filled stay in order.
        blank_patterns:   Vec<Vec<bool>>,
        /// Normalized cumulative weights of `blank_patterns`, ascending, last
        /// element == 1.0. A single pattern takes no draw.
        blank_cumulative: Vec<f64>,
    },
    /// Gaussian-copula generation for a set of correlated numeric columns. Each
    /// column keeps its own quartile-bucket marginal exactly; the copula only
    /// couples them, reproducing the source's rank (Spearman) correlation.
    ///
    /// With categorical members, each row first draws a stratum (one source
    /// combination of their values) by its share of the source rows, then
    /// draws the numeric members from that stratum's own buckets, so each
    /// category keeps its own numeric distributions.
    Correlated {
        /// Output column index per member: the numeric members, in member
        /// declared order, then the categorical (stratum) members.
        col_indices:       Vec<usize>,
        /// Row-normalized lower-triangular Cholesky factor of the correlation
        /// matrix of the numeric members (within strata, when there are
        /// any); `cholesky_l[i][j]` is used for `j <= i`.
        cholesky_l:        Vec<Vec<f64>>,
        /// Per-member quantile buckets — a piecewise-uniform inverse CDF that
        /// maps a uniform draw back onto the column's own distribution.
        /// Empty when the group has strata, which carry their own.
        marginals:         Vec<Vec<(f64, f64)>>,
        /// Per numeric member flag: round the emitted value to an integer.
        is_int:            Vec<bool>,
        /// Per numeric member decimal places a Float member is rounded to;
        /// `None` for Integer and unrounded members.
        decimals:          Vec<Option<u32>>,
        /// Per-member null ratio. Each member draws null independently, so its
        /// marginal null ratio is preserved and nulls do not all co-occur.
        /// Empty when the group has strata, which carry their own.
        null_ratios:       Vec<f64>,
        /// The source combinations of the categorical members, sorted.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        strata:            Vec<Stratum>,
        /// Normalized cumulative weights of `strata`, ascending, last element
        /// == 1.0.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        strata_cumulative: Vec<f64>,
    },
}

/// One combination of a `Correlated` group's categorical members, with the
/// numeric members' distributions in the source rows holding it.
#[derive(Serialize, Deserialize)]
pub(crate) struct Stratum {
    /// The categorical members' values, in `col_indices` order.
    pub(crate) values:      Vec<String>,
    /// Per numeric member quantile buckets within this stratum.
    pub(crate) marginals:   Vec<Vec<(f64, f64)>>,
    /// Per numeric member share of this stratum's rows left blank.
    pub(crate) null_ratios: Vec<f64>,
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
                decimals,
                date_format,
                anchor_buckets,
                gap_buckets,
                pattern_ranges,
                null_ratio,
                blank_patterns,
                blank_cumulative,
            } => {
                // An all-blank row is drawn first and draws no values, so a
                // chain whose members are blank in the same rows takes the
                // same draws as one whose anchor's blanks blank it whole.
                if draw_null(*null_ratio, rng) {
                    return member_cols.iter().map(|&c| (c, String::new())).collect();
                }
                // A chain with one pattern takes no draw, so one whose members
                // are blank in the same rows keeps the draws it had when a
                // blank anchor blanked the chain.
                let p = if blank_patterns.len() > 1 {
                    let r = rng.random_range(0.0..1.0);
                    blank_cumulative
                        .partition_point(|&c| c < r)
                        .min(blank_patterns.len() - 1)
                } else {
                    0
                };
                let (blanks, ranges) = (&blank_patterns[p], &pattern_ranges[p]);
                let text = |value: f64, i: usize| {
                    if blanks[i] {
                        String::new()
                    } else {
                        format_ordered(value, *kind, is_int[i], decimals[i], date_format.as_ref())
                    }
                };
                // The anchor's upper bound is capped by every filled
                // follower's (see `resolve_ordered`), so the chain can always
                // stay in range.
                let mut value = sample_bucket(anchor_buckets, rng).min(ranges[0].1);
                // Chain from the value actually emitted: a member is printed
                // rounded to its grid (whole numbers for Integer, its source
                // decimals for Float), so the next member must build on the
                // rounded value or it can print below it. A follower rounds no
                // lower than its predecessor, or a coarser grid can print it
                // below a finer one (1.24 at one decimal is 1.2). That stays
                // within range: the predecessor's cap is at most this
                // member's grid-floored cap. A blank member prints nothing,
                // so it isn't rounded: its grid must not move the members
                // after it.
                let quantize = |v: f64, i: usize, floor: f64| {
                    if blanks[i] {
                        v
                    } else {
                        round_at_least(v, floor, member_grid(*kind, is_int[i], decimals[i]))
                    }
                };
                value = quantize(value, 0, f64::NEG_INFINITY);
                let mut out = Vec::with_capacity(member_cols.len());
                out.push((member_cols[0], text(value, 0)));
                for (i, gaps) in gap_buckets.iter().enumerate() {
                    // Gap buckets are clamped to >= 0, and the clamp into the
                    // follower's range is floored at the predecessor, so
                    // `value` stays non-decreasing along the chain.
                    let prev = value;
                    let (lo, hi) = ranges[i + 1];
                    value = quantize(
                        (value + sample_bucket(gaps, rng)).clamp(lo, hi).max(prev),
                        i + 1,
                        prev,
                    );
                    out.push((member_cols[i + 1], text(value, i + 1)));
                }
                out
            },

            GroupGenerator::Correlated {
                col_indices,
                cholesky_l,
                marginals,
                is_int,
                decimals,
                null_ratios,
                strata,
                strata_cumulative,
            } => {
                // A group with no categorical members draws only the copula,
                // as it always has.
                let stratum = (!strata.is_empty()).then(|| {
                    let r = rng.random_range(0.0..1.0);
                    &strata[strata_cumulative
                        .partition_point(|&c| c < r)
                        .min(strata.len() - 1)]
                });
                let (marginals, null_ratios) =
                    stratum.map_or((marginals, null_ratios), |s| (&s.marginals, &s.null_ratios));
                let k = cholesky_l.len();
                // k iid standard normals.
                let z: Vec<f64> = (0..k).map(|_| standard_normal(rng)).collect();
                // x = L z gives correlated standard normals; mapping each
                // through Phi yields correlated uniforms, then each column's
                // own quantile function restores its exact marginal.
                let mut out = Vec::with_capacity(k);
                for i in 0..k {
                    let mut x = 0.0;
                    for (j, &z_j) in z.iter().enumerate().take(i + 1) {
                        x = cholesky_l[i][j].mul_add(z_j, x);
                    }
                    let value = quantile_value(&marginals[i], normal_cdf(x));
                    // Each member draws null independently (after the copula
                    // values are computed, so the RNG sequence stays fixed) —
                    // its marginal null ratio is preserved and nulls do not
                    // all co-occur.
                    let text = if draw_null(null_ratios[i], rng) {
                        String::new()
                    } else {
                        format_number(value, is_int[i], decimals[i])
                    };
                    out.push((col_indices[i], text));
                }
                if let Some(s) = stratum {
                    out.extend(
                        col_indices[k..]
                            .iter()
                            .copied()
                            .zip(s.values.iter().cloned()),
                    );
                }
                out
            },
        }
    }
}

/// Uniformly draw a value from a randomly chosen quantile bucket.
fn sample_bucket(buckets: &[(f64, f64)], rng: &mut StdRng) -> f64 {
    let (lo, hi) = buckets[rng.random_range(0..buckets.len())];
    if lo < hi {
        rng.random_range(lo..hi)
    } else {
        lo
    }
}

/// The decimal grid an ordered member's emitted value lies on: whole numbers
/// for an Integer member, its source decimals for a Float one, none (`None`)
/// for date members, which `format_ordered` rounds itself.
pub(crate) const fn member_grid(
    kind: OrderedKind,
    is_int: bool,
    decimals: Option<u32>,
) -> Option<u32> {
    match kind {
        OrderedKind::Numeric if is_int => Some(0),
        OrderedKind::Numeric => decimals,
        OrderedKind::Date | OrderedKind::DateTime => None,
    }
}

/// Format a numeric-domain value back into the member column's textual form.
/// `is_int` and `decimals` select integer vs float rendering for a `Numeric`
/// member; they are ignored for Date/DateTime.
fn format_ordered(
    value: f64,
    kind: OrderedKind,
    is_int: bool,
    decimals: Option<u32>,
    date_format: Option<&String>,
) -> String {
    #[allow(clippy::cast_possible_truncation)]
    match kind {
        OrderedKind::Numeric => format_number(value, is_int, decimals),
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
// Not a midpoint: this is the CDF identity Phi(x) = (1 + erf(x/sqrt(2)))/2.
// clippy::manual_midpoint pattern-matches `0.5 * (a + b)` but f64::midpoint
// would obscure the formula and carries no precision benefit here.
#[allow(clippy::manual_midpoint)]
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Map a uniform `u` in `[0, 1)` back onto a column's distribution, treating
/// its quantile buckets as a piecewise-uniform inverse CDF. Each of the `n`
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
