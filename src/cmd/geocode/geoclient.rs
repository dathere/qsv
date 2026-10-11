//! NYC Geoclient v2 backend for `qsv geocode geoclient` / `geoclientnow`.
//!
//! Geoclient (<https://github.com/mlipper/geoclient>) is a REST wrapper around NYC DCP's
//! Geosupport. Facts this module relies on (verified against upstream docs, samples and source):
//! - one GET per location — there is no batch endpoint;
//! - the public gateway authenticates with the `Ocp-Apim-Subscription-Key` HEADER, so the key never
//!   appears in a URL; a self-hosted instance has no auth at all;
//! - HTTP 200 does not mean "matched": success is `geosupportReturnCode` (or, for the two-function
//!   1B calls behind `/address` and `/search`, `geosupportReturnCode2`) being `00` or `01`;
//! - `/bbl` and `/bin` return no `latitude`/`longitude`, only the `*InternalLabel` pair;
//! - `/search` result `status` values beyond `EXACT_MATCH` are undocumented, so matching keys on
//!   the return codes, never on that status.

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU32,
    path::Path,
    time::Duration,
};

use cached::{ConcurrentCached, RedbCache};
use futures::StreamExt;
use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use indicatif::{ProgressBar, ProgressDrawTarget};
use serde_json::Value;
use uuid::Uuid;

use super::{
    Args, OPENCAGE_FORMATSTR_REGEX, add_fields, decode_lookup_cache, encode_lookup_cache,
    json_dotpath, parse_dyncols_spec,
};
use crate::{
    CliError, CliResult,
    config::Config,
    select::SelectColumns,
    util::{self, replace_column_value},
};

/// The public Geoclient v2 gateway. `QSV_GEOCLIENT_API_URL` overrides it (self-hosting, tests).
pub(super) const DEFAULT_GEOCLIENT_API_URL: &str = "https://api.nyc.gov/geoclient/v2";

/// The `cached` crate names the redb file after this, under the cache dir.
pub(super) const GEOCLIENT_CACHE_NAME: &str = "geocode-geoclient";

/// Default requests/second: under the public gateway's 2,500/minute cap (41.6/s).
pub(super) const DEFAULT_GEOCLIENT_RATE_LIMIT: u32 = 40;

/// Responses are a few KB; anything bigger is a broken or hostile server (or a compression bomb).
const MAX_BODY_BYTES: usize = 4 * 1024 * 1024;

/// Retries after the first attempt, for 429/408/5xx/network errors.
const MAX_RETRIES: u32 = 3;

/// Consecutive transient failures that abort the run: once the daily quota is gone every request
/// is refused, and grinding retries through the rest of a large file helps nobody.
const CIRCUIT_BREAKER_THRESHOLD: u32 = 10;

const SUBSCRIPTION_KEY_HEADER: &str = "Ocp-Apim-Subscription-Key";

/// Resolve the Geoclient base URL (trailing slashes trimmed) from `QSV_GEOCLIENT_API_URL`.
pub(super) fn geoclient_base_url() -> String {
    std::env::var("QSV_GEOCLIENT_API_URL")
        .ok()
        .map(|u| u.trim().trim_end_matches('/').to_string())
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| DEFAULT_GEOCLIENT_API_URL.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Endpoint {
    Search,
    Bin,
    Bbl,
    Address,
    AddressPoint,
    Intersection,
    Place,
    Blockface,
}

impl Endpoint {
    pub(super) fn parse(value: &str) -> CliResult<Self> {
        Ok(match value.trim().to_ascii_lowercase().as_str() {
            "search" => Self::Search,
            "bin" => Self::Bin,
            "bbl" => Self::Bbl,
            "address" => Self::Address,
            "addresspoint" => Self::AddressPoint,
            "intersection" => Self::Intersection,
            "place" => Self::Place,
            "blockface" => Self::Blockface,
            other => {
                return fail_incorrectusage_clierror!(
                    "Invalid --endpoint {other:?}. Valid endpoints are: search, bin, bbl, \
                     address, addresspoint, intersection, place and blockface."
                );
            },
        })
    }

    /// The URL path segment, which is also the key the response is wrapped in.
    const fn path(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Bin => "bin",
            Self::Bbl => "bbl",
            Self::Address => "address",
            Self::AddressPoint => "addresspoint",
            Self::Intersection => "intersection",
            Self::Place => "place",
            Self::Blockface => "blockface",
        }
    }

    /// Structured endpoints take their inputs from `--params`; the others from `<column>`.
    pub(super) const fn is_structured(self) -> bool {
        !matches!(self, Self::Search | Self::Bin | Self::Bbl)
    }

    /// (required, optional) query parameters. A required entry with `|` means "one of these".
    /// Parameter names are case-sensitive in the Geoclient API.
    const fn param_spec(self) -> (&'static [&'static str], &'static [&'static str]) {
        match self {
            Self::Search => (
                &[],
                &[
                    "exactMatchForSingleSuccess",
                    "exactMatchMaxLevel",
                    "returnPolicy",
                    "returnPossiblesWithExact",
                    "returnRejections",
                    "returnTokens",
                    "similarNamesDistance",
                ],
            ),
            Self::Bin | Self::Bbl => (&[], &[]),
            Self::Address | Self::AddressPoint => (&["houseNumber", "street", "borough|zip"], &[]),
            Self::Intersection => (
                &["crossStreetOne", "crossStreetTwo", "borough"],
                &["boroughCrossStreetTwo", "compassDirection"],
            ),
            Self::Place => (&["name", "borough|zip"], &[]),
            Self::Blockface => (
                &["onStreet", "crossStreetOne", "crossStreetTwo", "borough"],
                &[
                    "boroughCrossStreetOne",
                    "boroughCrossStreetTwo",
                    "compassDirection",
                ],
            ),
        }
    }

    fn allowed_params(self) -> Vec<&'static str> {
        let (required, optional) = self.param_spec();
        required
            .iter()
            .flat_map(|r| r.split('|'))
            .chain(optional.iter().copied())
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ParamValue {
    Column(String),
    Literal(String),
}

