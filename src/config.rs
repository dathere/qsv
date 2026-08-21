use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use csv_nose::{SampleSize, Sniffer};
use log::{debug, info, warn};
use serde::de::{Deserialize, Deserializer, Error};

use crate::{
    CliResult,
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
};

// rdr default is 8k in csv crate, we're making it 128k
pub const DEFAULT_RDR_BUFFER_CAPACITY: usize = 128 * (1 << 10);
// previous wtr default in xsv is 32k, we're making it 512k
pub const DEFAULT_WTR_BUFFER_CAPACITY: usize = 512 * (1 << 10);

// number of rows for csv-nose to sample
const DEFAULT_SNIFFER_SAMPLE: usize = 100;

// file size at which we warn user that a large file has not been indexed
const NO_INDEX_WARNING_FILESIZE: u64 = 100 * (1 << 20); // 100MB

pub static SPONSOR_MESSAGE: &str = r"sponsored by datHere - Data Infrastructure Engineering (https://qsv.datHere.com)
Need a UI & more advanced data-wrangling? Upgrade to qsv pro (https://qsvpro.datHere.com)
";

pub static TEMP_FILE_DIR: OnceLock<PathBuf> = OnceLock::new();

// Index paths this process has already rebuilt because they looked stale.
//
// Several parallel workers call `index_files()` on the same input and can all observe the
// same stale index. Worse, a data file whose mtime is in the FUTURE never stops looking
// stale, so the condition does not clear after a rebuild. Without this, every worker
// rebuilds - each `File::create`ing (truncating) the very index the others are reading,
// which corrupts it and fails the run with a bare "Invalid argument".
//
// The lock is held ACROSS the rebuild, so late arrivals block until it is complete and then
// simply re-open the finished file.
static AUTOINDEXED_STALE: OnceLock<std::sync::Mutex<std::collections::HashSet<PathBuf>>> =
    OnceLock::new();

fn autoindexed_stale() -> &'static std::sync::Mutex<std::collections::HashSet<PathBuf>> {
    AUTOINDEXED_STALE.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()))
}

#[cfg(feature = "polars")]
pub static POLARS_FLOAT_PRECISION: OnceLock<Option<usize>> = OnceLock::new();

