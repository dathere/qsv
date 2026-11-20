//! Output formats (`--format`): column types and per-chunk encoding.
//!
//! Each column's output type is decided once, at fit time, and saved in the
//! model, so typed output never fails partway through a run. A generator that
//! carries its own type (quartile numbers, dates, booleans) decides it; a
//! column sampled from a pool of real values takes its `stats` type only when
//! every value in the pool converts without changing, and is text otherwise.

use std::io::Write;

use serde::{Deserialize, Serialize};

use super::{
    generator::ColumnGenerator,
    group_generator::{GroupGenerator, OrderedKind},
    relationships::EmitUnit,
    report::parse_date,
};
use crate::{CliError, CliResult};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    Csv,
    Jsonl,
    Parquet,
}

impl Format {
    pub(crate) fn parse(s: &str) -> CliResult<Self> {
        match s.to_ascii_lowercase().as_str() {
            "csv" => Ok(Self::Csv),
            "jsonl" => Ok(Self::Jsonl),
            "parquet" if cfg!(feature = "polars") => Ok(Self::Parquet),
            "parquet" => Err(CliError::Other(
                "--format parquet needs a qsv built with the polars feature.".to_string(),
            )),
            other => Err(CliError::Other(format!(
                "--format must be csv, jsonl or parquet, not '{other}'."
            ))),
        }
    }
}

/// A column's output type. Dates carry the format the generator writes them
/// in (`None`: read with the date parser `stats` uses).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum OutType {
    Integer,
    Float,
    Boolean,
    Date { format: Option<String> },
    DateTime { format: Option<String> },
    String,
}

impl OutType {
    pub(crate) fn date_format(&self) -> Option<&String> {
        match self {
            Self::Date { format } | Self::DateTime { format } => format.as_ref(),
            _ => None,
        }
    }
}

/// The output type of every column. `stats_types` are the source columns'
/// `stats` types, indexed like the output columns.
pub(crate) fn resolve_types(
    schedule: &[EmitUnit],
    stats_types: &[&str],
    prefer_dmy: bool,
) -> Vec<OutType> {
    let mut types = vec![OutType::String; stats_types.len()];
    for unit in schedule {
        match unit {
            EmitUnit::Independent { col_idx, generator } => {
                types[*col_idx] = match generator {
                    ColumnGenerator::NumericQuantile { is_int, .. } => number_type(*is_int),
                    ColumnGenerator::DateQuantile {
                        is_datetime,
                        date_format,
                        ..
                    } => date_type(*is_datetime, date_format.as_ref()),
                    ColumnGenerator::Boolean { .. } => OutType::Boolean,
                    ColumnGenerator::FrequencyWeighted { values, .. } => pooled_type(
                        stats_types[*col_idx],
                        values.iter().map(String::as_str),
                        prefer_dmy,
                    ),
                    _ => OutType::String,
                };
            },
            EmitUnit::Group(GroupGenerator::Joint {
                col_indices,
                tuples,
                ..
            }) => {
                for (k, &col) in col_indices.iter().enumerate() {
                    types[col] = pooled_type(
                        stats_types[col],
                        tuples.iter().map(|t| t[k].as_str()),
                        prefer_dmy,
                    );
                }
            },
            EmitUnit::Group(GroupGenerator::Ordered {
                member_cols,
                kind,
                is_int,
                date_format,
                ..
            }) => {
                for (k, &col) in member_cols.iter().enumerate() {
                    types[col] = match kind {
                        OrderedKind::Numeric => number_type(is_int[k]),
                        OrderedKind::Date => date_type(false, date_format.as_ref()),
                        OrderedKind::DateTime => date_type(true, date_format.as_ref()),
                    };
                }
            },
            EmitUnit::Group(GroupGenerator::Correlated {
                col_indices,
                is_int,
                ..
            }) => {
                for (k, &col) in col_indices.iter().enumerate() {
                    types[col] = number_type(is_int[k]);
                }
            },
        }
    }
    types
}

/// Whether values a generator types `values` keep their written form in a
/// column typed `column`: the same type, or Integer values in a Float column.
pub(crate) fn holds_exactly(column: &OutType, values: &OutType) -> bool {
    column == values || (*column == OutType::Float && *values == OutType::Integer)
}

