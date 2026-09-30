static USAGE: &str = r#"
Convert SAS, Stata & SPSS files to CSV.

EXPERIMENTAL: the readers this command is built on are young. Spot-check the
output against your source files before relying on a conversion, and please
report anything that looks wrong.

Supported input formats:
    SAS     .sas7bdat, .xpt, .xpt5, .xpt8
    Stata   .dta
    SPSS    .sav, .zsav, .por

Coded values are written as their underlying codes, not their labels, so the
conversion is lossless. Use --value-labels to decode them instead. SAS keeps
its value labels in a separate .sas7bcat format catalog, which --value-labels
finds next to the data file, or --sas7bcat names.

User-defined missing values ("sentinels") - SAS's .A to .Z & ._, Stata's .a
to .z, SPSS's declared missing codes - become empty cells by default, like any
other missing value. Use --sentinels-as to keep them.

The variable metadata these formats carry - variable labels, value labels,
missing-value codes, measure & display settings - can be dumped instead of the
data with --metadata.

  Convert a SAS dataset to CSV:
    qsv readstat data.sas7bdat > data.csv

  Convert an SPSS file, decoding coded values to their labels:
    qsv readstat --value-labels survey.sav -o survey.csv

  Keep the sentinel codes of a SAS dataset, in a <name>_null column after each
  numeric variable:
    qsv readstat --sentinels-as value data.sas7bdat

  Keep them in the variables' own columns instead:
    qsv readstat --sentinels-as value --sentinels-embedded data.sas7bdat

  Keep the sentinels of two SPSS variables, as their labels:
    qsv readstat --value-labels --sentinels-as label --sentinels-columns q1,q2 survey.sav

  Dump the variable dictionary of a Stata file:
    qsv readstat --metadata pretty-json panel.dta

  Pipe straight into another qsv command:
    qsv readstat data.sas7bdat | qsv stats

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_readstat.rs.

Usage:
    qsv readstat [options] [<input>]
    qsv readstat --help

readstat options:
    --metadata <fmt>       Dump variable metadata instead of the data.
                           Valid values: none, csv, json, pretty-json.
                           For SAS, the value labels are included when a
                           format catalog is used (see --value-labels).
                           [default: none]
    --value-labels         Decode coded values to their label strings
                           (e.g. 1 becomes "Male") instead of writing the
                           underlying codes. For SAS .sas7bdat files, the
                           labels come from the format catalog: the file
                           named by --sas7bcat, else <name>.sas7bcat or
                           formats.sas7bcat next to the data file. Not
                           supported for .xpt files.
    --sas7bcat <file>      The SAS format catalog (.sas7bcat) holding the
                           value labels of a .sas7bdat file. Implies
                           the option --value-labels.
    --compress-numeric     Write float variables that only ever hold whole
                           numbers as integers, without the ".0" (e.g. 3.0
                           becomes 3). SPSS stores every number as a float, so
                           this matters most for SPSS files. It is decided per
                           variable, over the whole file: one value like 2.5
                           keeps the ".0" on every row of that variable. The
                           file is read twice - once to check the values.
                           With sentinel labels embedded, a variable with a
                           label that reads as a number (e.g. "1.0") keeps its
                           ".0", so the label is not rewritten.
    --sentinels-as <what>  Keep sentinels instead of writing them as empty
                           cells. Each eligible variable gets a <name>_null
                           column right after it, holding the sentinel of each
                           row that has one & empty otherwise.
                           Valid values: none, value, label. [default: none]
                             value - the sentinel's code (e.g. .A or 99).
                             label - the sentinel's value label if it has one,
                                     else its code.
                           SAS supports value only - the catalog reader does
                           not yet tell which sentinel a label belongs to.
                           SPSS takes its sentinel labels from the value
                           labels, so for SPSS, label requires --value-labels
                           & value rules it out.
                           Not supported for .xpt & .por files. Tracking
                           sentinels makes SAS & Stata files read on a single
                           thread, so --jobs has no effect on them.
    --sentinels-embedded   Write each sentinel into its variable's own column
                           instead of a <name>_null column. Those columns then
                           mix numbers & sentinels. Requires --sentinels-as.
    --sentinels-columns <list>  Comma-separated variables to keep sentinels
                           for. Requires --sentinels-as. By default, every
                           eligible variable: the numeric ones for SAS &
                           Stata, those with declared missing values for SPSS.
    -j, --jobs <arg>       Number of reader threads. [default: 1]
                           Raising it speeds up large uncompressed files at the
                           cost of memory, as out-of-order chunks have to be
                           buffered to keep the rows in source order. Row order
                           is preserved either way.
    -b, --batch <size>     Number of rows to read into memory at a time.
                           Does not apply to SPSS portable (.por) files - they
                           have no chunked reader upstream, so they are read
                           whole & memory scales with the file.
                           [default: 50000]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The delimiter to use when writing CSV data.
                           Must be a single character. [default: ,]
