use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    str,
    time::SystemTime,
};

use log::warn;

use super::{extract_all_zip_entries, get_envvar_flag};
use crate::{
    CliError, config,
    config::{SpecialFormat, get_special_format},
};

/// Validates if a file is actually a Snappy-compressed file before attempting decompression
pub fn is_valid_snappy_file(path: &PathBuf) -> Result<bool, CliError> {
    let mut file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(&mut file);

    // Try to create a FrameDecoder and read the first few bytes
    // This will fail immediately if the file doesn't have a valid Snappy header
    let decoder = snap::read::FrameDecoder::new(&mut reader);
    let mut buffer = Vec::with_capacity(50);

    match decoder.take(50).read_to_end(&mut buffer) {
        Ok(_) => {
            // Successfully read some bytes, this is likely a valid Snappy file
            log::debug!("File {} appears to be a valid Snappy file", path.display());
            Ok(true)
        },
        Err(e) => {
            // Failed to read, this is not a valid Snappy file
            log::debug!("File {} is not a valid Snappy file: {}", path.display(), e);
            Ok(false)
        },
    }
}

/// A new, empty file for one `decompress_snappy_file` call, in its own
/// exclusively created subdirectory of `tmpdir`, so it never overwrites another
/// input's scratch or final file. The name stays `qsv_temp_decompressed__<stem>`:
/// `sniff` recovers the original name from it, and callers that read the
/// extension (e.g. `joinp`) still see it.
fn snappy_scratch_file(tmpdir: &Path, file_stem: &str) -> std::io::Result<(File, PathBuf)> {
    let dir = tempfile::Builder::new()
        .prefix("qsv_snappy_")
        .tempdir_in(tmpdir)?
        .keep();
    let path = dir.join(format!("qsv_temp_decompressed__{file_stem}"));
    Ok((File::create(&path)?, path))
}

