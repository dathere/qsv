use std::sync::OnceLock;
#[cfg(feature = "self_update")]
use std::time::Duration;

use docopt::Docopt;
use serde::de::DeserializeOwned;

use crate::{CliError, CliResult, config::Delimiter, select::SelectColumns};

#[macro_export]
macro_rules! regex_oncelock {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        #[allow(clippy::regex_creation_in_loops)] // false positive as we use oncelock
        RE.get_or_init(|| regex::Regex::new($re).expect("Invalid regex"))
    }};
}

// leave at least 20% of the available memory free
const DEFAULT_FREEMEMORY_HEADROOM_PCT: u8 = 20;

// safety margin for memory-aware chunking
// (uses 80% of available memory to leave headroom for system operations & other processes)
pub const SAFETY_MARGIN: f64 = 0.8;

const DEFAULT_BATCH_SIZE: usize = 50_000;

const DEFAULT_STATSCACHE_MODE: &str = "auto";

static ROW_COUNT: OnceLock<Option<u64>> = OnceLock::new();

static JOBS_TO_USE: OnceLock<usize> = OnceLock::new();

pub static QUIET_FLAG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Set once at startup by `init_allocator_runtime()` when jemalloc's
// `background_thread` is successfully enabled (Linux; rejected on macOS). When
// true, jemalloc purges freed pages on background threads at no RSS cost, so the
// per-command page-retention lever (`retain_alloc_pages_for_aggregation`) is
// skipped to avoid paying its higher RSS for no additional benefit.
// Only read under the jemalloc-gated retain path / version string; harmless
// elsewhere.
#[allow(dead_code)]
pub static BACKGROUND_THREADS_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

// only consumed by non-lite commands (describegpt, lens, python, luau)
#[cfg(not(feature = "lite"))]
pub static FILE_PATH_PREFIX: &str = "file:";

