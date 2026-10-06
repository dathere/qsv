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
a dictionary, the column types are inferred from the CSV: numbers from its stats
cache (created beside it if missing, so later runs are faster), and dates, times
& true/false values as Polars recognises them. Set QSV_STATSCACHE_MODE to none
to have Polars infer every type itself instead, reading the whole file on a
single thread, which is much slower.

The dictionary also says how readstat wrote the CSV, and that is undone: the
values decoded by the option --value-labels go back to their codes, and the
SPSS sentinels of --sentinels-as become declared missing values again, whether
in <name>_null columns or embedded. A label that can't be turned back into one
code - shared by two codes, reading as a number that isn't its code, or also
an ordinary value - is an error: write the CSV again with readstat without
the label.

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
    title:            Option<String>,
    qsv_type:         Option<String>,
    json_format:      Option<String>,
    value_labels:     Vec<(Value, String)>,
    missing:          Option<Value>,
    measure:          Option<String>,
    alignment:        Option<String>,
    display_width:    Option<i64>,
    source_format:    Option<String>,
    storage_type:     Option<String>,
    indicator_for:    Option<String>,
    /// labels readstat wrote that are also ordinary values of the variable
    label_collisions: Vec<String>,
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

    /// The labels of SAS & Stata tagged missing values (.a): numeric
    /// variables only, as an SPSS text code may start with a dot too.
    fn sentinel_labels(&self) -> impl Iterator<Item = (&str, &str)> {
        let numeric = self.numeric_variable();
        self.value_labels.iter().filter_map(move |(code, label)| {
            code.as_str()
                .filter(|c| numeric && c.starts_with('.'))
                .map(|c| (c, label.as_str()))
        })
    }

    /// Whether `code` is one of the SPSS missing values the variable declares.
    fn declares_missing(&self, code: f64) -> bool {
        let Some(missing) = &self.missing else {
            return false;
        };
        let discrete = missing["discrete"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|v| v.as_f64() == Some(code));
        let range = match missing["range"].as_array().map(Vec::as_slice) {
            Some([lo, hi]) => lo
                .as_f64()
                .zip(hi.as_f64())
                .is_some_and(|(lo, hi)| (lo..=hi).contains(&code)),
            _ => false,
        };
        discrete || range
    }

    fn declares_missing_text(&self, code: &str) -> bool {
        self.missing.as_ref().is_some_and(|m| {
            m["strings"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|v| v.as_str() == Some(code))
        })
    }

    fn text_labels(&self) -> impl Iterator<Item = (&str, &str)> {
        let numeric = self.numeric_variable();
        self.value_labels.iter().filter_map(move |(code, label)| {
            code.as_str()
                .filter(|c| !numeric || !c.starts_with('.'))
                .map(|c| (c, label.as_str()))
        })
    }

    fn missing_texts(&self) -> impl Iterator<Item = &str> {
        self.missing
            .iter()
            .flat_map(|m| m["strings"].as_array().into_iter().flatten())
            .filter_map(Value::as_str)
    }
}

#[derive(Default)]
struct Dictionary {
    columns:          HashMap<String, Column>,
    order:            Vec<String>,
    file_label:       Option<String>,
    table_name:       Option<String>,
    family:           Option<Family>,
    /// readstat --value-labels: the CSV holds labels, not codes
    decoded:          bool,
    /// readstat --sentinels-embedded: sentinels are in the variables' columns
    embedded:         bool,
    /// readstat --sentinels-as label: sentinels are written as their labels
    sentinel_labels:  bool,
    /// readstat --sentinels-columns: the variables whose sentinels were kept
    sentinel_columns: Option<Vec<String>>,
}

impl Dictionary {
    /// Whether readstat wrote the sentinels of `name` as their labels, in
    /// its own column.
    fn embeds_sentinel_labels(&self, name: &str) -> bool {
        self.sentinel_labels && self.embeds_sentinels(name)
    }

