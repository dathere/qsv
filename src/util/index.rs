use std::{
    fs,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use csv_index::RandomAccessSimple;

use crate::{
    CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY},
};

pub fn idx_path(csv_path: &Path) -> PathBuf {
    // safety: we know the path has a filename
    let mut p = csv_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap();
    p.push_str(".idx");
    PathBuf::from(&p)
}

/// Builds a CSV index for `rdr` into a unique temp file beside `dest`, then renames it into
/// place - so `dest` is only ever replaced by a COMPLETE index.
///
/// Writing straight to `dest` means `fs::File::create` TRUNCATES it before a single byte of
/// the new index is written, and a `BufWriter` dropped on the error path flushes whatever it
/// had buffered to that same destination. So any mid-build failure both destroys a
/// pre-existing good index and leaves a partial one in its place.
///
/// That partial index is not detectably broken to a reader: `RandomAccessSimple::open` reads
/// only the trailing 8 bytes and trusts them as the record count, so in a truncated index it
/// returns a byte OFFSET as the count and every downstream command reports a wrong row count
/// at exit 0 (issue #4615). `Config::index_files` rejects such an index defensively, but the
/// real repair is to never produce one.
///
/// The temp is created with `OpenOptions::create_new` (`O_CREAT`|`O_EXCL`) rather than through
/// `tempfile`: it applies the process umask, whereas `tempfile` creates 0600 and would carry
/// that onto the `.idx` through the rename, silently turning indexes into owner-only files
/// (roborev 4371). Uniqueness needs no randomness - `create_new` fails with `AlreadyExists`,
/// so the loop advances the counter until it wins. pid separates processes, the counter
/// separates threads and repeat calls within one.
pub fn write_index_atomically(rdr: &mut csv::Reader<fs::File>, dest: &Path) -> CliResult<()> {
    // The temp MUST be a sibling of the DESTINATION, not of the input: `qsv index --output`
    // can point anywhere, and `fs::rename` across filesystems fails with EXDEV.
    let idx_dir = dest.parent().unwrap_or_else(|| Path::new("."));

    static TMP_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let pid = std::process::id();
    let mut tmp_path = None;
    let mut tmp_file = None;
    for _ in 0..16 {
        let seq = TMP_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let candidate = idx_dir.join(format!(".qsv-index-{pid}-{seq}.tmp"));
        if let Ok(f) = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)
        {
            tmp_path = Some(candidate);
            tmp_file = Some(f);
            break;
        }
    }
    let (Some(tmp_path), Some(idxfile)) = (tmp_path, tmp_file) else {
        return fail_clierror!(
            "Could not create a temp index file for {} in {}",
            dest.display(),
            idx_dir.display()
        );
    };

    // any early return from here on must not leave the temp behind
    let cleanup = |tmp: &Path| {
        let _ = fs::remove_file(tmp);
    };

    // the BufWriter is scoped so its file handle is dropped BEFORE the rename below -
    // an un-dropped write handle deadlocks on Windows ARM64
    let build = {
        let mut wtr = BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, idxfile);
        RandomAccessSimple::create(rdr, &mut wtr).and_then(|()| wtr.flush().map_err(Into::into))
    };
    if let Err(e) = build {
        cleanup(&tmp_path);
        return Err(e.into());
    }

    // On a REBUILD, carry over the existing index's permissions, so an index someone
    // deliberately chmod'd keeps that mode instead of reverting to the umask default.
    #[cfg(unix)]
    if let Ok(existing) = fs::metadata(dest) {
        let _ = fs::set_permissions(&tmp_path, existing.permissions());
    }

    // rename is atomic on the same filesystem: readers see either the old complete index
    // or the new one, never a torn file
    if let Err(e) = fs::rename(&tmp_path, dest) {
        cleanup(&tmp_path);
        return fail_clierror!(
            "Could not put the index in place at {}: {e}",
            dest.display()
        );
    }
    Ok(())
}

/// Creates an index file for the given CSV file path.
///
/// This function creates a CSV index file that enables random access and parallel processing.
/// It checks for edge cases like snappy-compressed files and stdin input.
///
/// The index is built atomically (see `write_index_atomically`), so a failure here leaves any
/// pre-existing index untouched and no partial index behind.
///
/// # Arguments
///
/// * `path` - Path to the CSV file to index
/// * `rconfig` - CSV reader configuration
///
/// # Returns
///
/// * `Ok(())` - Index was successfully created
/// * `Err(CliError)` - If index creation failed
pub fn create_index_for_file(path: &Path, rconfig: &Config) -> CliResult<()> {
    // Don't create index for snappy-compressed files
    if path.to_string_lossy().to_ascii_lowercase().ends_with(".sz") {
        return fail_clierror!(
            "Cannot create index for snappy-compressed files. Please decompress first."
        );
    }

    let pidx = idx_path(path);
    log::info!("Auto-creating index file: {}", pidx.display());

    let mut rdr = rconfig.reader_file()?;
    write_index_atomically(&mut rdr, &pidx)?;

    log::info!("Successfully created index file: {}", pidx.display());
    Ok(())
}
