//! The per-column generation model.
//!
//! Each source column is reduced to one `ColumnGenerator` built from its
//! `stats` record, its `frequency` records, and its dictionary `content_type`.
//! `ColumnGenerator::next` then emits one synthetic value per call.
//!
//! Construction precedence (highest first):
//!   1. `FakerMapped` — only when `--consistent-fakes` is set, the column is fully enumerated by
//!      `frequency`, and the dictionary `content_type` is a STRUCTURED faker token (not in
//!      `UNSTRUCTURED_CONTENT_TYPES`). Each distinct source value is paired with one fake at build
//!      time, so the same source value always emits the same fake while the source frequency
//!      distribution is preserved. Overrides `FrequencyWeighted` for these columns (the whole point
//!      of the flag: emit fakes, not real values).
//!   2. `FrequencyWeighted` — the column is *fully enumerated* by `frequency` (no "Other" catch-all
//!      bucket, not all-unique). The real value set is emitted with real frequency weights, so
//!      cardinality, repetition, and value identity are reproduced exactly. This wins over a faker
//!      mapping for non-`--consistent-fakes` runs (a `state` column with 50 enumerated values emits
//!      the real 50).
//!   3. `Faker` — the dictionary `content_type` maps to a `fake-rs` faker. Bounded-cardinality
//!      columns sample from a fixed pre-generated pool of distinct fake values (the consistency
//!      mechanism); all-unique columns generate a fresh value per row.
//!   4. Type-based — `NumericQuantile` / `DateQuantile` (quantile-bucketed to reproduce
//!      distribution shape), `Boolean`, or `LoremFallback`.
//!
//! Cross-column correlation is out of scope: every column is generated
//! independently.

use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use rand::{RngExt, rngs::StdRng};
use serde::{Deserialize, Serialize};

use super::faker_map::{self, Locale};
use crate::{
    CliResult,
    cmd::describegpt::dictionary::{FrequencyRecord, StatsRecord},
};

/// `qsv frequency`'s default text for null/empty values (`--null-text`).
const NULL_TEXT: &str = "(NULL)";
/// `qsv frequency`'s sentinel for all-unique (ID) columns (`--all-unique-text`).
const ALL_UNIQUE_SENTINEL: &str = "<ALL_UNIQUE>";
/// Upper bound on the size of a pre-generated faker pool, so a tiny-valued
/// faker (e.g. `state_abbr`) with a huge declared cardinality cannot blow up
/// memory or loop excessively.
const CARDINALITY_POOL_CAP: u64 = 100_000;

/// Content-type tokens treated as *unstructured* free text — the only paths
/// where string-length statistics (`min_length`/`max_length`/`avg_length`/
/// `stddev_length`) are applied. Structured fakers (email, name, uuid, phone,
/// address parts, …) are intentionally excluded because truncating them would
/// corrupt their semantic format (e.g. `"john.doe@fa"` is a broken email).
///
/// Note: `is_faker_token` filters `unknown` out of the `Faker` variant — it
/// always lands in `LoremFallback`. `unknown` is kept here as defense-in-depth
/// for hand-crafted dictionaries that bypass that gate.
const UNSTRUCTURED_CONTENT_TYPES: &[&str] = &[
    "lorem_word",
    "lorem_sentence",
    "lorem_paragraph",
    "free_text",
    "unknown",
];

/// Per-column character-length distribution extracted from `qsv stats`
/// (`min_length` / `max_length` / `avg_length` / `stddev_length`). Only attached
/// to unstructured/free-text generators — structured fakers ignore it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct LengthStats {
    pub min:    usize,
    pub max:    usize,
    pub avg:    f64,
    /// `0.0` when absent or unparsable → falls back to uniform `[min, max]`
    /// sampling in `sample_target_length`.
    pub stddev: f64,
}

impl LengthStats {
    /// Parse length stats from a `StatsRecord`'s `addl_cols`. Returns `None`
    /// when `min_length` or `max_length` is absent/unparseable, or when the
    /// resulting range is empty (`max == 0` or `max < min`).
    pub(crate) fn from_addl_cols(addl: &IndexMap<String, String>) -> Option<Self> {
        let min = addl
            .get("min_length")
            .and_then(|s| s.trim().parse::<usize>().ok())?;
        let max = addl
            .get("max_length")
            .and_then(|s| s.trim().parse::<usize>().ok())?;
        if max == 0 || max < min {
            return None;
        }
        #[allow(clippy::cast_precision_loss)]
        let midpoint = (min + max) as f64 / 2.0;
        let avg = addl
            .get("avg_length")
            .and_then(|s| parse_f64(s))
            .unwrap_or(midpoint);
        let stddev = addl
            .get("stddev_length")
            .and_then(|s| parse_f64(s))
            .unwrap_or(0.0);
        Some(Self {
            min,
            max,
            avg,
            stddev: stddev.max(0.0),
        })
    }
}

/// Whether `content_type` is an unstructured / free-text token eligible for
/// length-stat post-processing.
fn is_unstructured(content_type: &str) -> bool {
    UNSTRUCTURED_CONTENT_TYPES.contains(&content_type)
}

#[derive(Clone, Serialize, Deserialize)]
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
    /// `length_stats` is `Some` for any *unstructured* token in
    /// `UNSTRUCTURED_CONTENT_TYPES` (whether pooled or not) so free-text
    /// columns get truncated to the source length distribution; structured
    /// fakers (email/uuid/phone/…) keep it `None` to preserve their format.
    Faker {
        content_type: String,
        pool:         Option<Vec<String>>,
        locale:       Locale,
        null_ratio:   f64,
        length_stats: Option<LengthStats>,
    },
    /// Structured-faker column with a stable source-value -> fake-value mapping.
    /// Built only when `--consistent-fakes` is set AND the column has a *structured*
    /// faker `content_type` (i.e. `is_faker_token` is true and the type is NOT in
    /// `UNSTRUCTURED_CONTENT_TYPES`) AND `frequency` fully enumerates the column
    /// (same gate as `FrequencyWeighted`). Each distinct source value is pre-paired
    /// with one fake value at build time; the per-row draw samples a *source value*
    /// by its real frequency and emits the paired fake. This preserves the source
    /// distribution and guarantees the same source value always emits the same
    /// fake. No `length_stats` field — structured fakers preserve their semantic
    /// format (same rationale as the `Faker` variant when `is_unstructured` is
    /// false).
    FakerMapped {
        /// One fake per distinct source value, indexed in sync with `cumulative`.
        fakes:      Vec<String>,
        /// Normalized cumulative weights, ascending, last element == 1.0, derived
        /// from the source frequency counts.
        cumulative: Vec<f64>,
        null_ratio: f64,
    },
    /// Numeric column reproduced via quantile buckets.
    NumericQuantile {
        buckets:    Vec<(f64, f64)>,
        is_int:     bool,
        /// Float columns only: decimal places to round to (the source's
        /// `max_precision`); `None` leaves values unrounded.
        decimals:   Option<u32>,
        null_ratio: f64,
    },
    /// Date / `DateTime` column reproduced via quantile buckets over epoch seconds.
    /// `date_format` carries the describegpt-inferred chrono strftime format
    /// (from a `date:<fmt>` / `datetime:<fmt>` content type); when `None`,
    /// `next()` falls back to RFC 3339 (`DateTime`) / `%Y-%m-%d` (Date).
    DateQuantile {
        buckets:     Vec<(i64, i64)>,
        is_datetime: bool,
        date_format: Option<String>,
        null_ratio:  f64,
    },
    /// Boolean column — safety net; 2-value columns are normally caught by
    /// `FrequencyWeighted` first.
    Boolean { true_ratio: f64, null_ratio: f64 },
    /// Last-resort text generator for non-faker, non-enumerated string columns.
    /// `locale` is stored so per-row `lorem_sentence` generation uses the same
    /// locale as the rest of the column build. When `length_stats` is `Some`,
    /// each generated sentence is truncated to a target character length drawn
    /// from `Normal(avg, stddev)` clamped to `[min, max]`.
    LoremFallback {
        locale:       Locale,
        null_ratio:   f64,
        length_stats: Option<LengthStats>,
    },
    /// All-null / no-data column — always emits an empty value.
    Empty,
}