    /// Whether readstat wrote the sentinels of `name` in its own column.
    fn embeds_sentinels(&self, name: &str) -> bool {
        self.embedded
            && self
                .sentinel_columns
                .as_ref()
                .is_none_or(|cols| cols.iter().any(|c| c == name))
    }
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
            sentinel_labels: flags["sentinels_as"].as_str() == Some("label"),
            sentinel_columns: flags["sentinels_columns"]
                .as_str()
                .map(|list| list.split(',').map(|n| n.trim().to_string()).collect()),
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
                    label_collisions: x["label_collisions"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect(),
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

    // stdin is spooled to a file, which the CSV reader needs. It gets a
    // directory of its own, so the stats cache written beside it goes with it.
    let mut _stdin_spool = None;
    let input: PathBuf = match args.arg_input.as_deref() {
        Some(i) => PathBuf::from(i),
        None => {
            let dir = tempfile::tempdir()?;
            let path = dir.path().join("stdin.csv");
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            std::fs::write(&path, &buf)?;
            _stdin_spool = Some(dir);
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
    let df = if dict.is_some() {
        // with a dictionary, every column is read as text & typed by it
        csv_reader(&input, delim)
            .with_infer_schema_length(Some(0))
            .finish()?
            .collect()?
    } else {
        read_inferred(&input, delim)?
    };

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
    let df = fit_temporal(df, format, &mut losses)?;
    let empty = Dictionary::default();
    let dict = dict.as_ref().unwrap_or(&empty);
    // before anything is checked, as a sentinel may be ambiguous
    let (df, pairs) = merge_string_sentinels(df, dict, &pairs, &mut losses)?;

    // everything is checked before the writer creates --output
    check_names(&df, format, &pairs, &mut losses)?;
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

/// A value label's code, as readstat recorded it.
enum Code<'a> {
    Number(f64),
    /// a SAS or Stata tagged missing value (.a)
    Tag(&'a str),
}

impl std::fmt::Display for Code<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Code::Number(n) => write!(f, "{n}"),
            Code::Tag(t) => f.write_str(t),
        }
    }
}

/// The labels readstat wrote for a numeric variable's codes: all of them
/// under --value-labels; under --sentinels-as label, those of its sentinels
/// (an SPSS declared missing value or a SAS/Stata tag), if it kept them.
fn numeric_reverse_labels<'a>(
    name: &str,
    col: &'a Column,
    dict: &Dictionary,
) -> HashMap<&'a str, Vec<Code<'a>>> {
    let sentinels = dict.embeds_sentinel_labels(name);
    let mut map: HashMap<&str, Vec<Code>> = HashMap::new();
    for (code, label) in &col.value_labels {
        let code = if let Some(n) = code.as_f64() {
            // a declared missing value's label is written only where its
            // sentinels are embedded as labels; elsewhere the cell is empty
            let missing = col.declares_missing(n);
            if !((dict.decoded && !missing) || (sentinels && missing)) {
                continue;
            }
            Code::Number(n)
        } else if let Some(tag) = code.as_str().filter(|c| c.starts_with('.')) {
            if !sentinels {
                continue;
            }
            Code::Tag(tag)
        } else {
            continue;
        };
        map.entry(label.as_str()).or_default().push(code);
    }
    map
}

/// Why a cell holding `label` can't be turned back into one code, if it
/// can't: two codes share the label, or the label reads as a number that
/// isn't its code - and so can't be told apart from that number - or, where
/// `raw_tags` may be written as they are, it reads as a tag (.a).
fn ambiguity(label: &str, codes: &[Code], raw_tags: bool) -> Option<String> {
    if codes.len() > 1 {
        let list = codes.iter().map(ToString::to_string).collect::<Vec<_>>();
        return Some(format!("the label of {}", list.join(", ")));
    }
    let code = codes.first()?;
    if let Ok(n) = label.parse::<f64>() {
        return match code {
            Code::Number(c) if *c == n => None,
            code => Some(format!("both a number and the label of {code}")),
        };
    }
    let tag = label.len() == 2
        && label.starts_with('.')
        && label[1..]
            .bytes()
            .all(|b| b.is_ascii_alphabetic() || b == b'_');
    match code {
        Code::Tag(t) if *t == label => None,
        code if raw_tags && tag => Some(format!(
            "both a tagged missing value and the label of {code}"
        )),
        _ => None,
    }
}

fn ambiguous_cell<T>(name: &str, cell: &str, why: &str) -> CliResult<T> {
    fail_incorrectusage_clierror!(
        "Column \"{name}\" holds \"{cell}\", which is {why}, so the code it stands for can't be \
         told. Write the CSV again with 'qsv readstat' without --value-labels, or with \
         --sentinels-as value."
    )
}

