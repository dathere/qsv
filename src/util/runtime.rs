use std::{env, io::Write, path::PathBuf};

use human_panic::setup_panic;

#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
use super::{BACKGROUND_THREADS_ACTIVE, get_envvar_flag};
use super::{JOBS_TO_USE, WHITESPACE_MARKERS, num_cpus};
use crate::{CliResult, QsvExitCode};

/// Render a duration in seconds as a compact, single-unit string: "28d", "6h", "45m", "3s".
///
/// Deliberately ONE unit, the largest that fits, and deliberately WORDLESS. Callers that want a
/// phrase build it around this ("last fetched {} ago"); callers that want a table cell use it
/// as-is. An earlier version baked the phrase in, which made it unusable as a column value.
///
/// Precision beyond one unit is noise for both consumers — the number exists to answer "do I
/// care?", not to be arithmetic. Days are the largest unit: `qsv get --older-than` also accepts
/// weeks on INPUT, but "28d" reads more plainly in a cache listing than "4w", and no caller
/// round-trips this back through that parser.
// `diskcache::rich`'s stale-refresh warning and `get cache-list`'s AGE column are the only
// callers, and both live under the `get` feature.
#[cfg(any(feature = "get", test))]
pub fn fmt_duration_compact(secs: i64) -> String {
    const MINUTE: i64 = 60;
    const HOUR: i64 = 60 * MINUTE;
    const DAY: i64 = 24 * HOUR;

    match secs {
        s if s >= DAY => format!("{}d", s / DAY),
        s if s >= HOUR => format!("{}h", s / HOUR),
        s if s >= MINUTE => format!("{}m", s / MINUTE),
        s => format!("{s}s"),
    }
}

pub fn reset_sigpipe() {
    cfg_select! {
        unix => unsafe {
            libc::signal(libc::SIGPIPE, libc::SIG_DFL);
        },
        _ => {},
    }
}

pub fn current_exe() -> CliResult<PathBuf> {
    let exe_path = std::env::current_exe()?;
    Ok(exe_path)
}

/// Visualizes whitespace characters in a string by replacing them with visible markers
///
/// This function takes a string and returns a new string where whitespace characters
/// are replaced with visible Unicode markers to make them easier to see.
///
/// # Arguments
///
/// * `s` - The input string to visualize whitespace in
///
/// # Returns
///
/// A new String with whitespace characters replaced by visible markers
///
/// # Behavior
///
/// - If the input string contains only spaces, each space is replaced with "《_》"
/// - For other whitespace characters (tab, newline, etc), uses markers defined in
///   `WHITESPACE_MARKERS`
/// - Non-whitespace characters are left unchanged
/// - For strings with mixed content, single spaces are preserved as-is
///
/// # Examples
///
/// ```
/// let s = "hello\tworld\n";
/// let vis = visualize_whitespace(s);
/// assert_eq!(vis, "hello《→》world《¶》");
///
/// let spaces = "   ";
/// let vis = visualize_whitespace(spaces);
/// assert_eq!(vis, "《_》《_》《_》");
/// ```
pub fn visualize_whitespace(s: &str) -> String {
    // Check if string is all spaces
    let is_all_spaces = s.chars().all(|c| c == ' ');

    let mut result = String::with_capacity(s.len() * 3);
    for c in s.chars() {
        if c == ' ' {
            if is_all_spaces {
                // Only use space marker if entire string is spaces
                result.push_str("《_》");
            } else {
                result.push(c);
            }
        } else if let Some((_, replacement)) = WHITESPACE_MARKERS.iter().find(|(ws, _)| *ws == c) {
            result.push_str(replacement);
        } else {
            result.push(c);
        }
    }
    result
}