/// Whether a column typed `column` can hold values a generator types
/// `values`: the same type, Integer values in a Float column, or anything in a
/// String column.
pub(crate) fn holds(column: &OutType, values: &OutType) -> bool {
    column == values
        || *column == OutType::String
        || (*column == OutType::Float && *values == OutType::Integer)
}

const fn number_type(is_int: bool) -> OutType {
    if is_int {
        OutType::Integer
    } else {
        OutType::Float
    }
}

/// A generated date column's type. A custom format is kept only if it reads
/// back the value it wrote: `%Y-%m` loses the day and `%y` the century, so
/// such columns are written as text.
fn date_type(is_datetime: bool, date_format: Option<&String>) -> OutType {
    let format = date_format.cloned();
    if let Some(fmt) = &format {
        let lossless = [-631_152_000_i64, 1_614_834_367].into_iter().all(|secs| {
            let Some(dt) = chrono::DateTime::from_timestamp(secs, 0) else {
                return false;
            };
            let dt = if is_datetime {
                dt
            } else {
                dt.date_naive().and_time(chrono::NaiveTime::MIN).and_utc()
            };
            parse_date(&dt.format(fmt).to_string(), Some(fmt), false).is_some_and(|back| back == dt)
        });
        if !lossless {
            return OutType::String;
        }
    }
    if is_datetime {
        OutType::DateTime { format }
    } else {
        OutType::Date { format }
    }
}

/// A pooled column's type: its `stats` type when every value in the pool
/// converts to it unchanged, else text. An integer must print back as itself,
/// so a zero-padded code stays text; a float must keep its decimal value
/// through f64; a date must be a whole day and a datetime must fit in
/// microseconds, as Parquet stores them.
fn pooled_type<'a>(
    stats_type: &str,
    mut values: impl Iterator<Item = &'a str>,
    prefer_dmy: bool,
) -> OutType {
    let (ty, converts): (OutType, &dyn Fn(&str) -> bool) = match stats_type {
        "Integer" => (OutType::Integer, &|s| {
            s.parse::<i64>().is_ok_and(|i| i.to_string() == s)
        }),
        "Float" => (OutType::Float, &|s| {
            s.parse::<f64>().is_ok_and(|f| {
                f.is_finite()
                    && decimal_digits(s)
                        == serde_json::to_string(&f)
                            .ok()
                            .and_then(|j| decimal_digits(&j))
            })
        }),
        "Boolean" => (OutType::Boolean, &|s| matches!(s, "true" | "false")),
        "Date" => (OutType::Date { format: None }, &|s| {
            parse_date(s, None, prefer_dmy).is_some_and(|dt| dt.time() == chrono::NaiveTime::MIN)
        }),
        "DateTime" => (OutType::DateTime { format: None }, &|s| {
            parse_date(s, None, prefer_dmy)
                .is_some_and(|dt| dt.timestamp_subsec_nanos() % 1_000 == 0)
        }),
        _ => return OutType::String,
    };
    if values.all(|s| s.is_empty() || converts(s)) {
        ty
    } else {
        OutType::String
    }
}