/// Decompresses a Snappy-compressed file to a temporary directory.
///
/// # Arguments
///
/// * `path` - Path to the file with `.sz` extension that should be decompressed
/// * `tmpdir` - Temporary directory where the decompressed file will be placed
///
/// # Returns
///
/// Returns the path to the decompressed file in the temporary directory as a `String`.
///
/// # Fallback Behavior
///
/// If a file has a `.sz` extension but is not actually a valid Snappy-compressed file,
/// this function falls back gracefully by copying the file to the temp directory as a
/// plain file (without the `.sz` extension). This prevents "corrupt input" errors when
/// plain CSV files are incorrectly detected as snappy-compressed (e.g., due to temp file
/// naming bugs or extension detection issues).
///
/// This fallback is necessary because this function is used by commands that require
/// file-based access (like `slice`, `lens`, `describegpt`, `tojsonl`, `joinp`, `sniff`)
/// which use `process_input()` to decompress files before creating a `Config`. Since
/// `process_input()` strips the `.sz` extension from the temp file path, `Config::io_reader()`
/// never sees the extension and cannot handle the fallback. Therefore, the validation and
/// fallback must happen here in `decompress_snappy_file()`.
///
/// Commands that use streaming (like `count`, `stats`, `frequency`) use `Config::io_reader()`
/// directly, which also has validation and fallback logic. Both code paths need their own
/// validation because they serve different purposes (file-based vs streaming access).
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be opened
/// - File validation fails (though this now falls back to plain file handling)
/// - The file cannot be copied to the temp directory (fallback case)
/// - Decompression fails for valid snappy files
pub fn decompress_snappy_file(
    path: &PathBuf,
    tmpdir: &tempfile::TempDir,
) -> Result<String, CliError> {
    // First, validate that this is actually a Snappy file
    if !is_valid_snappy_file(path)? {
        // File has .sz extension but is not a valid Snappy file.
        // Fall back gracefully by copying it to temp directory as a plain file.
        // This prevents "corrupt input" errors when plain CSV files are incorrectly
        // detected as snappy (e.g., due to temp file naming bugs).
        warn!(
            "File {} has .sz extension but is not a valid Snappy file. Treating as plain file.",
            path.display()
        );

        // Copy to a scratch file, not the final name: `process_input` renames it,
        // and `joinp` decompresses both inputs into one tmpdir.
        let file_stem = Path::new(&path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("plain_file");
        let (mut fallback_file, fallback_filepath) = snappy_scratch_file(tmpdir.path(), file_stem)?;
        std::io::copy(&mut File::open(path)?, &mut fallback_file)?;
        fallback_file.flush()?;
        // Windows: release the write handle before anything re-opens the path
        drop(fallback_file);
        return Ok(format!("{}", fallback_filepath.display()));
    }

    // Proceed with decompression since we've validated the file
    let mut snappy_file = std::fs::File::open(path.clone())?;
    let mut snappy_reader = snap::read::FrameDecoder::new(&mut snappy_file);
    // safety: we know that file_stem() will not be None as we opened the file above
    let file_stem = Path::new(&path).file_stem().unwrap().to_string_lossy();
    let (mut decompressed_file, decompressed_filepath) =
        snappy_scratch_file(tmpdir.path(), &file_stem)?;

    match std::io::copy(&mut snappy_reader, &mut decompressed_file) {
        Ok(num_bytes) => {
            decompressed_file.flush()?;
            // Explicitly close the write handle before returning. On Windows
            // the next reader to open this path can deadlock if a writer
            // handle is still alive in this process — observed as an
            // indefinite hang on Windows ARM64 (sniff URL → snappy → sniffer).
            drop(decompressed_file);
            log::debug!(
                "Successfully decompressed Snappy file: {} ({} bytes)",
                path.display(),
                num_bytes
            );
            Ok(format!("{}", decompressed_filepath.display()))
        },
        Err(e) => {
            // Clean up the partially created file
            let _ = std::fs::remove_file(&decompressed_filepath);
            fail_clierror!(
                "Failed to decompress Snappy file '{}': {}. The file may be corrupted or \
                 incomplete.",
                path.display(),
                e
            )
        },
    }
}

/// this is a non-allocating `to_lowercase` that uses an existing buffer
/// and should be faster than the allocating `std::to_lowercase`
#[inline]
pub fn to_lowercase_into(s: &str, buf: &mut String) {
    buf.clear();
    for c in s.chars() {
        for lc in c.to_lowercase() {
            buf.push(lc);
        }
    }
}

/// load the first BUFFER*8 (1024k) bytes of the file and check if it is utf8
pub fn isutf8_file(path: &Path) -> Result<bool, CliError> {
    let metadata = std::fs::metadata(path)?;
    let buffer_len = config::DEFAULT_RDR_BUFFER_CAPACITY * 8;
    let file_size = metadata.len() as usize;
    let bytes_to_read: usize = if file_size < buffer_len {
        file_size
    } else {
        buffer_len
    };

    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(bytes_to_read);
    reader.read_to_end(&mut buffer)?;

    Ok(simdutf8::basic::from_utf8(&buffer).is_ok())
}

// check if a file is supported by process_input
pub(super) fn is_supported_file(path: &Path) -> bool {
    // If QSV_SKIP_FORMAT_CHECK is set, consider all files as supported
    if get_envvar_flag("QSV_SKIP_FORMAT_CHECK") {
        return true;
    }

    let ext = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_lowercase)
        .unwrap_or_default();
    match ext.as_str() {
        "csv" | "ssv" | "tsv" | "tab" => true,
        _ => get_special_format(path) != SpecialFormat::Unknown,
    }
}

