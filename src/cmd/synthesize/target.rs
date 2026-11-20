//! `--target <col>`: conditional generation.
//!
//! The input is split into one temporary file per class of the target column,
//! every other column is fitted once per class, and each synthetic row first
//! draws a class (by its share of source rows), then fills the row from that
//! class's schedule. A target with at most `MAX_CLASSES` distinct values is
//! split by value; a numeric target with more is split by its own (whole-input)
//! quantile buckets, and its value is drawn from the class's bucket (from the
//! written values that land back in it), each class at its share of source
//! rows.

use std::{collections::HashMap, fs::File, path::PathBuf};

use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

use super::{generator::format_number, relationships::EmitUnit};
use crate::{CliResult, config::Config};

/// Most distinct values a target is split by. A numeric target with more is
/// binned by its quantile buckets; any other is refused.
pub(crate) const MAX_CLASSES: usize = 50;

/// The fitted target: its column and one profile per class.
#[derive(Serialize, Deserialize)]
pub(crate) struct Target {
    pub(crate) col_idx:  usize,
    /// Binned targets: how a value drawn from a class's bucket is written.
    pub(crate) is_int:   bool,
    pub(crate) decimals: Option<u32>,
    pub(crate) classes:  Vec<Class>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Class {
    /// The target value the class holds (trimmed, as `frequency` reads it);
    /// empty for the null class. For a binned class, its bucket as `lo..hi`.
    pub(crate) label:    String,
    /// Binned classes only: the bucket the class's target values are drawn
    /// from.
    pub(crate) bucket:   Option<(f64, f64)>,
    /// Binned classes of a rounded target: the written values that land back
    /// in the bucket (see `grid`), drawn instead of the bucket itself.
    pub(crate) grid:     Option<(i64, i64)>,
    /// Source rows in the class; its weight when a row draws a class.
    pub(crate) rows:     u64,
    /// Generators for every column except the target.
    pub(crate) schedule: Vec<EmitUnit>,
}

impl Target {
    /// Normalized cumulative class weights, ascending, last element == 1.0.
    pub(crate) fn cumulative(&self) -> Vec<f64> {
        let total: u64 = self.classes.iter().map(|c| c.rows).sum();
        let mut acc = 0_u64;
        self.classes
            .iter()
            .map(|c| {
                acc += c.rows;
                #[allow(clippy::cast_precision_loss)]
                {
                    acc as f64 / total as f64
                }
            })
            .collect()
    }

