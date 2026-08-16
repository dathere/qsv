//! US Census `TIGERweb` boundary resolution for `viz --geojson auto|census`.
//!
//! `--geojson auto` turns a dataset that merely *names* US standard jurisdictions (county FIPS
//! today; ZCTA/tract/place to follow) into a choropleth without the user hunting down a boundary
//! file. This module owns the fetch side of that: service discovery, layer resolution, scoping,
//! pagination and assembly into a single GeoJSON `FeatureCollection`.
//!
//! # Four facts about `TIGERweb` that the design hangs on
//!
//! These were established by probing the live service; each one invalidates a plausible-looking
//! shortcut, so they are recorded here rather than rediscovered.
//!
//! 1. **Layer ids are NOT stable across vintages.** `Counties` is layer 82 in `tigerWMS_ACS2023`
//!    and `tigerWMS_Current`, 84 in `tigerWMS_ACS2019`, and 78 in `tigerWMS_ACS2021`. Hardcoding an
//!    id does not fail loudly — it silently fetches a *different geography*. Layers are therefore
//!    always resolved by NAME against the `MapServer`'s own catalog ([`resolve_layer_id`]).
//! 2. **`properties.GEOID` is the canonical feature id for every layer we target**, including
//!    ZCTAs, where `GEOID == ZCTA5`. The `ZCTA5CE20`/`ZCTA5CE10` field names that appear in the
//!    Census *shapefiles* do not exist in `TIGERweb`, so there is no per-layer id special-casing.
//! 3. **The ZCTA layer carries no `STATE` field** (its fields are `OID, ZCTA5, GEOID, BASENAME,
//!    LSADC, NAME, MTFCC, ZCTA5CC, FUNCSTAT, AREALAND, ...`). State-scoping is thus impossible for
//!    ZCTAs and they must be scoped by exact code set instead — which is also why deriving a state
//!    set from ZIP prefixes was never viable: ZIP prefixes cross state lines.
//! 4. **Vintage is discoverable, not computable.** The service catalog lists the vintages that
//!    actually exist, so [`latest_acs_vintage`] reads them rather than assuming "current year minus
//!    one", which rots annually.
//!
//! Pagination uses `orderByFields` + `resultOffset`/`resultRecordCount`: verified to yield stable,
//! non-overlapping pages. The response envelope sets `exceededTransferLimit: true` while more
//! records remain and omits it on the final page; [`query_layer_geojson`] treats either that flag
//! or a short page as the stop condition.

use std::io::Read as _;

use crate::{CliResult, util};

/// Root of the `TIGERweb` `ArcGIS` REST catalog.
const TIGERWEB_ROOT: &str = "https://tigerweb.geo.census.gov/arcgis/rest/services";

/// The catalog root to actually use, honoring `QSV_CENSUS_TIGERWEB_URL`.
///
/// The override exists for users behind a mirror or an offline replica of the service, and it is
/// what lets the request-level behavior — state scoping, pagination, and the zero-network cache
/// hit — be asserted against a mock server rather than the live Census endpoint.
fn tigerweb_root() -> String {
    std::env::var("QSV_CENSUS_TIGERWEB_URL").unwrap_or_else(|_| TIGERWEB_ROOT.to_string())
}

/// Per-request timeout for `TIGERweb` calls. Boundary payloads are large (an ungeneralized
/// 67-county state runs ~7 MB), so this is more generous than the plain `--geojson` URL fetch.
/// Honors `QSV_TIMEOUT` via [`util::timeout_secs`], like every other network path in qsv.
const CENSUS_FETCH_TIMEOUT_SECS: u16 = 60;

/// Safety ceiling on a single `TIGERweb` response body, mirroring `viz`'s `GEOJSON_MAX_BYTES`.
/// A CEILING, not a target: state-scoped county layers are single-digit MB.
const CENSUS_MAX_BYTES: usize = 512_000_000;

/// Records requested per page. Well under the service's advertised `maxRecordCount` (100000) so
/// the server never silently truncates below our own paging, and small enough that a single
/// oversized page cannot blow the byte ceiling on geometry-bearing layers.
const PAGE_SIZE: usize = 500;

/// Region codes per `IN (...)` clause. Each code costs ~8 URL characters, so this keeps a request
/// URL a few KB — comfortably inside every proxy's line limit — while still resolving a few
/// thousand ZCTAs in a handful of round trips.
const CODES_PER_REQUEST: usize = 200;

/// Minimum fraction of codes a layer must resolve for `auto` to consider it at all.
const LAYER_PROBE_MIN_RATIO: f64 = 0.5;

/// How long a cached boundary set stays fresh, in days. Overridable with
/// `QSV_VIZ_BOUNDARY_CACHE_TTL_DAYS`.
///
/// Boundaries are annual-vintage artifacts, so this is long by qsv standards. It exists so a new
/// vintage is eventually picked up, not to bound staleness within a session.
const BOUNDARY_CACHE_TTL_DAYS: u64 = 30;

/// Cache subdirectory under the qsv cache dir.
const BOUNDARY_CACHE_SUBDIR: &str = "~/.qsv-cache/viz-boundaries";

/// Sidecar recorded next to a cached boundary file.
///
/// The resolved vintage and layer live here rather than in the cache KEY: resolving them requires
/// network (the service catalog, then the probe), so a key built from them could never produce a
/// zero-network second run. The key is built from the INPUTS instead, and this records what those
/// inputs resolved to so a hit can report identical provenance without asking the service again.
#[derive(serde::Serialize, serde::Deserialize)]
struct CachedMeta {
    feature_id_key: String,
    provenance:     String,
    scope_key:      String,
    /// Unix seconds at fetch time, for the TTL check.
    fetched_at:     u64,
}

/// A resolved boundary set, ready to hand to `viz`'s existing `--geojson` consumers.
///
/// Deliberately provider-agnostic: the eventual second provider (Eurostat GISCO/NUTS is the
/// obvious one) returns this same shape, so nothing downstream learns that Census exists.
pub struct BoundarySet {
    /// The assembled `FeatureCollection`.
    pub geojson:        serde_json::Value,
    /// Feature-id path for `--feature-id-key`, e.g. `properties.GEOID`.
    pub feature_id_key: String,
    /// Human-readable provenance for the panel subtitle / sidecar.
    pub provenance:     String,
    /// On-disk path of the materialized boundary document. Empty until it is written; several
    /// `--geojson` consumers re-read the flag as a path, so this is what they get.
    pub path:           String,
    /// Stable identity of exactly what was fetched: provider, layer, vintage and the full scope.
    ///
    /// This is the cache key, and it must name the scope itself rather than summarize it — a key
    /// built from the provenance string would fold "2 states" for PA+OH and NY+NJ onto the same
    /// entry, which is invisible while every run overwrites but starts serving one dataset's
    /// boundaries to another the moment a cache HIT is honored.
    pub scope_key:      String,
}

/// The US standard jurisdictions `--geojson auto` can resolve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    County,
    Zcta,
    Tract,
    Place,
}

impl Layer {
    /// Every layer `auto` will probe, in the order they are reported when the choice is ambiguous.
    pub const ALL: [Self; 4] = [Self::County, Self::Zcta, Self::Tract, Self::Place];

    /// Does a `MapServer` catalog entry name this layer?
    ///
    /// Counties match exactly, but the ZCTA layer's name carries its decennial delineation
    /// (`2010 Census ZIP Code Tabulation Areas` in ACS2019, `2020 Census ...` from ACS2021), so it
    /// is matched on the stable part. The FULL matched name is kept for provenance — the
    /// delineation is read from the service rather than inferred from the vintage.
    fn matches_catalog_name(self, name: &str) -> bool {
        match self {
            Self::County => name == "Counties",
            Self::Zcta => name.contains("ZIP Code Tabulation Areas"),
            Self::Tract => name == "Census Tracts",
            Self::Place => name == "Incorporated Places",
        }
    }

    /// Attribute holding the canonical feature id. `GEOID` for every layer — for ZCTAs it is
    /// identical to `ZCTA5` (module fact 2), so one field name serves both probe and fetch.
    const fn id_field(self) -> &'static str {
        "GEOID"
    }

    /// Width of a well-formed code for this layer, used to re-pad codes whose leading zero a
    /// numeric CSV column dropped.
    const fn code_width(self) -> usize {
        match self {
            // 2-digit state + 3-digit county
            Self::County => 5,
            // 5-digit ZIP Code Tabulation Area
            Self::Zcta => 5,
            // 2-digit state + 3-digit county + 6-digit tract
            Self::Tract => 11,
            // 2-digit state + 5-digit place
            Self::Place => 7,
        }
    }

    /// Human label used in errors and provenance.
    pub const fn label(self) -> &'static str {
        match self {
            Self::County => "county",
            Self::Zcta => "ZCTA",
            Self::Tract => "census tract",
            Self::Place => "place",
        }
    }

    /// The explicit `--geojson census:<name>` selector for this layer.
    #[must_use]
    pub const fn selector(self) -> &'static str {
        match self {
            Self::County => "census:county",
            Self::Zcta => "census:zcta",
            Self::Tract => "census:tract",
            Self::Place => "census:place",
        }
    }

    /// Parse an explicit `census:<layer>` selector.
    pub fn from_selector(spec: &str) -> Option<Self> {
        match spec {
            "census:county" | "census:counties" => Some(Self::County),
            "census:zcta" | "census:zip" => Some(Self::Zcta),
            "census:tract" | "census:tracts" => Some(Self::Tract),
            "census:place" | "census:places" => Some(Self::Place),
            _ => None,
        }
    }
}

/// Build the HTTP client used for every `TIGERweb` call.
///
/// Uses qsv's shared builder so `TIGERweb` inherits the same retry-on-503, compression, rustls and
/// (crucially) *both* the total and connect timeouts that every other qsv network path gets — a
/// hand-rolled `reqwest` client here would silently miss the connect timeout.
fn census_client() -> CliResult<reqwest::blocking::Client> {
    let timeout = util::timeout_secs(CENSUS_FETCH_TIMEOUT_SECS).map_err(crate::CliError::Other)?;
    util::create_reqwest_blocking_client(
        None,
        u16::try_from(timeout).unwrap_or(CENSUS_FETCH_TIMEOUT_SECS),
        Some(tigerweb_root()),
    )
}