pub type ByteString = Vec<u8>;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatsMode {
    Schema,
    /// Like `Schema`, but additionally computes quartiles (q1/median/q3)
    /// and mode. Used by `qsv profile` so its descriptive-statistics
    /// projection (e.g. the Croissant annotations) can surface the full
    /// extended stat set on a fresh run without a pre-built `--everything`
    /// stats cache.
    ProfileSchema,
    Frequency,
    FrequencyForceStats,
    #[cfg(feature = "polars")]
    PolarsSchema,
    Outliers,
    None,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct SchemaArgs {
    pub flag_enum_threshold:  u64,
    pub flag_ignore_case:     bool,
    pub flag_strict_dates:    bool,
    pub flag_strict_formats:  bool,
    pub flag_pattern_columns: SelectColumns,
    pub flag_dates_whitelist: String,
    pub flag_prefer_dmy:      bool,
    pub flag_force:           bool,
    pub flag_stdout:          bool,
    pub flag_jobs:            Option<usize>,
    pub flag_polars:          bool,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
    pub arg_input:            Option<String>,
    pub flag_memcheck:        bool,
    pub flag_output:          Option<String>,
}

#[inline]
pub fn num_cpus() -> usize {
    num_cpus::get()
}

static QSV_PATH: OnceLock<String> = OnceLock::new();

pub const CARGO_BIN_NAME: &str = env!("CARGO_BIN_NAME");
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const TARGET: &str = match option_env!("TARGET") {
    Some(target) => target,
    None => "Unknown_target",
};
const QSV_KIND: &str = match option_env!("QSV_KIND") {
    Some(kind) => kind,
    None => "installed",
};

#[cfg(feature = "polars")]
const QSV_POLARS_REV: &str = match option_env!("QSV_POLARS_REV") {
    Some(rev) => rev,
    None => "",
};

// Add constant for whitespace visualization
// the whitespace markers as as defined in
// https://doc.rust-lang.org/reference/whitespace.html
const WHITESPACE_MARKERS: &[(char, &str)] = &[
    // common whitespace markers other than space
    ('\t', "《→》"), // tab
    ('\n', "《¶》"), // newline
    ('\r', "《⏎》"), // carriage return
    // more obscure whitespace markers
    ('\u{000B}', "《⋮》"), // vertical tab
    ('\u{000C}', "《␌》"), // form feed
    // note: the horizontal tab (U+0009) is the same code point as '\t' above,
    // so it is intentionally not repeated here (the '\t' entry already covers it).
    ('\u{0085}', "《␤》"), // next line
    ('\u{200E}', "《␎》"), // left-to-right mark
    ('\u{200F}', "《␏》"), // right-to-left mark
    ('\u{2028}', "《␊》"), // line separator
    ('\u{2029}', "《␍》"), // paragraph separator
    // additional common whitespace markers beyond
    // https://doc.rust-lang.org/reference/whitespace.html
    ('\u{00A0}', "《⍽》"),     // non-breaking space
    ('\u{2003}', "《emsp》"),  // em space
    ('\u{2007}', "《figsp》"), // figure space
    ('\u{200B}', "《zwsp》"),  // zero width space
];

const OTHER_ENV_VARS: &[&str] = &["all_proxy", "no_proxy", "http_proxy", "https_proxy"];

#[cfg(all(feature = "fetch", not(feature = "lite")))]
macro_rules! update_cache_info {
    ($progress:expr_2021, $cache_instance:expr_2021) => {
        use cached::Cached;
        use indicatif::HumanCount;

        let cache = $cache_instance.read();
        let size = cache.cache_size();
        if size > 0 {
            let hits = cache.cache_hits().unwrap_or_default();
            let misses = cache.cache_misses().unwrap_or(1);
            #[allow(clippy::cast_precision_loss)]
            let hit_ratio = (hits as f64 / (hits + misses) as f64) * 100.0;
            let capacity = cache.cache_capacity();
            drop(cache);
            $progress.set_message(format!(
                " of {} records. Cache {:.2}% entries: {} capacity: {}.",
                HumanCount($progress.length().unwrap()),
                hit_ratio,
                HumanCount(size as u64),
                HumanCount(capacity.unwrap() as u64),
            ));
        }
    };
    ($progress:expr_2021, $cache_hits:expr_2021, $num_rows:expr_2021) => {
        use indicatif::HumanCount;

        #[allow(clippy::cast_precision_loss)]
        let hit_ratio = ($cache_hits as f64 / $num_rows as f64) * 100.0;
        $progress.set_message(format!(
            " of {} records. Cache hit ratio: {hit_ratio:.2}%",
            HumanCount($progress.length().unwrap()),
        ));
    };
}

#[cfg(all(feature = "fetch", not(feature = "lite")))]
pub(crate) use update_cache_info;

pub fn get_args<T>(usage: &str, argv: &[&str]) -> CliResult<T>
where
    T: DeserializeOwned,
{
    // `Docopt::deserialize` is just `parse().and_then(ArgvMap::deserialize)`, so splitting
    // it costs nothing and lets the guard see docopt's PARSED values. `parse` is also
    // where --help/--version are detected, so both still short-circuit ahead of the guard.
    let vals = Docopt::new(usage)
        .and_then(|d| {
            d.argv(argv.iter().copied())
                .version(Some(version()))
                .parse()
        })
        .map_err(CliError::from)?;

    check_output_is_not_input(&vals, argv)?;

    vals.deserialize().map_err(CliError::from)
}

pub type Idx = Option<usize>;

// how often a background (non-flag) update check is allowed to hit GitHub: ~twice a month
#[cfg(feature = "self_update")]
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_hours(360);

/// Tabular file extensions qsv recognizes for delimiter inference inside a zip.
const ZIP_TABULAR_EXTS: [&str; 4] = ["csv", "tsv", "tab", "ssv"];

mod args;
mod derived_file;
mod envvars;
mod headers;
mod http;
mod index;
mod input;
mod json_out;
mod rows;
mod runtime;
mod special_format;
mod stats_cache;
mod subprocess;
mod update;
mod version;

pub use args::*;
pub use derived_file::*;
pub use envvars::*;
pub use headers::*;
pub use http::*;
pub use index::*;
pub use input::*;
pub use json_out::*;
pub use rows::*;
pub use runtime::*;
pub use special_format::*;
pub use stats_cache::*;
pub use subprocess::*;
pub use update::*;
pub use version::*;

#[cfg(test)]
mod tests;