    /// Draws a class: its target value for this row and its schedule.
    pub(crate) fn draw(&self, cumulative: &[f64], rng: &mut StdRng) -> (String, &[EmitUnit]) {
        let r = rng.random_range(0.0..1.0);
        let idx = cumulative
            .partition_point(|&c| c < r)
            .min(self.classes.len() - 1);
        let class = &self.classes[idx];
        let value = match (class.grid, scale(self.is_int, self.decimals), class.bucket) {
            (Some((a, b)), Some(scale), _) => {
                #[allow(clippy::cast_precision_loss)]
                let v = rng.random_range(a..=b) as f64 / scale;
                format_number(v, self.is_int, self.decimals)
            },
            (_, _, Some((lo, hi))) => {
                let v = if lo < hi {
                    rng.random_range(lo..hi)
                } else {
                    lo
                };
                format_number(v, self.is_int, self.decimals)
            },
            _ => class.label.clone(),
        };
        (value, &class.schedule)
    }
}

/// RNG for one class's fit, keyed on the base seed and the class index only.
pub(crate) fn class_rng(base_seed: u64, class: usize) -> StdRng {
    let mut key = [0_u8; 32];
    key[..8].copy_from_slice(&base_seed.to_le_bytes());
    key[8..16].copy_from_slice(&(class as u64).to_le_bytes());
    key[16..24].copy_from_slice(b"qsvclass");
    StdRng::from_seed(key)
}

/// How many written steps make one unit: 1 for an Integer target, 10^decimals
/// for a rounded Float one; `None` for a Float written unrounded.
fn scale(is_int: bool, decimals: Option<u32>) -> Option<f64> {
    if is_int {
        return Some(1.0);
    }
    let scale = 10_f64.powi(i32::try_from(decimals?).ok()?);
    scale.is_finite().then_some(scale)
}

/// The written values of a binned class, as a range of steps (see `scale`):
/// those that, written and read back, land in the bucket under `bin_of`, i.e.
/// above `lo` (or at it, for the first bucket, `!open_low`) and at most `hi`.
/// Drawing from it keeps every row's target in the class its other columns
/// were drawn for, which a value drawn from the bucket and then rounded
/// doesn't. `None` for a target written unrounded, or when no written value
/// lands in the bucket.
pub(crate) fn grid(
    (lo, hi): (f64, f64),
    open_low: bool,
    is_int: bool,
    decimals: Option<u32>,
) -> Option<(i64, i64)> {
    // Steps are exact integers in an f64 below 2^53.
    const LIMIT: f64 = 9_007_199_254_740_992.0;
    let scale = scale(is_int, decimals)?;
    let (a, b) = (lo * scale, hi * scale);
    if !(a.abs() < LIMIT && b.abs() < LIMIT) || a > b {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    let fits = |k: i64| {
        let v: f64 = format_number(k as f64 / scale, is_int, decimals)
            .parse()
            .unwrap_or(f64::NAN);
        (if open_low { v > lo } else { v >= lo }) && v <= hi
    };
    // Start a step outside each edge and move in: rounding can put the value
    // written nearest an edge on either side of it.
    #[allow(clippy::cast_possible_truncation)]
    let (mut kmin, mut kmax) = (a.floor() as i64 - 1, b.ceil() as i64 + 1);
    while kmin <= kmax && !fits(kmin) {
        kmin += 1;
    }
    while kmax >= kmin && !fits(kmax) {
        kmax -= 1;
    }
    (kmin <= kmax).then_some((kmin, kmax))
}

/// One class's rows, written to a temporary comma-separated file.
pub(crate) struct ClassFile {
    pub(crate) label:    String,
    pub(crate) bucket:   Option<(f64, f64)>,
    /// Binned classes: whether the bucket excludes its low edge (every bucket
    /// but the first; see `bin_of`).
    pub(crate) open_low: bool,
    pub(crate) rows:     u64,
    pub(crate) path:     PathBuf,
}

/// Writes each source row to its class's file in `dir`, in one read of the
/// source (with its delimiter, header and comment settings). With `bins`, a
/// row's class is the bucket its (trimmed) target value falls in; without, it
/// is the trimmed value itself, and `None` is returned once a
/// `MAX_CLASSES + 1`th value is seen. An empty value is the null class. Classes
/// are returned in a fixed order: null first, then by value (byte order) or by
/// bucket. The files are plain CSV with a header unless the source has none,
/// and are closed before this returns.
pub(crate) fn split(
    source: &Config,
    target_col: usize,
    bins: Option<&[(f64, f64)]>,
    dir: &std::path::Path,
) -> CliResult<Option<Vec<ClassFile>>> {
    struct Open {
        key:    ClassKey,
        writer: csv::Writer<File>,
        rows:   u64,
        path:   PathBuf,
    }
    let mut rdr = source.reader()?;
    let headers = (!source.no_headers)
        .then(|| rdr.byte_headers().cloned())
        .transpose()?;
    let mut index: HashMap<ClassKey, usize> = HashMap::new();
    let mut open: Vec<Open> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let raw = String::from_utf8_lossy(record.get(target_col).unwrap_or_default());
        let value = raw.trim();
        let key = match bins {
            _ if value.is_empty() => ClassKey::Null,
            Some(bins) => ClassKey::Bin(bin_of(value.parse().unwrap_or(f64::NAN), bins)),
            None => ClassKey::Value(value.to_string()),
        };
        let slot = if let Some(&slot) = index.get(&key) {
            slot
        } else {
            // The null class doesn't count toward the cap.
            if bins.is_none()
                && key != ClassKey::Null
                && open.iter().filter(|o| o.key != ClassKey::Null).count() == MAX_CLASSES
            {
                return Ok(None);
            }
            let path = dir.join(format!("class{}.csv", open.len()));
            // Every field quoted, so no line starts with a QSV_COMMENT_CHAR the
            // class readers would skip (a quoted "#north" would otherwise be
            // written bare).
            let mut writer = csv::WriterBuilder::new()
                .quote_style(csv::QuoteStyle::Always)
                .from_path(&path)?;
            if let Some(headers) = &headers {
                writer.write_byte_record(headers)?;
            }
            index.insert(key.clone(), open.len());
            open.push(Open {
                key,
                writer,
                rows: 0,
                path,
            });
            open.len() - 1
        };
        open[slot].writer.write_byte_record(&record)?;
        open[slot].rows += 1;
    }
    let mut classes = Vec::with_capacity(open.len());
    for mut class in open {
        class.writer.flush()?;
        drop(class.writer);
        classes.push((class.key, class.rows, class.path));
    }
    classes.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(Some(
        classes
            .into_iter()
            .map(|(key, rows, path)| {
                let (label, bucket, open_low) = match key {
                    ClassKey::Null => (String::new(), None, false),
                    ClassKey::Value(v) => (v, None, false),
                    ClassKey::Bin(i) => {
                        let bins = bins.unwrap_or_default();
                        let (lo, hi) = bins[i];
                        (format!("{lo}..{hi}"), Some((lo, hi)), i > 0)
                    },
                };
                ClassFile {
                    label,
                    bucket,
                    open_low,
                    rows,
                    path,
                }
            })
            .collect(),
    ))
}

/// A class's sort key: the null class first, then values in byte order, or
/// buckets in order.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ClassKey {
    Null,
    Value(String),
    Bin(usize),
}

