use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::CliResult;

/// Reads a CSV file (comma-delimited) and writes it with a different delimiter to a writer.
pub fn csv_to_delimited_writer<W: Write>(
    input_csv: &str,
    writer: &mut W,
    delimiter: u8,
) -> CliResult<()> {
    let file = File::open(input_csv)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(file);
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(writer);

    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    for result in rdr.records() {
        let record = result?;
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

/// What mode does a newly created file ACTUALLY get in `dir`?
///
/// Creates an empty probe at 0666, reads back the mode the kernel gave it, and removes it. This
/// is an oracle rather than a calculation: it captures the process umask and anything else the
/// filesystem applies to new files here (on Linux a directory default ACL constrains the mode the
/// same way), none of which std exposes directly.
///
/// The real derived file must NOT be created this way - an fd obtained while it is briefly wide
/// survives every later chmod, which is the hole roborev 4796 closed. The probe is safe precisely
/// because it never receives a single byte, so an fd on it reveals nothing.
///
/// Returns `None` if no probe could be created; the caller then assumes owner-only, and the real
/// temp's own creation produces the actionable error.
#[cfg(unix)]
pub fn new_file_mode_in(dir: &Path) -> Option<u32> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    static PROBE_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let pid = std::process::id();
    for _ in 0..16 {
        let seq = PROBE_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let probe_path = dir.join(format!(".qsv-probe-{pid}-{seq}.tmp"));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o666)
            .open(&probe_path)
        {
            Ok(probe) => {
                // no `?` between create and remove, so the probe cannot be leaked
                let mode = probe
                    .metadata()
                    .ok()
                    .map(|m| m.permissions().mode() & 0o777);
                drop(probe);
                let _ = std::fs::remove_file(&probe_path);
                return mode;
            },
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return None,
        }
    }
    None
}

/// A `<FILESTEM>.*` artifact derived from a source file - a stats or frequency cache - that
/// must never be readable by anyone the source itself does not admit (#4619).
///
/// Such an artifact holds the DATA (every non-high-cardinality value and its count, a column's
/// min/max/mode/antimode), not byte offsets like an `.idx`, so a 0600 CSV must not yield a 0644
/// cache. It is built in a sibling temp, CREATED at its final mode, and renamed over `dest`:
///   - the file never exists wider than its target. Creating at the umask default and chmod-ing
///     afterwards - in either order relative to the write - leaves a window in which the file is
///     openable at the wider mode, and an fd obtained in that window stays readable through every
///     later chmod. The only fix is for the file never to be born wide;
///   - a failure part-way (ENOSPC, EDQUOT, EIO, a network-mount hiccup) leaves the PREVIOUS
///     artifact intact rather than a 0-byte or half-written one. What that is worth was measured on
///     ONE reader, so it is stated as such: for `frequency`, a truncated cache is not a correctness
///     problem - `read_frequency_cache` treats a missing or unparseable metadata line, an
///     unparseable entry, and an empty entry set alike as a cache MISS and recomputes, so the
///     command still prints the right answer and exits 0. What it costs there is speed, silently
///     and indefinitely: an ordinary run does not rewrite the broken file, so every later run pays
///     the full recompute until the cache is regenerated explicitly. That - not recoverability - is
///     what is being bought here.
///
///     Do NOT promote that to "every reader misses": it is already false for one. `scoresql`'s
///     `load_freq_cache` reads the same `.freq.csv.data.jsonl` and silently SKIPS lines that fail
///     to parse, so a truncated cache scores against a partial one instead of missing. A consumer
///     gets whatever its own reader does; check that reader rather than assuming this bullet.
///     The temp is removed on drop, so no stray is left beside the user's CSV;
///   - the temp is in the same directory, so `rename` cannot fail with EXDEV. `create_new`
///     (`O_CREAT|O_EXCL`) means two concurrent writers cannot collide on one temp, and a
///     pre-existing file or symlink at a guessable name cannot be written through.
///
/// Mode resolution on unix, from what `std` exposes - mode bits and gid:
///   1. start from the mode of the artifact being REPLACED, so a deliberate chmod survives a
///      refresh. A brand-new artifact starts from the umask default, probed rather than assumed
///      (`new_file_mode_in`). Rule 2 overrides this: "survives" means "survives, but never beyond
///      what the source grants";
///   2. intersect with the source's mode - grant nothing the source does not. Intersecting is what
///      lets this coexist with the umask default: a world-readable CSV still gets a world-readable
///      cache, so shared-machine reuse is untouched;
///   3. GROUP bits are granted only when the new file's gid equals the source's. A mode bit is not
///      portable between inodes: a new file's group is the DIRECTORY's on macOS/BSD and the
///      process's egid on Linux (unless the directory is setgid) - never, in general, the source's.
///      So a 0640 CSV owned by group `admin` could yield a 0640 cache owned by group `staff`,
///      handing the values to every member of a group that cannot read the CSV at all. We do NOT
///      fchown to the source's group: a non-root process can only chgrp into a group it already
///      belongs to, so that fails exactly when the source's group is the restricted one that
///      matters. Dropping the bits is simpler and fails closed;
///   4. on a gid mismatch, OTHER bits survive only where the source grants them to its own group
///      too (`source_group & source_other`). "other" is the same CLASS on both files but not the
///      same set of users, because Unix selects exactly one class - owner, else group, else other -
///      with no fallthrough. A 0604 source DENIES its own group while allowing everyone else (the
///      "block this group" idiom); copying 0604 onto a file with a different group puts those very
///      users in the OTHER class and lets them in. A world-readable 0644 source still yields 0604,
///      so genuinely public data is unaffected.
///
/// Known residual, deliberately not chased (#4619): ACLs, xattrs and MAC labels can all make
/// effective access differ from what mode+gid say, and none is visible through `std`. The
/// probe does capture a Linux directory default ACL's effect on the mode, which is partial
/// mitigation, not a solution.
///
/// Failures are errors, not `let _ =`: this is a confidentiality control, and silently
/// publishing a 0644 copy of a 0600 CSV because a stat failed is the outcome it exists to
/// prevent. Callers that must not fail the whole command over a cache (a best-effort refresh)
/// decide that at their level.
pub struct DerivedFile {
    file:     File,
    /// `Some` while the temp exists and is ours to remove; `None` once installed.
    tmp_path: Option<PathBuf>,
    dest:     PathBuf,
}