/// Converts a byte slice to a `Cow<str>` for output formatting.
///
/// Uses SIMD-accelerated UTF-8 validation via `simdutf8::basic::from_utf8` on the
/// happy path (returns a borrowed `&str` with no allocation on valid UTF-8, which
/// is the overwhelming common case including all-ASCII).
///
/// Falls back to `String::from_utf8_lossy` only when the bytes are not valid
/// UTF-8 (replacement characters substituted, allocates).
///
/// This is faster than `String::from_utf8_lossy` alone, which always uses scalar
/// UTF-8 validation and pre-allocates a `String` of the input length.
#[inline]
pub fn bytes_to_cow_str(c: &[u8]) -> std::borrow::Cow<'_, str> {
    if let Ok(s) = simdutf8::basic::from_utf8(c) {
        std::borrow::Cow::Borrowed(s)
    } else {
        String::from_utf8_lossy(c)
    }
}

/// Does this panic payload come from Rust's own print machinery?
///
/// `println!`/`eprintln!` go through `std::io::_print`, which PANICS on a write error rather
/// than returning it — `panic!("failed printing to {label}: {e}")` (`library/std/src/io/
/// stdio.rs`, unchanged across every toolchain checked). That is an I/O failure, not a qsv
/// bug, and it is not reachable from qsv's own `wout!`/`werr!` macros (they handle their
/// `Result`). Kept as a free function so the classification is unit-testable without
/// provoking a real panic.
///
/// The trailing `": "` is part of that format string and is matched DELIBERATELY: without it
/// the prefix would also swallow any future panic merely beginning "failed printing to
/// stdout…", and a false positive here costs a real bug its crash report. Matching the full
/// literal prefix costs nothing, since std always emits the separator.
pub(super) fn is_io_print_panic(payload: &str) -> bool {
    payload.starts_with("failed printing to stdout: ")
        || payload.starts_with("failed printing to stderr: ")
}

/// The panic payload as a string, covering both `panic!("literal")` (`&str`) and
/// `panic!("{}", fmt)` (`String`).
fn panic_payload_str<'a>(info: &'a std::panic::PanicHookInfo<'_>) -> Option<&'a str> {
    let payload = info.payload();
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

pub fn qsv_custom_panic() {
    setup_panic!(
        human_panic::Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .authors("datHere qsv maintainers")
            .homepage("https://qsv.dathere.com")
            .support("- Open a GitHub issue at https://github.com/dathere/qsv/issues")
    );

    // Intercept the ONE panic shape that is an I/O failure rather than a bug: a failed
    // `println!`/`eprintln!`. Left alone, it reaches the user through human_panic as "qsv had
    // a problem and crashed", with a crash report to file — which is exactly what issue #2661
    // reported (`qsv sniff --json | jaq <bad filter>`) and what #4516 tracks for the
    // remaining cases (a full disk, a closed fd).
    //
    // Deliberately NARROW: only this known payload prefix is intercepted, so a genuine qsv
    // panic still gets the full crash report. This does not replace `reset_sigpipe` — on Unix
    // a closed PIPE never reaches here at all, since SIGPIPE is SIG_DFL (see #2664), and that
    // must stay that way.
    //
    // In a debug build `setup_panic!` installs nothing, so this wraps the default hook
    // instead; the interception behaves identically either way.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if let Some(payload) = panic_payload_str(info)
            && is_io_print_panic(payload)
        {
            // Best-effort: the stream we are complaining about may be the broken one.
            let _ = writeln!(&mut std::io::stderr(), "{payload}");
            std::process::exit(QsvExitCode::Bad as i32);
        }
        previous_hook(info);
    }));
}

pub fn max_jobs() -> usize {
    let num_cpus = num_cpus();
    let max_jobs = match env::var("QSV_MAX_JOBS") {
        Ok(val) => val.parse::<usize>().unwrap_or(1_usize),
        Err(_) => num_cpus,
    };
    if (1..=num_cpus).contains(&max_jobs) {
        max_jobs
    } else {
        num_cpus
    }
}