/// Where `process_input` should put an unpacked input named `name` (a file, or a
/// zip's extraction directory): `tmpdir/name` if it is free, else
/// `tmpdir/dup<n>/name` for the first free `n`. Keeping the first claimant at the
/// root leaves paths unchanged when nothing clashes; `validate --split-ragged`
/// derives its output names from the tmpdir-relative path.
pub(super) fn unique_unpack_path(
    tmpdir: &Path,
    name: &std::ffi::OsStr,
) -> std::io::Result<PathBuf> {
    let candidate = tmpdir.join(name);
    if !candidate.exists() {
        return Ok(candidate);
    }
    for n in 1_u32.. {
        let dir = tmpdir.join(format!("dup{n}"));
        // an unpacked input may itself be named `dup<n>`
        if dir.exists() && !dir.is_dir() {
            continue;
        }
        let candidate = dir.join(name);
        if !candidate.exists() {
            std::fs::create_dir_all(&dir)?;
            return Ok(candidate);
        }
    }
    unreachable!("ran out of dup<n> directory names")
}

/// Process the input files and return a vector of paths to the input files.
///
/// If the input is empty, try to copy stdin to a file named stdin in the passed temp directory.
/// If the input is empty and stdin is empty, return an error.
/// If it's not empty, check the input files if they exist, and return an error if they don't.
///
/// If the input is a directory, add all the files in the directory to the input.
/// If the input is a zip file, add all the files in the zip file to the input.
/// If the input is a file with the extension ".infile-list", read the file & add each line as a
/// file to the input.
/// If the input is a file, add the file to the input.
/// If the input are snappy compressed files, uncompress them before adding them to the input.
pub fn process_input(
    arg_input: Vec<PathBuf>,
    tmpdir: &tempfile::TempDir,
    custom_empty_stdin_errmsg: &str,
) -> Result<Vec<PathBuf>, CliError> {
    Ok(
        process_input_with_sources(arg_input, tmpdir, custom_empty_stdin_errmsg)?
            .into_iter()
            .map(|(path, _)| path)
            .collect(),
    )
}

/// Where a path returned by `process_input_with_sources` came from. Stdin and
/// unpacked inputs live in the temp dir, whose path means nothing to the user.
#[derive(Clone, Debug, PartialEq)]
pub enum InputSource {
    Stdin,
    /// A file read in place (including a resolved `dc:` disk-cache reference).
    File(PathBuf),
    /// A decompressed `.sz` file, as given.
    Snappy(PathBuf),
    /// An entry extracted from a zip archive: the archive as given, and the
    /// entry's path inside it.
    ZipEntry {
        archive: PathBuf,
        entry:   PathBuf,
    },
}

// only `cat` reads these, and qsvdp has no `cat`
#[cfg_attr(feature = "datapusher_plus", allow(dead_code))]
impl InputSource {
    /// The path the input would have if it were unpacked in place: a `.sz` file
    /// without its `.sz`, a zip entry inside a directory named for the archive.
    /// `None` for stdin.
    pub fn logical_path(&self) -> Option<PathBuf> {
        match self {
            Self::Stdin => None,
            Self::File(p) => Some(p.clone()),
            Self::Snappy(p) => Some(p.with_extension("")),
            Self::ZipEntry { archive, entry } => Some(archive.with_extension("").join(entry)),
        }
    }

    /// `logical_path`, anchored at the real location of the file on disk
    /// (symlinks and relative paths resolved). `None` for stdin.
    pub fn resolved_path(&self) -> std::io::Result<Option<PathBuf>> {
        Ok(match self {
            Self::Stdin => None,
            Self::File(p) => Some(p.canonicalize()?),
            Self::Snappy(p) => Some(p.canonicalize()?.with_extension("")),
            Self::ZipEntry { archive, entry } => {
                Some(archive.canonicalize()?.with_extension("").join(entry))
            },
        })
    }