"#;

use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use polars::prelude::{
    CsvWriter, DataFrame, DataType, IntoSeries, PlSmallStr, PolarsResult, Schema, SerWriter,
    StringChunked,
};
use polars_readstat_rs::{
    CatalogKey, InformativeNullColumns, InformativeNullMode, InformativeNullOpts, ReadStatFormat,
    ScanOptions, readstat_batch_iter, readstat_metadata_json, readstat_schema,
};
use serde::Deserialize;

use crate::{CliResult, cmd::joinp::tsvssv_delim, config::Delimiter, util};

#[derive(Deserialize)]
struct Args {
    arg_input:               Option<String>,
    flag_metadata:           String,
    flag_value_labels:       bool,
    flag_compress_numeric:   bool,
    flag_sas7bcat:           Option<String>,
    flag_sentinels_as:       String,
    flag_sentinels_embedded: bool,
    flag_sentinels_columns:  Option<String>,
    flag_jobs:               Option<usize>,
    flag_batch:              usize,
    flag_output:             Option<String>,
    flag_delimiter:          Option<Delimiter>,
}

/// The input formats this command accepts.
///
/// This deliberately does NOT delegate to the reader crate's own extension
/// sniffing, which maps `.sas7bcat` (a *format catalog*, not a dataset) onto
/// the SAS reader and has no entry for `.por` at all.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    Sas,
    SasXpt,
    Stata,
    Spss,
    /// SPSS portable. Readable, but the crate has no chunked-streaming
    /// implementation for it, so it is read whole.
    SpssPor,
}

impl Format {
    fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "sas7bdat" => Some(Self::Sas),
            "xpt" | "xpt5" | "xpt8" => Some(Self::SasXpt),
            "dta" => Some(Self::Stata),
            "sav" | "zsav" => Some(Self::Spss),
            "por" => Some(Self::SpssPor),
            _ => None,
        }
    }

    /// The reader crate's format tag. `None` for `.por`, which has no variant.
    const fn readstat_format(self) -> Option<ReadStatFormat> {
        match self {
            Self::Sas => Some(ReadStatFormat::Sas),
            Self::SasXpt => Some(ReadStatFormat::SasXpt),
            Self::Stata => Some(ReadStatFormat::Stata),
            Self::Spss => Some(ReadStatFormat::Spss),
            Self::SpssPor => None,
        }
    }

    /// True for the formats whose readers report user-defined missing values.
    /// The XPT & POR readers accept the option and silently ignore it.
    const fn honors_sentinels(self) -> bool {
        matches!(self, Self::Sas | Self::Stata | Self::Spss)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SentinelsAs {
    None,
    Value,
    Label,
}

impl SentinelsAs {
    fn parse(s: &str) -> CliResult<Self> {
        match s.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "value" => Ok(Self::Value),
            "label" => Ok(Self::Label),
            _ => fail_incorrectusage_clierror!(
                "\"{s}\" is not a valid --sentinels-as value. Valid values: none, value, label."
            ),
        }
    }
}

#[derive(PartialEq, Eq)]
enum MetadataMode {
    None,
    Csv,
    Json,
    PrettyJson,
}

impl MetadataMode {
    fn parse(s: &str) -> CliResult<Self> {
        match s.to_ascii_lowercase().as_str() {
            "none" => Ok(Self::None),
            "csv" => Ok(Self::Csv),
            "json" => Ok(Self::Json),
            "pretty-json" | "prettyjson" => Ok(Self::PrettyJson),
            _ => fail_incorrectusage_clierror!(
                "\"{s}\" is not a valid --metadata format. Valid values: none, csv, json, \
                 pretty-json."
            ),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let metadata_mode = MetadataMode::parse(&args.flag_metadata)?;

    let Some(input) = args.arg_input.as_ref() else {
        return fail_incorrectusage_clierror!(
            "qsv readstat needs a file to read. These are binary formats identified by their file \
             extension, so reading from stdin is not supported."
        );
    };
    let path = Path::new(input);
    if !path.exists() {
        return fail_incorrectusage_clierror!("Cannot find input file \"{input}\".");
    }

    let Some(format) = Format::from_path(path) else {
        let ext = path.extension().map_or_else(
            || "no extension".to_string(),
            |e| format!("\".{}\"", e.to_string_lossy()),
        );
        if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("sas7bcat"))
        {
            return fail_incorrectusage_clierror!(
                "\"{input}\" is a SAS format catalog, not a dataset. Pass the .sas7bdat file, \
                 with this catalog as --sas7bcat."
            );
        }
        return fail_incorrectusage_clierror!(
            "Don't know how to read {ext}. qsv readstat handles .sas7bdat, .xpt/.xpt5/.xpt8, \
             .dta, .sav/.zsav & .por files."
        );
    };

    if args.flag_sas7bcat.is_some() && format != Format::Sas {
        return fail_incorrectusage_clierror!("--sas7bcat applies to SAS .sas7bdat files only.");
    }
    if args.flag_value_labels && format == Format::SasXpt {
        return fail_incorrectusage_clierror!(
            "--value-labels is not supported for SAS transport (.xpt) files."
        );
    }
    // before --output is created, so a rejected request leaves it untouched
    let sas_labels = SasLabels::resolve(&args, input, path, format)?;

    let sentinels_as = SentinelsAs::parse(&args.flag_sentinels_as)?;
    if sentinels_as != SentinelsAs::None && metadata_mode != MetadataMode::None {
        return fail_incorrectusage_clierror!(
            "--sentinels-as applies to the data, not to --metadata. The metadata already lists \
             each variable's missing-value codes."
        );
    }
    if args.flag_compress_numeric && metadata_mode != MetadataMode::None {
        return fail_incorrectusage_clierror!(
            "--compress-numeric applies to the data, not to --metadata."
        );
    }
    // before --output is created, so a rejected request leaves it untouched
    let sentinels = sentinel_opts(&args, input, path, format, sentinels_as)?;

    let mut delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = std::env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };

