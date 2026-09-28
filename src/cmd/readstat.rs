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
conversion is lossless. Use --value-labels to decode them instead.

User-defined missing values ("sentinels") - SAS's .A to .Z & ._, SPSS's
declared missing codes - become empty cells by default, like any other missing
value. Use --sentinels-as to keep them.

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
                           [default: none]
    --value-labels         Decode coded values to their label strings
                           (e.g. 1 becomes "Male") instead of writing the
                           underlying codes. Stata & SPSS only - SAS keeps its
                           value labels in a separate .sas7bcat catalog, which
                           this command does not read yet.
    --sentinels-as <what>  Keep sentinels instead of writing them as empty
                           cells. Each eligible variable gets a <name>_null
                           column right after it, holding the sentinel of each
                           row that has one & empty otherwise.
                           Valid values: none, value, label. [default: none]
                             value - the sentinel's code (e.g. .A or 99).
                             label - the sentinel's value label if it has one,
                                     else its code.
                           SAS supports value only - its labels live in a
                           .sas7bcat catalog. SPSS takes its sentinel labels
                           from the value labels, so for SPSS, label requires
                           the --value-labels option & value rules it out.
                           Not supported yet for Stata files, nor for .xpt &
                           .por files. Tracking sentinels makes SAS files read
                           on a single thread, so --jobs has no effect on them.
    --sentinels-embedded   Write each sentinel into its variable's own column
                           instead of a <name>_null column. Those columns then
                           mix numbers & sentinels. Requires --sentinels-as.
    --sentinels-columns <list>  Comma-separated variables to keep sentinels
                           for. Requires --sentinels-as. By default, every
                           eligible variable: the numeric ones for SAS, those
                           with declared missing values for SPSS.
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
    path::Path,
};

use polars::prelude::{
    CsvWriter, DataFrame, DataType, Float32Chunked, Float64Chunked, IntoSeries, PlSmallStr,
    PolarsResult, SerWriter, StringChunked,
};
use polars_readstat_rs::{
    InformativeNullColumns, InformativeNullMode, InformativeNullOpts, ReadStatFormat, ScanOptions,
    readstat_batch_iter, readstat_metadata_json, readstat_schema,
};
use serde::Deserialize;

use crate::{CliResult, cmd::joinp::tsvssv_delim, config::Delimiter, util};

#[derive(Deserialize)]
struct Args {
    arg_input:               Option<String>,
    flag_metadata:           String,
    flag_value_labels:       bool,
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

    /// True for the formats whose readers apply value labels while decoding.
    const fn honors_value_labels(self) -> bool {
        matches!(self, Self::Stata | Self::Spss | Self::SpssPor)
    }

