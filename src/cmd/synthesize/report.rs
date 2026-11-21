//! `--report`: a JSON summary of how much of the source a run reproduced.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use chrono::NaiveTime;
use serde::Serialize;
use xxhash_rust::xxh3::Xxh3;

use super::{
    generator::ColumnGenerator, group_generator::GroupGenerator, range_trim::RangeKind,
    relationships::EmitUnit,
};
use crate::{
    CliResult,
    cmd::describegpt::dictionary::StatsRecord,
    config::{Config, Delimiter},
};

/// Values or `joint` tuples dropped by `--min-count`.
#[derive(Serialize)]
pub(crate) struct Suppression {
    pub(crate) columns:  Vec<String>,
    /// Distinct values (or tuples) dropped.
    pub(crate) distinct: usize,
    /// Source rows holding a dropped value (or tuple).
    pub(crate) rows:     u64,
    /// With `--target`, the class whose profile dropped them; `None` for the
    /// whole-input profile and for dropped classes themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) class:    Option<String>,
}

/// Hashes a row by value, so the same value printed differently (`10.50` vs
/// `10.5`, `01/02/2020` vs `2020-01-02`) still counts as a match. Columns with
/// no source column are left out on both sides.
pub(crate) struct RowKeyer {
    cols:       Vec<KeyedCol>,
    prefer_dmy: bool,
}

struct KeyedCol {
    stats_idx:   usize,
    src_idx:     usize,
    kind:        Option<RangeKind>,
    /// The `date:<fmt>` output format the generator writes this column in.
    date_format: Option<String>,
}

impl RowKeyer {
    pub(crate) fn new(
        stats_records: &[StatsRecord],
        source_cols: &[Option<usize>],
        schedule: &[EmitUnit],
        prefer_dmy: bool,
    ) -> Self {
        let mut formats: Vec<Option<String>> = vec![None; stats_records.len()];
        for unit in schedule {
            match unit {
                EmitUnit::Independent {
                    col_idx,
                    generator: ColumnGenerator::DateQuantile { date_format, .. },
                } => formats[*col_idx].clone_from(date_format),
                EmitUnit::Group(GroupGenerator::Ordered {
                    member_cols,
                    date_format,
                    ..
                }) => {
                    for &c in member_cols {
                        formats[c].clone_from(date_format);
                    }
                },
                _ => {},
            }
        }
        let cols = stats_records
            .iter()
            .enumerate()
            .filter_map(|(i, sr)| {
                Some(KeyedCol {
                    stats_idx:   i,
                    src_idx:     source_cols.get(i).copied().flatten()?,
                    kind:        RangeKind::from_type(&sr.r#type),
                    date_format: formats[i].take(),
                })
            })
            .collect();
        Self { cols, prefer_dmy }
    }

    /// Stats indices of the columns compared.
    pub(crate) fn compared(&self) -> impl Iterator<Item = usize> + '_ {
        self.cols.iter().map(|c| c.stats_idx)
    }

    fn key<'a>(&self, cell: impl Fn(&KeyedCol) -> &'a [u8]) -> u128 {
        let mut h = Xxh3::new();
        for col in &self.cols {
            // Lossily, as synthetic values copied from a non-UTF8 source cell
            // are written (U+FFFD for each invalid sequence).
            let text = String::from_utf8_lossy(cell(col));
            let raw = text.as_bytes();
            let parsed = col.kind.and_then(|kind| {
                value_key(
                    kind,
                    text.trim(),
                    col.date_format.as_deref(),
                    self.prefer_dmy,
                )
            });
            match parsed {
                Some((tag, a, b)) => {
                    h.update(&[tag]);
                    h.update(&a.to_le_bytes());
                    h.update(&b.to_le_bytes());
                },
                None => {
                    h.update(&[0]);
                    h.update(&(raw.len() as u64).to_le_bytes());
                    h.update(raw);
                },
            }
        }
        h.digest128()
    }

    /// Key of a source record (indexed by source column).
    pub(crate) fn source_key(&self, record: &csv::ByteRecord) -> u128 {
        self.key(|c| record.get(c.src_idx).unwrap_or_default())
    }

    /// Key of a synthetic row (indexed by stats column).
    pub(crate) fn synthetic_key(&self, row: &[String]) -> u128 {
        self.key(|c| row[c.stats_idx].as_bytes())
    }
}