    let w = match args.flag_output.as_ref() {
        Some(p) => {
            delim = tsvssv_delim(p, delim);
            Box::new(File::create(p)?) as Box<dyn Write>
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    let mut w = io::BufWriter::with_capacity(crate::config::DEFAULT_WTR_BUFFER_CAPACITY, w);

    if metadata_mode == MetadataMode::None {
        let rows = write_data(
            &args,
            path,
            format,
            sentinels,
            sas_labels.as_ref(),
            delim,
            &mut w,
        )?;
        w.flush()?;
        winfo!("{rows} row{} exported.", if rows == 1 { "" } else { "s" });
    } else {
        write_metadata(
            path,
            format,
            &metadata_mode,
            sas_labels.as_ref(),
            delim,
            &mut w,
        )?;
        w.flush()?;
    }

    Ok(())
}

/// A suffix no variable name can carry, so probing with it never collides.
const PROBE_SUFFIX: &str = "\u{0}qsv-sentinel-probe";

/// Validate the --sentinels-* options & turn them into the reader's options.
///
/// Every check runs here, against the file's metadata only, because the
/// reader itself accepts a bad request silently: the XPT & POR readers ignore
/// the option outright, and unknown or ineligible --sentinels-columns names are
/// dropped without a word.
fn sentinel_opts(
    args: &Args,
    input: &str,
    path: &Path,
    format: Format,
    sentinels_as: SentinelsAs,
) -> CliResult<Option<InformativeNullOpts>> {
    if sentinels_as == SentinelsAs::None {
        if args.flag_sentinels_embedded || args.flag_sentinels_columns.is_some() {
            return fail_incorrectusage_clierror!(
                "--sentinels-embedded & --sentinels-columns require --sentinels-as value or \
                 --sentinels-as label."
            );
        }
        return Ok(None);
    }

    let rs_format = match format.readstat_format() {
        Some(rs_format) if format.honors_sentinels() => rs_format,
        _ => {
            return fail_incorrectusage_clierror!(
                "--sentinels-as is not supported for SAS transport (.xpt) or SPSS portable (.por) \
                 files. Their readers do not report user-defined missing values."
            );
        },
    };
    match (format, sentinels_as) {
        (Format::Sas, SentinelsAs::Label) => {
            return fail_incorrectusage_clierror!(
                "--sentinels-as label is not supported for SAS files yet: the .sas7bcat reader \
                 does not tell which sentinel (.A-.Z, ._) a label belongs to \
                 (https://github.com/jrothbaum/polars_readstat/issues/65). Use --sentinels-as \
                 value to keep the sentinel codes."
            );
        },
        // The SPSS reader labels sentinels exactly when it decodes value labels,
        // whatever --sentinels-as asks for.
        (Format::Spss, SentinelsAs::Label) if !args.flag_value_labels => {
            return fail_incorrectusage_clierror!(
                "--sentinels-as label needs --value-labels on SPSS files: the sentinels' labels \
                 come from the variables' value labels."
            );
        },
        (Format::Spss, SentinelsAs::Value) if args.flag_value_labels => {
            return fail_incorrectusage_clierror!(
                "--sentinels-as value cannot be combined with --value-labels on SPSS files, as \
                 that writes the sentinels' labels. Use --sentinels-as label instead."
            );
        },
        _ => {},
    }

    let columns = match args.flag_sentinels_columns.as_deref() {
        None => InformativeNullColumns::All,
        Some(list) => {
            let mut names: Vec<String> = Vec::new();
            for name in list.split(',').map(str::trim) {
                if name.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--sentinels-columns \"{list}\" has an empty variable name."
                    );
                }
                if !names.iter().any(|n| n == name) {
                    names.push(name.to_string());
                }
            }
            check_sentinel_columns(args, input, path, format, rs_format, &names)?;
            InformativeNullColumns::Selected(names)
        },
    };