/// A decimal number as (negative, significant digits, power of ten of the
/// last digit), so `1.50`, `1.5` and `15e-1` compare equal. `None` if `s` is
/// not a plain decimal.
fn decimal_digits(s: &str) -> Option<(bool, String, i64)> {
    let (negative, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (mantissa, exp) = match s.split_once(['e', 'E']) {
        Some((m, e)) => (m, e.parse::<i64>().ok()?),
        None => (s, 0),
    };
    let (int, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    if !(int.bytes().chain(frac.bytes())).all(|b| b.is_ascii_digit()) {
        return None;
    }
    let digits = format!("{int}{frac}");
    let trimmed_end = digits.trim_end_matches('0');
    let power = exp
        .checked_sub(frac.len() as i64)?
        .checked_add((digits.len() - trimmed_end.len()) as i64)?;
    let digits = trimmed_end.trim_start_matches('0');
    if digits.is_empty() {
        return Some((false, String::new(), 0));
    }
    Some((negative, digits.to_string(), power))
}

/// Refuses duplicate column names, which JSONL objects and Parquet schemas
/// can't hold.
pub(crate) fn check_unique_columns(columns: &[String], format: Format) -> CliResult<()> {
    if format == Format::Csv {
        return Ok(());
    }
    let mut seen = std::collections::HashSet::new();
    match columns.iter().find(|c| !seen.insert(c.as_str())) {
        Some(dup) => Err(CliError::Other(format!(
            "--format {} needs unique column names, but '{dup}' appears more than once.",
            if format == Format::Jsonl {
                "jsonl"
            } else {
                "parquet"
            }
        ))),
        None => Ok(()),
    }
}

/// One encoded chunk.
pub(crate) enum Encoded {
    Bytes(Vec<u8>),
    #[cfg(feature = "polars")]
    Frame(polars::prelude::DataFrame),
}

impl Encoded {
    /// Bytes held, for sizing the generation window.
    pub(crate) fn size(&self) -> u64 {
        match self {
            Self::Bytes(b) => b.len() as u64,
            #[cfg(feature = "polars")]
            Self::Frame(df) => df.estimated_size(true) as u64,
        }
    }
}

/// Encodes rows into one chunk in the output format.
pub(crate) enum Sink {
    Csv {
        wtr:    Box<csv::Writer<Vec<u8>>>,
        record: csv::StringRecord,
    },
    Jsonl {
        buf:  Vec<u8>,
        /// Each column's `"name":` prefix, JSON-escaped once.
        keys: std::sync::Arc<[String]>,
    },
    #[cfg(feature = "polars")]
    Parquet(parquet::Columns),
}

impl Sink {
    pub(crate) fn csv(builder: &csv::WriterBuilder, size_hint: usize) -> Self {
        Self::Csv {
            wtr:    Box::new(builder.from_writer(Vec::with_capacity(size_hint))),
            record: csv::StringRecord::new(),
        }
    }

    pub(crate) fn jsonl(keys: std::sync::Arc<[String]>, size_hint: usize) -> Self {
        Self::Jsonl {
            buf: Vec::with_capacity(size_hint),
            keys,
        }
    }

    /// Writes one row. Parquet takes the row's values, so `row` must be
    /// fully refilled before the next call.
    pub(crate) fn push(&mut self, row: &mut [String], types: &[OutType]) -> CliResult<()> {
        match self {
            Self::Csv { wtr, record } => {
                record.clear();
                for value in row.iter() {
                    record.push_field(value);
                }
                wtr.write_record(&*record)?;
            },
            Self::Jsonl { buf, keys } => {
                buf.push(b'{');
                for (i, (value, ty)) in row.iter().zip(types).enumerate() {
                    if i > 0 {
                        buf.push(b',');
                    }
                    buf.extend_from_slice(keys[i].as_bytes());
                    write_json_value(buf, value, ty)?;
                }
                buf.extend_from_slice(b"}\n");
            },
            #[cfg(feature = "polars")]
            Self::Parquet(cols) => cols.push(row)?,
        }
        Ok(())
    }

    pub(crate) fn finish(self) -> CliResult<Encoded> {
        match self {
            Self::Csv { wtr, .. } => Ok(Encoded::Bytes(
                wtr.into_inner().map_err(|e| e.into_error())?,
            )),
            Self::Jsonl { buf, .. } => Ok(Encoded::Bytes(buf)),
            #[cfg(feature = "polars")]
            Self::Parquet(cols) => Ok(Encoded::Frame(cols.finish()?)),
        }
    }
}

/// The JSONL `"name":` prefix of each column.
pub(crate) fn jsonl_keys(columns: &[String]) -> CliResult<std::sync::Arc<[String]>> {
    columns
        .iter()
        .map(|c| Ok(serde_json::to_string(c)? + ":"))
        .collect()
}

/// An empty cell is null, as in `tojsonl`. Numbers and booleans are written
/// as JSON values; dates and text as strings.
fn write_json_value(buf: &mut Vec<u8>, value: &str, ty: &OutType) -> CliResult<()> {
    if value.is_empty() {
        buf.extend_from_slice(b"null");
        return Ok(());
    }
    match ty {
        OutType::Integer => {
            let i: i64 = value
                .parse()
                .map_err(|_| unconvertible(value, "an integer"))?;
            buf.extend_from_slice(itoa::Buffer::new().format(i).as_bytes());
        },
        OutType::Float => {
            let f: f64 = value
                .parse()
                .ok()
                .filter(|f: &f64| f.is_finite())
                .ok_or_else(|| unconvertible(value, "a number"))?;
            serde_json::to_writer(&mut *buf, &f)?;
        },
        OutType::Boolean => match value {
            "true" | "false" => buf.extend_from_slice(value.as_bytes()),
            _ => return Err(unconvertible(value, "a boolean")),
        },
        OutType::Date { .. } | OutType::DateTime { .. } | OutType::String => {
            serde_json::to_writer(&mut *buf, value)?;
        },
    }
    Ok(())
}

fn unconvertible(value: &str, what: &str) -> CliError {
    CliError::Other(format!(
        "synthesize generated '{value}', which is not {what}."
    ))
}

/// Opens the JSONL or CSV byte sink, or the Parquet writer.
pub(crate) enum Output {
    Bytes(Box<dyn Write>),
    #[cfg(feature = "polars")]
    Parquet(Box<parquet::Writer>),
}

impl Output {
    /// Opens `path` (stdout when `None`) for `format`, writing the CSV header.
    pub(crate) fn open(
        format: Format,
        path: Option<&str>,
        columns: &[String],
        types: &[OutType],
    ) -> CliResult<Self> {
        let conf = crate::config::Config::new(path.map(str::to_string).as_ref());
        match format {
            Format::Csv => {
                let mut wtr = conf.writer()?;
                wtr.write_record(columns)?;
                // Chunks arrive already encoded, so append them to the raw
                // writer; taking it out of the csv writer flushes the header.
                Ok(Self::Bytes(Box::new(
                    wtr.into_inner().map_err(|e| e.into_error())?,
                )))
            },
            Format::Jsonl => Ok(Self::Bytes(conf.io_writer()?)),
            #[cfg(feature = "polars")]
            Format::Parquet => Ok(Self::Parquet(Box::new(parquet::Writer::create(
                path.unwrap_or_default(),
                &parquet::schema(columns, types),
            )?))),
            #[cfg(not(feature = "polars"))]
            Format::Parquet => {
                let _ = types;
                unreachable!("Format::parse refuses parquet")
            },
        }
    }

    pub(crate) fn write(&mut self, chunk: u64, end_chunk: u64, enc: Encoded) -> CliResult<()> {
        match (self, enc) {
            (Self::Bytes(out), Encoded::Bytes(b)) => out.write_all(&b)?,
            #[cfg(feature = "polars")]
            (Self::Parquet(w), Encoded::Frame(df)) => w.push(chunk, end_chunk, df)?,
            #[cfg(feature = "polars")]
            _ => unreachable!("a chunk is encoded in the output's own format"),
        }
        #[cfg(not(feature = "polars"))]
        let _ = (chunk, end_chunk);
        Ok(())
    }

    pub(crate) fn finish(self) -> CliResult<()> {
        match self {
            Self::Bytes(mut out) => out.flush()?,
            #[cfg(feature = "polars")]
            Self::Parquet(w) => w.finish()?,
        }
        Ok(())
    }
}

/// The `--output` template of `--rows-per-file`: `{}` becomes the zero-based
/// number of each file's first row, as in `qsv split`, zero-padded to the
/// widest one so the names sort in order.
pub(crate) struct FileNames {
    prefix: String,
    suffix: String,
    width:  usize,
}

impl FileNames {
    pub(crate) fn new(template: &str, last_start: u64) -> CliResult<Self> {
        match template.split("{}").collect::<Vec<_>>()[..] {
            [prefix, suffix] if !suffix.contains(['/', '\\']) => Ok(Self {
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
                width:  last_start.to_string().len(),
            }),
            _ => Err(CliError::IncorrectUsage(
                "--rows-per-file needs an --output name with one {} in its file name, which is \
                 replaced by each file's first row number."
                    .to_string(),
            )),
        }
    }

    pub(crate) fn name(&self, start: u64) -> String {
        format!("{}{start:0>w$}{}", self.prefix, self.suffix, w = self.width)
    }

    /// Whether `path` is one of the files this template writes: a file whose
    /// first row is a multiple of `step` below `total`.
    pub(crate) fn writes(&self, path: &str, step: u64, total: u64) -> bool {
        use super::report::resolve_path;
        let ours = resolve_path(&self.name(0));
        let theirs = resolve_path(path);
        if ours.parent() != theirs.parent() {
            return false;
        }
        let file_prefix = self.prefix.rsplit(['/', '\\']).next().unwrap_or_default();
        theirs
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| {
                n.strip_prefix(file_prefix)?
                    .strip_suffix(self.suffix.as_str())
            })
            .filter(|digits| {
                digits.len() == self.width && digits.bytes().all(|b| b.is_ascii_digit())
            })
            .and_then(|digits| digits.parse::<u64>().ok())
            .is_some_and(|start| start < total && start.is_multiple_of(step))
    }
}

#[cfg(feature = "polars")]
pub(crate) mod parquet {
    use polars::prelude::*;

    use super::{OutType, parse_date, unconvertible};
    use crate::{CliError, CliResult, cmd::synthesize::CHUNK_ROWS};

    /// Chunks per Parquet row group. Groups end at fixed multiples of this
    /// chunk index, or earlier once the buffered rows reach `GROUP_BYTES`, never
    /// at a window boundary. Both depend only on the rows, so the file is the
    /// same at any --jobs.
    const GROUP_CHUNKS: u64 = 8;
    /// Buffered bytes that end a row group early, so wide rows can't hold
    /// eight chunks in memory beside the generation window.
    const GROUP_BYTES: usize = 128 * 1024 * 1024;

    enum Col {
        Int(Vec<Option<i64>>),
        Float(Vec<Option<f64>>),
        Bool(Vec<Option<bool>>),
        /// Days since the UNIX epoch.
        Date(Vec<Option<i32>>, Option<String>),
        /// Microseconds since the UNIX epoch.
        DateTime(Vec<Option<i64>>, Option<String>),
        Str(Vec<Option<String>>),
    }

    pub(crate) struct Columns {
        names:      std::sync::Arc<[String]>,
        cols:       Vec<Col>,
        prefer_dmy: bool,
    }

    fn dtype(ty: &OutType) -> DataType {
        match ty {
            OutType::Integer => DataType::Int64,
            OutType::Float => DataType::Float64,
            OutType::Boolean => DataType::Boolean,
            OutType::Date { .. } => DataType::Date,
            OutType::DateTime { .. } => DataType::Datetime(TimeUnit::Microseconds, None),
            OutType::String => DataType::String,
        }
    }

    pub(crate) fn schema(names: &[String], types: &[OutType]) -> Schema {
        names
            .iter()
            .zip(types)
            .map(|(n, t)| Field::new(n.as_str().into(), dtype(t)))
            .collect()
    }

    impl Columns {
        pub(crate) fn new(
            names: std::sync::Arc<[String]>,
            types: &[OutType],
            rows: usize,
            prefer_dmy: bool,
        ) -> Self {
            let cols = types
                .iter()
                .map(|t| match t {
                    OutType::Integer => Col::Int(Vec::with_capacity(rows)),
                    OutType::Float => Col::Float(Vec::with_capacity(rows)),
                    OutType::Boolean => Col::Bool(Vec::with_capacity(rows)),
                    OutType::Date { format } => Col::Date(Vec::with_capacity(rows), format.clone()),
                    OutType::DateTime { format } => {
                        Col::DateTime(Vec::with_capacity(rows), format.clone())
                    },
                    OutType::String => Col::Str(Vec::with_capacity(rows)),
                })
                .collect();
            Self {
                names,
                cols,
                prefer_dmy,
            }
        }

        pub(super) fn push(&mut self, row: &mut [String]) -> CliResult<()> {
            let dmy = self.prefer_dmy;
            let date = |s: &str, fmt: &Option<String>| {
                parse_date(s, fmt.as_deref(), dmy).ok_or_else(|| unconvertible(s, "a date"))
            };
            for (col, value) in self.cols.iter_mut().zip(row.iter_mut()) {
                let v = value.as_str();
                let empty = v.is_empty();
                match col {
                    Col::Int(c) => c.push(if empty {
                        None
                    } else {
                        Some(v.parse().map_err(|_| unconvertible(v, "an integer"))?)
                    }),
                    Col::Float(c) => c.push(if empty {
                        None
                    } else {
                        Some(v.parse().map_err(|_| unconvertible(v, "a number"))?)
                    }),
                    Col::Bool(c) => c.push(match v {
                        "" => None,
                        "true" => Some(true),
                        "false" => Some(false),
                        _ => return Err(unconvertible(v, "a boolean")),
                    }),
                    Col::Date(c, fmt) => c.push(if empty {
                        None
                    } else {
                        let days = date(v, fmt)?.date_naive().to_epoch_days();
                        Some(days)
                    }),
                    Col::DateTime(c, fmt) => c.push(if empty {
                        None
                    } else {
                        Some(date(v, fmt)?.timestamp_micros())
                    }),
                    Col::Str(c) => c.push((!empty).then(|| std::mem::take(value))),
                }
            }
            Ok(())
        }

        pub(super) fn finish(self) -> CliResult<DataFrame> {
            let polars_err = |e: PolarsError| CliError::Other(e.to_string());
            let height = self.cols.first().map_or(0, |c| match c {
                Col::Int(v) | Col::DateTime(v, _) => v.len(),
                Col::Float(v) => v.len(),
                Col::Bool(v) => v.len(),
                Col::Date(v, _) => v.len(),
                Col::Str(v) => v.len(),
            });
            let columns = self
                .cols
                .into_iter()
                .zip(self.names.iter())
                .map(|(col, name)| {
                    let name: PlSmallStr = name.as_str().into();
                    let series = match col {
                        Col::Int(v) => Series::new(name, v),
                        Col::Float(v) => Series::new(name, v),
                        Col::Bool(v) => Series::new(name, v),
                        Col::Date(v, _) => Series::new(name, v).cast(&DataType::Date)?,
                        Col::DateTime(v, _) => Series::new(name, v)
                            .cast(&DataType::Datetime(TimeUnit::Microseconds, None))?,
                        Col::Str(v) => Series::new(name, v),
                    };
                    Ok(series.into_column())
                })
                .collect::<PolarsResult<Vec<_>>>()
                .map_err(polars_err)?;
            DataFrame::new(height, columns).map_err(polars_err)
        }
    }

    pub(crate) struct Writer {
        inner:   polars::io::parquet::write::BatchedWriter<std::io::BufWriter<std::fs::File>>,
        pending: Option<DataFrame>,
    }

    impl Writer {
        pub(crate) fn create(path: &str, schema: &Schema) -> CliResult<Self> {
            let file = std::fs::File::create(path)
                .map_err(|e| CliError::Other(format!("Unable to create '{path}': {e}")))?;
            let inner = ParquetWriter::new(std::io::BufWriter::new(file))
                .with_row_group_size(Some((GROUP_CHUNKS * CHUNK_ROWS) as usize))
                .batched(schema)
                .map_err(|e| CliError::Other(e.to_string()))?;
            Ok(Self {
                inner,
                pending: None,
            })
        }

        /// Buffers chunk `chunk`'s frame, writing a row group at each fixed
        /// group boundary, once the buffer reaches `GROUP_BYTES`, and at the
        /// last chunk.
        pub(super) fn push(&mut self, chunk: u64, end_chunk: u64, df: DataFrame) -> CliResult<()> {
            let polars_err = |e: PolarsError| CliError::Other(e.to_string());
            match &mut self.pending {
                Some(pending) => {
                    pending.vstack_mut_owned(df).map_err(polars_err)?;
                },
                None => self.pending = Some(df),
            }
            let full = self
                .pending
                .as_ref()
                .is_some_and(|p| p.estimated_size(true) >= GROUP_BYTES);
            if full || (chunk + 1).is_multiple_of(GROUP_CHUNKS) || chunk + 1 == end_chunk {
                let mut group = self.pending.take().unwrap_or_default();
                group.rechunk_mut_par();
                self.inner.write_batch(&group).map_err(polars_err)?;
            }
            Ok(())
        }

        pub(super) fn finish(mut self) -> CliResult<()> {
            if let Some(mut group) = self.pending.take() {
                group.rechunk_mut_par();
                self.inner
                    .write_batch(&group)
                    .map_err(|e| CliError::Other(e.to_string()))?;
            }
            self.inner
                .finish()
                .map_err(|e| CliError::Other(e.to_string()))?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[test]
    fn date_formats_that_lose_information_are_text() {
        assert_eq!(date_type(false, fmt("%Y-%m").as_ref()), OutType::String);
        // %y reads 1950 back as 2050.
        assert_eq!(date_type(false, fmt("%y-%m-%d").as_ref()), OutType::String);
        // A datetime written without its time can't read the time back.
        assert_eq!(date_type(true, fmt("%Y-%m-%d").as_ref()), OutType::String);
        assert_eq!(
            date_type(false, fmt("%d/%m/%Y").as_ref()),
            OutType::Date {
                format: fmt("%d/%m/%Y"),
            }
        );
        assert_eq!(
            date_type(true, fmt("%Y-%m-%d %H:%M:%S").as_ref()),
            OutType::DateTime {
                format: fmt("%Y-%m-%d %H:%M:%S"),
            }
        );
        assert_eq!(date_type(true, None), OutType::DateTime { format: None });
    }

    #[test]
    fn pooled_values_keep_their_stats_type_only_if_all_convert_unchanged() {
        let pool = |t: &str, v: &[&str]| pooled_type(t, v.iter().copied(), false);
        assert_eq!(pool("Integer", &["1", "", "-20"]), OutType::Integer);
        assert_eq!(pool("Integer", &["1", "007"]), OutType::String);
        assert_eq!(pool("Integer", &["1", "+5"]), OutType::String);
        assert_eq!(pool("Float", &["1.50", "2"]), OutType::Float);
        assert_eq!(pool("Float", &["1.5", "NaN"]), OutType::String);
        assert_eq!(
            pool("Date", &["2024-01-05", "05/01/2024"]),
            OutType::Date { format: None }
        );
        assert_eq!(pool("Date", &["2024-01-05", "soon"]), OutType::String);
        assert_eq!(pool("String", &["1", "2"]), OutType::String);
        // 2^53 + 1 rounds to 2^53 through f64.
        assert_eq!(pool("Float", &["9007199254740993", "0.5"]), OutType::String);
        assert_eq!(pool("Float", &["0.1", "1e3", "-2.50"]), OutType::Float);
        // Same f64 as 1000000000000000.2, which is how the JSONL writer prints it.
        assert_eq!(pool("Float", &["1000000000000000.3"]), OutType::String);
        // Underflows to 0.0; its exponent must not overflow.
        assert_eq!(
            pool("Float", &["1.0e-9223372036854775808"]),
            OutType::String
        );
        assert_eq!(pool("Boolean", &["true", "false", ""]), OutType::Boolean);
        assert_eq!(pool("Boolean", &["yes", "no"]), OutType::String);
        // A Parquet Date can't hold a time of day.
        assert_eq!(
            pool("Date", &["2024-01-02T00:00:00.000456Z"]),
            OutType::String
        );
        // A Parquet datetime holds microseconds, not nanoseconds.
        assert_eq!(
            pool("DateTime", &["2024-01-02T03:04:05.123456789Z"]),
            OutType::String
        );
        assert_eq!(
            pool("DateTime", &["2024-01-02T03:04:05.123456Z"]),
            OutType::DateTime { format: None }
        );
    }

    #[test]
    fn jsonl_values_are_typed_and_empty_is_null() {
        let enc = |v: &str, t: &OutType| {
            let mut buf = Vec::new();
            write_json_value(&mut buf, v, t).unwrap();
            String::from_utf8(buf).unwrap()
        };
        assert_eq!(enc("", &OutType::Integer), "null");
        assert_eq!(enc("", &OutType::String), "null");
        assert_eq!(enc("-42", &OutType::Integer), "-42");
        assert_eq!(enc("1.50", &OutType::Float), "1.5");
        assert_eq!(enc("true", &OutType::Boolean), "true");
        assert_eq!(enc("a\"b", &OutType::String), r#""a\"b""#);
        assert_eq!(
            enc("2024-01-05", &OutType::Date { format: None }),
            "\"2024-01-05\""
        );
        let mut buf = Vec::new();
        assert!(write_json_value(&mut buf, "x", &OutType::Integer).is_err());
    }
}
