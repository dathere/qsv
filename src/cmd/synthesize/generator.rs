//! The per-column generation model.
//!
//! Each source column is reduced to one `ColumnGenerator` built from its
//! `stats` record, its `frequency` records, and its dictionary `content_type`.
//! `ColumnGenerator::next` then emits one synthetic value per call.
//!
//! Construction precedence (highest first):
//!   1. `FrequencyWeighted` — the column is *fully enumerated* by `frequency` (no "Other" catch-all
//!      bucket, not all-unique). The real value set is emitted with real frequency weights, so
//!      cardinality, repetition, and value identity are reproduced exactly. This wins even over a
//!      faker mapping (a `state` column with 50 enumerated values emits the real 50).
//!   2. `Faker` — the dictionary `content_type` maps to a `fake-rs` faker. Bounded-cardinality
//!      columns sample from a fixed pre-generated pool of distinct fake values (the consistency
//!      mechanism); all-unique columns generate a fresh value per row.
//!   3. Type-based — `NumericQuantile` / `DateQuantile` (quartile-bucketed to reproduce
//!      distribution shape), `Boolean`, or `LoremFallback`.
//!
//! Cross-column correlation is out of scope: every column is generated
//! independently.

use std::collections::HashSet;

use rand::{RngExt, rngs::StdRng};

use super::faker_map::{self, Locale};
use crate::cmd::describegpt::dictionary::{FrequencyRecord, StatsRecord};

/// `qsv frequency`'s default text for null/empty values (`--null-text`).
const NULL_TEXT: &str = "(NULL)";
/// `qsv frequency`'s sentinel for all-unique (ID) columns (`--all-unique-text`).
const ALL_UNIQUE_SENTINEL: &str = "<ALL_UNIQUE>";
/// Upper bound on the size of a pre-generated faker pool, so a tiny-valued
/// faker (e.g. `state_abbr`) with a huge declared cardinality cannot blow up
/// memory or loop excessively.
const CARDINALITY_POOL_CAP: u64 = 100_000;

pub(crate) enum ColumnGenerator {
    /// Sample real values with real frequency weights.
    FrequencyWeighted {
        values:     Vec<String>,
        /// Normalized cumulative weights, ascending, last element == 1.0.
        cumulative: Vec<f64>,
        null_ratio: f64,
    },
    /// Semantic faker. `pool` is `Some` for bounded-cardinality columns (sample
    /// from a fixed set of distinct fake values), `None` for all-unique columns
    /// (generate a fresh value per row). `locale` is stored here (not threaded
    /// through `next()`) so the build-time and per-row locale can never diverge.
    Faker {
        content_type: String,
        pool:         Option<Vec<String>>,
        locale:       Locale,
        null_ratio:   f64,
    },
    /// Numeric column reproduced via quartile buckets.
    NumericQuantile {
        buckets:    Vec<(f64, f64)>,
        is_int:     bool,
        null_ratio: f64,
    },
    /// Date / DateTime column reproduced via quartile buckets over epoch seconds.
    DateQuantile {
        buckets:     Vec<(i64, i64)>,
        is_datetime: bool,
        null_ratio:  f64,
    },
    /// Boolean column — safety net; 2-value columns are normally caught by
    /// `FrequencyWeighted` first.
    Boolean { true_ratio: f64, null_ratio: f64 },
    /// Last-resort text generator for non-faker, non-enumerated string columns.
    /// `locale` is stored so per-row `lorem_sentence` generation uses the same
    /// locale as the rest of the column build.
    LoremFallback { locale: Locale, null_ratio: f64 },
    /// All-null / no-data column — always emits an empty value.
    Empty,
}