    let null_opts = InformativeNullOpts {
        columns,
        mode: if args.flag_sentinels_embedded {
            InformativeNullMode::MergedString
        } else {
            InformativeNullMode::SeparateColumn {
                suffix: "_null".to_string(),
            }
        },
        use_value_labels: sentinels_as == SentinelsAs::Label,
    };

    // Surface a <name>_null collision now, with advice the user can act on.
    let check = ScanOptions {
        value_labels_as_strings: Some(args.flag_value_labels),
        informative_nulls: Some(null_opts.clone()),
        ..ScanOptions::default()
    };
    if let Err(e) = readstat_schema(path, Some(check), Some(rs_format)) {
        let msg = e.to_string();
        if msg.contains("conflicts with an existing column") {
            let taken = msg.split('\'').nth(1).unwrap_or("<name>_null");
            return fail_incorrectusage_clierror!(
                "Cannot keep sentinels: the file already has a variable named \"{taken}\", which \
                 is where a sentinel column would go. Leave its variable out with \
                 --sentinels-columns."
            );
        }
        return Err(e.into());
    }

    if args.flag_jobs.unwrap_or(1) > 1 && matches!(format, Format::Sas | Format::Stata) {
        wwarn!(
            "--jobs has no effect with --sentinels-as on SAS & Stata files: tracking sentinels \
             reads them on a single thread."
        );
    }

    Ok(Some(null_opts))
}

