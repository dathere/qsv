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

    let log = wrk.read_to_string("qsv_rCURRENT.log").unwrap_or_default();
    assert!(
        log.contains("alloc tuning disabled via QSV_NO_ALLOC_TUNING"),
        "expected the .env-configured QSV_NO_ALLOC_TUNING opt-out to be honored at startup \
         (load_dotenv must precede init_allocator_runtime); log was:\n{log}"
    );
}
