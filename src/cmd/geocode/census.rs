//! US Census Geocoder backend for `qsv geocode census` / `censusnow`.
//!
//! The Census Geocoder (<https://geocoding.geo.census.gov/geocoder>) is free and keyless. Facts
//! this module relies on (verified live 2026-10-12/13):
//! - single records: `geographies/{onelineaddress,address,coordinates}`, always asked for
//!   `layers=all`, so one response serves every output format;
//! - a no-match is HTTP 200 with `"addressMatches": []`; an HTTP 400 is a request the API will
//!   never accept (missing benchmark/vintage, a parsed address with only a street), so it is fatal;
//! - `geographies/address` needs a street plus city and state, or a ZIP code (anything less is a
//!   400);
//! - batch: `geographies/{addressbatch,coordinatesbatch}` take a headerless CSV of up to 10,000
//!   rows and answer an unordered, headerless, fully-quoted CSV with only STATE, COUNTY, TRACT and
//!   BLOCK for geographies, so batch results carry a subset of what a single lookup returns;
//! - layer names vary by vintage: only `Current_*` vintages have `2020 Census Blocks`.
//!
//! The result is normalized client-side to one JSON shape (see `normalize_*`), so the output is
//! identical against census.gov and a wire-compatible caching proxy (`ucgeocoder`).

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
use serde_json::{Value, json};
use uuid::Uuid;

use super::{
    Args, BodyError, MAX_BODY_BYTES, OPENCAGE_FORMATSTR_REGEX, ParamSource, ParamValue, add_fields,
    first_selected_column, json_dotpath, normalize_opencage_query, parse_dyncols_spec,
    parse_param_list, read_capped, resolve_param_sources, squish,
};
use crate::{
    CliError, CliResult,
    config::Config,
    select::SelectColumns,
    util::{self, replace_column_value},
};

/// The public Census Geocoder. `QSV_CENSUS_GEOCODER_URL` overrides it (a caching proxy, tests).
pub(super) const DEFAULT_CENSUS_GEOCODER_URL: &str = "https://geocoding.geo.census.gov/geocoder";

/// The `cached` crate names the redb file after this, under the cache dir. NOT
/// "geocode-census": that is the `ucgeocoder` proxy's own db name, which stores `&[u8]` values.
pub(super) const CENSUS_CACHE_NAME: &str = "geocode-censusgeocoder";

pub(super) const DEFAULT_BENCHMARK: &str = "Public_AR_Current";
pub(super) const DEFAULT_VINTAGE: &str = "Current_Current";

/// Default requests/second for single-record lookups. Census publishes no limit; this is the
/// same pace the `ucgeocoder` proxy uses upstream.
pub(super) const DEFAULT_CENSUS_RATE_LIMIT: u32 = 5;

/// The `--params` names: the parts of a parsed address.
const PARAM_NAMES: [&str; 4] = ["street", "city", "state", "zip"];

/// Resolve the Census Geocoder base URL (trailing slashes trimmed) from `QSV_CENSUS_GEOCODER_URL`.
pub(super) fn census_base_url() -> String {
    std::env::var("QSV_CENSUS_GEOCODER_URL")
        .ok()
        .map(|u| u.trim().trim_end_matches('/').to_string())
        .filter(|u| !u.is_empty())
        .unwrap_or_else(|| DEFAULT_CENSUS_GEOCODER_URL.to_string())
}

/// What a row's input is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum InputMode {
    /// A full address in `<column>`.
    OneLine,
    /// Address parts mapped with `--params`.
    Parsed,
    /// A "lat, lng" coordinate in `<column>` (`--reverse`).
    Coordinates,
}

impl InputMode {
    pub(super) const fn from_args(args: &Args) -> Self {
        if args.flag_reverse {
            Self::Coordinates
        } else if args.flag_params.is_some() {
            Self::Parsed
        } else {
            Self::OneLine
        }
    }
}

/// `--census-mode`: whether lookups use the batch endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CensusMode {
    /// Batch when the output allows it and enough rows of a read chunk need a lookup.
    Auto,
    /// Always batch; an output that needs single-record fields is a usage error.
    Batch,
    /// Never batch.
    Single,
}

impl CensusMode {
    pub(super) fn parse(value: Option<&str>) -> CliResult<Self> {
        Ok(
            match value.unwrap_or("auto").trim().to_ascii_lowercase().as_str() {
                "auto" => Self::Auto,
                "batch" => Self::Batch,
                "single" => Self::Single,
                other => {
                    return fail_incorrectusage_clierror!(
                        "Invalid --census-mode {other:?}. Valid modes are: auto, batch and single."
                    );
                },
            },
        )
    }
}

/// Does a parsed address have enough parts for the Census Geocoder? A street plus a ZIP code,
/// or a street plus city and state - anything less is an HTTP 400 (live probe, 2026-10-12).
/// Checked over the `--params` names at startup, and over each row's non-empty values.
fn parsed_address_complete(has: impl Fn(&str) -> bool) -> bool {
    has("street") && (has("zip") || (has("city") && has("state")))
}

/// Parse and validate the census `--params` ("street={Street},city={City},state=NY,zip={Zip}").
pub(super) fn parse_params(template: Option<&str>) -> CliResult<Vec<(String, ParamValue)>> {
    let params = parse_param_list(template, &PARAM_NAMES, "census", true)?;
    if template.is_some() && !parsed_address_complete(|name| params.iter().any(|(n, _)| n == name))
    {
        return fail_incorrectusage_clierror!(
            "census --params needs street plus zip, or street plus city and state (the Census \
             Geocoder rejects anything less), e.g. \
             \"street={{Street}},city={{City}},state={{State}},zip={{Zip}}\"."
        );
    }
    Ok(params)
}

/// The Census block layer's name in a vintage: Census2010/Census2020 vintages call it
/// "Census Blocks", `Current` vintages "2020 Census Blocks". ACS vintages have no block layer,
/// so the name is simply absent from their results.
pub(super) fn block_layer(vintage: &str) -> &'static str {
    if vintage.starts_with("Census") {
        "Census Blocks"
    } else {
        "2020 Census Blocks"
    }
}

/// `%census-geoids`: the GEOIDs of the location's state, county, tract, block group and block,
/// as `%dyncols:` columns.
fn census_geoids_dyncols(vintage: &str) -> String {
    format!(
        "%dyncols: {{census_state:flat.States.GEOID}}, {{census_county:flat.Counties.GEOID}}, \
         {{census_tract:flat.Census Tracts.GEOID}}, {{census_blockgroup:flat.Census Block \
         Groups.GEOID}}, {{census_block:flat.{}.GEOID}}",
        block_layer(vintage)
    )
}

/// The `%dyncols:` spec a named Census column bundle stands for, if `formatstr` is one.
pub(super) fn preset_dyncols(formatstr: &str, vintage: &str) -> Option<String> {
    (formatstr == "%census-geoids").then(|| census_geoids_dyncols(vintage))
}

/// The geography layers a batch result is synthesized into (from its STATE, COUNTY, TRACT and
/// BLOCK columns), with the attributes each gets. Only valid for `Current_*` vintages.
const BATCH_LAYERS: [(&str, &[&str]); 5] = [
    ("States", &["STATE", "GEOID"]),
    ("Counties", &["STATE", "COUNTY", "GEOID"]),
    ("Census Tracts", &["STATE", "COUNTY", "TRACT", "GEOID"]),
    (
        "Census Block Groups",
        &["STATE", "COUNTY", "TRACT", "BLKGRP", "GEOID"],
    ),
    (
        "2020 Census Blocks",
        &["STATE", "COUNTY", "TRACT", "BLKGRP", "BLOCK", "GEOID"],
    ),
];

/// Is `key` (a `json_dotpath`) one a batch result fills the same way a single lookup does?
fn is_batch_key(key: &str) -> bool {
    if matches!(
        key,
        "matchedAddress"
            | "coordinates.x"
            | "coordinates.y"
            | "tigerLine.tigerLineId"
            | "tigerLine.side"
            | "_matchType"
            | "_source"
    ) {
        return true;
    }
    key.strip_prefix("flat.").is_some_and(|rest| {
        BATCH_LAYERS.iter().any(|(layer, attrs)| {
            rest.strip_prefix(layer)
                .and_then(|r| r.strip_prefix('.'))
                .is_some_and(|attr| attrs.contains(&attr))
        })
    })
}