/// Is a failed request worth answering from a stale cache entry?
///
/// Only a transient failure is: a semantic one (the service understood the request and refused)
/// must surface as itself rather than being reported as a connectivity problem and silently
/// answered with a previous run's boundaries.
///
/// `None` means no response arrived at all — a connect failure, a timeout, or a body that would
/// not decode — which is transient by definition.
fn status_is_transient(status: Option<reqwest::StatusCode>) -> bool {
    let Some(status) = status else {
        return true;
    };
    // 408 and 429 are 4xx but mean "ask again later", not "no". Both are realistic here rather
    // than theoretical: a large ZCTA set is fetched as a LOOP of chunked requests, which is
    // exactly the shape a public service rate-limits.
    if status == reqwest::StatusCode::REQUEST_TIMEOUT
        || status == reqwest::StatusCode::TOO_MANY_REQUESTS
    {
        return true;
    }
    !status.is_client_error()
}

/// GET `url` with `params` and parse the response as JSON, reading through a bounded `take` so an
/// endless or merely enormous body cannot be buffered unchecked (the same guard
/// `viz::load_geojson` applies).
///
/// Query parameters are passed to reqwest rather than interpolated into the URL: a `where` clause
/// carries quotes, commas and spaces that must be percent-encoded, and hand-rolling that is how
/// injection-shaped bugs get in.
/// Strip an API key out of any text that may quote a request URL.
///
/// NOT optional hygiene: `reqwest`'s own error `Display` embeds the full URL, query string
/// included, so interpolating an error into a message printed the Census key to stderr verbatim.
/// A key is a credential — it must not reach a log, a terminal, or a bug report — so every error
/// this module builds from a URL or an underlying error goes through here.
fn redact_api_key(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("key=") {
        // Redact unless "key=" is the tail of a longer word ("monkey="). Deliberately NOT
        // "preceded by ? or &": a wrapped or reformatted log line puts whitespace there, and a
        // credential that survives because a line broke in the wrong place is not protected.
        let is_param = at == 0
            || !matches!(
                rest.as_bytes()[at - 1],
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-'
            );
        out.push_str(&rest[..at + 4]);
        rest = &rest[at + 4..];
        if !is_param {
            continue;
        }
        let end = rest
            .find(|c: char| c == '&' || c == ')' || c == '\'' || c.is_whitespace())
            .unwrap_or(rest.len());
        out.push_str("REDACTED");
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

fn get_json(
    client: &reqwest::blocking::Client,
    url: &str,
    params: &[(&str, &str)],
) -> CliResult<serde_json::Value> {
    // `RequestBuilder::query` is behind a reqwest feature qsv does not enable, so the parameters
    // are encoded onto the URL here instead — same percent-encoding, no new build feature.
    let target = reqwest::Url::parse_with_params(url, params.iter().copied()).map_err(|e| {
        crate::CliError::Other(redact_api_key(&format!(
            "could not build Census URL '{url}': {e}"
        )))
    })?;
    // Classify the failure, because only a TRANSIENT one may be answered from a stale cache entry
    // (see `resolve`). `error_for_status` turns any 4xx into an error too, so wrapping everything
    // as `Network` would let a rejected query or a removed endpoint — the service answering us
    // perfectly well, with "no" — be reported as a connectivity problem AND silently served from a
    // previous run's boundaries.
    //
    //   * 408/429 -> "ask again later": transient despite being 4xx. Worth calling out for this
    //     feature in particular — a large ZCTA set is fetched as a LOOP of chunked requests, which
    //     is exactly the shape a public service rate-limits.
    //   * other 4xx -> the service understood and refused: semantic, surface it as itself.
    //   * 5xx  -> the service is unwell: transient. (503 is already retried by the shared client.)
    //   * none -> no response at all: connect failure, timeout, or a body that would not decode.
    let mut resp = match client
        .get(target)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
    {
        Ok(resp) => resp,
        Err(e) => {
            let msg = redact_api_key(&format!("Census request to '{url}' failed: {e}"));
            return Err(if status_is_transient(e.status()) {
                crate::CliError::Network(msg)
            } else {
                crate::CliError::Other(msg)
            });
        },
    };
    let mut buf: Vec<u8> = Vec::new();
    // one byte past the cap, so exceeding it is distinguishable from exactly reaching it
    resp.by_ref()
        .take(CENSUS_MAX_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| {
            crate::CliError::Network(redact_api_key(&format!(
                "reading Census response body: {e}"
            )))
        })?;
    if buf.len() > CENSUS_MAX_BYTES {
        return Err(crate::CliError::Other(format!(
            "Census response from '{url}' exceeds the {} MB limit. Narrow the dataset's \
             geographic extent, or supply an explicit --geojson file.",
            CENSUS_MAX_BYTES / 1_000_000
        )));
    }
    serde_json::from_slice(&buf).map_err(|e| {
        crate::CliError::Other(redact_api_key(&format!(
            "Census response is not valid JSON: {e}"
        )))
    })
}

/// Every ACS vintage the service publishes, ascending.
///
/// Read from the catalog rather than computed from the current year: the vintages present are a
/// property of the service (it listed `tigerWMS_ACS2012`..`tigerWMS_ACS2025` when this was
/// written), and any "this year minus one" rule silently drifts every January. The full list — not
/// just the newest — is what lets a vintage mismatch be diagnosed instead of merely reported.
pub fn available_acs_vintages(client: &reqwest::blocking::Client) -> CliResult<Vec<u16>> {
    let catalog = get_json(
        client,
        &format!("{}/TIGERweb", tigerweb_root()),
        &[("f", "json")],
    )?;
    let mut vintages: Vec<u16> = catalog
        .get("services")
        .and_then(serde_json::Value::as_array)
        .map(|services| {
            services
                .iter()
                .filter_map(|s| s.get("name").and_then(serde_json::Value::as_str))
                // names arrive folder-qualified, e.g. "TIGERweb/tigerWMS_ACS2023"
                .filter_map(|name| name.rsplit('/').next())
                .filter_map(|name| name.strip_prefix("tigerWMS_ACS"))
                .filter_map(|year| year.parse::<u16>().ok())
                .collect()
        })
        .unwrap_or_default();
    vintages.sort_unstable();
    vintages.dedup();
    if vintages.is_empty() {
        return Err(crate::CliError::Other(format!(
            "no ACS vintages found in the Census TIGERweb catalog at '{}/TIGERweb'. The service \
             may have been reorganized; supply an explicit --geojson file.",
            tigerweb_root()
        )));
    }
    Ok(vintages)
}

/// Resolve a layer's numeric id by NAME within a vintage's `MapServer`, returning
/// `(layer_id, catalog_name)`.
///
/// See module fact 1: ids move between vintages, and a stale id fetches the wrong geography
/// without erroring. Matching on the catalog's own name makes a rename fail loudly instead. The
/// catalog name comes back so provenance can quote the service's own wording — for ZCTAs that
/// string carries the delineation year, which is not derivable from the vintage.
fn resolve_layer_id(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layer: Layer,
) -> CliResult<(u64, String)> {
    let url = format!(
        "{}/TIGERweb/tigerWMS_ACS{vintage}/MapServer",
        tigerweb_root()
    );
    let meta = get_json(client, &url, &[("f", "json")])?;
    let found = meta
        .get("layers")
        .and_then(serde_json::Value::as_array)
        .and_then(|layers| {
            layers.iter().find_map(|l| {
                let name = l.get("name").and_then(serde_json::Value::as_str)?;
                if !layer.matches_catalog_name(name) {
                    return None;
                }
                Some((
                    l.get("id").and_then(serde_json::Value::as_u64)?,
                    name.to_string(),
                ))
            })
        });
    found.map_or_else(
        || {
            Err(crate::CliError::Other(format!(
                "--geojson auto: no {} layer in the Census TIGERweb {vintage} vintage. Supply an \
                 explicit --geojson file.",
                layer.label()
            )))
        },
        Ok,
    )
}

/// Normalize region codes for a layer: keep only codes that could BE one of its ids, and re-pad
/// those whose leading zero a numeric CSV column dropped (`7936` -> `07936`, `1001` -> `01001`).
///
/// Padding must happen BEFORE the query, not just when matching results back: a `where ... IN
/// ('7936')` clause matches no ZCTA at all, so an unpadded code would read as a nonexistent region
/// rather than a formatting artifact.
///
/// Codes are also filtered to a plausible width band for the layer — every Census id here is
/// all-digit and fixed-width, so a code outside `[width-2, width]` cannot be one no matter how it
/// is padded. That is what stops a 5-digit county dataset from wasting probes against the 7-digit
/// place and 11-digit tract layers. Note this filters only what is QUERIED; coverage is still
/// scored against the caller's original, unfiltered codes, so nothing is hidden from the honesty
/// check by being dropped here.
fn normalize_codes(codes: &[String], layer: Layer) -> Vec<String> {
    let width = layer.code_width();
    // How many leading zeros a numeric column can actually have eaten is a property of the id, not
    // a guess: a GEOID that starts with a state FIPS loses at most ONE (only states 01-09 have a
    // leading zero), whereas a ZCTA can lose TWO, because Puerto Rico's run 006xx-009xx and
    // `00601` arrives as `601`. Using the loosest band for every layer would make a 5-digit county
    // code look like a paddable 7-digit place id (`0042003`, state "00", which does not exist) and
    // buy a wasted probe on every run.
    let min_width = width.saturating_sub(if matches!(layer, Layer::Zcta) { 2 } else { 1 });
    let mut out: Vec<String> = codes
        .iter()
        .filter_map(|raw| {
            let raw = raw.trim();
            if raw.is_empty()
                || !raw.bytes().all(|b| b.is_ascii_digit())
                || raw.len() > width
                || raw.len() < min_width
            {
                return None;
            }
            Some(format!("{raw:0>width$}"))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Build the `<id_field> IN ('a','b',...)` clauses for a code set, chunked so no single request
/// URL grows unbounded.
///
/// Codes are single-quoted for a SQL-ish `IN` list, so any code containing a quote is dropped
/// rather than escaped — a region code never legitimately contains one, and dropping it keeps the
/// clause un-injectable.
fn in_clauses(codes: &[String], layer: Layer) -> Vec<String> {
    let field = layer.id_field();
    codes
        .chunks(CODES_PER_REQUEST)
        .filter_map(|chunk| {
            let quoted: Vec<String> = chunk
                .iter()
                .filter(|c| !c.contains('\''))
                .map(|c| format!("'{c}'"))
                .collect();
            (!quoted.is_empty()).then(|| format!("{field} IN ({})", quoted.join(",")))
        })
        .collect()
}

/// Count how many of `codes` exist in `layer`, without downloading any geometry.
///
/// This is what makes `auto` able to CHOOSE a geography: county FIPS codes and ZIP codes are both
/// 5-digit numerics, so the codes' shape cannot tell them apart, but their resolution rate against
/// each layer can. `returnGeometry=false` keeps each probe to a few KB.
fn probe_layer(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layer: Layer,
    codes: &[String],
) -> CliResult<usize> {
    let (layer_id, _) = resolve_layer_id(client, vintage, layer)?;
    let url = format!(
        "{}/TIGERweb/tigerWMS_ACS{vintage}/MapServer/{layer_id}/query",
        tigerweb_root()
    );
    let mut found = 0usize;
    for clause in in_clauses(codes, layer) {
        let page = get_json(
            client,
            &url,
            &[
                ("where", clause.as_str()),
                ("outFields", layer.id_field()),
                ("returnGeometry", "false"),
                ("f", "geojson"),
            ],
        )?;
        found += page
            .get("features")
            .and_then(serde_json::Value::as_array)
            .map_or(0, Vec::len);
    }
    Ok(found)
}

/// Query one layer, following pagination, and return the merged feature array.
///
/// Paging is explicit (`orderByFields` + `resultOffset`) rather than trusting a single unbounded
/// request: without `orderByFields` `ArcGIS` does not guarantee a stable order across pages, which
/// would duplicate and drop features.
fn query_layer_geojson(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layer_id: u64,
    where_clause: &str,
    out_fields: &str,
    order_by: &str,
) -> CliResult<Vec<serde_json::Value>> {
    let url = format!(
        "{}/TIGERweb/tigerWMS_ACS{vintage}/MapServer/{layer_id}/query",
        tigerweb_root()
    );
    let mut features: Vec<serde_json::Value> = Vec::new();
    let mut offset = 0usize;
    loop {
        let offset_str = offset.to_string();
        let page_size_str = PAGE_SIZE.to_string();
        let page = get_json(
            client,
            &url,
            &[
                ("where", where_clause),
                ("outFields", out_fields),
                ("returnGeometry", "true"),
                ("outSR", "4326"),
                ("orderByFields", order_by),
                ("resultOffset", &offset_str),
                ("resultRecordCount", &page_size_str),
                ("f", "geojson"),
            ],
        )?;
        // ArcGIS reports a query-level failure as a 200 with an `error` object, so a body that
        // isn't a FeatureCollection must be surfaced rather than silently read as zero features.
        if let Some(err) = page.get("error") {
            let msg = err
                .get("message")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown error");
            return Err(crate::CliError::Other(format!(
                "--geojson auto: Census rejected the boundary query: {msg}"
            )));
        }
        let page_features = page
            .get("features")
            .and_then(serde_json::Value::as_array)
            .cloned()
            .unwrap_or_default();
        let got = page_features.len();
        features.extend(page_features);
        // stop on either signal: the envelope's own flag, or a short/empty page
        let more = page
            .get("exceededTransferLimit")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
        if !more || got == 0 {
            break;
        }
        offset += got;
    }
    Ok(features)
}

/// Derive the distinct 2-digit state FIPS set from a set of county/tract GEOIDs.
///
/// A numeric CSV column routinely drops the leading zero from a FIPS code (Alabama's `01001`
/// arrives as `1001`), so codes are zero-padded to the layer's width before the state prefix is
/// taken — the same normalization `viz::match_region_code` applies when matching cells to
/// feature ids. Codes that cannot be a GEOID at all are skipped here and surface later as
/// unmatched rows in the overlap check, rather than being silently corrected.
fn state_fips_from_geoids(codes: &[String], layer: Layer) -> Vec<String> {
    // Only GEOIDs that actually EMBED a state prefix may go through here. A ZCTA is also a
    // 5-digit numeric, so nothing in the shape of the code would stop this from "deriving"
    // state 15 from ZCTA 15213 — which is not a state at all.
    debug_assert!(
        !matches!(layer, Layer::Zcta),
        "state FIPS can only be derived from a GEOID that embeds one; a ZCTA does not"
    );
    let geoid_width = layer.code_width();
    let mut states: Vec<String> = codes
        .iter()
        .filter_map(|raw| {
            let raw = raw.trim();
            if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            // pad only when the code is short enough to be a leading-zero-stripped GEOID
            let padded = if raw.len() < geoid_width {
                format!("{raw:0>geoid_width$}")
            } else {
                raw.to_string()
            };
            (padded.len() == geoid_width).then(|| padded[..2].to_string())
        })
        .collect();
    states.sort_unstable();
    states.dedup();
    states
}

/// A parsed `--geojson auto|census[:layer][@vintage]` request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AutoSpec {
    /// `None` means "probe every layer and choose".
    pub layer:   Option<Layer>,
    /// `None` means "use the newest vintage the service publishes".
    pub vintage: Option<u16>,
}

/// Parse a `--geojson` value as an automatic-resolution request, or `None` if it names a source.
///
/// Grammar: `auto` | `census` | `census:<layer>`, each optionally suffixed `@<vintage>`.
/// The vintage suffix exists because boundaries are re-drawn: Connecticut replaced its 8 counties
/// with 9 planning regions in the 2022 vintages, with NO GEOID in common, so a pre-2022 CT dataset
/// cannot be mapped against the newest boundaries at all.
pub fn parse_auto_spec(spec: &str) -> Option<AutoSpec> {
    let (base, vintage) = match spec.split_once('@') {
        Some((base, year)) => (base, Some(year.parse::<u16>().ok()?)),
        None => (spec, None),
    };
    let layer = match base {
        "auto" | "census" => None,
        other => Some(Layer::from_selector(other)?),
    };
    Some(AutoSpec { layer, vintage })
}

/// How many older vintages to probe when nothing matches, before giving up on diagnosing why.
///
/// Only ever runs on the failure path, and each probe is geometry-free. Four is enough to reach
/// across a decennial boundary change from the newest vintage (Connecticut's county-to-planning-
/// region switch is 4 vintages back from 2025).
const VINTAGE_FALLBACK_PROBES: usize = 4;

/// Look for an older vintage whose boundaries the codes DO match.
///
/// This turns a dead end into an instruction. Without it, a 2015 Connecticut county dataset
/// reports "matches no Census geography" — true, but misleading, because the codes are perfectly
/// valid county FIPS that simply predate the planning regions.
fn suggest_alternate_vintage(
    client: &reqwest::blocking::Client,
    vintages: &[u16],
    current: u16,
    codes: &[String],
) -> Option<(u16, Layer, usize, usize)> {
    let older: Vec<u16> = vintages
        .iter()
        .copied()
        .filter(|v| *v < current)
        .rev()
        .take(VINTAGE_FALLBACK_PROBES)
        .collect();
    for vintage in older {
        for layer in Layer::ALL {
            let normalized = normalize_codes(codes, layer);
            if normalized.is_empty() {
                continue;
            }
            // a probe against a vintage that lacks the layer is a miss, not an error
            let Ok(matched) = probe_layer(client, vintage, layer, &normalized) else {
                continue;
            };
            #[allow(clippy::cast_precision_loss)]
            let ratio = matched as f64 / normalized.len() as f64;
            if ratio >= LAYER_PROBE_MIN_RATIO {
                return Some((vintage, layer, matched, normalized.len()));
            }
        }
    }
    None
}

/// Resolve the boundary cache directory, creating it if absent.
fn cache_dir() -> CliResult<std::path::PathBuf> {
    Ok(std::path::PathBuf::from(
        crate::diskcache::set_qsv_cache_dir(BOUNDARY_CACHE_SUBDIR)?,
    ))
}

/// Cache key for a request, derived only from what the USER supplied.
///
/// Built from the requested layer (or `auto`) plus the sorted distinct codes, so an identical
/// command hits without contacting the service at all. Sorting means column order and row order do
/// not perturb the key; deduping means row count does not either.
fn cache_key(spec: AutoSpec, codes: &[String]) -> String {
    let mut sorted: Vec<&str> = codes
        .iter()
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect();
    sorted.sort_unstable();
    sorted.dedup();
    let mut hasher = blake3::Hasher::new();
    // The version retires entries whose STAMPED CONTENT this build no longer matches — reusing one
    // makes the same command behave differently depending on when the cache was warmed, which is
    // invisible and lasts until the 30-day TTL expires.
    //   v2 (#4414): added `x-qsv.property_units`, without which an area denominator silently
    //               labelled itself "per 100,000 AREALAND".
    //   v3 (#4395): added `x-qsv.layer`, without which a ZCTA set is indistinguishable from a
    //               county set and a Census denominator fetches the wrong geography.
    hasher.update(b"census/v3/");
    // the service root is part of the entry's IDENTITY: pointing QSV_CENSUS_TIGERWEB_URL at a
    // mirror (or a mock) must not be served boundaries fetched from a different source
    hasher.update(tigerweb_root().as_bytes());
    hasher.update(b"/");
    hasher.update(spec.layer.map_or("auto", Layer::selector).as_bytes());
    hasher.update(b"@");
    hasher.update(
        spec.vintage
            .map_or_else(|| "latest".to_string(), |v| v.to_string())
            .as_bytes(),
    );
    for code in sorted {
        hasher.update(b"\0");
        hasher.update(code.as_bytes());
    }
    hasher.finalize().to_hex()[..32].to_string()
}

/// Seconds since the Unix epoch, or 0 if the clock is before it.
fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Configured cache TTL in seconds.
fn cache_ttl_secs() -> u64 {
    std::env::var("QSV_VIZ_BOUNDARY_CACHE_TTL_DAYS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(BOUNDARY_CACHE_TTL_DAYS)
        * 24
        * 60
        * 60
}

/// Paths of a cache entry: the boundary document and its sidecar.
fn cache_paths(key: &str) -> CliResult<(std::path::PathBuf, std::path::PathBuf)> {
    let dir = cache_dir()?;
    Ok((
        dir.join(format!("{key}.geojson")),
        dir.join(format!("{key}.json")),
    ))
}

/// Read a cache entry if present, reporting whether it is still within the TTL.
///
/// Returns `(BoundarySet, is_fresh)`. A STALE entry is still returned, because it is the right
/// answer when the service is unreachable — better a boundary set from last month than no map at
/// all, as long as the staleness is reported.
fn cache_get(key: &str) -> Option<(BoundarySet, bool)> {
    let (geojson_path, meta_path) = cache_paths(key).ok()?;
    let meta: CachedMeta = serde_json::from_slice(&std::fs::read(&meta_path).ok()?).ok()?;
    let geojson: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&geojson_path).ok()?).ok()?;
    let fresh = now_secs().saturating_sub(meta.fetched_at) <= cache_ttl_secs();
    Some((
        BoundarySet {
            geojson,
            feature_id_key: meta.feature_id_key,
            provenance: meta.provenance,
            scope_key: meta.scope_key,
            path: geojson_path.to_string_lossy().into_owned(),
        },
        fresh,
    ))
}

/// Write a boundary set to the cache and return its on-disk path.
///
/// Materializing is not optional even without caching: several `--geojson` consumers re-read the
/// flag as a path, so an in-memory-only boundary set would re-fetch or panic. It also satisfies
/// the "exportable" requirement — the path can be passed back as an explicit `--geojson` for an
/// archival, network-free run.
fn cache_put(key: &str, boundaries: &BoundarySet) -> CliResult<String> {
    let (geojson_path, meta_path) = cache_paths(key)?;
    let write_err = |path: &std::path::Path, e: &dyn std::fmt::Display| {
        crate::CliError::Other(format!(
            "--geojson auto: could not write boundary cache '{}': {e}",
            path.display()
        ))
    };
    let file = std::fs::File::create(&geojson_path).map_err(|e| write_err(&geojson_path, &e))?;
    serde_json::to_writer(std::io::BufWriter::new(file), &boundaries.geojson)
        .map_err(|e| write_err(&geojson_path, &e))?;
    let meta = CachedMeta {
        feature_id_key: boundaries.feature_id_key.clone(),
        provenance:     boundaries.provenance.clone(),
        scope_key:      boundaries.scope_key.clone(),
        fetched_at:     now_secs(),
    };
    std::fs::write(
        &meta_path,
        serde_json::to_vec(&meta).map_err(|e| write_err(&meta_path, &e))?,
    )
    .map_err(|e| write_err(&meta_path, &e))?;
    Ok(geojson_path.to_string_lossy().into_owned())
}

/// Resolve which ACS vintage to work against, returning the service's full list beside it.
///
/// Shared by the fetch path and the candidate scorer so a probe can never be issued against a
/// different vintage than the fetch it is meant to predict.
fn resolve_vintage(
    client: &reqwest::blocking::Client,
    spec: AutoSpec,
) -> CliResult<(Vec<u16>, u16)> {
    let vintages = available_acs_vintages(client)?;
    let vintage = match spec.vintage {
        Some(v) if vintages.contains(&v) => v,
        Some(v) => {
            return Err(crate::CliError::Other(format!(
                "--geojson: the Census service has no {v} ACS vintage. Available: {}.",
                vintages
                    .iter()
                    .map(u16::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )));
        },
        // safe: available_acs_vintages errors rather than returning an empty list
        None => *vintages.last().unwrap(),
    };
    Ok((vintages, vintage))
}

/// How well one candidate column's codes resolve against a Census geography.
#[derive(Clone, Copy, Debug)]
pub struct CandidateScore {
    /// The layer the codes resolved against best.
    pub layer:   Layer,
    /// How many of `total` codes exist in that layer.
    pub matched: usize,
    /// Codes considered, AFTER that layer's normalization — not the raw distinct count.
    pub total:   usize,
}

/// Score several candidate code sets against the Census geographies, downloading no geometry.
///
/// `viz choropleth` is told which column holds region codes (`--locations`), but `viz smart`
/// chooses that column itself, so it must compare columns BEFORE committing to a fetch. Scoring
/// by fetching would be circular for a code-set-scoped layer: the boundary set would be derived
/// from the very column being scored, and every column would then match perfectly.
///
/// A candidate that resolves against nothing scores `None` rather than failing the run — on the
/// smart path the column choice is qsv's, so a column that turns out not to hold region codes must
/// lose to a better one, not abort. (`choose_layer` keeps the erroring behavior for the explicit
/// `--locations` path, where the column IS the user's stated intent.)
pub fn score_candidates(
    candidates: &[Vec<String>],
    spec: AutoSpec,
) -> CliResult<Vec<Option<CandidateScore>>> {
    let client = census_client()?;
    let (_, vintage) = resolve_vintage(&client, spec)?;
    // an explicit `census:<layer>` pins the geography: probing the others would rank a column on a
    // layer this run is never going to fetch
    let layers: Vec<Layer> = spec.layer.map_or_else(|| Layer::ALL.to_vec(), |l| vec![l]);

    let mut out: Vec<Option<CandidateScore>> = Vec::with_capacity(candidates.len());
    for codes in candidates {
        let scored = probe_scores(&client, vintage, &layers, codes, true)?;
        // best by match ratio, compared exactly (matched_a/total_a vs matched_b/total_b as
        // integers), mirroring `choose_layer`'s ordering
        let best = scored
            .into_iter()
            .filter(|(_, m, t)| *t > 0 && ratio_at_least(*m, *t, LAYER_PROBE_MIN_RATIO))
            .max_by(|a, b| (a.1 * b.2).cmp(&(b.1 * a.2)))
            .map(|(layer, matched, total)| CandidateScore {
                layer,
                matched,
                total,
            });
        out.push(best);
    }
    Ok(out)
}

/// Is `matched/total` at least `min`?
#[allow(clippy::cast_precision_loss)]
fn ratio_at_least(matched: usize, total: usize, min: f64) -> bool {
    total > 0 && (matched as f64 / total as f64) >= min
}

/// Resolve boundaries for `codes` according to `spec`.
///
/// County FIPS codes and ZIP codes are both 5-digit numerics, so when `spec.layer` is `None` the
/// layer cannot be inferred from the codes' shape — `auto` probes each candidate layer
/// (geometry-free, a few KB) and takes the one the codes actually resolve against. A genuine tie
/// is reported rather than guessed: silently picking between two plausible geographies is
/// precisely the silent-wrong-map outcome `auto` exists to avoid.
///
/// A fresh cache entry short-circuits everything, so a repeated command makes NO network request —
/// not even the catalog lookup. If the service is unreachable and only a stale entry exists, the
/// stale boundaries are used and reported, since a month-old boundary set beats no map at all.
pub fn resolve(codes: &[String], spec: AutoSpec) -> CliResult<BoundarySet> {
    let key = cache_key(spec, codes);
    let cached = cache_get(&key);
    if let Some((boundaries, true)) = &cached {
        log::info!(
            "--geojson auto: cache hit ({}), no network",
            boundaries.provenance
        );
        return Ok(cached.map(|(b, _)| b).unwrap());
    }

    let fetched = (|| -> CliResult<BoundarySet> {
        let client = census_client()?;
        let (vintages, vintage) = resolve_vintage(&client, spec)?;
        let layer = match spec.layer {
            Some(l) => l,
            None => choose_layer(&client, &vintages, vintage, codes)?,
        };
        fetch_layer(&client, vintage, layer, codes)
    })();

    match fetched {
        Ok(mut boundaries) => {
            boundaries.path = cache_put(&key, &boundaries)?;
            Ok(boundaries)
        },
        // Only a TRANSIENT failure may be answered from a stale entry. Catching every error here
        // reported semantic failures — a vintage the service does not publish, a layer that no
        // longer exists, a rejected query — as "could not reach the Census service", and answered
        // them with a previous run's boundaries. Those must surface as themselves.
        Err(e) if matches!(e, crate::CliError::Network(_)) => {
            if let Some((boundaries, _)) = cached {
                log::warn!("Census fetch failed, falling back to cache: {e}");
                wwarn!(
                    "--geojson auto: could not reach the Census service; using cached boundaries \
                     from a previous run ({}).",
                    boundaries.provenance
                );
                return Ok(boundaries);
            }
            Err(e)
        },
        // a semantic failure propagates as itself, cache or no cache
        Err(e) => Err(e),
    }
}

/// Are two `(matched, total)` scores exactly the same fraction?
///
/// Cross-multiplied so the comparison is exact integer arithmetic. "Equally good" is a semantic
/// condition that decides whether `auto` picks a geography or refuses to; deciding it by float
/// equality would make it depend on rounding rather than on the counts.
const fn ratios_equal(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 * b.1 == b.0 * a.1
}

/// Probe each of `layers` and report how many of `codes` resolve against it.
///
/// The counts are reported against the layer's OWN normalization (`normalize_codes` re-pads to
/// that layer's code width), so `total` is the per-layer denominator rather than the raw code
/// count — comparing a layer's matches against a denominator it was not scored on would rank
/// layers on different populations.
fn probe_scores(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layers: &[Layer],
    codes: &[String],
    skip_unavailable: bool,
) -> CliResult<Vec<(Layer, usize, usize)>> {
    let mut scored: Vec<(Layer, usize, usize)> = Vec::new();
    for &layer in layers {
        let normalized = normalize_codes(codes, layer);
        if normalized.is_empty() {
            continue;
        }
        let matched = match probe_layer(client, vintage, layer, &normalized) {
            Ok(m) => m,
            // A vintage need not publish every layer, and which ones it does is not knowable
            // before asking. When the caller is RANKING (`score_candidates`), a layer the service
            // does not carry is a layer these codes cannot resolve against — a miss, exactly as
            // `suggest_alternate_vintage` already treats it — not a reason to abandon a run whose
            // other candidates resolve fine. A NETWORK failure still propagates: "no layer
            // matched" must never be how an unreachable service is reported.
            Err(e) if skip_unavailable && !matches!(e, crate::CliError::Network(_)) => {
                log::info!("--geojson auto: {} layer unavailable ({e})", layer.label());
                continue;
            },
            Err(e) => return Err(e),
        };
        scored.push((layer, matched, normalized.len()));
    }
    Ok(scored)
}

/// Probe every candidate layer and return the one the codes resolve against best.
fn choose_layer(
    client: &reqwest::blocking::Client,
    vintages: &[u16],
    vintage: u16,
    codes: &[String],
) -> CliResult<Layer> {
    // the explicit `--locations` path: the user named this column, so a layer the service
    // cannot serve is reported rather than quietly scored as a miss
    let scored = probe_scores(client, vintage, &Layer::ALL, codes, false)?;

    let mut viable: Vec<&(Layer, usize, usize)> = scored
        .iter()
        .filter(|(_, m, t)| ratio_at_least(*m, *t, LAYER_PROBE_MIN_RATIO))
        .collect();
    // NB: ordering and tie detection below compare the ratios as INTEGERS
    // (a.matched * b.total vs b.matched * a.total). The two layers normally share a denominator,
    // so a float compare would in fact work — but "equally good" is an exact semantic condition,
    // and cross-multiplication decides it without depending on that coincidence or on rounding.
    if viable.is_empty() {
        let detail = scored
            .iter()
            .map(|(l, m, t)| format!("{} {m}/{t}", l.label()))
            .collect::<Vec<_>>()
            .join(", ");
        // before blaming the codes, check whether they are simply from an older vintage —
        // boundaries get re-drawn, and valid codes for one vintage can be absent from the next
        if let Some((alt_vintage, alt_layer, matched, total)) =
            suggest_alternate_vintage(client, vintages, vintage, codes)
        {
            return Err(crate::CliError::Other(format!(
                "--geojson auto: the --locations values match no Census geography in the \
                 {vintage} vintage ({detail}), but {matched} of {total} match the {alt_vintage} \
                 {} layer. Boundaries are re-drawn between vintages — pass `--geojson \
                 {}@{alt_vintage}`.",
                alt_layer.label(),
                alt_layer.selector()
            )));
        }
        return Err(crate::CliError::Other(format!(
            "--geojson auto: the --locations values match no Census geography in the {vintage} \
             vintage ({detail}), nor in the vintages before it. They may not be US county FIPS or \
             ZIP codes — supply an explicit --geojson file."
        )));
    }
    // descending by match ratio, compared exactly
    viable.sort_by(|a, b| (b.1 * a.2).cmp(&(a.1 * b.2)));
    // an exact tie is genuinely ambiguous — the same codes are equally valid as either geography
    if viable.len() > 1 && ratios_equal((viable[0].1, viable[0].2), (viable[1].1, viable[1].2)) {
        {
            let options = viable
                .iter()
                .map(|(l, m, t)| {
                    format!("{} ({m}/{t}, use `--geojson {}`)", l.label(), l.selector())
                })
                .collect::<Vec<_>>()
                .join(" or ");
            return Err(crate::CliError::Other(format!(
                "--geojson auto: the --locations values match more than one Census geography \
                 equally well — {options}. Name the one you mean."
            )));
        }
    }
    Ok(viable[0].0)
}

/// Fetch a layer's geometry for `codes`, scoped the only way that layer allows.
///
/// Counties are scoped by STATE so the boundary set is not derived from the candidate column
/// (scoring a column against a feature set built from that same column is circular). ZCTAs have no
/// STATE field at all (module fact 3), so they are scoped by exact code set — there the resolution
/// rate against the layer IS the honesty check, which the probe already established.
fn fetch_layer(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layer: Layer,
    codes: &[String],
) -> CliResult<BoundarySet> {
    let normalized = normalize_codes(codes, layer);
    let (layer_id, catalog_name) = resolve_layer_id(client, vintage, layer)?;

    let (where_clauses, scope_desc, scope_key_part) = match layer {
        // every layer whose GEOID embeds a state prefix is scoped by STATE, which keeps the
        // boundary set independent of the column being scored
        Layer::County | Layer::Tract | Layer::Place => {
            let states = state_fips_from_geoids(&normalized, layer);
            if states.is_empty() {
                return Err(crate::CliError::Other(
                    "--geojson auto: no US state could be derived from the region-code column, so \
                     there is no boundary set to fetch. Check that the column holds 5-digit \
                     county FIPS codes, or supply an explicit --geojson file."
                        .to_string(),
                ));
            }
            let quoted: Vec<String> = states.iter().map(|s| format!("'{s}'")).collect();
            (
                vec![format!("STATE IN ({})", quoted.join(","))],
                format!(
                    "{} state{}",
                    states.len(),
                    if states.len() == 1 { "" } else { "s" }
                ),
                states.join(","),
            )
        },
        Layer::Zcta => (
            in_clauses(&normalized, layer),
            format!("{} ZCTAs", normalized.len()),
            normalized.join(","),
        ),
    };

    let mut features: Vec<serde_json::Value> = Vec::new();
    for clause in where_clauses {
        features.extend(query_layer_geojson(
            client,
            vintage,
            layer_id,
            &clause,
            &format!("{},NAME,AREALAND,AREAWATER", layer.id_field()),
            layer.id_field(),
        )?);
    }
    if features.is_empty() {
        return Err(crate::CliError::Other(format!(
            "--geojson auto: the Census {vintage} {} layer returned no boundaries for \
             {scope_desc}. Supply an explicit --geojson file.",
            layer.label()
        )));
    }

    // Give every feature a top-level `id` mirroring its GEOID. The cached file is advertised as
    // reusable — pass the printed path back as an explicit `--geojson` for an archival, offline
    // run — but `--feature-id-key` defaults to `id`, and TIGERweb puts the GEOID only under
    // `properties`. Without this, reusing the path exactly as printed failed validation with
    // "--feature-id-key 'id' resolves on no feature". Setting both makes the artifact portable
    // with the default key AND with `properties.GEOID`.
    let id_field = layer.id_field();
    for feature in &mut features {
        let Some(geoid) = feature
            .get("properties")
            .and_then(|p| p.get(id_field))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
        else {
            continue;
        };
        if let Some(obj) = feature.as_object_mut() {
            obj.insert("id".to_string(), serde_json::Value::String(geoid));
        }
    }

    // DECLARE the units of the numeric properties this fetch asked for (issue #4414). TIGERweb's
    // AREALAND/AREAWATER are square METRES, and this is the one place that knows it asked for
    // them — the same way it knows the feature-id key is `properties.GEOID`. Without a
    // declaration, `--denominator-key properties.AREALAND` can only take the raw field NAME as its
    // label and divide by metres, so the map reads "per 100,000 AREALAND" at ~0 everywhere.
    //
    // Carried as a GeoJSON foreign member (RFC 7946 §6.1) rather than on `BoundarySet` because
    // the document IS the cache artifact: a warm run reads the declaration back with the
    // geometry, so cold and warm runs cannot label the same map differently. It also survives the
    // advertised archival flow — passing the cached path back as an explicit `--geojson` keeps
    // the units. Consumers that do not know the member ignore it.
    let geojson = serde_json::json!({
        "type": "FeatureCollection",
        "x-qsv": {
            "property_units": {
                "properties.AREALAND": "m2",
                "properties.AREAWATER": "m2",
            },
            // WHICH geography these regions are. A 5-digit ZCTA and a 5-digit county FIPS are
            // indistinguishable by shape — that ambiguity is why `auto` probes at all — so a
            // consumer that needs to know (a Census denominator covers counties and states, not
            // ZCTAs) would otherwise have to guess from the code width and be wrong.
            "layer": layer.selector(),
        },
        "features": features,
    });
    Ok(BoundarySet {
        geojson,
        // filled in by the caller once it is written to the cache
        path: String::new(),
        feature_id_key: format!("properties.{}", layer.id_field()),
        // names the scope exactly — see the field docs for the collision this avoids
        scope_key: format!("census/{}/acs{vintage}/{scope_key_part}", layer.label()),
        // quotes the catalog's own layer name, so a ZCTA set reports its delineation year rather
        // than one inferred from the vintage
        provenance: format!("Census TIGERweb {vintage}, {catalog_name} ({scope_desc})"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_is_redacted_from_anything_quoting_a_url() {
        // a credential must not survive into stderr, a log, or a pasted bug report. reqwest's own
        // error Display embeds the whole query string, so this is the last line of defence.
        //
        // The fixture is SYNTHETIC on purpose: never paste a captured secret in here. The natural
        // way to write this test is to copy the error you just reproduced, which is how a real key
        // once ended up committed inside the very fix for credential leaking.
        let leaked = concat!(
            "error sending request for url (https://api.census.gov/data/2024/acs/acs5",
            "?get=B01003_001E&for=state%3A01&key=EXAMPLE0000NOTAREALKEY0000EXAMPLE)"
        );
        let safe = redact_api_key(leaked);
        assert!(!safe.contains("EXAMPLE0000"), "{safe}");
        assert!(safe.contains("key=REDACTED"), "{safe}");
        // the rest of the URL survives, because it is what makes the error diagnosable
        assert!(safe.contains("for=state%3A01"), "{safe}");
        // a trailing paren ends the value rather than being swallowed
        assert!(safe.ends_with(')'), "{safe}");
        // multiple occurrences, and a key= that is not a query parameter
        assert_eq!(
            redact_api_key("?key=aaa&x=1 and ?key=bbb"),
            "?key=REDACTED&x=1 and ?key=REDACTED"
        );
        assert_eq!(redact_api_key("the monkey=banana"), "the monkey=banana");
        // a URL that a log wrapped: the key is no less secret for having a space before it
        assert_eq!(redact_api_key("...& key=abc123"), "...& key=REDACTED");
        assert_eq!(redact_api_key("no key here"), "no key here");
    }

    #[test]
    fn state_fips_pads_leading_zero_stripped_codes() {
        // Alabama's 01001 arrives from a numeric column as 1001
        let codes = vec!["1001".to_string(), "42003".to_string(), "42007".to_string()];
        assert_eq!(
            state_fips_from_geoids(&codes, Layer::County),
            vec!["01", "42"]
        );
    }

    #[test]
    fn state_fips_skips_non_geoid_values() {
        let codes = vec![
            "  42003  ".to_string(),
            String::new(),
            "not-a-code".to_string(),
            "123456789".to_string(),
        ];
        assert_eq!(state_fips_from_geoids(&codes, Layer::County), vec!["42"]);
    }

    #[test]
    fn zcta_layer_matches_either_delineation() {
        // the ZCTA layer's catalog name carries its decennial delineation, so it cannot be
        // matched exactly — ACS2019 says 2010, ACS2021+ says 2020
        assert!(Layer::Zcta.matches_catalog_name("2020 Census ZIP Code Tabulation Areas"));
        assert!(Layer::Zcta.matches_catalog_name("2010 Census ZIP Code Tabulation Areas"));
        assert!(!Layer::Zcta.matches_catalog_name("Counties"));
        assert!(!Layer::County.matches_catalog_name("2020 Census ZIP Code Tabulation Areas"));
    }

    #[test]
    fn normalize_filters_to_a_plausible_width_band() {
        // 5-digit county codes cannot be tract (11) or place (7) ids no matter how padded, so
        // those layers are never probed for them
        let county_codes = vec!["42003".to_string(), "1001".to_string()];
        assert_eq!(
            normalize_codes(&county_codes, Layer::County),
            vec!["01001", "42003"]
        );
        assert!(normalize_codes(&county_codes, Layer::Tract).is_empty());
        assert!(normalize_codes(&county_codes, Layer::Place).is_empty());

        // an 11-digit tract id is likewise not a county
        let tract_codes = vec!["09160310200".to_string()];
        assert_eq!(
            normalize_codes(&tract_codes, Layer::Tract),
            vec!["09160310200"]
        );
        assert!(normalize_codes(&tract_codes, Layer::County).is_empty());

        // non-numeric can never be a Census GEOID
        assert!(normalize_codes(&["Allegheny".to_string()], Layer::County).is_empty());

        // a Puerto Rico ZCTA loses TWO leading zeros in a numeric column (00601 -> 601), which is
        // why ZCTAs get a wider band than the state-prefixed layers
        assert_eq!(
            normalize_codes(&["601".to_string()], Layer::Zcta),
            vec!["00601"]
        );
    }

    #[test]
    fn parse_auto_spec_grammar() {
        assert_eq!(parse_auto_spec("auto"), Some(AutoSpec::default()));
        assert_eq!(parse_auto_spec("census"), Some(AutoSpec::default()));
        assert_eq!(
            parse_auto_spec("census:county"),
            Some(AutoSpec {
                layer:   Some(Layer::County),
                vintage: None,
            })
        );
        // the vintage suffix on every form — `is_geojson_auto_spec` once accepted a narrower
        // grammar than `resolve` understood, and `census:county@2021` was reported as a missing
        // file. One parser now defines both.
        assert_eq!(
            parse_auto_spec("auto@2019"),
            Some(AutoSpec {
                layer:   None,
                vintage: Some(2019),
            })
        );
        assert_eq!(
            parse_auto_spec("census:zcta@2021"),
            Some(AutoSpec {
                layer:   Some(Layer::Zcta),
                vintage: Some(2021),
            })
        );
        // things that name a source, not a request
        assert_eq!(parse_auto_spec("regions.geojson"), None);
        assert_eq!(parse_auto_spec("https://example.com/a.json"), None);
        assert_eq!(parse_auto_spec("census:nope"), None);
        assert_eq!(parse_auto_spec("auto@notayear"), None);
    }

    #[test]
    fn cache_key_ignores_row_and_column_order() {
        // the key is what makes a repeated command zero-network, so it must not be perturbed by
        // how the CSV happened to be ordered, or by duplicate rows
        let a = vec!["42003".to_string(), "42101".to_string()];
        let b = vec![
            "42101".to_string(),
            "42003".to_string(),
            "42003".to_string(),
        ];
        let auto = AutoSpec::default();
        assert_eq!(cache_key(auto, &a), cache_key(auto, &b));
        // ... but a different code set, layer, or vintage is a different entry
        let c = vec!["42003".to_string(), "42102".to_string()];
        let zcta = AutoSpec {
            layer:   Some(Layer::Zcta),
            vintage: None,
        };
        let county = AutoSpec {
            layer:   Some(Layer::County),
            vintage: None,
        };
        let county_2019 = AutoSpec {
            layer:   Some(Layer::County),
            vintage: Some(2019),
        };
        assert_ne!(cache_key(auto, &a), cache_key(auto, &c));
        assert_ne!(cache_key(auto, &a), cache_key(zcta, &a));
        assert_ne!(cache_key(county, &a), cache_key(zcta, &a));
        // a vintage-pinned request must not collide with the unpinned one
        assert_ne!(cache_key(county, &a), cache_key(county_2019, &a));
    }

    #[test]
    fn only_transient_failures_may_be_answered_from_stale_cache() {
        use reqwest::StatusCode;
        // no response at all: connect failure, timeout, undecodable body
        assert!(status_is_transient(None));
        // "ask again later" — 4xx by number, transient in meaning. A chunked ZCTA fetch is a loop
        // of requests, so a rate limit here is realistic rather than theoretical.
        assert!(status_is_transient(Some(StatusCode::REQUEST_TIMEOUT)));
        assert!(status_is_transient(Some(StatusCode::TOO_MANY_REQUESTS)));
        // the service is unwell
        assert!(status_is_transient(Some(StatusCode::BAD_GATEWAY)));
        assert!(status_is_transient(Some(StatusCode::SERVICE_UNAVAILABLE)));
        // the service understood and refused: must NOT be answered from a stale entry, nor
        // reported to the user as a connectivity problem
        assert!(!status_is_transient(Some(StatusCode::BAD_REQUEST)));
        assert!(!status_is_transient(Some(StatusCode::NOT_FOUND)));
        assert!(!status_is_transient(Some(StatusCode::FORBIDDEN)));
    }

    #[test]
    fn tie_detection_is_exact_across_denominators() {
        // same fraction, different denominators — a float compare happens to work here, but the
        // integer compare does not depend on that
        assert!(ratios_equal((2, 4), (3, 6)));
        assert!(ratios_equal((1, 3), (2, 6)));
        assert!(ratios_equal((6, 6), (5, 5)));
        // the real PA county case: 6/6 counties vs 5/6 ZCTAs is NOT a tie, so auto picks
        // rather than bailing out
        assert!(!ratios_equal((6, 6), (5, 6)));
        assert!(!ratios_equal((0, 6), (6, 6)));
    }

    #[test]
    fn selectors_round_trip() {
        for layer in Layer::ALL {
            assert_eq!(Layer::from_selector(layer.selector()), Some(layer));
        }
        assert_eq!(Layer::from_selector("census:nope"), None);
        assert_eq!(Layer::from_selector("auto"), None);
    }

    #[test]
    fn normalize_pads_and_dedupes() {
        let codes = vec![
            "7936".to_string(),
            " 15213 ".to_string(),
            "15213".to_string(),
            String::new(),
        ];
        // 7936 -> 07936 BEFORE the query: an unpadded code matches no ZCTA at all, so it would
        // otherwise read as a nonexistent region rather than a formatting artifact
        assert_eq!(normalize_codes(&codes, Layer::Zcta), vec!["07936", "15213"]);
    }

    #[test]
    fn in_clauses_chunk_and_reject_quotes() {
        let codes: Vec<String> = (0..450).map(|i| format!("{:05}", i)).collect();
        let clauses = in_clauses(&codes, Layer::Zcta);
        // 450 codes at 200 per request
        assert_eq!(clauses.len(), 3);
        assert!(clauses[0].starts_with("GEOID IN ('00000',"));

        // a quote-bearing code is dropped rather than escaped, keeping the clause un-injectable
        let sneaky = vec!["15213".to_string(), "x') OR 1=1 --".to_string()];
        let clauses = in_clauses(&sneaky, Layer::Zcta);
        assert_eq!(clauses, vec!["GEOID IN ('15213')"]);
    }

    #[test]
    fn county_layer_name_is_the_catalog_spelling() {
        // guards module fact 1: the name is the lookup key, so a typo here is a silent miss
        assert!(Layer::County.matches_catalog_name("Counties"));
        assert!(!Layer::County.matches_catalog_name("County"));
    }
}

// ===========================================================================================
// Census DATA API (issue #4395) — population denominators.
//
// A DIFFERENT service from everything above: TIGERweb serves boundary GEOMETRY from an ArcGIS
// catalog; this serves TABULAR estimates from `api.census.gov`, with its own host, its own
// optional API key, and an array-of-arrays response shape. The two share a Bureau and nothing
// else, so the only reuse is the plumbing this module already settled — the HTTP client, the
// transient-vs-semantic error rule, and the disk cache.
// ===========================================================================================

/// Root of the Census Data API.
const ACS_ROOT: &str = "https://api.census.gov/data";

/// The Data API root to actually use, honoring `QSV_CENSUS_API_URL` — same rationale as
/// [`tigerweb_root`]: a mirror for users behind one, and a mock server for the tests.
fn acs_root() -> String {
    std::env::var("QSV_CENSUS_API_URL").unwrap_or_else(|_| ACS_ROOT.to_string())
}

/// ACS 5-year table for TOTAL POPULATION. `B01003_001E` is the estimate cell.
///
/// 5-year rather than 1-year because it is the only product published for every geography this
/// resolves (the 1-year product omits small counties entirely), and because a rate map compares
/// regions — a denominator that exists for some of them and not others silently reshapes the map.
const ACS_POPULATION_TABLE: &str = "B01003_001E";

/// How many vintages back to probe when finding the newest published one. ACS 5-year releases lag
/// roughly a year, and a probe is a single tiny request, so this only has to cover the gap between
/// "this year" and the newest release.
const ACS_VINTAGE_PROBES: u16 = 4;

/// How long a cached denominator set stays fresh, in days. Overridable with
/// `QSV_VIZ_DENOMINATOR_CACHE_TTL_DAYS`. Long for the same reason boundaries are: an ACS vintage
/// is an annual artifact, so this exists to pick up a new release eventually, not to bound
/// staleness within a session.
const DENOMINATOR_CACHE_TTL_DAYS: u64 = 30;

/// Cache subdirectory under the qsv cache dir.
const DENOMINATOR_CACHE_SUBDIR: &str = "~/.qsv-cache/viz-denominators";

/// The geographies a Census population denominator can be resolved for.
///
/// Deliberately only the two whose ACS coverage is complete and uncaveated. Tracts and ZCTAs are
/// real follow-ups, not oversights: a ZCTA is not a ZIP code (PO-box ZIPs have no ZCTA at all, and
/// the boundaries differ), which has to be stamped into the provenance rather than quietly
/// approximated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenominatorGeography {
    State,
    County,
}

impl DenominatorGeography {
    /// The Data API `for=` clause for this geography.
    const fn for_clause(self) -> &'static str {
        match self {
            Self::State => "state:*",
            Self::County => "county:*",
        }
    }

    /// Human label used in provenance and errors.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::State => "state",
            Self::County => "county",
        }
    }

    /// Which `--geojson auto` boundary layer this geography pairs with, so a caller can derive one
    /// from the other rather than asking the user twice.
    #[must_use]
    pub const fn from_layer(layer: Layer) -> Option<Self> {
        match layer {
            Layer::County => Some(Self::County),
            // a state-level dataset resolves against the county layer's STATE scoping, so there is
            // no separate state layer to map from; state denominators are requested directly
            Layer::Zcta | Layer::Tract | Layer::Place => None,
        }
    }
}

/// A per-region population denominator fetched from the Census Data API.
pub struct PopulationSet {
    /// GEOID -> population estimate. Keyed exactly as the boundary features are, so the caller
    /// joins on the same ids it already canonicalized.
    pub values:     std::collections::HashMap<String, f64>,
    /// Human-readable lineage for the panel subtitle, e.g.
    /// "ACS 2019–2023 5-yr (B01003), fetched 2026-08-16".
    pub provenance: String,
    /// The vintage actually used, so a caller can report it separately from the prose.
    pub vintage:    u16,
}

/// Parse a `--denominator` value as a Census request, returning the pinned vintage if one was
/// given. `None` means the value names a column, not this source.
///
/// Grammar: `census` | `census@<year>`, mirroring `--geojson census:county@2021` so the two flags
/// pin a vintage the same way. `census` is a RESERVED value: a dataset column of that name cannot
/// be used as a denominator, which the flag's help text states.
#[must_use]
pub fn parse_census_denominator(spec: &str) -> Option<Option<u16>> {
    let (base, vintage) = match spec.split_once('@') {
        Some((base, year)) => (base, Some(year.parse::<u16>().ok()?)),
        None => (spec, None),
    };
    (base.eq_ignore_ascii_case("census")).then_some(vintage)
}

/// The Census Data API key to use, or an actionable error.
///
/// The key is REQUIRED, which contradicts what the Bureau's older documentation (and issue #4395)
/// says. Verified against the live service: every unkeyed request — including the documented
/// one-row example — 302s to `data/missing_key.html`, which then answers 200 with HTML. So an
/// unkeyed run fails not with an HTTP error but with "response is not valid JSON", which reads
/// like a qsv bug. Demand the key up front instead, and say where to get one.
///
/// Skipped when `QSV_CENSUS_API_URL` points somewhere else: an override means a mirror or a mock,
/// and neither is obliged to want a Bureau key.
fn census_api_key() -> CliResult<Option<String>> {
    let key = std::env::var("QSV_CENSUS_API_KEY")
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty());
    if key.is_none() && acs_root() == ACS_ROOT {
        return Err(crate::CliError::Other(
            "--denominator census: the Census Data API requires an API key. Request a free one at \
             https://api.census.gov/data/key_signup.html, then set QSV_CENSUS_API_KEY. (An \
             unkeyed request is redirected to a 'Missing Key' page rather than refused, so it \
             cannot be reported as a permission error.)"
                .to_string(),
        ));
    }
    Ok(key)
}

/// Build the Data API query parameters, appending the API key.
///
/// The key is a credential, so every error this module builds from a URL or an underlying error is
/// passed through [`redact_api_key`] — `reqwest`'s error `Display` embeds the full query string,
/// and an earlier version of this code printed the key to stderr on any network failure.
fn acs_params<'a>(
    get: &'a str,
    for_clause: &'a str,
    in_clause: Option<&'a str>,
    key: Option<&'a str>,
) -> Vec<(&'a str, &'a str)> {
    let mut params = vec![("get", get), ("for", for_clause)];
    if let Some(in_clause) = in_clause {
        params.push(("in", in_clause));
    }
    if let Some(key) = key {
        params.push(("key", key));
    }
    params
}

/// `get_json` for the Data API, mapping an unparseable body to the API-key hint.
///
/// The Bureau answers a missing OR INVALID key with an HTML page under HTTP 200, so the raw
/// failure is "response is not valid JSON" — which reads like a qsv bug. The hint used to live
/// only in the vintage probe, so a pinned `census@2023` (or an unpinned run whose vintage was
/// already cached) skipped the probe and surfaced the raw error. Every ACS request goes through
/// here instead, so the diagnosis cannot depend on which path reached the service.
///
/// Classified as `IncorrectUsage` rather than `Other`: it is the user's configuration that is
/// wrong, and the vintage walk-back uses that distinction to stop rather than blame the vintage.
fn acs_get_json(
    client: &reqwest::blocking::Client,
    url: &str,
    params: &[(&str, &str)],
) -> CliResult<serde_json::Value> {
    get_json(client, url, params).map_err(|e| {
        let msg = redact_api_key(&e.to_string());
        if msg.contains("not valid JSON") {
            crate::CliError::IncorrectUsage(format!(
                "--denominator census: the Census Data API did not return JSON, which is what it \
                 does for a missing or invalid key. Check QSV_CENSUS_API_KEY (request one at \
                 https://api.census.gov/data/key_signup.html). Underlying error: {msg}"
            ))
        } else {
            e
        }
    })
}

/// The newest ACS 5-year vintage the service actually publishes.
///
/// Probed rather than computed: the release calendar slips, and a computed guess would produce a
/// 404 that reads like a code bug. Each probe is a single one-row request.
fn newest_acs_vintage(client: &reqwest::blocking::Client, this_year: u16) -> CliResult<u16> {
    let key = census_api_key()?;
    let mut last_err: Option<crate::CliError> = None;
    for vintage in (this_year.saturating_sub(ACS_VINTAGE_PROBES)..=this_year).rev() {
        let url = format!("{}/{vintage}/acs/acs5", acs_root());
        match acs_get_json(
            client,
            &url,
            &acs_params(ACS_POPULATION_TABLE, "state:01", None, key.as_deref()),
        ) {
            Ok(_) => return Ok(vintage),
            // A vintage the service does not publish answers 404 — a SEMANTIC "no", so keep
            // walking back. Two failures are NOT about the vintage and must not be reported as
            // "no vintage exists": a transient failure (the service is unwell) and a key problem
            // (every vintage would fail the same way).
            Err(e @ (crate::CliError::Network(_) | crate::CliError::IncorrectUsage(_))) => {
                return Err(e);
            },
            Err(e) => last_err = Some(e),
        }
    }
    Err(crate::CliError::Other(format!(
        "--denominator census: no published ACS 5-year vintage found in the last \
         {ACS_VINTAGE_PROBES} years. Last response: {}",
        last_err.map_or_else(|| "none".to_string(), |e| e.to_string())
    )))
}

/// Parse a Data API response — a JSON array of arrays whose FIRST row names the columns — into
/// `GEOID -> value`.
///
/// The column order is read from that header row rather than assumed positionally: the API returns
/// the requested variables followed by the geography columns, and the geography ones vary by query
/// (`state` alone, or `state` + `county`). Assuming a position silently mis-keys every region the
/// day the shape changes.
fn parse_acs_rows(
    body: &serde_json::Value,
    geo: DenominatorGeography,
) -> CliResult<std::collections::HashMap<String, f64>> {
    let rows = body.as_array().ok_or_else(|| {
        crate::CliError::Other(
            "--denominator census: the Census Data API returned something that is not a JSON array"
                .to_string(),
        )
    })?;
    let Some((header, data)) = rows.split_first() else {
        return Ok(std::collections::HashMap::new());
    };
    let names: Vec<&str> = header
        .as_array()
        .map(|h| h.iter().filter_map(serde_json::Value::as_str).collect())
        .unwrap_or_default();
    let col = |want: &str| names.iter().position(|n| *n == want);
    let (Some(value_at), Some(state_at)) = (col(ACS_POPULATION_TABLE), col("state")) else {
        return Err(crate::CliError::Other(format!(
            "--denominator census: the Census Data API response is missing the \
             {ACS_POPULATION_TABLE} or state column (got: {})",
            names.join(", ")
        )));
    };
    let county_at = col("county");
    let mut out = std::collections::HashMap::new();
    for row in data {
        let Some(cells) = row.as_array() else {
            continue;
        };
        let cell = |at: usize| cells.get(at).and_then(serde_json::Value::as_str);
        let (Some(state), Some(raw)) = (cell(state_at), cell(value_at)) else {
            continue;
        };
        // the API returns numbers as STRINGS; a negative value is one of the Bureau's annotation
        // sentinels (-666666666 and friends) meaning "not available", never a population
        let Ok(value) = raw.parse::<f64>() else {
            continue;
        };
        if !value.is_finite() || value < 0.0 {
            continue;
        }
        let geoid = match (geo, county_at.and_then(cell)) {
            (DenominatorGeography::County, Some(county)) => format!("{state:0>2}{county:0>3}"),
            (DenominatorGeography::County, None) => continue,
            (DenominatorGeography::State, _) => format!("{state:0>2}"),
        };
        out.insert(geoid, value);
    }
    Ok(out)
}

/// Fetch population for `geo`, scoped to `states` (ignored for a state-level request).
fn fetch_population(
    client: &reqwest::blocking::Client,
    vintage: u16,
    geo: DenominatorGeography,
    states: &[String],
) -> CliResult<std::collections::HashMap<String, f64>> {
    let key = census_api_key()?;
    let url = format!("{}/{vintage}/acs/acs5", acs_root());
    let get = format!("{ACS_POPULATION_TABLE},NAME");
    let mut values = std::collections::HashMap::new();
    match geo {
        // one request covers every state
        DenominatorGeography::State => {
            let body = acs_get_json(
                client,
                &url,
                &acs_params(&get, geo.for_clause(), None, key.as_deref()),
            )?;
            values.extend(parse_acs_rows(&body, geo)?);
        },
        // one request per state, mirroring how the boundary fetch scopes by STATE — a handful of
        // requests for a handful of states, and no dependence on the API's support for a
        // comma-separated `in=` list, which varies by vintage
        DenominatorGeography::County => {
            for state in states {
                let in_clause = format!("state:{state}");
                let body = acs_get_json(
                    client,
                    &url,
                    &acs_params(&get, geo.for_clause(), Some(&in_clause), key.as_deref()),
                )?;
                values.extend(parse_acs_rows(&body, geo)?);
            }
        },
    }
    Ok(values)
}

/// Cache key for a denominator request, derived only from what the request IS.
fn denominator_cache_key(geo: DenominatorGeography, vintage: u16, states: &[String]) -> String {
    let mut sorted: Vec<&str> = states.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    sorted.dedup();
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"acs/v1/");
    // the service root is part of the entry's IDENTITY, exactly as it is for boundaries
    hasher.update(acs_root().as_bytes());
    hasher.update(b"/");
    hasher.update(geo.label().as_bytes());
    hasher.update(b"@");
    hasher.update(vintage.to_string().as_bytes());
    for state in sorted {
        hasher.update(b"\0");
        hasher.update(state.as_bytes());
    }
    hasher.finalize().to_hex()[..32].to_string()
}

