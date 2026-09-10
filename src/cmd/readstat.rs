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

The variable metadata these formats carry - variable labels, value labels,
missing-value codes, measure & display settings - can be dumped instead of the
data with --metadata.

  Convert a SAS dataset to CSV:
    qsv readstat data.sas7bdat > data.csv

  Convert an SPSS file, decoding coded values to their labels:
    qsv readstat --value-labels survey.sav -o survey.csv

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
    -j, --jobs <arg>       Number of reader threads. [default: 1]
                           Raising it speeds up large uncompressed files at the
                           cost of memory, as out-of-order chunks have to be
                           buffered to keep the rows in source order. Row order
                           is preserved either way.
    -b, --batch <size>     Number of rows to read into memory at a time.
                           [default: 50000]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The delimiter to use when writing CSV data.
                           Must be a single character. [default: ,]
"#;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use polars::prelude::{CsvWriter, SerWriter};
use polars_readstat_rs::{
    ReadStatFormat, ScanOptions, readstat_batch_iter, readstat_metadata_json,
};
use serde::Deserialize;

use crate::{CliResult, cmd::joinp::tsvssv_delim, config::Delimiter, util};

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    flag_metadata:     String,
    flag_value_labels: bool,
    flag_jobs:         Option<usize>,
    flag_batch:        usize,
    flag_output:       Option<String>,
    flag_delimiter:    Option<Delimiter>,
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
        let rows = write_data(&args, path, format, delim, &mut w)?;
        w.flush()?;
        winfo!("{rows} row{} exported.", if rows == 1 { "" } else { "s" });
    } else {
        write_metadata(path, format, &metadata_mode, delim, &mut w)?;
        w.flush()?;
    }

    Ok(())
}

/// Stream the file to CSV, one batch at a time, so memory stays bounded.
fn write_data<W: Write>(
    args: &Args,
    path: &Path,
    format: Format,
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
        ..ScanOptions::default()
    };

    // `.por` has no chunked reader upstream, so it is read whole. Every other
    // format streams, and takes its schema from the file's own variable
    // dictionary rather than from the first batch - that way the header is
    // written exactly once even when the file has no rows at all.
    let rs_format = format.readstat_format();
    let (schema, por_df) = if let Some(rs_format) = rs_format {
        (
            polars_readstat_rs::readstat_schema(path, Some(opts.clone()), Some(rs_format))?,
            None,
        )
    } else {
        let df = polars_readstat_rs::scan_por(path, opts.clone())?.collect()?;
        (df.schema().clone(), Some(df))
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