/// Parse and validate `--params` ("name={Column},name=literal,...") for `endpoint`.
/// Header-dependent checks (does `{Column}` exist?) happen later in [`resolve_param_sources`].
pub(super) fn parse_params(
    endpoint: Endpoint,
    template: Option<&str>,
) -> CliResult<Vec<(String, ParamValue)>> {
    let allowed = endpoint.allowed_params();
    let mut params: Vec<(String, ParamValue)> = Vec::new();
    for entry in template.unwrap_or_default().split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let Some((name, value)) = entry.split_once('=') else {
            return fail_incorrectusage_clierror!(
                "Invalid --params entry {entry:?}: expected name={{Column}} or name=value."
            );
        };
        let (name, value) = (name.trim(), value.trim());
        if !allowed.contains(&name) {
            return fail_incorrectusage_clierror!(
                "Invalid --params name {name:?} for --endpoint {}. Valid names (case-sensitive) \
                 are: {}.",
                endpoint.path(),
                if allowed.is_empty() {
                    "none".to_string()
                } else {
                    allowed.join(", ")
                }
            );
        }
        if params.iter().any(|(n, _)| n == name) {
            return fail_incorrectusage_clierror!("--params names {name:?} more than once.");
        }
        let value = if let Some(column) = value.strip_prefix('{').and_then(|v| v.strip_suffix('}'))
        {
            let column = column.trim();
            if column.is_empty() {
                return fail_incorrectusage_clierror!(
                    "Invalid --params entry {entry:?}: the column name is empty."
                );
            }
            if !endpoint.is_structured() {
                return fail_incorrectusage_clierror!(
                    "--endpoint {} takes its input from <column>; its --params may only set \
                     literal options, not {{Column}} references ({entry:?}).",
                    endpoint.path()
                );
            }
            ParamValue::Column(column.to_string())
        } else if value.is_empty() {
            return fail_incorrectusage_clierror!(
                "Invalid --params entry {entry:?}: the value is empty."
            );
        } else {
            ParamValue::Literal(value.to_string())
        };
        params.push((name.to_string(), value));
    }

    if endpoint.is_structured() {
        if params.is_empty() {
            return fail_incorrectusage_clierror!(
                "--endpoint {} requires --params, e.g. {}.",
                endpoint.path(),
                params_example(endpoint)
            );
        }
        let (required, _) = endpoint.param_spec();
        for req in required {
            if !req.split('|').any(|r| params.iter().any(|(n, _)| n == r)) {
                return fail_incorrectusage_clierror!(
                    "--endpoint {} requires the {} parameter in --params, e.g. {}.",
                    endpoint.path(),
                    req.replace('|', " or "),
                    params_example(endpoint)
                );
            }
        }
    }
    Ok(params)
}