/// Resolve population denominators, answering from the disk cache when it can.
///
/// The cache is what keeps the Data Schematic's "deterministic and offline" promise honest: a
/// repeated command makes no network request at all, and the cached JSON can be inspected. Like
/// the boundary cache, a STALE entry is preferred to no map when the service is unreachable, and
/// the staleness is reported by the caller through the provenance it stamps.
pub fn resolve_population(
    geo: DenominatorGeography,
    states: &[String],
    pinned_vintage: Option<u16>,
    this_year: u16,
) -> CliResult<PopulationSet> {
    let client = census_client()?;
    let vintage = match pinned_vintage {
        Some(v) => v,
        None => newest_acs_vintage_cached(&client, this_year)?,
    };
    let cache_key = denominator_cache_key(geo, vintage, states);
    let path = std::path::PathBuf::from(crate::diskcache::set_qsv_cache_dir(
        DENOMINATOR_CACHE_SUBDIR,
    )?)
    .join(format!("{cache_key}.json"));

    let ttl = denominator_cache_ttl_secs();
    let cached: Option<(std::collections::HashMap<String, f64>, bool)> = std::fs::read(&path)
        .ok()
        .and_then(|raw| serde_json::from_slice::<CachedPopulation>(&raw).ok())
        .map(|c| {
            let fresh = now_secs().saturating_sub(c.fetched_at) <= ttl;
            (c.values, fresh)
        });
    if let Some((values, true)) = cached {
        log::info!(
            "--denominator census: cache hit ({vintage} {}), no network",
            geo.label()
        );
        return Ok(PopulationSet {
            values,
            provenance: acs_provenance(vintage),
            vintage,
        });
    }

    match fetch_population(&client, vintage, geo, states) {
        Ok(values) => {
            if values.is_empty() {
                return Err(crate::CliError::Other(format!(
                    "--denominator census: the {vintage} ACS 5-year release returned no \
                     population for any requested {}. Supply the denominator explicitly if this \
                     is intentional.",
                    geo.label()
                )));
            }
            let payload = CachedPopulation {
                values:     values.clone(),
                fetched_at: now_secs(),
            };
            if let Ok(raw) = serde_json::to_vec(&payload) {
                let _ = std::fs::write(&path, raw);
            }
            Ok(PopulationSet {
                values,
                provenance: acs_provenance(vintage),
                vintage,
            })
        },
        // only a TRANSIENT failure may be answered from a stale entry — the same rule the boundary
        // fetch settled: a rejected query or an unpublished vintage is the service answering "no",
        // and must surface as itself rather than as last month's numbers
        Err(e @ crate::CliError::Network(_)) => match cached {
            Some((values, _)) => {
                log::warn!("--denominator census: {e}; using stale cached population");
                Ok(PopulationSet {
                    values,
                    provenance: format!("{} (stale cache)", acs_provenance(vintage)),
                    vintage,
                })
            },
            None => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// The newest published vintage, remembered on disk.
///
/// The probe is a network request, and it runs BEFORE the denominator cache can be consulted
/// (the cache is keyed by vintage, so the vintage has to be known first). Without this, a repeated
/// command still made one request — which is exactly the "no network on a warm run" promise the
/// rest of this module keeps. Cached under the same TTL as the denominators themselves, since it
/// answers the same question: has the Bureau published a new release?
fn newest_acs_vintage_cached(client: &reqwest::blocking::Client, this_year: u16) -> CliResult<u16> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"acs-newest/v1/");
    hasher.update(acs_root().as_bytes());
    let key = hasher.finalize().to_hex()[..16].to_string();
    let path = std::path::PathBuf::from(crate::diskcache::set_qsv_cache_dir(
        DENOMINATOR_CACHE_SUBDIR,
    )?)
    .join(format!("newest-{key}.json"));

    if let Ok(raw) = std::fs::read(&path)
        && let Ok(cached) = serde_json::from_slice::<CachedVintage>(&raw)
        && now_secs().saturating_sub(cached.fetched_at) <= denominator_cache_ttl_secs()
    {
        return Ok(cached.vintage);
    }
    let vintage = newest_acs_vintage(client, this_year)?;
    if let Ok(raw) = serde_json::to_vec(&CachedVintage {
        vintage,
        fetched_at: now_secs(),
    }) {
        let _ = std::fs::write(&path, raw);
    }
    Ok(vintage)
}

/// Configured denominator-cache TTL in seconds.
fn denominator_cache_ttl_secs() -> u64 {
    std::env::var("QSV_VIZ_DENOMINATOR_CACHE_TTL_DAYS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(DENOMINATOR_CACHE_TTL_DAYS)
        * 24
        * 60
        * 60
}

/// On-disk shape of the remembered newest vintage.
#[derive(serde::Serialize, serde::Deserialize)]
struct CachedVintage {
    vintage:    u16,
    fetched_at: u64,
}

/// The provenance line for an ACS 5-year vintage: the window it covers and the table it came from.
fn acs_provenance(vintage: u16) -> String {
    format!("ACS {}–{vintage} 5-yr (B01003)", vintage.saturating_sub(4))
}

/// On-disk shape of a cached denominator set.
#[derive(serde::Serialize, serde::Deserialize)]
struct CachedPopulation {
    values:     std::collections::HashMap<String, f64>,
    fetched_at: u64,
}

/// USPS two-letter code -> state FIPS, for joining a `--location-mode usa-states` column to Census
/// data (which is keyed by FIPS).
///
/// Duplicated rather than borrowed from `geocode`'s table because `viz` must not depend on the
/// `geocode` feature — a Data Schematic builds in a build without it. The data is inert: postal
/// abbreviations and state FIPS have been stable for decades, and a new entry would be a new
/// state.
const USPS_STATE_FIPS: &[(&str, &str)] = &[
    ("AL", "01"),
    ("AK", "02"),
    ("AZ", "04"),
    ("AR", "05"),
    ("CA", "06"),
    ("CO", "08"),
    ("CT", "09"),
    ("DE", "10"),
    ("DC", "11"),
    ("FL", "12"),
    ("GA", "13"),
    ("HI", "15"),
    ("ID", "16"),
    ("IL", "17"),
    ("IN", "18"),
    ("IA", "19"),
    ("KS", "20"),
    ("KY", "21"),
    ("LA", "22"),
    ("ME", "23"),
    ("MD", "24"),
    ("MA", "25"),
    ("MI", "26"),
    ("MN", "27"),
    ("MS", "28"),
    ("MO", "29"),
    ("MT", "30"),
    ("NE", "31"),
    ("NV", "32"),
    ("NH", "33"),
    ("NJ", "34"),
    ("NM", "35"),
    ("NY", "36"),
    ("NC", "37"),
    ("ND", "38"),
    ("OH", "39"),
    ("OK", "40"),
    ("OR", "41"),
    ("PA", "42"),
    ("RI", "44"),
    ("SC", "45"),
    ("SD", "46"),
    ("TN", "47"),
    ("TX", "48"),
    ("UT", "49"),
    ("VT", "50"),
    ("VA", "51"),
    ("WA", "53"),
    ("WV", "54"),
    ("WI", "55"),
    ("WY", "56"),
    // territories the ACS publishes alongside the states
    ("PR", "72"),
];

/// State FIPS for a USPS code, case-insensitively.
#[must_use]
pub fn state_fips_for_usps(code: &str) -> Option<&'static str> {
    let folded = code.trim().to_ascii_uppercase();
    USPS_STATE_FIPS
        .iter()
        .find(|(usps, _)| *usps == folded)
        .map(|(_, fips)| *fips)
}
