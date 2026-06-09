//! Regression tests for startup allocator-tuning ordering (roborev #2717 / #2718).
//!
//! `load_dotenv()` must run before `init_allocator_runtime()` (and before
//! `util::version()` is evaluated during Docopt setup) so a `QSV_NO_ALLOC_TUNING`
//! opt-out configured via the supported `.env` flow is honored at startup. If a
//! future refactor moves dotenv loading back after allocator init, this fails.
//!
//! jemalloc-only: under the mimalloc / standard allocators `init_allocator_runtime`
//! is a no-op stub that emits no such log line, so there is nothing to assert.

#[cfg(all(feature = "jemallocator", not(feature = "mimalloc")))]
#[test]
fn alloc_tuning_dotenv_opt_out_honored_at_startup() {
    use crate::workdir::Workdir;

    let wrk = Workdir::new("alloc_tuning_dotenv");

    // Opt-out supplied ONLY via .env (loaded by load_dotenv from the cwd), not the
    // process environment — this is exactly the flow the ordering bug broke.
    wrk.create_from_string(".env", "QSV_NO_ALLOC_TUNING=true\n");

    // `--list` returns normally so flexi_logger flushes its buffer to disk on exit
    // (`--version` would process::exit and skip the flush). QSV_LOG_LEVEL is set via
    // the process env so logging is on regardless of the .env contents, and the log
    // is written to the cwd (the workdir) by default.
    let mut cmd = wrk.command("--list");
    // Sanitize the inherited environment so the test is hermetic: the opt-out must
    // come ONLY from the workdir .env (an inherited QSV_NO_ALLOC_TUNING would mask
    // the regression), load_dotenv must read the workdir .env (not an inherited
    // QSV_DOTENV_PATH), and the log must land in the workdir cwd (not an inherited
    // QSV_LOG_DIR).
    cmd.env_remove("QSV_NO_ALLOC_TUNING")
        .env_remove("QSV_DOTENV_PATH")
        .env_remove("QSV_LOG_DIR")
        .env("QSV_LOG_LEVEL", "info");
    // assert_success runs the command and verifies it exited 0 before we inspect
    // the log, so a `--list` failure surfaces as a clear failure rather than an
    // empty/partial log silently failing the later assertion.
    wrk.assert_success(&mut cmd);

    // flexi_logger's `FileSpec::default()` names the log after the running binary, so the
    // file is `<bin>_rCURRENT.log` — `qsv_rCURRENT.log` for the main binary but
    // `qsvlite_rCURRENT.log` / `qsvdp_rCURRENT.log` under the `lite` / `datapusher_plus`
    // feature builds. Derive the name from the binary under test instead of hardcoding `qsv`,
    // otherwise the read silently returns "" in those builds and the assert fails on an empty
    // log. `file_stem()` also drops the `.exe` suffix on Windows.
    let log_name = format!(
        "{}_rCURRENT.log",
        wrk.qsv_bin().file_stem().unwrap().to_string_lossy()
    );
    let log = wrk.read_to_string(&log_name).unwrap_or_default();
    assert!(
        log.contains("alloc tuning disabled via QSV_NO_ALLOC_TUNING"),
        "expected the .env-configured QSV_NO_ALLOC_TUNING opt-out to be honored at startup \
         (load_dotenv must precede init_allocator_runtime); log ({log_name}) was:\n{log}"
    );
}

// THP Lever C (`maybe_apply_thp`) re-execs to enable jemalloc Transparent Huge
// Pages, and like Lever A must observe the `.env` flow — so it has to run AFTER
// `load_dotenv()` (roborev #2810). On a successful re-exec the child carries the
// `QSV_THP_APPLIED` sentinel, which `--envlist` surfaces (it lists every `QSV_`
// var). These assert the sentinel's presence/absence to prove the `.env`-driven
// decision works end-to-end.
//
// Linux + jemalloc only: `maybe_apply_thp` is a no-op stub elsewhere (THP is a
// Linux-only jemalloc opt.* knob), so there is no re-exec to observe.

/// `.env`-configured `QSV_THP` must trigger the re-exec (proving `maybe_apply_thp`
/// runs after `load_dotenv`, not before it where the process env is all it sees).
#[cfg(all(
    target_os = "linux",
    feature = "jemallocator",
    not(feature = "mimalloc")
))]
#[test]
fn thp_dotenv_opt_in_honored_at_startup() {
    use crate::workdir::Workdir;

    let wrk = Workdir::new("thp_dotenv_opt_in");

    // Opt-in supplied ONLY via .env (loaded by load_dotenv from the cwd), not the
    // process environment — exactly the flow the pre-dotenv ordering broke.
    wrk.create_from_string(".env", "QSV_THP=true\n");

    let mut cmd = wrk.command("--envlist");
    // Hermetic env: the opt-in must come ONLY from the workdir .env (an inherited
    // QSV_THP would mask the regression), load_dotenv must read the workdir .env
    // (not an inherited QSV_DOTENV_PATH), no inherited QSV_NO_ALLOC_TUNING may
    // suppress it, and no pre-set sentinel may short-circuit the re-exec or pollute
    // the assertion.
    cmd.env_remove("QSV_THP")
        .env_remove("QSV_THP_APPLIED")
        .env_remove("QSV_NO_ALLOC_TUNING")
        .env_remove("QSV_DOTENV_PATH");
    let got: String = wrk.stdout(&mut cmd);

    assert!(
        got.contains("QSV_THP_APPLIED"),
        "expected the .env-configured QSV_THP opt-in to trigger the THP re-exec (maybe_apply_thp \
         must run after load_dotenv); --envlist was:\n{got}"
    );
}

/// A `.env`-configured `QSV_NO_ALLOC_TUNING` must suppress the THP re-exec even
/// when `.env` also sets `QSV_THP` — both are only visible after `load_dotenv`.
#[cfg(all(
    target_os = "linux",
    feature = "jemallocator",
    not(feature = "mimalloc")
))]
#[test]
fn thp_dotenv_opt_in_suppressed_by_no_alloc_tuning() {
    use crate::workdir::Workdir;

    let wrk = Workdir::new("thp_dotenv_suppressed");

    wrk.create_from_string(".env", "QSV_THP=true\nQSV_NO_ALLOC_TUNING=true\n");

    let mut cmd = wrk.command("--envlist");
    cmd.env_remove("QSV_THP")
        .env_remove("QSV_THP_APPLIED")
        .env_remove("QSV_NO_ALLOC_TUNING")
        .env_remove("QSV_DOTENV_PATH");
    let got: String = wrk.stdout(&mut cmd);

    // `QSV_THP` (the toggle) still appears in --envlist; only the APPLIED sentinel
    // (set solely by a successful re-exec) must be absent.
    assert!(
        !got.contains("QSV_THP_APPLIED"),
        "expected .env QSV_NO_ALLOC_TUNING to suppress the THP re-exec; --envlist was:\n{got}"
    );
}
