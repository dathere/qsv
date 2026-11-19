//! Saving and loading a fitted model (`--save-model` / `--model`).
//!
//! A model is the fitted emit schedule plus the output column names, as JSON.
//! Its format is internal to qsv: any change to what it holds bumps
//! `FORMAT_VERSION`, and a model of another version is refused rather than
//! migrated.

use serde::{Deserialize, Serialize};

use super::{
    generator::ColumnGenerator, group_generator::GroupGenerator, output::OutType,
    relationships::EmitUnit,
};
use crate::{CliError, CliResult, cmd::describegpt::dictionary::is_valid_strftime};

const FORMAT_VERSION: u32 = 2;

#[derive(Serialize, Deserialize)]
pub(crate) struct Model {
    qsv_synthesize_model:  u32,
    qsv_version:           String,
    fit:                   FitInfo,
    pub(crate) columns:    Vec<String>,
    /// Each column's output type, for `--format jsonl|parquet`.
    pub(crate) types:      Vec<OutType>,
    /// The source's day-first preference, for reading sampled real dates.
    pub(crate) prefer_dmy: bool,
    pub(crate) schedule:   Vec<EmitUnit>,
}

/// How the model was fitted. Informational only: generation never reads it.
#[derive(Serialize, Deserialize)]
pub(crate) struct FitInfo {
    pub(crate) seed:        Option<u64>,
    pub(crate) rows:        u64,
    pub(crate) source_rows: u64,
    pub(crate) min_count:   u64,
}

impl Model {
    pub(crate) fn new(
        columns: Vec<String>,
        types: Vec<OutType>,
        prefer_dmy: bool,
        schedule: Vec<EmitUnit>,
        fit: FitInfo,
    ) -> Self {
        Self {
            qsv_synthesize_model: FORMAT_VERSION,
            qsv_version: env!("CARGO_PKG_VERSION").to_string(),
            fit,
            columns,
            types,
            prefer_dmy,
            schedule,
        }
    }

    /// Writes the model to `path`. The JSON is parsed back first, so a model
    /// this qsv could not load (e.g. a non-finite float, which JSON writes as
    /// null) fails here instead of in a later `--model` run.
    pub(crate) fn save(&self, path: &str) -> CliResult<()> {
        let json = serde_json::to_string(self)?;
        Self::parse(&json)
            .map_err(|e| CliError::Other(format!("Unable to save the model to '{path}': {e}")))?;
        std::fs::write(path, json + "\n")?;
        Ok(())
    }

    pub(crate) fn load(path: &str) -> CliResult<Self> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| CliError::Other(format!("Unable to read model '{path}': {e}")))?;
        Self::parse(&raw).map_err(|e| CliError::Other(format!("Invalid model '{path}': {e}")))
    }

    fn parse(raw: &str) -> Result<Self, String> {
        let value: serde_json::Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
        // Check the version before the layout, so a model from another qsv
        // gets this message rather than a field-level parse error.
        match value
            .get("qsv_synthesize_model")
            .and_then(serde_json::Value::as_u64)
        {
            Some(v) if v == u64::from(FORMAT_VERSION) => {},
            Some(v) => {
                return Err(format!(
                    "it is model format version {v}, but this qsv reads version {FORMAT_VERSION}. \
                     Fit it again with --save-model."
                ));
            },
            None => return Err("not a model written by `synthesize --save-model`.".to_string()),
        }
        let model: Self = serde_json::from_value(value).map_err(|e| e.to_string())?;
        model.check()?;
        Ok(model)
    }

    /// Checks what generation relies on without re-checking: every column is
    /// filled exactly once, and nothing `next()` indexes, clamps or formats
    /// can panic.
    fn check(&self) -> Result<(), String> {
        let ncols = self.columns.len();
        if ncols == 0 {
            return Err("it has no columns.".to_string());
        }
        if self.types.len() != ncols {
            return bad("a type list that doesn't match its columns");
        }
        for ty in &self.types {
            check_date_format(ty.date_format())?;
        }
        let mut filled = vec![false; ncols];
        for unit in &self.schedule {
            let cols: &[usize] = match unit {
                EmitUnit::Independent { col_idx, generator } => {
                    check_column(generator)?;
                    std::slice::from_ref(col_idx)
                },
                EmitUnit::Group(group) => {
                    check_group(group)?;
                    group.col_indices()
                },
            };
            for &col in cols {
                match filled.get_mut(col) {
                    Some(seen) if !*seen => *seen = true,
                    Some(_) => return Err(format!("column {col} is generated twice.")),
                    None => return Err(format!("column {col} is out of range.")),
                }
            }
        }
        match filled.iter().position(|seen| !seen) {
            Some(col) => Err(format!("column {col} is never generated.")),
            None => Ok(()),
        }
    }
}