/// Keep the first few distinct cells, for a message.
fn note(cells: &mut Vec<String>, cell: &str) {
    if cells.len() < 3 && !cells.iter().any(|c| c == cell) {
        cells.push(cell.to_string());
    }
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
        let by_label = numeric_reverse_labels(&name, col, dict);
        let mut tagged: Vec<String> = Vec::new();
        let mut unreadable: Vec<String> = Vec::new();
        let mut ambiguous: Option<(String, String)> = None;
        let values: Float64Chunked = strings
            .iter()
            .map(|cell| {
                let cell = cell.filter(|c| !c.is_empty())?;
                if let Some(codes) = by_label.get(cell) {
                    if let Some(why) = ambiguity(
                        cell,
                        codes,
                        dict.family != Some(Family::Spss) && dict.embeds_sentinels(&name),
                    ) {
                        ambiguous.get_or_insert_with(|| (cell.to_string(), why));
                        return None;
                    }
                    return match codes[0] {
                        Code::Number(n) => Some(n),
                        // no writer can store a tagged missing value (.a)
                        Code::Tag(_) => {
                            note(&mut tagged, cell);
                            None
                        },
                    };
                }
                if let Ok(n) = cell.parse::<f64>() {
                    return Some(n);
                }
                if cell.starts_with('.') {
                    note(&mut tagged, cell);
                } else {
                    note(&mut unreadable, cell);
                }
                None
            })
            .collect();
        if let Some((cell, why)) = ambiguous {
            return ambiguous_cell(&name, &cell, &why);
        }
        if let Some(cell) = unreadable.first() {
            return fail_clierror!(
                "Column \"{name}\" holds \"{cell}\", which is neither a number nor one of its \
                 value labels."
            );
        }
        if let Some(cell) = tagged.first() {
            losses.add(format!(
                "the sentinels of \"{name}\" (e.g. \"{cell}\"): {} can't hold SAS & Stata tagged \
                 missing values, so they would be written as missing",
                format.name()
            ));
        }
        return Ok(values.into_series());
    }

    // a text variable with its labels written for its codes: back to them
    if !col.numeric_variable() {
        let sentinels = dict.embeds_sentinel_labels(&name);
        let mut by_label: HashMap<&str, Vec<&str>> = HashMap::new();
        for (code, label) in col.text_labels() {
            let missing = col.declares_missing_text(code);
            if (dict.decoded && !missing) || (sentinels && missing) {
                by_label.entry(label).or_default().push(code);
            }
        }
        if !by_label.is_empty() {
            // the codes readstat wrote as they are: labeled ones it didn't
            // decode, & the embedded sentinels it didn't label
            // (elsewhere a declared missing value is written empty)
            let embedded = dict.embeds_sentinels(&name);
            let raw: Vec<&str> = col
                .text_labels()
                .map(|(code, _)| code)
                .filter(|code| embedded || !col.declares_missing_text(code))
                .chain(col.missing_texts().filter(|_| embedded))
                .filter(|code| !by_label.values().flatten().any(|c| c == code))
                .collect();
            let mut ambiguous: Option<(String, String)> = None;
            let values: StringChunked = strings
                .iter()
                .map(|cell| {
                    cell.map(|c| {
                        let Some(codes) = by_label.get(c) else {
                            return c;
                        };
                        if codes.len() > 1 {
                            ambiguous.get_or_insert_with(|| {
                                (c.to_string(), format!("the label of {}", codes.join(", ")))
                            });
                        } else if raw.contains(&c) {
                            ambiguous.get_or_insert_with(|| {
                                (
                                    c.to_string(),
                                    format!("both a code and the label of {}", codes[0]),
                                )
                            });
                        } else if col.label_collisions.iter().any(|l| l == c) {
                            ambiguous.get_or_insert_with(|| {
                                (
                                    c.to_string(),
                                    format!("both a value and the label of {}", codes[0]),
                                )
                            });
                        }
                        codes[0]
                    })
                })
                .collect();
            if let Some((cell, why)) = ambiguous {
                return ambiguous_cell(&name, &cell, &why);
            }
            return Ok(values.into_series());
        }
    }

    let target = match col.qsv_type.as_deref() {
        Some("Integer") => DataType::Int64,
        Some("Float") => DataType::Float64,
        Some("Boolean") => DataType::Boolean,
        Some("Date") => return parse_temporal(strings, Temporal::Date),
        Some("DateTime") => {
            let series = parse_temporal(strings, Temporal::DateTime)?;
            if strings.iter().flatten().any(has_nanoseconds) {
                losses.add(format!(
                    "the nanoseconds of the datetimes in \"{name}\": a {} file holds {}",
                    format.name(),
                    unit_name(temporal_precision(format).0).1
                ));
            }
            return Ok(series);
        },
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

/// A datetime written finer than microseconds (more than 6 fraction digits
/// that aren't all zero), which is parsed to microseconds.
fn has_nanoseconds(cell: &str) -> bool {
    cell.rsplit_once('.')
        .is_some_and(|(_, fraction)| fraction.len() > 6 && fraction[6..].bytes().any(|b| b != b'0'))
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
            let micros: CliResult<Int64Chunked> = cells
                .map(|c| {
                    c.map(|c| {
                        ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S%.f"]
                            .iter()
                            .find_map(|f| chrono::NaiveDateTime::parse_from_str(c, f).ok())
                            .map(|dt| dt.and_utc().timestamp_micros())
                            .ok_or_else(|| bad(c))
                    })
                    .transpose()
                })
                .collect();
            micros?
                .into_series()
                .cast(&DataType::Datetime(TimeUnit::Microseconds, None))?
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

/// The finest unit each writer keeps, in nanoseconds, of a datetime & of a
/// time of day: SAS transport keeps microseconds (times as float seconds),
/// Stata milliseconds, and both SPSS writers whole seconds.
const fn temporal_precision(format: Format) -> (i64, i64) {
    match format {
        Format::Xpt(_) => (1_000, 1),
        Format::Dta => (1_000_000, 1_000_000),
        Format::Sav | Format::Por => (1_000_000_000, 1_000_000_000),
    }
}

fn unit_name(step_ns: i64) -> (&'static str, &'static str) {
    match step_ns {
        1_000_000_000 => ("fractions of a second", "whole seconds"),
        1_000_000 => ("microseconds", "milliseconds"),
        _ => ("nanoseconds", "microseconds"),
    }
}

/// Datetimes & times more precise than the writer keeps are a loss.
/// Datetimes go to each writer in the unit it reads: microseconds for SAS
/// transport files, milliseconds for the others - the SPSS portable writer
/// takes them as milliseconds whatever their unit.
fn fit_temporal(mut df: DataFrame, format: Format, losses: &mut Losses) -> CliResult<DataFrame> {
    let (datetime_step, time_step) = temporal_precision(format);
    let names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(ToString::to_string)
        .collect();
    for name in names {
        let series = df.column(&name)?.as_materialized_series().clone();
        let (step, kind) = match series.dtype() {
            DataType::Datetime(TimeUnit::Nanoseconds, _) => (datetime_step, "datetimes"),
            DataType::Datetime(unit, _) => {
                // in the column's own unit
                let per = match unit {
                    TimeUnit::Microseconds => 1_000,
                    _ => 1_000_000,
                };
                (datetime_step / per, "datetimes")
            },
            DataType::Time => (time_step, "times"),
            _ => continue,
        };
        let lossy = step > 1
            && series
                .to_physical_repr()
                .cast(&DataType::Int64)?
                .i64()?
                .iter()
                .flatten()
                .any(|v| v % step != 0);
        if lossy {
            let full = match series.dtype() {
                DataType::Datetime(TimeUnit::Microseconds, _) => step * 1_000,
                DataType::Datetime(TimeUnit::Milliseconds, _) => step * 1_000_000,
                _ => step,
            };
            let (what, held) = unit_name(full);
            losses.add(format!(
                "the {what} of the {kind} in \"{name}\": a {} file holds {held}",
                format.name()
            ));
        }
        if kind == "datetimes" {
            let unit = if matches!(format, Format::Xpt(_)) {
                TimeUnit::Microseconds
            } else {
                TimeUnit::Milliseconds
            };
            let fitted = series.cast(&DataType::Datetime(unit, None))?;
            df.replace(&name, fitted.into_column())?;
        }
    }
    Ok(df)
}

/// Names the format can't hold are an error (or, for Stata, a loss): the
/// writers would otherwise fail midway or rename columns silently.
fn check_names(
    df: &DataFrame,
    format: Format,
    pairs: &HashMap<String, String>,
    losses: &mut Losses,
) -> CliResult<()> {
    // the <name>_null columns are merged into their variables, not written
    let names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(ToString::to_string)
        .filter(|n| !pairs.values().any(|p| p == n))
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
    losses: &mut Losses,
) -> CliResult<(DataFrame, HashMap<String, String>)> {
    let mut numeric = HashMap::with_capacity(pairs.len());
    for (base, indicator) in pairs {
        if df.column(base)?.dtype() != &DataType::String {
            numeric.insert(base.clone(), indicator.clone());
            continue;
        }
        // under --sentinels-as label, a sentinel with a label is written as
        // its label; the others, & all under --sentinels-as value, as codes
        let col = dict.columns.get(base);
        if col.is_none_or(|c| c.missing_texts().next().is_none()) {
            // nothing to merge into: the sentinels are left out
            losses.add(format!(
                "the sentinels of \"{base}\": the dictionary declares no missing values"
            ));
            df.drop_in_place(indicator)?;
            continue;
        }
        let mut by_label: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut raw: Vec<&str> = Vec::new();
        if let Some(col) = col {
            for code in col.missing_texts() {
                let label = col.text_labels().find(|(c, _)| *c == code).map(|(_, l)| l);
                match label {
                    Some(label) if dict.sentinel_labels => {
                        by_label.entry(label).or_default().push(code);
                    },
                    _ => raw.push(code),
                }
            }
        }
        let mut ambiguous: Option<(String, String)> = None;
        let merged: StringChunked = {
            let values = df.column(base)?.str()?;
            let sentinels = df.column(indicator)?.str()?;
            values
                .iter()
                .zip(sentinels.iter())
                .map(|(value, sentinel)| {
                    value.or_else(|| {
                        sentinel.map(|s| {
                            let Some(codes) = by_label.get(s) else {
                                return s;
                            };
                            if codes.len() > 1 {
                                ambiguous.get_or_insert_with(|| {
                                    (s.to_string(), format!("the label of {}", codes.join(", ")))
                                });
                            } else if raw.contains(&s) {
                                ambiguous.get_or_insert_with(|| {
                                    (
                                        s.to_string(),
                                        format!("both a code and the label of {}", codes[0]),
                                    )
                                });
                            }
                            codes[0]
                        })
                    })
                })
                .collect()
        };
        if let Some((cell, why)) = ambiguous {
            return ambiguous_cell(indicator, &cell, &why);
        }
        df.replace(base, merged.with_name(base.as_str().into()).into_column())?;
        df.drop_in_place(indicator)?;
    }
    Ok((df, numeric))
}

fn csv_reader(input: &Path, delim: u8) -> LazyCsvReader {
    LazyCsvReader::new(PlRefPath::new(input.to_string_lossy().as_ref()))
        .with_has_header(true)
        .with_separator(delim)
}

/// Without a dictionary, the numeric columns are typed from the stats cache:
/// polars' own inference reads the whole file on one thread, trying date
/// patterns on every text cell. The text columns holding dates, times or
/// booleans are then recognised as polars would. Without usable stats (e.g.
/// `QSV_STATSCACHE_MODE=none`), polars infers the types itself.
fn read_inferred(input: &Path, delim: u8) -> CliResult<DataFrame> {
    if let Some(dtypes) = stats_dtypes(input, delim) {
        let mut lf = csv_reader(input, delim)
            .with_infer_schema_length(Some(0))
            .finish()?;
        // the stats engine & polars may split a line differently (e.g. under
        // QSV_COMMENT_CHAR), so a column count they disagree on isn't used
        if lf.collect_schema()?.len() == dtypes.len() {
            let df = csv_reader(input, delim)
                .with_infer_schema_length(Some(0))
                .with_dtype_overwrite_by_position(Some(Arc::new(dtypes)))
                .finish()?
                .collect()
                .map_err(|e| {
                    // the stats engine accepts fewer numbers than polars reads,
                    // so only a stale cache gets here
                    CliError::Other(format!(
                        "Could not read \"{}\" with the column types of its stats cache, which \
                         may be out of date: {e}\nDelete {}* & try again, or set \
                         QSV_STATSCACHE_MODE=none to infer the types without it.",
                        input.display(),
                        input.with_extension("stats.csv").display()
                    ))
                })?;
            return Ok(recognise_text_columns(df)?);
        }
        log::info!("writestat: the stats cache & polars disagree on the column count");
    }
    Ok(csv_reader(input, delim)
        .with_infer_schema_length(None)
        .with_try_parse_dates(true)
        .finish()?
        .collect()?)
}

/// The column types the stats cache gives, as read by polars: whole numbers as
/// Int64, other numbers as Float64 & the rest as text.
fn stats_dtypes(input: &Path, delim: u8) -> Option<Vec<DataType>> {
    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").ok()?,
        // dates are recognised as polars recognises them instead
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy:      false,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            Some(util::njobs(None)),
        flag_polars:          false,
        flag_no_headers:      false,
        // as polars reads it, rather than guessed from the file extension
        flag_delimiter:       Some(Delimiter(delim)),
        arg_input:            Some(input.to_string_lossy().into_owned()),
        flag_memcheck:        false,
        flag_output:          None,
    };
    match util::get_stats_records(&schema_args, util::StatsMode::PolarsSchema) {
        Ok((_, stats)) if !stats.is_empty() => Some(
            stats
                .iter()
                .map(|s| match s.r#type.as_str() {
                    "Integer" => DataType::Int64,
                    "Float" => DataType::Float64,
                    _ => DataType::String,
                })
                .collect(),
        ),
        Ok(_) => None,
        Err(e) => {
            log::info!("writestat: no stats cache, so polars infers the column types: {e}");
            None
        },
    }
}

