use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    str,
};
#[cfg(feature = "polars")]
use std::{io::BufReader, io::BufWriter, sync::Arc};

#[cfg(feature = "polars")]
use polars::prelude::Schema;
use zip::read::root_dir_common_filter;

#[cfg(feature = "polars")]
use super::{SchemaArgs, StatsMode, current_exe, get_envvar_flag, get_stats_records, njobs};
use super::{ZIP_TABULAR_EXTS, is_supported_file, unique_unpack_path};
use crate::{CliError, CliResult, config::SpecialFormat};
#[cfg(feature = "polars")]
use crate::{cmd::stats::StatsData, config::Config};

/// Expand the tilde (`~`) from within the provided path.
// consumed by non-lite commands (describegpt, fetchpost, geocode) and the
// `get`/diskcache subsystem (none of which are in qsvlite), plus the throttled
// self-update check - which can be enabled alongside `lite`, hence the extra gate.
#[cfg(any(not(feature = "lite"), feature = "self_update"))]
pub fn expand_tilde(path: impl AsRef<Path>) -> Option<PathBuf> {
    let p = path.as_ref();

    let expanded = if p.starts_with("~") {
        let mut base = directories::BaseDirs::new()?.home_dir().to_path_buf();

        if !p.ends_with("~") {
            base.extend(p.components().skip(1));
        }
        base
    } else {
        p.to_path_buf()
    };
    Some(expanded)
}

/// Loads a Polars schema from a pschema.json file if it exists.
///
/// # Arguments
///
/// * `path` - The path to the input file
///
/// # Returns
///
/// * `Option<Arc<Schema>>` - The loaded schema if the file exists and can be parsed, None otherwise
#[cfg(feature = "polars")]
fn load_schema_from_file(path: &Path) -> Result<Option<Arc<Schema>>, Box<dyn std::error::Error>> {
    // Append .pschema.json to the full filename (including extension)
    // e.g. data.csv -> data.csv.pschema.json, data.tsv.gz -> data.tsv.gz.pschema.json
    let schema_file = PathBuf::from(format!("{}.pschema.json", path.display()));

    if schema_file.exists() {
        // Load the schema from the pschema.json file
        let file = File::open(&schema_file)?;
        let mut buf_reader = BufReader::new(file);
        let mut schema_json = String::with_capacity(100);
        buf_reader.read_to_string(&mut schema_json)?;
        let schema: Schema = serde_json::from_str(&schema_json)?;
        Ok(Some(Arc::new(schema)))
    } else {
        Ok(None)
    }
}

/// Choose which entry of a zip archive to treat as the tabular payload.
///
/// Returns the chosen entry's 0-based index and its lower-cased tabular extension
/// (`csv`/`tsv`/`tab`/`ssv`). Selection rule:
/// 1. Skip directory entries and system files (e.g. `__MACOSX`, `.DS_Store`) via
///    `root_dir_common_filter`.
/// 2. Use the first entry (in archive order) whose extension is tabular.
/// 3. Error if no tabular entry exists — including the single-entry case. We do NOT silently treat
///    an arbitrary file as CSV, matching the documented "first CSV/TSV/TAB/SSV entry" behavior.
///
/// By design, a `.zip` is treated as a container of delimited text. A **special
/// binary format** (Parquet/Avro/Arrow) nested inside a `.zip` is intentionally
/// NOT selectable here — those formats are already compressed, so zipping them is
/// not an expected workflow; provide such files directly instead. (Issue #3988 —
/// "nested special formats": won't-support, documented in README.)
///
/// Generic over `Read + Seek` so it serves both file-backed archives
/// (`extract_zip_to_temp`) and in-memory `Cursor`-backed ones (`diskcache`).
pub fn select_zip_entry<R: Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
) -> CliResult<(usize, Option<String>)> {
    let mut usable_count = 0_usize;
    let mut first_tabular: Option<(usize, String)> = None;

    for i in 0..archive.len() {
        // Inspect the central-directory name only (no decoder is constructed), so
        // an encrypted or unsupported-compression entry elsewhere in the archive
        // can't fail selection of a valid earlier CSV/TSV/TAB/SSV entry. The chosen
        // entry is decoded later (by `extract_zip_to_temp` via `by_index`).
        let entry = archive.by_index_raw(i)?;
        let name = entry.name()?.to_string();
        if name.ends_with('/') || !root_dir_common_filter(Path::new(&name)) {
            continue;
        }
        usable_count += 1;
        if first_tabular.is_none()
            && let Some(ext) = Path::new(&name)
                .extension()
                .and_then(|e| e.to_str())
                .map(str::to_ascii_lowercase)
                .filter(|e| ZIP_TABULAR_EXTS.contains(&e.as_str()))
        {
            first_tabular = Some((i, ext));
        }
    }

    if let Some((idx, ext)) = first_tabular {
        if usable_count > 1 {
            log::info!(
                "zip archive has multiple entries; using first CSV/TSV/TAB/SSV entry (index {idx})"
            );
        }
        return Ok((idx, Some(ext)));
    }
    Err(CliError::IncorrectUsage(if usable_count == 0 {
        "zip archive contains no usable file entry.".to_string()
    } else {
        "zip archive contains no CSV/TSV/TAB/SSV entry.".to_string()
    }))
}

