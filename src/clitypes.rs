#![macro_use]
use std::{
    fmt, io,
    process::{ExitCode, Termination},
    sync::OnceLock,
};

use cached::{RedbCacheError, RedisCacheError};

// None of these macros may `.unwrap()` their write. A panic here is routed through
// `util::qsv_custom_panic` and reaches the user as a crash report asking them to file a bug,
// for what is really just a failed write — a full disk, a closed fd (issue #4516).
//
// The two stream classes are handled differently ON PURPOSE:
//   - stdout carries DATA. Losing it silently is unacceptable, so the first failure is recorded in
//     `WRITE_ERROR` and `QsvExitCode::report` turns it into a non-zero exit.
//   - stderr carries DIAGNOSTICS. A warning that cannot be delivered must not change the outcome of
//     an otherwise successful run, so it is dropped. The `log` record, emitted before the write,
//     survives either way.

/// write to stdout
macro_rules! wout {
    ($($arg:tt)*) => ({
        use std::io::Write;
        if let Err(e) = writeln!(&mut ::std::io::stdout(), $($arg)*) {
            crate::clitypes::record_write_error("stdout", &e);
        }
    });
}

/// write to stdout and `log::info`
macro_rules! woutinfo {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use log::info;
        let info = format!($($arg)*);
        info!("{info}");
        if let Err(e) = writeln!(&mut ::std::io::stdout(), $($arg)*) {
            crate::clitypes::record_write_error("stdout", &e);
        }
    });
}

/// write to stderr and `log::error`
macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use log::error;
        let error = format!($($arg)*);
        error!("{error}");
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)*);
    });
}

/// write to stderr and `log::warn`
macro_rules! wwarn {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use log::warn;
        let warning = format!($($arg)*);
        warn!("{warning}");
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)*);
    });
}

/// write to stderr and `log::info`
macro_rules! winfo {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use log::info;
        let info = format!($($arg)*);
        info!("{info}");
        let _ = writeln!(&mut ::std::io::stderr(), $($arg)*);
    });
}

/// write to stderr and `log::error`, returning Err(err)
macro_rules! fail {
    ($e:expr_2021) => {{
        use log::error;
        let err = ::std::convert::From::from($e);
        error!("{err}");
        Err(err)
    }};
}

/// write to stderr and `log::error`, using `CliError::Other`
macro_rules! fail_clierror {
    ($($t:tt)*) => {{
        use log::error;
        use crate::CliError;
        let err = format!($($t)*);
        error!("{err}");
        Err(CliError::Other(err))
    }};
}

/// write to stderr and `log::error`, using `CliError::IncorrectUsage`
macro_rules! fail_incorrectusage_clierror {
    ($($t:tt)*) => {{
        use log::error;
        use crate::CliError;
        let err = format!($($t)*);
        error!("{err}");
        Err(CliError::IncorrectUsage(err))
    }};
}

/// write to stderr and `log::error`, using `CliError::Encoding`
macro_rules! fail_encoding_clierror {
    ($($t:tt)*) => {{
        use log::error;
        use crate::CliError;
        let err = format!($($t)*);
        error!("{err}");
        Err(CliError::Encoding(err))
    }};
}

/// write to stderr and `log::error`, using `CliError::OutOfMemory`
macro_rules! fail_oom_clierror {
    ($($t:tt)*) => {{
        use log::error;
        use crate::CliError;
        let err = format!($($t)*);
        error!("{err}");
        Err(CliError::OutOfMemory(err))
    }};
}

/// write to stderr and `log::error`, returning Err(err) using a format string
macro_rules! fail_format {
    ($($t:tt)*) => {{
        use log::error;
        let err = format!($($t)*);
        error!("{err}");
        Err(err)
    }};
}

pub static CURRENT_COMMAND: OnceLock<String> = OnceLock::new();

/// The first failed write to a DATA stream (stdout), recorded instead of panicked on.
///
/// `wout!`/`woutinfo!` are used as statements in hundreds of places and cannot return an error
/// to their caller, but silently losing data output is not acceptable either. So the first
/// failure is stashed here and `QsvExitCode::report` — the single point every binary's `main`
/// funnels through — turns it into a message and a non-zero exit.
///
/// Diagnostic writes (stderr) deliberately do NOT set this.
pub static WRITE_ERROR: OnceLock<String> = OnceLock::new();

/// Record a failed data-stream write. Only the FIRST is kept: whatever breaks one write
/// (a full device, a closed fd) breaks every subsequent one too, and one accurate message
/// beats several thousand identical ones.
pub fn record_write_error(stream: &str, e: &io::Error) {
    let _ = WRITE_ERROR.set(format!("error writing to {stream}: {e}"));
}

#[repr(u8)]
pub enum QsvExitCode {
    Good           = 0,
    Bad            = 1,
    IncorrectUsage = 2,
    NetworkError   = 3,
    OutOfMemory    = 4,
    EncodingError  = 5,
    Warning        = 255,
}

/// The status a run should actually report, given whether a data-stream write failed.
///
/// A run that would otherwise have SUCCEEDED must not claim success when its output never
/// made it; an already-failing run keeps its more specific code. Split out from `report` so
/// the promotion rule is testable without a real `ExitCode` or the process-global
/// `WRITE_ERROR`.
const fn effective_exit_code(code: u8, had_write_error: bool) -> u8 {
    if had_write_error && code == QsvExitCode::Good as u8 {
        QsvExitCode::Bad as u8
    } else {
        code
    }
}