impl ColumnGenerator {
    pub(crate) fn build(
        stats: &StatsRecord,
        freqs: &[&FrequencyRecord],
        content_type: &str,
        total_rows: u64,
        requested_rows: u64,
        locale: Locale,
        rng: &mut StdRng,
    ) -> ColumnGenerator {
        let null_ratio = compute_null_ratio(stats.nullcount, total_rows);

        // All-null or no distinct values → nothing to synthesize.
        if stats.cardinality == 0 || (total_rows > 0 && stats.nullcount >= total_rows) {
            return ColumnGenerator::Empty;
        }

        // 1. Fully-enumerated frequency pool wins — reproduces the real value set, weights,
        //    cardinality and repetition structure exactly.
        if let Some(generator) = try_frequency_weighted(freqs, null_ratio) {
            return generator;
        }

        // 2. Semantic faker.
        if faker_map::is_faker_token(content_type) {
            let pool =
                build_faker_pool(content_type, stats.cardinality, requested_rows, locale, rng);
            return ColumnGenerator::Faker {
                content_type: content_type.to_string(),
                pool,
                locale,
                null_ratio,
            };
        }

        // 3. Type-based fallback.
        match stats.r#type.as_str() {
            "Integer" => build_numeric(stats, true, null_ratio),
            "Float" => build_numeric(stats, false, null_ratio),
            "Date" => build_date(stats, false, null_ratio),
            "DateTime" => build_date(stats, true, null_ratio),
            "Boolean" => build_boolean(freqs, null_ratio),
            "NULL" => ColumnGenerator::Empty,
            // "String" and anything unrecognized.
            _ => ColumnGenerator::LoremFallback { locale, null_ratio },
        }
    }

    /// Emit one synthetic value. Locale is taken from the variant (set at
    /// `build()` time) — never threaded through `next()`, so build-time and
    /// per-row locale cannot diverge.
    pub(crate) fn next(&self, rng: &mut StdRng) -> String {
        match self {
            ColumnGenerator::Empty => String::new(),

            ColumnGenerator::FrequencyWeighted {
                values,
                cumulative,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                let r = rng.random_range(0.0..1.0);
                let idx = cumulative.partition_point(|&c| c < r).min(values.len() - 1);
                values[idx].clone()
            },

            ColumnGenerator::Faker {
                content_type,
                pool,
                locale,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                match pool {
                    Some(p) if !p.is_empty() => p[rng.random_range(0..p.len())].clone(),
                    _ => faker_map::content_type_to_value(content_type, *locale, rng)
                        .unwrap_or_default(),
                }
            },

            ColumnGenerator::NumericQuantile {
                buckets,
                is_int,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                let (lo, hi) = buckets[rng.random_range(0..buckets.len())];
                let value = if lo < hi {
                    rng.random_range(lo..hi)
                } else {
                    lo
                };
                if *is_int {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        (value.round() as i64).to_string()
                    }
                } else {
                    value.to_string()
                }
            },

            ColumnGenerator::DateQuantile {
                buckets,
                is_datetime,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                let (lo, hi) = buckets[rng.random_range(0..buckets.len())];
                let unit = if lo < hi {
                    rng.random_range(lo..=hi)
                } else {
                    lo
                };
                // For DateTime: `unit` is epoch seconds, format as RFC 3339.
                // For Date: `unit` is whole days since the UNIX epoch, multiply
                // back to seconds and format as YYYY-MM-DD.
                let seconds = if *is_datetime { unit } else { unit * 86_400 };
                match chrono::DateTime::from_timestamp(seconds, 0) {
                    Some(dt) if *is_datetime => dt.to_rfc3339(),
                    Some(dt) => dt.format("%Y-%m-%d").to_string(),
                    None => String::new(),
                }
            },

            ColumnGenerator::Boolean {
                true_ratio,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                if rng.random_bool(*true_ratio) {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            },

            ColumnGenerator::LoremFallback { locale, null_ratio } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                faker_map::content_type_to_value("lorem_sentence", *locale, rng).unwrap_or_default()
            },
        }
    }
}