/// Given a desired number of cores to use
/// returns number of cores to actually use and set
/// rayon global thread pool size accordingly.
/// If desired is None, zero, or greater than available cores,
/// returns `max_jobs`, which is equal to number of available cores
/// If desired is Some and less than available cores,
/// returns desired number of cores
pub fn njobs(flag_jobs: Option<usize>) -> usize {
    let njobs_result = JOBS_TO_USE.get_or_init(|| {
        let max_jobs = max_jobs();
        let jobs_to_use = flag_jobs.map_or(max_jobs, |jobs| {
            if jobs == 0 || jobs > max_jobs {
                max_jobs
            } else {
                jobs
            }
        });
        match rayon::ThreadPoolBuilder::new()
            .num_threads(jobs_to_use)
            .build_global()
        {
            Err(e) => {
                log::warn!("Failed to set global thread pool size to {jobs_to_use}: {e}");
            },
            _ => {
                log::info!("Using {jobs_to_use} jobs...");
            },
        }
        jobs_to_use
    });
    *njobs_result
}

/// One-time, process-wide allocator setup. Called from `main()` right after the
/// signal handlers are installed, before any command runs.
///
/// Lever A: enable jemalloc `background_thread`, which offloads page purging
/// (`madvise`) to background threads, removing it from the foreground hot path at
/// NO RSS cost. Supported on Linux; rejected on macOS (the write errors — handled
/// best-effort, leaving `BACKGROUND_THREADS_ACTIVE` false). The per-command
/// retention lever (Lever B) is itself gated to Linux, so it does NOT engage on
/// macOS as a fallback. Disabled entirely by `QSV_NO_ALLOC_TUNING`. No
/// `cfg(target_os)` gate: the write+read-back is self-correcting across platforms.
#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
pub fn init_allocator_runtime() {
    use std::sync::atomic::Ordering;

    use tikv_jemalloc_ctl::background_thread;

    if get_envvar_flag("QSV_NO_ALLOC_TUNING") {
        log::info!("alloc tuning disabled via QSV_NO_ALLOC_TUNING");
        return;
    }
    // Best-effort: ignore the write error (e.g. macOS, where it's unsupported)
    // and trust the read-back for the authoritative state.
    let _ = background_thread::write(true);
    if let Ok(active) = background_thread::read() {
        BACKGROUND_THREADS_ACTIVE.store(active, Ordering::Relaxed);
        log::info!("jemalloc background_thread active: {active}");
    }
}

#[cfg(not(all(feature = "jemallocator", not(feature = "mimalloc"))))]
pub fn init_allocator_runtime() {}

