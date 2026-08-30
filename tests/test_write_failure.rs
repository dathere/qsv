//! End-to-end coverage for issue #4516: a failed WRITE must be reported as an I/O error and a
//! non-zero exit, never as a qsv crash (a panic, or `human_panic`'s "qsv had a problem" report).
//!
//! These tests need a stdout that accepts `open()` but fails every `write()`, which is exactly
//! what `/dev/full` is. macOS has no equivalent — its `/dev/null` swallows writes successfully,
//! and the shell tricks that look like substitutes are not (closing fd 1 lets the next `open()`
//! claim it). So the assertions only really run on Linux, where CI runs.
//!
//! Compiled on every unix rather than gated to Linux ON PURPOSE: a `#[cfg(target_os = "linux")]`
//! module is never built on a macOS dev machine, so it rots silently until CI catches it. Here
//! the code always compiles and typechecks locally, and merely SKIPS at runtime when
//! `/dev/full` is missing.
//!
//! Both routes into the crash report are covered, because they are handled by different code:
//!   - `woutinfo!` (via `count`)  -> `WRITE_ERROR` + `QsvExitCode::report`
//!   - `println!`  (via `sniff --json`, the issue #2661 repro) -> the `qsv_custom_panic` hook

use std::{fs::OpenOptions, process::Stdio};

use crate::workdir::Workdir;

const ROWS: &str = "letter,number\na,1\nb,2\n";

/// Run `cmd` with stdout pointed at `/dev/full` (every write fails with ENOSPC) and stderr
/// captured. Returns `None` if `/dev/full` is unusable, so an unusual container skips rather
/// than fails.
///
/// Cannot use `Workdir::output`: `Command::output()` forces stdout to a pipe, which would
/// undo the very redirection under test.
fn run_with_failing_stdout(cmd: &mut std::process::Command) -> Option<(Option<i32>, String)> {
    let devfull = OpenOptions::new().write(true).open("/dev/full").ok()?;
    let child = cmd
        .stdout(Stdio::from(devfull))
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let out = child.wait_with_output().unwrap();
    Some((
        out.status.code(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    ))
}

/// Assertions common to both routes: a plain I/O failure, not a crash.
fn assert_io_error_not_a_crash(code: Option<i32>, stderr: &str, expected: &str) {
    assert!(
        stderr.contains(expected),
        "expected an I/O error mentioning {expected:?} on stderr; got:\n{stderr}"
    );
    // a panic that escaped the hook prints this and exits 101
    assert!(
        !stderr.contains("panicked"),
        "a failed write must not panic; got:\n{stderr}"
    );
    // human_panic's crash report (release builds); asserted in debug too so a release-mode
    // run of this suite cannot regress silently
    assert!(
        !stderr.contains("had a problem"),
        "a failed write must not produce a crash report; got:\n{stderr}"
    );
    assert_eq!(
        code,
        Some(1),
        "a failed write must exit 1 (not 0, and not 101 from a panic); stderr:\n{stderr}"
    );
}

// The macro route: `count` prints via `woutinfo!`, which records the failure in `WRITE_ERROR`
// so `QsvExitCode::report` can turn an otherwise-successful run into a failing one.
#[test]
fn write_failure_via_macros_is_an_io_error_not_a_crash() {
    let wrk = Workdir::new("write_failure_via_macros_is_an_io_error_not_a_crash");
    wrk.create_from_string("wf.csv", ROWS);

    let mut cmd = wrk.command("count");
    cmd.arg("wf.csv");

    let Some((code, stderr)) = run_with_failing_stdout(&mut cmd) else {
        eprintln!("skipping: /dev/full is not usable in this environment");
        return;
    };
    assert_io_error_not_a_crash(code, &stderr, "error writing to stdout");
}

// The `println!` route: `sniff --json` prints with `println!`, which panics INSIDE Rust's
// stdio rather than returning an error — no change to qsv's macros can reach it. This is the
// exact shape issue #2661 hit (`qsv sniff --json | jaq <bad filter>`), so it is the route that
// most needs a regression test.
#[test]
fn write_failure_via_println_is_an_io_error_not_a_crash() {
    let wrk = Workdir::new("write_failure_via_println_is_an_io_error_not_a_crash");
    wrk.create_from_string("wf.csv", ROWS);

    let mut cmd = wrk.command("sniff");
    cmd.arg("wf.csv").arg("--json");

    let Some((code, stderr)) = run_with_failing_stdout(&mut cmd) else {
        eprintln!("skipping: /dev/full is not usable in this environment");
        return;
    };
    assert_io_error_not_a_crash(code, &stderr, "failed printing to stdout");
}