    /// True for the formats whose readers report user-defined missing values.
    /// The XPT & POR readers accept the option and silently ignore it; the
    /// Stata reader drops some sentinels (see `sentinel_opts`).
    const fn honors_sentinels(self) -> bool {
        matches!(self, Self::Sas | Self::Spss)
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
                "\"{input}\" is a SAS format catalog, not a dataset. Pass the .sas7bdat file \
                 instead."
            );
        }
        return fail_incorrectusage_clierror!(
            "Don't know how to read {ext}. qsv readstat handles .sas7bdat, .xpt/.xpt5/.xpt8, \
             .dta, .sav/.zsav & .por files."
        );
    };

    if args.flag_value_labels && !format.honors_value_labels() {
        return fail_incorrectusage_clierror!(
            "--value-labels is not supported for SAS files. SAS keeps value labels in a separate \
             .sas7bcat catalog, which this command does not read yet."
        );
    }

    let sentinels_as = SentinelsAs::parse(&args.flag_sentinels_as)?;
    if sentinels_as != SentinelsAs::None && metadata_mode != MetadataMode::None {
        return fail_incorrectusage_clierror!(
            "--sentinels-as applies to the data, not to --metadata. The metadata already lists \
             each variable's missing-value codes."
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
        let rows = write_data(&args, path, format, sentinels, delim, &mut w)?;
        w.flush()?;
        winfo!("{rows} row{} exported.", if rows == 1 { "" } else { "s" });
    } else {
        write_metadata(path, format, &metadata_mode, delim, &mut w)?;
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

    // The Stata reader drops the sentinels of float & double variables (its
    // extended-missing offsets use the wrong bit step) and the labels of .a-.z,
    // so it would exit 0 having silently lost most of them.
    if format == Format::Stata {
        return fail_incorrectusage_clierror!(
            "--sentinels-as is not supported for Stata files yet: the reader loses the sentinels \
             of float & double variables."
        );
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
                "--sentinels-as label is not supported for SAS files. SAS keeps value labels in a \
                 separate .sas7bcat catalog, which this command does not read yet. Use \
                 --sentinels-as value to keep the sentinel codes."
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

    if args.flag_jobs.unwrap_or(1) > 1 && format == Format::Sas {
        wwarn!(
            "--jobs has no effect with --sentinels-as on SAS files: tracking sentinels reads them \
             on a single thread."
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

/// Upstream returns Stata `.a`-`.z` in `float`/`double` variables as a NaN
/// *value* instead of a null (#4627), so they would print as `NaN` while the
/// same missing in an integer variable prints empty. Stata has no NaN of its
/// own - every bit pattern above the largest valid value is a missing - so
/// nulling NaN here loses nothing.
///
/// Under `--value-labels` a labeled float variable arrives already stringified,
/// its NaN rendered as the text `"NaN"`; `labeled.cols` names those columns.
/// Where a value label is itself spelled "NaN" that text is ambiguous, so
/// those columns null exactly the rows in `labeled.raw_nan_rows` instead.
/// `offset` is the row number of the batch's first row.
fn stata_nan_to_null(
    df: &mut DataFrame,
    labeled: &StataLabeledFloats,
    offset: u64,
) -> PolarsResult<()> {
    let names: Vec<PlSmallStr> = df
        .columns()
        .iter()
        .filter(|c| c.dtype().is_float() || labeled.cols.contains(c.name()))
        .map(|c| c.name().clone())
        .collect();
    for name in names {
        let col = df.column(&name)?.as_materialized_series();
        let fixed = match col.dtype() {
            DataType::Float64 => {
                let ca = col.f64()?;
                if !ca.iter().any(|v| v.is_some_and(f64::is_nan)) {
                    continue;
                }
                ca.iter()
                    .map(|v| v.filter(|x| !x.is_nan()))
                    .collect::<Float64Chunked>()
                    .with_name(name.clone())
                    .into_series()
            },
            DataType::Float32 => {
                let ca = col.f32()?;
                if !ca.iter().any(|v| v.is_some_and(f32::is_nan)) {
                    continue;
                }
                ca.iter()
                    .map(|v| v.filter(|x| !x.is_nan()))
                    .collect::<Float32Chunked>()
                    .with_name(name.clone())
                    .into_series()
            },
            DataType::String => {
                let ca = col.str()?;
                let fixed: StringChunked = if let Some(nan_rows) = labeled.raw_nan_rows.get(&name) {
                    let end = offset + ca.len() as u64;
                    let nan_rows = &nan_rows[nan_rows.partition_point(|&r| r < offset)
                        ..nan_rows.partition_point(|&r| r < end)];
                    if nan_rows.is_empty() {
                        continue;
                    }
                    ca.iter()
                        .zip(offset..)
                        .map(|(v, row)| v.filter(|_| nan_rows.binary_search(&row).is_err()))
                        .collect()
                } else {
                    if !ca.iter().any(|v| v == Some("NaN")) {
                        continue;
                    }
                    ca.iter().map(|v| v.filter(|s| *s != "NaN")).collect()
                };
                fixed.with_name(name.clone()).into_series()
            },
            _ => continue,
        };
        df.replace(&name, fixed.into())?;
    }
    Ok(())
}

/// Stata float variables that `--value-labels` turned into strings.
#[derive(Default)]
struct StataLabeledFloats {
    cols:         Vec<PlSmallStr>,
    /// Sorted row numbers holding a raw NaN, for the columns that have a value
    /// label spelled "NaN".
    raw_nan_rows: HashMap<PlSmallStr, Vec<u64>>,
}

impl StataLabeledFloats {
    fn new(
        path: &Path,
        opts: &ScanOptions,
        rs_format: ReadStatFormat,
        labeled_schema: &polars::prelude::Schema,
        batch_size: Option<usize>,
    ) -> CliResult<Self> {
        let raw_opts = ScanOptions {
            value_labels_as_strings: Some(false),
            ..opts.clone()
        };
        let cols: Vec<PlSmallStr> = readstat_schema(path, Some(raw_opts.clone()), Some(rs_format))?
            .iter()
            .filter(|(name, dt)| {
                dt.is_float() && labeled_schema.get(name.as_str()) == Some(&DataType::String)
            })
            .map(|(name, _)| name.clone())
            .collect();
        if cols.is_empty() {
            return Ok(Self::default());
        }

        let meta = readstat_metadata_json(path, Some(rs_format)).map_err(|e| {
            crate::CliError::Other(format!(
                "Could not read the metadata of \"{}\": {e}",
                path.display()
            ))
        })?;
        let meta: serde_json::Value = serde_json::from_str(&meta)?;
        let ambiguous: Vec<PlSmallStr> = meta["variables"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|v| {
                v["value_labels"]
                    .as_object()
                    .is_some_and(|labels| labels.values().any(|l| l == "NaN"))
            })
            .filter_map(|v| v["name"].as_str())
            .filter_map(|name| cols.iter().find(|c| c.as_str() == name).cloned())
            .collect();

        let mut raw_nan_rows: HashMap<PlSmallStr, Vec<u64>> =
            ambiguous.iter().map(|c| (c.clone(), Vec::new())).collect();
        if !ambiguous.is_empty() {
            let batches = readstat_batch_iter(
                path,
                Some(raw_opts),
                Some(rs_format),
                Some(ambiguous.iter().map(ToString::to_string).collect()),
                None,
                batch_size,
            )?;
            let mut offset = 0_u64;
            for batch in batches {
                let df = batch?;
                for (name, rows) in &mut raw_nan_rows {
                    let col = df
                        .column(name)?
                        .as_materialized_series()
                        .cast(&DataType::Float64)?;
                    rows.extend(
                        col.f64()?
                            .iter()
                            .zip(offset..)
                            .filter(|(v, _)| v.is_some_and(f64::is_nan))
                            .map(|(_, row)| row),
                    );
                }
                offset += df.height() as u64;
            }
        }
        Ok(Self { cols, raw_nan_rows })
    }
}

/// Stream the file to CSV, one batch at a time, so memory stays bounded.
fn write_data<W: Write>(
    args: &Args,
    path: &Path,
    format: Format,
    sentinels: Option<InformativeNullOpts>,
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
    let (schema, por_df) = if let Some(rs_format) = rs_format {
        (
            readstat_schema(path, Some(opts.clone()), Some(rs_format))?,
            None,
        )
    } else {
        let df = polars_readstat_rs::scan_por(path, opts.clone())?.collect()?;
        (df.schema().clone(), Some(df))
    };

    // Labeled float variables that `--value-labels` turned into strings: their
    // extended missings arrive as the text "NaN" (see `stata_nan_to_null`).
    let stata_labeled_floats = match rs_format {
        Some(rs_format) if format == Format::Stata && args.flag_value_labels => {
            StataLabeledFloats::new(
                path,
                &opts,
                rs_format,
                &schema,
                (args.flag_batch > 0).then_some(args.flag_batch),
            )?
        },
        _ => StataLabeledFloats::default(),
    };

    let mut wtr = CsvWriter::new(w)
        .include_header(true)
        .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
        .with_separator(delim)
        .with_float_precision(float_precision)
        .batched(&schema)?;

    let mut rows = 0_u64;
    if let Some(rs_format) = rs_format {
        let batches = readstat_batch_iter(
            path,
            Some(opts),
            Some(rs_format),
            None,
            None,
            (args.flag_batch > 0).then_some(args.flag_batch),
        )?;
        for batch in batches {
            let mut df = batch?;
            if format == Format::Stata {
                stata_nan_to_null(&mut df, &stata_labeled_floats, rows)?;
            }
            rows += df.height() as u64;
            // write_batch panics on unaligned chunks
            df.align_chunks();
            wtr.write_batch(&df)?;
        }
    } else if let Some(mut df) = por_df {
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
    delim: u8,
    w: &mut W,
) -> CliResult<()> {
    let json = if let Some(rs_format) = format.readstat_format() {
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