/// PROTOTYPE (jemalloc per-command tuning): aggregation-heavy commands
/// (`stats -E`, `frequency`, ...) build many `Frequencies` hashmaps that grow and
/// merge across rayon workers, producing heavy `madvise` page-return churn under
/// jemalloc's default decay. Retaining freed pages (dirty/muzzy decay = -1) for
/// the duration keeps them mapped for reuse, trading higher peak RSS (~+16% on a
/// 514 MB input) for fewer syscalls (~6-9% faster `stats -E`). Scoped per-command
/// — qsv is short-lived, so the process exits before retained pages matter; no
/// restore is performed.
///
/// LINUX-ONLY: this is only a win where jemalloc's default `madvise`-based page
/// decay creates the foreground purge churn it removes. On macOS, Lever A
/// (`background_thread`) is unsupported, so this fallback would otherwise run —
/// but the macOS memory compressor turns page retention into a net loss
/// (measured on a 32 GB host: `moarstats --bivariate-stats all` peak footprint
/// +23% and peak RSS ~32 GB, tipping into swap for a 40-60% slowdown; even on a
/// 64 GB host where it fits, ~+10 GB footprint, ~1.7x system time, and ~6%
/// slower wall-clock, with byte-identical output). So it is gated off there.
///
/// NOTE: the `MALLCTL_ARENAS_ALL` (4096) pseudo-index segfaults here for decay
/// writes via BOTH the string-name (`arena.4096.*`) and numeric-MIB interfaces
/// (tikv-jemalloc-sys 0.7 / macOS), so all arenas can't be set in one call.
/// Instead set the `arenas.*` template (inherited by arenas created later, e.g.
/// lazily-bound rayon workers) and loop over the currently-initialized arenas by
/// index. This is timing-dependent — arenas not yet created when this runs are
/// covered only by the template — but benchmarks show it captures essentially all
/// of what `opt.dirty_decay_ms` (the env-var path) delivers, within noise.
#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
pub fn retain_alloc_pages_for_aggregation() {
    use std::sync::atomic::Ordering;

    use tikv_jemalloc_ctl::raw;

    // Skip when tuning is disabled, or when Lever A (background_thread) is active —
    // background purging already removes the madvise churn without the RSS cost of
    // retaining pages, so applying retention on top would be pure RSS loss.
    if get_envvar_flag("QSV_NO_ALLOC_TUNING") || BACKGROUND_THREADS_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    // Retention only pays off on Linux (see fn doc). On macOS the memory
    // compressor makes it a net loss, and macOS can't run Lever A to offset it,
    // so the fallback would otherwise misfire here.
    if !cfg!(target_os = "linux") {
        return;
    }
    // safety: tikv_jemalloc_ctl::raw {read,write} call jemalloc's mallctl by name.
    // Invariants upheld here:
    //   - every key is a NUL-terminated byte string (the trailing `\0`), as the name-based mallctl
    //     API requires;
    //   - value widths match the mallctl types: `*_decay_ms` is `ssize_t` (isize, -1 = "never
    //     decay/retain pages"); `arenas.narenas` is `unsigned` (u32);
    //   - every result is discarded best-effort (`let _ =`), so a jemalloc build or platform
    //     lacking a given knob returns an error rather than aborting the command. No pointers are
    //     passed; mallctl copies the value in/out.
    unsafe {
        let _ = raw::write(b"arenas.dirty_decay_ms\0", -1_isize);
        let _ = raw::write(b"arenas.muzzy_decay_ms\0", -1_isize);
        if let Ok(narenas) = raw::read::<u32>(b"arenas.narenas\0") {
            for i in 0..narenas {
                let _ = raw::write(format!("arena.{i}.dirty_decay_ms\0").as_bytes(), -1_isize);
                let _ = raw::write(format!("arena.{i}.muzzy_decay_ms\0").as_bytes(), -1_isize);
            }
        }
    }
}

#[cfg(not(all(feature = "jemallocator", not(feature = "mimalloc"))))]
pub fn retain_alloc_pages_for_aggregation() {}

pub fn timeout_secs(timeout: u16) -> Result<u64, String> {
    let timeout = match env::var("QSV_TIMEOUT") {
        Ok(val) => val.parse::<u16>().unwrap_or(30_u16),
        Err(_) => timeout,
    };

    if timeout > 3600 {
        return fail_format!("Timeout cannot be more than 3,600 seconds (1 hour): {timeout}");
    } else if timeout == 0 {
        return fail!("Timeout cannot be zero.");
    }
    log::info!("TIMEOUT: {timeout}");
    Ok(timeout as u64)
}

/// Whether `dir` is safe for `log_end` to delete WHOLESALE.
///
/// Every site that initializes `TEMP_FILE_DIR` does so with a `tempfile::TempDir` qsv created,
/// so in practice this always answers true. It exists because the cost of being wrong is
/// severe and asymmetric: `Config::new` once fell back to `env::temp_dir()` itself when
/// `TempDir::new()` failed, and the cleanup would then have removed the SYSTEM temp root along
/// with every other process's files (roborev 4366). That site no longer initializes the lock at
/// all - it only reads it - but this check lives at the point of DELETION so a future init site
/// cannot quietly reintroduce the hazard.
pub(crate) fn temp_dir_is_removable(dir: &std::path::Path) -> bool {
    // a filesystem root has no parent
    if dir.parent().is_none() {
        return false;
    }
    let system_temp = std::env::temp_dir();
    if dir == system_temp {
        return false;
    }
    // compare resolved forms too, so /tmp vs /private/tmp (macOS) or a trailing separator
    // cannot sneak the system temp root past the check above
    match (dir.canonicalize(), system_temp.canonicalize()) {
        (Ok(resolved_dir), Ok(resolved_system)) => resolved_dir != resolved_system,
        // if either cannot be resolved we cannot prove they differ; the equality checks
        // above already passed, so allow it
        _ => true,
    }
}

