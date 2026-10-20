use std::{path::Path, process::Command, time::Instant};

use super::{QSV_PATH, QUIET_FLAG, current_exe};
use crate::{CliError, CliResult};

/// BLAKE3 hash of a file optimized for maximum performance
/// Uses memory mapping and multithreading for fast hashing of files of any size
// consumed by describegpt and viz, both excluded from qsvlite
#[cfg(not(feature = "lite"))]
pub fn hash_blake3_file(path: &Path) -> CliResult<String> {
    let mut hasher = blake3::Hasher::new();

    // Use BLAKE3's optimized memory-mapped + rayon parallel hashing
    // This automatically handles chunking and parallel processing internally
    hasher.update_mmap_rayon(path)?;

    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(not(feature = "lite"))]
pub fn is_executable(path: &str) -> std::io::Result<bool> {
    cfg_select! {
        unix => {
            use std::{fs, os::unix::fs::PermissionsExt};

            let metadata = fs::metadata(path)?;
            Ok(metadata.permissions().mode() & 0o111 != 0)
        },
        windows => {
            use std::path::Path;
            let p = Path::new(path);
            Ok(p.extension().and_then(|e| e.to_str()).is_some_and(|ext| {
                matches!(
                    ext.to_ascii_lowercase().as_str(),
                    "exe" | "bat" | "cmd" | "com"
                )
            }))
        },
    }
}

/// Print a status message with elapsed time if not in quiet mode
pub fn print_status(msg: &str, elapsed: Option<std::time::Duration>) {
    // this checks the QUIET_FLAG atomic boolean if it was set
    // Otherwise, it defaults to false and prints the message
    if !QUIET_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
        if let Some(duration) = elapsed {
            eprintln!("{msg} (elapsed: {:.2}s)", duration.as_secs_f64());
        } else {
            eprintln!("{msg}");
        }
    }
}

// Helper function to run qsv commands with consistent error handling and timing
pub fn run_qsv_cmd(
    command: &str,
    args: &[&str],
    input_path: &str,
    status_msg: &str,
) -> CliResult<(String, String)> {
    let start_time = Instant::now();

    // safety: we know that the current_exe() is very unlikely to fail as qsv is already running
    let qsv_path = QSV_PATH.get_or_init(|| current_exe().unwrap().to_string_lossy().to_string());
    let mut cmd = Command::new(qsv_path);

    // sample takes a positional <sample-size> argument *before* the input path
    // (e.g. `qsv sample 1000 input.csv`), so for sample we order args before
    // input. Every other command in qsv accepts the input as the first
    // positional, so input goes before flags/args.
    if command == "sample" {
        cmd.arg(command).args(args).arg(input_path);
    } else {
        cmd.arg(command).arg(input_path).args(args);
    }

    let output = cmd
        .output()
        .map_err(|e| CliError::Other(format!("Error while executing command {command}: {e:?}")))?;
    log::debug!("qsv command {command} output: {output:?}");

    if !output.status.success() {
        return fail_clierror!("Command {command} failed: {output:?}");
    }

    print_status(status_msg, Some(start_time.elapsed()));

    let stdout_str = std::str::from_utf8(&output.stdout).map_err(|e| {
        CliError::Other(format!(
            "Unable to parse output of qsv command {command}: {e:?}"
        ))
    })?;
    let stderr_str = std::str::from_utf8(&output.stderr).map_err(|e| {
        CliError::Other(format!(
            "Unable to parse stderr of qsv command {command}: {e:?}"
        ))
    })?;

    Ok((stdout_str.to_string(), stderr_str.to_string()))
}