impl Termination for QsvExitCode {
    fn report(self) -> ExitCode {
        // Surface a data-stream write failure that `wout!`/`woutinfo!` recorded rather than
        // panicked on. This is the one place all three binaries' `main` funnels through, so
        // the check cannot be forgotten by a new exit path.
        let write_error = WRITE_ERROR.get();
        if let Some(err) = write_error {
            use std::io::Write;

            log::error!("{err}");
            let _ = writeln!(&mut io::stderr(), "{err}");
        }
        ExitCode::from(effective_exit_code(self as u8, write_error.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::{QsvExitCode, effective_exit_code};

    // A failed data write must never be reported as success — and must never mask a more
    // specific failure that already happened.
    #[test]
    fn a_failed_data_write_promotes_only_a_successful_run() {
        // clean run, working stdout => unchanged
        assert_eq!(
            effective_exit_code(QsvExitCode::Good as u8, false),
            QsvExitCode::Good as u8
        );
        // clean run whose output never landed => must fail
        assert_eq!(
            effective_exit_code(QsvExitCode::Good as u8, true),
            QsvExitCode::Bad as u8
        );
        // an already-failing run keeps its own, more informative code
        for code in [
            QsvExitCode::IncorrectUsage,
            QsvExitCode::NetworkError,
            QsvExitCode::OutOfMemory,
            QsvExitCode::EncodingError,
            QsvExitCode::Warning,
        ] {
            let code = code as u8;
            assert_eq!(effective_exit_code(code, true), code);
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(docopt::Error),
    Help(String),
    Csv(csv::Error),
    Io(io::Error),
    NoMatch(),
    IncorrectUsage(String),
    Network(String),
    OutOfMemory(String),
    Encoding(String),
    // An LLM inference/completion failure (HTTP/API error, empty response, etc.), as opposed
    // to an infrastructure error (cache backend, IO). Lets callers degrade gracefully on a
    // failed inference while still propagating infrastructure failures.
    Inference(String),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Flag(e) => e.fmt(f),
            CliError::Help(e) => e.fmt(f),
            CliError::Csv(e) => e.fmt(f),
            CliError::Io(e) => e.fmt(f),
            CliError::NoMatch() => f.write_str("no_match"),
            CliError::Other(s)
            | CliError::IncorrectUsage(s)
            | CliError::Encoding(s)
            | CliError::OutOfMemory(s)
            | CliError::Inference(s)
            | CliError::Network(s) => f.write_str(s),
        }
    }
}

impl From<docopt::Error> for CliError {
    fn from(err: docopt::Error) -> CliError {
        if let docopt::Error::WithProgramUsage(ref errtype, ref usage_text) = err {
            if matches!(**errtype, docopt::Error::Help) {
                CliError::Help(usage_text.to_string())
            } else {
                CliError::Flag(err)
            }
        } else {
            CliError::Flag(err)
        }
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        if !err.is_io_error() {
            return CliError::Csv(err);
        }
        if let csv::ErrorKind::Io(v) = err.into_kind() {
            From::from(v)
        } else {
            // safety: we checked for !is_io_error above
            unreachable!()
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> CliError {
        CliError::Other(err)
    }
}

impl<'a> From<&'a str> for CliError {
    fn from(err: &'a str) -> CliError {
        CliError::Other(err.to_owned())
    }
}

impl From<regex::Error> for CliError {
    fn from(err: regex::Error) -> CliError {
        CliError::Other(format!("Regex error: {err:?}"))
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> CliError {
        CliError::Other(format!("JSON error: {err:?}"))
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
        CliError::Network(err.to_string())
    }
}

#[cfg(feature = "polars")]
impl From<polars::error::PolarsError> for CliError {
    fn from(err: polars::error::PolarsError) -> CliError {
        CliError::Other(format!("Polars error: {err:?}"))
    }
}

impl From<flexi_logger::FlexiLoggerError> for CliError {
    fn from(err: flexi_logger::FlexiLoggerError) -> CliError {
        CliError::Other(format!("FlexiLogger error: {err:?}"))
    }
}

impl From<chrono_tz::ParseError> for CliError {
    fn from(err: chrono_tz::ParseError) -> CliError {
        CliError::Other(format!("ChronoTZ error: {err:?}"))
    }
}

impl From<simd_json::Error> for CliError {
    fn from(err: simd_json::Error) -> CliError {
        CliError::Other(format!("SimdJSON error: {err:?}"))
    }
}

impl From<zip::result::ZipError> for CliError {
    fn from(err: zip::result::ZipError) -> CliError {
        match err {
            zip::result::ZipError::Io(e) => CliError::Io(e),
            zip::result::ZipError::InvalidArchive(e) => {
                CliError::IncorrectUsage(format!("Zip error: {e:?}"))
            },
            zip::result::ZipError::FileNotFound => {
                CliError::IncorrectUsage("Zip error: zip archive not found.".to_string())
            },
            zip::result::ZipError::InvalidPassword => {
                CliError::IncorrectUsage("Zip error: password-protected zip file.".to_string())
            },
            _ => CliError::Other(format!("Zip error: {err:?}")),
        }
    }
}

impl From<RedbCacheError> for CliError {
    fn from(err: RedbCacheError) -> CliError {
        CliError::Other(format!("RedbCache error: {err:?}"))
    }
}

impl From<RedisCacheError> for CliError {
    fn from(err: RedisCacheError) -> CliError {
        CliError::Other(format!("RedisCache error: {err:?}"))
    }
}