const fn params_example(endpoint: Endpoint) -> &'static str {
    match endpoint {
        Endpoint::Address | Endpoint::AddressPoint => {
            "\"houseNumber={HouseNo},street={Street},borough={Borough}\""
        },
        Endpoint::Intersection => {
            "\"crossStreetOne={Street1},crossStreetTwo={Street2},borough={Borough}\""
        },
        Endpoint::Place => "\"name={Place},borough={Borough}\"",
        Endpoint::Blockface => {
            "\"onStreet={On},crossStreetOne={From},crossStreetTwo={To},borough={Borough}\""
        },
        Endpoint::Search | Endpoint::Bin | Endpoint::Bbl => "\"exactMatchMaxLevel=5\"",
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ParamSource {
    Column(usize),
    Literal(String),
}

/// Map each `{Column}` reference to its index in `headers` (exact, case-sensitive match).
fn resolve_param_sources(
    params: &[(String, ParamValue)],
    headers: &csv::StringRecord,
) -> CliResult<Vec<(String, ParamSource)>> {
    params
        .iter()
        .map(|(name, value)| {
            let source = match value {
                ParamValue::Literal(lit) => ParamSource::Literal(lit.clone()),
                ParamValue::Column(column) => match headers.iter().position(|h| h == column) {
                    Some(idx) => ParamSource::Column(idx),
                    None => {
                        return fail_incorrectusage_clierror!(
                            "--params references column {column:?}, which is not in the input \
                             headers."
                        );
                    },
                },
            };
            Ok((name.clone(), source))
        })
        .collect()
}

/// Drop a spreadsheet/pandas float tail ("1000670001.0") from an identifier.
fn strip_float_zeros(value: &str) -> &str {
    match value.split_once('.') {
        Some((int, frac)) if !frac.is_empty() && frac.bytes().all(|b| b == b'0') => int,
        _ => value,
    }
}

/// Split a BBL into (borough, block, lot). Accepts the 10-digit form ("1000670001", also with a
/// float tail) and the delimited form ("1-67-1", "1/00067/0001"), zero-padding block and lot.
fn split_bbl(cell: &str) -> Option<(String, String, String)> {
    let cell = strip_float_zeros(cell.trim());
    let (boro, block, lot) = if cell.contains(['-', '/']) {
        let parts: Vec<&str> = cell.split(['-', '/']).map(str::trim).collect();
        let [boro, block, lot] = parts.as_slice() else {
            return None;
        };
        if boro.len() != 1 || block.is_empty() || block.len() > 5 || lot.is_empty() || lot.len() > 4
        {
            return None;
        }
        (
            (*boro).to_string(),
            format!("{block:0>5}"),
            format!("{lot:0>4}"),
        )
    } else {
        if cell.len() != 10 {
            return None;
        }
        (
            cell[..1].to_string(),
            cell[1..6].to_string(),
            cell[6..].to_string(),
        )
    };
    let all_digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if !all_digits(&boro) || !all_digits(&block) || !all_digits(&lot) {
        return None;
    }
    if !matches!(boro.as_str(), "1" | "2" | "3" | "4" | "5") {
        return None;
    }
    Some((boro, block, lot))
}

/// Collapse internal whitespace runs and trim.
fn squish(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One row's request: its cache key and query parameters, or `None` when the row cannot be
/// looked up (empty input, malformed BBL, missing required structured params).
struct Request {
    key:    String,
    params: Vec<(String, String)>,
}

fn prepare_request(
    endpoint: Endpoint,
    base_url: &str,
    cell: &str,
    record: &csv::StringRecord,
    sources: &[(String, ParamSource)],
) -> Option<Request> {
    let mut params: Vec<(String, String)> = Vec::new();
    match endpoint {
        Endpoint::Search => {
            let input = squish(cell);
            if input.is_empty() {
                return None;
            }
            params.push(("input".to_string(), input));
        },
        Endpoint::Bin => {
            let bin = strip_float_zeros(cell.trim());
            if bin.is_empty() {
                return None;
            }
            params.push(("bin".to_string(), bin.to_string()));
        },
        Endpoint::Bbl => {
            let (borough, block, lot) = split_bbl(cell)?;
            params.push(("borough".to_string(), borough));
            params.push(("block".to_string(), block));
            params.push(("lot".to_string(), lot));
        },
        _ => {},
    }
    for (name, source) in sources {
        let value = match source {
            ParamSource::Literal(lit) => lit.clone(),
            ParamSource::Column(idx) => squish(record.get(*idx).unwrap_or_default()),
        };
        // an empty optional cell is simply not sent; required params are checked below
        if !value.is_empty() {
            params.push((name.clone(), value));
        }
    }
    let (required, _) = endpoint.param_spec();
    if !required
        .iter()
        .all(|req| req.split('|').any(|r| params.iter().any(|(n, _)| n == r)))
    {
        return None;
    }

    // Geoclient values are case-insensitive (names are not), so the key folds value case
    let mut key_parts: Vec<String> = params
        .iter()
        .map(|(n, v)| format!("{n}={}", v.to_uppercase()))
        .collect();
    key_parts.sort_unstable();
    let key = format!("{base_url}|{}|{}", endpoint.path(), key_parts.join("&"));
    Some(Request { key, params })
}

/// Error from a Geoclient request. `Fatal` aborts the run (bad key, wrong base URL);
/// `Rejected` is a per-row HTTP 400 (bad parameters); `Transient` is a per-row failure
/// (network error, 429, 5xx) left after retries.
#[derive(Debug)]
enum GcError {
    Fatal(String),
    Rejected(String),
    Transient(String),
}

/// Is a Geosupport return code a success? `00` = success, `01` = success with a warning.
fn rc_ok(obj: &Value, key: &str) -> bool {
    matches!(
        obj.get(key).and_then(Value::as_str).map(str::trim),
        Some("00" | "01")
    )
}

/// Pull the geocoded object out of a response body, or `None` for a non-match.
///
/// Every endpoint wraps its object under its own name (`{"bbl": {...}}`) except `/search`
/// (`{"results": [{"response": {...}}, ...]}`), where the first result whose return codes
/// succeed is taken. `latitude`/`longitude` are filled from the `*InternalLabel` pair when absent
/// (always, for `/bbl` and `/bin`), so `{latitude}` and `%lat-long` work for every endpoint.
fn extract_result(endpoint: Endpoint, body: &Value) -> Option<Value> {
    let is_success = |obj: &Value| {
        obj.is_object()
            && (rc_ok(obj, "geosupportReturnCode") || rc_ok(obj, "geosupportReturnCode2"))
    };
    let mut obj = if endpoint == Endpoint::Search {
        body.get("results")?
            .as_array()?
            .iter()
            .filter_map(|r| r.get("response"))
            .find(|r| is_success(r))?
            .clone()
    } else {
        let obj = body.get(endpoint.path())?;
        if !is_success(obj) {
            return None;
        }
        obj.clone()
    };
    if let Some(map) = obj.as_object_mut() {
        for (coord, label) in [
            ("latitude", "latitudeInternalLabel"),
            ("longitude", "longitudeInternalLabel"),
        ] {
            if map.get(coord).is_none_or(Value::is_null)
                && let Some(v) = map.get(label).cloned()
            {
                map.insert(coord.to_string(), v);
            }
        }
    }
    Some(obj)
}

/// Read a response body, refusing more than `cap` bytes (decompressed) instead of buffering it all.
async fn read_capped(mut resp: reqwest::Response, cap: usize) -> Result<Vec<u8>, String> {
    if resp.content_length().is_some_and(|n| n > cap as u64) {
        return Err(format!("response body is larger than {cap} bytes"));
    }
    let mut buf = Vec::new();
    while let Some(chunk) = resp.chunk().await.map_err(|e| e.to_string())? {
        if buf.len() + chunk.len() > cap {
            return Err(format!("response body is larger than {cap} bytes"));
        }
        buf.extend_from_slice(&chunk);
    }
    Ok(buf)
}

fn retry_base() -> Duration {
    let ms = std::env::var("QSV_GEOCLIENT_RETRY_BASE_MS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(1000);
    Duration::from_millis(ms)
}

/// GET one location, retrying 408/429/5xx/network errors with exponential backoff (honoring a
/// `Retry-After` in seconds, capped at 60s). Returns the extracted result, `None` for a non-match.
async fn geoclient_fetch(
    client: &reqwest::Client,
    limiter: &DefaultDirectRateLimiter,
    base_url: &str,
    api_key: Option<&str>,
    endpoint: Endpoint,
    params: &[(String, String)],
    retry_base: Duration,
) -> Result<Option<Value>, GcError> {
    let url = format!("{base_url}/{}", endpoint.path());
    let mut last_error = String::new();
    for attempt in 0..=MAX_RETRIES {
        limiter.until_ready().await;
        let mut request = client.get(&url).query(params);
        if let Some(key) = api_key {
            request = request.header(SUBSCRIPTION_KEY_HEADER, key);
        }
        let mut retry_after: Option<Duration> = None;
        match request.send().await {
            Err(e) => {
                last_error = util::redact_api_key(&format!("Geoclient network error: {e}"));
            },
            Ok(resp) => {
                let status = resp.status().as_u16();
                match status {
                    200 => {
                        let bytes = read_capped(resp, MAX_BODY_BYTES).await.map_err(|e| {
                            GcError::Transient(util::redact_api_key(&format!(
                                "Geoclient response error: {e}"
                            )))
                        })?;
                        let body: Value = serde_json::from_slice(&bytes).map_err(|e| {
                            GcError::Transient(format!("Geoclient response is not JSON: {e}"))
                        })?;
                        return Ok(extract_result(endpoint, &body));
                    },
                    401 | 403 => {
                        return Err(GcError::Fatal(format!(
                            "Geoclient rejected the request (HTTP {status}): check --api-key or \
                             the QSV_GEOCLIENT_API_KEY environment variable, and that your key \
                             is subscribed to \"Geoclient V2 User\" at https://api-portal.nyc.gov."
                        )));
                    },
                    404 => {
                        return Err(GcError::Fatal(format!(
                            "Geoclient endpoint not found (HTTP 404) at {url}. Check \
                             QSV_GEOCLIENT_API_URL - it should end in /geoclient/v2."
                        )));
                    },
                    400 => {
                        let body = read_capped(resp, 4096).await.unwrap_or_default();
                        let body: String =
                            String::from_utf8_lossy(&body).chars().take(200).collect();
                        return Err(GcError::Rejected(format!(
                            "Geoclient rejected the parameters (HTTP 400): {body}"
                        )));
                    },
                    408 | 429 | 500..=599 => {
                        retry_after = resp
                            .headers()
                            .get(reqwest::header::RETRY_AFTER)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.trim().parse::<u64>().ok())
                            .map(|secs| Duration::from_secs(secs.min(60)));
                        last_error = format!("Geoclient returned HTTP {status}.");
                    },
                    _ => {
                        return Err(GcError::Transient(format!(
                            "Geoclient returned HTTP {status}."
                        )));
                    },
                }
            },
        }
        if attempt < MAX_RETRIES {
            let backoff = retry_after.unwrap_or_else(|| retry_base * 2u32.pow(attempt));
            tokio::time::sleep(backoff.min(Duration::from_secs(60))).await;
        }
    }
    Err(GcError::Transient(last_error))
}

/// Format a Geoclient result per `formatstr`: a predefined %-format or a `{field}` template.
/// String values are trimmed (Geosupport pads fields such as `censusTract2020`); a field the
/// response omits (Geoclient drops null fields) renders empty.
fn format_geoclient_result(obj: &Value, formatstr: &str) -> String {
    let field = |key: &str| json_dotpath(obj, key).trim().to_string();
    let lat_long = || {
        let (lat, lng) = (field("latitude"), field("longitude"));
        if lat.is_empty() || lng.is_empty() {
            None
        } else {
            Some((lat, lng))
        }
    };
    if formatstr.starts_with('%') {
        match formatstr {
            "%location" => lat_long()
                .map(|(lat, lng)| format!("({lat}, {lng})"))
                .unwrap_or_default(),
            "%bbl" => field("bbl"),
            "%bin" => field("buildingIdentificationNumber"),
            "%json" => serde_json::to_string(obj).unwrap_or_else(|_| "null".to_string()),
            "%pretty-json" => {
                serde_json::to_string_pretty(obj).unwrap_or_else(|_| "null".to_string())
            },
            // "%+", "%lat-long" and any unknown %-format
            _ => lat_long()
                .map(|(lat, lng)| format!("{lat}, {lng}"))
                .unwrap_or_default(),
        }
    } else {
        OPENCAGE_FORMATSTR_REGEX()
            .replace_all(formatstr, |caps: &regex::Captures| field(&caps[1]))
            .into_owned()
    }
}

/// What a row resolved to.
enum RowOutcome {
    /// Nothing to look up (empty input, malformed BBL, missing required params).
    NoQuery,
    /// The lookup failed (rejected or transient error) - not cached.
    Failed,
    /// No match.
    NoMatch,
    /// Matched: the rendered output value(s) - one, or one per `%dyncols:` column.
    Hit(Vec<String>),
}

/// Geocode NYC locations via the Geoclient v2 API.
///
/// Rows are read in `--batch`-sized batches. Within a batch, rows are turned into requests, the
/// requests are de-duplicated and checked against the on-disk cache, and the remainder are sent
/// concurrently (`--jobs` in flight, all behind one rate limiter) with order-preserving
/// `buffered`, so output order matches input order.
#[allow(clippy::future_not_send)]
pub(super) async fn run_geoclient(args: Args, now_cmd: bool, cache_dir: &Path) -> CliResult<()> {
    let endpoint = Endpoint::parse(args.flag_endpoint.as_deref().unwrap_or("search"))?;
    let params = parse_params(endpoint, args.flag_params.as_deref())?;
    let base_url = geoclient_base_url();
    let api_key = args.flag_api_key.as_deref().filter(|k| !k.is_empty());

    let tempdir = tempfile::Builder::new().prefix("qsv-geocode").tempdir()?;
    let input = if now_cmd {
        let temp_csv_path = format!(
            "{}/{}.csv",
            tempdir.path().to_string_lossy(),
            Uuid::new_v4()
        );
        let mut temp_csv_wtr = csv::WriterBuilder::new().from_path(&temp_csv_path)?;
        temp_csv_wtr.write_record(["Location"])?;
        temp_csv_wtr.write_record([&args.arg_location])?;
        temp_csv_wtr.flush()?;
        Some(temp_csv_path)
    } else {
        args.arg_input.clone()
    };

    let rconfig = Config::new(input.as_ref())
        .delimiter(args.flag_delimiter)
        .select(SelectColumns::parse(&args.arg_column)?);

    let dyncols_mode = args.flag_formatstr.starts_with("%dyncols:");
    let (column_names, column_keys) = if dyncols_mode {
        parse_dyncols_spec(&args.flag_formatstr)?
    } else {
        (Vec::new(), Vec::new())
    };
    let Ok(dyncols_len) = u8::try_from(column_keys.len()) else {
        return fail_incorrectusage_clierror!(
            "Too many %dyncols columns: {} (max 255).",
            column_keys.len()
        );
    };

    #[cfg(feature = "datapusher_plus")]
    let show_progress = false;
    #[cfg(not(feature = "datapusher_plus"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let json_output =
        now_cmd && (args.flag_formatstr == "%json" || args.flag_formatstr == "%pretty-json");

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref())
        .quote_style(if json_output {
            csv::QuoteStyle::Never
        } else {
            csv::QuoteStyle::Necessary
        })
        .writer()?;

    let header_record = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&header_record)?;
    let column_index = *sel.iter().next().unwrap();

    let input_headers = rdr.headers()?.clone();
    let sources = resolve_param_sources(&params, &input_headers)?;

    let mut headers = input_headers;
    if let Some(new_name) = &args.flag_rename {
        let new_col_names = util::ColumnNameParser::new(new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of new columns does not match input column selection."
            );
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }
    if let Some(new_column) = &args.flag_new_column {
        headers.push_field(new_column);
    }
    for column_name in &column_names {
        headers.push_field(column_name);
    }
    if !json_output {
        wtr.write_record(&headers)?;
    }

    let timeout = Duration::from_secs(util::timeout_secs(args.flag_timeout)?);
    let client = reqwest::Client::builder()
        .user_agent(util::set_user_agent(None)?)
        .timeout(timeout)
        .use_rustls_tls()
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .zstd(true)
        // never follow redirects: reqwest keeps custom headers on a cross-host redirect, so a 3xx
        // would hand the subscription key to whatever host it points at
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // allow_burst(1) avoids an initial burst (see governor#39); run() rejects --rate-limit 0
    let rate_limit = NonZeroU32::new(args.flag_rate_limit.unwrap_or(DEFAULT_GEOCLIENT_RATE_LIMIT))
        .unwrap_or(NonZeroU32::MIN);
    let limiter = RateLimiter::direct(Quota::per_second(rate_limit).allow_burst(NonZeroU32::MIN));

    let disk_cache = if args.flag_no_cache {
        None
    } else {
        let mut cache_builder = RedbCache::builder(GEOCLIENT_CACHE_NAME)
            .disk_dir(cache_dir)
            .refresh_on_hit(false)
            .durable(false);
        if args.flag_cache_ttl != 0 {
            cache_builder = cache_builder.ttl(Duration::from_secs(args.flag_cache_ttl));
        }
        let cache: RedbCache<String, String> = cache_builder
            .build()
            .map_err(|e| CliError::Other(format!("Error building Geoclient disk cache: {e}")))?;
        if let Err(e) = cache.remove_expired_entries() {
            log::warn!("Could not remove expired Geoclient cache entries: {e}");
        }
        Some(cache)
    };

    let render = |obj: &Value| -> Vec<String> {
        if dyncols_mode {
            column_keys
                .iter()
                .map(|key| json_dotpath(obj, key).trim().to_string())
                .collect()
        } else {
            vec![format_geoclient_result(obj, &args.flag_formatstr)]
        }
    };
    // decode a cached "F<json>"/"N" entry into rendered values (None = no match)
    let render_encoded = |encoded: &str| -> Option<Vec<String>> {
        let json = decode_lookup_cache(encoded)?;
        let obj: Value = serde_json::from_str(&json).ok()?;
        Some(render(&obj))
    };

    let invalid_result = args.flag_invalid_result.clone().unwrap_or_default();
    let concurrency = util::njobs(args.flag_jobs).max(1);
    let batch_size = if args.flag_batch == 0 {
        usize::MAX
    } else {
        args.flag_batch
    };
    let retry_base = retry_base();

    let mut transient_failures: u64 = 0;
    let mut rejected: u64 = 0;
    let mut consecutive_transient: u32 = 0;
    let mut rows_written: u64 = 0;

    let mut record = csv::StringRecord::new();
    let mut batch: Vec<csv::StringRecord> = Vec::new();
    loop {
        batch.clear();
        while batch.len() < batch_size && rdr.read_record(&mut record)? {
            batch.push(record.clone());
        }
        if batch.is_empty() {
            break;
        }

        let requests: Vec<Option<Request>> = batch
            .iter()
            .map(|rec| {
                let cell = rec.get(column_index).unwrap_or_default();
                prepare_request(endpoint, &base_url, cell, rec, &sources)
            })
            .collect();

        // per-batch: key -> rendered outcome. Keeping it per batch bounds memory; the disk cache
        // de-duplicates across batches.
        let mut resolved: HashMap<&str, Option<Vec<String>>> = HashMap::new();
        let mut failed: HashSet<&str> = HashSet::new();
        let mut to_fetch: Vec<&Request> = Vec::new();
        let mut queued: HashSet<&str> = HashSet::new();
        for req in requests.iter().flatten() {
            let key = req.key.as_str();
            if resolved.contains_key(key) || queued.contains(key) {
                continue;
            }
            if let Some(dc) = &disk_cache
                && let Ok(Some(encoded)) = dc.cache_get(&req.key)
            {
                resolved.insert(key, render_encoded(&encoded));
                continue;
            }
            queued.insert(key);
            to_fetch.push(req);
        }

        let client = &client;
        let limiter = &limiter;
        let base_url = base_url.as_str();
        let mut responses = futures::stream::iter(to_fetch.into_iter().map(|req| async move {
            let result = geoclient_fetch(
                client,
                limiter,
                base_url,
                api_key,
                endpoint,
                &req.params,
                retry_base,
            )
            .await;
            (req.key.as_str(), result)
        }))
        .buffered(concurrency);

        while let Some((key, result)) = responses.next().await {
            match result {
                Ok(obj) => {
                    consecutive_transient = 0;
                    let json = obj.as_ref().and_then(|o| serde_json::to_string(o).ok());
                    if let Some(dc) = &disk_cache {
                        let _ = dc.cache_set(key.to_string(), encode_lookup_cache(json.as_deref()));
                    }
                    resolved.insert(key, obj.as_ref().map(render));
                },
                Err(GcError::Fatal(msg)) => return fail_clierror!("{msg}"),
                Err(GcError::Rejected(msg)) => {
                    consecutive_transient = 0;
                    log::warn!("Geoclient lookup rejected for {key:?}: {msg}");
                    rejected += 1;
                    failed.insert(key);
                },
                Err(GcError::Transient(msg)) => {
                    log::warn!("Geoclient lookup failed for {key:?}: {msg}");
                    transient_failures += 1;
                    consecutive_transient += 1;
                    failed.insert(key);
                    if consecutive_transient >= CIRCUIT_BREAKER_THRESHOLD {
                        wtr.flush()?;
                        return fail_clierror!(
                            "Aborting: {consecutive_transient} Geoclient requests in a row failed \
                             (last error: {msg}). If you are using the public API, you may have \
                             hit its quota (2,500 requests/minute, 500,000/day) - lower \
                             --rate-limit or try again later. {rows_written} rows were written \
                             before the abort."
                        );
                    }
                },
            }
        }
        drop(responses);

        for (rec, req) in batch.iter().zip(&requests) {
            let outcome = match req {
                None => RowOutcome::NoQuery,
                Some(req) if failed.contains(req.key.as_str()) => RowOutcome::Failed,
                Some(req) => match resolved.get(req.key.as_str()) {
                    Some(Some(values)) => RowOutcome::Hit(values.clone()),
                    Some(None) | None => RowOutcome::NoMatch,
                },
            };
            let cell = rec.get(column_index).unwrap_or_default();

            if dyncols_mode {
                let mut out_record = rec.clone();
                if let RowOutcome::Hit(values) = outcome {
                    for value in &values {
                        out_record.push_field(value);
                    }
                } else {
                    add_fields(&mut out_record, &invalid_result, dyncols_len);
                }
                wtr.write_record(&out_record)?;
            } else {
                let fallback = || {
                    if invalid_result.is_empty() {
                        cell.to_string()
                    } else {
                        invalid_result.clone()
                    }
                };
                let out_cell = match outcome {
                    // an empty <column> cell is left untouched, as with opencage
                    RowOutcome::NoQuery if !endpoint.is_structured() && cell.trim().is_empty() => {
                        cell.to_string()
                    },
                    RowOutcome::Hit(mut values) => values.pop().unwrap_or_default(),
                    RowOutcome::NoQuery | RowOutcome::Failed | RowOutcome::NoMatch => fallback(),
                };
                let out_record = if args.flag_new_column.is_some() {
                    let mut new_record = rec.clone();
                    new_record.push_field(&out_cell);
                    new_record
                } else {
                    replace_column_value(rec, column_index, &out_cell)
                };
                wtr.write_record(&out_record)?;
            }
            rows_written += 1;
            if show_progress {
                progress.inc(1);
            }
        }
    }

    if show_progress {
        util::finish_progress(&progress);
    }
    let fate = if invalid_result.is_empty() {
        "left unchanged"
    } else {
        "set to --invalid-result"
    };
    if rejected > 0 {
        wwarn!(
            "{rejected} Geoclient lookup(s) were rejected as invalid requests (HTTP 400), so \
             those rows were {fate}. Check --params. Set QSV_LOG_LEVEL=warn to log each one to \
             the qsv log file."
        );
    }
    if transient_failures > 0 {
        wwarn!(
            "{transient_failures} Geoclient lookup(s) failed after retries (network error, rate \
             limit or server error), so those rows were {fate}. Set QSV_LOG_LEVEL=warn to log \
             each failure to the qsv log file."
        );
    }
    Ok(wtr.flush()?)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn bbl_forms() {
        let s = |a: &str, b: &str, c: &str| Some((a.to_string(), b.to_string(), c.to_string()));
        assert_eq!(split_bbl("1000670001"), s("1", "00067", "0001"));
        assert_eq!(split_bbl(" 1000670001.0 "), s("1", "00067", "0001"));
        assert_eq!(split_bbl("1-67-1"), s("1", "00067", "0001"));
        assert_eq!(split_bbl("5/12345/6789"), s("5", "12345", "6789"));
        assert_eq!(split_bbl("6000670001"), None); // no borough 6
        assert_eq!(split_bbl("100067000"), None); // 9 digits
        assert_eq!(split_bbl("1000670001.5"), None);
        assert_eq!(split_bbl("1-123456-1"), None); // block > 5 digits
        assert_eq!(split_bbl("A000670001"), None);
        assert_eq!(split_bbl(""), None);
    }

    #[test]
    fn params_validation() {
        let p = parse_params(
            Endpoint::Address,
            Some("houseNumber={HouseNo}, street={Street},zip=10025"),
        )
        .unwrap();
        assert_eq!(
            p[0],
            (
                "houseNumber".to_string(),
                ParamValue::Column("HouseNo".to_string())
            )
        );
        assert_eq!(
            p[2],
            ("zip".to_string(), ParamValue::Literal("10025".to_string()))
        );
        // borough OR zip is required
        assert!(parse_params(Endpoint::Address, Some("houseNumber={H},street={S}")).is_err());
        // structured endpoints require --params
        assert!(parse_params(Endpoint::Place, None).is_err());
        // names are case-sensitive and checked per endpoint
        assert!(parse_params(Endpoint::Address, Some("housenumber={H},street={S},zip=1")).is_err());
        assert!(parse_params(Endpoint::Bin, Some("exactMatchMaxLevel=5")).is_err());
        // search takes literal options only
        assert!(parse_params(Endpoint::Search, Some("exactMatchMaxLevel=5")).is_ok());
        assert!(parse_params(Endpoint::Search, Some("returnPolicy={Col}")).is_err());
        assert!(parse_params(Endpoint::Search, None).unwrap().is_empty());
        // duplicates, empties and malformed entries
        assert!(parse_params(Endpoint::Place, Some("name={A},name={B},zip=1")).is_err());
        assert!(parse_params(Endpoint::Place, Some("name={},zip=1")).is_err());
        assert!(parse_params(Endpoint::Place, Some("name={A},zip=")).is_err());
        assert!(parse_params(Endpoint::Place, Some("name")).is_err());
    }

    #[test]
    fn requests_and_cache_keys() {
        let headers = csv::StringRecord::from(vec!["House", "Street", "Boro"]);
        let params = parse_params(
            Endpoint::Address,
            Some("houseNumber={House},street={Street},borough={Boro}"),
        )
        .unwrap();
        let sources = resolve_param_sources(&params, &headers).unwrap();
        let rec = csv::StringRecord::from(vec!["314", "west  100 st", "manhattan"]);
        let req = prepare_request(Endpoint::Address, "http://x", "", &rec, &sources).unwrap();
        assert!(
            req.params
                .contains(&("street".to_string(), "west 100 st".to_string()))
        );
        // the key is case-folded, so a differently-cased row hits the same cache entry
        let rec2 = csv::StringRecord::from(vec!["314", "WEST 100 ST", "Manhattan"]);
        let req2 = prepare_request(Endpoint::Address, "http://x", "", &rec2, &sources).unwrap();
        assert_eq!(req.key, req2.key);
        // a missing required value means no request at all
        let rec3 = csv::StringRecord::from(vec!["314", "", "manhattan"]);
        assert!(prepare_request(Endpoint::Address, "http://x", "", &rec3, &sources).is_none());
        // an unknown column is caught against the headers
        let bad = parse_params(Endpoint::Place, Some("name={Nope},zip=10001")).unwrap();
        assert!(resolve_param_sources(&bad, &headers).is_err());
    }

    #[test]
    fn extract_result_return_codes_and_coords() {
        // /bbl: no latitude/longitude, only the internal-label pair
        let body = json!({"bbl": {"geosupportReturnCode": "00", "bbl": "1000670001",
            "latitudeInternalLabel": 40.708659, "longitudeInternalLabel": -74.007118}});
        let obj = extract_result(Endpoint::Bbl, &body).unwrap();
        assert_eq!(format_geoclient_result(&obj, "%+"), "40.708659, -74.007118");
        // /search: RC 01 + RC2 00 is a success (upstream sample search-900)
        let body = json!({"status": "OK", "results": [{"status": "EXACT_MATCH", "response": {
            "geosupportReturnCode": "01", "geosupportReturnCode2": "00",
            "latitude": 40.798502, "longitude": -73.972709, "latitudeInternalLabel": 1.0}}]});
        let obj = extract_result(Endpoint::Search, &body).unwrap();
        assert_eq!(
            format_geoclient_result(&obj, "%location"),
            "(40.798502, -73.972709)"
        );
        // /address: only RC2 succeeds -> still a success
        let body = json!({"address": {"geosupportReturnCode": "EE",
            "geosupportReturnCode2": "00", "latitude": 1.5, "longitude": 2.5}});
        assert!(extract_result(Endpoint::Address, &body).is_some());
        // a reject on both codes, and a wrong wrapper key, are non-matches
        let body =
            json!({"address": {"geosupportReturnCode": "EE", "geosupportReturnCode2": "42"}});
        assert!(extract_result(Endpoint::Address, &body).is_none());
        let body = json!({"bin": {"geosupportReturnCode": "00"}});
        assert!(extract_result(Endpoint::Bbl, &body).is_none());
    }

    #[test]
    fn formatting_trims_and_tolerates_missing_fields() {
        let obj = json!({"bbl": "1000670001", "censusTract2020": " 187  ",
            "buildingIdentificationNumber": "1079043"});
        assert_eq!(format_geoclient_result(&obj, "%bbl"), "1000670001");
        assert_eq!(format_geoclient_result(&obj, "%bin"), "1079043");
        assert_eq!(
            format_geoclient_result(&obj, "tract {censusTract2020} / {nta}"),
            "tract 187 / "
        );
        // no coordinates -> empty, never a dangling ", "
        assert_eq!(format_geoclient_result(&obj, "%+"), "");
    }
}