pub fn log_end(mut qsv_args: String, now: std::time::Instant) {
    use crate::config::TEMP_FILE_DIR;

    // Remove the temp directory this run may have created, now that the command has
    // finished. `unwrap_or_default()` avoids a panic if it is already gone.
    //
    // This is deliberately NOT gated on `polars`. `TEMP_FILE_DIR` is declared
    // unconditionally and is populated in every build: `extract_zip_to_temp` is always
    // compiled (it needs only the non-optional `zip` crate), and the stdin spill temps in
    // `split`, `stats` and `partition` land here too. Gating the cleanup meant a non-polars
    // build (e.g. qsvlite) left a full decompressed copy of every `.zip` input on disk, once
    // per invocation, until the OS reclaimed the temp dir - which on many systems is only at
    // reboot. This is the ONLY cleanup site for `TEMP_FILE_DIR`.
    if let Some(temp_dir) = TEMP_FILE_DIR.get() {
        if temp_dir_is_removable(temp_dir) {
            std::fs::remove_dir_all(temp_dir).unwrap_or_default();
        } else {
            log::warn!(
                "refusing to remove {} - it is not a qsv-created temp directory",
                temp_dir.display()
            );
        }
    }
    if log::log_enabled!(log::Level::Info) {
        let ellipsis = if qsv_args.len() > 24 {
            utf8_truncate(&mut qsv_args, 24);
            "..."
        } else {
            ""
        };
        log::info!(
            "END \"{qsv_args}{ellipsis}\" elapsed: {}",
            now.elapsed().as_secs_f32()
        );
    }
}

/// Truncates a UTF-8 encoded string to fewer than `maxsize` bytes, preserving
/// valid UTF-8.
///
/// The result is at most `maxsize - 1` bytes (or 0 if `maxsize == 0`). The
/// "off-by-one" is intentional: stats `*PREVIEW*` snapshots are calibrated to
/// it. Don't change the offset without re-blessing those tests.
///
/// # Caller convention
///
/// To produce a string of at most `N` bytes (e.g., before appending an
/// ellipsis), callers should:
///
/// ```ignore
/// if input.len() > N {
///     util::utf8_truncate(&mut input, N + 1);  // +1 cancels the internal -1
///     input.push_str("...");
/// }
/// ```
///
/// The outer length guard avoids touching strings that already fit; the
/// `N + 1` argument compensates for the internal `maxsize - 1` so the result
/// is exactly `N` bytes before the suffix is appended. See the call site in
/// `src/cmd/stats.rs` (`antimodes` truncation).
///
/// # Arguments
///
/// * `input` - A mutable reference to the String to truncate
/// * `maxsize` - One past the maximum byte length the result may occupy
///
/// Uses `str::floor_char_boundary` so multi-byte characters are never split.
pub fn utf8_truncate(input: &mut String, maxsize: usize) {
    if input.len() > maxsize {
        // Find the largest char boundary strictly less than maxsize.
        // We deliberately use maxsize - 1 (not maxsize) because callers and
        // snapshot tests rely on truncation cutting one byte short of the
        // requested maximum (used by stats *PREVIEW* truncation).
        input.truncate(if maxsize > 0 {
            input.floor_char_boundary(maxsize - 1)
        } else {
            0
        });
    }
}
