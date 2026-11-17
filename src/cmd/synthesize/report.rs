//! `--report`: a JSON summary of how much of the source a run reproduced.

use std::collections::HashSet;

use serde::Serialize;
use xxhash_rust::xxh3::Xxh3;

use super::range_trim::RangeKind;
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
}

/// Hashes a row by value, so the same value printed differently (`10.50` vs
/// `10.5`, `01/02/2020` vs `2020-01-02`) still counts as a match. Columns with
/// no source column are left out on both sides.
pub(crate) struct RowKeyer {
    /// (stats index, source index, numeric/date kind).
    cols:       Vec<(usize, usize, Option<RangeKind>)>,
    prefer_dmy: bool,
}

impl RowKeyer {
    pub(crate) fn new(
        stats_records: &[StatsRecord],
        source_cols: &[Option<usize>],
        prefer_dmy: bool,
    ) -> Self {
        let cols = stats_records
            .iter()
            .enumerate()
            .filter_map(|(i, sr)| {
                let src = source_cols.get(i).copied().flatten()?;
                Some((i, src, RangeKind::from_type(&sr.r#type)))
            })
            .collect();
        Self { cols, prefer_dmy }
    }

    /// Stats indices of the columns compared.
    pub(crate) fn compared(&self) -> impl Iterator<Item = usize> + '_ {
        self.cols.iter().map(|&(i, ..)| i)
    }

    fn key<'a>(&self, cell: impl Fn(usize, usize) -> &'a str) -> u128 {
        let mut h = Xxh3::new();
        for &(stats_idx, src_idx, kind) in &self.cols {
            let raw = cell(stats_idx, src_idx);
            match kind.and_then(|k| k.key(raw, self.prefer_dmy)) {
                Some(v) => {
                    // + 0.0 folds -0.0 into 0.0.
                    h.update(&[1]);
                    h.update(&(v + 0.0).to_bits().to_le_bytes());
                },
                None => {
                    h.update(&[0]);
                    h.update(&(raw.len() as u64).to_le_bytes());
                    h.update(raw.as_bytes());
                },
            }
        }
        h.digest128()
    }

    /// Key of a source record (indexed by source column).
    pub(crate) fn source_key(&self, record: &csv::StringRecord) -> u128 {
        self.key(|_, src| record.get(src).unwrap_or_default())
    }

    /// Key of a synthetic row (indexed by stats column).
    pub(crate) fn synthetic_key(&self, row: &[String]) -> u128 {
        self.key(|stats, _| row[stats].as_str())
    }
}

/// One pass over the source: the set of row keys.
pub(crate) fn source_row_keys(
    keyer: &RowKeyer,
    input_path: &str,
    delimiter: Option<Delimiter>,
) -> CliResult<HashSet<u128>> {
    let mut rdr = Config::new(Some(&input_path.to_string()))
        .delimiter(delimiter)
        .reader()?;
    let mut keys = HashSet::new();
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        keys.insert(keyer.source_key(&record));
    }
    Ok(keys)
}