/// Map a chosen zip entry's tabular extension to a temp-file suffix that qsv's
/// delimiter inference (`get_delim_by_extension`) understands.
fn zip_inner_ext_to_suffix(inner_ext: Option<&str>) -> &'static str {
    match inner_ext {
        Some("tsv" | "tab") => ".tsv",
        Some("ssv") => ".ssv",
        _ => ".csv",
    }
}

/// Extract the tabular entry of a zip archive to a temp file whose extension
/// selects the delimiter downstream (via `get_delim_by_extension`). The temp file
/// lives in `TEMP_FILE_DIR` and is `keep()`-ed (process-lifetime), matching
/// `convert_special_format`'s temp lifecycle.
///
/// Always compiled (needs only the non-optional `zip` crate), so it is usable in
/// non-polars builds and from `convert_special_format`'s non-polars stub.
pub fn extract_zip_to_temp(
    path: &Path,
    _default_delim: u8,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let zip_file = File::open(path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    let (idx, inner_ext) = select_zip_entry(&mut archive).map_err(|e| e.to_string())?;
    let suffix = zip_inner_ext_to_suffix(inner_ext.as_deref());

    let temp_dir =
        crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());
    let mut temp_file = tempfile::Builder::new()
        .suffix(suffix)
        .tempfile_in(temp_dir)?;

    let mut entry = archive.by_index(idx)?;
    std::io::copy(&mut entry, &mut temp_file)?;
    temp_file.flush()?;

    let out = temp_file.path().to_path_buf();
    temp_file.keep()?;
    Ok(out)
}