/// `nullcount / total_rows`, clamped to `[0.0, 1.0]`.
fn compute_null_ratio(nullcount: u64, total_rows: u64) -> f64 {
    if total_rows == 0 {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let ratio = nullcount as f64 / total_rows as f64;
    ratio.clamp(0.0, 1.0)
}

/// Decide whether this cell should be null, given the column's null ratio.
fn draw_null(null_ratio: f64, rng: &mut StdRng) -> bool {
    if null_ratio <= 0.0 {
        false
    } else if null_ratio >= 1.0 {
        true
    } else {
        rng.random_bool(null_ratio)
    }
}

/// Build a `FrequencyWeighted` generator if the column is fully enumerated by
/// the frequency records: a non-empty set of real values, no rank-0 catch-all
/// "Other" bucket, and not the all-unique sentinel. Returns `None` otherwise so
/// the caller falls through to a faker or type-based generator.
fn try_frequency_weighted(freqs: &[&FrequencyRecord], null_ratio: f64) -> Option<ColumnGenerator> {
    if freqs.is_empty() {
        return None;
    }
    // `qsv frequency` gives rank 0 to both the "Other" catch-all bucket and the
    // `<ALL_UNIQUE>` sentinel. Either one means the column is NOT fully
    // enumerated for our purposes.
    if freqs.iter().any(|f| f.rank == 0.0) {
        return None;
    }

    let mut values = Vec::new();
    let mut weights = Vec::new();
    for f in freqs {
        // Nulls are reproduced via `null_ratio`, not the value pool.
        if f.value == NULL_TEXT || f.value.contains(ALL_UNIQUE_SENTINEL) || f.count == 0 {
            continue;
        }
        values.push(f.value.clone());
        #[allow(clippy::cast_precision_loss)]
        weights.push(f.count as f64);
    }
    if values.is_empty() {
        return None;
    }

    let total: f64 = weights.iter().sum();
    let mut cumulative = Vec::with_capacity(weights.len());
    let mut acc = 0.0;
    for w in &weights {
        acc += w / total;
        cumulative.push(acc);
    }
    // Pin the last element to exactly 1.0 to avoid floating-point drift.
    if let Some(last) = cumulative.last_mut() {
        *last = 1.0;
    }

    Some(ColumnGenerator::FrequencyWeighted {
        values,
        cumulative,
        null_ratio,
    })
}

/// Pre-generate a pool of distinct faker values when the column's cardinality
/// is bounded and smaller than the requested row count. Returns `None` when the
/// column is effectively all-unique (cardinality >= requested rows) or above
/// the pool cap — in those cases the caller generates a fresh value per row.
fn build_faker_pool(
    content_type: &str,
    cardinality: u64,
    requested_rows: u64,
    locale: Locale,
    rng: &mut StdRng,
) -> Option<Vec<String>> {
    if cardinality == 0 || cardinality >= requested_rows || cardinality > CARDINALITY_POOL_CAP {
        return None;
    }

    #[allow(clippy::cast_possible_truncation)]
    let target = cardinality as usize;
    let mut pool = Vec::with_capacity(target);
    let mut seen = HashSet::with_capacity(target);
    // Cap attempts so a faker with a small value space (e.g. `state_abbr`) does
    // not loop forever trying to reach an unreachable distinct count.
    let max_attempts = target.saturating_mul(20).max(1000);

    for _ in 0..max_attempts {
        if pool.len() >= target {
            break;
        }
        let value = faker_map::content_type_to_value(content_type, locale, rng)?;
        if seen.insert(value.clone()) {
            pool.push(value);
        }
    }

    if pool.len() < target {
        log::warn!(
            "synthesize: faker '{content_type}' could only produce {} distinct values (wanted \
             {target}); using the smaller pool",
            pool.len()
        );
    }
    if pool.is_empty() { None } else { Some(pool) }
}

/// Build a quartile-bucketed numeric generator. Falls back to a single
/// `[min, max]` bucket when quartiles are missing/inconsistent, and to `Empty`
/// when there is no usable numeric range at all.
fn build_numeric(stats: &StatsRecord, is_int: bool, null_ratio: f64) -> ColumnGenerator {
    let min = parse_f64(&stats.min);
    let max = parse_f64(&stats.max);
    let (Some(lo), Some(hi)) = (min, max) else {
        return ColumnGenerator::Empty;
    };
    if hi < lo {
        return ColumnGenerator::Empty;
    }

    let q1 = stats.addl_cols.get("q1").and_then(|s| parse_f64(s));
    let q2 = stats.addl_cols.get("q2_median").and_then(|s| parse_f64(s));
    let q3 = stats.addl_cols.get("q3").and_then(|s| parse_f64(s));

    let buckets = match (q1, q2, q3) {
        (Some(a), Some(b), Some(c)) if lo <= a && a <= b && b <= c && c <= hi => {
            vec![(lo, a), (a, b), (b, c), (c, hi)]
        },
        _ => vec![(lo, hi)],
    };

    ColumnGenerator::NumericQuantile {
        buckets,
        is_int,
        null_ratio,
    }
}

/// Build a quartile-bucketed date/datetime generator. Bucket bounds are
/// expressed in *days since the UNIX epoch* for `Date` columns and *seconds
/// since the UNIX epoch* for `DateTime` columns — see `parse_epoch`.
fn build_date(stats: &StatsRecord, is_datetime: bool, null_ratio: f64) -> ColumnGenerator {
    let min = parse_epoch(&stats.min, is_datetime);
    let max = parse_epoch(&stats.max, is_datetime);
    let (Some(lo), Some(hi)) = (min, max) else {
        return ColumnGenerator::Empty;
    };
    if hi < lo {
        return ColumnGenerator::Empty;
    }

    let q1 = stats
        .addl_cols
        .get("q1")
        .and_then(|s| parse_epoch(s, is_datetime));
    let q2 = stats
        .addl_cols
        .get("q2_median")
        .and_then(|s| parse_epoch(s, is_datetime));
    let q3 = stats
        .addl_cols
        .get("q3")
        .and_then(|s| parse_epoch(s, is_datetime));

    let buckets = match (q1, q2, q3) {
        (Some(a), Some(b), Some(c)) if lo <= a && a <= b && b <= c && c <= hi => {
            vec![(lo, a), (a, b), (b, c), (c, hi)]
        },
        _ => vec![(lo, hi)],
    };

    ColumnGenerator::DateQuantile {
        buckets,
        is_datetime,
        null_ratio,
    }
}

/// Build a boolean generator, deriving the true/false ratio from the frequency
/// records when available (defaults to 0.5).
fn build_boolean(freqs: &[&FrequencyRecord], null_ratio: f64) -> ColumnGenerator {
    let mut true_count = 0_u64;
    let mut total = 0_u64;
    for f in freqs {
        if f.value == NULL_TEXT || f.rank == 0.0 {
            continue;
        }
        let normalized = f.value.to_ascii_lowercase();
        if matches!(normalized.as_str(), "true" | "t" | "1" | "yes" | "y") {
            true_count += f.count;
        }
        total += f.count;
    }
    #[allow(clippy::cast_precision_loss)]
    let true_ratio = if total > 0 {
        (true_count as f64 / total as f64).clamp(0.0, 1.0)
    } else {
        0.5
    };
    ColumnGenerator::Boolean {
        true_ratio,
        null_ratio,
    }
}

/// Parse a numeric stats value; empty string and non-finite (NaN/±∞) values
/// return `None`. Non-finite endpoints would make `rng.random_range(lo..hi)`
/// panic, so they must be rejected here.
fn parse_f64(s: &str) -> Option<f64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse::<f64>().ok().filter(|v| v.is_finite())
    }
}

