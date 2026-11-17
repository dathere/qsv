//! `--min-count` range trimming for numeric and date/datetime columns.
//!
//! Type-based generators draw from `[stats.min, stats.max]`, so a lone outlier
//! (a top salary, an earliest date) reappears at or near its real value. This
//! module replaces each range end with the k-th smallest / k-th largest value
//! (k = `--min-count`), so at least k source rows lie at or beyond each end.
//! The new bounds are written back into the `StatsRecord`, so every consumer of
//! `numeric_quartile_buckets` / `date_quartile_buckets` (independent columns
//! and `ordered` / `correlated` groups) sees the trimmed range.

use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{
    CliResult,
    cmd::describegpt::dictionary::StatsRecord,
    config::{Config, Delimiter},
};

#[derive(Clone, Copy)]
enum RangeKind {
    Integer,
    Float,
    Date,
    DateTime,
}

impl RangeKind {
    fn from_type(t: &str) -> Option<Self> {
        match t {
            "Integer" => Some(Self::Integer),
            "Float" => Some(Self::Float),
            "Date" => Some(Self::Date),
            "DateTime" => Some(Self::DateTime),
            _ => None,
        }
    }

    /// Parse a raw cell to a sortable key: the number itself, or seconds since
    /// the UNIX epoch for dates (parsed with the same parser and DMY preference
    /// `stats` used to type the column).
    fn key(self, raw: &str, prefer_dmy: bool) -> Option<f64> {
        let s = raw.trim();
        if s.is_empty() {
            return None;
        }
        match self {
            Self::Integer | Self::Float => s.parse::<f64>().ok().filter(|v| v.is_finite()),
            Self::Date | Self::DateTime =>
            {
                #[allow(clippy::cast_precision_loss)]
                qsv_dateparser::parse_with_preference(s, prefer_dmy)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            },
        }
    }

    /// Format a key back into the form `parse_f64` / `parse_epoch` read.
    #[allow(clippy::cast_possible_truncation)]
    fn format(self, key: f64) -> String {
        match self {
            Self::Integer => format!("{}", key as i64),
            Self::Float => format!("{key}"),
            Self::Date | Self::DateTime => {
                let Some(dt) = chrono::DateTime::from_timestamp(key as i64, 0) else {
                    return String::new();
                };
                if matches!(self, Self::Date) {
                    dt.format("%Y-%m-%d").to_string()
                } else {
                    dt.to_rfc3339()
                }
            },
        }
    }
}

/// `f64` with a total order, for the bounded heaps.
#[derive(Clone, Copy, PartialEq)]
struct Key(f64);
impl Eq for Key {}
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

/// Keeps the k smallest and k largest keys seen, plus the non-null count.
struct Extremes {
    k:        usize,
    smallest: BinaryHeap<Key>, // max-heap of the k smallest
    largest:  BinaryHeap<std::cmp::Reverse<Key>>, // min-heap of the k largest
    count:    u64,
}

impl Extremes {
    fn new(k: usize) -> Self {
        Self {
            k,
            smallest: BinaryHeap::new(),
            largest: BinaryHeap::new(),
            count: 0,
        }
    }

    fn add(&mut self, v: f64) {
        self.count += 1;
        self.smallest.push(Key(v));
        if self.smallest.len() > self.k {
            self.smallest.pop();
        }
        self.largest.push(std::cmp::Reverse(Key(v)));
        if self.largest.len() > self.k {
            self.largest.pop();
        }
    }

    /// The trimmed `(lo, hi)`: the k-th smallest and k-th largest values, so at
    /// least k rows lie at or beyond each end. Needs at least 2k-1 non-null
    /// values; with fewer the two cross and no interval satisfies the rule, so
    /// this returns `None`.
    fn bounds(&self) -> Option<(f64, f64)> {
        // Enforced on the count, not on whether the ends cross: with ties the
        // k-th smallest and k-th largest can meet on fewer than 2k-1 values.
        let needed = (self.k as u64).saturating_mul(2).saturating_sub(1);
        if self.count < needed {
            return None;
        }
        let lo = self.smallest.peek()?.0;
        let hi = self.largest.peek()?.0.0;
        Some((lo, hi))
    }
}

/// Map each stats record to its column in the source CSV by header name.
/// Stats records are not positional: `--stats-options` can `--select` a subset
/// or reorder columns. Duplicate names are matched by occurrence. `None` means
/// the record has no source column.
pub(crate) fn source_column_indices(
    stats_records: &[StatsRecord],
    headers: &csv::StringRecord,
) -> Vec<Option<usize>> {
    let mut used = vec![false; headers.len()];
    stats_records
        .iter()
        .map(|sr| {
            let idx = headers
                .iter()
                .enumerate()
                .find(|(i, h)| !used[*i] && *h == sr.field)
                .map(|(i, _)| i)?;
            used[idx] = true;
            Some(idx)
        })
        .collect()
}

