pub(crate) static USAGE: &str = r#"
Write a CSV as an SPSS, Stata or SAS transport file.

EXPERIMENTAL: the writers this command is built on are young. Check the files
it writes in the package that will read them before relying on them, and please
report anything that looks wrong.

Output formats, chosen by the --output file's extension or by --format:
    SPSS    .sav, .por
    Stata   .dta
    SAS     .xpt (transport version 8, or 5 with --format xpt5)
Compressed SPSS .zsav files & SAS .sas7bdat datasets can't be written.

The variable metadata comes from a JSON Schema data dictionary (--dictionary).
One written by 'qsv readstat --dictionary' brings a converted file back with its
variable labels, value labels, missing values & display settings; one written
by 'qsv describegpt --dictionary' gives its labels as variable labels. Without
a dictionary, the column types are inferred from the CSV.

The dictionary also says how readstat wrote the CSV, and that is undone: the
values decoded by the option --value-labels go back to their codes, and the
SPSS sentinel columns of --sentinels-as (<name>_null) become declared missing
values again.

Metadata the output format can't hold is refused rather than dropped silently:
value labels & missing values in SAS transport & SPSS portable files, missing
values & file labels in Stata files, labels too long for SAS transport files,
column names Stata would have to change, and so on. With --lossy, the file is
written anyway, and a warning names everything left out. Display settings
(measure, alignment, width & format) are written where the format has them and
left out where it doesn't.

  Convert an SPSS file to CSV & back, with its metadata:
    qsv readstat --dictionary survey.schema.json survey.sav -o survey.csv
    qsv writestat --dictionary survey.schema.json survey.csv -o survey2.sav

  Write the same data as a Stata file:
    qsv writestat --dictionary survey.schema.json survey.csv -o survey.dta

  Write a CSV as a SAS transport file, inferring the types:
    qsv writestat data.csv -o data.xpt

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_writestat.rs.

Usage:
    qsv writestat [options] [<input>]
    qsv writestat --help

writestat options:
    --dictionary <file>    The JSON Schema data dictionary giving the variable
                           metadata & types (see above).
    --format <fmt>         The format to write, if not the one the --output
                           extension names. Valid values: sav, por, dta,
                           xpt, xpt5, xpt8.
    --lossy                Write the file even if metadata the format can't
                           hold has to be left out, warning about each.
    --compress             Compress an SPSS .sav file (bytecode compression,
                           which every SPSS version reads).
    --table-name <name>    The dataset name in a SAS transport file. Defaults
                           to the dictionary's, else the --output file name.

Common options:
    -h, --help             Display this message
    -o, --output <file>    The file to write. Required.
    -d, --delimiter <arg>  The field delimiter for reading the CSV.
                           Must be a single character. (default: ,)
"#;

use std::{
    collections::{BTreeMap, HashMap},
    io::Read,
    path::{Path, PathBuf},
};

use polars::prelude::*;
use polars_readstat_rs::{
    PorWriteOptions, SpssAlignment, SpssMeasure, SpssMissingValue, SpssValueLabelKey,
    SpssVariableFormat, SpssWriter, StataWriter, XptWriter, merge_informative_null_columns,
    pandas_make_stata_column_names, write_por,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    CliError, CliResult, cmd::readstat::dictionary::parse_spss_format, config::Delimiter, util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_dictionary: Option<String>,
    flag_format:     Option<String>,
    flag_lossy:      bool,
    flag_compress:   bool,
    flag_table_name: Option<String>,
    flag_output:     Option<String>,
    flag_delimiter:  Option<Delimiter>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Format {
    Sav,
    Por,
    Dta,
    Xpt(u8),
}

impl Format {
    fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "sav" => Some(Self::Sav),
            "por" => Some(Self::Por),
            "dta" => Some(Self::Dta),
            "xpt" | "xpt8" => Some(Self::Xpt(8)),
            "xpt5" => Some(Self::Xpt(5)),
            _ => None,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Sav => "SPSS .sav",
            Self::Por => "SPSS portable .por",
            Self::Dta => "Stata .dta",
            Self::Xpt(5) => "SAS transport (version 5) .xpt",
            Self::Xpt(_) => "SAS transport (version 8) .xpt",
        }
    }
}

/// The package family a dictionary's source file came from: a display format
/// is only meaningful to the package that wrote it.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Family {
    Spss,
    Stata,
    Sas,
}

