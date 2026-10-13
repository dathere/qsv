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

#![allow(dead_code)]

use std::path::Path;

use super::{Args, OPENCAGE_FORMATSTR_REGEX, ParamValue, parse_dyncols_spec, parse_param_list};
use crate::CliResult;

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

/// Geocode US addresses or coordinates via the US Census Geocoder.
#[allow(clippy::future_not_send, clippy::unused_async)]
pub(super) async fn run_census(_args: Args, _now_cmd: bool, _cache_dir: &Path) -> CliResult<()> {
    fail_clierror!("The census subcommands are not implemented yet.")
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
}