/// A cell's value as (tag, a, b). Integers stay exact as i64 instead of
/// rounding through f64, and dates keep their fractional seconds. A date is
/// read with the column's output format first, so a generated `03/02/2020`
/// under `date:%d/%m/%Y` is February 3 whatever the source's DMY preference.
fn value_key(
    kind: RangeKind,
    s: &str,
    date_format: Option<&str>,
    prefer_dmy: bool,
) -> Option<(u8, i64, i64)> {
    if s.is_empty() {
        return None;
    }
    match kind {
        RangeKind::Integer | RangeKind::Float => {
            if let Ok(i) = s.parse::<i64>() {
                return Some((2, i, 0));
            }
            let v = s.parse::<f64>().ok().filter(|v| v.is_finite())?;
            // A whole float below 2^53 keys like the integer, so `10.0`
            // matches `10`; + 0.0 folds -0.0 into 0.0.
            #[allow(clippy::cast_possible_truncation)]
            if v.fract() == 0.0 && v.abs() < 9.0e15 {
                return Some((2, v as i64, 0));
            }
            Some((1, (v + 0.0).to_bits().cast_signed(), 0))
        },
        RangeKind::Date | RangeKind::DateTime => {
            let dt = parse_date(s, date_format, prefer_dmy)?;
            Some((3, dt.timestamp(), i64::from(dt.timestamp_subsec_nanos())))
        },
    }
}

/// Reads a date with the column's output format first, falling back to the
/// date parser `stats` uses, so a generated `03/02/2020` under
/// `date:%d/%m/%Y` is February 3 whatever the source's DMY preference.
pub(crate) fn parse_date(
    s: &str,
    date_format: Option<&str>,
    prefer_dmy: bool,
) -> Option<chrono::DateTime<chrono::Utc>> {
    let formatted = date_format.and_then(|fmt| {
        chrono::NaiveDateTime::parse_from_str(s, fmt)
            .or_else(|_| {
                chrono::NaiveDate::parse_from_str(s, fmt).map(|d| d.and_time(NaiveTime::MIN))
            })
            .ok()
            .map(|dt| dt.and_utc())
    });
    match formatted {
        Some(dt) => Some(dt),
        None => qsv_dateparser::parse_with_preference(s, prefer_dmy).ok(),
    }
}

/// One pass over the source: the set of row keys. Reads bytes, so a cell that
/// is not UTF-8 is keyed lossily instead of failing the run.
pub(crate) fn source_row_keys(
    keyer: &RowKeyer,
    input_path: &str,
    delimiter: Option<Delimiter>,
) -> CliResult<HashSet<u128>> {
    let mut rdr = Config::new(Some(&input_path.to_string()))
        .delimiter(delimiter)
        .reader()?;
    let mut keys = HashSet::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        keys.insert(keyer.source_key(&record));
    }
    Ok(keys)
}

/// A path's identity for clash checks: canonical when it exists, else its
/// canonical parent joined with the file name, so `./out.json` and `out.json`
/// compare equal before either is written.
pub(crate) fn resolve_path(p: &str) -> PathBuf {
    let path = Path::new(p);
    if let Ok(c) = path.canonicalize() {
        return c;
    }
    let parent = match path.parent() {
        Some(d) if !d.as_os_str().is_empty() => d,
        _ => Path::new("."),
    };
    match (parent.canonicalize(), path.file_name()) {
        (Ok(dir), Some(name)) => dir.join(name),
        _ => path.to_path_buf(),
    }
}
