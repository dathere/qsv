//! Shared, data-wrangling MiniJinja filters/functions used across qsv's
//! MiniJinja-powered commands (`template`, `fetchpost`, `describegpt`,
//! `profile`).
//!
//! These fill real gaps that neither MiniJinja core, `minijinja-contrib`, nor
//! the command-specific qsv filters already cover:
//!   - regex (none exists anywhere in the engine): `regex_replace`, `regex_match`, `regex_find`
//!   - rounding modes the core `round` lacks: `floor`, `ceil`
//!   - messy-date parsing via qsv's own `qsv-dateparser`: `datefmt`
//!   - padding (pycompat has no `zfill`/`rjust`/`ljust`): `zfill`, `lpad`, `rpad`
//!   - URL/DB/CKAN-safe slugs: `slugify`
//!   - stable surrogate/content keys: `blake3`
//!   - JSON-in-a-cell parsing: `fromjson` (alias `parse_json`)
//!   - multi-arg first-non-empty: `coalesce` (function)
//!
//! All functions are pure and `Send + Sync`, so the single `Environment` that
//! `template` shares across rayon worker threads can call them concurrently.
//! There is no cargo-feature gate: every dependency used here (`regex`,
//! `blake3`, `qsv-dateparser`, `serde_json`) is always compiled in, so these
//! filters are available in every binary variant.

use std::{
    collections::HashMap,
    sync::{OnceLock, RwLock},
};

use minijinja::{Environment, Error, ErrorKind, Value, value::Rest};
use regex::Regex;

/// Register every shared qsv filter/function on `env`.
///
/// Mirrors the `register`-style entry points already used by `template`
/// (`register_qsv_extensions`) and `profile` (`formula_helpers::register`), so
/// each command adds these with a single call at environment-setup time.
pub fn register(env: &mut Environment) {
    env.add_filter("regex_replace", regex_replace);
    env.add_filter("regex_match", regex_match);
    env.add_filter("regex_find", regex_find);

    env.add_filter("floor", floor);
    env.add_filter("ceil", ceil);

    env.add_filter("datefmt", datefmt);

    env.add_filter("zfill", zfill);
    env.add_filter("lpad", lpad);
    env.add_filter("rpad", rpad);

    env.add_filter("slugify", slugify);
    env.add_filter("blake3", blake3_hex);

    env.add_filter("fromjson", fromjson);
    env.add_filter("parse_json", fromjson); // alias

    env.add_function("coalesce", coalesce);
}

// --- regex ---------------------------------------------------------------

// Runtime cache of compiled patterns. A template's patterns are constant, but
// the filter runs once per row, so compiling on every call would be wasteful.
// `Regex` is internally Arc-backed, so cloning out of the cache is cheap and
// lets us drop the read lock before matching (rayon threads never serialize on
// a held lock during the actual match).
//
// The cache is bounded: a pattern can come from row data (e.g.
// `{{ v|regex_match(pattern_column) }}`), so an unbounded cache could retain one
// compiled regex per distinct row and exhaust memory. Once the cache is full we
// stop inserting (still compiling on demand, just without caching), which keeps
// the common case of a handful of literal patterns fully cached while capping
// worst-case memory for data-dependent patterns.
static REGEX_CACHE: OnceLock<RwLock<HashMap<String, Regex>>> = OnceLock::new();
const REGEX_CACHE_MAX: usize = 256;

fn compiled(pattern: &str) -> Result<Regex, Error> {
    let cache = REGEX_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    if let Ok(map) = cache.read()
        && let Some(re) = map.get(pattern)
    {
        return Ok(re.clone());
    }
    let re = Regex::new(pattern).map_err(|e| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("invalid regex `{pattern}`: {e}"),
        )
    })?;
    if let Ok(mut map) = cache.write()
        && map.len() < REGEX_CACHE_MAX
    {
        map.insert(pattern.to_owned(), re.clone());
    }
    Ok(re)
}

fn regex_replace(value: &Value, pattern: &str, replacement: &str) -> Result<String, Error> {
    let s = value.to_string();
    Ok(compiled(pattern)?.replace_all(&s, replacement).into_owned())
}

fn regex_match(value: &Value, pattern: &str) -> Result<bool, Error> {
    Ok(compiled(pattern)?.is_match(&value.to_string()))
}

fn regex_find(value: &Value, pattern: &str) -> Result<String, Error> {
    let s = value.to_string();
    Ok(compiled(pattern)?
        .find(&s)
        .map_or_else(String::new, |m| m.as_str().to_owned()))
}