impl ColumnGenerator {
    #[cfg(test)]
    pub(crate) fn build(
        stats: &StatsRecord,
        freqs: &[&FrequencyRecord],
        content_type: &str,
        total_rows: u64,
        requested_rows: u64,
        locale: Locale,
        consistent_fakes: bool,
        min_count: u64,
        rng: &mut StdRng,
    ) -> ColumnGenerator {
        Self::build_tracked(
            stats,
            freqs,
            content_type,
            total_rows,
            requested_rows,
            locale,
            consistent_fakes,
            min_count,
            &mut None,
            rng,
        )
    }

    /// `suppressed_out` receives the (distinct values, rows) that `min_count`
    /// dropped, when the column's real values were sampled from (or would
    /// have been, before every value was dropped).
    pub(crate) fn build_tracked(
        stats: &StatsRecord,
        freqs: &[&FrequencyRecord],
        content_type: &str,
        total_rows: u64,
        requested_rows: u64,
        locale: Locale,
        consistent_fakes: bool,
        min_count: u64,
        suppressed_out: &mut Option<(usize, u64)>,
        rng: &mut StdRng,
    ) -> ColumnGenerator {
        let null_ratio = compute_null_ratio(stats.nullcount, total_rows);

        // All-null or no distinct values → nothing to synthesize.
        if stats.cardinality == 0 || (total_rows > 0 && stats.nullcount >= total_rows) {
            return ColumnGenerator::Empty;
        }

        // Date/DateTime columns ALWAYS take the `build_date()` path so their
        // synthetic values respect the source's real min/max (and quartile)
        // bounds — never a content-type faker. When describegpt tagged the
        // column `date:<fmt>` / `datetime:<fmt>`, the inferred chrono strftime
        // format is extracted here and threaded into `build_date` so generated
        // values are emitted in the column's original on-disk format. Any other
        // LLM-emitted content_type on a date column (e.g. a stray `time` — which
        // is *time-of-day*, "14:30:45", not a date — or `duration`) is
        // suppressed so the faker branches below fall through to the type-based
        // match instead of emitting time-of-day strings for a date column.
        let is_date_column = matches!(stats.r#type.as_str(), "Date" | "DateTime");
        let date_format = if is_date_column {
            faker_map::parse_date_format(content_type)
        } else {
            None
        };
        let content_type = if is_date_column { "" } else { content_type };

        // 1. When `--consistent-fakes` is set AND the column has a STRUCTURED faker content type
        //    AND frequency fully enumerates the column, build a stable source-value -> fake-value
        //    map. Runs BEFORE `try_frequency_weighted` so it overrides the "emit real values" path
        //    — that's the whole point of the flag: fakes (deidentified), not real values, with
        //    stable mapping.
        if consistent_fakes
            && faker_map::is_faker_token(content_type)
            && !is_unstructured(content_type)
            && let Some(generator) = try_faker_mapped(freqs, content_type, locale, null_ratio, rng)
        {
            return generator;
        }

        // 2. Fully-enumerated frequency pool — reproduces the real value set, weights, cardinality
        //    and repetition structure, minus values seen fewer than `min_count` times.
        if let Some(generator) =
            try_frequency_weighted(freqs, null_ratio, min_count, suppressed_out)
        {
            return generator;
        }

        // 3. Semantic faker.
        if faker_map::is_faker_token(content_type) {
            let pool =
                build_faker_pool(content_type, stats.cardinality, requested_rows, locale, rng);
            // Length stats apply to unstructured fakers regardless of pool
            // presence — bounded-cardinality free-text columns (a common
            // shape) would otherwise produce un-truncated lorem sentences
            // from the pool. Structured fakers (email/uuid/phone/…) keep
            // `length_stats = None` so their semantic format stays intact;
            // the per-row hook in `next()` is then a no-op.
            let length_stats = if is_unstructured(content_type) {
                LengthStats::from_addl_cols(&stats.addl_cols)
            } else {
                None
            };
            return ColumnGenerator::Faker {
                content_type: content_type.to_string(),
                pool,
                locale,
                null_ratio,
                length_stats,
            };
        }

        // 4. Type-based fallback.
        match stats.r#type.as_str() {
            "Integer" => build_numeric(stats, true, null_ratio),
            "Float" => build_numeric(stats, false, null_ratio),
            "Date" => build_date(stats, false, null_ratio, date_format),
            "DateTime" => build_date(stats, true, null_ratio, date_format),
            "Boolean" => build_boolean(freqs, null_ratio),
            "NULL" => ColumnGenerator::Empty,
            // "String" and anything unrecognized.
            _ => ColumnGenerator::LoremFallback {
                locale,
                null_ratio,
                length_stats: LengthStats::from_addl_cols(&stats.addl_cols),
            },
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
                length_stats,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                // Pick a value: from the pool (bounded-cardinality fakers) or
                // freshly generated (all-unique / above-cap fakers).
                let value = if let Some(p) = pool
                    && !p.is_empty()
                {
                    p[rng.random_range(0..p.len())].clone()
                } else {
                    faker_map::content_type_to_value(content_type, *locale, rng).unwrap_or_default()
                };
                // Length truncation only kicks in when `build()` attached
                // `length_stats` — i.e. for unstructured tokens. Structured
                // fakers (email/uuid/phone/…) reach this with `length_stats =
                // None` so their semantic format is preserved.
                match length_stats {
                    Some(ls) => truncate_to_chars(value, sample_target_length(ls, rng)),
                    None => value,
                }
            },

            ColumnGenerator::FakerMapped {
                fakes,
                cumulative,
                null_ratio,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                // Same weighted draw as `FrequencyWeighted`, but the indexed
                // vector holds pre-paired fakes (one per distinct source value),
                // so the same source value always emits the same fake. Structured
                // fakers preserve their semantic format — no length truncation.
                let r = rng.random_range(0.0..1.0);
                let idx = cumulative.partition_point(|&c| c < r).min(fakes.len() - 1);
                fakes[idx].clone()
            },

            ColumnGenerator::NumericQuantile {
                buckets,
                is_int,
                decimals,
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
                format_number(value, *is_int, *decimals)
            },

            ColumnGenerator::DateQuantile {
                buckets,
                is_datetime,
                date_format,
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
                // For DateTime: `unit` is epoch seconds. For Date: `unit` is
                // whole days since the UNIX epoch, multiplied back to seconds.
                // `date_format` (when present) is the describegpt-inferred
                // chrono strftime format; otherwise fall back to RFC 3339
                // (DateTime) / `%Y-%m-%d` (Date). The format is re-validated by
                // `parse_date_format`, so `format()` cannot panic here.
                let seconds = if *is_datetime { unit } else { unit * 86_400 };
                match chrono::DateTime::from_timestamp(seconds, 0) {
                    Some(dt) => match date_format {
                        Some(fmt) => dt.format(fmt.as_str()).to_string(),
                        None if *is_datetime => dt.to_rfc3339(),
                        None => dt.format("%Y-%m-%d").to_string(),
                    },
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

            ColumnGenerator::LoremFallback {
                locale,
                null_ratio,
                length_stats,
            } => {
                if draw_null(*null_ratio, rng) {
                    return String::new();
                }
                let value = faker_map::content_type_to_value("lorem_sentence", *locale, rng)
                    .unwrap_or_default();
                match length_stats {
                    Some(ls) => truncate_to_chars(value, sample_target_length(ls, rng)),
                    None => value,
                }
            },
        }
    }
}

/// `nullcount / total_rows`, clamped to `[0.0, 1.0]`.
pub(crate) fn compute_null_ratio(nullcount: u64, total_rows: u64) -> f64 {
    if total_rows == 0 {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let ratio = nullcount as f64 / total_rows as f64;
    ratio.clamp(0.0, 1.0)
}

/// Decide whether this cell should be null, given the column's null ratio.
pub(crate) fn draw_null(null_ratio: f64, rng: &mut StdRng) -> bool {
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
///
/// Values seen fewer than `min_count` times are dropped so rare real values
/// never reach the output; their weight is redistributed over the kept values.
/// If every value is dropped, returns `None`.
fn try_frequency_weighted(
    freqs: &[&FrequencyRecord],
    null_ratio: f64,
    min_count: u64,
    suppressed_out: &mut Option<(usize, u64)>,
) -> Option<ColumnGenerator> {
    if freqs.is_empty() {
        return None;
    }
    // `qsv frequency` gives rank 0 to both the "Other" catch-all bucket and the
    // `<ALL_UNIQUE>` sentinel. Either one means the column is NOT fully
    // enumerated for our purposes.
    // The `(NULL)` row also parses to rank 0 (frequency leaves its rank blank),
    // but nulls are reproduced via `null_ratio`, so it must not disqualify
    // the column.
    if freqs.iter().any(|f| f.rank == 0.0 && f.value != NULL_TEXT) {
        return None;
    }

    let mut values = Vec::new();
    let mut weights = Vec::new();
    let mut suppressed = 0_usize;
    let mut suppressed_rows = 0_u64;
    for f in freqs {
        // Nulls are reproduced via `null_ratio`, not the value pool.
        if f.value == NULL_TEXT || f.value.contains(ALL_UNIQUE_SENTINEL) || f.count == 0 {
            continue;
        }
        if f.count < min_count {
            suppressed += 1;
            suppressed_rows += f.count;
            continue;
        }
        values.push(f.value.clone());
        #[allow(clippy::cast_precision_loss)]
        weights.push(f.count as f64);
    }
    if suppressed > 0 {
        *suppressed_out = Some((suppressed, suppressed_rows));
        log::info!(
            "synthesize: column '{}' — suppressed {suppressed} value(s) seen fewer than \
             --min-count ({min_count}) times",
            freqs[0].field
        );
    }
    if values.is_empty() {
        if suppressed > 0 {
            wwarn!(
                "synthesize: every value of column '{}' occurs fewer than --min-count \
                 ({min_count}) times; it will be generated from its faker or type-based generator \
                 instead of its real values",
                freqs[0].field
            );
        }
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

/// Build a `FakerMapped` generator if the column is fully enumerated by the
/// frequency records AND a structured faker is available for `content_type`.
/// Each distinct source value is paired with one fake value at build time;
/// distinct fakes are preferred (best-effort, capped attempts — same pattern
/// as `build_faker_pool`) so the cardinality structure is preserved, but
/// duplicates are accepted when the faker's value space is smaller than the
/// source cardinality (e.g. `state_abbr` with > 50 distinct sources). All
/// RNG draws go through the master `rng`, so `--seed` reproducibility is
/// preserved.
///
/// Returns `None` when the same gate `try_frequency_weighted` uses rejects
/// the column (no enumeration, all rank-0, no real values after null
/// filtering), when the post-filter source cardinality exceeds
/// `CARDINALITY_POOL_CAP` (matching `build_faker_pool`'s safety bound — keeps
/// allocations and the `source_count * 20` attempt loop bounded when a user
/// passes `--freq-limit 0` on a high-cardinality column), OR when the faker
/// fails to produce any value at all — in any of those cases the caller
/// falls through to the regular `Faker` variant.
fn try_faker_mapped(
    freqs: &[&FrequencyRecord],
    content_type: &str,
    locale: Locale,
    null_ratio: f64,
    rng: &mut StdRng,
) -> Option<ColumnGenerator> {
    if freqs.is_empty() {
        return None;
    }
    // Same enumeration gate as `try_frequency_weighted`: rank 0 marks either
    // the "Other" catch-all or the `<ALL_UNIQUE>` sentinel — both mean the
    // column is NOT fully enumerated.
    // The `(NULL)` row also parses to rank 0 (frequency leaves its rank blank),
    // but nulls are reproduced via `null_ratio`, so it must not disqualify
    // the column.
    if freqs.iter().any(|f| f.rank == 0.0 && f.value != NULL_TEXT) {
        return None;
    }

    // Collect source value weights (nulls and sentinels are skipped — they
    // are reproduced via `null_ratio`, not the value pool).
    let mut weights = Vec::new();
    let mut source_count = 0usize;
    for f in freqs {
        if f.value == NULL_TEXT || f.value.contains(ALL_UNIQUE_SENTINEL) || f.count == 0 {
            continue;
        }
        source_count += 1;
        #[allow(clippy::cast_precision_loss)]
        weights.push(f.count as f64);
    }
    if source_count == 0 {
        return None;
    }
    // Bound the work — match `build_faker_pool`'s safety cap so a user passing
    // `--freq-limit 0` on a high-cardinality column does not allocate a huge
    // `HashSet`/`Vec` or spin in a `source_count * 20` attempt loop. Above the
    // cap, fall through to the regular `Faker` variant (per-row generation, no
    // pre-allocated map).
    if source_count as u64 > CARDINALITY_POOL_CAP {
        return None;
    }

    // Pre-generate `source_count` fakes; prefer distinct (best-effort) so the
    // cardinality structure is preserved. Attempt cap mirrors
    // `build_faker_pool` to handle fakers with a small value space gracefully.
    let mut fakes = Vec::with_capacity(source_count);
    let mut seen = HashSet::with_capacity(source_count);
    let max_attempts = source_count.saturating_mul(20).max(1000);

    for _ in 0..max_attempts {
        if fakes.len() >= source_count {
            break;
        }
        let value = faker_map::content_type_to_value(content_type, locale, rng)?;
        if seen.insert(value.clone()) {
            fakes.push(value);
        }
    }
    // If we couldn't reach `source_count` distinct fakes within the attempt
    // cap, the faker's value space is smaller than the source cardinality
    // (e.g. `state_abbr` with > 50 distinct sources). Warn once, then fill the
    // remainder with fresh draws — duplicates are acceptable (the mapping is
    // still deterministic per-source-value, just not 1:1 globally).
    if fakes.len() < source_count {
        log::warn!(
            "synthesize: faker '{content_type}' could only produce {} distinct fakes for \
             {source_count} source values; some source values will share a fake",
            fakes.len()
        );
        while fakes.len() < source_count {
            let value = faker_map::content_type_to_value(content_type, locale, rng)?;
            fakes.push(value);
        }
    }

    // Build cumulative weights (pin last to exactly 1.0 to avoid FP drift).
    let total: f64 = weights.iter().sum();
    let mut cumulative = Vec::with_capacity(weights.len());
    let mut acc = 0.0;
    for w in &weights {
        acc += w / total;
        cumulative.push(acc);
    }
    if let Some(last) = cumulative.last_mut() {
        *last = 1.0;
    }

    Some(ColumnGenerator::FakerMapped {
        fakes,
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

/// Build a quantile-bucketed numeric generator. Falls back to `Empty` when
/// there is no usable numeric range at all.
fn build_numeric(stats: &StatsRecord, is_int: bool, null_ratio: f64) -> ColumnGenerator {
    match numeric_buckets(stats) {
        Some(buckets) => ColumnGenerator::NumericQuantile {
            buckets,
            is_int,
            decimals: if is_int { None } else { float_decimals(stats) },
            null_ratio,
        },
        None => ColumnGenerator::Empty,
    }
}

/// Above this many decimal places a column's `max_precision` reflects binary
/// float noise (`0.30000000000000004` counts 17), not how the source was
/// written, so its values are left unrounded.
const MAX_ROUNDED_DECIMALS: u32 = 15;

/// The decimal places a Float column's synthetic values are rounded to: the
/// most any source value shows (`stats` `max_precision`), or `None` when that
/// is unknown or float noise.
pub(crate) fn float_decimals(stats: &StatsRecord) -> Option<u32> {
    stats
        .addl_cols
        .get("max_precision")?
        .parse::<u32>()
        .ok()
        .filter(|&d| d <= MAX_ROUNDED_DECIMALS)
}

/// `value` on the grid of `decimals` places, as `(steps, scale)` with
/// `value ≈ steps / scale`; `None` when it can't be rounded exactly (no
/// decimals, or too large for the grid to be finer than an f64's spacing).
/// Working in whole steps keeps clear of noise like `1.1 * 100 =
/// 110.00000000000001`: `steps / scale` is the f64 nearest the decimal, so it
/// prints as that decimal.
pub(crate) fn decimal_steps(value: f64, decimals: Option<u32>) -> Option<(f64, f64)> {
    // Below 2^50 the error in `value * scale` is under a quarter step, so a
    // value already on the grid rounds to itself; nearer 2^53 it can land on
    // a half and round a step away (4.000000000000005 at 15 places). Whole
    // numbers aren't scaled, so they round exactly up to 2^53, beyond which
    // every f64 is already whole.
    const EXACT_STEPS: f64 = 1_125_899_906_842_624.0;
    const EXACT_WHOLE: f64 = 9_007_199_254_740_992.0;
    let decimals = decimals?;
    let scale = 10_f64.powi(i32::try_from(decimals).ok()?);
    let scaled = value * scale;
    let limit = if decimals == 0 {
        EXACT_WHOLE
    } else {
        EXACT_STEPS
    };
    (scaled.abs() < limit).then(|| (scaled.round(), scale))
}

/// Round `value` to `decimals` places (no-op for `None`), never returning
/// negative zero.
pub(crate) fn round_to(value: f64, decimals: Option<u32>) -> f64 {
    match decimal_steps(value, decimals) {
        Some((steps, scale)) => steps / scale + 0.0,
        None => value,
    }
}

/// The largest value on the `decimals` grid that is <= `value` (no-op for
/// `None`).
pub(crate) fn floor_to(value: f64, decimals: Option<u32>) -> f64 {
    match decimal_steps(value, decimals) {
        Some((steps, scale)) => {
            let rounded = steps / scale;
            (if rounded > value {
                (steps - 1.0) / scale
            } else {
                rounded
            }) + 0.0
        },
        None => value,
    }
}

/// `value` rounded to the `decimals` grid, moved up one step if rounding took
/// it below `floor` (no-op for `None`). With `value >= floor`, the result is
/// the smallest grid value >= `floor` in that case, so it stays in order
/// after a predecessor and no higher than any grid value >= `floor`.
pub(crate) fn round_at_least(value: f64, floor: f64, decimals: Option<u32>) -> f64 {
    match decimal_steps(value, decimals) {
        Some((steps, scale)) => {
            let rounded = steps / scale;
            (if rounded < floor {
                (steps + 1.0) / scale
            } else {
                rounded
            }) + 0.0
        },
        None => value,
    }
}

/// Render a numeric value as an Integer column (rounded to a whole number) or
/// a Float column (rounded to `decimals` places, shortest form).
pub(crate) fn format_number(value: f64, is_int: bool, decimals: Option<u32>) -> String {
    if is_int {
        #[allow(clippy::cast_possible_truncation)]
        {
            (value.round() as i64).to_string()
        }
    } else {
        round_to(value, decimals).to_string()
    }
}

/// Equal-mass buckets for a numeric column, from its `stats` min/max and its
/// interior edges (see `interior_edges`). Falls back to a single `[min, max]`
/// bucket when the edges are missing/inconsistent; returns `None` when there is
/// no usable numeric range.
pub(crate) fn numeric_buckets(stats: &StatsRecord) -> Option<Vec<(f64, f64)>> {
    let lo = parse_f64(&stats.min)?;
    let hi = parse_f64(&stats.max)?;
    if hi < lo {
        return None;
    }
    // Edges are clamped into [lo, hi]: `--min-count` range trimming can move
    // the ends inside the source quantiles on small columns.
    let edges = interior_edges(stats).and_then(|e| {
        e.iter()
            .map(|s| parse_f64(s).map(|v| v.clamp(lo, hi)))
            .collect::<Option<Vec<f64>>>()
    });
    Some(buckets_from_edges(lo, hi, edges))
}

/// Build a quantile-bucketed date/datetime generator. Bucket bounds are
/// expressed in *days since the UNIX epoch* for `Date` columns and *seconds
/// since the UNIX epoch* for `DateTime` columns — see `parse_epoch`.
///
/// `date_format` is the describegpt-inferred chrono strftime output format
/// (from a `date:<fmt>` / `datetime:<fmt>` content type); `None` falls back to
/// RFC 3339 (`DateTime`) / `%Y-%m-%d` (Date) in `next()`.
fn build_date(
    stats: &StatsRecord,
    is_datetime: bool,
    null_ratio: f64,
    date_format: Option<String>,
) -> ColumnGenerator {
    match date_buckets(stats, is_datetime) {
        Some(buckets) => ColumnGenerator::DateQuantile {
            buckets,
            is_datetime,
            date_format,
            null_ratio,
        },
        None => ColumnGenerator::Empty,
    }
}

/// Equal-mass buckets for a date/datetime column. Bounds are *days since the
/// UNIX epoch* for `Date` and *seconds since the UNIX epoch* for `DateTime` —
/// see `parse_epoch`. Falls back to a single `[min, max]` bucket when the edges
/// are missing/inconsistent; returns `None` when there is no usable range.
pub(crate) fn date_buckets(stats: &StatsRecord, is_datetime: bool) -> Option<Vec<(i64, i64)>> {
    let lo = parse_epoch(&stats.min, is_datetime)?;
    let hi = parse_epoch(&stats.max, is_datetime)?;
    if hi < lo {
        return None;
    }
    // Clamped into [lo, hi] for the same reason as `numeric_buckets`.
    let edges = interior_edges(stats).and_then(|e| {
        e.iter()
            .map(|s| parse_epoch(s, is_datetime).map(|v| v.clamp(lo, hi)))
            .collect::<Option<Vec<i64>>>()
    });
    Some(buckets_from_edges(lo, hi, edges))
}

/// `addl_cols` key under which `apply_quantiles` stores a column's
/// `--quantiles` interior edges, joined by `QUANTILE_EDGE_SEP`.
const QUANTILE_EDGES_KEY: &str = "synthesize_quantile_edges";
const QUANTILE_EDGE_SEP: char = '\u{1f}';

/// Bucket counts `--quantiles` accepts: the divisors of 100, so every edge is a
/// whole percentile (`stats` truncates fractional ones) and every bucket holds
/// the same share of rows, which the samplers assume.
pub(crate) const QUANTILE_CHOICES: [usize; 8] = [2, 4, 5, 10, 20, 25, 50, 100];

/// The bucket count a column of `values` non-null values gets for
/// `--quantiles requested`: the largest choice dividing `requested` that leaves
/// every bucket MORE than `min_count` values. Nearest-rank edges are real
/// source values, and the range ends are already trimmed to the k-th smallest
/// and largest values: with exactly k per bucket the first edge lands on the
/// trimmed minimum, and that zero-width bucket would emit a value seen once for
/// a whole bucket's share. `None` = keep the default quartiles (also when
/// nothing qualifies).
pub(crate) fn effective_quantiles(requested: usize, values: u64, min_count: u64) -> Option<usize> {
    if requested == 4 {
        return None;
    }
    QUANTILE_CHOICES
        .iter()
        .rev()
        .copied()
        .find(|&d| requested.is_multiple_of(d) && values / d as u64 > min_count.max(1))
        // Fewer than the default 4 only when asked for: a capped count below
        // 4 would be coarser than not passing --quantiles at all.
        .filter(|&d| d > 4 || d == requested)
}

/// Interior bucket edges of a column, as stats strings: the `--quantiles`
/// edges when `apply_quantiles` stored them, else the quartiles.
fn interior_edges(stats: &StatsRecord) -> Option<Vec<&str>> {
    if let Some(edges) = stats.addl_cols.get(QUANTILE_EDGES_KEY) {
        return Some(edges.split(QUANTILE_EDGE_SEP).collect());
    }
    let get = |k: &str| stats.addl_cols.get(k).map(String::as_str);
    Some(vec![get("q1")?, get("q2_median")?, get("q3")?])
}

/// `[lo, e1], [e1, e2], …, [en, hi]`, or the single `[lo, hi]` when the edges
/// are missing or out of order.
fn buckets_from_edges<T: Copy + PartialOrd>(lo: T, hi: T, edges: Option<Vec<T>>) -> Vec<(T, T)> {
    let Some(edges) = edges else {
        return vec![(lo, hi)];
    };
    let mut chain = Vec::with_capacity(edges.len() + 2);
    chain.push(lo);
    chain.extend(edges);
    chain.push(hi);
    let pairs: Vec<(T, T)> = chain
        .iter()
        .copied()
        .zip(chain.iter().copied().skip(1))
        .collect();
    if !pairs.iter().all(|(a, b)| a <= b) {
        return vec![(lo, hi)];
    }
    pairs
}

/// For `--quantiles` other than 4, store each numeric/date column's interior
/// edges (from the `stats --percentiles` cell) for the bucket builders. A
/// column keeps its quartiles when `effective_quantiles` says so.
pub(crate) fn apply_quantiles(
    records: &mut [StatsRecord],
    requested: usize,
    total_rows: u64,
    min_count: u64,
) -> CliResult<()> {
    if requested == 4 {
        return Ok(());
    }
    let sep = crate::cmd::stats::stats_separator_from_env();
    let requested_labels: Vec<String> = (1..requested)
        .map(|i| (100 * i / requested).to_string())
        .collect();
    for sr in records.iter_mut() {
        if !matches!(
            sr.r#type.as_str(),
            "Integer" | "Float" | "Date" | "DateTime"
        ) {
            continue;
        }
        let values = total_rows.saturating_sub(sr.nullcount);
        let Some(n) = effective_quantiles(requested, values, min_count) else {
            continue;
        };
        let Some(cell) = sr.addl_cols.get("percentiles").filter(|c| !c.is_empty()) else {
            continue;
        };
        let by_label = parse_percentiles(cell, &sep, &requested_labels);
        let mut edges = Vec::with_capacity(n - 1);
        for i in 1..n {
            let label = (100 * i / n).to_string();
            let Some(v) = by_label.get(label.as_str()) else {
                return fail_clierror!(
                    "stats reported no {label}th percentile for column '{}' (got '{cell}').",
                    sr.field
                );
            };
            edges.push(*v);
        }
        let joined = edges.join(&QUANTILE_EDGE_SEP.to_string());
        sr.addl_cols.insert(QUANTILE_EDGES_KEY.to_string(), joined);
    }
    Ok(())
}

/// Parse a `stats --percentiles` cell, `<l1>: <v1><sep><l2>: <v2>…`, for the
/// percentile `labels` requested, in order. Each value ends where the next
/// `<sep><label>: ` begins, so a separator that also occurs inside values
/// (`QSV_STATS_SEPARATOR=-` with ISO dates) or a `DateTime` value's own colons can't
/// split a value. Stops at the first label it can't find.
fn parse_percentiles<'a>(
    cell: &'a str,
    sep: &str,
    labels: &'a [String],
) -> HashMap<&'a str, &'a str> {
    let mut out = HashMap::with_capacity(labels.len());
    let Some(first) = labels.first() else {
        return out;
    };
    let Some(mut rest) = cell.strip_prefix(&format!("{first}: ")) else {
        return out;
    };
    for (i, label) in labels.iter().enumerate() {
        match labels.get(i + 1) {
            Some(next) => {
                let marker = format!("{sep}{next}: ");
                let Some(pos) = rest.find(&marker) else {
                    break;
                };
                out.insert(label.as_str(), &rest[..pos]);
                rest = &rest[pos + marker.len()..];
            },
            None => {
                out.insert(label.as_str(), rest);
            },
        }
    }
    out
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
pub(crate) fn parse_f64(s: &str) -> Option<f64> {
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
pub(crate) fn parse_epoch(s: &str, is_datetime: bool) -> Option<i64> {
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

/// Sample a target character length for unstructured-string generation.
///
/// When `stats.stddev > 0`, draws from `Normal(stats.avg, stats.stddev)` using
/// the Box-Muller transform (one Normal per call, cosine variant — no extra
/// dependency) and clamps to `[stats.min, stats.max]`. When `stddev` is zero or
/// missing, falls back to a uniform draw over `[stats.min, stats.max]`. The
/// returned length is always at least 1 so we never request an empty string.
///
/// **On clamping vs. rejection sampling:** clamping is intentional. The
/// `avg`/`stddev` reported by `qsv stats` can be inconsistent with `min`/`max`
/// (rounding, an averaged value near a bound, or a large `stddev` relative to
/// the range), and rejection-sampling could loop unboundedly. Clamping piles a
/// small probability mass at the bounds in those edge cases, which slightly
/// biases the empirical mean — an acceptable trade for synthetic data and a
/// guaranteed-bounded sampler. Likewise the `lo >= hi` guard returns
/// `lo.max(1)`, so a degenerate `min == max == 0` yields a 1-char target
/// rather than an empty string.
fn sample_target_length(stats: &LengthStats, rng: &mut StdRng) -> usize {
    let (lo, hi) = (stats.min, stats.max);
    if lo >= hi {
        return lo.max(1);
    }
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    let raw = if stats.stddev > 0.0 {
        // Box-Muller (cosine variant). `u1 >= f64::EPSILON` guards `ln(0) = -∞`.
        let u1: f64 = rng.random_range(f64::EPSILON..1.0);
        let u2: f64 = rng.random_range(0.0..1.0);
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        stats.avg + z * stats.stddev
    } else {
        rng.random_range(lo as f64..=hi as f64)
    };
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    {
        let clamped = raw.clamp(lo as f64, hi as f64).round();
        (clamped as usize).max(1)
    }
}

/// UTF-8-safe truncate to at most `n` characters. Returns the string unchanged
/// when it is already short enough — we never pad, since natural variation
/// below the target is acceptable per the length-stats design.
///
/// Note: with very small `max_length`, lorem-style fakers (minimum ~4 words
/// ≈ 20+ chars) may still need significant truncation, producing fragments
/// like `"Lorem ip"`. This is acceptable for synthetic data.
fn truncate_to_chars(s: String, n: usize) -> String {
    if s.chars().count() <= n {
        s
    } else {
        s.chars().take(n).collect()
    }
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
    fn effective_quantiles_caps_thin_buckets() {
        // 4 is the quartile path, never capped.
        assert_eq!(effective_quantiles(4, 3, 5), None);
        assert_eq!(effective_quantiles(20, 1000, 5), Some(20));
        // 60 values at k=5 hold at most 12 buckets: 100 → 10, 20 → 10.
        assert_eq!(effective_quantiles(100, 60, 5), Some(10));
        assert_eq!(effective_quantiles(20, 60, 5), Some(10));
        // 25's only smaller choice is 5.
        assert_eq!(effective_quantiles(25, 60, 5), Some(5));
        // More than k per bucket: 60 values at k=6 can't hold 10 (6 each) → 5.
        assert_eq!(effective_quantiles(100, 60, 6), Some(5));
        // Nothing qualifies → the quartiles.
        assert_eq!(effective_quantiles(100, 8, 5), None);
        assert_eq!(effective_quantiles(20, 11, 5), None);
        // A cap below 4 would be coarser than the default → the quartiles,
        // unless 2 is what was asked for.
        assert_eq!(effective_quantiles(100, 20, 5), None);
        assert_eq!(effective_quantiles(10, 4, 0), None);
        assert_eq!(effective_quantiles(2, 20, 5), Some(2));
    }

    #[test]
    fn apply_quantiles_builds_buckets_from_percentiles() {
        let mut records = vec![
            stats(
                "n",
                "Float",
                100,
                0,
                "0",
                "100",
                &[
                    ("q1", "25"),
                    ("q2_median", "50"),
                    ("q3", "75"),
                    ("percentiles", "20: 1|40: 2|60: 3|80: 90"),
                ],
            ),
            stats(
                "ts",
                "DateTime",
                100,
                0,
                "2024-01-01T00:00:00+00:00",
                "2024-01-05T00:00:00+00:00",
                &[(
                    "percentiles",
                    "20: 2024-01-02T00:00:00+00:00|40: 2024-01-03T00:00:00+00:00|60: \
                     2024-01-03T12:00:00+00:00|80: 2024-01-04T00:00:00+00:00",
                )],
            ),
            stats("s", "String", 3, 0, "a", "c", &[("percentiles", "")]),
        ];
        apply_quantiles(&mut records, 5, 100, 5).unwrap();
        assert_eq!(
            numeric_buckets(&records[0]).unwrap(),
            vec![
                (0.0, 1.0),
                (1.0, 2.0),
                (2.0, 3.0),
                (3.0, 90.0),
                (90.0, 100.0)
            ]
        );
        let day = 86_400;
        let start = 1_704_067_200;
        assert_eq!(
            date_buckets(&records[1], true).unwrap(),
            vec![
                (start, start + day),
                (start + day, start + 2 * day),
                (start + 2 * day, start + 2 * day + day / 2),
                (start + 2 * day + day / 2, start + 3 * day),
                (start + 3 * day, start + 4 * day),
            ]
        );
        assert!(!records[2].addl_cols.contains_key(QUANTILE_EDGES_KEY));

        // A missing percentile is an error, not a silent fallback.
        let mut short = vec![stats(
            "n",
            "Integer",
            100,
            0,
            "0",
            "9",
            &[("percentiles", "20: 1|40: 2")],
        )];
        assert!(apply_quantiles(&mut short, 5, 100, 5).is_err());
    }

    #[test]
    fn parse_percentiles_splits_on_labels_not_every_separator() {
        let labels: Vec<String> = ["25", "50", "75"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        let cell = "25: 2020-01-02-50: 2020-06-30-75: 2021-01-01T10:00:00+00:00";
        let got = parse_percentiles(cell, "-", &labels);
        assert_eq!(got["25"], "2020-01-02");
        assert_eq!(got["50"], "2020-06-30");
        assert_eq!(got["75"], "2021-01-01T10:00:00+00:00");
        // A missing label stops the parse, so the caller reports it.
        let got = parse_percentiles("25: 1|75: 3", "|", &labels);
        assert!(
            !got.contains_key("25") && !got.contains_key("75"),
            "{got:?}"
        );
    }

    #[test]
    fn decimal_rounding_helpers() {
        let mut sr = stats("v", "Float", 10, 0, "0", "1", &[]);
        let mut decimals = |p: &str| {
            sr.addl_cols
                .insert("max_precision".to_string(), p.to_string());
            float_decimals(&sr)
        };
        assert_eq!(decimals("2"), Some(2));
        assert_eq!(decimals("15"), Some(15));
        assert_eq!(decimals("17"), None, "float noise is left unrounded");

        assert_eq!(round_to(1.234_5, Some(2)), 1.23);
        let zero = round_to(-0.004, Some(2));
        assert!(zero == 0.0 && zero.is_sign_positive(), "no negative zero");
        assert_eq!(format_number(-0.004, false, Some(2)), "0");
        assert_eq!(round_to(1.234_5, None), 1.234_5);
        assert_eq!(round_to(1e300, Some(2)), 1e300, "too large to round");
        // Too close to an f64's spacing to round exactly: left as is rather
        // than moved a step off a real value.
        let edge = 4.000_000_000_000_005;
        assert_eq!(round_to(edge, Some(15)), edge);
        assert_eq!(floor_to(edge, Some(15)), edge);
        assert_eq!(round_at_least(edge, edge, Some(15)), edge);
        assert_eq!(
            round_to(0.123_456_789_012_345, Some(15)),
            0.123_456_789_012_345
        );
        // Whole numbers round exactly all the way to 2^53, so an Integer
        // member's cap and value stay whole above the decimal cutoff.
        let big = 1_125_899_906_842_624.75;
        assert_eq!(floor_to(big, Some(0)), 1_125_899_906_842_624.0);
        assert_eq!(round_at_least(big, big, Some(0)), 1_125_899_906_842_625.0);

        // A cap is floored to the grid, never rounded past the real bound.
        assert_eq!(floor_to(10.47, Some(1)), 10.4);
        assert_eq!(floor_to(0.29, Some(2)), 0.29, "0.29 * 100 is 28.999…");
        assert_eq!(floor_to(-1.25, Some(1)), -1.3);
        assert_eq!(floor_to(7.9, Some(0)), 7.0);

        // A follower moves up a step rather than print below its predecessor.
        assert_eq!(round_at_least(10.43, 10.42, Some(1)), 10.5);
        assert_eq!(round_at_least(10.46, 10.42, Some(1)), 10.5);
        assert_eq!(round_at_least(10.44, 10.4, Some(1)), 10.4);
        assert_eq!(round_at_least(3.2, 3.1, Some(0)), 4.0);
    }

    #[test]
    fn null_ratio_is_reproduced_within_tolerance() {
        let generator = ColumnGenerator::LoremFallback {
            locale:       Locale::EN,
            null_ratio:   0.3,
            length_stats: None,
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
        let generator = try_frequency_weighted(&refs, 0.0, 1, &mut None).unwrap();

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
        assert!(try_frequency_weighted(&refs, 0.0, 1, &mut None).is_none());
    }

    #[test]
    fn frequency_weighted_drops_values_below_min_count() {
        let freqs = [
            freq("c", "A", 70, 1.0),
            freq("c", "B", 30, 2.0),
            freq("c", "RARE", 4, 3.0),
        ];
        let refs: Vec<&FrequencyRecord> = freqs.iter().collect();
        let generator = try_frequency_weighted(&refs, 0.0, 5, &mut None).unwrap();

        let mut rng = StdRng::seed_from_u64(11); // DevSkim: ignore DS148264
        let n = 20_000;
        let mut a = 0;
        for _ in 0..n {
            match generator.next(&mut rng).as_str() {
                "A" => a += 1,
                "B" => {},
                other => panic!("value below --min-count emitted: {other:?}"),
            }
        }
        // RARE's weight is redistributed: A is 70 / (70 + 30), not 70 / 104.
        #[allow(clippy::cast_precision_loss)]
        let ratio = f64::from(a) / f64::from(n);
        assert!((ratio - 0.7).abs() < 0.02, "A ratio {ratio} not near 0.7");
    }

    #[test]
    fn frequency_weighted_returns_none_when_all_values_below_min_count() {
        let freqs = [freq("c", "A", 4, 1.0), freq("c", "B", 1, 2.0)];
        let refs: Vec<&FrequencyRecord> = freqs.iter().collect();
        assert!(try_frequency_weighted(&refs, 0.0, 5, &mut None).is_none());
        // The threshold is inclusive: count == min_count is kept.
        assert!(try_frequency_weighted(&refs, 0.0, 4, &mut None).is_some());
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
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "unknown",
            1000,
            5000,
            Locale::EN,
            false,
            1,
            &mut rng,
        );

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
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "city",
            1000,
            5000,
            Locale::EN,
            false,
            1,
            &mut rng,
        );

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
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "unknown",
            500,
            1000,
            Locale::EN,
            false,
            1,
            &mut rng,
        );

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
            false,
            1,
            &mut build_rng,
        );

        let mut rng1 = StdRng::seed_from_u64(123); // DevSkim: ignore DS148264
        let mut rng2 = StdRng::seed_from_u64(123); // DevSkim: ignore DS148264
        for _ in 0..200 {
            assert_eq!(generator.next(&mut rng1), generator.next(&mut rng2));
        }
    }

    // ---- Length-stats: parsing -------------------------------------------

    #[test]
    fn length_stats_from_addl_cols_present() {
        let mut addl = IndexMap::new();
        addl.insert("min_length".to_string(), "5".to_string());
        addl.insert("max_length".to_string(), "50".to_string());
        addl.insert("avg_length".to_string(), "20.5".to_string());
        addl.insert("stddev_length".to_string(), "7.25".to_string());

        let ls = LengthStats::from_addl_cols(&addl).expect("should parse");
        assert_eq!(ls.min, 5);
        assert_eq!(ls.max, 50);
        assert!((ls.avg - 20.5).abs() < 1e-9);
        assert!((ls.stddev - 7.25).abs() < 1e-9);
    }

    #[test]
    fn length_stats_from_addl_cols_missing_returns_none() {
        let addl: IndexMap<String, String> = IndexMap::new();
        assert!(LengthStats::from_addl_cols(&addl).is_none());

        let mut only_min = IndexMap::new();
        only_min.insert("min_length".to_string(), "5".to_string());
        assert!(LengthStats::from_addl_cols(&only_min).is_none());
    }

    #[test]
    fn length_stats_from_addl_cols_rejects_empty_or_inverted_range() {
        let mut max_zero = IndexMap::new();
        max_zero.insert("min_length".to_string(), "0".to_string());
        max_zero.insert("max_length".to_string(), "0".to_string());
        assert!(LengthStats::from_addl_cols(&max_zero).is_none());

        let mut inverted = IndexMap::new();
        inverted.insert("min_length".to_string(), "10".to_string());
        inverted.insert("max_length".to_string(), "5".to_string());
        assert!(LengthStats::from_addl_cols(&inverted).is_none());
    }

    #[test]
    fn length_stats_stddev_optional_with_midpoint_avg_fallback() {
        let mut addl = IndexMap::new();
        addl.insert("min_length".to_string(), "10".to_string());
        addl.insert("max_length".to_string(), "30".to_string());

        let ls = LengthStats::from_addl_cols(&addl).expect("should parse");
        assert_eq!(ls.min, 10);
        assert_eq!(ls.max, 30);
        // No avg_length → fallback to midpoint (10+30)/2 = 20.
        assert!((ls.avg - 20.0).abs() < 1e-9);
        // No stddev_length → 0.0.
        assert!((ls.stddev - 0.0).abs() < 1e-9);
    }

    // ---- Length-stats: target-length sampling ----------------------------

    #[test]
    fn sample_target_length_normal_respects_bounds_and_mean() {
        let stats = LengthStats {
            min:    5,
            max:    50,
            avg:    20.0,
            stddev: 8.0,
        };
        let mut rng = StdRng::seed_from_u64(2024); // DevSkim: ignore DS148264
        let n = 10_000;
        let mut sum = 0_usize;
        for _ in 0..n {
            let t = sample_target_length(&stats, &mut rng);
            assert!((5..=50).contains(&t), "sample {t} out of [5, 50]");
            sum += t;
        }
        #[allow(clippy::cast_precision_loss)]
        let mean = sum as f64 / n as f64;
        // Normal(20, 8) clamped to [5, 50] still has its mass near 20.
        // Allow ±1.5 — tight enough to detect any real shift.
        assert!((mean - 20.0).abs() < 1.5, "mean {mean} not near 20.0");
    }

    #[test]
    fn sample_target_length_uniform_when_stddev_zero() {
        let stats = LengthStats {
            min:    10,
            max:    20,
            avg:    15.0,
            stddev: 0.0,
        };
        let mut rng = StdRng::seed_from_u64(99); // DevSkim: ignore DS148264
        let mut saw_low = false;
        let mut saw_high = false;
        for _ in 0..2000 {
            let t = sample_target_length(&stats, &mut rng);
            assert!((10..=20).contains(&t), "sample {t} out of [10, 20]");
            if t <= 11 {
                saw_low = true;
            }
            if t >= 19 {
                saw_high = true;
            }
        }
        assert!(
            saw_low && saw_high,
            "uniform fallback should cover both ends of [10, 20]"
        );
    }

    #[test]
    fn sample_target_length_min_max_equal_returns_value() {
        let stats = LengthStats {
            min:    7,
            max:    7,
            avg:    7.0,
            stddev: 0.0,
        };
        let mut rng = StdRng::seed_from_u64(1); // DevSkim: ignore DS148264
        for _ in 0..50 {
            assert_eq!(sample_target_length(&stats, &mut rng), 7);
        }
    }

    // ---- Length-stats: generator integration -----------------------------

    #[test]
    fn lorem_fallback_respects_length_stats() {
        let record = stats(
            "blurb",
            "String",
            1000,
            0,
            "",
            "",
            &[
                ("min_length", "8"),
                ("max_length", "40"),
                ("avg_length", "20"),
                ("stddev_length", "5"),
            ],
        );
        let mut build_rng = StdRng::seed_from_u64(17); // DevSkim: ignore DS148264
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "unknown",
            1000,
            5000,
            Locale::EN,
            false,
            1,
            &mut build_rng,
        );

        // Sanity: this should be a LoremFallback variant with length stats set.
        match &generator {
            ColumnGenerator::LoremFallback {
                length_stats: Some(_),
                ..
            } => {},
            _ => panic!("expected LoremFallback with length_stats set"),
        }

        let mut rng = StdRng::seed_from_u64(2025); // DevSkim: ignore DS148264
        let n = 2000;
        let mut total_chars = 0_usize;
        for _ in 0..n {
            let value = generator.next(&mut rng);
            let len = value.chars().count();
            assert!(
                (1..=40).contains(&len),
                "value {value:?} has {len} chars, expected in [1, 40]"
            );
            total_chars += len;
        }
        #[allow(clippy::cast_precision_loss)]
        let mean = total_chars as f64 / n as f64;
        // Lorem fakers can't go below their natural minimum (~4 words),
        // so the empirical mean shifts a bit above avg_length=20.
        // Allow a generous ±30% band.
        assert!(
            (14.0..=26.0).contains(&mean),
            "mean length {mean} not within ±30% of avg_length=20"
        );
    }

    #[test]
    fn structured_faker_ignores_length_stats() {
        let record = stats(
            "email",
            "String",
            10_000,
            0,
            "",
            "",
            &[
                ("min_length", "3"),
                ("max_length", "3"),
                ("avg_length", "3"),
                ("stddev_length", "0"),
            ],
        );
        let mut build_rng = StdRng::seed_from_u64(31); // DevSkim: ignore DS148264
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "email",
            10_000,
            5000,
            Locale::EN,
            false,
            1,
            &mut build_rng,
        );

        // The Faker variant for "email" must NOT carry length_stats —
        // truncating an email would destroy the @ structure.
        match &generator {
            ColumnGenerator::Faker {
                content_type,
                length_stats,
                ..
            } => {
                assert_eq!(content_type, "email");
                assert!(
                    length_stats.is_none(),
                    "structured faker must not carry length_stats"
                );
            },
            _ => panic!("expected Faker variant for 'email' content_type"),
        }

        let mut rng = StdRng::seed_from_u64(7); // DevSkim: ignore DS148264
        for _ in 0..50 {
            let value = generator.next(&mut rng);
            assert!(
                value.contains('@'),
                "email '{value}' should retain @ — must not be truncated"
            );
        }
    }

    #[test]
    fn bounded_pool_ignores_length_stats() {
        // Low-cardinality faker column (pool path).
        let record = stats(
            "city",
            "String",
            5,
            0,
            "",
            "",
            &[
                ("min_length", "3"),
                ("max_length", "3"),
                ("avg_length", "3"),
                ("stddev_length", "0"),
            ],
        );
        let mut build_rng = StdRng::seed_from_u64(55); // DevSkim: ignore DS148264
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "city",
            1000,
            5000,
            Locale::EN,
            false,
            1,
            &mut build_rng,
        );

        // Pooled faker variant — length_stats must be None.
        match &generator {
            ColumnGenerator::Faker {
                pool: Some(_),
                length_stats,
                ..
            } => assert!(
                length_stats.is_none(),
                "pooled faker must not carry length_stats"
            ),
            _ => panic!("expected pooled Faker variant for low-cardinality 'city'"),
        }

        // Output values come from the pool verbatim — they will exceed the
        // bogus max_length=3 we set above, proving no truncation happened.
        let mut rng = StdRng::seed_from_u64(11); // DevSkim: ignore DS148264
        let mut saw_longer_than_three = false;
        for _ in 0..200 {
            if generator.next(&mut rng).chars().count() > 3 {
                saw_longer_than_three = true;
                break;
            }
        }
        assert!(
            saw_longer_than_three,
            "pool values should not be truncated to max_length=3"
        );
    }

    #[test]
    fn unstructured_pooled_faker_truncates_to_length_stats() {
        // Bounded-cardinality FREE-TEXT column: the pool path is taken (since
        // cardinality < requested_rows), AND length stats must still apply —
        // otherwise the pre-generated pool of full lorem sentences would
        // bypass truncation and produce 60-100 char strings for a column
        // whose source caps at 25.
        let record = stats(
            "blurb",
            "String",
            10,
            0,
            "",
            "",
            &[
                ("min_length", "5"),
                ("max_length", "25"),
                ("avg_length", "15"),
                ("stddev_length", "4"),
            ],
        );
        let mut build_rng = StdRng::seed_from_u64(81); // DevSkim: ignore DS148264
        let generator = ColumnGenerator::build(
            &record,
            &[],
            "free_text",
            1000,
            5000,
            Locale::EN,
            false,
            1,
            &mut build_rng,
        );

        // Verify the variant carries length_stats AND a pool — this is the
        // unstructured + bounded-cardinality path.
        match &generator {
            ColumnGenerator::Faker {
                content_type,
                pool: Some(_),
                length_stats: Some(_),
                ..
            } => assert_eq!(content_type, "free_text"),
            _ => panic!(
                "expected pooled Faker variant with length_stats for unstructured 'free_text'"
            ),
        }

        let mut rng = StdRng::seed_from_u64(2026); // DevSkim: ignore DS148264
        for _ in 0..500 {
            let value = generator.next(&mut rng);
            let len = value.chars().count();
            assert!(
                (1..=25).contains(&len),
                "value {value:?} has {len} chars; expected in [1, 25]"
            );
        }
    }
}