/// Types the text columns that polars, inferring the types itself, would have
/// read as booleans, dates, datetimes or times.
fn recognise_text_columns(mut df: DataFrame) -> PolarsResult<DataFrame> {
    use rayon::prelude::*;
    let found: Vec<(usize, Series)> = df
        .columns()
        .par_iter()
        .enumerate()
        .filter_map(|(i, col)| recognise(col.str().ok()?).map(|s| (i, s)))
        .collect();
    for (i, series) in found {
        df.replace_column(i, series.into_column())?;
    }
    Ok(df)
}

/// polars types a column by every value, so the first decides what is tried &
/// a single value that doesn't fit leaves the column as text.
fn recognise(strings: &StringChunked) -> Option<Series> {
    use polars::chunked_array::temporal::string::{
        StringMethods, infer::infer_pattern_single, patterns::Pattern,
    };

    let first = strings.iter().flatten().next()?;
    let is_bool = |v: &str| v.eq_ignore_ascii_case("true") || v.eq_ignore_ascii_case("false");
    if is_bool(first) {
        return strings.iter().flatten().all(is_bool).then(|| {
            strings
                .iter()
                .map(|v| v.map(|v| v.eq_ignore_ascii_case("true")))
                .collect::<BooleanChunked>()
                .with_name(strings.name().clone())
                .into_series()
        });
    }
    let ambiguous = StringChunked::from_iter([Some("raise")]);
    let tu = TimeUnit::Microseconds;
    let converted = match infer_pattern_single(first)? {
        Pattern::DateDMY | Pattern::DateYMD => strings.as_date(None, true).ok()?.into_series(),
        Pattern::DatetimeDMY | Pattern::DatetimeYMD => strings
            .as_datetime(None, tu, true, false, None, &ambiguous)
            .ok()?
            .into_series(),
        Pattern::DatetimeYMDZ => strings
            .as_datetime(None, tu, true, true, Some(&TimeZone::UTC), &ambiguous)
            .ok()?
            .into_series(),
        Pattern::Time => strings.as_time(None, true).ok()?.into_series(),
    };
    (converted.null_count() == strings.null_count()).then_some(converted)
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
            // the numeric <name>_null columns become declared missing values
            // again (the text ones already have)
            let df = if pairs.is_empty() {
                df
            } else {
                merge_informative_null_columns(df, Some(&labels), Some(pairs))
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

#[cfg(test)]
mod tests {
    use super::{Code, ambiguity};

    #[test]
    fn ambiguity_of_tag_shaped_labels() {
        // a value labeled ".A", where raw tags (.A) may be written too
        assert!(ambiguity(".A", &[Code::Number(1.0)], true).is_some());
        assert!(ambiguity(".A", &[Code::Number(1.0)], false).is_none());
        // a tag labeled as itself is no ambiguity
        assert!(ambiguity(".a", &[Code::Tag(".a")], true).is_none());
        assert!(ambiguity("Refused", &[Code::Tag(".a")], true).is_none());
        // numbers & shared labels
        assert!(ambiguity("1.0", &[Code::Tag(".b")], false).is_some());
        assert!(ambiguity("1", &[Code::Number(1.0)], false).is_none());
        assert!(ambiguity("Yes", &[Code::Number(1.0), Code::Number(2.0)], false).is_some());
    }
}