/// Extract a zip archive's usable entries into a temp subdirectory and return
/// that subdirectory and the entries' paths, **tabular entries first** (CSV/TSV/TAB/SSV in archive
/// order), followed by other supported entries (special formats parquet/avro/json/…, in
/// archive order — or *any* file when `QSV_SKIP_FORMAT_CHECK` is set, via
/// `is_supported_file`). Directories, system files (`__MACOSX`, `.DS_Store`, …),
/// zip-slip entries, and unsupported file types are skipped. Errors with a clear
/// message if no usable entry remains.
///
/// This is the command-level ("extract all entries") counterpart to the
/// reader-level `select_zip_entry`/`extract_zip_to_temp` (which lazily extracts
/// only the first tabular entry). Both share the same selection rule — the
/// tabular extensions in `ZIP_TABULAR_EXTS`, filtered by `root_dir_common_filter`
/// — so a single-input command that consumes the first returned path selects the
/// SAME entry the `Config` reader path would, while multi-input commands (`cat`,
/// `sqlp`, …) still receive every entry. Issue #3988.
pub(super) fn extract_all_zip_entries(
    path: &Path,
    tmpdir: &tempfile::TempDir,
) -> Result<(PathBuf, Vec<PathBuf>), CliError> {
    // A per-archive subdirectory keeps each entry's relative path (zips may carry
    // nested directories); `unique_unpack_path` keeps it apart from other inputs
    // unpacked under the same name (e.g. `a/data.zip` and `b/data.zip`).
    let zip_stem = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map_or_else(
            || "zip".to_string(),
            |n| {
                n.strip_suffix(".zip")
                    .or_else(|| n.strip_suffix(".ZIP"))
                    .unwrap_or(n)
                    .to_string()
            },
        );
    let zip_extract_dir = unique_unpack_path(tmpdir.path(), zip_stem.as_ref())?;
    std::fs::create_dir_all(&zip_extract_dir)?;

    let mut archive = zip::ZipArchive::new(File::open(path)?)?;

    // `get_special_format` classifies by inspecting the on-disk file, so each
    // entry must be extracted before it can be classified. Collect tabular and
    // other-supported entries separately to preserve the tabular-first ordering.
    let mut tabular = Vec::new();
    let mut other_supported = Vec::new();
    for i in 0..archive.len() {
        let mut zip_entry = archive.by_index(i)?;
        let entry_name = zip_entry.name()?.to_string();

        if entry_name.ends_with('/') || !root_dir_common_filter(Path::new(&entry_name)) {
            log::info!("  Skipping system file or directory: {entry_name}");
            continue;
        }

        // Reject path-traversal entries (zip-slip): `enclosed_name()` returns None
        // when the entry name would escape the extraction root.
        let Some(safe_name) = zip_entry.enclosed_name() else {
            log::warn!("  Skipping zip entry with unsafe path (zip-slip): {entry_name}");
            continue;
        };

        let file_path = zip_extract_dir.join(safe_name);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut outfile = File::create(&file_path)?;
        std::io::copy(&mut zip_entry, &mut outfile)?;
        outfile.flush()?;
        // Release the write handle before classifying (Windows: a reader can't
        // re-open the path while a write handle is live).
        drop(outfile);

        let ext = file_path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();
        if ZIP_TABULAR_EXTS.contains(&ext.as_str()) {
            tabular.push(file_path);
        } else if is_supported_file(&file_path) {
            // Non-tabular but supported: a special format (parquet/avro/json/…),
            // or — when `QSV_SKIP_FORMAT_CHECK` is set — any file (the global
            // escape hatch, honored via `is_supported_file`).
            other_supported.push(file_path);
        } else {
            log::info!("  Skipping unsupported file type: {}", file_path.display());
        }
    }

    if tabular.is_empty() && other_supported.is_empty() {
        return fail_clierror!(
            "zip archive '{}' contains no supported (CSV/TSV/TAB/SSV or special-format) file \
             entry.",
            path.display()
        );
    }

    // Tabular entries first, so a single-input consumer's first element matches
    // the reader-level `select_zip_entry` choice (the first CSV/TSV/TAB/SSV entry).
    tabular.extend(other_supported);
    Ok((zip_extract_dir, tabular))
}

