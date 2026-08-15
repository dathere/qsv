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
}

impl Layer {
    /// Every layer `auto` will probe, in the order they are reported when the choice is ambiguous.
    pub const ALL: [Self; 2] = [Self::County, Self::Zcta];

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
        }
    }

    /// Human label used in errors and provenance.
    const fn label(self) -> &'static str {
        match self {
            Self::County => "county",
            Self::Zcta => "ZCTA",
        }
    }

    /// The explicit `--geojson census:<name>` selector for this layer.
    const fn selector(self) -> &'static str {
        match self {
            Self::County => "census:county",
            Self::Zcta => "census:zcta",
        }
    }

    /// Parse an explicit `census:<layer>` selector.
    pub fn from_selector(spec: &str) -> Option<Self> {
        match spec {
            "census:county" | "census:counties" => Some(Self::County),
            "census:zcta" | "census:zip" => Some(Self::Zcta),
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
        Some(TIGERWEB_ROOT.to_string()),
    )
}

/// GET `url` with `params` and parse the response as JSON, reading through a bounded `take` so an
/// endless or merely enormous body cannot be buffered unchecked (the same guard
/// `viz::load_geojson` applies).
///
/// Query parameters are passed to reqwest rather than interpolated into the URL: a `where` clause
/// carries quotes, commas and spaces that must be percent-encoded, and hand-rolling that is how
/// injection-shaped bugs get in.
fn get_json(
    client: &reqwest::blocking::Client,
    url: &str,
    params: &[(&str, &str)],
) -> CliResult<serde_json::Value> {
    // `RequestBuilder::query` is behind a reqwest feature qsv does not enable, so the parameters
    // are encoded onto the URL here instead — same percent-encoding, no new build feature.
    let target = reqwest::Url::parse_with_params(url, params.iter().copied())
        .map_err(|e| crate::CliError::Other(format!("could not build Census URL '{url}': {e}")))?;
    let mut resp = client
        .get(target)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| crate::CliError::Other(format!("Census request to '{url}' failed: {e}")))?;
    let mut buf: Vec<u8> = Vec::new();
    // one byte past the cap, so exceeding it is distinguishable from exactly reaching it
    resp.by_ref()
        .take(CENSUS_MAX_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| crate::CliError::Other(format!("reading Census response body: {e}")))?;
    if buf.len() > CENSUS_MAX_BYTES {
        return Err(crate::CliError::Other(format!(
            "Census response from '{url}' exceeds the {} MB limit. Narrow the dataset's \
             geographic extent, or supply an explicit --geojson file.",
            CENSUS_MAX_BYTES / 1_000_000
        )));
    }
    serde_json::from_slice(&buf)
        .map_err(|e| crate::CliError::Other(format!("Census response is not valid JSON: {e}")))
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
        &format!("{TIGERWEB_ROOT}/TIGERweb"),
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
            "no ACS vintages found in the Census TIGERweb catalog at '{TIGERWEB_ROOT}/TIGERweb'. \
             The service may have been reorganized; supply an explicit --geojson file."
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
    let url = format!("{TIGERWEB_ROOT}/TIGERweb/tigerWMS_ACS{vintage}/MapServer");
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

/// Normalize region codes for a layer: trim, drop empties, and re-pad numeric codes whose leading
/// zero a numeric CSV column dropped (`7936` -> `07936`, `1001` -> `01001`).
///
/// Padding must happen BEFORE the query, not just when matching results back: a `where ... IN
/// ('7936')` clause matches no ZCTA at all, so an unpadded code would read as a nonexistent
/// region rather than a formatting artifact.
fn normalize_codes(codes: &[String], layer: Layer) -> Vec<String> {
    let width = layer.code_width();
    let mut out: Vec<String> = codes
        .iter()
        .filter_map(|raw| {
            let raw = raw.trim();
            if raw.is_empty() {
                return None;
            }
            if raw.len() < width && raw.bytes().all(|b| b.is_ascii_digit()) {
                Some(format!("{raw:0>width$}"))
            } else {
                Some(raw.to_string())
            }
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
    let url = format!("{TIGERWEB_ROOT}/TIGERweb/tigerWMS_ACS{vintage}/MapServer/{layer_id}/query");
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
    let url = format!("{TIGERWEB_ROOT}/TIGERweb/tigerWMS_ACS{vintage}/MapServer/{layer_id}/query");
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
        matches!(layer, Layer::County),
        "state FIPS can only be derived from a GEOID that embeds one"
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
    hasher.update(b"census/v1/");
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
        let vintages = available_acs_vintages(&client)?;
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
        Err(e) => {
            // the service is unreachable (or refused); a stale entry is the right answer here
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

/// Probe every candidate layer and return the one the codes resolve against best.
fn choose_layer(
    client: &reqwest::blocking::Client,
    vintages: &[u16],
    vintage: u16,
    codes: &[String],
) -> CliResult<Layer> {
    let mut scored: Vec<(Layer, usize, usize)> = Vec::new();
    for layer in Layer::ALL {
        let normalized = normalize_codes(codes, layer);
        if normalized.is_empty() {
            continue;
        }
        let matched = probe_layer(client, vintage, layer, &normalized)?;
        scored.push((layer, matched, normalized.len()));
    }

    #[allow(clippy::cast_precision_loss)]
    let ratio = |matched: usize, total: usize| -> f64 {
        if total == 0 {
            0.0
        } else {
            matched as f64 / total as f64
        }
    };
    let mut viable: Vec<&(Layer, usize, usize)> = scored
        .iter()
        .filter(|(_, m, t)| ratio(*m, *t) >= LAYER_PROBE_MIN_RATIO)
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
        Layer::County => {
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

    let geojson = serde_json::json!({
        "type": "FeatureCollection",
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