/// Reject --sentinels-columns names the reader would silently drop: names not
/// in the file, and variables that cannot hold a sentinel.
///
/// Eligibility differs by format, so rather than restating the reader's rules
/// this asks the reader itself: a name is eligible iff selecting it adds an
/// indicator column to the schema.
fn check_sentinel_columns(
    args: &Args,
    input: &str,
    path: &Path,
    format: Format,
    rs_format: ReadStatFormat,
    names: &[String],
) -> CliResult<()> {
    let probe = ScanOptions {
        value_labels_as_strings: Some(args.flag_value_labels),
        informative_nulls: Some(InformativeNullOpts {
            columns:          InformativeNullColumns::Selected(names.to_vec()),
            mode:             InformativeNullMode::SeparateColumn {
                suffix: PROBE_SUFFIX.to_string(),
            },
            use_value_labels: false,
        }),
        ..ScanOptions::default()
    };
    let schema = readstat_schema(path, Some(probe), Some(rs_format))?;

    let quoted = |v: Vec<&String>| {
        v.iter()
            .map(|n| format!("\"{n}\""))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let unknown: Vec<&String> = names.iter().filter(|n| !schema.contains(n)).collect();
    if !unknown.is_empty() {
        return fail_incorrectusage_clierror!(
            "--sentinels-columns: \"{input}\" has no variable named {}.",
            quoted(unknown)
        );
    }
    let ineligible: Vec<&String> = names
        .iter()
        .filter(|n| !schema.contains(&format!("{n}{PROBE_SUFFIX}")))
        .collect();
    if !ineligible.is_empty() {
        let rule = if format == Format::Spss {
            "Only variables with declared missing values"
        } else {
            "Only numeric variables"
        };
        return fail_incorrectusage_clierror!(
            "--sentinels-columns: {} cannot hold sentinels. {rule} can.",
            quoted(ineligible)
        );
    }
    Ok(())
}

/// Above these magnitudes a float's spacing is 1 or more, so "whole number"
/// says nothing about the data: such columns keep their float form.
const F64_WHOLE_LIMIT: f64 = 9_007_199_254_740_992.0; // 2^53
const F32_WHOLE_LIMIT: f64 = 16_777_216.0; // 2^24

fn is_whole(v: f64, limit: f64) -> bool {
    v.is_finite() && v.fract() == 0.0 && v.abs() <= limit
}

/// Tracks which float columns have held only whole numbers so far.
struct WholeNumberScan {
    candidates: Vec<(PlSmallStr, bool)>,
}

impl WholeNumberScan {
    fn new(schema: &Schema) -> Self {
        Self {
            candidates: schema
                .iter()
                .filter(|(_, dt)| dt.is_float())
                .map(|(name, _)| (name.clone(), true))
                .collect(),
        }
    }

    fn names(&self) -> Vec<String> {
        self.candidates.iter().map(|(n, _)| n.to_string()).collect()
    }

    fn update(&mut self, df: &DataFrame) -> PolarsResult<()> {
        for (name, whole) in &mut self.candidates {
            if !*whole {
                continue;
            }
            let col = df.column(name)?.as_materialized_series();
            *whole = match col.dtype() {
                DataType::Float64 => col
                    .f64()?
                    .iter()
                    .flatten()
                    .all(|v| is_whole(v, F64_WHOLE_LIMIT)),
                DataType::Float32 => col
                    .f32()?
                    .iter()
                    .flatten()
                    .all(|v| is_whole(f64::from(v), F32_WHOLE_LIMIT)),
                _ => false,
            };
        }
        Ok(())
    }

    fn columns(self) -> Vec<PlSmallStr> {
        self.candidates
            .into_iter()
            .filter_map(|(name, whole)| whole.then_some(name))
            .collect()
    }
}

/// `--compress-numeric` pass 1: the float columns whose every value, across
/// the whole file, is a whole number. Upstream's own `compress_numeric` can't
/// be used: it decides per batch, so one column can print `2` in one batch and
/// `2.0` in the next, and it turns 0/1 & all-null columns into Booleans.
///
/// Sentinel columns are left out of the scan (`informative_nulls: None`), so a
/// variable that `--sentinels-embedded` turns into text is judged by its
/// numeric values alone.
fn whole_number_columns(
    path: &Path,
    rs_format: ReadStatFormat,
    opts: &ScanOptions,
    batch_size: Option<usize>,
) -> CliResult<Vec<PlSmallStr>> {
    let scan_opts = ScanOptions {
        informative_nulls: None,
        preserve_order: Some(false),
        ..opts.clone()
    };
    let schema = readstat_schema(path, Some(scan_opts.clone()), Some(rs_format))?;
    let mut scan = WholeNumberScan::new(&schema);
    if scan.candidates.is_empty() {
        return Ok(Vec::new());
    }
    // Only the float columns are read, except from XPT files: the XPT reader
    // (polars-readstat-rs 0.23.2) decodes projected columns from the file's
    // first columns' offsets (jrothbaum/polars_readstat#64), so it reads them all.
    let columns = (!matches!(rs_format, ReadStatFormat::SasXpt)).then(|| scan.names());
    let batches = readstat_batch_iter(
        path,
        Some(scan_opts),
        Some(rs_format),
        columns,
        None,
        batch_size,
    )?;
    for batch in batches {
        scan.update(&batch?)?;
    }
    Ok(scan.columns())
}

/// `--compress-numeric` pass 2: write the `whole` columns without their ".0".
/// A float column becomes Int64 (exact, as pass 1 bounded its values). A column
/// `--sentinels-embedded` turned into text has each number rewritten in place,
/// leaving its sentinels alone.
fn drop_whole_number_fraction(df: &mut DataFrame, whole: &[PlSmallStr]) -> PolarsResult<()> {
    for name in whole {
        let Ok(col) = df.column(name) else {
            continue;
        };
        let col = col.as_materialized_series();
        let fixed = match col.dtype() {
            dt if dt.is_float() => col.cast(&DataType::Int64)?,
            DataType::String => col
                .str()?
                .iter()
                .map(|v| v.map(|s| whole_number_text(s).unwrap_or_else(|| s.to_string())))
                .collect::<StringChunked>()
                .with_name(name.clone())
                .into_series(),
            _ => continue,
        };
        df.replace(name, fixed.into())?;
    }
    Ok(())
}

/// The integer text of `s` if it reads as a whole number, e.g. "3.0" -> "3".
fn whole_number_text(s: &str) -> Option<String> {
    match s.parse::<f64>() {
        #[allow(clippy::cast_possible_truncation)]
        Ok(n) if is_whole(n, F64_WHOLE_LIMIT) => Some((n as i64).to_string()),
        _ => None,
    }
}

/// Under `--sentinels-as label --sentinels-embedded` a variable's text column
/// mixes its numbers with its sentinels' labels, and nothing tells them apart.
/// The variables with a label that the ".0" rewrite would change (e.g.
/// "001.0") are returned, so they keep their ".0" rather than lose the label.
fn numeric_label_variables(path: &Path, rs_format: ReadStatFormat) -> CliResult<Vec<String>> {
    let meta = readstat_metadata_json(path, Some(rs_format)).map_err(|e| {
        crate::CliError::Other(format!(
            "Could not read the metadata of \"{}\": {e}",
            path.display()
        ))
    })?;
    let meta: serde_json::Value = serde_json::from_str(&meta)?;
    Ok(meta["variables"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|v| {
            // only sentinel labels are embedded; the metadata keys them as
            // MISSING_a..MISSING_z, ordinary values by their code
            v["value_labels"].as_object().is_some_and(|labels| {
                labels
                    .iter()
                    .filter(|(code, _)| code.starts_with("MISSING_"))
                    .filter_map(|(_, label)| label.as_str())
                    .any(|l| whole_number_text(l).is_some_and(|t| t != l))
            })
        })
        .filter_map(|v| v["name"].as_str().map(str::to_string))
        .collect())
}

/// One SAS format's labels, as read from a `.sas7bcat` catalog.
#[derive(Clone, Default)]
struct FormatLabels {
    /// A numeric format; character formats are named with a leading `$`.
    numeric_format: bool,
    numeric:        Vec<(f64, String)>,
    text:           HashMap<String, String>,
}

impl FormatLabels {
    fn number(&self, v: f64) -> String {
        self.numeric
            .iter()
            .find(|(code, _)| *code == v)
            .map_or_else(|| number_text(v), |(_, label)| label.clone())
    }

    /// A cell of a labeled column that is already text: a character variable,
    /// or a numeric one that `--sentinels-embedded` turned into text. In the
    /// latter, a cell that isn't a number is a sentinel and is left alone.
    fn text(&self, s: &str) -> String {
        if let Some(label) = self.text.get(s.trim_end()) {
            return label.clone();
        }
        if self.numeric_format
            && let Ok(v) = s.parse::<f64>()
        {
            return self.number(v);
        }
        s.to_string()
    }
}

/// How a number in a labeled column is written when it has no label: like the
/// Stata & SPSS readers do, whole numbers without a ".0".
fn number_text(v: f64) -> String {
    if is_whole(v, F64_WHOLE_LIMIT) {
        #[allow(clippy::cast_possible_truncation)]
        let whole = v as i64;
        whole.to_string()
    } else {
        v.to_string()
    }
}

/// The value labels of a SAS dataset, from its `.sas7bcat` format catalog.
///
/// The reader crate only parses the catalog (format name -> labels), so the
/// join to the dataset's columns is done here: each column's format, from
/// the dataset's metadata, names its catalog entry.
struct SasLabels {
    columns: Vec<(PlSmallStr, FormatLabels)>,
}

impl SasLabels {
    /// The catalog is used when `--sas7bcat` names one, or when
    /// `--value-labels` is given, in which case it is looked for next to the
    /// data file as `<name>.sas7bcat`, then `formats.sas7bcat`.
    fn resolve(args: &Args, input: &str, path: &Path, format: Format) -> CliResult<Option<Self>> {
        if format != Format::Sas || !(args.flag_value_labels || args.flag_sas7bcat.is_some()) {
            return Ok(None);
        }
        let catalog = if let Some(named) = &args.flag_sas7bcat {
            let named = PathBuf::from(named);
            if !named.is_file() {
                return fail_incorrectusage_clierror!(
                    "Cannot find the SAS format catalog \"{}\".",
                    named.display()
                );
            }
            named
        } else {
            let beside = path.with_extension("sas7bcat");
            let shared = path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join("formats.sas7bcat");
            match [&beside, &shared].into_iter().find(|p| p.is_file()) {
                Some(found) => found.clone(),
                None => {
                    return fail_incorrectusage_clierror!(
                        "--value-labels on a SAS file needs its .sas7bcat format catalog, and \
                         there is none at \"{}\" or \"{}\". Pass it with --sas7bcat.",
                        beside.display(),
                        shared.display()
                    );
                },
            }
        };

        let parsed = polars_readstat_rs::read_sas7bcat(&catalog).map_err(|e| {
            crate::CliError::Other(format!(
                "Could not read the SAS format catalog \"{}\": {e}",
                catalog.display()
            ))
        })?;
        let mut formats: HashMap<String, FormatLabels> = HashMap::new();
        for (name, entries) in parsed {
            let labels = formats.entry(format_key(&name)).or_default();
            labels.numeric_format = !name.starts_with('$');
            for (code, label) in entries {
                match code {
                    CatalogKey::Numeric(v) => labels.numeric.push((v, label)),
                    CatalogKey::Text(s) => {
                        labels.text.insert(s.trim_end().to_string(), label);
                    },
                    // the reader drops which sentinel (.A-.Z, ._) a label is for
                    CatalogKey::Missing => {},
                }
            }
        }

        let meta = readstat_metadata_json(path, Some(ReadStatFormat::Sas)).map_err(|e| {
            crate::CliError::Other(format!(
                "Could not read the metadata of \"{}\": {e}",
                path.display()
            ))
        })?;
        let meta: serde_json::Value = serde_json::from_str(&meta)?;
        let columns: Vec<(PlSmallStr, FormatLabels)> = meta["columns"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|col| {
                let name = col["name"].as_str()?;
                let labels = formats.get(&format_key(col["format"].as_str()?))?;
                Some((PlSmallStr::from(name), labels.clone()))
            })
            .collect();

        if columns.is_empty() {
            wwarn!(
                "None of the formats of \"{input}\" are in the SAS format catalog \"{}\", so no \
                 values were labeled.",
                catalog.display()
            );
        } else {
            winfo!("Using SAS format catalog \"{}\".", catalog.display());
        }
        Ok(Some(Self { columns }))
    }

    fn get(&self, name: &str) -> Option<&FormatLabels> {
        self.columns
            .iter()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, labels)| labels)
    }

    /// Replace the labeled columns' codes with their labels. They become text.
    fn apply(&self, df: &mut DataFrame) -> PolarsResult<()> {
        for (name, labels) in &self.columns {
            let Ok(col) = df.column(name) else {
                continue;
            };
            let col = col.as_materialized_series();
            let labeled: StringChunked = match col.dtype() {
                dt if dt.is_float() => col
                    .cast(&DataType::Float64)?
                    .f64()?
                    .iter()
                    .map(|v| v.map(|v| labels.number(v)))
                    .collect(),
                DataType::String => col
                    .str()?
                    .iter()
                    .map(|v| v.map(|s| labels.text(s)))
                    .collect(),
                _ => continue,
            };
            df.replace(name, labeled.with_name(name.clone()).into_series().into())?;
        }
        Ok(())
    }

    /// `--metadata`: add each labeled column's labels, keyed by code as the
    /// Stata & SPSS metadata key theirs.
    fn annotate(&self, json: &str) -> CliResult<String> {
        let mut meta: serde_json::Value = serde_json::from_str(json)?;
        if let Some(cols) = meta["columns"].as_array_mut() {
            for col in cols {
                let Some(labels) = col["name"].as_str().and_then(|n| self.get(n)) else {
                    continue;
                };
                let mut map = serde_json::Map::new();
                for (code, label) in &labels.numeric {
                    map.insert(number_text(*code), label.clone().into());
                }
                let mut text: Vec<_> = labels.text.iter().collect();
                text.sort();
                for (code, label) in text {
                    map.insert(code.clone(), label.clone().into());
                }
                // a format labeling only sentinels has nothing to show until
                // the reader says which sentinel each label is for
                if !map.is_empty() {
                    col["value_labels"] = map.into();
                }
            }
        }
        Ok(serde_json::to_string(&meta)?)
    }
}