/// One property of the data dictionary.
#[derive(Default)]
struct Column {
    title:         Option<String>,
    qsv_type:      Option<String>,
    json_format:   Option<String>,
    value_labels:  Vec<(Value, String)>,
    missing:       Option<Value>,
    measure:       Option<String>,
    alignment:     Option<String>,
    display_width: Option<i64>,
    source_format: Option<String>,
    storage_type:  Option<String>,
    indicator_for: Option<String>,
}

impl Column {
    fn numeric_variable(&self) -> bool {
        self.storage_type
            .as_deref()
            .is_some_and(|t| t.starts_with("Numeric"))
    }

    /// The value labels whose codes are numbers, sentinels (`.a`) excluded.
    fn numeric_labels(&self) -> impl Iterator<Item = (f64, &str)> {
        self.value_labels
            .iter()
            .filter_map(|(code, label)| Some((code.as_f64()?, label.as_str())))
    }

    fn sentinel_labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.value_labels.iter().filter_map(|(code, label)| {
            code.as_str()
                .filter(|c| c.starts_with('.'))
                .map(|c| (c, label.as_str()))
        })
    }

    fn text_labels(&self) -> impl Iterator<Item = (&str, &str)> {
        self.value_labels.iter().filter_map(|(code, label)| {
            code.as_str()
                .filter(|c| !c.starts_with('.'))
                .map(|c| (c, label.as_str()))
        })
    }
}

#[derive(Default)]
struct Dictionary {
    columns:    HashMap<String, Column>,
    order:      Vec<String>,
    file_label: Option<String>,
    table_name: Option<String>,
    family:     Option<Family>,
    /// readstat --value-labels: the CSV holds labels, not codes
    decoded:    bool,
    /// readstat --sentinels-embedded: sentinels are in the variables' columns
    embedded:   bool,
}

fn text(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
}

impl Dictionary {
    fn load(path: &str) -> CliResult<Self> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| CliError::Other(format!("Cannot read --dictionary \"{path}\": {e}")))?;
        let doc: Value = serde_json::from_str(&raw).map_err(|e| {
            CliError::IncorrectUsage(format!("--dictionary \"{path}\" is not valid JSON: {e}"))
        })?;
        let Some(properties) = doc["properties"].as_object() else {
            return fail_incorrectusage_clierror!(
                "--dictionary \"{path}\" is not a JSON Schema data dictionary: it has no \
                 \"properties\"."
            );
        };
        let top = &doc["x-qsv"];
        let flags = &top["readstat_flags"];
        let family = top["source_format"].as_str().and_then(|f| {
            if f.starts_with("SPSS") {
                Some(Family::Spss)
            } else if f.starts_with("Stata") {
                Some(Family::Stata)
            } else if f.starts_with("SAS") {
                Some(Family::Sas)
            } else {
                None
            }
        });
        let mut dict = Self {
            file_label: text(&top["file_label"]),
            table_name: text(&top["table_name"]),
            family,
            decoded: flags["value_labels"].as_bool() == Some(true),
            embedded: flags["sentinels_embedded"].as_bool() == Some(true),
            ..Self::default()
        };
        for (name, prop) in properties {
            let x = &prop["x-qsv"];
            let value_labels = x["value_labels"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|e| Some((e["value"].clone(), e["label"].as_str()?.to_string())))
                .collect();
            dict.order.push(name.clone());
            dict.columns.insert(
                name.clone(),
                Column {
                    title: text(&prop["title"]),
                    qsv_type: text(&x["qsv_type"]),
                    json_format: text(&prop["format"]),
                    value_labels,
                    missing: x.get("missing_values").filter(|m| !m.is_null()).cloned(),
                    measure: text(&x["measure"]),
                    alignment: text(&x["alignment"]),
                    display_width: x["display_width"].as_i64(),
                    source_format: text(&x["source_format"]),
                    storage_type: text(&x["storage_type"]),
                    indicator_for: text(&x["indicator_for"]),
                },
            );
        }
        Ok(dict)
    }
}

/// What the output format can't hold. Refused, unless --lossy, which drops it
/// with a warning.
#[derive(Default)]
struct Losses(Vec<String>);