fn bad(what: &str) -> Result<(), String> {
    Err(format!("it has {what}."))
}

fn check_weighted<T>(values: &[T], cumulative: &[f64]) -> Result<(), String> {
    if values.is_empty() || values.len() != cumulative.len() {
        return bad("a value list that is empty or doesn't match its weights");
    }
    Ok(())
}

fn check_date_format(date_format: Option<&String>) -> Result<(), String> {
    match date_format {
        Some(fmt) if !is_valid_strftime(fmt) => bad(&format!("an invalid date format '{fmt}'")),
        _ => Ok(()),
    }
}

fn check_column(generator: &ColumnGenerator) -> Result<(), String> {
    match generator {
        ColumnGenerator::FrequencyWeighted {
            values, cumulative, ..
        } => check_weighted(values, cumulative),
        ColumnGenerator::FakerMapped {
            fakes, cumulative, ..
        } => check_weighted(fakes, cumulative),
        ColumnGenerator::NumericQuantile { buckets, .. } if buckets.is_empty() => {
            bad("a numeric column with no buckets")
        },
        ColumnGenerator::DateQuantile {
            buckets,
            date_format,
            ..
        } => {
            if buckets.is_empty() {
                return bad("a date column with no buckets");
            }
            check_date_format(date_format.as_ref())
        },
        ColumnGenerator::Boolean { true_ratio, .. } if !(0.0..=1.0).contains(true_ratio) => {
            bad("a boolean ratio outside 0 to 1")
        },
        _ => Ok(()),
    }
}

fn check_group(group: &GroupGenerator) -> Result<(), String> {
    match group {
        GroupGenerator::Joint {
            col_indices,
            tuples,
            cumulative,
        } => {
            check_weighted(tuples, cumulative)?;
            if tuples.iter().any(|t| t.len() != col_indices.len()) {
                return bad("a joint tuple of the wrong width");
            }
            Ok(())
        },
        GroupGenerator::Ordered {
            member_cols,
            is_int,
            date_format,
            anchor_buckets,
            gap_buckets,
            member_ranges,
            ..
        } => {
            let n = member_cols.len();
            if n == 0
                || is_int.len() != n
                || member_ranges.len() != n
                || gap_buckets.len() != n - 1
                || anchor_buckets.is_empty()
                || gap_buckets.iter().any(Vec::is_empty)
            {
                return bad("an ordered group with inconsistent members");
            }
            if member_ranges.iter().any(|(lo, hi)| lo > hi) {
                return bad("an ordered range whose low end is above its high end");
            }
            check_date_format(date_format.as_ref())
        },
        GroupGenerator::Correlated {
            col_indices,
            cholesky_l,
            marginals,
            is_int,
            null_ratios,
        } => {
            let k = col_indices.len();
            if cholesky_l.len() != k
                || cholesky_l.iter().enumerate().any(|(i, row)| row.len() <= i)
                || marginals.len() != k
                || marginals.iter().any(Vec::is_empty)
                || is_int.len() != k
                || null_ratios.len() != k
            {
                return bad("a correlated group with inconsistent members");
            }
            Ok(())
        },
    }
}