/// A SAS format name as the catalog keys it: without the trailing width &
/// decimals (`WORKSHOP5.` is `WORKSHOP`), upper-cased. A SAS format name
/// cannot end in a digit, so trailing digits are always the width.
fn format_key(format: &str) -> String {
    format
        .trim_end_matches(|c: char| c.is_ascii_digit() || c == '.')
        .to_ascii_uppercase()
}

/// Stream the file to CSV, one batch at a time, so memory stays bounded.
fn write_data<W: Write>(
    args: &Args,
    path: &Path,
    format: Format,
    sentinels: Option<InformativeNullOpts>,
    sas_labels: Option<&SasLabels>,
    delim: u8,
    w: &mut W,
) -> CliResult<u64> {
    let float_precision = *crate::config::POLARS_FLOAT_PRECISION.get_or_init(|| {
        std::env::var("QSV_POLARS_FLOAT_PRECISION")
            .ok()
            .and_then(|s| s.parse().ok())
    });

    // Two independent guards keep the rows in source order, because the readers
    // only stay ordered by construction on a single thread: with more than one
    // thread (and an uncompressed file over 1k rows) they switch to a
    // chunk-interleaving path that is ordered only when `preserve_order` makes
    // it buffer chunks back into sequence. Defaulting to one thread therefore
    // also keeps memory flat; `--jobs` trades that for speed, still ordered.
    let embedded_labels = sentinels
        .as_ref()
        .is_some_and(|s| s.use_value_labels && matches!(s.mode, InformativeNullMode::MergedString));
    let opts = ScanOptions {
        threads: Some(args.flag_jobs.unwrap_or(1).max(1)),
        chunk_size: (args.flag_batch > 0).then_some(args.flag_batch),
        value_labels_as_strings: Some(args.flag_value_labels),
        preserve_order: Some(true),
        informative_nulls: sentinels,
        ..ScanOptions::default()
    };

    // `.por` has no chunked reader upstream, so it is read whole. Every other
    // format streams, and takes its schema from the file's own variable
    // dictionary rather than from the first batch - that way the header is
    // written exactly once even when the file has no rows at all.
    let rs_format = format.readstat_format();
    let batch_size = (args.flag_batch > 0).then_some(args.flag_batch);
    let (mut schema, por_df) = if let Some(rs_format) = rs_format {
        (
            readstat_schema(path, Some(opts.clone()), Some(rs_format))?,
            None,
        )
    } else {
        let df = polars_readstat_rs::scan_por(path, opts.clone())?.collect()?;
        (df.schema().clone(), Some(df))
    };

    let mut whole = if !args.flag_compress_numeric {
        Vec::new()
    } else if let Some(rs_format) = rs_format {
        whole_number_columns(path, rs_format, &opts, batch_size)?
    } else if let Some(df) = &por_df {
        let mut whole = WholeNumberScan::new(df.schema());
        whole.update(df)?;
        whole.columns()
    } else {
        Vec::new()
    };
    if let Some(rs_format) = rs_format
        && embedded_labels
        && !whole.is_empty()
    {
        let risky = numeric_label_variables(path, rs_format)?;
        whole.retain(|name| {
            schema.get(name) != Some(&DataType::String) || !risky.iter().any(|r| r == name.as_str())
        });
    }
    if let Some(labels) = sas_labels {
        // labeled columns are text, their unlabeled numbers already without ".0"
        whole.retain(|name| labels.get(name).is_none());
        for (name, _) in &labels.columns {
            if schema.contains(name) {
                std::sync::Arc::make_mut(&mut schema).set_dtype(name, DataType::String);
            }
        }
    }
    for name in &whole {
        if schema.get(name).is_some_and(DataType::is_float) {
            std::sync::Arc::make_mut(&mut schema).set_dtype(name, DataType::Int64);
        }
    }

    let mut wtr = CsvWriter::new(w)
        .include_header(true)
        .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
        .with_separator(delim)
        .with_float_precision(float_precision)
        .batched(&schema)?;

    let mut rows = 0_u64;
    if let Some(rs_format) = rs_format {
        let batches =
            readstat_batch_iter(path, Some(opts), Some(rs_format), None, None, batch_size)?;
        for batch in batches {
            let mut df = batch?;
            if let Some(labels) = sas_labels {
                labels.apply(&mut df)?;
            }
            drop_whole_number_fraction(&mut df, &whole)?;
            rows += df.height() as u64;
            // write_batch panics on unaligned chunks
            df.align_chunks();
            wtr.write_batch(&df)?;
        }
    } else if let Some(mut df) = por_df {
        drop_whole_number_fraction(&mut df, &whole)?;
        rows += df.height() as u64;
        df.align_chunks();
        wtr.write_batch(&df)?;
    }
    // writes the header if no batch did
    wtr.finish()?;

    Ok(rows)
}

