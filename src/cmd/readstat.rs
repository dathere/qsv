pub(crate) static USAGE: &str = r#"
Convert SAS, Stata & SPSS files to CSV.

EXPERIMENTAL: the readers this command is built on are young. Spot-check the
output against your source files before relying on a conversion, and please
report anything that looks wrong.

Supported input formats:
    SAS     .sas7bdat, .xpt, .xpt5, .xpt8
    Stata   .dta
    SPSS    .sav, .zsav, .por

Coded values are written as their underlying codes, not their labels, so the
conversion is lossless. SPSS durations (DTIME) are written as their number of
seconds, as SPSS stores them. Use --value-labels to decode them instead. SAS keeps
its value labels in a separate .sas7bcat format catalog, which --value-labels
finds next to the data file, or --sas7bcat names.

User-defined missing values ("sentinels") - SAS's .A to .Z & ._, Stata's .a
to .z, SPSS's declared missing codes - become empty cells by default, like any
other missing value. Use --sentinels-as to keep them. Without it, a warning
says how many sentinels were dropped & where. Counting them reads files on a
single thread, and SPSS files whole (see --batch), so it is skipped when the
option --jobs is above 1, and for SPSS files holding over about 128 MB of
data. As SPSS files declare their missing values, the warning then names the
variables that do instead. SAS & Stata files don't, so they get no warning.

The variable metadata these formats carry - variable labels, value labels,
missing-value codes, measure & display settings - can be dumped instead of the
data with --metadata.

Only part of a file can be read: --select picks variables, which the readers
skip over without decoding, and --offset, --limit & --sample pick rows. Rows
before the --offset are still read through, so skipping far into a large file
takes time, but stops early once --limit is reached.

  Convert a SAS dataset to CSV:
    qsv readstat data.sas7bdat > data.csv

  Convert an SPSS file, decoding coded values to their labels:
    qsv readstat --value-labels survey.sav -o survey.csv

  Keep the sentinel codes of a SAS dataset, in a <name>_null column after each
  numeric variable:
    qsv readstat --sentinels-as value data.sas7bdat

  Keep them in the variables' own columns instead:
    qsv readstat --sentinels-as value --sentinels-embedded data.sas7bdat

  Keep the sentinels of two SPSS variables as their labels, leaving the other
  values as codes:
    qsv readstat --sentinels-as label --sentinels-columns q1,q2 survey.sav

  Read three variables & the first 1000 rows of a wide SPSS file:
    qsv readstat --select id,age,income --limit 1000 survey.sav

  Read every variable from "q1" to "q20", in file order:
    qsv readstat --select q1-q20 survey.sav

  Read every variable except two (a leading ! excludes those listed):
    qsv readstat --select '!notes,comments' survey.sav

  Take a reproducible random sample of 500 rows, in file order:
    qsv readstat --sample 500 --seed 42 data.sas7bdat

  Dump the variable dictionary of a Stata file:
    qsv readstat --metadata pretty-json panel.dta

  Dump the metadata of just two variables:
    qsv readstat --metadata json --select age,income panel.dta

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
                           value labels of a .sas7bdat file, for use by
                           the options --value-labels, --sentinels-as label
                           & --metadata. It only names the catalog, so the
                           data needs one of the first two.
    --compress-numeric     Write float variables that only ever hold whole
                           numbers as integers, without the ".0" (e.g. 3.0
                           becomes 3). SPSS stores every number as a float, so
                           this matters most for SPSS files. It is decided per
                           variable, over the rows written: one value like 2.5
                           keeps the ".0" on every row of that variable. The
                           file is read twice - once to check the values.
                           With sentinel labels embedded, a variable with a
                           label that reads as a number (e.g. "1.0") keeps its
                           ".0", so the label is not rewritten.
    --sentinels-as <what>  Keep sentinels instead of writing them as empty
                           cells. Each eligible variable gets a <name>_null
                           column right after it, holding the sentinel of each
                           row that has one & empty otherwise.
                           Valid values: none, value, label.
                             none  - write them as empty cells, without the
                                     check & its warning.
                             value - the sentinel's code (e.g. .A or 99).
                             label - the sentinel's value label if it has one,
                                     else its code.
                           label labels only the sentinels: other values stay
                           codes unless --value-labels is also given.
                           SAS takes its sentinel labels from the format
                           catalog, found as for --value-labels. For SPSS,
                           value cannot be combined with --value-labels.
                           Not supported for .xpt & .por files. Tracking
                           sentinels reads files on a single thread, so the
                           option --jobs has no effect, and reads SPSS .sav
                           & .zsav files whole (see --batch).
    --sentinels-embedded   Write each sentinel into its variable's own column
                           instead of a <name>_null column. Those columns then
                           mix numbers & sentinels. Requires --sentinels-as.
    --sentinels-columns <list>  Comma-separated variables to keep sentinels
                           for. Requires --sentinels-as. By default, every
                           eligible variable: the numeric ones for SAS &
                           Stata, those with declared missing values for SPSS.
    --select <cols>        The variables to read, in the order given, using
                           qsv's select syntax: names, 1-based indices,
                           ranges (q1-q20) & /regex/, or a leading ! to
                           read every variable except those listed. See
                           'qsv select --help' for the full syntax. Variables
                           left out are skipped by the reader. Also applies
                           to the option --metadata, which then lists only
                           these variables.
    --offset <n>           Skip the first <n> rows. [default: 0]
    --limit <n>            Write at most <n> rows, counted after the
                           rows skipped by --offset.
    --sample <n>           Write a random sample of <n> rows, in file order,
                           drawn from the rows --offset & --limit select.
                           If there are no more than <n> such rows, all of
                           them are written.
    --seed <number>        Seed the random number generator of --sample,
                           so the same sample is drawn every time.
    -j, --jobs <arg>       Number of reader threads. [default: 1]
                           Raising it speeds up large uncompressed files at the
                           cost of memory, as out-of-order chunks have to be
                           buffered to keep the rows in source order. Row order
                           is preserved either way.
    -b, --batch <size>     Number of rows to read into memory at a time.
                           Does not apply to SPSS portable (.por) files - they
                           have no chunked reader upstream, so they are read
                           whole & memory scales with the file. Nor does it
                           apply to .sav & .zsav files while their sentinels
                           are tracked, which also reads them whole: with
                           the option --sentinels-as value or label, or by
                           the sentinel check when a variable declares
                           missing values & the file holds at most about
                           128 MB of data (see --sentinels-as none).
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
    CsvWriter, DataFrame, DataType, IdxCa, IdxSize, IntoSeries, PlSmallStr, PolarsResult, Schema,
    SchemaRef, SerWriter, StringChunked, TimeUnit,
};
use polars_readstat_rs::{
    CatalogKey, InformativeNullColumns, InformativeNullMode, InformativeNullOpts, ReadStatFormat,
    ScanOptions, readstat_batch_iter, readstat_metadata_json, readstat_schema,
};
use rand::{SeedableRng, rngs::StdRng};
use serde::Deserialize;