/// Converts files in special formats (Parquet, Avro, Arrow IPC, JSONL, JSON, or compressed CSV)
/// into a standard delimited text file. The output file extension will be:
/// - .tsv for tab-delimited
/// - .ssv for semicolon-delimited
/// - .csv for comma-delimited
///
/// # Arguments
///
/// * `path` - The path to the input file.
/// * `format` - The format of the input file.
/// * `delim` - The delimiter to use for the output CSV file.
///
/// # Returns
///
/// A `Result` containing the path to the temporary CSV file.
/// The caller is responsible for deleting the temporary file.
#[cfg(feature = "polars")]
pub fn convert_special_format(
    path: &Path,
    format: SpecialFormat,
    delim: u8,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    use polars::{
        io::avro::AvroReader,
        prelude::{
            CsvParseOptions, CsvReadOptions, CsvWriter, IpcReader, JsonReader, LazyFileListReader,
            LazyJsonLineReader, ParquetReader, PlRefPath, SerReader, SerWriter,
        },
    };

    // Zip is not a polars-native format; extract the chosen tabular entry to a
    // temp file (always-compiled path) and return it directly.
    if format == SpecialFormat::CompressedZip {
        return extract_zip_to_temp(path, delim);
    }

    // Check if there's a pschema.json file with the same filestem
    // the Polars schema will be used in parsing
    // JSON/JSONL and compressed CSV files only
    let schema = if let SpecialFormat::Avro | SpecialFormat::Parquet | SpecialFormat::Ipc = format {
        None
    } else {
        load_schema_from_file(path)?
    };

    let mut extension = ".csv";
    // Create a reader based on the file format and convert to DataFrame
    let mut df = match format {
        SpecialFormat::Avro => AvroReader::new(BufReader::new(File::open(path)?)).finish()?,
        SpecialFormat::Parquet => ParquetReader::new(BufReader::new(File::open(path)?)).finish()?,
        SpecialFormat::Ipc => IpcReader::new(BufReader::new(File::open(path)?)).finish()?,
        SpecialFormat::Jsonl => {
            let path_str = path.to_string_lossy();
            let lf = LazyJsonLineReader::new(PlRefPath::new(&*path_str));
            if let Some(schema) = schema {
                lf.with_schema(Some(schema)).finish()?
            } else {
                lf.finish()?
            }
            .collect()?
        },
        SpecialFormat::Json => {
            let df = JsonReader::new(BufReader::new(File::open(path)?));
            if let Some(schema) = schema {
                df.with_schema(schema).finish()?
            } else {
                df.finish()?
            }
        },
        SpecialFormat::CompressedCsv
        | SpecialFormat::CompressedTsv
        | SpecialFormat::CompressedSsv => {
            let separator = match format {
                SpecialFormat::CompressedTsv => {
                    extension = ".tsv";
                    b'\t'
                },
                SpecialFormat::CompressedSsv => {
                    extension = ".ssv";
                    b';'
                },
                _ => delim,
            };

            // Create base CSV read options with the appropriate separator
            let base_options = CsvReadOptions::default()
                .with_parse_options(CsvParseOptions::default().with_separator(separator));

            // Try reading the compressed file with a schema if available
            let reader = CsvReadOptions::default()
                .try_into_reader_with_file_path(Some(path.to_path_buf()))?
                .with_options(if let Some(schema) = schema {
                    base_options.clone().with_schema(Some(schema))
                } else {
                    // it failed, try to infer it with 1,000 rows
                    base_options.clone().with_infer_schema_length(Some(1_000))
                });

            if let Ok(df) = reader.finish() {
                df
            } else {
                // Got an error. Try again with a larger infer schema length of 10,000 rows
                log::warn!(
                    "Falling back to reading file \"{}\" without a schema. 2nd try using infer \
                     schema length of 10,000 rows.",
                    path.display()
                );

                let reader_2ndtry = CsvReadOptions::default()
                    .try_into_reader_with_file_path(Some(path.to_path_buf()))?
                    .with_options(base_options.clone().with_infer_schema_length(Some(10_000)));

                if let Ok(df) = reader_2ndtry.finish() {
                    df
                } else {
                    log::warn!("Still failing. 3rd try - scanning the whole file to infer schema.");

                    // Try one last time without an infer schema length, scanning the whole file
                    let reader_3rdtry = CsvReadOptions::default()
                        .try_into_reader_with_file_path(Some(path.to_path_buf()))?
                        .with_options(base_options.with_infer_schema_length(None));

                    reader_3rdtry.finish()?
                }
            }
        },
        SpecialFormat::Unknown => return Err("Unknown format".into()),
        // handled by the early return at the top of this function
        SpecialFormat::CompressedZip => unreachable!(),
    };

    // Get or initialize temp directory that persists until program exit
    // safety: we know that the tempfile::TempDir::new() will not ordinarily fail
    // otherwise, we have a bigger problem
    let temp_dir =
        crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());

    // Create temp file with appropriate extension
    let mut temp_file = tempfile::Builder::new()
        .suffix(extension)
        .tempfile_in(temp_dir)?;

    // Get QSV_POLARS_FORMAT_FLOAT_PRECISION env var
    let precision = crate::config::POLARS_FLOAT_PRECISION.get_or_init(|| {
        std::env::var("QSV_POLARS_FLOAT_PRECISION")
            .ok()
            .and_then(|s| s.parse().ok())
    });

    // Write DataFrame to CSV with specified delimiter/separator
    CsvWriter::new(BufWriter::new(&temp_file))
        .with_separator(delim)
        .with_float_precision(*precision)
        .finish(&mut df)?;
    temp_file.flush()?;

    let path = temp_file.path().to_path_buf();
    temp_file.keep()?; // Prevent auto-deletion

    Ok(path)
}