    /// Where the input really is: the file itself (a `.sz` file keeps its `.sz`),
    /// or for a zip entry the archive joined with the entry, which names one
    /// source unambiguously where `resolved_path` may not (`jan/data.csv.sz`
    /// resolves to the same path as `jan/data.csv`). Symlinks and relative paths
    /// resolved; `None` for stdin.
    pub fn source_path(&self) -> std::io::Result<Option<PathBuf>> {
        Ok(match self {
            Self::Stdin => None,
            Self::File(p) | Self::Snappy(p) => Some(p.canonicalize()?),
            Self::ZipEntry { archive, entry } => Some(archive.canonicalize()?.join(entry)),
        })
    }

    /// How to name the input to the user: as given, a zip entry as `archive/entry`.
    pub fn display_name(&self) -> String {
        match self {
            Self::Stdin => "<stdin>".to_string(),
            Self::File(p) | Self::Snappy(p) => p.display().to_string(),
            Self::ZipEntry { archive, entry } => archive.join(entry).display().to_string(),
        }
    }
}

/// `process_input`, also returning where each path came from.
pub fn process_input_with_sources(
    arg_input: Vec<PathBuf>,
    tmpdir: &tempfile::TempDir,
    custom_empty_stdin_errmsg: &str,
) -> Result<Vec<(PathBuf, InputSource)>, CliError> {
    let mut processed_input = Vec::with_capacity(arg_input.len());

    let work_input = if arg_input.len() == 1 {
        let input_path = &arg_input[0];
        if input_path.is_dir() {
            // if the input is a directory, add all the supported files in the directory to the
            // input
            std::fs::read_dir(input_path)?
                .map(|entry| entry.map(|e| e.path()))
                .filter_map(|path| path.ok().filter(|p| is_supported_file(p)))
                .collect::<Vec<_>>()
        } else if input_path.is_file() {
            // if the input is a file and has the extension "infile-list" case-insensitive,
            // read the file. Each line is a file path
            if input_path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("infile-list"))
            {
                let mut input_file = std::fs::File::open(input_path)?;
                let mut input_file_contents = String::new();
                let mut canonical_invalid_path = PathBuf::new();
                let mut invalid_files = 0_u32;
                input_file.read_to_string(&mut input_file_contents)?;
                let infile_list_vec = input_file_contents
                    .lines()
                    .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
                    .map(PathBuf::from)
                    .filter_map(|path| {
                        if path.exists() {
                            Some(path)
                        } else {
                            // note that we're warn logging if files do not exist for
                            // each line in the infile-list file
                            // even though we're returning an error on the FIRST file that
                            // doesn't exist in the next section. This is because
                            // we want to log ALL the invalid file paths in the infile-list
                            // file, not just the first one.
                            invalid_files += 1;
                            canonical_invalid_path = path.canonicalize().unwrap_or_default();
                            log::warn!(
                                ".infile-list file '{}': '{}' does not exist",
                                path.display(),
                                canonical_invalid_path.display()
                            );
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                log::info!(
                    ".infile-list file parsed. Filecount - valid:{} invalid:{invalid_files}",
                    infile_list_vec.len()
                );
                infile_list_vec
            } else {
                // if the input is not an ".infile-list" file, add the file to the input
                arg_input
            }
        } else {
            arg_input
        }
    } else {
        arg_input
    };

    // Reserve `tmpdir/stdin.csv` before unpacking anything else, so stdin always
    // owns that path whatever the argument order (`cat` relies on it to label
    // stdin) and a decompressed `stdin.csv.sz` gets a `dup<n>/` path instead.
    // stdin itself is still read only when its turn comes.
    let stdin_path = tmpdir.path().join("stdin.csv");
    if work_input.iter().any(|p| p == "-") {
        std::fs::File::create(&stdin_path)?;
    }
    let mut stdin_copied = false;

    // check the input files
    for path in work_input {
        // check if the path is "-" (stdin)
        if &path == "-" {
            if !stdin_copied {
                let mut tmp_file = std::fs::File::create(&stdin_path)?;
                std::io::copy(&mut std::io::stdin(), &mut tmp_file)?;
                tmp_file.flush()?;
                stdin_copied = true;
            }
            processed_input.push((stdin_path.clone(), InputSource::Stdin));
            continue;
        }

        // Resolve a `dc:<name>` disk-cache reference (the `get` command's cache)
        // to its materialized CSV path BEFORE the existence check below. Without
        // this, the literal "dc:…" string fails `path.exists()`, so every command
        // that routes inputs through `process_input` (cat, slice, join, …) wrongly
        // rejected a valid `dc:` handle even though `Config::new` resolves it. The
        // resolved temp CSV ships a sibling .idx, so indexed commands work too.
        #[cfg(feature = "get")]
        if let Some(dc_name) = path.to_str().and_then(|s| s.strip_prefix("dc:")) {
            let resolved = crate::diskcache::resolve_dc_path(dc_name)?;
            processed_input.push((resolved.clone(), InputSource::File(resolved)));
            continue;
        }

        if !path.exists() {
            return fail_clierror!("Input file '{}' does not exist", path.display());
        }

        // is the input file snappy compressed?
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("sz"))
        {
            // if so, decompress the file
            let decompressed_filepath = decompress_snappy_file(&path, tmpdir)?;

            // rename the decompressed file to the original filename, but still
            // inside the temp directory. this is so that the decompressed file can be
            // processed as if it was the original file without the "sz" extension
            let original_filepath = path.with_extension("");
            // safety: we know the path has a filename
            let original_filename = original_filepath.file_name().unwrap();

            let final_decompressed_filepath = unique_unpack_path(tmpdir.path(), original_filename)?;
            std::fs::rename(&decompressed_filepath, &final_decompressed_filepath)?;

            processed_input.push((final_decompressed_filepath, InputSource::Snappy(path)));
        }
        // is the input file a zip archive?
        else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("zip"))
        {
            // Extract its usable entries (tabular-first) via the shared zip module
            // so this command-level "all entries" path and the reader-level
            // `Config`/`select_zip_entry` "first tabular entry" path agree on what
            // a zip's entries are. Issue #3988.
            log::info!("Extracting files from zip archive: {}", path.display());
            let (extract_dir, entries) = extract_all_zip_entries(&path, tmpdir)?;
            log::info!(
                "Extracted {} usable entries from zip archive {}",
                entries.len(),
                path.display()
            );
            processed_input.extend(entries.into_iter().map(|extracted| {
                // safety: every entry was extracted under `extract_dir`
                let entry = extracted.strip_prefix(&extract_dir).unwrap().to_path_buf();
                let source = InputSource::ZipEntry {
                    archive: path.clone(),
                    entry,
                };
                (extracted, source)
            }));
        } else {
            processed_input.push((path.clone(), InputSource::File(path)));
        }
    }

    if processed_input.is_empty() {
        if custom_empty_stdin_errmsg.is_empty() {
            return fail_clierror!(
                "No data on stdin. Please provide at least one input file or pipe data to stdin."
            );
        }
        return fail_clierror!("{custom_empty_stdin_errmsg}");
    }
    log::debug!("processed input file/s: {processed_input:?}");
    Ok(processed_input)
}

#[inline]
pub fn replace_column_value(
    record: &csv::StringRecord,
    column_index: usize,
    new_value: &str,
) -> csv::StringRecord {
    record
        .into_iter()
        .enumerate()
        .map(|(i, v)| if i == column_index { new_value } else { v })
        .collect()
}

/// format a `SystemTime` from a file's metadata to a string using the format specifier
#[inline]
pub fn format_systemtime(time: SystemTime, format_specifier: &str) -> String {
    // Default to UNIX_EPOCH for pre-1970 timestamps. They do occur in the wild —
    // Windows files can carry the FILETIME epoch (1601-01-01), and archived /
    // restored files or corrupted FS metadata sometimes report pre-1970 mtimes.
    let timestamp = time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_default();
    format!("{datetime}", datetime = datetime.format(format_specifier))
}