fn write_metadata<W: Write>(
    path: &Path,
    format: Format,
    mode: &MetadataMode,
    sas_labels: Option<&SasLabels>,
    delim: u8,
    w: &mut W,
) -> CliResult<()> {
    let mut json = if let Some(rs_format) = format.readstat_format() {
        readstat_metadata_json(path, Some(rs_format))
    } else {
        polars_readstat_rs::metadata_json_por(path).map_err(|e| e.to_string())
    }
    .map_err(|e| {
        crate::CliError::Other(format!(
            "Could not read the metadata of \"{}\": {e}",
            path.display()
        ))
    })?;
    if let Some(labels) = sas_labels {
        json = labels.annotate(&json)?;
    }

    match mode {
        MetadataMode::Json => {
            w.write_all(json.as_bytes())?;
            w.write_all(b"\n")?;
        },
        MetadataMode::PrettyJson => {
            let value: serde_json::Value = serde_json::from_str(&json)?;
            serde_json::to_writer_pretty(&mut *w, &value)?;
            w.write_all(b"\n")?;
        },
        MetadataMode::Csv => metadata_to_csv(&json, delim, w)?,
        MetadataMode::None => unreachable!("handled by the caller"),
    }
    Ok(())
}

/// Flatten the per-variable metadata into a CSV table.
///
/// The reader crate names the array `columns` for SAS and `variables` for
/// Stata & SPSS, and the keys within it differ per format too, so the columns
/// are derived from the data rather than hardcoded. Nested values (value-label
/// maps, missing-value lists) are written as JSON text.
fn metadata_to_csv<W: Write>(json: &str, delim: u8, w: &mut W) -> CliResult<()> {
    use serde_json::Value;

    let value: Value = serde_json::from_str(json)?;
    let vars = value
        .get("variables")
        .or_else(|| value.get("columns"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            crate::CliError::Other(
                "The metadata has no \"variables\"/\"columns\" list to tabulate.".to_string(),
            )
        })?;

    // union of keys, in first-seen order
    let mut headers: Vec<&str> = Vec::new();
    for var in vars {
        if let Some(obj) = var.as_object() {
            for key in obj.keys() {
                if !headers.iter().any(|h| h == key) {
                    headers.push(key);
                }
            }
        }
    }

    let mut wtr = csv::WriterBuilder::new().delimiter(delim).from_writer(w);
    wtr.write_record(&headers)?;
    for var in vars {
        let record: Vec<String> = headers
            .iter()
            .map(|h| match var.get(*h) {
                None | Some(Value::Null) => String::new(),
                Some(Value::String(s)) => s.clone(),
                Some(v) => v.to_string(),
            })
            .collect();
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}