#[cfg(not(feature = "polars"))]
#[allow(unused_variables)]
pub fn convert_special_format(
    path: &Path,
    format: SpecialFormat,
    delim: u8,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Zip extraction needs only the always-compiled `zip` crate, so it works
    // even in non-polars builds.
    if format == SpecialFormat::CompressedZip {
        return extract_zip_to_temp(path, delim);
    }
    Err(
        "This file type cannot be opened with your current version of qsv. You need the full, \
         polars-enabled version to work with Avro, Arrow, Parquet, JSON/JSONL and gzip/zlib/zst \
         compressed files. Please download the full version from the qsv website."
            .into(),
    )
}

#[cfg(feature = "polars")]
pub fn infer_polars_schema(
    delimiter: Option<crate::config::Delimiter>,
    debuglog_flag: bool,
    table: &Path,
    schema_file: &std::path::PathBuf,
    prefer_dmy: bool,
) -> Result<bool, crate::clitypes::CliError> {
    // Run sniff to auto-detect date/datetime columns
    let qsv_bin = current_exe()?;
    let mut sniff_cmd = std::process::Command::new(&qsv_bin);
    let table_str = table.to_string_lossy().to_string();
    // without an explicit delimiter, sniff guesses one, and on a single-column file of
    // dates it picks '/' and loses the date columns. Use the delimiter qsv itself would
    // read the table with (extension, then QSV_DEFAULT_DELIMITER).
    let delimiter = delimiter.or_else(|| {
        Some(crate::config::Delimiter(
            Config::new(Some(&table_str)).get_delimiter(),
        ))
    });
    sniff_cmd.args(["sniff", "--json"]);
    if let Some(d) = delimiter {
        let delim_byte = d.as_byte();
        if delim_byte == b'\t' {
            sniff_cmd.args(["--delimiter", r"\t"]);
        } else {
            // safety: delimiter is guaranteed to be a valid ASCII byte
            sniff_cmd.args(["--delimiter", &(delim_byte as char).to_string()]);
        }
    }
    sniff_cmd.arg(&table_str);
    if prefer_dmy {
        sniff_cmd.arg("--prefer-dmy");
    }
    let sniff_output = sniff_cmd.output()?;

    let mut sniff_dates_whitelist = String::new();
    if sniff_output.status.success() {
        match serde_json::from_slice::<serde_json::Value>(&sniff_output.stdout) {
            Ok(sniff_json) => {
                if let (Some(fields), Some(types)) = (
                    sniff_json["fields"].as_array(),
                    sniff_json["types"].as_array(),
                ) {
                    let date_fields: Vec<&str> = fields
                        .iter()
                        .zip(types.iter())
                        .filter_map(|(field, typ)| {
                            let type_str = typ.as_str().unwrap_or_default();
                            if type_str == "Date" || type_str == "DateTime" {
                                field.as_str()
                            } else {
                                None
                            }
                        })
                        .collect();
                    sniff_dates_whitelist = date_fields.join(",");
                    if debuglog_flag {
                        if sniff_dates_whitelist.is_empty() {
                            log::debug!(
                                "sniff did not detect any date/datetime columns for {table_str}"
                            );
                        } else {
                            log::debug!(
                                "sniff detected date/datetime columns for {table_str}: \
                                 {sniff_dates_whitelist}"
                            );
                        }
                    }
                } else if debuglog_flag {
                    log::debug!(
                        "sniff JSON for {table_str} did not contain expected 'fields'/'types' \
                         arrays"
                    );
                }
            },
            Err(err) => {
                if debuglog_flag {
                    log::debug!("failed to parse sniff JSON output for {table_str}: {err}");
                }
            },
        }
    } else if debuglog_flag {
        log::debug!(
            "sniff command failed for {table_str} with status {:?}. stderr: {}",
            sniff_output.status,
            String::from_utf8_lossy(&sniff_output.stderr)
        );
    }

    let schema_args = SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        // we still get all the stats columns so we can use the stats cache
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: sniff_dates_whitelist,
        flag_prefer_dmy:      prefer_dmy,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            Some(njobs(None)),
        flag_polars:          false,
        flag_no_headers:      false,
        flag_delimiter:       delimiter,
        arg_input:            Some(table.to_string_lossy().into_owned()),
        flag_memcheck:        false,
        flag_output:          None,
    };
    let (csv_fields, csv_stats) = get_stats_records(&schema_args, StatsMode::PolarsSchema)?;
    let mut schema = polars::prelude::Schema::with_capacity(csv_stats.len());

    // stats min/max of date columns are normalised to RFC3339, so whether polars
    // can read a column's dates is checked against its raw values instead
    let temporal_samples = raw_temporal_samples(&table_str, delimiter, &csv_stats)?;
    // the stats engine folds the env var in too
    let dmy = prefer_dmy || get_envvar_flag("QSV_PREFER_DMY");

    // fetch the decimal scale from the QSV_POLARS_DECIMAL_SCALE env var
    let scale = std::env::var("QSV_POLARS_DECIMAL_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5); // default scale is 5
    for (idx, stat) in csv_stats.iter().enumerate() {
        // safety: we know that the get(idx) will not be None as we are using an iterator
        schema.insert(
            polars::prelude::PlSmallStr::from_str(
                simdutf8::basic::from_utf8(csv_fields.get(idx).unwrap()).unwrap(),
            ),
            {
                let datatype = &stat.r#type;
                #[allow(clippy::match_same_arms)]
                match datatype.as_str() {
                    "String" => polars::datatypes::DataType::String,
                    "Integer" => {
                        // safety: integer types are guaranteed to have a min and max
                        let min = stat.min.as_ref().unwrap();
                        let max = stat.max.as_ref().unwrap();

                        // Check if all values are non-negative to
                        // use unsigned types
                        if let (Ok(min_val), Ok(max_val)) = (min.parse::<i64>(), max.parse::<i64>())
                        {
                            if min_val >= 0 {
                                // Use smallest unsigned type that can hold
                                // the max value
                                if max_val <= u8::MAX as i64 {
                                    polars::datatypes::DataType::UInt8
                                } else if max_val <= u16::MAX as i64 {
                                    polars::datatypes::DataType::UInt16
                                } else if max_val <= u32::MAX as i64 {
                                    polars::datatypes::DataType::UInt32
                                } else {
                                    polars::datatypes::DataType::UInt64
                                }
                            } else {
                                // Use signed types for negative values
                                if min_val >= i32::MIN as i64 && max_val <= i32::MAX as i64 {
                                    polars::datatypes::DataType::Int32
                                } else {
                                    polars::datatypes::DataType::Int64
                                }
                            }
                        } else {
                            // Fallback to Int64 if parsing fails
                            polars::datatypes::DataType::Int64
                        }
                    },
                    "Float" => {
                        let precision = stat.max_precision.unwrap_or(0);

                        // As we use f64 internally, its unlikely that we have more
                        // than 16 digits of precision, but we do this anyway to
                        // document it as the polars engine does support it
                        if precision > 16 {
                            // For very high precision, use Decimal type
                            polars::datatypes::DataType::Decimal(precision as usize, scale)
                        } else {
                            // Never narrow to Float32: it holds only ~7 significant digits
                            // (max_precision counts decimal places, not significant digits)
                            // and `str::parse::<f32>` returns Ok(inf) for out-of-range values,
                            // so a cached schema would silently change query results.
                            polars::datatypes::DataType::Float64
                        }
                    },
                    "Boolean" => polars::datatypes::DataType::Boolean,
                    "Date" | "DateTime" => {
                        let samples = temporal_samples.get(&idx).map_or(&[][..], Vec::as_slice);
                        polars_temporal_dtype(samples, dmy).unwrap_or_else(|| {
                            if debuglog_flag {
                                log::debug!(
                                    "column {:?} typed String in the Polars schema: polars can't \
                                     read its dates as qsv does (first value: {:?})",
                                    stat.field,
                                    samples.first()
                                );
                            }
                            polars::datatypes::DataType::String
                        })
                    },
                    _ => polars::datatypes::DataType::String,
                }
            },
        );
    }
    let stats_schema = std::sync::Arc::new(schema);
    // Use serde_json for schema serialization as the schema may contain compound types
    // (e.g. Datetime) that simd_json::to_string_pretty doesn't serialize correctly
    let stats_schema_json = serde_json::to_string_pretty(&stats_schema)?;
    let mut file = std::io::BufWriter::new(File::create(schema_file)?);
    file.write_all(stats_schema_json.as_bytes())?;
    file.flush()?;
    if debuglog_flag {
        log::debug!("Saved stats_schema to file: {}", schema_file.display());
    }
    Ok(true)
}