impl DerivedFile {
    /// Creates the temp beside `dest`, at the mode `dest` will end up with, derived from
    /// `source` as described on the type. `tag` names the temp (`.qsv-<tag>-<pid>-<seq>.tmp`)
    /// so a stray is attributable to the artifact that left it.
    pub fn create(source: &Path, dest: &Path, tag: &str) -> CliResult<Self> {
        let dir = dest
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));

        #[cfg(not(unix))]
        let _ = source;
        #[cfg(unix)]
        let (create_mode, target_mode, source_gid, source_group_as_other) = {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};

            let src = std::fs::metadata(source)?;
            let input_mode = src.permissions().mode() & 0o777;
            let base_mode = match std::fs::metadata(dest) {
                // refreshing: start from the mode the user left on the artifact. No probe needed.
                Ok(existing) => existing.permissions().mode() & 0o777,
                // new: ask an empty probe what a new file in this directory actually gets
                Err(_) => new_file_mode_in(dir).unwrap_or(0o600),
            };
            // Create OWNER-ONLY. The group and other bits are added back after creation, once
            // the temp's real gid is known - see the fchmod below. Creating at the full target
            // and narrowing afterwards would reopen roborev 4796.
            (
                (base_mode & input_mode) & 0o700,
                base_mode & input_mode,
                src.gid(),
                // the source's GROUP bits shifted into the OTHER position - rule 4
                (input_mode & 0o070) >> 3,
            )
        };

        // Uniqueness needs no randomness: `create_new` fails with `AlreadyExists`, so the loop
        // advances the counter until it wins. The pid separates processes, the counter
        // separates threads and repeat calls within one.
        static TMP_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let pid = std::process::id();
        let mut open_opts = std::fs::OpenOptions::new();
        open_opts.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            open_opts.mode(create_mode);
        }
        let mut created = None;
        for _ in 0..16 {
            let seq = TMP_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let candidate = dir.join(format!(".qsv-{tag}-{pid}-{seq}.tmp"));
            match open_opts.open(&candidate) {
                Ok(file) => {
                    created = Some((candidate, file));
                    break;
                },
                // ONLY a name collision is worth retrying. Retrying a permission, quota,
                // read-only-filesystem or bad-path error just burns 15 more syscalls and then
                // reports a generic message, throwing away the actionable OS error.
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => {
                    return fail_clierror!(
                        "Could not create a temp file for {} in {}: {e}",
                        dest.display(),
                        dir.display()
                    );
                },
            }
        }
        let Some((tmp_path, file)) = created else {
            // only reachable by exhausting the retries on AlreadyExists, which really is a
            // collision problem and has no OS error worth relaying
            return fail_clierror!(
                "Could not find a free temp filename for {} in {} after 16 attempts",
                dest.display(),
                dir.display()
            );
        };
        // from here on every `?` removes the temp on the way out (Drop)
        let derived = Self {
            file,
            tmp_path: Some(tmp_path),
            dest: dest.to_path_buf(),
        };

        // Widen from owner-only to the access the artifact should actually grant - never past
        // it, so no wider-than-target window is ever opened. Through the fd: no permission
        // change here goes by pathname, which a directory-entry swap in a shared directory
        // could redirect onto another file. For a REPLACED artifact the umask may have
        // narrowed the creation request below the mode the user chose, so this is also what
        // restores it; it only ever widens, from a mode already no broader than the target.
        #[cfg(unix)]
        {
            use std::os::unix::fs::{MetadataExt, PermissionsExt};

            let meta = derived.file.metadata()?;
            let allowed = if meta.gid() == source_gid {
                target_mode
            } else {
                target_mode & (0o700 | source_group_as_other)
            };
            if (meta.permissions().mode() & 0o777) != allowed {
                derived
                    .file
                    .set_permissions(std::fs::Permissions::from_mode(allowed))?;
            }
        }

        Ok(derived)
    }

    /// fsyncs the temp and renames it over `dest`. On any failure the temp is removed and
    /// `dest` is left as it was.
    pub fn install(mut self) -> CliResult<()> {
        // Windows has no group/world bits to leak, so the only thing to carry over is the
        // read-only flag - applied AFTER the write rather than before, because marking the temp
        // read-only up front would make our own writes fail. Through the fd, like the unix
        // branch.
        #[cfg(not(unix))]
        if let Ok(existing) = std::fs::metadata(&self.dest) {
            self.file.set_permissions(existing.permissions())?;
        }

        // fsync before the rename. `rename` is atomic for the name -> inode mapping, but that
        // says nothing about the new file's DATA being on storage: a crash or power loss can
        // persist the rename while the blocks behind it are still in writeback, leaving the
        // truncated artifact described on the type - which for `frequency` costs every later
        // run a full recompute, silently, until the cache is regenerated by hand, and which a
        // reader that skips bad lines instead of missing (`scoresql`) consumes partially. There
        // is no buffer to flush - `io::Write::flush` on a `File` is a no-op - so fsync is the
        // only thing that helps. It is the same inode the mode was set on, so this syncs that
        // metadata too; nothing has been renamed yet.
        //
        // Kept because it is one fsync per cache WRITE - not per run and not per row - against
        // a failure whose cost is unbounded in time. If that trade ever stops looking worth it,
        // this is one line to delete.
        //
        // The parent directory is deliberately NOT synced. Without that, a crash can lose the
        // rename itself and leave the OLD artifact in place - stale, but intact and still
        // healable. That is a perfectly safe outcome, so a second fsync would buy nothing here.
        self.file.sync_all()?;
        // `?` on the rename drops `self` with `tmp_path` still armed, so the temp is removed
        let tmp_path = self.tmp_path.as_deref().unwrap_or_else(|| {
            unreachable!("DerivedFile::install: the temp is armed until this rename succeeds")
        });
        std::fs::rename(tmp_path, &self.dest)?;
        self.tmp_path = None;
        Ok(())
    }
}

impl Write for DerivedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl Drop for DerivedFile {
    fn drop(&mut self) {
        if let Some(path) = self.tmp_path.take() {
            let _ = std::fs::remove_file(path);
        }
    }
}