/// Like `run_qsv_cmd`, but *streams* the child's live progress to the terminal instead of capturing
/// its output. The child inherits STDERR, so its own `winfo!`/status lines and progress bars render
/// as it runs; STDOUT is discarded so the child can never corrupt this process's own stdout (e.g.
/// `viz`'s piped HTML). `QSV_PROGRESSBAR=1` is set for the child so its OWN in-process progress
/// bars are enabled. This surfaces only what the child writes to its own stderr — it does NOT
/// reveal progress from any subprocess the child itself spawns and captures (e.g. `moarstats` runs
/// its nested `stats`/`frequency` via capturing calls, so those stay hidden). Returns `Ok(())` on
/// success; use when the caller only needs the child's on-disk side effects (e.g. `moarstats`'
/// stats/bivariate cache sidecars), not its stdout. Callers that render their own progress bar
/// should suspend it around this call so the two don't collide.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
pub fn run_qsv_cmd_streaming(
    command: &str,
    args: &[&str],
    input_path: &str,
    status_msg: &str,
) -> CliResult<()> {
    let start_time = Instant::now();

    // safety: we know that the current_exe() is very unlikely to fail as qsv is already running
    let qsv_path = QSV_PATH.get_or_init(|| current_exe().unwrap().to_string_lossy().to_string());
    let mut cmd = Command::new(qsv_path);

    if command == "sample" {
        cmd.arg(command).args(args).arg(input_path);
    } else {
        cmd.arg(command).arg(input_path).args(args);
    }

    cmd.env("QSV_PROGRESSBAR", "1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::inherit());

    let status = cmd
        .status()
        .map_err(|e| CliError::Other(format!("Error while executing command {command}: {e:?}")))?;

    if !status.success() {
        return fail_clierror!("Command {command} failed with exit status: {status}");
    }

    print_status(status_msg, Some(start_time.elapsed()));
    Ok(())
}

/// Sync subprocess output to disk and validate it is non-empty.
///
/// After a child `qsv` subprocess writes its output via `--output <path>` and
/// exits, the file's bytes may still be in the kernel page cache and not yet
/// visible to follow-up `open()` calls in another process. This is especially
/// noticeable on macOS APFS under heavy parallel test load and has produced
/// silent "primary-only" join output in CI.
///
/// Opens the file with WRITE access (no truncate, no create — just a writable
/// handle), calls `sync_all()` to force a flush, then verifies the resulting
/// file is non-empty.
///
/// Why write access: on Windows, `sync_all()` is implemented via
/// `FlushFileBuffers`, which rejects read-only handles with "Access is
/// denied" (os error 5). Opening with `.write(true)` works on Linux/macOS too
/// and doesn't modify the file as long as `truncate`/`create` are not set.
pub fn sync_subprocess_output(path: &Path) -> CliResult<()> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|e| {
            CliError::Other(format!(
                "Unable to open subprocess output for fsync ({}): {e}",
                path.display()
            ))
        })?;
    f.sync_all().map_err(|e| {
        CliError::Other(format!(
            "Failed to sync subprocess output ({}): {e}",
            path.display()
        ))
    })?;
    drop(f);

    let size = std::fs::metadata(path)
        .map_err(|e| {
            CliError::Other(format!(
                "Unable to stat subprocess output ({}): {e}",
                path.display()
            ))
        })?
        .len();
    if size == 0 {
        return fail_clierror!("Subprocess output is empty after sync: {}", path.display());
    }
    Ok(())
}

/// Best-effort fsync of the directory at `path` so its directory entry
/// metadata is durable.
///
/// `fsync(file)` on Linux does NOT flush the parent directory's metadata.
/// While that primarily matters for crash safety, some FS configurations
/// (overlayfs, fuse, network mounts) and edge-case timing have been seen
/// to affect read-after-write visibility for newly-created files when a
/// follow-up process opens them via path lookup. This helper is layered
/// on top of the file-level `sync_subprocess_output` and exists purely
/// as a belt-and-suspenders guard.
///
/// Errors are intentionally NOT propagated:
/// - Directory fsync is not supported uniformly across Linux filesystems (FAT, some
///   FUSE/overlay/network mounts return `EINVAL`/`EOPNOTSUPP`).
/// - Even when supported, the caller has already gone through the file-level fsync; failing the
///   whole operation because the directory fsync errored would punish environments that don't
///   support it.
///
/// Errors are logged at debug level. No-op on non-unix targets (on Windows,
/// `FlushFileBuffers` does not accept directory handles).
pub fn sync_directory(path: &Path) {
    #[cfg(unix)]
    {
        // Best-effort: directory fsync is not supported uniformly across
        // Linux filesystems (FAT, some FUSE/overlay/network mounts return
        // EINVAL/EOPNOTSUPP). Since this is a defensive guard layered on
        // top of the file-level fsync, an error here must NOT fail the
        // caller — log at debug and continue.
        match std::fs::File::open(path) {
            Ok(f) => {
                if let Err(e) = f.sync_all() {
                    log::debug!(
                        "sync_directory: fsync of {} returned {e} (continuing — not all FS \
                         configurations support directory fsync)",
                        path.display()
                    );
                }
            },
            Err(e) => {
                log::debug!(
                    "sync_directory: open of {} returned {e} (continuing — best-effort guard)",
                    path.display()
                );
            },
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path; // suppress unused-variable warning on Windows
    }
}