use crate::{CliResult, cmd::joinp::tsvssv_delim, config::Delimiter, select::SelectColumns, util};

#[derive(Deserialize)]
struct Args {
    arg_input:               Option<String>,
    flag_metadata:           String,
    flag_value_labels:       bool,
    flag_compress_numeric:   bool,
    flag_sas7bcat:           Option<String>,
    flag_sentinels_as:       Option<String>,
    flag_sentinels_embedded: bool,
    flag_sentinels_columns:  Option<String>,
    flag_select:             Option<SelectColumns>,
    flag_offset:             usize,
    flag_limit:              Option<usize>,
    flag_sample:             Option<usize>,
    flag_seed:               Option<u64>,
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
    let sentinels_as = SentinelsAs::parse(args.flag_sentinels_as.as_deref().unwrap_or("none"))?;
    if sentinels_as != SentinelsAs::None && metadata_mode != MetadataMode::None {
        return fail_incorrectusage_clierror!(
            "--sentinels-as applies to the data, not to --metadata. The metadata already lists \
             each variable's missing-value codes."
        );
    }
    let label_sentinels = sentinels_as == SentinelsAs::Label;
    if args.flag_sas7bcat.is_some()
        && metadata_mode == MetadataMode::None
        && !args.flag_value_labels
        && !label_sentinels
    {
        return fail_incorrectusage_clierror!(
            "--sas7bcat only names the SAS format catalog. To use it, add --value-labels to label \
             the values, or --sentinels-as label to label the sentinels."
        );
    }
    // before --output is created, so a rejected request leaves it untouched
    let sas_labels = SasLabels::resolve(&args, input, path, format, label_sentinels)?;
    if args.flag_compress_numeric && metadata_mode != MetadataMode::None {
        return fail_incorrectusage_clierror!(
            "--compress-numeric applies to the data, not to --metadata."
        );
    }
    if metadata_mode != MetadataMode::None
        && (args.flag_offset > 0 || args.flag_limit.is_some() || args.flag_sample.is_some())
    {
        return fail_incorrectusage_clierror!(
            "--offset, --limit & --sample pick rows of the data, so they don't apply to \
             --metadata."
        );
    }
    if args.flag_sample == Some(0) {
        return fail_incorrectusage_clierror!("--sample must be at least 1.");
    }
    if args.flag_seed.is_some() && args.flag_sample.is_none() {
        return fail_incorrectusage_clierror!("--seed requires --sample.");
    }
    // before --output is created, so a rejected request leaves it untouched
    let selection = resolve_selection(&args, path, format)?;
    let sentinels = sentinel_opts(
        &args,
        input,
        path,
        format,
        sentinels_as,
        selection.as_deref(),
    )?;
    let check = if metadata_mode == MetadataMode::None {
        sentinel_check(&args, path, format, selection.as_deref())
    } else {
        SentinelCheck::Off
    };
    // the readers label only the sentinels themselves for Stata, and for SPSS
    // they label sentinels only along with every value
    let sentinel_labels = match format {
        _ if sentinels.is_none() || !label_sentinels => None,
        Format::Sas => sas_labels.as_ref().map(SentinelLabels::sas),
        Format::Spss if !args.flag_value_labels => Some(SentinelLabels::spss(path)?),
        _ => None,
    };