/// Parse a date/datetime stats value (RFC 3339 or `YYYY-MM-DD`) to a sortable
/// integer. For `Date` columns we return *whole days since the UNIX epoch* —
/// the right unit for uniform sampling that doesn't massively under-weight the
/// max date (stats min/max/q* values are always at midnight, so sampling over
/// seconds gives the max-day a single tick out of an 86,400-tick day). For
/// `DateTime` columns we keep seconds since the UNIX epoch.
fn parse_epoch(s: &str, is_datetime: bool) -> Option<i64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let seconds = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(trimmed) {
        dt.timestamp()
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        date.and_hms_opt(0, 0, 0)
            .map(|ndt| ndt.and_utc().timestamp())?
    } else {
        return None;
    };
    Some(if is_datetime {
        seconds
    } else {
        seconds.div_euclid(86_400)
    })
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use rand::SeedableRng;

    use super::*;

    fn stats(
        field: &str,
        r#type: &str,
        cardinality: u64,
        nullcount: u64,
        min: &str,
        max: &str,
        addl: &[(&str, &str)],
    ) -> StatsRecord {
        let mut addl_cols = IndexMap::new();
        for (k, v) in addl {
            addl_cols.insert((*k).to_string(), (*v).to_string());
        }
        StatsRecord {
            field: field.to_string(),
            r#type: r#type.to_string(),
            cardinality,
            nullcount,
            min: min.to_string(),
            max: max.to_string(),
            addl_cols,
        }
    }

    fn freq(field: &str, value: &str, count: u64, rank: f64) -> FrequencyRecord {
        FrequencyRecord {
            field: field.to_string(),
            value: value.to_string(),
            count,
            percentage: 0.0,
            rank,
        }
    }

    #[test]
    fn null_ratio_is_reproduced_within_tolerance() {
        let generator = ColumnGenerator::LoremFallback {
            locale:     Locale::EN,
            null_ratio: 0.3,
        };
        let mut rng = StdRng::seed_from_u64(42); // DevSkim: ignore DS148264
        let n = 20_000;
        let empty = (0..n)
            .filter(|_| generator.next(&mut rng).is_empty())
            .count();
        #[allow(clippy::cast_precision_loss)]
        let ratio = empty as f64 / f64::from(n);
        assert!(
            (ratio - 0.3).abs() < 0.03,
            "null ratio {ratio} not near 0.3"
        );
    }

    #[test]
    fn frequency_weighted_only_emits_pool_values_at_the_right_ratio() {
        let freqs = [freq("c", "A", 700, 1.0), freq("c", "B", 300, 2.0)];
        let refs: Vec<&FrequencyRecord> = freqs.iter().collect();
        let generator = try_frequency_weighted(&refs, 0.0).unwrap();

        let mut rng = StdRng::seed_from_u64(7); // DevSkim: ignore DS148264
        let n = 20_000;
        let mut a = 0;
        for _ in 0..n {
            match generator.next(&mut rng).as_str() {
                "A" => a += 1,
                "B" => {},
                other => panic!("unexpected value {other:?}"),
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let ratio = f64::from(a) / f64::from(n);
        assert!((ratio - 0.7).abs() < 0.03, "A ratio {ratio} not near 0.7");
    }

    #[test]
    fn frequency_weighted_rejects_other_bucket() {
        let freqs = [freq("c", "A", 700, 1.0), freq("c", "Other (5)", 300, 0.0)];
        let refs: Vec<&FrequencyRecord> = freqs.iter().collect();
        assert!(try_frequency_weighted(&refs, 0.0).is_none());
    }

    #[test]
    fn numeric_quantile_stays_in_range_and_respects_shape() {
        let record = stats(
            "n",
            "Integer",
            1000,
            0,
            "0",
            "100",
            &[("q1", "10"), ("q2_median", "20"), ("q3", "30")],
        );
        let mut rng = StdRng::seed_from_u64(1); // DevSkim: ignore DS148264
        let generator =
            ColumnGenerator::build(&record, &[], "unknown", 1000, 5000, Locale::EN, &mut rng);

        let mut at_or_below_q3 = 0;
        let n = 10_000;
        for _ in 0..n {
            let value: f64 = generator.next(&mut rng).parse().unwrap();
            assert!((0.0..=100.0).contains(&value), "value {value} out of range");
            if value <= 30.0 {
                at_or_below_q3 += 1;
            }
        }
        #[allow(clippy::cast_precision_loss)]
        let ratio = f64::from(at_or_below_q3) / f64::from(n);
        assert!(ratio > 0.6, "expected ~75% <= q3, got {ratio}");
    }

    #[test]
    fn faker_pool_is_bounded_by_cardinality() {
        let record = stats("city", "String", 5, 0, "", "", &[]);
        let mut rng = StdRng::seed_from_u64(99); // DevSkim: ignore DS148264
        let generator =
            ColumnGenerator::build(&record, &[], "city", 1000, 5000, Locale::EN, &mut rng);

        let mut distinct = HashSet::new();
        for _ in 0..2000 {
            distinct.insert(generator.next(&mut rng));
        }
        assert!(distinct.len() <= 5, "pool should have <= 5 values");
    }

    #[test]
    fn date_quantile_stays_in_range() {
        let record = stats("d", "Date", 500, 0, "2020-01-01", "2020-12-31", &[]);
        let mut rng = StdRng::seed_from_u64(3); // DevSkim: ignore DS148264
        let generator =
            ColumnGenerator::build(&record, &[], "unknown", 500, 1000, Locale::EN, &mut rng);

        // Date type → values are whole days since the UNIX epoch.
        let lo = parse_epoch("2020-01-01", false).unwrap();
        let hi = parse_epoch("2020-12-31", false).unwrap();
        for _ in 0..1000 {
            let value = generator.next(&mut rng);
            let epoch = parse_epoch(&value, false).unwrap();
            assert!((lo..=hi).contains(&epoch), "date {value} out of range");
        }
    }

    #[test]
    fn same_seed_produces_identical_sequences() {
        let record = stats(
            "n",
            "Integer",
            1000,
            100,
            "0",
            "100",
            &[("q1", "10"), ("q2_median", "20"), ("q3", "30")],
        );
        let mut build_rng = StdRng::seed_from_u64(11); // DevSkim: ignore DS148264
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "unknown",
            1000,
            5000,
            Locale::EN,
            &mut build_rng,
        );

        let mut rng1 = StdRng::seed_from_u64(123); // DevSkim: ignore DS148264
        let mut rng2 = StdRng::seed_from_u64(123); // DevSkim: ignore DS148264
        for _ in 0..200 {
            assert_eq!(generator.next(&mut rng1), generator.next(&mut rng2));
        }
    }
}