// --- numeric -------------------------------------------------------------

// Coerce a value (string or number) to f64. Mirrors the string-friendly
// behavior of `format_float`/`round_banker` in template.rs so users don't have
// to `|float` first.
fn as_f64(value: &Value) -> Result<f64, Error> {
    let s = value.to_string();
    s.trim().parse::<f64>().map_err(|_| {
        Error::new(
            ErrorKind::InvalidOperation,
            format!("expected a number, got `{s}`"),
        )
    })
}

// Return an integer (whole number) so `{{ "42.7"|floor }}` renders `42`, not
// `42.0`. NaN/infinity and values outside the i64 range surface a template
// error rather than silently saturating to 0/i64::MIN/i64::MAX from an `as`
// cast.
fn to_i64(rounded: f64) -> Result<i64, Error> {
    if rounded.is_finite() && (i64::MIN as f64..=i64::MAX as f64).contains(&rounded) {
        Ok(rounded as i64)
    } else {
        Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("value `{rounded}` is not a finite integer in i64 range"),
        ))
    }
}

fn floor(value: &Value) -> Result<i64, Error> {
    to_i64(as_f64(value)?.floor())
}

fn ceil(value: &Value) -> Result<i64, Error> {
    to_i64(as_f64(value)?.ceil())
}

// --- dates ---------------------------------------------------------------

// Parse a messy date/datetime string (qsv-dateparser recognizes 19+ formats)
// and reformat with a chrono format string. Unlike minijinja-contrib's
// `dateformat`, this PARSES arbitrary strings rather than formatting an
// already-typed date.
fn datefmt(value: &Value, fmt: &str, prefer_dmy: Option<bool>) -> Result<String, Error> {
    let s = value.to_string();
    let dt = qsv_dateparser::parse_with_preference(&s, prefer_dmy.unwrap_or(false))
        .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("datefmt: {e}")))?;
    Ok(dt.format(fmt).to_string())
}

// --- padding -------------------------------------------------------------

// Zero-fill to `width`, Python-style: a leading sign stays in front of the
// zeros (e.g. "-7"|zfill(4) -> "-007").
fn zfill(value: &Value, width: usize) -> String {
    let s = value.to_string();
    let (sign, digits) = match s.strip_prefix(['-', '+']) {
        Some(rest) => (&s[..1], rest),
        None => ("", s.as_str()),
    };
    let pad = width.saturating_sub(sign.len() + digits.len());
    format!("{sign}{}{digits}", "0".repeat(pad))
}

fn lpad(value: &Value, width: usize, fill: Option<char>) -> String {
    let s = value.to_string();
    let pad = width.saturating_sub(s.chars().count());
    format!("{}{s}", fill.unwrap_or(' ').to_string().repeat(pad))
}

fn rpad(value: &Value, width: usize, fill: Option<char>) -> String {
    let s = value.to_string();
    let pad = width.saturating_sub(s.chars().count());
    format!("{s}{}", fill.unwrap_or(' ').to_string().repeat(pad))
}

// --- slug & hash ---------------------------------------------------------

fn slug_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"[^a-z0-9]+").unwrap())
}

// Lowercase, collapse runs of non-alphanumeric characters to a single hyphen,
// and trim leading/trailing hyphens. e.g. "NYC 311 Data!" -> "nyc-311-data".
fn slugify(value: &Value) -> String {
    let lower = value.to_string().to_lowercase();
    slug_re()
        .replace_all(&lower, "-")
        .trim_matches('-')
        .to_owned()
}

fn blake3_hex(value: &Value) -> String {
    blake3::hash(value.to_string().as_bytes())
        .to_hex()
        .to_string()
}

// --- json & coalesce -----------------------------------------------------

// Parse a JSON string (typically a CSV cell holding JSON) into a value that
// can be indexed in the template, e.g. `{{ (meta|fromjson).author }}`.
fn fromjson(value: &Value) -> Result<Value, Error> {
    let s = value.to_string();
    let parsed: serde_json::Value = serde_json::from_str(&s)
        .map_err(|e| Error::new(ErrorKind::InvalidOperation, format!("fromjson: {e}")))?;
    Ok(Value::from_serialize(&parsed))
}

// Return the first argument that is neither undefined, none, nor an empty
// string. Broader than the single-fallback `default`/`d` builtin.
fn coalesce(args: Rest<Value>) -> Value {
    args.iter()
        .find(|v| !v.is_undefined() && !v.is_none() && !v.as_str().is_some_and(str::is_empty))
        .cloned()
        .unwrap_or(Value::UNDEFINED)
}