    let mut delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = std::env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };

    // The shared --output guard only sees command-line arguments, so it misses
    // a catalog that --value-labels or --sentinels-as label found on its own.
    if let (Some(out), Some(labels)) = (&args.flag_output, &sas_labels)
        && same_file::is_same_file(out, &labels.catalog).unwrap_or(false)
    {
        return fail_incorrectusage_clierror!(
            "--output ({out}) is the SAS format catalog being read ({}). Pass a different \
             --output.",
            labels.catalog.display()
        );
    }
    let w = match args.flag_output.as_ref() {
        Some(p) => {
            delim = tsvssv_delim(p, delim);
            Box::new(File::create(p)?) as Box<dyn Write>
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    let mut w = io::BufWriter::with_capacity(crate::config::DEFAULT_WTR_BUFFER_CAPACITY, w);

    if metadata_mode == MetadataMode::None {
        let (mut dropped, watch) = DroppedSentinels::new(format, args.flag_value_labels, check);
        let seed = args.flag_seed.unwrap_or_else(rand::random);
        if args.flag_sample.is_some() && args.flag_seed.is_none() {
            winfo!("--sample drew its rows with --seed {seed}; pass it to draw them again.");
        }
        let rows = write_data(
            &args,
            path,
            format,
            selection.as_deref(),
            seed,
            sentinels,
            watch,
            &mut dropped,
            sas_labels.as_ref().filter(|_| args.flag_value_labels),
            sentinel_labels.as_ref(),
            delim,
            &mut w,
        )?;
        w.flush()?;
        winfo!("{rows} row{} exported.", if rows == 1 { "" } else { "s" });
        dropped.warn();
    } else {
        write_metadata(
            path,
            format,
            selection.as_deref(),
            &metadata_mode,
            sas_labels.as_ref(),
            delim,
            &mut w,
        )?;
        w.flush()?;
    }

    Ok(())
}

/// The file's variable names, in file order, as the readers name them.
fn file_variables(path: &Path, format: Format) -> CliResult<Vec<String>> {
    let schema = match format.readstat_format() {
        Some(rs_format) => readstat_schema(path, None, Some(rs_format))?,
        None => polars_readstat_rs::scan_por(path, ScanOptions::default())?.collect_schema()?,
    };
    Ok(schema.iter_names().map(ToString::to_string).collect())
}

/// The variables `--select` picks, in the order it names them.
fn resolve_selection(args: &Args, path: &Path, format: Format) -> CliResult<Option<Vec<String>>> {
    let Some(select) = &args.flag_select else {
        return Ok(None);
    };
    let variables = file_variables(path, format)?;
    let header: csv::ByteRecord = variables.iter().map(String::as_bytes).collect();
    let picked = match select.selection(&header, true) {
        Ok(picked) => picked,
        Err(e) => {
            let e = e.replace(
                "does not exist as a named header in the given CSV data",
                &format!("is not a variable of \"{}\"", path.display()),
            );
            return fail_incorrectusage_clierror!("--select: {e}");
        },
    };
    if picked.is_empty() {
        return fail_incorrectusage_clierror!("--select picks no variables.");
    }
    let mut names: Vec<String> = Vec::with_capacity(picked.len());
    for &i in picked.iter() {
        let name = &variables[i];
        if names.contains(name) {
            return fail_incorrectusage_clierror!(
                "--select picks the variable \"{name}\" more than once."
            );
        }
        names.push(name.clone());
    }
    Ok(Some(names))
}

fn row_count(path: &Path, format: Format) -> CliResult<Option<u64>> {
    let Some(rs_format) = format.readstat_format() else {
        return Ok(None);
    };
    if format == Format::SasXpt {
        // XPT's metadata JSON doesn't carry its row count
        return Ok(Some(
            polars_readstat_rs::read_xpt_metadata(path)?.row_count as u64,
        ));
    }
    let meta = readstat_metadata_json(path, Some(rs_format)).map_err(|e| {
        crate::CliError::Other(format!(
            "Could not read the metadata of \"{}\": {e}",
            path.display()
        ))
    })?;
    let meta: serde_json::Value = serde_json::from_str(&meta)?;
    Ok(meta["row_count"].as_u64())
}

/// The rows `--offset`, `--limit` & `--sample` keep, by their 0-based
/// position in the file. The sample is drawn once, so every pass over the
/// file (the `--compress-numeric` check, then the write) keeps the same rows.
#[derive(Clone, Default)]
struct RowWindow {
    offset: u64,
    end:    Option<u64>,
    /// Sorted positions, all within `offset..end`.
    sample: Option<Vec<u64>>,
}

impl RowWindow {
    fn new(args: &Args) -> Self {
        let offset = args.flag_offset as u64;
        Self {
            offset,
            end: args
                .flag_limit
                .map(|limit| offset.saturating_add(limit as u64)),
            sample: None,
        }
    }

    /// Draw `--sample` from the window, now that the file's row count is
    /// known. With no more rows than the sample size, every row is kept.
    fn draw(&mut self, args: &Args, seed: u64, rows: u64) {
        let Some(n) = args.flag_sample else {
            return;
        };
        let end = self.end.map_or(rows, |end| end.min(rows));
        let len = end.saturating_sub(self.offset);
        if (n as u64) >= len {
            return;
        }
        // sampling rows, not a security function, and seeded so --seed reproduces it
        let mut rng = StdRng::seed_from_u64(seed); // DevSkim: ignore DS148264
        let mut positions: Vec<u64> = rand::seq::index::sample(&mut rng, len as usize, n)
            .into_iter()
            .map(|i| self.offset + i as u64)
            .collect();
        positions.sort_unstable();
        self.sample = Some(positions);
    }

    fn is_all(&self) -> bool {
        self.offset == 0 && self.end.is_none() && self.sample.is_none()
    }

    /// How many rows the reader has to read: up to the last row kept.
    fn n_rows(&self) -> Option<usize> {
        let last = match &self.sample {
            Some(sample) => Some(sample.last().map_or(0, |&p| p + 1)),
            None => self.end,
        };
        last.map(|n| usize::try_from(n).unwrap_or(usize::MAX))
    }

    /// Keep the rows of a batch whose first row is at position `start`.
    fn apply(&self, df: &DataFrame, start: u64) -> PolarsResult<DataFrame> {
        let height = df.height() as u64;
        let lo = self.offset.max(start);
        let hi = self
            .end
            .map_or(start + height, |end| end.min(start + height));
        if lo >= hi {
            return Ok(df.clear());
        }
        match &self.sample {
            None => Ok(df.slice((lo - start) as i64, (hi - lo) as usize)),
            Some(sample) => {
                let first = sample.partition_point(|&p| p < lo);
                let last = sample.partition_point(|&p| p < hi);
                let idx: Vec<IdxSize> = sample[first..last]
                    .iter()
                    .map(|&p| (p - start) as IdxSize)
                    .collect();
                df.take(&IdxCa::from_vec(PlSmallStr::EMPTY, idx))
            },
        }
    }
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
    selection: Option<&[String]>,
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
    // The SPSS reader labels sentinels whenever it decodes value labels.
    if format == Format::Spss && sentinels_as == SentinelsAs::Value && args.flag_value_labels {
        return fail_incorrectusage_clierror!(
            "--sentinels-as value cannot be combined with --value-labels on SPSS files, as that \
             writes the sentinels' labels. Use --sentinels-as label instead."
        );
    }

    let columns = match args.flag_sentinels_columns.as_deref() {
        // the reader leaves out the selected variables that can't hold one,
        // and unselected ones can't clash with a sentinel column
        None => selection.map_or(InformativeNullColumns::All, |sel| {
            InformativeNullColumns::Selected(sel.to_vec())
        }),
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
            if let Some(selection) = selection {
                let outside: Vec<String> = names
                    .iter()
                    .filter(|n| !selection.contains(n))
                    .map(|n| format!("\"{n}\""))
                    .collect();
                if !outside.is_empty() {
                    return fail_incorrectusage_clierror!(
                        "--sentinels-columns names {}, which --select leaves out.",
                        outside.join(", ")
                    );
                }
            }
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

    if args.flag_jobs.unwrap_or(1) > 1 {
        wwarn!(
            "--jobs has no effect with --sentinels-as: tracking sentinels reads the file on a \
             single thread."
        );
    }

    Ok(Some(null_opts))
}

/// What the hidden sentinel check does when --sentinels-as is not given.
enum SentinelCheck {
    Off,
    /// Track the sentinels, in indicator columns named with [`PROBE_SUFFIX`],
    /// to count those written as empty cells.
    Count(InformativeNullOpts),
    /// Only name the SPSS variables that declare missing values, because
    /// counting would cost the reader its parallelism or read too large a
    /// file whole.
    Declared {
        variables: Vec<String>,
        reason:    DeclaredOnly,
    },
}

#[derive(Clone, Copy)]
enum DeclaredOnly {
    Jobs,
    TooLarge,
}

/// Above this much data (uncompressed), the SPSS sentinel check names the
/// variables that declare missing values instead of counting the sentinels,
/// as counting reads the file whole.
const SPSS_COUNT_MAX_BYTES: u64 = 128 * 1024 * 1024;

/// The hidden sentinel check. Counting is not done where it would cost the
/// reader its parallelism (`--jobs` above 1), nor for SPSS files too large to
/// read whole: SPSS files declare their missing values, so those are named
/// instead, while SAS & Stata files don't, so they get no check. Nor is it
/// done for files with no variable that can hold a sentinel, or for the XPT &
/// POR readers, which don't report sentinels.
fn sentinel_check(
    args: &Args,
    path: &Path,
    format: Format,
    selection: Option<&[String]>,
) -> SentinelCheck {
    if args.flag_sentinels_as.is_some() {
        return SentinelCheck::Off;
    }
    let Some(rs_format) = format
        .readstat_format()
        .filter(|_| format.honors_sentinels())
    else {
        return SentinelCheck::Off;
    };
    let jobs = args.flag_jobs.unwrap_or(1) > 1;
    if jobs && format != Format::Spss {
        return SentinelCheck::Off;
    }
    let watch = InformativeNullOpts {
        columns:          InformativeNullColumns::All,
        mode:             InformativeNullMode::SeparateColumn {
            suffix: PROBE_SUFFIX.to_string(),
        },
        use_value_labels: false,
    };
    let probe = ScanOptions {
        value_labels_as_strings: Some(args.flag_value_labels),
        informative_nulls: Some(watch.clone()),
        ..ScanOptions::default()
    };
    // a best-effort check: if the probe fails, the conversion goes ahead without it
    let Ok(schema) = readstat_schema(path, Some(probe), Some(rs_format)) else {
        return SentinelCheck::Off;
    };
    let variables: Vec<String> = schema
        .iter_names()
        .filter_map(|name| name.strip_suffix(PROBE_SUFFIX))
        .filter(|name| selection.is_none_or(|sel| sel.iter().any(|s| s == name)))
        .map(str::to_string)
        .collect();
    if variables.is_empty() {
        return SentinelCheck::Off;
    }
    let reason = if format != Format::Spss {
        None
    } else if jobs {
        Some(DeclaredOnly::Jobs)
    } else if spss_data_bytes(path).is_none_or(|bytes| bytes > spss_count_max_bytes()) {
        Some(DeclaredOnly::TooLarge)
    } else {
        None
    };
    match reason {
        Some(reason) => SentinelCheck::Declared { variables, reason },
        None => SentinelCheck::Count(watch),
    }
}

fn spss_count_max_bytes() -> u64 {
    std::env::var("QSV_TEST_READSTAT_SPSS_COUNT_MAX_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(SPSS_COUNT_MAX_BYTES)
}

/// The uncompressed size of an SPSS file's data, from its metadata: rows times
/// the stored width of a row. None if the file doesn't record its row count.
fn spss_data_bytes(path: &Path) -> Option<u64> {
    let meta = readstat_metadata_json(path, Some(ReadStatFormat::Spss)).ok()?;
    let meta: serde_json::Value = serde_json::from_str(&meta).ok()?;
    let rows = meta["row_count"].as_u64()?;
    let width: u64 = meta["variables"]
        .as_array()?
        .iter()
        .filter_map(|var| var["storage_width_bytes"].as_u64())
        .sum();
    Some(rows.saturating_mul(width))
}

/// The sentinels the hidden check found, per variable: how many, and one of
/// their codes (or labels, under --value-labels) as an example.
struct DroppedSentinels {
    variables: Vec<(String, u64, String)>,
    /// The SPSS variables declaring missing values, when they were not counted.
    declared:  Option<(Vec<String>, DeclaredOnly)>,
    /// SAS reports an unrecognized missing as a bare ".", which is system
    /// missing, not a sentinel. In SPSS "." can be a declared missing string.
    skip_dot:  bool,
    /// The --sentinels-as value that keeps them: SPSS refuses `value` with
    /// --value-labels.
    keep:      &'static str,
}

impl DroppedSentinels {
    /// Also returns the reader options the check counts the sentinels with,
    /// if it does.
    fn new(
        format: Format,
        value_labels: bool,
        check: SentinelCheck,
    ) -> (Self, Option<InformativeNullOpts>) {
        let (watch, declared) = match check {
            SentinelCheck::Off => (None, None),
            SentinelCheck::Count(watch) => (Some(watch), None),
            SentinelCheck::Declared { variables, reason } => (None, Some((variables, reason))),
        };
        let dropped = Self {
            variables: Vec::new(),
            declared,
            skip_dot: format == Format::Sas,
            keep: if format == Format::Spss && value_labels {
                "label"
            } else {
                "value"
            },
        };
        (dropped, watch)
    }

    /// Tally & remove the check's indicator columns from a batch.
    fn take(&mut self, df: &mut DataFrame) -> PolarsResult<()> {
        let probes: Vec<PlSmallStr> = df
            .get_column_names()
            .into_iter()
            .filter(|name| name.ends_with(PROBE_SUFFIX))
            .cloned()
            .collect();
        for probe in probes {
            let col = df.drop_in_place(&probe)?;
            let name = probe.trim_end_matches(PROBE_SUFFIX);
            let col = col.as_materialized_series();
            let mut codes = col
                .str()?
                .iter()
                .flatten()
                .filter(|code| !(self.skip_dot && *code == "."));
            let Some(first) = codes.next() else {
                continue;
            };
            let found = 1 + codes.count() as u64;
            if let Some(var) = self.variables.iter_mut().find(|(n, ..)| n == name) {
                var.1 += found;
            } else {
                self.variables
                    .push((name.to_string(), found, first.to_string()));
            }
        }
        Ok(())
    }

    fn warn(&self) {
        const SHOWN: usize = 5;
        let list = |names: &mut dyn Iterator<Item = &str>, len: usize| {
            let mut list = names.take(SHOWN).collect::<Vec<_>>().join(", ");
            if len > SHOWN {
                list.push_str(", ...");
            }
            list
        };
        let plural = |n: usize, word: &str| format!("{n} {word}{}", if n == 1 { "" } else { "s" });
        let advice = format!(
            "Use --sentinels-as {} to keep them, or --sentinels-as none to skip this check.",
            self.keep
        );

        if let Some((declared, reason)) = &self.declared {
            let why = match reason {
                DeclaredOnly::Jobs => {
                    "Counting them reads the file on a single thread, so it is skipped when --jobs \
                     is above 1."
                },
                DeclaredOnly::TooLarge => {
                    "Counting them reads the file whole, so it is skipped for files holding over \
                     about 128 MB of data."
                },
            };
            let verb = if declared.len() == 1 {
                "declares"
            } else {
                "declare"
            };
            wwarn!(
                "{} ({}) {verb} user-defined missing values (sentinels); any in the data were \
                 written as empty cells. {why} {advice}",
                plural(declared.len(), "variable"),
                list(&mut declared.iter().map(String::as_str), declared.len()),
            );
            return;
        }

        let Some((var, _, example)) = self.variables.first() else {
            return;
        };
        let total: u64 = self.variables.iter().map(|(_, n, _)| n).sum();
        wwarn!(
            "{total} sentinel{} (user-defined missing values) in {} ({}) were written as empty \
             cells, e.g. {example} in {var}. {advice}",
            if total == 1 { "" } else { "s" },
            plural(self.variables.len(), "variable"),
            list(
                &mut self.variables.iter().map(|(n, ..)| n.as_str()),
                self.variables.len()
            ),
        );
    }
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

fn whole_number_columns(
    path: &Path,
    rs_format: ReadStatFormat,
    opts: &ScanOptions,
    batch_size: Option<usize>,
    selection: Option<&[String]>,
    window: &RowWindow,
) -> CliResult<Vec<PlSmallStr>> {
    // the window picks rows by position, so they must arrive in file order
    let scan_opts = ScanOptions {
        informative_nulls: None,
        preserve_order: Some(!window.is_all()),
        ..opts.clone()
    };
    let mut schema = readstat_schema(path, Some(scan_opts.clone()), Some(rs_format))?;
    durations_as_seconds_schema(&mut schema);
    let mut scan = WholeNumberScan::new(&schema);
    if let Some(selection) = selection {
        scan.candidates
            .retain(|(name, _)| selection.iter().any(|s| s == name.as_str()));
    }
    if scan.candidates.is_empty() {
        return Ok(Vec::new());
    }
    // only the float columns are read
    let batches = readstat_batch_iter(
        path,
        Some(scan_opts),
        Some(rs_format),
        Some(scan.names()),
        window.n_rows(),
        batch_size,
    )?;
    let mut start = 0_u64;
    for batch in batches {
        let mut batch = batch?;
        durations_as_seconds(&mut batch)?;
        let height = batch.height() as u64;
        if window.is_all() {
            scan.update(&batch)?;
        } else {
            scan.update(&window.apply(&batch, start)?)?;
        }
        start += height;
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
    /// Keyed by the sentinel's code as the data reader writes it (`.A`, `._`).
    sentinels:      HashMap<String, String>,
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
    /// latter, a cell that isn't a number is a sentinel and is left alone,
    /// for [`SentinelLabels`] to label.
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
    catalog: PathBuf,
    columns: Vec<(PlSmallStr, FormatLabels)>,
}

impl SasLabels {
    /// The catalog is used when `--sas7bcat` names one, or when
    /// `--value-labels` or `--sentinels-as label` is given, in which case it
    /// is looked for next to the data file as `<name>.sas7bcat`, then
    /// `formats.sas7bcat`.
    fn resolve(
        args: &Args,
        input: &str,
        path: &Path,
        format: Format,
        label_sentinels: bool,
    ) -> CliResult<Option<Self>> {
        if format != Format::Sas
            || !(args.flag_value_labels || args.flag_sas7bcat.is_some() || label_sentinels)
        {
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
                        "{} on a SAS file needs its .sas7bcat format catalog, and there is none \
                         at \"{}\" or \"{}\". Pass it with --sas7bcat.",
                        if args.flag_value_labels {
                            "--value-labels"
                        } else {
                            "--sentinels-as label"
                        },
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
                    CatalogKey::Missing(Some(tag)) => {
                        labels.sentinels.insert(format!(".{tag}"), label);
                    },
                    // system missing `.` is an empty cell, never labeled
                    CatalogKey::Missing(None) => {},
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
                "None of the formats of \"{input}\" are in the SAS format catalog \"{}\", so \
                 nothing was labeled.",
                catalog.display()
            );
        } else {
            winfo!("Using SAS format catalog \"{}\".", catalog.display());
        }
        Ok(Some(Self { catalog, columns }))
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
                // keyed MISSING_A for .A, as the Stata metadata keys .a
                let mut sentinels: Vec<_> = labels.sentinels.iter().collect();
                sentinels.sort();
                for (code, label) in sentinels {
                    map.insert(format!("MISSING_{}", &code[1..]), label.clone().into());
                }
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

/// Whether `--sentinels-columns` (or its default, every eligible variable)
/// keeps the sentinels of `name`.
fn tracked(columns: &InformativeNullColumns, name: &str) -> bool {
    match columns {
        InformativeNullColumns::All => true,
        InformativeNullColumns::Selected(names) => names.iter().any(|n| n == name),
    }
}

/// One variable's sentinel labels, keyed by the sentinel's code as the reader
/// writes it: `.A` for SAS, `8` or `NA` for SPSS.
#[derive(Default)]
struct CodeLabels {
    text:    HashMap<String, String>,
    numeric: Vec<(f64, String)>,
}

impl CodeLabels {
    fn get(&self, code: &str) -> Option<&String> {
        self.text.get(code.trim_end()).or_else(|| {
            let v = code.parse::<f64>().ok()?;
            self.numeric
                .iter()
                .find(|(c, _)| *c == v)
                .map(|(_, label)| label)
        })
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty() && self.numeric.is_empty()
    }
}

/// The sentinel labels qsv applies itself under `--sentinels-as label`, where
/// the reader can't label only the sentinels: SAS (from the format catalog)
/// and SPSS without `--value-labels` (from the variables' value labels).
/// Other values are left alone, so they stay codes unless `--value-labels`.
struct SentinelLabels {
    columns: Vec<(PlSmallStr, CodeLabels)>,
}

impl SentinelLabels {
    fn sas(catalog: &SasLabels) -> Self {
        let columns = catalog
            .columns
            .iter()
            .filter(|(_, labels)| labels.numeric_format && !labels.sentinels.is_empty())
            .map(|(name, labels)| {
                let codes = CodeLabels {
                    text:    labels.sentinels.clone(),
                    numeric: Vec::new(),
                };
                (name.clone(), codes)
            })
            .collect();
        Self { columns }
    }

    /// An SPSS variable's sentinels are its declared missing values: discrete
    /// codes, a range (the first two of `missing_doubles` when
    /// `missing_range`) plus at most one code, or strings.
    fn spss(path: &Path) -> CliResult<Self> {
        let meta = readstat_metadata_json(path, Some(ReadStatFormat::Spss)).map_err(|e| {
            crate::CliError::Other(format!(
                "Could not read the metadata of \"{}\": {e}",
                path.display()
            ))
        })?;
        let meta: serde_json::Value = serde_json::from_str(&meta)?;
        let mut columns = Vec::new();
        for var in meta["variables"].as_array().into_iter().flatten() {
            let (Some(name), Some(labels)) =
                (var["name"].as_str(), var["value_labels"].as_object())
            else {
                continue;
            };
            let mut codes = CodeLabels::default();
            if var["type"] == "Str" {
                let missing: Vec<&str> = var["missing_strings"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::trim_end)
                    .collect();
                for (code, label) in labels {
                    if let Some(label) = label.as_str()
                        && missing.contains(&code.trim_end())
                    {
                        codes
                            .text
                            .insert(code.trim_end().to_string(), label.to_string());
                    }
                }
            } else {
                let missing: Vec<f64> = var["missing_doubles"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(serde_json::Value::as_f64)
                    .collect();
                let range = var["missing_range"].as_bool() == Some(true) && missing.len() >= 2;
                let is_missing = |v: f64| {
                    if range {
                        (missing[0]..=missing[1]).contains(&v) || missing.get(2) == Some(&v)
                    } else {
                        missing.contains(&v)
                    }
                };
                for (code, label) in labels {
                    if let (Ok(v), Some(label)) = (code.parse::<f64>(), label.as_str())
                        && is_missing(v)
                    {
                        codes.numeric.push((v, label.to_string()));
                    }
                }
            }
            if !codes.is_empty() {
                columns.push((PlSmallStr::from(name), codes));
            }
        }
        Ok(Self { columns })
    }

    /// The tracked variables with a sentinel label that the
    /// `--compress-numeric` ".0" rewrite would change (e.g. "1.0"), were it
    /// embedded among numbers.
    fn numeric_looking<'a>(
        &'a self,
        columns: &'a InformativeNullColumns,
    ) -> impl Iterator<Item = &'a PlSmallStr> {
        self.columns
            .iter()
            .filter(|(name, _)| tracked(columns, name))
            .filter(|(_, codes)| {
                codes
                    .text
                    .values()
                    .chain(codes.numeric.iter().map(|(_, l)| l))
                    .any(|l| whole_number_text(l).is_some_and(|t| &t != l))
            })
            .map(|(name, _)| name)
    }

    /// Label the sentinels of the variables `columns` tracks: in their own
    /// column if `embedded`, else in their `<name>_null` column. A variable of
    /// the file that happens to be named `<name>_null` is never touched.
    fn apply(
        &self,
        df: &mut DataFrame,
        columns: &InformativeNullColumns,
        embedded: bool,
    ) -> PolarsResult<()> {
        for (name, codes) in &self.columns {
            if !tracked(columns, name) {
                continue;
            }
            let target = if embedded {
                name.clone()
            } else {
                PlSmallStr::from(format!("{name}_null"))
            };
            let Ok(col) = df.column(&target) else {
                continue;
            };
            let col = col.as_materialized_series();
            if col.dtype() != &DataType::String {
                continue;
            }
            let labeled: StringChunked = col
                .str()?
                .iter()
                .map(|v| v.map(|s| codes.get(s).map_or_else(|| s.to_string(), Clone::clone)))
                .collect();
            df.replace(
                &target,
                labeled.with_name(target.clone()).into_series().into(),
            )?;
        }
        Ok(())
    }
}

/// SPSS DTIME variables come back from the readers as polars Durations, which
/// the CSV writer cannot write. They are written as seconds instead - the unit
/// SPSS stores them in - so they stay numeric & lossless.
fn units_per_second(dtype: &DataType) -> Option<f64> {
    match dtype {
        DataType::Duration(TimeUnit::Nanoseconds) => Some(1e9),
        DataType::Duration(TimeUnit::Microseconds) => Some(1e6),
        DataType::Duration(TimeUnit::Milliseconds) => Some(1e3),
        _ => None,
    }
}

fn durations_as_seconds_schema(schema: &mut SchemaRef) {
    let durations: Vec<PlSmallStr> = schema
        .iter()
        .filter(|(_, dtype)| units_per_second(dtype).is_some())
        .map(|(name, _)| name.clone())
        .collect();
    for name in durations {
        std::sync::Arc::make_mut(schema).set_dtype(&name, DataType::Float64);
    }
}

fn durations_as_seconds(df: &mut DataFrame) -> PolarsResult<()> {
    let durations: Vec<(PlSmallStr, f64)> = df
        .columns()
        .iter()
        .filter_map(|col| units_per_second(col.dtype()).map(|per| (col.name().clone(), per)))
        .collect();
    for (name, per_second) in durations {
        let col = df.column(&name)?.as_materialized_series();
        let seconds = col.cast(&DataType::Int64)?.cast(&DataType::Float64)? / per_second;
        df.replace(&name, seconds.with_name(name.clone()).into())?;
    }
    Ok(())
}

/// Stream the file to CSV, one batch at a time, so memory stays bounded.
fn write_data<W: Write>(
    args: &Args,
    path: &Path,
    format: Format,
    selection: Option<&[String]>,
    seed: u64,
    sentinels: Option<InformativeNullOpts>,
    watch: Option<InformativeNullOpts>,
    dropped: &mut DroppedSentinels,
    sas_labels: Option<&SasLabels>,
    sentinel_labels: Option<&SentinelLabels>,
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
    let embedded = sentinels
        .as_ref()
        .is_some_and(|s| matches!(s.mode, InformativeNullMode::MergedString));
    let embedded_labels = embedded && sentinels.as_ref().is_some_and(|s| s.use_value_labels);
    let sentinel_columns = sentinels.as_ref().map(|s| s.columns.clone());
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
    let mut window = RowWindow::new(args);
    let (mut schema, por_df) = if let Some(rs_format) = rs_format {
        if args.flag_sample.is_some() {
            let Some(rows) = row_count(path, format)? else {
                return fail_clierror!(
                    "Cannot --sample \"{}\": its metadata doesn't record how many rows it has.",
                    path.display()
                );
            };
            window.draw(args, seed, rows);
        }
        (
            readstat_schema(path, Some(opts.clone()), Some(rs_format))?,
            None,
        )
    } else {
        let mut df = polars_readstat_rs::scan_por(path, opts.clone())?.collect()?;
        durations_as_seconds(&mut df)?;
        if let Some(selection) = selection {
            df = df.select(selection.iter().map(String::as_str))?;
        }
        window.draw(args, seed, df.height() as u64);
        if !window.is_all() {
            df = window.apply(&df, 0)?;
        }
        (df.schema().clone(), Some(df))
    };
    durations_as_seconds_schema(&mut schema);

    // The columns written, in order: each selected variable, followed by its
    // sentinel column if it has one. A variable of the file that happens to
    // be named `<name>_null` is not a sentinel column.
    let order: Option<Vec<PlSmallStr>> = match (selection, rs_format) {
        (Some(selection), Some(_)) => {
            let separate = sentinel_columns.is_some() && !embedded;
            let variables = if separate {
                file_variables(path, format)?
            } else {
                Vec::new()
            };
            let mut order: Vec<PlSmallStr> = Vec::with_capacity(selection.len());
            for name in selection {
                order.push(PlSmallStr::from(name.as_str()));
                let indicator = format!("{name}_null");
                if separate && schema.contains(&indicator) && !variables.contains(&indicator) {
                    order.push(PlSmallStr::from(indicator));
                }
            }
            let mut projected = Schema::with_capacity(order.len());
            for name in &order {
                if let Some(dtype) = schema.get(name) {
                    projected.insert(name.clone(), dtype.clone());
                }
            }
            schema = std::sync::Arc::new(projected);
            Some(order)
        },
        _ => None,
    };

    let mut whole = if !args.flag_compress_numeric {
        Vec::new()
    } else if let Some(rs_format) = rs_format {
        whole_number_columns(path, rs_format, &opts, batch_size, selection, &window)?
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
    if embedded && let (Some(labels), Some(columns)) = (sentinel_labels, &sentinel_columns) {
        let risky: Vec<&PlSmallStr> = labels.numeric_looking(columns).collect();
        whole.retain(|name| !risky.contains(&name));
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
        // the hidden sentinel check reads the data with its own indicator
        // columns, which are tallied & dropped before anything else sees them
        let read_opts = if watch.is_some() {
            ScanOptions {
                informative_nulls: watch,
                ..opts
            }
        } else {
            opts
        };
        let batches = readstat_batch_iter(
            path,
            Some(read_opts),
            Some(rs_format),
            selection.map(<[String]>::to_vec),
            window.n_rows(),
            batch_size,
        )?;
        let mut start = 0_u64;
        for batch in batches {
            let mut df = batch?;
            durations_as_seconds(&mut df)?;
            let height = df.height() as u64;
            if !window.is_all() {
                // before the sentinel check, so it counts only the rows written
                df = window.apply(&df, start)?;
            }
            start += height;
            if df.height() == 0 {
                continue;
            }
            dropped.take(&mut df)?;
            if let Some(order) = &order {
                df = df.select(order.iter().cloned())?;
            }
            if let Some(labels) = sas_labels {
                labels.apply(&mut df)?;
            }
            if let (Some(labels), Some(columns)) = (sentinel_labels, &sentinel_columns) {
                labels.apply(&mut df, columns, embedded)?;
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
    selection: Option<&[String]>,
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
    if let Some(selection) = selection {
        json = select_metadata(&json, selection)?;
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

/// Keep only the `--select`ed variables in the per-variable metadata, in the
/// order selected. The list is the array of objects with a "name" - named
/// `columns` for SAS & `variables` for Stata & SPSS - and the file-level keys
/// are kept as they are.
fn select_metadata(json: &str, selection: &[String]) -> CliResult<String> {
    use serde_json::Value;

    let mut value: Value = serde_json::from_str(json)?;
    let Some(obj) = value.as_object_mut() else {
        return Ok(json.to_string());
    };
    for list in obj.values_mut() {
        let Some(vars) = list.as_array_mut() else {
            continue;
        };
        if vars.is_empty()
            || !vars
                .iter()
                .all(|v| v.get("name").is_some_and(Value::is_string))
        {
            continue;
        }
        let picked: Vec<Value> = selection
            .iter()
            .filter_map(|name| {
                vars.iter()
                    .find(|v| v["name"].as_str() == Some(name.as_str()))
                    .cloned()
            })
            .collect();
        *vars = picked;
    }
    Ok(serde_json::to_string(&value)?)
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