/// The bucket a value falls in: the first whose upper edge is at or above it,
/// so a value on a shared edge goes to the lower bucket. Values outside the
/// buckets (beyond the trimmed range ends) go to the first or last one.
fn bin_of(value: f64, bins: &[(f64, f64)]) -> usize {
    bins.iter()
        .position(|&(_, hi)| value <= hi)
        .unwrap_or(bins.len().saturating_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_of_uses_upper_edges_and_clamps() {
        let bins = [(0.0, 10.0), (10.0, 20.0), (20.0, 30.0)];
        assert_eq!(bin_of(-5.0, &bins), 0);
        assert_eq!(bin_of(10.0, &bins), 0);
        assert_eq!(bin_of(10.5, &bins), 1);
        assert_eq!(bin_of(30.0, &bins), 2);
        assert_eq!(bin_of(99.0, &bins), 2);
    }

    /// Every written value a class can draw lands back in that class.
    fn assert_grid_lands_in_its_bucket(bins: &[(f64, f64)], is_int: bool, decimals: Option<u32>) {
        let scale = scale(is_int, decimals).unwrap();
        for (i, &bucket) in bins.iter().enumerate() {
            let Some((a, b)) = grid(bucket, i > 0, is_int, decimals) else {
                continue;
            };
            for k in a..=b {
                #[allow(clippy::cast_precision_loss)]
                let written = format_number(k as f64 / scale, is_int, decimals);
                assert_eq!(
                    bin_of(written.parse().unwrap(), bins),
                    i,
                    "{written} drawn for bucket {i} {bucket:?}"
                );
            }
        }
    }

    #[test]
    fn grid_keeps_rounded_draws_in_their_bucket() {
        // A draw in (2, 2.5) from the second bucket used to round to 2, which
        // belongs to the first.
        let ints = [(0.0, 2.0), (2.0, 5.5), (5.5, 9.0)];
        assert_eq!(grid(ints[0], false, true, None), Some((0, 2)));
        assert_eq!(grid(ints[1], true, true, None), Some((3, 5)));
        assert_eq!(grid(ints[2], true, true, None), Some((6, 9)));
        assert_grid_lands_in_its_bucket(&ints, true, None);

        let floats = [(0.0, 0.125), (0.125, 0.3), (0.3, 0.30000000000000004)];
        assert_eq!(grid(floats[0], false, false, Some(2)), Some((0, 12)));
        assert_eq!(grid(floats[1], true, false, Some(2)), Some((13, 30)));
        // 0.3 is written as 0.3, which is the second bucket's.
        assert_eq!(grid(floats[2], true, false, Some(2)), None);
        assert_grid_lands_in_its_bucket(&floats, false, Some(2));

        // No integer in (2.2, 2.7]; unrounded floats have no grid.
        assert_eq!(grid((2.2, 2.7), true, true, None), None);
        assert_eq!(grid((0.0, 1.0), false, false, None), None);
    }

    #[test]
    fn draw_writes_binned_values_from_the_class_grid() {
        let bucket = (2.0, 5.5);
        let target = Target {
            col_idx:  0,
            is_int:   true,
            decimals: None,
            classes:  vec![Class {
                label:    "2..5.5".into(),
                bucket:   Some(bucket),
                grid:     grid(bucket, true, true, None),
                rows:     1,
                schedule: Vec::new(),
            }],
        };
        let cumulative = target.cumulative();
        let mut rng = StdRng::seed_from_u64(7);
        let mut seen = std::collections::BTreeSet::new();
        for _ in 0..1000 {
            seen.insert(target.draw(&cumulative, &mut rng).0);
        }
        assert_eq!(seen, ["3", "4", "5"].map(String::from).into());
    }

    #[test]
    fn class_keys_sort_null_first_then_value_or_bin() {
        let mut keys = vec![
            ClassKey::Value("b".into()),
            ClassKey::Null,
            ClassKey::Value("a".into()),
        ];
        keys.sort();
        assert_eq!(
            keys,
            [
                ClassKey::Null,
                ClassKey::Value("a".into()),
                ClassKey::Value("b".into())
            ]
        );
        assert!(ClassKey::Null < ClassKey::Bin(0));
    }
}