/// Raw values of the columns stats typed Date/DateTime, keyed by column index.
///
/// Only a leading sample is read - up to `SAMPLE_MAX` non-empty values per column
/// within the first `ROW_MAX` rows - as a full pass would double the cost of a
/// schema stats already took a pass over. A later row in a format polars can't
/// read still fails the read.
#[cfg(feature = "polars")]
fn raw_temporal_samples(
    table: &String,
    delimiter: Option<crate::config::Delimiter>,
    stats: &[StatsData],
) -> CliResult<std::collections::HashMap<usize, Vec<String>>> {
    const SAMPLE_MAX: usize = 1_000;
    const ROW_MAX: usize = 10_000;

    let mut samples: std::collections::HashMap<usize, Vec<String>> = stats
        .iter()
        .enumerate()
        .filter(|(_, s)| matches!(s.r#type.as_str(), "Date" | "DateTime"))
        .map(|(i, _)| (i, Vec::new()))
        .collect();
    if samples.is_empty() {
        return Ok(samples);
    }
    let mut rdr = Config::new(Some(table)).delimiter(delimiter).reader()?;
    let mut record = csv::ByteRecord::new();
    let mut rows = 0;
    // a row the reader can't read ends the sample early rather than failing the
    // schema (a column with no values sampled is typed String)
    while rows < ROW_MAX && rdr.read_byte_record(&mut record).unwrap_or(false) {
        rows += 1;
        let mut all_full = true;
        for (&i, values) in &mut samples {
            if values.len() < SAMPLE_MAX {
                // untrimmed, as the polars CSV reader parses it
                if let Some(v) = record.get(i).filter(|v| !v.is_empty()) {
                    values.push(String::from_utf8_lossy(v).into_owned());
                }
                all_full &= values.len() >= SAMPLE_MAX;
            }
        }
        if all_full {
            break;
        }
    }
    Ok(samples)
}

/// The Polars type a column of dates/datetimes can be read as, if the Polars CSV
/// reader can parse its values the way qsv does.
///
/// Mirrors `writestat::recognise`: the first value picks the pattern, and every
/// value must then parse with it. Day-first patterns are only accepted with
/// `prefer_dmy`, as Polars reads `04/05/2019` as 4 May while qsv reads 5 April.
#[cfg(feature = "polars")]
fn polars_temporal_dtype(
    samples: &[String],
    prefer_dmy: bool,
) -> Option<polars::prelude::DataType> {
    use polars::{
        chunked_array::temporal::string::{
            StringMethods, infer::infer_pattern_single, patterns::Pattern,
        },
        prelude::{DataType, StringChunked, TimeUnit},
    };

    let first = samples.first()?;
    let strings = StringChunked::from_iter(samples.iter().map(|s| Some(s.as_str())));
    let ambiguous = StringChunked::from_iter([Some("raise")]);
    let tu = TimeUnit::Milliseconds;
    let (dtype, nulls) = match infer_pattern_single(first)? {
        Pattern::DateDMY if !prefer_dmy => return None,
        Pattern::DatetimeDMY if !prefer_dmy => return None,
        Pattern::DateYMD | Pattern::DateDMY => (
            DataType::Date,
            strings.as_date(None, true).ok()?.null_count(),
        ),
        Pattern::DatetimeYMD | Pattern::DatetimeDMY => (
            DataType::Datetime(tu, None),
            strings
                .as_datetime(None, tu, true, false, None, &ambiguous)
                .ok()?
                .null_count(),
        ),
        Pattern::DatetimeYMDZ => (
            DataType::Datetime(tu, None),
            strings
                .as_datetime(
                    None,
                    tu,
                    true,
                    true,
                    Some(&polars::prelude::TimeZone::UTC),
                    &ambiguous,
                )
                .ok()?
                .null_count(),
        ),
        Pattern::Time => return None,
    };
    (nulls == 0).then_some(dtype)
}