impl Losses {
    fn add(&mut self, what: impl Into<String>) {
        self.0.push(what.into());
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let Some(output) = args.flag_output.as_deref() else {
        return fail_incorrectusage_clierror!(
            "qsv writestat needs --output: the formats it writes are binary."
        );
    };
    let out_path = Path::new(output);
    let format = match args.flag_format.as_deref() {
        Some(f) => Format::parse(f).ok_or_else(|| {
            CliError::IncorrectUsage(format!(
                "\"{f}\" is not a valid --format. Valid values: sav, por, dta, xpt, xpt5, xpt8."
            ))
        })?,
        None => {
            let ext = out_path
                .extension()
                .map(|e| e.to_string_lossy().to_ascii_lowercase())
                .unwrap_or_default();
            if ext == "zsav" {
                return fail_incorrectusage_clierror!(
                    "Compressed SPSS .zsav files can't be written. Write a .sav, with --compress \
                     for bytecode compression."
                );
            }
            Format::parse(&ext).ok_or_else(|| {
                CliError::IncorrectUsage(format!(
                    "Can't tell which format to write from \"{output}\". Use a .sav, .por, .dta \
                     or .xpt --output, or pass --format."
                ))
            })?
        },
    };
    if args.flag_compress && format != Format::Sav {
        return fail_incorrectusage_clierror!("--compress applies to SPSS .sav files only.");
    }
    if args.flag_table_name.is_some() && !matches!(format, Format::Xpt(_)) {
        return fail_incorrectusage_clierror!(
            "--table-name applies to SAS transport (.xpt) files only."
        );
    }

    // stdin is spooled to a file, which the CSV reader needs
    let mut _stdin_spool = None;
    let input: PathBuf = match args.arg_input.as_deref() {
        Some(i) => PathBuf::from(i),
        None => {
            let mut spool = tempfile::Builder::new().suffix(".csv").tempfile()?;
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            std::io::Write::write_all(&mut spool, &buf)?;
            let path = spool.path().to_path_buf();
            _stdin_spool = Some(spool);
            path
        },
    };
    if !input.exists() {
        return fail_incorrectusage_clierror!("Cannot find input file \"{}\".", input.display());
    }
    if same_file::is_same_file(&input, out_path).unwrap_or(false) {
        return fail_incorrectusage_clierror!("--output would overwrite the input file.");
    }

    let dict = args
        .flag_dictionary
        .as_deref()
        .map(Dictionary::load)
        .transpose()?;
    let delim = args.flag_delimiter.map_or(b',', Delimiter::as_byte);
    let df = LazyCsvReader::new(PlRefPath::new(input.to_string_lossy().as_ref()))
        .with_has_header(true)
        .with_separator(delim)
        // with a dictionary, every column is read as text & typed by it
        .with_infer_schema_length(if dict.is_some() { Some(0) } else { None })
        .with_try_parse_dates(dict.is_none())
        .finish()?
        .collect()?;

    let mut losses = Losses::default();
    let (df, pairs) = match &dict {
        Some(dict) => apply_dictionary(df, dict, format, &mut losses)?,
        None => (df, HashMap::new()),
    };
    let df = if format == Format::Dta {
        integer_labeled_columns(df, dict.as_ref())?
    } else {
        df
    };
    let empty = Dictionary::default();
    let dict = dict.as_ref().unwrap_or(&empty);

    // everything is checked before the writer creates --output
    check_names(&df, format, &mut losses)?;
    let meta = Metadata::collect(&df, dict, format, &pairs, &mut losses);
    if !losses.0.is_empty() {
        if !args.flag_lossy {
            let list = losses
                .0
                .iter()
                .map(|l| format!("  - {l}"))
                .collect::<Vec<_>>()
                .join("\n");
            return fail_incorrectusage_clierror!(
                "A {} file can't hold all of this file's metadata:\n{list}\nPass --lossy to write \
                 it anyway, without them.",
                format.name()
            );
        }
        for loss in &losses.0 {
            wwarn!("--lossy: left out of the {} file: {loss}", format.name());
        }
    }

    let rows = df.height();
    write(df, out_path, format, &args, dict, meta, &pairs)?;
    winfo!(
        "{rows} row{} written to \"{output}\".",
        if rows == 1 { "" } else { "s" }
    );
    Ok(())
}

/// Type each column as the dictionary says, & undo what readstat did to write
/// it: labels back to codes, embedded sentinels out of the numbers. Returns
/// the SPSS sentinel columns, as (variable -> its `<name>_null` column).
fn apply_dictionary(
    mut df: DataFrame,
    dict: &Dictionary,
    format: Format,
    losses: &mut Losses,
) -> CliResult<(DataFrame, HashMap<String, String>)> {
    let missing: Vec<&String> = dict
        .order
        .iter()
        .filter(|name| df.column(name).is_err())
        .collect();
    if !missing.is_empty() {
        return fail_incorrectusage_clierror!(
            "The --dictionary describes columns the CSV doesn't have: {}. Is it the CSV's \
             dictionary?",
            missing
                .iter()
                .map(|n| format!("\"{n}\""))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let mut pairs = HashMap::new();
    let names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(ToString::to_string)
        .collect();
    for name in names {
        let Some(col) = dict.columns.get(&name) else {
            continue;
        };
        if let Some(base) = &col.indicator_for {
            if format == Format::Sav {
                pairs.insert(base.clone(), name.clone());
            } else {
                losses.add(format!(
                    "the sentinels of \"{base}\" (column \"{name}\"): only SPSS .sav files \
                     declare missing values"
                ));
                df.drop_in_place(&name)?;
            }
            continue;
        }
        let series = df.column(&name)?.as_materialized_series().clone();
        let typed = type_column(&series, col, dict, format, losses)?;
        df.replace(&name, typed.with_name(name.as_str().into()).into())?;
    }
    Ok((df, pairs))
}

/// One column as written by readstat (text) -> the type the dictionary says.
fn type_column(
    series: &Series,
    col: &Column,
    dict: &Dictionary,
    format: Format,
    losses: &mut Losses,
) -> CliResult<Series> {
    let name = series.name().to_string();
    let strings = series.str()?;
    let source_spss = dict.family == Some(Family::Spss);
    let spss_kind = col
        .source_format
        .as_deref()
        .filter(|_| source_spss)
        .and_then(parse_spss_format)
        .map(|(code, ..)| code);

    // a numeric variable readstat wrote as text: its labels decoded, or its
    // sentinels embedded among its numbers
    // (SPSS times are numeric variables written as text too, but parsed below)
    let time = matches!(spss_kind, Some(21 | 40));
    let text_numeric = col.numeric_variable()
        && !time
        && col.qsv_type.as_deref() == Some("String")
        && (dict.decoded || dict.embedded);
    if text_numeric {
        let by_label: HashMap<&str, f64> = col
            .numeric_labels()
            .map(|(code, label)| (label, code))
            .collect();
        let mut unreadable: Vec<String> = Vec::new();
        let values: Float64Chunked = strings
            .iter()
            .map(|cell| {
                let cell = cell.filter(|c| !c.is_empty())?;
                if let Some(code) = by_label.get(cell) {
                    return Some(*code);
                }
                if let Ok(n) = cell.parse::<f64>() {
                    return Some(n);
                }
                // a sentinel tag (.a) or its label: no writer can store those
                if unreadable.len() < 3 && !unreadable.iter().any(|u| u == cell) {
                    unreadable.push(cell.to_string());
                }
                None
            })
            .collect();
        if !unreadable.is_empty() {
            let tags = col
                .sentinel_labels()
                .any(|(c, l)| unreadable.iter().any(|u| u == c || u == l))
                || unreadable.iter().any(|u| u.starts_with('.'));
            if tags {
                losses.add(format!(
                    "the sentinels of \"{name}\" (e.g. \"{}\"): {} can't hold SAS & Stata tagged \
                     missing values, so they would be written as missing",
                    unreadable[0],
                    format.name()
                ));
            } else {
                return fail_clierror!(
                    "Column \"{name}\" holds \"{}\", which is neither a number nor one of its \
                     value labels.",
                    unreadable[0]
                );
            }
        }
        return Ok(values.into_series());
    }

    // a text variable with its labels decoded: back to its codes
    if dict.decoded && !col.numeric_variable() && col.text_labels().next().is_some() {
        let by_label: HashMap<&str, &str> = col
            .text_labels()
            .map(|(code, label)| (label, code))
            .collect();
        let values: StringChunked = strings
            .iter()
            .map(|cell| cell.map(|c| by_label.get(c).copied().unwrap_or(c)))
            .collect();
        return Ok(values.into_series());
    }

    let target = match col.qsv_type.as_deref() {
        Some("Integer") => DataType::Int64,
        Some("Float") => DataType::Float64,
        Some("Boolean") => DataType::Boolean,
        Some("Date") => return parse_temporal(strings, Temporal::Date),
        Some("DateTime") => return parse_temporal(strings, Temporal::DateTime),
        Some("NULL") => DataType::Float64,
        Some("String") if time => return parse_temporal(strings, Temporal::Time),
        Some("String") => return Ok(series.clone()),
        _ if col.json_format.as_deref() == Some("date") => {
            return parse_temporal(strings, Temporal::Date);
        },
        _ => return Ok(series.clone()),
    };
    // empty cells are missing, not values that fail to parse
    let cleaned: StringChunked = strings
        .iter()
        .map(|c| c.filter(|c| !c.is_empty()))
        .collect();
    cleaned.into_series().strict_cast(&target).map_err(|_| {
        CliError::Other(format!(
            "Column \"{name}\" can't be read as {target}, as the dictionary types it. Is it the \
             CSV's dictionary?"
        ))
    })
}

enum Temporal {
    Date,
    DateTime,
    Time,
}

/// Parse readstat's renderings of dates, datetimes & times back.
fn parse_temporal(strings: &StringChunked, kind: Temporal) -> CliResult<Series> {
    let bad = |cell: &str| {
        CliError::Other(format!(
            "Column \"{}\" holds \"{cell}\", which isn't a {}.",
            strings.name(),
            match kind {
                Temporal::Date => "date (YYYY-MM-DD)",
                Temporal::DateTime => "date & time (YYYY-MM-DDTHH:MM:SS)",
                Temporal::Time => "time (HH:MM:SS)",
            }
        ))
    };
    let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default();
    let cells = strings.iter().map(|c| c.filter(|c| !c.is_empty()));
    let series = match kind {
        Temporal::Date => {
            let days: CliResult<Int32Chunked> = cells
                .map(|c| {
                    c.map(|c| {
                        chrono::NaiveDate::parse_from_str(c, "%Y-%m-%d")
                            .map(|d| (d - epoch).num_days() as i32)
                            .map_err(|_| bad(c))
                    })
                    .transpose()
                })
                .collect();
            days?.into_series().cast(&DataType::Date)?
        },
        Temporal::DateTime => {
            let millis: CliResult<Int64Chunked> = cells
                .map(|c| {
                    c.map(|c| {
                        ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"]
                            .iter()
                            .find_map(|f| chrono::NaiveDateTime::parse_from_str(c, f).ok())
                            .map(|dt| dt.and_utc().timestamp_millis())
                            .ok_or_else(|| bad(c))
                    })
                    .transpose()
                })
                .collect();
            millis?
                .into_series()
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))?
        },
        Temporal::Time => {
            let nanos: CliResult<Int64Chunked> = cells
                .map(|c| {
                    c.map(|c| {
                        chrono::NaiveTime::parse_from_str(c, "%H:%M:%S%.f")
                            .map(|t| {
                                i64::from(chrono::Timelike::num_seconds_from_midnight(&t))
                                    * 1_000_000_000
                                    + i64::from(chrono::Timelike::nanosecond(&t))
                            })
                            .map_err(|_| bad(c))
                    })
                    .transpose()
                })
                .collect();
            nanos?.into_series().cast(&DataType::Time)?
        },
    };
    Ok(series.with_name(strings.name().clone()))
}

/// Stata labels only whole numbers, so whole-number labeled columns from
/// other packages are stored as integers (the writer picks the width). A
/// Stata dictionary's float or double variables keep their storage type.
fn integer_labeled_columns(mut df: DataFrame, dict: Option<&Dictionary>) -> CliResult<DataFrame> {
    let Some(dict) = dict else {
        return Ok(df);
    };
    for (name, col) in &dict.columns {
        if col.numeric_labels().next().is_none()
            || matches!(
                col.storage_type.as_deref(),
                Some("Numeric(Double)" | "Numeric(Float)")
            )
        {
            continue;
        }
        let Ok(column) = df.column(name) else {
            continue;
        };
        let series = column.as_materialized_series();
        if !series.dtype().is_float() {
            continue;
        }
        let whole = series
            .f64()?
            .iter()
            .flatten()
            .all(|v| v.fract() == 0.0 && v.abs() <= f64::from(i32::MAX));
        if whole {
            let ints = series.cast(&DataType::Int64)?;
            df.replace(name, ints.into())?;
        }
    }
    Ok(df)
}

/// Names the format can't hold are an error (or, for Stata, a loss): the
/// writers would otherwise fail midway or rename columns silently.
fn check_names(df: &DataFrame, format: Format, losses: &mut Losses) -> CliResult<()> {
    let names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(ToString::to_string)
        .collect();
    let max = match format {
        Format::Sav => 64,
        Format::Xpt(5) => 8,
        Format::Xpt(_) => 32,
        Format::Por => 8,
        Format::Dta => {
            let stata = pandas_make_stata_column_names(&names);
            for (old, new) in names.iter().zip(&stata) {
                if old != new {
                    losses.add(format!(
                        "the column name \"{old}\": Stata would rename it \"{new}\""
                    ));
                }
            }
            return Ok(());
        },
    };
    let long: Vec<&String> = names.iter().filter(|n| n.len() > max).collect();
    if !long.is_empty() {
        return fail_incorrectusage_clierror!(
            "A {} file's variable names are at most {max} bytes; these are longer: {}. Rename \
             them first (e.g. with 'qsv rename').",
            format.name(),
            long.iter()
                .map(|n| format!("\"{n}\""))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if matches!(format, Format::Sav | Format::Por) {
        let invalid: Vec<&String> = names.iter().filter(|n| !valid_spss_name(n)).collect();
        if !invalid.is_empty() {
            return fail_incorrectusage_clierror!(
                "These column names aren't valid SPSS variable names: {}. A name starts with a \
                 letter, holds only letters, digits & . _ @ # $, and isn't a reserved word in any \
                 case (ALL, AND, BY, EQ, GE, GT, LE, LT, NE, NOT, OR, TO, WITH). Rename them \
                 first (e.g. with 'qsv rename').",
                invalid
                    .iter()
                    .map(|n| format!("\"{n}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }
    Ok(())
}

/// The variable names readstat's own SPSS writer accepts, but with SPSS's
/// reserved words in any case, as SPSS syntax is case-insensitive.
fn valid_spss_name(name: &str) -> bool {
    const RESERVED: [&str; 13] = [
        "ALL", "AND", "BY", "EQ", "GE", "GT", "LE", "LT", "NE", "NOT", "OR", "TO", "WITH",
    ];
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || !c.is_ascii())
        && chars.all(|c| c.is_ascii_alphanumeric() || "._@#$".contains(c) || !c.is_ascii())
        && !RESERVED.iter().any(|r| r.eq_ignore_ascii_case(name))
}

/// The metadata the output format holds, gathered from the dictionary.
#[derive(Default)]
struct Metadata {
    variable_labels: HashMap<String, String>,
    numeric_labels:  HashMap<String, Vec<(f64, String)>>,
    missing:         HashMap<String, Vec<SpssMissingValue>>,
    measures:        HashMap<String, SpssMeasure>,
    alignments:      HashMap<String, SpssAlignment>,
    widths:          HashMap<String, i32>,
    spss_formats:    HashMap<String, SpssVariableFormat>,
    formats:         HashMap<String, String>,
    file_label:      Option<String>,
}

impl Metadata {
    fn collect(
        df: &DataFrame,
        dict: &Dictionary,
        format: Format,
        pairs: &HashMap<String, String>,
        losses: &mut Losses,
    ) -> Self {
        let mut meta = Self::default();
        let spss = format == Format::Sav;
        let label_limit = match format {
            Format::Xpt(_) => Some(40),
            Format::Dta => Some(80),
            _ => None,
        };
        if let Some(label) = &dict.file_label {
            match format {
                Format::Sav | Format::Dta => losses.add(format!(
                    "the file label \"{label}\": the {} writer can't set one",
                    format.name()
                )),
                _ if label_limit.is_some_and(|max| label.len() > max) => {
                    losses.add(format!("the file label \"{label}\": longer than 40 bytes"))
                },
                _ => meta.file_label = Some(label.clone()),
            }
        }

        for name in df.get_column_names() {
            let name = name.as_str();
            let Some(col) = dict.columns.get(name) else {
                continue;
            };
            if let Some(title) = &col.title {
                if label_limit.is_some_and(|max| title.len() > max) {
                    losses.add(format!(
                        "the label of \"{name}\": longer than {} bytes",
                        label_limit.unwrap_or_default()
                    ));
                } else {
                    meta.variable_labels.insert(name.to_string(), title.clone());
                }
            }

            // value labels: numbers in SPSS & Stata; never text codes or tags
            let numbers: Vec<(f64, String)> = col
                .numeric_labels()
                .map(|(c, l)| (c, l.to_string()))
                .collect();
            if !numbers.is_empty() {
                match format {
                    Format::Sav => {
                        meta.numeric_labels.insert(name.to_string(), numbers);
                    },
                    Format::Dta
                        if numbers
                            .iter()
                            .all(|(c, _)| c.fract() == 0.0 && c.abs() <= f64::from(i32::MAX)) =>
                    {
                        meta.numeric_labels.insert(name.to_string(), numbers);
                    },
                    Format::Dta => losses.add(format!(
                        "the value labels of \"{name}\": Stata labels whole numbers only"
                    )),
                    _ => losses.add(format!("the value labels of \"{name}\"")),
                }
            }
            if col.text_labels().next().is_some() {
                losses.add(format!(
                    "the value labels of \"{name}\": its codes are text, which {} can't label",
                    format.name()
                ));
            }
            if col.sentinel_labels().next().is_some() {
                losses.add(format!(
                    "the labels of \"{name}\"'s sentinels: tagged missing values (.a) can't be \
                     written"
                ));
            }

            if let Some(missing) = &col.missing {
                if spss {
                    meta.missing.insert(name.to_string(), spss_missing(missing));
                } else {
                    losses.add(format!(
                        "the missing values of \"{name}\": only SPSS .sav files declare them"
                    ));
                }
            } else if pairs.contains_key(name) {
                // sentinel columns without declared missing values: nothing to merge into
                losses.add(format!(
                    "the sentinels of \"{name}\": the dictionary declares no missing values"
                ));
            }

            if spss {
                if let Some(m) = col.measure.as_deref().and_then(measure) {
                    meta.measures.insert(name.to_string(), m);
                }
                if let Some(a) = col.alignment.as_deref().and_then(alignment) {
                    meta.alignments.insert(name.to_string(), a);
                }
                if let Some(w) = col.display_width.and_then(|w| i32::try_from(w).ok()) {
                    meta.widths.insert(name.to_string(), w);
                }
            }
            // a display format means something only to the package that wrote it
            if let Some(f) = &col.source_format {
                match (format, dict.family) {
                    (Format::Sav, Some(Family::Spss)) => {
                        if let Some((code, width, decimals)) = parse_spss_format(f) {
                            meta.spss_formats.insert(
                                name.to_string(),
                                SpssVariableFormat {
                                    format_type: Some(code),
                                    width:       Some(width),
                                    decimals:    Some(decimals),
                                },
                            );
                        }
                    },
                    (Format::Dta, Some(Family::Stata)) | (Format::Xpt(_), Some(Family::Sas)) => {
                        meta.formats.insert(name.to_string(), f.clone());
                    },
                    _ => {},
                }
            }
        }
        meta
    }
}

fn spss_missing(missing: &Value) -> Vec<SpssMissingValue> {
    let numbers = |key: &str| -> Vec<f64> {
        missing[key]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_f64)
            .collect()
    };
    let mut out = Vec::new();
    if let [lo, hi] = numbers("range")[..] {
        out.push(SpssMissingValue::Range { lo, hi });
    }
    out.extend(numbers("discrete").into_iter().map(SpssMissingValue::Num));
    out.extend(
        missing["strings"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(|s| SpssMissingValue::Str(s.to_string())),
    );
    out
}

fn measure(m: &str) -> Option<SpssMeasure> {
    match m.to_ascii_lowercase().as_str() {
        "scale" => Some(SpssMeasure::Scale),
        "nominal" => Some(SpssMeasure::Nominal),
        "ordinal" => Some(SpssMeasure::Ordinal),
        _ => None,
    }
}

fn alignment(a: &str) -> Option<SpssAlignment> {
    match a.to_ascii_lowercase().as_str() {
        "left" => Some(SpssAlignment::Left),
        "right" => Some(SpssAlignment::Right),
        "center" => Some(SpssAlignment::Center),
        _ => None,
    }
}

/// Upstream's `merge_informative_null_columns` only rebuilds numeric
/// variables, so put each text variable's sentinels back in its own cells
/// (a label back to its code) and leave the numeric pairs to upstream.
fn merge_string_sentinels(
    mut df: DataFrame,
    dict: &Dictionary,
    pairs: &HashMap<String, String>,
) -> CliResult<(DataFrame, HashMap<String, String>)> {
    let mut numeric = HashMap::with_capacity(pairs.len());
    for (base, indicator) in pairs {
        if df.column(base)?.dtype() != &DataType::String {
            numeric.insert(base.clone(), indicator.clone());
            continue;
        }
        let codes: HashMap<&str, &str> = dict
            .columns
            .get(base)
            .map(|col| {
                col.text_labels()
                    .map(|(code, label)| (label, code))
                    .collect()
            })
            .unwrap_or_default();
        let merged: StringChunked = {
            let values = df.column(base)?.str()?;
            let sentinels = df.column(indicator)?.str()?;
            values
                .iter()
                .zip(sentinels.iter())
                .map(|(value, sentinel)| {
                    value.or_else(|| sentinel.map(|s| codes.get(s).copied().unwrap_or(s)))
                })
                .collect()
        };
        df.replace(base, merged.with_name(base.as_str().into()).into_column())?;
        df.drop_in_place(indicator)?;
    }
    Ok((df, numeric))
}

fn write(
    df: DataFrame,
    path: &Path,
    format: Format,
    args: &Args,
    dict: &Dictionary,
    meta: Metadata,
    pairs: &HashMap<String, String>,
) -> CliResult<()> {
    let err = |e: &dyn std::fmt::Display| {
        CliError::Other(format!("Could not write \"{}\": {e}", path.display()))
    };
    match format {
        Format::Sav => {
            let labels: HashMap<String, HashMap<SpssValueLabelKey, String>> = meta
                .numeric_labels
                .into_iter()
                .map(|(name, labels)| {
                    let map = labels
                        .into_iter()
                        .map(|(code, label)| (SpssValueLabelKey::from_f64(code), label))
                        .collect();
                    (name, map)
                })
                .collect();
            // the <name>_null columns become declared missing values again
            let (df, pairs) = merge_string_sentinels(df, dict, pairs)?;
            let df = if pairs.is_empty() {
                df
            } else {
                merge_informative_null_columns(df, Some(&labels), Some(&pairs))
                    .map_err(|e| err(&e))?
            };
            SpssWriter::new(path)
                .with_compression(args.flag_compress)
                .with_variable_labels(meta.variable_labels)
                .with_value_labels(labels)
                .with_missing_values(meta.missing)
                .with_variable_measures(meta.measures)
                .with_variable_alignments(meta.alignments)
                .with_variable_display_widths(meta.widths)
                .with_variable_formats(meta.spss_formats)
                .write_df(&df)
                .map_err(|e| err(&e))
        },
        Format::Dta => {
            #[allow(clippy::cast_possible_truncation)]
            let labels: HashMap<String, BTreeMap<i32, String>> = meta
                .numeric_labels
                .into_iter()
                .map(|(name, labels)| {
                    let map = labels
                        .into_iter()
                        .map(|(code, label)| (code as i32, label))
                        .collect();
                    (name, map)
                })
                .collect();
            StataWriter::new(path)
                .with_variable_labels(meta.variable_labels)
                .with_value_labels(labels)
                .with_variable_formats(meta.formats)
                .write_df(&df)
                .map_err(|e| err(&e))
        },
        Format::Xpt(version) => {
            let table = args
                .flag_table_name
                .clone()
                .or_else(|| dict.table_name.clone())
                .unwrap_or_else(|| {
                    path.file_stem()
                        .map(|s| s.to_string_lossy().to_ascii_uppercase())
                        .unwrap_or_else(|| "DATASET".to_string())
                });
            let max = if version == 5 { 8 } else { 32 };
            if table.len() > max {
                return fail_incorrectusage_clierror!(
                    "The SAS transport dataset name \"{table}\" is longer than {max} bytes. Pass \
                     a shorter --table-name."
                );
            }
            let mut writer = XptWriter::new(path)
                .with_version(version)
                .with_table_name(table)
                .with_variable_labels(meta.variable_labels)
                .with_variable_formats(meta.formats);
            if let Some(label) = meta.file_label {
                writer = writer.with_file_label(label);
            }
            writer.write_df(&df).map_err(|e| err(&e))
        },
        Format::Por => write_por(
            &df,
            path,
            PorWriteOptions {
                file_label:      meta.file_label,
                variable_labels: Some(meta.variable_labels),
            },
        )
        .map_err(|e| err(&e)),
    }
}