/// Can the batch endpoints produce this output? Only for `Current_*` vintages (whose layer names
/// the batch synthesis uses), and only if every field the format reads is in the batch subset:
/// the coordinate formats, `%matched-address`, `%census-geoids`, and `%dyncols:`/`{field}` keys
/// that are all batch keys. `%json`/`%pretty-json` need the full single-record result.
/// `formatstr` is the expanded one (presets already turned into `%dyncols:`).
pub(super) fn batch_eligible(formatstr: &str, vintage: &str) -> bool {
    if !vintage.starts_with("Current_") {
        return false;
    }
    if formatstr.starts_with("%dyncols:") {
        parse_dyncols_spec(formatstr).is_ok_and(|(_, keys)| keys.iter().all(|k| is_batch_key(k)))
    } else if formatstr.starts_with('%') {
        !matches!(formatstr, "%json" | "%pretty-json")
    } else {
        OPENCAGE_FORMATSTR_REGEX()
            .captures_iter(formatstr)
            .all(|caps| is_batch_key(&caps[1]))
    }
}

/// Prefix of every cache key. Bump it when the cached value format or the meaning of a cached
/// outcome changes: older entries then never match and age out (TTL, `cache-prune`).
const CACHE_KEY_VERSION: &str = "v1";

/// Retries after the first attempt, for 408/429/5xx/network errors.
const MAX_RETRIES: u32 = 3;

/// Consecutive transient failures that abort the run: the service is down or refusing us, and
/// grinding retries through the rest of a large file helps nobody.
const CIRCUIT_BREAKER_THRESHOLD: u32 = 10;

impl InputMode {
    /// The single-record endpoint (`geographies/<searchtype>`), also the cache key's search type.
    const fn searchtype(self) -> &'static str {
        match self {
            Self::OneLine => "onelineaddress",
            Self::Parsed => "address",
            Self::Coordinates => "coordinates",
        }
    }
}

/// One row's lookup: its cache key, the single-record query parameters, and the fields of its
/// batch-file row (after the id): street, city, state, zip - or x, y for coordinates.
struct Request {
    key:   String,
    query: Vec<(&'static str, String)>,
    batch: Vec<String>,
}

/// A `--reverse` cell as (lat, lng), when it is a WGS-84 coordinate in range.
fn parse_coordinate(cell: &str) -> Option<(f64, f64)> {
    let normalized = normalize_opencage_query(cell, true)?;
    let (lat, lng) = normalized.split_once(',')?;
    let (lat, lng) = (lat.parse::<f64>().ok()?, lng.parse::<f64>().ok()?);
    ((-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lng)).then_some((lat, lng))
}

/// Build a row's request, or `None` when it cannot be looked up (an empty address, an
/// unparseable coordinate, a parsed address missing the parts Census requires - a request the API
/// would reject with an HTTP 400, which is fatal for the run).
///
/// The cache key holds the canonical input: whitespace-collapsed, ASCII-uppercased address parts
/// (Census matching ignores case; parsed parts as a JSON array), or the coordinate at 6 decimals
/// (~0.1 m), which is also what is sent, so the key describes the query exactly. The search type
/// comes from the input mode, never from whether the row is later batched, so batch and single
/// lookups share entries.
fn prepare_request(
    mode: InputMode,
    base_url: &str,
    benchmark: &str,
    vintage: &str,
    cell: &str,
    record: &csv::StringRecord,
    sources: &[(String, ParamSource)],
) -> Option<Request> {
    let (query, batch, canonical): (Vec<(&'static str, String)>, Vec<String>, String) = match mode {
        InputMode::OneLine => {
            let address = squish(cell);
            if address.is_empty() {
                return None;
            }
            let canonical = address.to_ascii_uppercase();
            (
                vec![("address", address.clone())],
                vec![address, String::new(), String::new(), String::new()],
                canonical,
            )
        },
        InputMode::Parsed => {
            let value = |name: &str| {
                sources
                    .iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, source)| match source {
                        ParamSource::Literal(lit) => squish(lit),
                        ParamSource::Column(idx) => squish(record.get(*idx).unwrap_or_default()),
                    })
                    .unwrap_or_default()
            };
            let parts: Vec<String> = PARAM_NAMES.iter().map(|n| value(n)).collect();
            let has = |name: &str| {
                PARAM_NAMES
                    .iter()
                    .position(|n| *n == name)
                    .is_some_and(|i| !parts[i].is_empty())
            };
            if !parsed_address_complete(has) {
                return None;
            }
            let query = PARAM_NAMES
                .iter()
                .zip(&parts)
                .filter(|(_, v)| !v.is_empty())
                .map(|(n, v)| (*n, v.clone()))
                .collect();
            // a JSON array, so no character in a part can make two addresses' keys collide
            let canonical = json!(
                parts
                    .iter()
                    .map(|p| p.to_ascii_uppercase())
                    .collect::<Vec<_>>()
            )
            .to_string();
            (query, parts, canonical)
        },
        InputMode::Coordinates => {
            let (lat, lng) = parse_coordinate(cell)?;
            let (x, y) = (format!("{lng:.6}"), format!("{lat:.6}"));
            let canonical = format!("{x},{y}");
            (
                vec![("x", x.clone()), ("y", y.clone())],
                vec![x, y],
                canonical,
            )
        },
    };
    let key = format!(
        "{CACHE_KEY_VERSION}|{base_url}|{}|{benchmark}|{vintage}|{canonical}",
        mode.searchtype()
    );
    Some(Request { key, query, batch })
}

/// A coordinate as a JSON number, from a JSON number or numeric text. Both the single-record
/// (JSON numbers) and the batch (text) paths go through it, so `json_dotpath` renders them the
/// same way.
fn coordinate_number(value: &Value) -> Option<Value> {
    let f = match value {
        Value::Number(n) => n.as_f64()?,
        Value::String(s) => s.trim().parse::<f64>().ok()?,
        _ => return None,
    };
    serde_json::Number::from_f64(f).map(Value::Number)
}

/// `{"x": lon, "y": lat}` from an object with `x`/`y`, if both are numeric.
fn coordinates_object(obj: &Value) -> Option<Value> {
    Some(json!({
        "x": coordinate_number(obj.get("x")?)?,
        "y": coordinate_number(obj.get("y")?)?,
    }))
}

/// `flat.<Layer>`: the scalar attributes (strings, numbers, booleans) of each layer's first
/// feature, so a dotted `%dyncols:` key like `flat.Census Tracts.GEOID` reaches them.
fn flatten_geographies(geographies: &Value) -> Value {
    let mut flat = serde_json::Map::new();
    if let Some(layers) = geographies.as_object() {
        for (layer, features) in layers {
            let Some(feature) = features
                .as_array()
                .and_then(|f| f.first())
                .and_then(Value::as_object)
            else {
                continue;
            };
            let attrs: serde_json::Map<String, Value> = feature
                .iter()
                .filter(|(_, v)| matches!(v, Value::String(_) | Value::Number(_) | Value::Bool(_)))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            flat.insert(layer.clone(), Value::Object(attrs));
        }
    }
    Value::Object(flat)
}

