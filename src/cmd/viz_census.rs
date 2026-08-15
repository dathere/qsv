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
}

/// The US standard jurisdictions `--geojson auto` can resolve.
///
/// Only [`Layer::County`] is wired up so far; the remaining variants land with their own scoping
/// strategies (ZCTAs by code set, see the module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    County,
}

impl Layer {
    /// The layer's name in the `TIGERweb` `MapServer` catalog. Matched exactly by
    /// [`resolve_layer_id`] — see module fact 1 for why the id is never hardcoded.
    const fn service_layer_name(self) -> &'static str {
        match self {
            Self::County => "Counties",
        }
    }

    /// Human label used in errors and provenance.
    const fn label(self) -> &'static str {
        match self {
            Self::County => "county",
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
    let target = reqwest::Url::parse_with_params(url, params.iter().copied()).map_err(|e| {
        crate::CliError::Other(format!(
            "--geojson auto: could not build Census URL '{url}': {e}"
        ))
    })?;
    let mut resp = client
        .get(target)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| {
            crate::CliError::Other(format!(
                "--geojson auto: Census request to '{url}' failed: {e}"
            ))
        })?;
    let mut buf: Vec<u8> = Vec::new();
    // one byte past the cap, so exceeding it is distinguishable from exactly reaching it
    resp.by_ref()
        .take(CENSUS_MAX_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| {
            crate::CliError::Other(format!("--geojson auto: reading Census response body: {e}"))
        })?;
    if buf.len() > CENSUS_MAX_BYTES {
        return Err(crate::CliError::Other(format!(
            "--geojson auto: Census response from '{url}' exceeds the {} MB limit. Narrow the \
             dataset's geographic extent, or supply an explicit --geojson file.",
            CENSUS_MAX_BYTES / 1_000_000
        )));
    }
    serde_json::from_slice(&buf).map_err(|e| {
        crate::CliError::Other(format!(
            "--geojson auto: Census response is not valid JSON: {e}"
        ))
    })
}

/// Discover the newest ACS vintage the service actually publishes.
///
/// Read from the catalog rather than computed from the current year: the vintages present are a
/// property of the service (it listed `tigerWMS_ACS2012`..`tigerWMS_ACS2025` when this was
/// written), and any "this year minus one" rule silently drifts every January.
pub fn latest_acs_vintage(client: &reqwest::blocking::Client) -> CliResult<u16> {
    let catalog = get_json(
        client,
        &format!("{TIGERWEB_ROOT}/TIGERweb"),
        &[("f", "json")],
    )?;
    let latest = catalog
        .get("services")
        .and_then(serde_json::Value::as_array)
        .and_then(|services| {
            services
                .iter()
                .filter_map(|s| s.get("name").and_then(serde_json::Value::as_str))
                // names arrive folder-qualified, e.g. "TIGERweb/tigerWMS_ACS2023"
                .filter_map(|name| name.rsplit('/').next())
                .filter_map(|name| name.strip_prefix("tigerWMS_ACS"))
                .filter_map(|year| year.parse::<u16>().ok())
                .max()
        });
    latest.map_or_else(
        || {
            Err(crate::CliError::Other(format!(
                "--geojson auto: no ACS vintages found in the Census TIGERweb catalog at \
                 '{TIGERWEB_ROOT}/TIGERweb'. The service may have been reorganized; supply an \
                 explicit --geojson file."
            )))
        },
        Ok,
    )
}