// Variants are constructed by `get_special_format` but only meaningfully matched
// when the `polars` feature is enabled (via `util::convert_special_format`),
// so non-polars builds see them as never read.
//
// Exception: `CompressedZip` is both *detected* and *handled* in all builds. It
// is handled by `util::extract_zip_to_temp` (always compiled — it needs only the
// non-optional `zip` crate), and `Config::new` preserves `CompressedZip` even in
// non-polars builds (mapping only the other, polars-only variants to `Unknown`).
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialFormat {
    Avro,
    Parquet,
    Ipc,
    Json,  // expects JSON Array
    Jsonl, // expects JSON Lines
    CompressedCsv,
    CompressedTsv,
    CompressedSsv,
    CompressedZip,
    Unknown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delimiter(pub u8);

/// Delimiter represents values that can be passed from the command line that
/// can be used as a field delimiter in CSV data.
///
/// Its purpose is to ensure that the Unicode character given decodes to a
/// valid ASCII character as required by the CSV parser.
impl Delimiter {
    pub const fn as_byte(self) -> u8 {
        self.0
    }

    pub fn decode_delimiter(s: &str) -> Result<Delimiter, String> {
        if s == r"\t" {
            return Ok(Delimiter(b'\t'));
        }

        if s.len() != 1 {
            return fail_format!("Could not convert '{s}' to a single ASCII character.");
        }

        let c = s.chars().next().unwrap();
        if c.is_ascii() {
            Ok(Delimiter(c as u8))
        } else {
            fail_format!("Could not convert '{c}' to ASCII delimiter.")
        }
    }
}

impl<'de> Deserialize<'de> for Delimiter {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Delimiter, D::Error> {
        let s = String::deserialize(d)?;
        match Delimiter::decode_delimiter(&s) {
            Ok(delim) => Ok(delim),
            Err(msg) => Err(D::Error::custom(msg)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub path:              Option<PathBuf>, // None implies <stdin>
    idx_path:              Option<PathBuf>,
    select_columns:        Option<SelectColumns>,
    delimiter:             u8,
    // The detected special input format (Parquet/Avro/Arrow/JSON/JSONL, compressed
    // CSV, or zip). Conversion to a delimited temp file is deferred to the read
    // path (see `prepared_for_read`), so a Config used only for writing never
    // converts its (output) path. `Unknown` for ordinary inputs, stdin & writers.
    special_format:        SpecialFormat,
    // Lazily-resolved, cached (shared across clones) converted-input (temp path,
    // delimiter) for `special_format` inputs. Populated on first read.
    read_input:            Arc<OnceLock<Result<(PathBuf, u8), String>>>,
    pub no_headers:        bool,
    pub flexible:          bool,
    terminator:            csv::Terminator,
    pub quote:             u8,
    quote_style:           csv::QuoteStyle,
    double_quote:          bool,
    escape:                Option<u8>,
    quoting:               bool,
    pub preamble_rows:     u64,
    trim:                  csv::Trim,
    pub autoindex_size:    u64,
    prefer_dmy:            bool,
    pub comment:           Option<u8>,
    snappy:                bool, // flag to enable snappy compression/decompression
    pub read_buffer:       u32,
    pub write_buffer:      u32,
    pub skip_format_check: bool,
    pub format_error:      Option<String>,
}

// Empty trait as an alias for Seek and Read that avoids auto trait errors
pub trait SeekRead: io::Seek + io::Read {}
impl<T: io::Seek + io::Read> SeekRead for T {}

/// Parse the named env var as `T`, falling back to `default` if it is unset or invalid.
/// Logs a warning if the env var is set but cannot be parsed.
fn parse_env_or_warn<T: std::str::FromStr + std::fmt::Display>(name: &str, default: T) -> T {
    match env::var(name) {
        Ok(s) => s.parse().unwrap_or_else(|_| {
            warn!("invalid {name} value {s:?}; using default {default}");
            default
        }),
        Err(_) => default,
    }
}

impl Config {
    /// Creates a new `Config` instance with default settings and optional file path.
    ///
    /// # Arguments
    ///
    /// * `path` - An optional reference to a `String` representing the file path.
    ///
    /// # Returns
    ///
    /// A new `Config` instance.
    ///
    /// # Details
    ///
    /// This function initializes a `Config` with the following behavior:
    /// - Uses env var `QSV_DEFAULT_DELIMITER` for default delimiter, or ',' if not set
    /// - Determines delimiter and Snappy compression based on file extension.
    /// - Supports sniffing delimiter and preamble rows if `QSV_SNIFF_DELIMITER` or
    ///   `QSV_SNIFF_PREAMBLE` is set.
    /// - Sets comment character from `QSV_COMMENT_CHAR` environment variable.
    /// - Sets headers behavior based on `QSV_NO_HEADERS` environment variable.
    /// - Configures various other settings from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `QSV_DEFAULT_DELIMITER`: Sets the default delimiter.
    /// - `QSV_SNIFF_DELIMITER` or `QSV_SNIFF_PREAMBLE`: Enables sniffing of delimiter and preamble
    ///   rows.
    /// - `QSV_COMMENT_CHAR`: Sets the comment character.
    /// - `QSV_NO_HEADERS`: Determines if the file has headers.
    /// - `QSV_AUTOINDEX_SIZE`: Sets the auto-index size.
    /// - `QSV_PREFER_DMY`: Sets date format preference.
    /// - `QSV_RDR_BUFFER_CAPACITY`: Sets read buffer capacity.
    /// - `QSV_WTR_BUFFER_CAPACITY`: Sets write buffer capacity.
    /// - `QSV_SKIP_FORMAT_CHECK`: Set to skip file extension checking.
    ///
    /// # This constructor may perform network I/O
    ///
    /// A `dc:<name>` input is a handle into the `get` command's disk cache, and it is resolved
    /// to a concrete path HERE. If the cached entry has reached its TTL (and its refresh policy
    /// is not `never`), that resolution revalidates the entry against its original source — a
    /// network round-trip, from inside a constructor that returns `Config` rather than
    /// `Result`. Surprising, and deliberate; see the bounds below.
    ///
    /// - **At most once per run.** `diskcache::DC_RESOLVED` memoizes a resolved handle for the life
    ///   of the process, so a command that builds many `Config`s cannot multiply that into many
    ///   fetches, and every consumer in a run sees the SAME materialized CSV. Before that memo,
    ///   `qsv frequency dc:x` resolved the handle 4 times in one run (issue #4257).
    /// - **Never fails the constructor.** A failed refresh falls back to the stale cached copy. A
    ///   handle that cannot be resolved at all is stashed in `format_error` and surfaced at reader
    ///   construction (`reader`, `reader_file`, `reader_file_stdin`).
    ///
    /// Why it stays here (issue #4274): the fetch cannot simply be hoisted to command entry.
    /// There is no uniform input-token position across commands, `qsv get`'s own subcommands
    /// take `dc:` names and are contractually offline, several `Args` structs are built without
    /// argv at all, and 39 commands reach the cache ONLY through this constructor — so hoisting
    /// would silently stop refreshing for them. The coherent fix is to defer resolution to the
    /// lazy read path (the `read_input` `OnceLock` / `prepared_for_read` machinery that
    /// auto-decompression already uses), which is a wider change than the problem warrants.
    pub fn new(path: Option<&String>) -> Config {
        // Resolve a `dc:<name>` cache reference (the `get` command's disk cache) to a concrete
        // file path up front, so the rest of Config::new treats it like an ordinary local file
        // (delimiter detection, indexing, etc.).
        //
        // NOTE: this MAY FETCH. Past TTL, `resolve_dc_path` revalidates against the origin.
        // It is memoized per run, and Config::new is infallible, so a resolution failure is
        // surfaced later via `format_error` rather than panicking. See the doc comment above.
        #[cfg(feature = "get")]
        let mut dc_format_error: Option<String> = None;
        #[cfg(feature = "get")]
        let dc_resolved: Option<String> = match path.and_then(|s| s.strip_prefix("dc:")) {
            Some(dc_name) => match crate::diskcache::resolve_dc_path(dc_name) {
                Ok(p) => Some(p.to_string_lossy().to_string()),
                Err(e) => {
                    dc_format_error = Some(e.to_string());
                    None
                },
            },
            _ => None,
        };
        #[cfg(feature = "get")]
        let path: Option<&String> = dc_resolved.as_ref().or(path);

        let default_delim = match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => match Delimiter::decode_delimiter(&delim) {
                Ok(d) => d.as_byte(),
                Err(e) => {
                    warn!("invalid QSV_DEFAULT_DELIMITER {delim:?} ({e}); using ','");
                    b','
                },
            },
            _ => b',',
        };
        let mut sniff = util::get_envvar_flag("QSV_SNIFF_DELIMITER")
            || util::get_envvar_flag("QSV_SNIFF_PREAMBLE");
        let mut skip_format_check = true;
        let mut format_error = None;
        let (path, mut delim, snappy, special_format) = match path {
            None => (None, default_delim, false, SpecialFormat::Unknown),
            // WIP: support remote files; currently only http(s) is supported
            // Some(ref s) if s.starts_with("http") && Url::parse(s).is_ok() => {
            //     let mut snappy = false;
            //     let delim = if s.ends_with(".csv.sz") {
            //         snappy = true;
            //         b','
            //     } else if s.ends_with(".tsv.sz") || s.ends_with(".tab.sz") {
            //         snappy = true;
            //         b'\t'
            //     } else {
            //         default_delim
            //     };
            //     // download the file to a temporary location
            //     util::download_file()
            //     (Some(PathBuf::from(s)), delim, snappy)
            // },
            Some(s) if s == "-" => (None, default_delim, false, SpecialFormat::Unknown),
            Some(s) => {
                let path = PathBuf::from(s);

                // if QSV_SKIP_FORMAT_CHECK is set or path is a temp file, we skip format check.
                //
                // `get()`, NOT `get_or_init()`. The temp dir is only ever READ here, to answer
                // a path comparison - yet initializing eagerly meant every `Config::new`
                // CREATED a temp directory just to ask that question, and needed a fallback
                // when creation failed. That fallback is what could put a directory qsv does
                // not own into `TEMP_FILE_DIR`, which `util::log_end` then deletes wholesale
                // (roborev 4366, 4367). There is no fallback to get wrong now.
                //
                // Behavior is unchanged: if this process has not created a temp dir yet, no
                // path can be inside one. Previously `get_or_init` would mint a FRESH
                // randomly-named dir here, which `starts_with` could never match either.
                skip_format_check = sniff
                    || util::get_envvar_flag("QSV_SKIP_FORMAT_CHECK")
                    || crate::config::TEMP_FILE_DIR
                        .get()
                        .is_some_and(|temp_dir| path.starts_with(temp_dir));

                // Detect special formats. The actual conversion to a delimited temp
                // file is DEFERRED to the read path (see `prepared_for_read`), so a
                // Config used only for writing never converts its (output) path.
                // `.zip` is detected even without polars (it needs only the `zip`
                // crate); the other special formats require polars to convert and so
                // stay `Unknown` otherwise.
                #[cfg(feature = "polars")]
                let special_format = get_special_format(&path);
                #[cfg(not(feature = "polars"))]
                let special_format = if get_special_format(&path) == SpecialFormat::CompressedZip {
                    SpecialFormat::CompressedZip
                } else {
                    SpecialFormat::Unknown
                };

                // Delimiter/snappy come from the path's own extension. For special
                // formats this is the write/fallback delimiter; the read delimiter
                // is re-derived from the converted temp in `prepared_for_read`.
                let (file_extension, delim, snappy) = get_delim_by_extension(&path, default_delim);

                if special_format == SpecialFormat::Unknown {
                    // Only ordinary inputs are subject to the extension check.
                    format_error = if skip_format_check {
                        None
                    } else {
                        match file_extension.as_str() {
                            "csv" | "tsv" | "tab" | "ssv" => None,
                            ext => Some(format!(
                                "{} is using an unsupported file format: {ext}. Set \
                                 QSV_SKIP_FORMAT_CHECK to skip input format checking.",
                                path.display()
                            )),
                        }
                    };
                } else {
                    // Don't sniff a binary/compressed special-format file; the
                    // converted temp is what actually gets read.
                    sniff = false;
                }
                (Some(path), delim, snappy, special_format)
            },
        };
        let comment: Option<u8> = env::var("QSV_COMMENT_CHAR")
            .ok()
            .and_then(|s| s.as_bytes().first().copied());
        let no_headers = util::get_envvar_flag("QSV_NO_HEADERS");
        let mut preamble = 0_u64;
        if let (true, Some(sniff_path_buf)) = (sniff, path.as_ref()) {
            if let Some(sniff_path) = sniff_path_buf.to_str() {
                match Sniffer::new()
                    .sample_size(SampleSize::Records(DEFAULT_SNIFFER_SAMPLE))
                    .sniff_path(sniff_path)
                {
                    Ok(metadata) => {
                        delim = metadata.dialect.delimiter;
                        preamble = metadata.dialect.header.num_preamble_rows as u64;
                        info!(
                            "sniffed delimiter {} and {preamble} preamble rows",
                            delim as char
                        );
                    },
                    // we only warn, as we don't want to stop processing the file
                    // if sniffing doesn't work
                    Err(e) => warn!("sniff error: {e}"),
                }
            } else {
                warn!(
                    "skipping delimiter sniff: path {} is not valid UTF-8",
                    sniff_path_buf.display()
                );
            }
        }

        // A failed `dc:` resolution takes precedence so its (more actionable)
        // error is what surfaces at reader-construction time.
        #[cfg(feature = "get")]
        let format_error = dc_format_error.or(format_error);

        Config {
            path,
            idx_path: None,
            select_columns: None,
            delimiter: delim,
            special_format,
            read_input: Arc::new(OnceLock::new()),
            no_headers,
            flexible: false,
            terminator: csv::Terminator::Any(b'\n'),
            quote: b'"',
            quote_style: csv::QuoteStyle::Necessary,
            double_quote: true,
            escape: None,
            quoting: true,
            preamble_rows: preamble,
            trim: csv::Trim::None,
            autoindex_size: parse_env_or_warn("QSV_AUTOINDEX_SIZE", 0_u64),
            prefer_dmy: util::get_envvar_flag("QSV_PREFER_DMY"),
            comment,
            snappy,
            read_buffer: parse_env_or_warn(
                "QSV_RDR_BUFFER_CAPACITY",
                DEFAULT_RDR_BUFFER_CAPACITY as u32,
            ),
            write_buffer: parse_env_or_warn(
                "QSV_WTR_BUFFER_CAPACITY",
                DEFAULT_WTR_BUFFER_CAPACITY as u32,
            ),
            format_error,
            skip_format_check,
        }
    }

    pub const fn delimiter(mut self, d: Option<Delimiter>) -> Config {
        if let Some(d) = d {
            self.delimiter = d.as_byte();
        }
        self
    }

    pub fn get_delimiter(&self) -> u8 {
        // For a special-format input the effective delimiter is the converted
        // temp's (e.g. a `.tsv` entry inside a `.zip`), not the outer path's
        // default. Resolve (cached) so callers see the real delimiter regardless
        // of whether they've read yet; fall back to the configured delimiter if
        // conversion fails.
        if self.special_format != SpecialFormat::Unknown
            && let Ok((_, delim)) = self.resolve_converted()
        {
            return delim;
        }
        self.delimiter
    }

    pub const fn comment(mut self, c: Option<u8>) -> Config {
        self.comment = c;
        self
    }

    pub const fn get_dmy_preference(&self) -> bool {
        self.prefer_dmy
    }

    /// Explicitly set `no_headers`, unconditionally overriding env var.
    /// Use this when a command knows the input has (or lacks) headers
    /// regardless of user configuration (e.g. internally-generated CSVs).
    pub const fn no_headers(mut self, yes: bool) -> Config {
        self.no_headers = yes;
        self
    }

    /// Apply the `--no-headers` CLI flag without overriding `QSV_NO_HEADERS` env var.
    /// When the flag is `false` (not passed), the env var value is preserved.
    /// When the flag is `true` (explicitly passed), it sets `no_headers = true`.
    /// Also respects `QSV_TOGGLE_HEADERS` to flip the flag value.
    pub fn no_headers_flag(mut self, mut yes: bool) -> Config {
        if env::var("QSV_TOGGLE_HEADERS").unwrap_or_else(|_| "0".to_owned()) == "1" {
            yes = !yes;
        }
        self.no_headers = self.no_headers || yes;
        self
    }

    pub const fn flexible(mut self, yes: bool) -> Config {
        self.flexible = yes;
        self
    }

    pub const fn skip_format_check(mut self, yes: bool) -> Config {
        self.skip_format_check = yes;
        self
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    pub const fn crlf(mut self, yes: bool) -> Config {
        if yes {
            self.terminator = csv::Terminator::CRLF;
        } else {
            self.terminator = csv::Terminator::Any(b'\n');
        }
        self
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    pub const fn terminator(mut self, term: csv::Terminator) -> Config {
        self.terminator = term;
        self
    }

    pub const fn quote(mut self, quote: u8) -> Config {
        self.quote = quote;
        self
    }

    pub const fn quote_style(mut self, style: csv::QuoteStyle) -> Config {
        self.quote_style = style;
        self
    }

    pub const fn double_quote(mut self, yes: bool) -> Config {
        self.double_quote = yes;
        self
    }

    pub const fn escape(mut self, escape: Option<u8>) -> Config {
        self.escape = escape;
        self
    }

    pub const fn quoting(mut self, yes: bool) -> Config {
        self.quoting = yes;
        self
    }

    pub const fn trim(mut self, trim_type: csv::Trim) -> Config {
        self.trim = trim_type;
        self
    }

    pub fn set_read_buffer(mut self, buffer: usize) -> Config {
        self.read_buffer = u32::try_from(buffer).unwrap_or_else(|_| {
            warn!(
                "read buffer {buffer} exceeds u32::MAX; using default \
                 {DEFAULT_RDR_BUFFER_CAPACITY}"
            );
            DEFAULT_RDR_BUFFER_CAPACITY as u32
        });
        self
    }

    pub fn set_write_buffer(mut self, buffer: usize) -> Config {
        self.write_buffer = u32::try_from(buffer).unwrap_or_else(|_| {
            warn!(
                "write buffer {buffer} exceeds u32::MAX; using default \
                 {DEFAULT_WTR_BUFFER_CAPACITY}"
            );
            DEFAULT_WTR_BUFFER_CAPACITY as u32
        });
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn select(mut self, sel_cols: SelectColumns) -> Config {
        self.select_columns = Some(sel_cols);
        self
    }

    pub const fn is_stdin(&self) -> bool {
        self.path.is_none()
    }

    #[cfg(feature = "polars")]
    pub const fn is_snappy(&self) -> bool {
        self.snappy
    }

    #[inline]
    /// Returns a `Selection` based on the config's `select_columns` & the first record of the CSV.
    ///
    /// # Arguments
    ///
    /// * `first_record` - A reference to the first `ByteRecord` of the CSV.
    ///
    /// # Returns
    ///
    /// * `Result<Selection, String>` - A `Selection` if successful, otherwise, an error msg
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The `Config` has no `SelectColumns` (i.e., `Config::select` was not called).
    pub fn selection(&self, first_record: &csv::ByteRecord) -> Result<Selection, String> {
        match self.select_columns {
            None => fail!("Config has no 'SelectColumns'. Did you call Config::select?"),
            Some(ref sel) => sel.selection(first_record, !self.no_headers),
        }
    }

    /// Writes the headers from a CSV reader to a CSV writer.
    ///
    /// This function reads the headers from the given CSV reader and writes them to the CSV writer,
    /// but only if the `no_headers` flag is not set. If the headers are empty, nothing is written.
    ///
    /// # Arguments
    ///
    /// * `r` - A mutable reference to a CSV reader.
    /// * `w` - A mutable reference to a CSV writer.
    ///
    /// # Returns
    ///
    /// Returns a `csv::Result<()>` which is `Ok(())` if the operation was successful,
    /// or an error if there was a problem reading or writing.
    pub fn write_headers<R: io::Read, W: io::Write>(
        &self,
        r: &mut csv::Reader<R>,
        w: &mut csv::Writer<W>,
    ) -> csv::Result<()> {
        if !self.no_headers {
            let r = r.byte_headers()?;
            if !r.is_empty() {
                w.write_record(r)?;
            }
        }
        Ok(())
    }

    pub fn writer(&self) -> io::Result<csv::Writer<Box<dyn io::Write + 'static>>> {
        Ok(self.from_writer(self.io_writer()?))
    }

    /// Lazily convert a `special_format` input to a delimited temp file, returning
    /// the temp path and its delimiter. The result is cached and shared across
    /// clones, so a Config that is read more than once converts only once. With an
    /// explicit `QSV_SKIP_FORMAT_CHECK`, a conversion failure falls back to reading
    /// the original bytes as-is rather than erroring.
    fn resolve_converted(&self) -> io::Result<(PathBuf, u8)> {
        // safety: only called when special_format != Unknown, which implies a path
        let src = self
            .path
            .as_ref()
            .expect("special-format Config must have a path");
        let cached = self.read_input.get_or_init(|| {
            match util::convert_special_format(src, self.special_format, self.delimiter) {
                Ok(temp) => {
                    let (_, delim, _) = get_delim_by_extension(&temp, self.delimiter);
                    Ok((temp, delim))
                },
                Err(e) => Err(format!("Failed to convert special format: {e}")),
            }
        });
        match cached {
            Ok((p, d)) => Ok((p.clone(), *d)),
            Err(e) => {
                // A detected special format that fails to convert is a hard error
                // for ALL formats — reading the raw (binary) bytes as delimited text
                // silently produces garbage with a success exit code. The only escape
                // hatch is an explicit QSV_SKIP_FORMAT_CHECK, which means "I know what
                // I'm doing, read the original bytes as-is".
                if util::get_envvar_flag("QSV_SKIP_FORMAT_CHECK") {
                    Ok((src.clone(), self.delimiter))
                } else {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, e.clone()))
                }
            },
        }
    }

    /// The path that will actually be read. For a `special_format` input this
    /// lazily converts to (and returns the path of) the delimited temp file, so
    /// callers that need the *decompressed* size — e.g. `util::mem_file_check`
    /// memory guards — see the real data rather than the compressed source. For
    /// ordinary inputs and stdin, returns `path` unchanged. The conversion is
    /// cached, so the subsequent reader reuses the same temp.
    pub fn resolved_path(&self) -> CliResult<Option<PathBuf>> {
        if self.special_format == SpecialFormat::Unknown {
            return Ok(self.path.clone());
        }
        Ok(Some(self.resolve_converted()?.0))
    }

    /// Whether this input is a special format (`.gz`/`.zip`/`.parquet`/`.jsonl`/...) that is read
    /// through a CONVERTED temp file rather than directly.
    ///
    /// Callers that reconstruct a fresh `Config` per worker need this: each `Config` converts to
    /// its OWN temp path, so anything keyed to that path - an autoindex, most notably - is
    /// invisible to every other `Config` built from the same input.
    #[inline]
    pub const fn is_special_format(&self) -> bool {
        !matches!(self.special_format, SpecialFormat::Unknown)
    }

    /// Returns a Config ready to *read* this input. For a `special_format` input,
    /// it is a clone whose `path` points at the lazily-converted delimited temp
    /// (with that temp's delimiter) and whose `special_format` is `Unknown`, so the
    /// read methods treat it as an ordinary delimited file (no re-entry). Only
    /// called from the read entry points when `special_format != Unknown`.
    pub(crate) fn prepared_for_read(&self) -> io::Result<Config> {
        if self.special_format == SpecialFormat::Unknown {
            return Ok(self.clone());
        }
        let (temp, delim) = self.resolve_converted()?;
        let mut c = self.clone();
        c.path = Some(temp);
        c.delimiter = delim;
        c.special_format = SpecialFormat::Unknown;
        Ok(c)
    }

    pub fn reader(&self) -> io::Result<csv::Reader<Box<dyn io::Read + Send + 'static>>> {
        if self.special_format != SpecialFormat::Unknown {
            return self.prepared_for_read()?.reader();
        }
        if !self.skip_format_check && self.format_error.is_some() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                self.format_error.clone().unwrap(),
            ))
        } else {
            Ok(self.from_reader(self.io_reader()?))
        }
    }

    pub fn reader_file(&self) -> io::Result<csv::Reader<fs::File>> {
        if self.special_format != SpecialFormat::Unknown {
            return self.prepared_for_read()?.reader_file();
        }
        match self.path {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use <stdin> here",
            )),
            Some(ref p) => {
                if !self.skip_format_check && self.format_error.is_some() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        self.format_error.clone().unwrap(),
                    ))
                } else {
                    fs::File::open(p).map(|f| self.from_reader(f))
                }
            },
        }
    }

    pub fn reader_file_stdin(&self) -> io::Result<csv::Reader<Box<dyn SeekRead + 'static>>> {
        if self.special_format != SpecialFormat::Unknown {
            return self.prepared_for_read()?.reader_file_stdin();
        }
        Ok(match self.path {
            None => {
                // Create a buffer in memory for stdin
                let mut buffer: Vec<u8> = Vec::new();
                let stdin = io::stdin();
                stdin.lock().read_to_end(&mut buffer)?;
                self.from_reader(Box::new(io::Cursor::new(buffer)))
            },
            Some(ref p) => {
                if !self.skip_format_check && self.format_error.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        self.format_error.clone().unwrap(),
                    ));
                }
                self.from_reader(Box::new(fs::File::open(p)?))
            },
        })
    }

    /// Automatically creates an index file for the CSV file.
    ///
    /// This function attempts to create an index file for the CSV file specified in `self.path`.
    /// It's designed to fail silently if any step of the process encounters an error, as it's
    /// intended to be a convenience function.
    ///
    /// # Behavior
    ///
    /// - If the file is Snappy-compressed, the function returns immediately w/o creating an index.
    /// - If `self.path` is `None`, the function returns without action.
    /// - The function creates an index file using `util::idx_path()` to determine index file path.
    /// - It builds the index into a sibling temp file and `rename`s it into place, so the existing
    ///   index is never truncated and readers never observe a partial one.
    /// - Returns whether an index was successfully put in place. Callers that memoize the rebuild
    ///   must only record it on `true`.
    /// - No process-global state is set here. A subsequent `index_files()` discovers the new index
    ///   by looking for it beside `self.path`, like any other index.
    ///
    /// # Errors
    ///
    /// While this function doesn't return any errors, it logs debug messages for both successful
    /// and failed index creation attempts.
    fn autoindex_file(&self) -> bool {
        if self.snappy {
            return false;
        }

        let Some(path_buf) = &self.path else {
            return false;
        };

        let pidx = util::idx_path(Path::new(path_buf));

        // Build into a SIBLING temp file and rename into place only after the index is
        // complete. Writing straight to `pidx` means `File::create` TRUNCATES it before a
        // single byte of the new index is written - so any later failure leaves an empty or
        // partial index behind, and a concurrent reader can open one mid-write. `rename` on
        // the same directory is atomic: readers see either the old complete index or the new
        // one, never a torn file.
        // `tempfile` picks a UNIQUE name and creates it exclusively (O_EXCL), so two
        // concurrent builds - which the `autoindex_size` path below does not serialize -
        // cannot land on the same file, and a pre-existing file or symlink at a guessable
        // name cannot be written through. A deterministic `<idx>.tmp<pid>` had both problems:
        // same-process builds share a pid.
        let idx_dir = pidx.parent().unwrap_or_else(|| Path::new("."));
        let Ok(mut tmp) = tempfile::Builder::new()
            .prefix(".qsv-idx")
            .tempfile_in(idx_dir)
        else {
            return false;
        };
        let Ok(mut rdr) = self.reader_file() else {
            // `tmp` removes itself on drop
            return false;
        };

        let build = {
            let mut wtr =
                io::BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, tmp.as_file_mut());
            csv_index::RandomAccessSimple::create(&mut rdr, &mut wtr)
                .and_then(|()| io::Write::flush(&mut wtr).map_err(Into::into))
        };
        if let Err(e) = build {
            debug!("autoindex of {} failed: {e}", path_buf.display());
            return false;
        }

        // persist() renames into place, which is atomic on the same filesystem: readers see
        // either the old complete index or the new one, never a torn file
        match tmp.persist(&pidx) {
            Ok(_) => {
                debug!("autoindex of {} successful.", path_buf.display());
                true
            },
            Err(e) => {
                debug!(
                    "autoindex of {} could not be put in place: {e}",
                    path_buf.display()
                );
                false
            },
        }
    }

    /// Check if the index file exists and is newer than the CSV file.
    /// If so, return the CSV file handle and the index file handle. If not, return None.
    /// Unless the CSV's file size >= `QSV_AUTOINDEX_SIZE`, then we'll create an index
    /// automatically. Stale indices (CSV newer than index) are rebuilt automatically on the
    /// `(Some(path), None)` branch that resolves the index path internally; only the
    /// explicit-`(path, idx_path)` branch skips the staleness recheck, since the caller
    /// supplied both paths and is trusted.
    pub fn index_files(&self) -> io::Result<Option<(csv::Reader<fs::File>, fs::File)>> {
        if self.special_format != SpecialFormat::Unknown {
            return self.prepared_for_read()?.index_files();
        }
        // Track the data file's mtime and the resolved index path *only* on the
        // path that may need a staleness recheck. For the explicit-(path, idx_path)
        // branch, staleness is not re-checked, so these stay at their default values.
        let mut data_modified = 0_u64;
        let data_fsize;
        let mut idx_path_work: Option<PathBuf> = None;

        // NOTE: there was once an `AUTO_INDEXED` global fast path here, skipping the
        // existence check whenever ANY file had been autoindexed in this process. It was
        // unsound: the flag carried no path, so it also fired for a DIFFERENT input, whose
        // sibling `.idx` then did not exist - and its `fs::File::open` was unconditional, so
        // the result was a hard ENOENT instead of a graceful "no index" (issue #4463).
        //
        // It bit whenever one command resolved one input through two `Config`s - each
        // converting a special-format input to its OWN temp - which is exactly what
        // `stats --infer-dates` does via `sniff`. The branch below already handles both
        // "index present" and "index absent" correctly, and unlike the fast path it performs
        // the staleness recheck. It costs two `metadata()` calls per `indexed()`, which
        // happens at setup and once per parallel worker - never per record.
        let (csv_file, mut idx_file) = match (&self.path, &self.idx_path) {
            (&None, &None) => return Ok(None),
            (&None, &Some(_)) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Cannot use <stdin> with indexes",
                ));
            },
            // When the caller supplies both paths explicitly, trust them and skip
            // the staleness recheck below (idx_path_work stays None).
            (Some(p), Some(ip)) => (fs::File::open(p)?, fs::File::open(ip)?),
            (Some(p), &None) => {
                // We generally don't want to report an error here, since we're
                // passively trying to find an index.

                (data_modified, data_fsize) = util::file_metadata(&p.metadata()?);
                let idx_path = util::idx_path(p);
                let idx_file = match fs::File::open(&idx_path) {
                    Err(_) => {
                        // the index file doesn't exist
                        if self.snappy {
                            // cannot index snappy compressed files
                            return Ok(None);
                        } else if self.autoindex_size > 0 && data_fsize >= self.autoindex_size {
                            // if CSV file size >= QSV_AUTOINDEX_SIZE, and
                            // its not a snappy file, create an index automatically.
                            // If that fails, fall back to "no index" rather than erroring -
                            // this branch is passively LOOKING for an index, and the caller
                            // can still process the file sequentially. `autoindex_file`
                            // renames into place only on success, so there is no partial
                            // index to open here.
                            if !self.autoindex_file() {
                                return Ok(None);
                            }
                            fs::File::open(&idx_path)?
                        } else if data_fsize >= NO_INDEX_WARNING_FILESIZE {
                            // warn user that the CSV file is large and not indexed
                            use indicatif::HumanBytes;

                            warn!(
                                "The {} CSV file is larger than the {} NO_INDEX_WARNING_FILESIZE \
                                 threshold. Consider creating an index file as it will make qsv \
                                 commands much faster.",
                                HumanBytes(data_fsize),
                                HumanBytes(NO_INDEX_WARNING_FILESIZE)
                            );
                            return Ok(None);
                        } else {
                            // CSV not greater than QSV_AUTOINDEX_SIZE, and not greater than
                            // NO_INDEX_WARNING_FILESIZE, so we don't create an index
                            return Ok(None);
                        }
                    },
                    Ok(f) => f,
                };
                idx_path_work = Some(idx_path);
                (fs::File::open(p)?, idx_file)
            },
        };
        // If the CSV data was last modified after the index file was last
        // modified, recreate the stale index automatically. Only checked when
        // we resolved the index path ourselves (idx_path_work is Some).
        if let Some(idx_path) = &idx_path_work {
            let (idx_modified, _) = util::file_metadata(&idx_file.metadata()?);
            if data_modified > idx_modified {
                // Rebuild AT MOST ONCE per path per process, and never concurrently - see
                // `AUTOINDEXED_STALE`. Late arrivals block here until the rebuild finishes,
                // then fall through and re-open the completed index.
                let usable = {
                    let mut rebuilt = autoindexed_stale()
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    if rebuilt.contains(idx_path) {
                        // an earlier caller in this process already rebuilt it successfully
                        true
                    } else {
                        info!("index stale... autoindexing...");
                        // record ONLY on success: `autoindex_file` reports failure quietly,
                        // and memoizing a failed rebuild would poison this path for the rest
                        // of the process - every later caller skipping the repair and reusing
                        // the stale index.
                        let ok = self.autoindex_file();
                        if ok {
                            rebuilt.insert(idx_path.clone());
                        }
                        ok
                    }
                };
                if !usable {
                    // The index is stale and could not be refreshed. Do NOT hand back the
                    // stale one: we just determined its offsets no longer describe the data,
                    // so a caller seeking with it can read the wrong rows or miss rows
                    // entirely while believing the index is valid. Reporting "no index" sends
                    // the caller down its sequential path, which is slower but correct.
                    warn!(
                        "index for {} is stale and could not be rebuilt; proceeding without an \
                         index",
                        self.path
                            .as_ref()
                            .map_or_else(String::new, |p| p.display().to_string())
                    );
                    return Ok(None);
                }
                idx_file = fs::File::open(idx_path)?;
            }
        }

        let csv_rdr = self.from_reader(csv_file);
        Ok(Some((csv_rdr, idx_file)))
    }

    /// Check if the index file exists and is newer than the CSV file.
    /// If so, return the index file.
    /// If not, return None.
    /// Unless `QSV_AUTOINDEX_SIZE` is set, in which case, we'll recreate the
    /// stale index automatically
    #[inline]
    pub fn indexed(&self) -> CliResult<Option<Indexed<fs::File, fs::File>>> {
        match self.index_files()? {
            None => Ok(None),
            Some((r, i)) => Ok(Some(Indexed::open(r, i)?)),
        }
    }

    pub fn io_reader(&self) -> io::Result<Box<dyn io::Read + Send + 'static>> {
        if self.special_format != SpecialFormat::Unknown {
            return self.prepared_for_read()?.io_reader();
        }
        Ok(match self.path {
            None => Box::new(io::stdin()),
            Some(ref p) => match fs::File::open(p) {
                Ok(x) => {
                    if self.snappy {
                        // Validate that the file is actually a snappy-compressed file
                        // before attempting decompression. This prevents "corrupt input" errors
                        // when a plain CSV file is incorrectly detected as snappy.
                        match util::is_valid_snappy_file(p) {
                            Ok(true) => {
                                info!("decoding snappy-compressed file: {}", p.display());
                                Box::new(snap::read::FrameDecoder::new(x))
                            },
                            Ok(false) => {
                                warn!(
                                    "File {} has .sz extension but is not a valid Snappy file. \
                                     Reading as plain file.",
                                    p.display()
                                );
                                Box::new(x)
                            },
                            Err(e) => {
                                warn!(
                                    "Failed to validate Snappy file {}: {}. Reading as plain file.",
                                    p.display(),
                                    e
                                );
                                Box::new(x)
                            },
                        }
                    } else {
                        Box::new(x)
                    }
                },
                Err(err) => {
                    let msg = format!("failed to open {}: {}", p.display(), err);
                    return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                },
            },
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_reader<R: Read>(&self, rdr: R) -> csv::Reader<R> {
        csv::ReaderBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .has_headers(!self.no_headers)
            .quote(self.quote)
            .quoting(self.quoting)
            .escape(self.escape)
            .buffer_capacity(self.read_buffer as usize)
            .comment(self.comment)
            .trim(self.trim)
            .from_reader(rdr)
    }

    pub fn io_writer(&self) -> io::Result<Box<dyn io::Write + 'static>> {
        // `path` is the user-specified path (special-format conversion is deferred
        // to the read path and never rewrites it), so writers target it directly.
        Ok(match self.path {
            None => Box::new(io::stdout()),
            Some(ref p) => {
                if p == "sink" {
                    // sink is /dev/null
                    Box::new(io::sink())
                } else if self.snappy {
                    info!("writing snappy-compressed file: {}", p.display());
                    Box::new(snap::write::FrameEncoder::new(fs::File::create(p)?))
                } else {
                    Box::new(fs::File::create(p)?)
                }
            },
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_writer<W: io::Write>(&self, mut wtr: W) -> csv::Writer<W> {
        if util::get_envvar_flag("QSV_OUTPUT_BOM")
            && let Err(e) = wtr.write_all("\u{FEFF}".as_bytes())
        {
            // BOM is best-effort: a broken pipe here would otherwise abort the
            // whole process. Log and let the next real write surface the error.
            warn!("failed to write UTF-8 BOM: {e}");
        }

        csv::WriterBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .quote(self.quote)
            .quote_style(self.quote_style)
            .double_quote(self.double_quote)
            .escape(self.escape.unwrap_or(b'\\'))
            .buffer_capacity(self.write_buffer as usize)
            .from_writer(wtr)
    }
}

/// Checks if a file path has a Snappy compression extension (.sz).
///
/// # Arguments
///
/// * `path` - A reference to the `Path` of the file.
///
/// # Returns
///
/// `true` if the file has a `.sz` extension (case-insensitive), `false` otherwise.
///
/// # Details
///
/// This function uses Rust's `Path::extension()` method which properly handles
/// multiple extensions (e.g., `file.csv.sz` → `Some("sz")`). It performs
/// case-insensitive comparison for robustness.
#[inline]
pub fn is_snappy_extension(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sz"))
}

/// This function examines the file extension to determine:
/// 1. The appropriate delimiter (tab for .tsv/.tab, semicolon for .ssv, comma for .csv).
/// 2. Whether the file is Snappy-compressed (indicated by a .sz extension).
/// 3. For Snappy-compressed files, it checks the extension before .sz to determine the delimiter.
///
/// If the file extension doesn't match known types, it returns the default delimiter.
pub fn get_delim_by_extension(path: &Path, default_delim: u8) -> (String, u8, bool) {
    let snappy = is_snappy_extension(path);

    // Get the extension before .sz if it's a snappy file, otherwise get the normal extension
    let file_extension = if snappy {
        // For snappy files like file.csv.sz, we need to get "csv"
        // We can do this by getting the file stem, then checking its extension
        path.file_stem()
            .and_then(|stem| Path::new(stem).extension())
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
    } else {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_ascii_lowercase()
    };

    let delim = match file_extension.as_str() {
        "tsv" | "tab" => b'\t',
        "ssv" => b';',
        "csv" => b',',
        _ => default_delim,
    };

    (file_extension, delim, snappy)
}

/// Determines if a file is a Parquet, Arrow IPC, JSONL, or compressed CSV file.
///
/// # Arguments
///
/// * `path` - A reference to the `Path` of the file.
///
/// # Returns
///
/// A `SpecialFormat` enum value indicating the type of special format the file is.
pub fn get_special_format(path: &Path) -> SpecialFormat {
    if !path.exists() {
        return SpecialFormat::Unknown;
    }

    let extension = path.extension().unwrap_or_default();
    match extension
        .to_str()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "avro" => SpecialFormat::Avro,
        "parquet" => SpecialFormat::Parquet,
        "ipc" | "arrow" => SpecialFormat::Ipc,
        "jsonl" | "ndjson" => SpecialFormat::Jsonl,
        "json" => SpecialFormat::Json,
        "gz" | "zst" | "zlib" => compressed_csv_format(path),
        // zip is detected at the outer-extension level (not via
        // `compressed_csv_format`), since the inner entry's name — and thus the
        // delimiter — is only knowable after opening the archive.
        "zip" => SpecialFormat::CompressedZip,
        _ => SpecialFormat::Unknown,
    }
}

/// For a path like `data.csv.gz`, classify the inner CSV-family extension
/// (`csv`, `tsv`/`tab`, or `ssv`) into a `SpecialFormat::Compressed*` variant.
/// Returns `Unknown` if the inner extension is missing or not a known CSV family.
fn compressed_csv_format(path: &Path) -> SpecialFormat {
    let inner_ext = path
        .file_stem()
        .and_then(|stem| Path::new(stem).extension())
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase);
    match inner_ext.as_deref() {
        Some("csv") => SpecialFormat::CompressedCsv,
        Some("tsv" | "tab") => SpecialFormat::CompressedTsv,
        Some("ssv") => SpecialFormat::CompressedSsv,
        _ => SpecialFormat::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// A special-format input must remain AUTOINDEXABLE at the Config level.
    ///
    /// The `stats` parallel path cannot use such an index - it rebuilds a fresh Config per
    /// worker, so each resolves a different converted temp file - and `stats` therefore skips
    /// autoindexing these inputs itself. That skip must NOT live here: `frequency` deliberately
    /// hands its workers the already-resolved Config (see the comments at its `indexed()` call
    /// and in `parallel_ftables`), so its temp path is consistent across threads and its
    /// special-format autoindexed parallel path is safe. Disabling autoindex in `index_files`
    /// silently dropped `frequency` back to sequential processing on large compressed inputs.
    #[test]
    fn special_format_input_is_still_autoindexable() {
        use std::io::Write;

        let dir = tempfile::tempdir().unwrap();
        let zip_path = dir.path().join("ai.zip");

        let mut csv = String::from("a,b\n");
        for i in 0..200 {
            csv.push_str(&format!("{i},{}\n", i * 2));
        }

        let zf = std::fs::File::create(&zip_path).unwrap();
        let mut zw = zip::ZipWriter::new(zf);
        zw.start_file("ai.csv", zip::write::SimpleFileOptions::default())
            .unwrap();
        zw.write_all(csv.as_bytes()).unwrap();
        zw.finish().unwrap();

        let mut config = Config::new(Some(&zip_path.to_string_lossy().to_string()));
        assert!(
            config.is_special_format(),
            "a .zip input should be detected as a special format"
        );

        // an autoindex size below the fixture size asks for an index to be built
        config.autoindex_size = 100;
        let indexed = config
            .index_files()
            .expect("index_files should not error for a special-format input");
        assert!(
            indexed.is_some(),
            "a special-format input must still autoindex its converted temp file - `frequency` \
             shares the resolved Config with its workers and depends on this"
        );
    }

    #[test]
    fn test_csv_extension() {
        let path = PathBuf::from("test.csv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "csv");
        assert_eq!(delim, b',');
        assert!(!snappy);
    }

    #[test]
    fn test_tsv_extension() {
        let path = PathBuf::from("test.tsv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "tsv");
        assert_eq!(delim, b'\t');
        assert!(!snappy);
    }

    #[test]
    fn test_ssv_extension() {
        let path = PathBuf::from("test.ssv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "ssv");
        assert_eq!(delim, b';');
        assert!(!snappy);
    }

    #[test]
    fn test_snappy_csv_extension() {
        let path = PathBuf::from("test.csv.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "csv");
        assert_eq!(delim, b',');
        assert!(snappy);
    }

    #[test]
    fn test_snappy_tsv_extension() {
        let path = PathBuf::from("test.tsv.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "tsv");
        assert_eq!(delim, b'\t');
        assert!(snappy);
    }

    #[test]
    fn test_unknown_extension() {
        let path = PathBuf::from("test.unknown");
        let default_delim = b'|';
        let (ext, delim, snappy) = get_delim_by_extension(&path, default_delim);
        assert_eq!(ext, "unknown");
        assert_eq!(delim, default_delim);
        assert!(!snappy);
    }

    #[test]
    fn test_no_extension() {
        let path = PathBuf::from("test");
        let default_delim = b',';
        let (ext, delim, snappy) = get_delim_by_extension(&path, default_delim);
        assert_eq!(ext, "");
        assert_eq!(delim, default_delim);
        assert!(!snappy);
    }

    #[test]
    fn test_is_snappy_extension_lowercase() {
        assert!(is_snappy_extension(Path::new("file.csv.sz")));
        assert!(is_snappy_extension(Path::new("file.sz")));
        assert!(is_snappy_extension(Path::new("file.tsv.sz")));
    }

    #[test]
    fn test_is_snappy_extension_uppercase() {
        assert!(is_snappy_extension(Path::new("file.csv.SZ")));
        assert!(is_snappy_extension(Path::new("file.SZ")));
    }

    #[test]
    fn test_is_snappy_extension_mixed_case() {
        assert!(is_snappy_extension(Path::new("file.csv.Sz")));
        assert!(is_snappy_extension(Path::new("file.sZ")));
    }

    #[test]
    fn test_is_snappy_extension_not_snappy() {
        assert!(!is_snappy_extension(Path::new("file.csv")));
        assert!(!is_snappy_extension(Path::new("file.gz")));
        assert!(!is_snappy_extension(Path::new("file")));
        assert!(!is_snappy_extension(Path::new("file.sz.backup")));
        // Test that extensions ending with "sz" but not exactly "sz" don't trigger detection
        assert!(!is_snappy_extension(Path::new("file.esz")));
        assert!(!is_snappy_extension(Path::new("file.KYpPcb8esz")));
        assert!(!is_snappy_extension(Path::new("data.esz")));
    }

    #[test]
    fn test_snappy_ssv_extension() {
        let path = PathBuf::from("test.ssv.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "ssv");
        assert_eq!(delim, b';');
        assert!(snappy);
    }

    #[test]
    fn test_snappy_case_insensitive() {
        let path = PathBuf::from("test.csv.SZ");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "csv");
        assert_eq!(delim, b',');
        assert!(snappy);

        let path = PathBuf::from("test.TSV.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "tsv");
        assert_eq!(delim, b'\t');
        assert!(snappy);
    }

    /// A STALE index must be rebuilt AT MOST ONCE per process, and never concurrently.
    ///
    /// Several parallel workers call `index_files()` on the same input and all observe the
    /// same stale index; a data file whose mtime is in the FUTURE never stops looking stale,
    /// so the condition does not clear after a rebuild either. Before the fix every caller
    /// rebuilt - each truncating the index the others were mid-read of - corrupting it and
    /// failing the run with a bare "Invalid argument" from `Indexed::open`. That is what broke
    /// `stats` on a stale index (test_index::index_outdated_stats) on macOS CI.
    ///
    /// The assertion is on the rebuild COUNT, not on the crash: the corruption is a race that
    /// a fast machine reliably wins, so a concurrency-only test passes even with the fix
    /// removed (verified). Watching the index's mtime instead is deterministic - a second
    /// rebuild necessarily rewrites the file.
    #[test]
    fn stale_index_is_rebuilt_once_not_once_per_caller() {
        use std::io::Write;

        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("stale.csv");

        let mut f = std::fs::File::create(&csv_path).unwrap();
        writeln!(f, "letter,number").unwrap();
        for i in 0..2_000 {
            writeln!(f, "{},{i}", char::from(b'a' + (i % 26) as u8)).unwrap();
        }
        f.sync_all().unwrap();
        drop(f);

        let path_str = csv_path.to_string_lossy().to_string();
        let config = Config::new(Some(&path_str));
        crate::util::create_index_for_file(&csv_path, &config).unwrap();
        let idx_path = crate::util::idx_path(&csv_path);

        // push the DATA mtime into the future so the index looks stale to EVERY caller and
        // STAYS stale after a rebuild - the exact shape of test_index::index_outdated_stats
        let future = filetime::FileTime::from_unix_time(
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(&csv_path).unwrap())
                .unix_seconds()
                + 86_400,
            0,
        );
        filetime::set_file_mtime(&csv_path, future).unwrap();

        // first call: sees the stale index and rebuilds it
        assert!(
            config.indexed().unwrap().is_some(),
            "the first caller must get a usable index"
        );

        // stamp the index with a distinctive mtime; any FURTHER rebuild overwrites it
        let marker = filetime::FileTime::from_unix_time(1_000_000, 0);
        filetime::set_file_mtime(&idx_path, marker).unwrap();

        // later callers - the parallel workers - must reuse it rather than rebuild
        for i in 0..8 {
            assert!(
                Config::new(Some(&path_str)).indexed().unwrap().is_some(),
                "caller {i} must get a usable index"
            );
        }

        let after =
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(&idx_path).unwrap());
        assert_eq!(
            after, marker,
            "the stale index was rebuilt again by a later caller; with parallel workers those \
             rebuilds race and truncate the index the others are reading"
        );
    }

    /// A FAILED stale-index rebuild must neither destroy the existing index nor poison the
    /// per-path memo.
    ///
    /// `autoindex_file` reports failure quietly. It used to `File::create` the real index
    /// path first, truncating it before writing a byte - so a failure part-way left an empty
    /// or partial index behind. Combined with memoizing the path BEFORE knowing the rebuild
    /// worked, one failure would poison that path for the rest of the process: every later
    /// caller skipped the repair and reopened the wreckage. roborev 4369.
    ///
    /// Failure is induced by making the directory read-only, so the sibling temp index cannot
    /// be created.
    #[cfg(unix)]
    #[test]
    fn failed_stale_rebuild_preserves_the_index_and_retries() {
        use std::{io::Write, os::unix::fs::PermissionsExt};

        let dir = tempfile::tempdir().unwrap();
        let csv_path = dir.path().join("failing.csv");

        let mut f = std::fs::File::create(&csv_path).unwrap();
        writeln!(f, "letter,number").unwrap();
        for i in 0..2_000 {
            writeln!(f, "{},{i}", char::from(b'a' + (i % 26) as u8)).unwrap();
        }
        f.sync_all().unwrap();
        drop(f);

        let path_str = csv_path.to_string_lossy().to_string();
        let config = Config::new(Some(&path_str));
        crate::util::create_index_for_file(&csv_path, &config).unwrap();
        let idx_path = crate::util::idx_path(&csv_path);
        let good_len = std::fs::metadata(&idx_path).unwrap().len();
        assert!(good_len > 0, "fixture index should be non-empty");

        // make the index look stale, permanently
        let future = filetime::FileTime::from_unix_time(
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(&csv_path).unwrap())
                .unix_seconds()
                + 86_400,
            0,
        );
        filetime::set_file_mtime(&csv_path, future).unwrap();

        // read-only directory => the sibling temp index cannot be created => rebuild fails
        let original_perms = std::fs::metadata(dir.path()).unwrap().permissions();
        let mut readonly = original_perms.clone();
        readonly.set_mode(0o555);
        std::fs::set_permissions(dir.path(), readonly).unwrap();

        let during_failure = Config::new(Some(&path_str)).indexed();

        std::fs::set_permissions(dir.path(), original_perms).unwrap();

        // A stale index that could not be refreshed must NOT be handed back: its offsets no
        // longer describe the data, so seeking with it can read the wrong rows. "No index"
        // sends the caller down its sequential path, which is slower but correct.
        // (roborev 4370 - an earlier revision of this test asserted the opposite.)
        assert!(
            during_failure.is_ok_and(|i| i.is_none()),
            "a stale index that could not be rebuilt must be reported as absent, not returned"
        );

        // ...but the existing index file must still be INTACT on disk - the failed rebuild
        // never touched it, so a later successful rebuild is not starting from wreckage
        assert_eq!(
            std::fs::metadata(&idx_path).unwrap().len(),
            good_len,
            "a failed rebuild must not truncate the existing index"
        );

        // and the failure must NOT have been memoized - a later caller retries and succeeds
        let marker = filetime::FileTime::from_unix_time(1_000_000, 0);
        filetime::set_file_mtime(&idx_path, marker).unwrap();
        assert!(
            Config::new(Some(&path_str)).indexed().unwrap().is_some(),
            "the retry must produce a usable index"
        );
        let after =
            filetime::FileTime::from_last_modification_time(&std::fs::metadata(&idx_path).unwrap());
        assert_ne!(
            after, marker,
            "a failed rebuild poisoned the memo: the retry skipped the repair"
        );
    }
}