/// Normalize a single-record `geographies/*` response; `None` is a no-match.
///
/// Address searches use the first of `addressMatches` (`_resultCount` says how many there were);
/// a coordinate search has no `addressMatches`, its point is the echoed `input.location`, and it
/// is a no-match when no layer has a feature (e.g. a point at sea).
fn normalize_single(mode: InputMode, body: &Value) -> Option<Value> {
    let result = body.get("result")?;
    let mut obj = serde_json::Map::new();
    let geographies = if mode == InputMode::Coordinates {
        let geographies = result.get("geographies").cloned().unwrap_or(Value::Null);
        let any_feature = geographies.as_object().is_some_and(|layers| {
            layers
                .values()
                .any(|f| f.as_array().is_some_and(|a| !a.is_empty()))
        });
        if !any_feature {
            return None;
        }
        if let Some(coords) = result
            .get("input")
            .and_then(|i| i.get("location"))
            .and_then(coordinates_object)
        {
            obj.insert("coordinates".into(), coords);
        }
        obj.insert("_resultCount".into(), json!(1));
        geographies
    } else {
        let matches = result.get("addressMatches")?.as_array()?;
        let first = matches.first()?;
        if let Some(v) = first.get("matchedAddress") {
            obj.insert("matchedAddress".into(), v.clone());
        }
        if let Some(coords) = first.get("coordinates").and_then(coordinates_object) {
            obj.insert("coordinates".into(), coords);
        }
        for key in ["tigerLine", "addressComponents"] {
            if let Some(v) = first.get(key) {
                obj.insert(key.into(), v.clone());
            }
        }
        obj.insert("_resultCount".into(), json!(matches.len()));
        first.get("geographies").cloned().unwrap_or(Value::Null)
    };
    obj.insert("flat".into(), flatten_geographies(&geographies));
    obj.insert("geographies".into(), geographies);
    obj.insert("_source".into(), json!("single"));
    Some(Value::Object(obj))
}

/// A cached outcome.
#[derive(Debug, PartialEq)]
enum Cached {
    /// A single-record result: satisfies every output.
    Full(Value),
    /// A batch result: satisfies only batch-eligible outputs.
    Batch(Value),
    /// A single-record no-match.
    NoMatch,
}

/// Cache encoding: "F<json>" single-record result, "B<json>" batch result, "N" no match.
fn encode_cached(cached: &Cached) -> Option<String> {
    match cached {
        Cached::Full(v) => serde_json::to_string(v).ok().map(|s| format!("F{s}")),
        Cached::Batch(v) => serde_json::to_string(v).ok().map(|s| format!("B{s}")),
        Cached::NoMatch => Some("N".to_string()),
    }
}

/// Decode `encode_cached`'s format; `None` (undecodable) is treated as a cache miss.
fn decode_cached(encoded: &str) -> Option<Cached> {
    match encoded.split_at_checked(1)? {
        ("N", "") => Some(Cached::NoMatch),
        ("F", json) => serde_json::from_str(json).ok().map(Cached::Full),
        ("B", json) => serde_json::from_str(json).ok().map(Cached::Batch),
        _ => None,
    }
}

/// Error from a Census request. `Fatal` aborts the run (HTTP 400/404: the request or the base
/// URL is wrong, so every other row would fail the same way); `Transient` is a per-row failure
/// (network error, 408/429/5xx, an oversized or unreadable body) left after retries.
#[derive(Debug)]
enum CensusError {
    Fatal(String),
    Transient(String),
}

fn retry_base() -> Duration {
    let ms = std::env::var("QSV_CENSUS_GEOCODER_RETRY_BASE_MS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(1000);
    Duration::from_millis(ms)
}

/// The reason in a Census error body: `{"errors": ["..."], "status": "400"}` (JSON endpoints) or
/// an HTML/text message (batch endpoints), trimmed to 300 chars.
fn error_reason(body: &[u8]) -> String {
    let reason = serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|v| {
            v.get("errors").and_then(Value::as_array).map(|errors| {
                errors
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join("; ")
            })
        })
        .unwrap_or_else(|| {
            // the batch endpoints answer HTML paragraphs: keep the text
            let text = String::from_utf8_lossy(body);
            let mut plain = String::with_capacity(text.len());
            let mut in_tag = false;
            for c in text.chars() {
                match c {
                    '<' => in_tag = true,
                    '>' => {
                        in_tag = false;
                        plain.push(' ');
                    },
                    _ if !in_tag => plain.push(c),
                    _ => {},
                }
            }
            squish(&plain)
        });
    reason.chars().take(300).collect()
}

/// The fatal error for a 400/404 answer.
fn fatal_status(status: u16, url: &str, body: &[u8]) -> CensusError {
    if status == 404 {
        CensusError::Fatal(format!(
            "Census Geocoder endpoint not found (HTTP 404) at {url}. Check \
             QSV_CENSUS_GEOCODER_URL - it should end in /geocoder."
        ))
    } else {
        let reason = error_reason(body);
        CensusError::Fatal(format!(
            "The Census Geocoder rejected the request (HTTP {status}){}. Check --benchmark and \
             --vintage.",
            if reason.is_empty() {
                String::new()
            } else {
                format!(": {reason}")
            }
        ))
    }
}

/// The `Retry-After` delay of a response, in seconds, capped at 60s.
fn retry_after(resp: &reqwest::Response) -> Option<Duration> {
    resp.headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(|secs| Duration::from_secs(secs.min(60)))
}