/// Trim every numeric/date column's `[min, max]` to its k-th smallest and
/// k-th largest values (k = `min_count`) with one streaming pass over `input`.
/// A column with fewer than 2k-1 non-null values gets an empty range, so its
/// type-based generator emits nothing (with a warning).
pub(crate) fn trim_range_bounds(
    stats_records: &mut [StatsRecord],
    source_cols: &[Option<usize>],
    input_path: &str,
    delimiter: Option<Delimiter>,
    min_count: u64,
    prefer_dmy: bool,
) -> CliResult<()> {
    if min_count <= 1 {
        return Ok(());
    }
    if !stats_records
        .iter()
        .any(|sr| RangeKind::from_type(&sr.r#type).is_some())
    {
        return Ok(());
    }

    let config = Config::new(Some(&input_path.to_string())).delimiter(delimiter);
    let mut rdr = config.reader()?;

    let targets: Vec<(usize, usize, RangeKind)> = stats_records
        .iter()
        .enumerate()
        .filter_map(|(stats_idx, sr)| {
            let kind = RangeKind::from_type(&sr.r#type)?;
            let src_idx = source_cols.get(stats_idx).copied().flatten()?;
            Some((stats_idx, src_idx, kind))
        })
        .collect();

    let k = usize::try_from(min_count).unwrap_or(usize::MAX);
    let mut extremes: Vec<Extremes> = targets.iter().map(|_| Extremes::new(k)).collect();

    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        for ((_, src_idx, kind), ext) in targets.iter().zip(extremes.iter_mut()) {
            if let Some(v) = record
                .get(*src_idx)
                .and_then(|raw| kind.key(raw, prefer_dmy))
            {
                ext.add(v);
            }
        }
    }

    for ((stats_idx, _, kind), ext) in targets.iter().zip(&extremes) {
        let sr = &mut stats_records[*stats_idx];
        if ext.count == 0 {
            continue;
        }
        if let Some((lo, hi)) = ext.bounds() {
            log::info!(
                "synthesize: column '{}' range trimmed by --min-count ({min_count}) from [{}, {}] \
                 to [{}, {}]",
                sr.field,
                sr.min,
                sr.max,
                kind.format(lo),
                kind.format(hi)
            );
            sr.min = kind.format(lo);
            sr.max = kind.format(hi);
        } else {
            wwarn!(
                "synthesize: column '{}' has only {} non-null value(s); trimming its range by \
                 --min-count ({min_count}) needs at least {}, so it will be left empty unless its \
                 real values are sampled",
                sr.field,
                ext.count,
                min_count.saturating_mul(2).saturating_sub(1)
            );
            sr.min = String::new();
            sr.max = String::new();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds_of(values: &[f64], k: usize) -> Option<(f64, f64)> {
        let mut e = Extremes::new(k);
        for &v in values {
            e.add(v);
        }
        e.bounds()
    }

    #[test]
    fn trims_to_kth_values() {
        let values: Vec<f64> = (1..=20).map(f64::from).chain([1_000_000.0]).collect();
        // 5th smallest = 5, 5th largest = 17 (the outlier and 18..20 are cut).
        assert_eq!(bounds_of(&values, 5), Some((5.0, 17.0)));
    }

    #[test]
    fn k_one_keeps_min_and_max() {
        assert_eq!(bounds_of(&[3.0, -2.0, 9.0], 1), Some((-2.0, 9.0)));
    }

    #[test]
    fn fewer_than_2k_minus_1_values_has_no_bounds() {
        // k = 5 needs 9 values; below that the k-th smallest passes the k-th
        // largest and no range keeps 5 rows outward of both ends.
        for n in 1..=8 {
            let values: Vec<f64> = (1..=n).map(f64::from).collect();
            assert_eq!(bounds_of(&values, 5), None, "n = {n}");
        }
    }

    #[test]
    fn ties_do_not_bypass_the_2k_minus_1_minimum() {
        // 8 values with a tie at 4: the 5th smallest and 5th largest both land
        // on 4, so the ends don't cross, but 8 < 9 values must still fail.
        assert_eq!(
            bounds_of(&[1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 6.0, 7.0], 5),
            None
        );
    }

    #[test]
    fn exactly_2k_minus_1_values_collapses_to_the_median() {
        let values: Vec<f64> = (1..=9).map(f64::from).collect();
        assert_eq!(bounds_of(&values, 5), Some((5.0, 5.0)));
    }

    #[test]
    fn date_keys_round_trip() {
        let k = RangeKind::Date.key("2024-03-15", false).unwrap();
        assert_eq!(RangeKind::Date.format(k), "2024-03-15");
        let k = RangeKind::Date.key("03/15/2024", false).unwrap();
        assert_eq!(RangeKind::Date.format(k), "2024-03-15");
    }
}