/// Resolve a layer's numeric id by NAME within a vintage's `MapServer`.
///
/// See module fact 1: ids move between vintages, and a stale id fetches the wrong geography
/// without erroring. Matching on the catalog's own name makes a rename fail loudly instead.
fn resolve_layer_id(
    client: &reqwest::blocking::Client,
    vintage: u16,
    layer: Layer,
) -> CliResult<u64> {
    let url = format!("{TIGERWEB_ROOT}/TIGERweb/tigerWMS_ACS{vintage}/MapServer");
    let meta = get_json(client, &url, &[("f", "json")])?;
    let wanted = layer.service_layer_name();
    let found = meta
        .get("layers")
        .and_then(serde_json::Value::as_array)
        .and_then(|layers| {
            layers.iter().find_map(|l| {
                (l.get("name").and_then(serde_json::Value::as_str) == Some(wanted))
                    .then(|| l.get("id").and_then(serde_json::Value::as_u64))
                    .flatten()
            })
        });
    found.map_or_else(
        || {
            Err(crate::CliError::Other(format!(
                "--geojson auto: no '{wanted}' layer in the Census TIGERweb {vintage} vintage. \
                 Supply an explicit --geojson file."
            )))
        },
        Ok,
    )
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
fn state_fips_from_geoids(codes: &[String], geoid_width: usize) -> Vec<String> {
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

/// Resolve US county boundaries covering the states present in `codes`.
///
/// Scoped by state (never nationwide): the county layer carries a `STATE` field, so the fetch is
/// exactly the states the data touches.
///
/// Whole states are fetched rather than only the counties the data names, so that the boundary set
/// is NOT derived from the candidate region-code column — scoring a column against a feature set
/// built from that same column is circular, and would report a perfect match for any column at
/// all. The unnamed counties are present in the file but are not drawn: a plotly choropleth
/// renders only features whose id appears in `locations`. They are there to make the match
/// fraction meaningful, not to add basemap context.
pub fn resolve_counties(codes: &[String]) -> CliResult<BoundarySet> {
    let states = state_fips_from_geoids(codes, 5);
    if states.is_empty() {
        return Err(crate::CliError::Other(
            "--geojson auto: no US state could be derived from the region-code column, so there \
             is no boundary set to fetch. Check that the column holds 5-digit county FIPS codes, \
             or supply an explicit --geojson file."
                .to_string(),
        ));
    }
    let client = census_client()?;
    let vintage = latest_acs_vintage(&client)?;
    let layer_id = resolve_layer_id(&client, vintage, Layer::County)?;

    let quoted: Vec<String> = states.iter().map(|s| format!("'{s}'")).collect();
    let where_clause = format!("STATE IN ({})", quoted.join(","));
    let features = query_layer_geojson(
        &client,
        vintage,
        layer_id,
        &where_clause,
        "GEOID,NAME,STATE,AREALAND,AREAWATER",
        "GEOID",
    )?;
    if features.is_empty() {
        return Err(crate::CliError::Other(format!(
            "--geojson auto: the Census {vintage} county layer returned no boundaries for state \
             FIPS {}. Supply an explicit --geojson file.",
            states.join(", ")
        )));
    }

    let geojson = serde_json::json!({
        "type": "FeatureCollection",
        "features": features,
    });
    Ok(BoundarySet {
        geojson,
        feature_id_key: "properties.GEOID".to_string(),
        provenance: format!(
            "boundaries: Census TIGERweb {vintage}, {} ({} state{})",
            Layer::County.label(),
            states.len(),
            if states.len() == 1 { "" } else { "s" }
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_fips_pads_leading_zero_stripped_codes() {
        // Alabama's 01001 arrives from a numeric column as 1001
        let codes = vec!["1001".to_string(), "42003".to_string(), "42007".to_string()];
        assert_eq!(state_fips_from_geoids(&codes, 5), vec!["01", "42"]);
    }

    #[test]
    fn state_fips_skips_non_geoid_values() {
        let codes = vec![
            "  42003  ".to_string(),
            String::new(),
            "not-a-code".to_string(),
            "123456789".to_string(),
        ];
        assert_eq!(state_fips_from_geoids(&codes, 5), vec!["42"]);
    }

    #[test]
    fn layer_names_are_the_catalog_spelling() {
        // guards module fact 1: the name is the lookup key, so a typo here is a silent miss
        assert_eq!(Layer::County.service_layer_name(), "Counties");
    }
}