/// GET one location, retrying 408/429/5xx/network errors and truncated bodies with exponential
/// backoff (honoring a `Retry-After` in seconds, capped at 60s). `Ok(None)` is a no-match.
#[allow(clippy::too_many_arguments)]
async fn census_fetch(
    client: &reqwest::Client,
    limiter: &DefaultDirectRateLimiter,
    base_url: &str,
    benchmark: &str,
    vintage: &str,
    mode: InputMode,
    query: &[(&'static str, String)],
    retry_base: Duration,
) -> Result<Option<Value>, CensusError> {
    let url = format!("{base_url}/geographies/{}", mode.searchtype());
    let mut params: Vec<(&str, &str)> = query.iter().map(|(n, v)| (*n, v.as_str())).collect();
    params.extend([
        ("benchmark", benchmark),
        ("vintage", vintage),
        ("layers", "all"),
        ("format", "json"),
    ]);
    let mut last_error = String::new();
    for attempt in 0..=MAX_RETRIES {
        limiter.until_ready().await;
        let mut delay: Option<Duration> = None;
        match client.get(&url).query(&params).send().await {
            Err(e) => last_error = format!("Census Geocoder network error: {e}"),
            Ok(resp) => {
                let status = resp.status().as_u16();
                match status {
                    200 => match read_capped(resp, MAX_BODY_BYTES).await {
                        Ok(bytes) => {
                            let body: Value = serde_json::from_slice(&bytes).map_err(|e| {
                                CensusError::Transient(format!(
                                    "Census Geocoder response is not JSON: {e}"
                                ))
                            })?;
                            if body.get("result").is_none() {
                                return Err(CensusError::Transient(format!(
                                    "Census Geocoder response has no result: {}",
                                    error_reason(&bytes)
                                )));
                            }
                            return Ok(normalize_single(mode, &body));
                        },
                        Err(BodyError::TooLarge(cap)) => {
                            return Err(CensusError::Transient(format!(
                                "Census Geocoder response body is larger than {cap} bytes."
                            )));
                        },
                        // a connection that dies mid-body is retried like one that never connected
                        Err(BodyError::Transport(e)) => {
                            last_error = format!("Census Geocoder response error: {e}");
                        },
                    },
                    400 | 404 => {
                        let body = read_capped(resp, 4096).await.unwrap_or_default();
                        return Err(fatal_status(status, &url, &body));
                    },
                    408 | 429 | 500..=599 => {
                        delay = retry_after(&resp);
                        last_error = format!("Census Geocoder returned HTTP {status}.");
                    },
                    _ => {
                        return Err(CensusError::Transient(format!(
                            "Census Geocoder returned HTTP {status}."
                        )));
                    },
                }
            },
        }
        if attempt < MAX_RETRIES {
            let backoff = delay.unwrap_or_else(|| retry_base * 2u32.pow(attempt));
            tokio::time::sleep(backoff.min(Duration::from_secs(60))).await;
        }
    }
    Err(CensusError::Transient(last_error))
}

/// Format a normalized result per `formatstr`: a predefined %-format or a `{field}` template.
/// A field the result lacks renders empty.
fn format_census_result(obj: &Value, formatstr: &str) -> String {
    let field = |key: &str| json_dotpath(obj, key).trim().to_string();
    let lat_long = || {
        let (lat, lng) = (field("coordinates.y"), field("coordinates.x"));
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
            "%matched-address" => field("matchedAddress"),
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

/// `--census-mode auto` batches a read chunk when at least this many of its lookups are left
/// after the cache: below it, the per-request pace beats an upload's fixed cost.
const BATCH_THRESHOLD: usize = 50;

/// Rows per batch upload. Census accepts up to 10,000, but a large upload can take minutes to
/// answer; `QSV_CENSUS_GEOCODER_BATCH_CHUNK` overrides it (tests).
const DEFAULT_BATCH_CHUNK: usize = 2000;
const MAX_BATCH_ROWS: usize = 10_000;

/// A batch upload's timeout is at least this long, whatever --timeout says: Census geocodes the
/// whole file before it answers.
const MIN_BATCH_TIMEOUT: Duration = Duration::from_secs(600);

/// Cap on a batch response: a few hundred bytes per row, for up to 10,000 rows.
const MAX_BATCH_BODY_BYTES: usize = 16 * 1024 * 1024;

fn batch_chunk_size() -> usize {
    std::env::var("QSV_CENSUS_GEOCODER_BATCH_CHUNK")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(DEFAULT_BATCH_CHUNK)
        .clamp(1, MAX_BATCH_ROWS)
}

impl InputMode {
    /// The batch endpoint and the name of its file field.
    const fn batch_endpoint(self) -> (&'static str, &'static str) {
        match self {
            Self::Coordinates => ("coordinatesbatch", "coordinatesFile"),
            Self::OneLine | Self::Parsed => ("addressbatch", "addressFile"),
        }
    }
}

/// One row of a batch response.
#[derive(Debug, PartialEq)]
enum BatchRow {
    /// The batch-normalized result.
    Match(Value),
    NoMatch,
    /// Several addresses matched; Census does not say which.
    Tie,
}

/// `flat` for a batch result: the five Census layers of a `Current_*` vintage, synthesized from
/// STATE, COUNTY, TRACT and BLOCK the way Census composes GEOIDs (county = STATE+COUNTY, tract =
/// county+TRACT, block group = tract+BLKGRP, the first digit of BLOCK, block = tract+BLOCK). A
/// layer whose parts are missing is left out, as a single lookup would have no feature for it.
fn synthesize_flat(state: &str, county: &str, tract: &str, block: &str) -> Value {
    let mut flat = serde_json::Map::new();
    let [states, counties, tracts, block_groups, blocks] = BATCH_LAYERS.map(|(name, _)| name);
    if state.is_empty() {
        return Value::Object(flat);
    }
    flat.insert(states.into(), json!({"STATE": state, "GEOID": state}));
    if county.is_empty() {
        return Value::Object(flat);
    }
    let county_geoid = format!("{state}{county}");
    flat.insert(
        counties.into(),
        json!({"STATE": state, "COUNTY": county, "GEOID": county_geoid}),
    );
    if tract.is_empty() {
        return Value::Object(flat);
    }
    let tract_geoid = format!("{county_geoid}{tract}");
    flat.insert(
        tracts.into(),
        json!({"STATE": state, "COUNTY": county, "TRACT": tract, "GEOID": tract_geoid}),
    );
    let Some(blkgrp) = block.get(..1) else {
        return Value::Object(flat);
    };
    flat.insert(
        block_groups.into(),
        json!({"STATE": state, "COUNTY": county, "TRACT": tract, "BLKGRP": blkgrp,
            "GEOID": format!("{tract_geoid}{blkgrp}")}),
    );
    flat.insert(
        blocks.into(),
        json!({"STATE": state, "COUNTY": county, "TRACT": tract, "BLKGRP": blkgrp,
            "BLOCK": block, "GEOID": format!("{tract_geoid}{block}")}),
    );
    Value::Object(flat)
}

/// Parse a batch response: a headerless, fully-quoted CSV in no particular order, keyed by the
/// row id qsv sent. Address rows are `id, input, Match|No_Match|Tie, Exact|Non_Exact, matched
/// address, "lon,lat", tigerLineId, side, STATE, COUNTY, TRACT, BLOCK`; coordinate rows are
/// `id, x, y, Match|No_Match, STATE, COUNTY, TRACT, BLOCK`. Missing geography columns are
/// tolerated (absent `flat` layers), as is any other unexpected status (no match).
fn parse_batch_response(mode: InputMode, body: &[u8]) -> Result<HashMap<usize, BatchRow>, String> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(body);
    let mut rows = HashMap::new();
    for record in rdr.records() {
        let record = record.map_err(|e| format!("Census batch response is not CSV: {e}"))?;
        let field = |i: usize| record.get(i).unwrap_or_default().trim();
        let Ok(id) = field(0).parse::<usize>() else {
            continue;
        };
        let coords = mode == InputMode::Coordinates;
        let status = field(if coords { 3 } else { 2 });
        let row = match status {
            "Match" => {
                let mut obj = serde_json::Map::new();
                let (x, y, geo) = if coords {
                    (field(1), field(2), 4)
                } else {
                    let (x, y) = field(5).split_once(',').unwrap_or_default();
                    obj.insert("matchedAddress".into(), json!(field(4)));
                    (x, y, 8)
                };
                if let Some(coordinates) = coordinates_object(&json!({"x": x, "y": y})) {
                    obj.insert("coordinates".into(), coordinates);
                }
                if !coords {
                    obj.insert(
                        "tigerLine".into(),
                        json!({"tigerLineId": field(6), "side": field(7)}),
                    );
                }
                obj.insert(
                    "flat".into(),
                    synthesize_flat(field(geo), field(geo + 1), field(geo + 2), field(geo + 3)),
                );
                obj.insert("_resultCount".into(), json!(1));
                if !coords {
                    obj.insert("_matchType".into(), json!(field(3)));
                }
                obj.insert("_source".into(), json!("batch"));
                BatchRow::Match(Value::Object(obj))
            },
            "Tie" => BatchRow::Tie,
            _ => BatchRow::NoMatch,
        };
        rows.insert(id, row);
    }
    Ok(rows)
}

/// Must a batch match be looked up again with the single-record endpoint? Connecticut's: since
/// 2022 its county equivalents are planning regions, and the Current vintage's Counties, Census
/// Tracts and Census Block Groups layers use them (county 110, "Capitol Planning Region"), but a
/// batch row only has the 2020 block's legacy county (003, Hartford County) - so the GEOIDs
/// synthesized from it would not be the Current vintage's (live probe, 2026-10-13).
fn needs_single_lookup(obj: &Value) -> bool {
    obj.get("flat")
        .and_then(|f| f.get("States"))
        .and_then(|s| s.get("STATE"))
        .and_then(Value::as_str)
        == Some("09")
}

/// The headerless CSV file a batch upload sends: one `id,<request's batch fields>` row each, the
/// id being the request's index in `requests`. The csv crate quotes fields that need it (a
/// one-line address is full of commas).
fn batch_file(requests: &[&Request]) -> Result<Vec<u8>, String> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(Vec::new());
    for (id, req) in requests.iter().enumerate() {
        wtr.write_record(std::iter::once(id.to_string()).chain(req.batch.iter().cloned()))
            .map_err(|e| e.to_string())?;
    }
    wtr.into_inner().map_err(|e| e.to_string())
}

/// Upload one batch, with the single-record retry policy. Returns the rows by id.
#[allow(clippy::too_many_arguments)]
async fn census_batch_fetch(
    client: &reqwest::Client,
    base_url: &str,
    benchmark: &str,
    vintage: &str,
    mode: InputMode,
    requests: &[&Request],
    timeout: Duration,
    retry_base: Duration,
) -> Result<HashMap<usize, BatchRow>, CensusError> {
    let (endpoint, file_field) = mode.batch_endpoint();
    let url = format!("{base_url}/geographies/{endpoint}");
    let file = batch_file(requests).map_err(CensusError::Transient)?;
    let mut last_error = String::new();
    for attempt in 0..=MAX_RETRIES {
        let part = reqwest::multipart::Part::bytes(file.clone())
            .file_name(format!("{endpoint}.csv"))
            .mime_str("text/csv")
            .map_err(|e| CensusError::Transient(e.to_string()))?;
        let form = reqwest::multipart::Form::new()
            .text("benchmark", benchmark.to_string())
            .text("vintage", vintage.to_string())
            .part(file_field, part);
        let mut delay: Option<Duration> = None;
        match client
            .post(&url)
            .multipart(form)
            .timeout(timeout)
            .send()
            .await
        {
            Err(e) => last_error = format!("Census Geocoder batch upload error: {e}"),
            Ok(resp) => {
                let status = resp.status().as_u16();
                match status {
                    200 => match read_capped(resp, MAX_BATCH_BODY_BYTES).await {
                        Ok(bytes) => {
                            let rows = parse_batch_response(mode, &bytes)
                                .map_err(CensusError::Transient)?;
                            // an answer to none of the rows (an empty body, a WAF or proxy page
                            // served with HTTP 200) is a failed upload, retried like one
                            if (0..requests.len()).any(|id| rows.contains_key(&id)) {
                                return Ok(rows);
                            }
                            last_error = format!(
                                "Census Geocoder batch response has none of the {} uploaded rows: \
                                 {}",
                                requests.len(),
                                error_reason(&bytes)
                            );
                        },
                        Err(BodyError::TooLarge(cap)) => {
                            return Err(CensusError::Transient(format!(
                                "Census Geocoder batch response is larger than {cap} bytes."
                            )));
                        },
                        Err(BodyError::Transport(e)) => {
                            last_error = format!("Census Geocoder batch response error: {e}");
                        },
                    },
                    400 | 404 => {
                        let body = read_capped(resp, 4096).await.unwrap_or_default();
                        return Err(fatal_status(status, &url, &body));
                    },
                    408 | 429 | 500..=599 => {
                        delay = retry_after(&resp);
                        last_error =
                            format!("Census Geocoder batch upload returned HTTP {status}.");
                    },
                    _ => {
                        return Err(CensusError::Transient(format!(
                            "Census Geocoder batch upload returned HTTP {status}."
                        )));
                    },
                }
            },
        }
        if attempt < MAX_RETRIES {
            let backoff = delay.unwrap_or_else(|| retry_base * 2u32.pow(attempt));
            tokio::time::sleep(backoff.min(Duration::from_secs(60))).await;
        }
    }
    Err(CensusError::Transient(last_error))
}

/// What a row resolved to.
#[derive(Clone)]
enum RowOutcome {
    /// Nothing to look up (empty or unparseable input, an incomplete parsed address).
    NoQuery,
    /// The lookup failed (a transient error after retries) - not cached.
    Failed,
    /// No match.
    NoMatch,
    /// Matched: the rendered output value(s) - one, or one per `%dyncols:` column.
    Hit(Vec<String>),
}

/// Geocode US addresses or coordinates via the US Census Geocoder.
///
/// Rows are read in `--batch`-sized chunks. Within a chunk, rows are turned into requests, the
/// requests are de-duplicated and checked against the on-disk cache, and the remainder are sent
/// concurrently (`--jobs` in flight, all behind one rate limiter) with order-preserving
/// `buffered`, so output order matches input order.
#[allow(clippy::future_not_send)]
pub(super) async fn run_census(args: Args, now_cmd: bool, cache_dir: &Path) -> CliResult<()> {
    let mode = InputMode::from_args(&args);
    let params = parse_params(args.flag_params.as_deref())?;
    let base_url = census_base_url();
    // run() resolved both to their defaults
    let benchmark = args.flag_benchmark.clone().unwrap_or_default();
    let vintage = args.flag_vintage.clone().unwrap_or_default();

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

    let formatstr = args.flag_formatstr.as_str();
    let dyncols_mode = formatstr.starts_with("%dyncols:");
    let (column_names, column_keys) = if dyncols_mode {
        parse_dyncols_spec(formatstr)?
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

    let json_output = now_cmd && matches!(formatstr, "%json" | "%pretty-json");

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
    let column_index = first_selected_column(&sel)?;

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
        // a redirect means a misconfigured QSV_CENSUS_GEOCODER_URL: report it, don't follow it
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    // allow_burst(1) avoids an initial burst (see governor#39); run() rejects --rate-limit 0
    let rate_limit = NonZeroU32::new(args.flag_rate_limit.unwrap_or(DEFAULT_CENSUS_RATE_LIMIT))
        .unwrap_or(NonZeroU32::MIN);
    let limiter = RateLimiter::direct(Quota::per_second(rate_limit).allow_burst(NonZeroU32::MIN));

    let disk_cache = if args.flag_no_cache {
        None
    } else {
        let mut cache_builder = RedbCache::builder(CENSUS_CACHE_NAME)
            .disk_dir(cache_dir)
            .refresh_on_hit(false)
            .durable(false);
        if args.flag_cache_ttl != 0 {
            cache_builder = cache_builder.ttl(Duration::from_secs(args.flag_cache_ttl));
        }
        let cache: RedbCache<String, String> = cache_builder.build().map_err(|e| {
            CliError::Other(format!("Error building Census Geocoder disk cache: {e}"))
        })?;
        if let Err(e) = cache.remove_expired_entries() {
            log::warn!("Could not remove expired Census Geocoder cache entries: {e}");
        }
        Some(cache)
    };

    // a cached batch result only serves an output the batch endpoints could have produced
    let batch_ok = batch_eligible(formatstr, &vintage);
    let render = |obj: &Value| -> Vec<String> {
        if dyncols_mode {
            column_keys
                .iter()
                .map(|key| json_dotpath(obj, key).trim().to_string())
                .collect()
        } else {
            vec![format_census_result(obj, formatstr)]
        }
    };
    let invalid_result = args.flag_invalid_result.clone().unwrap_or_default();

    let concurrency = util::njobs(args.flag_jobs).max(1);
    let batch_size = if args.flag_batch == 0 {
        usize::MAX
    } else {
        args.flag_batch
    };
    let retry_base = retry_base();
    let census_mode = CensusMode::parse(args.flag_census_mode.as_deref())?;
    let batch_chunk = batch_chunk_size();
    let batch_timeout = timeout.max(MIN_BATCH_TIMEOUT);
    // one upload in flight, and a pause between uploads: twice the retry base, so 2s by default
    let batch_gap = retry_base * 2;
    let mut last_upload: Option<tokio::time::Instant> = None;
    let mut tie_rows: u64 = 0;

    let mut transient_failures: u64 = 0;
    let mut consecutive_transient: u32 = 0;
    let mut rows_written: u64 = 0;

    let mut record = csv::StringRecord::new();
    let mut chunk: Vec<csv::StringRecord> = Vec::new();
    loop {
        chunk.clear();
        while chunk.len() < batch_size && rdr.read_record(&mut record)? {
            chunk.push(record.clone());
        }
        if chunk.is_empty() {
            break;
        }

        let requests: Vec<Option<Request>> = chunk
            .iter()
            .map(|rec| {
                let cell = rec.get(column_index).unwrap_or_default();
                prepare_request(mode, &base_url, &benchmark, &vintage, cell, rec, &sources)
            })
            .collect();

        // per-chunk: key -> rendered outcome. Keeping it per chunk bounds memory; the disk cache
        // de-duplicates across chunks.
        let mut resolved: HashMap<&str, RowOutcome> = HashMap::new();
        let mut failed: HashSet<&str> = HashSet::new();
        let mut to_fetch: Vec<&Request> = Vec::new();
        let mut queued: HashSet<&str> = HashSet::new();
        for req in requests.iter().flatten() {
            let key = req.key.as_str();
            if resolved.contains_key(key) || queued.contains(key) {
                continue;
            }
            // an undecodable entry - or a batch entry when this output needs a single lookup -
            // is a miss: it is fetched (and overwritten) again
            if let Some(dc) = &disk_cache
                && let Ok(Some(encoded)) = dc.cache_get(&req.key)
            {
                match decode_cached(&encoded) {
                    Some(Cached::Full(obj)) => {
                        resolved.insert(key, RowOutcome::Hit(render(&obj)));
                        continue;
                    },
                    Some(Cached::Batch(obj)) if batch_ok => {
                        resolved.insert(key, RowOutcome::Hit(render(&obj)));
                        continue;
                    },
                    Some(Cached::NoMatch) => {
                        resolved.insert(key, RowOutcome::NoMatch);
                        continue;
                    },
                    _ => {},
                }
            }
            queued.insert(key);
            to_fetch.push(req);
        }

        let use_batch = match census_mode {
            CensusMode::Single => false,
            // run() made sure the output is batch-eligible
            CensusMode::Batch => true,
            CensusMode::Auto => batch_ok && to_fetch.len() >= BATCH_THRESHOLD,
        };
        let mut ties: HashSet<&str> = HashSet::new();
        // the lookups for the single-record endpoint: all of them, or (after batch uploads) the
        // batch matches a batch result cannot describe correctly
        let mut single_fetch: Vec<&Request> = Vec::new();
        if use_batch {
            for group in to_fetch.chunks(batch_chunk) {
                if let Some(last) = last_upload {
                    tokio::time::sleep_until(last + batch_gap).await;
                }
                let result = census_batch_fetch(
                    &client,
                    &base_url,
                    &benchmark,
                    &vintage,
                    mode,
                    group,
                    batch_timeout,
                    retry_base,
                )
                .await;
                last_upload = Some(tokio::time::Instant::now());
                match result {
                    Ok(mut rows) => {
                        consecutive_transient = 0;
                        for (id, req) in group.iter().enumerate() {
                            let key = req.key.as_str();
                            match rows.remove(&id) {
                                Some(BatchRow::Match(obj)) if needs_single_lookup(&obj) => {
                                    single_fetch.push(req);
                                },
                                Some(BatchRow::Match(obj)) => {
                                    if let Some(dc) = &disk_cache
                                        && let Some(encoded) =
                                            encode_cached(&Cached::Batch(obj.clone()))
                                    {
                                        let _ = dc.cache_set(key.to_string(), encoded);
                                    }
                                    resolved.insert(key, RowOutcome::Hit(render(&obj)));
                                },
                                // batch matching varies with the load upstream: not cached
                                Some(BatchRow::NoMatch) => {
                                    resolved.insert(key, RowOutcome::NoMatch);
                                },
                                Some(BatchRow::Tie) => {
                                    ties.insert(key);
                                    resolved.insert(key, RowOutcome::NoMatch);
                                },
                                None => {
                                    log::warn!(
                                        "Census Geocoder batch response has no row for {key:?}"
                                    );
                                    transient_failures += 1;
                                    failed.insert(key);
                                },
                            }
                        }
                    },
                    Err(CensusError::Fatal(msg)) => {
                        wtr.flush()?;
                        return fail_clierror!("{msg}");
                    },
                    Err(CensusError::Transient(msg)) => {
                        log::warn!(
                            "Census Geocoder batch upload of {} rows failed: {msg}",
                            group.len()
                        );
                        transient_failures += group.len() as u64;
                        consecutive_transient += 1;
                        failed.extend(group.iter().map(|req| req.key.as_str()));
                        if consecutive_transient >= CIRCUIT_BREAKER_THRESHOLD {
                            wtr.flush()?;
                            return fail_clierror!(
                                "Aborting: {consecutive_transient} Census Geocoder batch uploads \
                                 in a row failed (last error: {msg}). The service may be down - \
                                 try again later. {rows_written} rows were written before the \
                                 abort."
                            );
                        }
                    },
                }
            }
        } else {
            single_fetch = to_fetch;
        }
        if !single_fetch.is_empty() {
            let client = &client;
            let limiter = &limiter;
            let (base_url, benchmark, vintage) =
                (base_url.as_str(), benchmark.as_str(), vintage.as_str());
            let mut responses =
                futures::stream::iter(single_fetch.into_iter().map(|req| async move {
                    let result = census_fetch(
                        client, limiter, base_url, benchmark, vintage, mode, &req.query, retry_base,
                    )
                    .await;
                    (req.key.as_str(), result)
                }))
                .buffered(concurrency);

            while let Some((key, result)) = responses.next().await {
                match result {
                    Ok(found) => {
                        consecutive_transient = 0;
                        let cached = found.map_or(Cached::NoMatch, Cached::Full);
                        if let Some(dc) = &disk_cache
                            && let Some(encoded) = encode_cached(&cached)
                        {
                            let _ = dc.cache_set(key.to_string(), encoded);
                        }
                        let outcome = match &cached {
                            Cached::Full(obj) | Cached::Batch(obj) => RowOutcome::Hit(render(obj)),
                            Cached::NoMatch => RowOutcome::NoMatch,
                        };
                        resolved.insert(key, outcome);
                    },
                    Err(CensusError::Fatal(msg)) => {
                        wtr.flush()?;
                        return fail_clierror!("{msg}");
                    },
                    Err(CensusError::Transient(msg)) => {
                        log::warn!("Census Geocoder lookup failed for {key:?}: {msg}");
                        transient_failures += 1;
                        consecutive_transient += 1;
                        failed.insert(key);
                        if consecutive_transient >= CIRCUIT_BREAKER_THRESHOLD {
                            wtr.flush()?;
                            return fail_clierror!(
                                "Aborting: {consecutive_transient} Census Geocoder requests in a \
                                 row failed (last error: {msg}). The service may be down or \
                                 limiting requests - lower --rate-limit or try again later. \
                                 {rows_written} rows were written before the abort."
                            );
                        }
                    },
                }
            }
            drop(responses);
        }

        for (rec, req) in chunk.iter().zip(&requests) {
            let outcome = match req {
                None => RowOutcome::NoQuery,
                Some(req) if failed.contains(req.key.as_str()) => RowOutcome::Failed,
                Some(req) => resolved
                    .get(req.key.as_str())
                    .cloned()
                    .unwrap_or(RowOutcome::NoMatch),
            };
            if let Some(req) = req
                && ties.contains(req.key.as_str())
            {
                tie_rows += 1;
            }
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
                let out_cell = match outcome {
                    // an empty <column> cell is left untouched, as with opencage
                    RowOutcome::NoQuery if mode != InputMode::Parsed && cell.trim().is_empty() => {
                        cell.to_string()
                    },
                    RowOutcome::Hit(mut values) => values.pop().unwrap_or_default(),
                    RowOutcome::NoQuery | RowOutcome::Failed | RowOutcome::NoMatch => {
                        if invalid_result.is_empty() {
                            cell.to_string()
                        } else {
                            invalid_result.clone()
                        }
                    },
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
    if tie_rows > 0 {
        wwarn!(
            "{tie_rows} row(s) matched more than one address in a Census batch lookup (a \
             \"Tie\"), so they were {fate}. The batch endpoints do not say which address is \
             right; --census-mode single uses the first match (_resultCount tells how many there \
             were)."
        );
    }
    if transient_failures > 0 {
        wwarn!(
            "{transient_failures} Census Geocoder lookup(s) failed after retries (network error, \
             rate limit or server error), so those rows were {fate}. Set QSV_LOG_LEVEL=warn to \
             log each failure to the qsv log file."
        );
    }
    Ok(wtr.flush()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn params_validation() {
        let p = parse_params(Some("street={Street}, zip=20233")).unwrap();
        assert_eq!(
            p[0],
            (
                "street".to_string(),
                ParamValue::Column("Street".to_string())
            )
        );
        assert_eq!(
            p[1],
            ("zip".to_string(), ParamValue::Literal("20233".to_string()))
        );
        assert!(parse_params(Some("street={S},city={C},state=DC")).is_ok());
        assert!(parse_params(None).unwrap().is_empty());
        // the Census Geocoder answers HTTP 400 to anything less (live probe, 2026-10-12)
        assert!(parse_params(Some("street={S}")).is_err());
        assert!(parse_params(Some("street={S},state=DC")).is_err());
        assert!(parse_params(Some("street={S},city={C}")).is_err());
        assert!(parse_params(Some("city={C},state=DC,zip=1")).is_err());
        assert!(parse_params(Some("")).is_err());
        // names are case-sensitive; unknown names, duplicates and empties are errors
        assert!(parse_params(Some("Street={S},zip=1")).is_err());
        assert!(parse_params(Some("street={S},zip=1,county=X")).is_err());
        assert!(parse_params(Some("street={S},street={T},zip=1")).is_err());
        assert!(parse_params(Some("street={},zip=1")).is_err());
        assert!(parse_params(Some("street={S},zip=")).is_err());
    }

    #[test]
    fn complete_parsed_addresses() {
        let has = |names: &'static [&'static str]| move |n: &str| names.contains(&n);
        assert!(parsed_address_complete(has(&["street", "zip"])));
        assert!(parsed_address_complete(has(&["street", "city", "state"])));
        assert!(parsed_address_complete(has(&[
            "street", "city", "state", "zip"
        ])));
        assert!(!parsed_address_complete(has(&["street", "city"])));
        assert!(!parsed_address_complete(has(&["city", "state", "zip"])));
    }

    #[test]
    fn modes() {
        assert_eq!(CensusMode::parse(None).unwrap(), CensusMode::Auto);
        assert_eq!(
            CensusMode::parse(Some(" BATCH ")).unwrap(),
            CensusMode::Batch
        );
        assert_eq!(
            CensusMode::parse(Some("single")).unwrap(),
            CensusMode::Single
        );
        assert!(CensusMode::parse(Some("bulk")).is_err());
    }

    #[test]
    fn census_geoids_by_vintage() {
        let current = preset_dyncols("%census-geoids", "Current_Current").unwrap();
        let (names, keys) = parse_dyncols_spec(&current).unwrap();
        assert_eq!(
            names,
            [
                "census_state",
                "census_county",
                "census_tract",
                "census_blockgroup",
                "census_block"
            ]
        );
        assert_eq!(keys[4], "flat.2020 Census Blocks.GEOID");
        let (_, keys) =
            parse_dyncols_spec(&preset_dyncols("%census-geoids", "Census2020_Current").unwrap())
                .unwrap();
        assert_eq!(keys[4], "flat.Census Blocks.GEOID");
        assert_eq!(keys[2], "flat.Census Tracts.GEOID");
        assert_eq!(preset_dyncols("%lat-long", "Current_Current"), None);
    }

    #[test]
    fn batch_eligibility() {
        let cur = "Current_Current";
        for ok in [
            "%+",
            "%lat-long",
            "%location",
            "%matched-address",
            "{matchedAddress} ({coordinates.y}, {coordinates.x})",
            "%dyncols: {a:matchedAddress}, {t:flat.Census Tracts.TRACT}, {m:_matchType}",
        ] {
            assert!(batch_eligible(ok, cur), "{ok}");
        }
        assert!(batch_eligible(
            &preset_dyncols("%census-geoids", cur).unwrap(),
            cur
        ));
        for single_only in [
            "%json",
            "%pretty-json",
            "%dyncols: {a:matchedAddress}, {n:flat.States.NAME}",
            "%dyncols: {c:_resultCount}",
            "%dyncols: {z:addressComponents.zip}",
            "%dyncols: {u:flat.Urban Areas.GEOID}",
            "{matchedAddress} {addressComponents.zip}",
        ] {
            assert!(!batch_eligible(single_only, cur), "{single_only}");
        }
        // other vintages name their layers differently: never batch
        assert!(!batch_eligible("%lat-long", "Census2020_Current"));
        assert!(!batch_eligible("%lat-long", "ACS2026_Current"));
    }

    const ADDRESS_FIXTURE: &str =
        include_str!("../../../resources/test/census_geo_address_all.json");
    const COORDS_FIXTURE: &str = include_str!("../../../resources/test/census_geo_coords_all.json");

    #[test]
    fn normalize_captured_responses() {
        let body: Value = serde_json::from_str(ADDRESS_FIXTURE).unwrap();
        let obj = normalize_single(InputMode::OneLine, &body).unwrap();
        assert_eq!(
            obj["matchedAddress"],
            json!("4600 SILVER HILL RD, WASHINGTON, DC, 20233")
        );
        assert_eq!(
            format_census_result(&obj, "%lat-long"),
            "38.84505589808, -76.92836638093"
        );
        assert_eq!(obj["tigerLine"]["tigerLineId"], json!("657091557"));
        assert_eq!(obj["_resultCount"], json!(1));
        assert_eq!(obj["geographies"].as_object().unwrap().len(), 19);
        let geoids: Vec<String> = parse_dyncols_spec(&census_geoids_dyncols(DEFAULT_VINTAGE))
            .unwrap()
            .1
            .iter()
            .map(|k| json_dotpath(&obj, k))
            .collect();
        assert_eq!(
            geoids,
            [
                "24",
                "24033",
                "24033802405",
                "240338024052",
                "240338024052004"
            ]
        );
        // flat keeps scalars only, in the layer's attribute order
        assert_eq!(obj["flat"]["States"]["STATE"], json!("24"));
        assert!(obj["flat"]["States"]["AREALAND"].is_number());

        let body: Value = serde_json::from_str(COORDS_FIXTURE).unwrap();
        let obj = normalize_single(InputMode::Coordinates, &body).unwrap();
        assert_eq!(
            format_census_result(&obj, "%location"),
            "(38.84601, -76.92744)"
        );
        assert_eq!(
            obj["flat"]["2020 Census Blocks"]["GEOID"],
            json!("240338024052004")
        );
        assert!(obj.get("matchedAddress").is_none());

        // no address match, or a point with no geographies: no match
        let none = json!({"result": {"input": {}, "addressMatches": []}});
        assert_eq!(normalize_single(InputMode::OneLine, &none), None);
        let at_sea = json!({"result": {"input": {"location": {"x": -175.0, "y": 30.0}},
            "geographies": {"States": [], "Counties": []}}});
        assert_eq!(normalize_single(InputMode::Coordinates, &at_sea), None);
    }

    #[test]
    fn coordinates_render_the_same_from_numbers_and_text() {
        let from_json = coordinates_object(&json!({"x": -76.92836638093, "y": 38.84505589808}));
        let from_text = coordinates_object(&json!({"x": "-76.92836638093", "y": "38.84505589808"}));
        assert_eq!(from_json, from_text);
        assert_eq!(coordinates_object(&json!({"x": "a", "y": 1})), None);
    }

    #[test]
    fn requests_and_cache_keys() {
        let key = |mode, cell: &str, rec: &[&str], sources: &[(String, ParamSource)]| {
            prepare_request(
                mode,
                "http://x",
                "B",
                "V",
                cell,
                &csv::StringRecord::from(rec.to_vec()),
                sources,
            )
            .map(|r| r.key)
        };
        // case and whitespace fold into one key
        assert_eq!(
            key(InputMode::OneLine, " 1  Main st ", &[], &[]),
            key(InputMode::OneLine, "1 MAIN ST", &[], &[])
        );
        assert_eq!(
            key(InputMode::OneLine, "1 Main St", &[], &[]).unwrap(),
            "v1|http://x|onelineaddress|B|V|1 MAIN ST"
        );
        assert_eq!(key(InputMode::OneLine, "  ", &[], &[]), None);
        // coordinates: 6 decimals, lon first
        let req = prepare_request(
            InputMode::Coordinates,
            "http://x",
            "B",
            "V",
            "(38.84601, -76.92744)",
            &csv::StringRecord::new(),
            &[],
        )
        .unwrap();
        assert_eq!(req.key, "v1|http://x|coordinates|B|V|-76.927440,38.846010");
        assert_eq!(
            req.query,
            [
                ("x", "-76.927440".to_string()),
                ("y", "38.846010".to_string())
            ]
        );
        assert_eq!(req.batch, ["-76.927440", "38.846010"]);
        assert_eq!(key(InputMode::Coordinates, "91, 0", &[], &[]), None);
        // parsed: fixed part order, empty cells not sent, incomplete rows not sent at all
        let headers = csv::StringRecord::from(vec!["S", "C", "Z"]);
        let params = parse_params(Some("zip={Z},street={S},city={C},state=dc")).unwrap();
        let sources = resolve_param_sources(&params, &headers).unwrap();
        assert_eq!(
            key(InputMode::Parsed, "", &["1 main st", "Town", ""], &sources).unwrap(),
            r#"v1|http://x|address|B|V|["1 MAIN ST","TOWN","DC",""]"#
        );
        // a separator inside a part cannot make two addresses share a key
        assert_ne!(
            key(
                InputMode::Parsed,
                "",
                &["1 Main St;Town", "New", "1"],
                &sources
            ),
            key(
                InputMode::Parsed,
                "",
                &["1 Main St", "Town;New", "1"],
                &sources
            )
        );
        assert_eq!(
            key(InputMode::Parsed, "", &["1 main st", "", ""], &sources),
            None
        );
        let req = prepare_request(
            InputMode::Parsed,
            "http://x",
            "B",
            "V",
            "",
            &csv::StringRecord::from(vec!["1 main st", "", "20233"]),
            &sources,
        )
        .unwrap();
        assert_eq!(
            req.query,
            [
                ("street", "1 main st".to_string()),
                ("state", "dc".to_string()),
                ("zip", "20233".to_string())
            ]
        );
        assert_eq!(req.batch, ["1 main st", "", "dc", "20233"]);
    }

    #[test]
    fn cache_round_trip() {
        for cached in [
            Cached::Full(json!({"a": 1})),
            Cached::Batch(json!({"b": 2})),
            Cached::NoMatch,
        ] {
            let encoded = encode_cached(&cached).unwrap();
            assert_eq!(decode_cached(&encoded), Some(cached));
        }
        // anything else is a miss, not a non-match
        for bad in ["", "Nx", "F{not json", "X{}"] {
            assert_eq!(decode_cached(bad), None, "{bad}");
        }
    }

    #[test]
    fn census_error_reasons() {
        assert_eq!(
            error_reason(br#"{"errors":["Vintage must be specified"],"status":"400"}"#),
            "Vintage must be specified"
        );
        assert!(
            error_reason(
                b"<p>An error occurred because the file exceeded the batch limit.<br> More</p>"
            )
            .starts_with("An error occurred because the file exceeded the batch limit. More")
        );
    }

    /// Live captures (2026-10-12): `batch_geo.csv`, `batch_tie.csv`, `batch_coords*.csv`, and a
    /// Match row without geography columns (`batch_widerow.csv`, a locations batch).
    #[test]
    fn batch_rows() {
        let geo = br#""1","4600 Silver Hill Rd, Washington, DC, 20233","Match","Exact","4600 SILVER HILL RD, WASHINGTON, DC, 20233","-76.92836638093,38.84505589808","657091557","L","24","033","802405","2004"
"2","zzzz nowhere, , , ","No_Match"
"3","100 Main St, , MA, ","Tie"
"4","4600 Silver Hill Rd, Washington, DC, 20233","Match","Non_Exact","4600 SILVER HILL RD, WASHINGTON, DC, 20233","-76.92836638093,38.84505589808","657091557","L"
"#;
        let rows = parse_batch_response(InputMode::OneLine, geo).unwrap();
        let Some(BatchRow::Match(obj)) = rows.get(&1) else {
            panic!("{rows:?}");
        };
        assert_eq!(
            format_census_result(obj, "%lat-long"),
            "38.84505589808, -76.92836638093"
        );
        assert_eq!(obj["coordinates"]["x"], json!(-76.92836638093));
        assert_eq!(obj["_matchType"], json!("Exact"));
        assert_eq!(
            obj["tigerLine"],
            json!({"tigerLineId": "657091557", "side": "L"})
        );
        let (_, keys) = parse_dyncols_spec(&census_geoids_dyncols(DEFAULT_VINTAGE)).unwrap();
        let geoids: Vec<String> = keys.iter().map(|k| json_dotpath(obj, k)).collect();
        assert_eq!(
            geoids,
            [
                "24",
                "24033",
                "24033802405",
                "240338024052",
                "240338024052004"
            ]
        );
        // the batch subset agrees with the captured single-record result, attribute for attribute
        let single: Value = serde_json::from_str(ADDRESS_FIXTURE).unwrap();
        let single = normalize_single(InputMode::OneLine, &single).unwrap();
        for (layer, attrs) in BATCH_LAYERS {
            for attr in attrs {
                let key = format!("flat.{layer}.{attr}");
                assert!(is_batch_key(&key));
                assert_eq!(
                    json_dotpath(obj, &key),
                    json_dotpath(&single, &key),
                    "{key}"
                );
            }
        }
        assert_eq!(rows.get(&2), Some(&BatchRow::NoMatch));
        assert_eq!(rows.get(&3), Some(&BatchRow::Tie));
        // no geography columns: a match without flat layers
        let Some(BatchRow::Match(obj)) = rows.get(&4) else {
            panic!("{rows:?}");
        };
        assert_eq!(obj["flat"], json!({}));
        assert_eq!(obj["_matchType"], json!("Non_Exact"));

        let coords = br#""1","-40","30","No_Match"
"2","-76.92744","38.84601","Match","24","033","802405","2004"
"#;
        let rows = parse_batch_response(InputMode::Coordinates, coords).unwrap();
        assert_eq!(rows.get(&1), Some(&BatchRow::NoMatch));
        let Some(BatchRow::Match(obj)) = rows.get(&2) else {
            panic!("{rows:?}");
        };
        assert_eq!(
            format_census_result(obj, "%location"),
            "(38.84601, -76.92744)"
        );
        assert_eq!(obj["flat"]["Census Tracts"]["GEOID"], json!("24033802405"));
        assert!(obj.get("matchedAddress").is_none());
        // an empty answer (an empty upload) is no rows, not an error
        assert!(
            parse_batch_response(InputMode::OneLine, b"")
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn connecticut_batch_matches_need_a_single_lookup() {
        let ct = br#""1","210 Capitol Ave, Hartford, CT, 06106","Match","Exact","210 CAPITOL AVE, HARTFORD, CT, 06106","-72.682254053971,41.762863585706","3541159","R","09","003","502100","2016"
"2","4600 Silver Hill Rd, Washington, DC, 20233","Match","Exact","4600 SILVER HILL RD, WASHINGTON, DC, 20233","-76.92836638093,38.84505589808","657091557","L","24","033","802405","2004"
"#;
        let rows = parse_batch_response(InputMode::Parsed, ct).unwrap();
        let needs = |id| match rows.get(&id) {
            Some(BatchRow::Match(obj)) => needs_single_lookup(obj),
            other => panic!("{other:?}"),
        };
        assert!(needs(1));
        assert!(!needs(2));
    }

    #[test]
    fn partial_geographies() {
        assert_eq!(synthesize_flat("", "033", "802405", "2004"), json!({}));
        let flat = synthesize_flat("24", "033", "", "");
        assert_eq!(flat["Counties"]["GEOID"], json!("24033"));
        assert!(flat.get("Census Tracts").is_none());
        let flat = synthesize_flat("24", "033", "802405", "");
        assert_eq!(flat["Census Tracts"]["GEOID"], json!("24033802405"));
        assert!(flat.get("Census Block Groups").is_none());
    }

    #[test]
    fn batch_upload_file() {
        let req = |batch: &[&str]| Request {
            key:   String::new(),
            query: Vec::new(),
            batch: batch.iter().map(ToString::to_string).collect(),
        };
        let a = req(&["4600 Silver Hill Rd, Washington, DC 20233", "", "", ""]);
        let b = req(&["1 Main St", "Town", "NY", "10001"]);
        assert_eq!(
            String::from_utf8(batch_file(&[&a, &b]).unwrap()).unwrap(),
            "0,\"4600 Silver Hill Rd, Washington, DC 20233\",,,\n1,1 Main St,Town,NY,10001\n"
        );
    }
}
