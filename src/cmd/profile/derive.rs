//! Stats-derived metadata helpers for `qsv profile`'s projection
//! templates — the qsv-native counterparts of the DP+ ports in
//! `formula_helpers.rs`.
//!
//! Each helper computes a property that is tedious or error-prone to
//! author by hand (resolution, regularity, quality measurements, CSVW
//! column constraints, content language) from the stats / frequency /
//! dpp analysis qsv already ran, or from a Polars-SQL pass over the
//! input CSV. All of them are deterministic and return Jinja UNDEFINED
//! — never an error — when their inputs are absent, so a profile field
//! guarded by `is defined` silently drops out instead of surfacing a
//! render error at Required severity.
//!
//! Two pieces of thread-local state support them:
//!
//!   * a memo of SQL-backed results, so a helper called from both a field's `emit_when` guard and
//!     its `template` scans the CSV once. Cleared whenever the SQL backend is (re)installed.
//!   * a notes sink: helpers record Info-level observations (a dropped null-island coordinate, a
//!     placeholder date) that `projection::emit_field` drains into `projection_warnings` against
//!     the field being rendered.

// minijinja's `Function` trait bounds require by-value args.
#![allow(clippy::needless_pass_by_value)]

use std::{cell::RefCell, collections::HashMap, fmt::Write as _};

use chrono::{Datelike, NaiveDateTime, Timelike};
use minijinja::{
    Environment, Value,
    value::{Rest, Serde},
};
use serde_json::{Value as Json, json};

use super::formula_helpers;

thread_local! {
    static SQL_MEMO: RefCell<HashMap<String, Option<Json>>> = RefCell::new(HashMap::new());
    static NOTES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// Forget memoized SQL results. Called by `set_sql_backend` so a new
/// input never sees a previous input's answers.
pub fn clear_memo() {
    SQL_MEMO.with(|m| m.borrow_mut().clear());
}

/// Record an Info-level observation against the field currently being
/// rendered.
pub(super) fn push_note(msg: String) {
    NOTES.with(|n| n.borrow_mut().push(msg));
}

/// Drain recorded notes, de-duplicated in first-seen order (a field's
/// `emit_when` and `template` both render, so each helper may note
/// twice).
pub fn drain_notes() -> Vec<String> {
    NOTES.with(|n| {
        let mut out: Vec<String> = Vec::new();
        for note in n.borrow_mut().drain(..) {
            if !out.contains(&note) {
                out.push(note);
            }
        }
        out
    })
}

/// Run `f` against the installed SQL backend, memoized under `key`.
/// `None` (no backend, or the query failed) is memoized too, so a
/// failing query is not retried within one render pass.
fn memo_sql(
    key: &str,
    f: impl FnOnce(&super::sql_backend::SqlBackend) -> Option<Json>,
) -> Option<Json> {
    if let Some(hit) = SQL_MEMO.with(|m| m.borrow().get(key).cloned()) {
        return hit;
    }
    let result = formula_helpers::current_sql_backend().and_then(|b| f(&b));
    SQL_MEMO.with(|m| m.borrow_mut().insert(key.to_string(), result.clone()));
    result
}

pub fn register(env: &mut Environment) {
    env.add_function(
        "coordinate_resolution_in_meters",
        coordinate_resolution_in_meters,
    );
    env.add_function("xsd_temporal_resolution", xsd_temporal_resolution);
    env.add_function(
        "confident_accrual_periodicity",
        confident_accrual_periodicity,
    );
    env.add_function("quality_measurements", quality_measurements);
    env.add_function("build_csvw_schema_rich", build_csvw_schema_rich);
    env.add_function("detect_data_language", detect_data_language);
    env.add_function("robust_bbox", robust_bbox);
    env.add_function("with_located", with_located);
    env.add_function("ckan_group_concepts", ckan_group_concepts);
    env.add_function("ckan_group_scheme", ckan_group_scheme);
}

// ---------------------------------------------------------------------
// small JSON accessors
// ---------------------------------------------------------------------

fn to_json(v: &Value) -> Json {
    serde_json::to_value(v).unwrap_or(Json::Null)
}

/// Per-column stats blob, unwrapping the `{stats: {...}}` dpps shape.
fn col_stats<'a>(stats: &'a Json, field: &str) -> Option<&'a Json> {
    let blob = stats.get(field)?;
    Some(blob.get("stats").unwrap_or(blob))
}

#[allow(clippy::cast_precision_loss)]
fn num(v: Option<&Json>) -> Option<f64> {
    let v = v?;
    v.as_f64()
        .or_else(|| v.as_i64().map(|i| i as f64))
        .or_else(|| v.as_str().and_then(|s| s.trim().parse::<f64>().ok()))
}

fn dpp_str<'a>(dpp: &'a Json, key: &str) -> Option<&'a str> {
    dpp.get(key)
        .and_then(Json::as_str)
        .filter(|s| !s.is_empty())
}

fn row_count(dpp: &Json) -> Option<u64> {
    dpp.get("RECORD_COUNT").and_then(Json::as_u64)
}

/// Format to `sig` significant figures without scientific notation
/// (`11.132 → "11.1"`, `1113.2 → "1110"`, `0.0011132 → "0.00111"`).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn sig_figs(x: f64, sig: i32) -> String {
    if x == 0.0 || !x.is_finite() {
        return "0".to_string();
    }
    let magnitude = x.abs().log10().floor() as i32;
    let decimals = sig - 1 - magnitude;
    if decimals > 0 {
        format!("{x:.prec$}", prec = decimals as usize)
    } else {
        let factor = 10f64.powi(-decimals);
        format!("{:.0}", (x / factor).round() * factor)
    }
}

// ---------------------------------------------------------------------
// spatial
// ---------------------------------------------------------------------

/// Metres per degree of latitude (and of longitude at the equator),
/// to the precision a resolution estimate warrants.
const METRES_PER_DEGREE: f64 = 111_320.0;

/// Where a dataset's coordinates are: a latitude and a longitude
/// column, or one WKT `POINT (longitude latitude)` column (Socrata's
/// "Location" shape), as `context` detected them into `dpp`.
pub(super) enum Coords {
    Columns { lat: String, lon: String },
    Point(String),
}

impl Coords {
    /// From `dpp`: `LAT_FIELD`/`LON_FIELD`, else `POINT_FIELD`. `None`
    /// when there are neither, or a name holds a `"` (it is quoted into
    /// SQL).
    pub(super) fn from_dpp(dpp: &Json) -> Option<Self> {
        let coords = match (dpp_str(dpp, "LAT_FIELD"), dpp_str(dpp, "LON_FIELD")) {
            (Some(lat), Some(lon)) => Self::Columns {
                lat: lat.to_string(),
                lon: lon.to_string(),
            },
            _ => Self::Point(dpp_str(dpp, "POINT_FIELD")?.to_string()),
        };
        let names = match &coords {
            Self::Columns { lat, lon } => [lat.as_str(), lon.as_str()],
            Self::Point(p) => [p.as_str(), p.as_str()],
        };
        (!names.iter().any(|n| n.contains('"'))).then_some(coords)
    }

    /// SQL text expressions for the latitude and longitude, over a
    /// backend reading every column as text. For a POINT column, the
    /// first token inside the parentheses is the longitude and the next
    /// the latitude (a `POINT Z`/`M` value's extra numbers are ignored);
    /// a `POINT EMPTY` or non-POINT value yields text that does not
    /// cast, so its row drops out.
    pub(super) fn texts(&self) -> (String, String) {
        match self {
            Self::Columns { lat, lon } => (format!(r#""{lat}""#), format!(r#""{lon}""#)),
            Self::Point(p) => {
                // tabs separate ordinates as well as spaces
                let inner = format!(
                    "(CASE WHEN UPPER(LTRIM(\"{p}\")) LIKE 'POINT%' THEN \
                     TRIM(REPLACE(SPLIT_PART(SPLIT_PART(\"{p}\", '(', 2), ')', 1), '\t', ' ')) \
                     END)"
                );
                // the latitude is the first token after the longitude, so
                // a POINT Z/M value's third and fourth numbers are ignored;
                // with no second token (`POINT (-71)`) there is no latitude,
                // rather than the longitude read twice
                (
                    format!(
                        "(CASE WHEN STRPOS({inner}, ' ') > 0 THEN SPLIT_PART(TRIM(SUBSTR({inner}, \
                         STRPOS({inner}, ' ') + 1)), ' ', 1) END)"
                    ),
                    format!("SPLIT_PART({inner}, ' ', 1)"),
                )
            },
        }
    }

    /// [`Self::texts`] cast to DOUBLE.
    pub(super) fn exprs(&self) -> (String, String) {
        let (la, lo) = self.texts();
        (
            format!("TRY_CAST({la} AS DOUBLE)"),
            format!("TRY_CAST({lo} AS DOUBLE)"),
        )
    }

    /// The column(s), for notes: "`LAT`/`LON` coordinates" or
    /// "`Vehicle Location` points".
    pub(super) fn describe(&self) -> String {
        match self {
            Self::Columns { lat, lon } => format!("`{lat}`/`{lon}` coordinates"),
            Self::Point(p) => format!("`{p}` points"),
        }
    }

    fn key(&self) -> String {
        match self {
            Self::Columns { lat, lon } => format!("{lat}\u{1f}{lon}"),
            Self::Point(p) => format!("point\u{1f}{p}"),
        }
    }
}

/// One memoized SQL scan over the plausible coordinates — in range, and
/// not the (0, 0) "null island" placeholder: their bounds, the number of
/// rows excluded, each axis' decimal precision and the mean latitude.
/// `None` when no backend is installed, the query fails, or no row
/// survives the filter.
fn coord_summary(coords: &Coords) -> Option<Json> {
    memo_sql(&format!("coord_summary\u{1f}{}", coords.key()), |backend| {
        // as text, so integer-looking leading rows don't type the column
        // as integer and fail the scan at the first decimal
        let backend = backend.clone().with_all_strings();
        let (la, lo) = coords.exprs();
        let (la_t, lo_t) = coords.texts();
        let keep = format!(
            "{la} BETWEEN -90 AND 90 AND {lo} BETWEEN -180 AND 180 AND NOT ({la} = 0 AND {lo} = 0)"
        );
        let places = |text: &str| {
            format!("MAX(CASE WHEN {keep} THEN LENGTH(SPLIT_PART(TRIM({text}), '.', 2)) END)")
        };
        let sql = format!(
            "SELECT MIN(CASE WHEN {keep} THEN {lo} END) AS min_lon, MAX(CASE WHEN {keep} THEN \
             {lo} END) AS max_lon, MIN(CASE WHEN {keep} THEN {la} END) AS min_lat, MAX(CASE WHEN \
             {keep} THEN {la} END) AS max_lat, AVG(CASE WHEN {keep} THEN {la} END) AS mean_lat, \
             {} AS p_lat, {} AS p_lon, SUM(CASE WHEN {la} IS NOT NULL AND {lo} IS NOT NULL AND \
             NOT ({keep}) THEN 1 ELSE 0 END) AS dropped FROM data",
            places(&la_t),
            places(&lo_t),
        );
        let df = backend.query(&sql).ok()?;
        let get = |c: &str| -> Option<f64> {
            let col = df.column(c).ok()?;
            let cast = col.cast(&polars::prelude::DataType::Float64).ok()?;
            cast.f64().ok()?.get(0)
        };
        Some(json!({
            "min_lon": get("min_lon")?,
            "max_lon": get("max_lon")?,
            "min_lat": get("min_lat")?,
            "max_lat": get("max_lat")?,
            "mean_lat": get("mean_lat")?,
            "p_lat": get("p_lat").unwrap_or(0.0),
            "p_lon": get("p_lon").unwrap_or(0.0),
            "dropped": get("dropped").unwrap_or(0.0),
        }))
    })
}

/// `coordinate_resolution_in_meters(dpp, stats)` — the finest spatial
/// separation the coordinates can express, from the decimal precision
/// of the lat/lon columns.
///
/// DCAT-US v3 defines `spatialResolutionInMeters` as the "smallest
/// spatial distance between data points". A coordinate recorded to `p`
/// decimal places cannot distinguish points closer than `10⁻ᵖ` degrees,
/// i.e. ~111 320·10⁻ᵖ m north–south and that times cos(latitude)
/// east–west; the coarser of the two axes bounds what the data
/// resolves. (The DP+ port `spatial_resolution_in_meters` returns the
/// bbox *diagonal* — the extent — and is kept as-is for DP+ parity.)
///
/// For a WKT POINT column, which has no numeric stats, the decimal places
/// are counted by the coordinate scan instead.
fn coordinate_resolution_in_meters(dpp: Value, stats: Value) -> Value {
    let (dpp, stats) = (to_json(&dpp), to_json(&stats));
    let (p_lat, p_lon, mean_lat) = match Coords::from_dpp(&dpp) {
        Some(Coords::Columns { lat, lon }) => {
            let (Some(lat_s), Some(lon_s)) = (col_stats(&stats, &lat), col_stats(&stats, &lon))
            else {
                return Value::UNDEFINED;
            };
            let (Some(p_lat), Some(p_lon)) = (
                lat_s.get("max_precision").and_then(Json::as_u64),
                lon_s.get("max_precision").and_then(Json::as_u64),
            ) else {
                return Value::UNDEFINED;
            };
            let mean_lat = num(lat_s.get("mean"))
                .or_else(|| {
                    Some(f64::midpoint(
                        num(lat_s.get("min"))?,
                        num(lat_s.get("max"))?,
                    ))
                })
                .unwrap_or(0.0);
            (p_lat, p_lon, mean_lat)
        },
        // a POINT column has no numeric stats: the coordinate scan counts
        // each axis' decimal places
        Some(point @ Coords::Point(_)) => {
            let Some(s) = coord_summary(&point) else {
                return Value::UNDEFINED;
            };
            let f = |k: &str| s.get(k).and_then(Json::as_f64);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let places = |k: &str| f(k).unwrap_or(0.0).max(0.0) as u64;
            (
                places("p_lat"),
                places("p_lon"),
                f("mean_lat").unwrap_or(0.0),
            )
        },
        None => return Value::UNDEFINED,
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let step = |p: u64| 10f64.powi(-(p.min(15) as i32));
    let ns = METRES_PER_DEGREE * step(p_lat);
    let ew = METRES_PER_DEGREE * mean_lat.to_radians().cos().abs() * step(p_lon);
    Value::from(sig_figs(ns.max(ew), 3))
}

/// `robust_bbox(dpp, stats)` — the `spatial` Location array from the
/// lat/lon (or WKT POINT) columns, excluding coordinates that cannot be real.
///
/// The stats min/max are used directly when they are already sane —
/// within ±90/±180 and with no exact-zero coordinate in either column —
/// so a clean file costs no extra scan. Otherwise a SQL pass recomputes
/// the bounds over rows with both coordinates in range and not the
/// (0, 0) "null island" placeholder, and the number of excluded rows is
/// noted. Without this, a single geocoder-failure row at (0, 0) widens
/// a Boston dataset's bbox to the Gulf of Guinea.
///
/// The out-of-range half of the filter is defensive: through `profile`
/// today, `context::detect_lat_lon` already rejects a column whose
/// min/max leaves ±90/±180, so `LAT_FIELD`/`LON_FIELD` never name one.
/// The (0, 0) half is the one real inputs reach. A custom `dpp` (or a
/// future, more tolerant detector) still gets correct bounds.
///
/// A WKT POINT column (`dpp.POINT_FIELD`) has no numeric stats, so its
/// bounds always come from the SQL pass; when no value parses, that is
/// noted.
fn robust_bbox(dpp: Value, stats: Value) -> Value {
    let (dpp, stats) = (to_json(&dpp), to_json(&stats));
    let Some(coords) = Coords::from_dpp(&dpp) else {
        return Value::UNDEFINED;
    };
    let bounds = match &coords {
        Coords::Columns { lat, lon } => {
            let (Some(lat_s), Some(lon_s)) = (col_stats(&stats, lat), col_stats(&stats, lon))
            else {
                return Value::UNDEFINED;
            };
            let bounds = (
                num(lon_s.get("min")),
                num(lon_s.get("max")),
                num(lat_s.get("min")),
                num(lat_s.get("max")),
            );
            let (Some(min_lon), Some(max_lon), Some(min_lat), Some(max_lat)) = bounds else {
                return Value::UNDEFINED;
            };
            let zeros = |s: &Json| s.get("n_zero").and_then(Json::as_u64).unwrap_or(0);
            let sane = (-90.0..=90.0).contains(&min_lat)
                && (-90.0..=90.0).contains(&max_lat)
                && (-180.0..=180.0).contains(&min_lon)
                && (-180.0..=180.0).contains(&max_lon)
                && zeros(lat_s) == 0
                && zeros(lon_s) == 0;
            if sane {
                Some((min_lon, max_lon, min_lat, max_lat))
            } else {
                filtered_bounds(&coords)
            }
        },
        // a POINT column has no numeric stats, so its bounds always come
        // from the coordinate scan
        Coords::Point(p) => {
            let b = filtered_bounds(&coords);
            if b.is_none() && formula_helpers::current_sql_backend().is_some() {
                push_note(format!(
                    "`{p}` looks like a WKT POINT column, but none of its values parse as a \
                     plausible `POINT (longitude latitude)`; no bounding box was computed"
                ));
            }
            b
        },
    };
    let Some((min_lon, max_lon, min_lat, max_lat)) = bounds else {
        return Value::UNDEFINED;
    };
    let polygon = format!(
        "POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, {max_lon} \
         {min_lat}, {min_lon} {min_lat}))"
    );
    Value::from(Serde(json!([{ "@type": "Location", "bbox": polygon }])))
}

/// Bounds over plausible coordinates only; notes how many rows were
/// excluded. `None` when no backend is installed, the query fails, or
/// no row survives the filter.
fn filtered_bounds(coords: &Coords) -> Option<(f64, f64, f64, f64)> {
    let hit = coord_summary(coords)?;
    let f = |k: &str| hit.get(k).and_then(Json::as_f64);
    let dropped = f("dropped").unwrap_or(0.0);
    if dropped > 0.0 {
        push_note(format!(
            "excluded {dropped:.0} row(s) whose {} are out of range or the (0, 0) null-island \
             placeholder from the bounding box",
            coords.describe()
        ));
    }
    Some((f("min_lon")?, f("max_lon")?, f("min_lat")?, f("max_lat")?))
}

/// `with_located(base, located)` — the `spatial` Location array with the
/// `--locate` results folded in: `located.primary` (prefLabel,
/// identifier, gazetteer, centroid) merges into the first Location —
/// the bbox, whose label it is — and `located.regions` (Census states
/// and counties) follow it. Keys the base Location already has win.
/// Without `--locate` (`located` null) the base passes through
/// unchanged; with no base, the regions alone are returned.
fn with_located(base: Value, located: Value) -> Value {
    let located = to_json(&located);
    let base_json = to_json(&base);
    if !located.is_object() {
        return base;
    }
    for note in located
        .get("notes")
        .and_then(Json::as_array)
        .into_iter()
        .flatten()
        .filter_map(Json::as_str)
    {
        push_note(note.to_string());
    }
    let mut out: Vec<Json> = base_json.as_array().cloned().unwrap_or_default();
    if let (Some(first), Some(primary)) = (
        out.first_mut().and_then(Json::as_object_mut),
        located.get("primary").and_then(Json::as_object),
    ) {
        for (k, v) in primary {
            first.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
    out.extend(
        located
            .get("regions")
            .and_then(Json::as_array)
            .into_iter()
            .flatten()
            .cloned(),
    );
    if out.is_empty() {
        return Value::UNDEFINED;
    }
    Value::from(Serde(Json::Array(out)))
}

/// `ckan_group_concepts(groups, scheme)` — CKAN package groups as DCAT-US v3 `theme`
/// Concepts: `prefLabel` from the group's `title`, else `display_name`, else `name` (a
/// bare-string group is its own label), `definition` from a non-empty `description`, and
/// `inScheme` the `scheme` `ConceptScheme` (see `ckan_group_scheme`) when it has a title.
/// A group with no label is skipped: a Concept without `prefLabel` is invalid. UNDEFINED
/// when no group has a label.
fn ckan_group_concepts(groups: Value, scheme: Value) -> Value {
    let groups = to_json(&groups);
    let scheme = to_json(&scheme);
    let scheme = scheme
        .get("title")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.trim().is_empty())
        .then_some(scheme);
    let text = |g: &Json, key: &str| {
        g.get(key)
            .and_then(Json::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };
    let concepts: Vec<Json> = groups
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|g| {
            let label = match g {
                Json::String(s) => Some(s.trim().to_string()).filter(|s| !s.is_empty()),
                _ => text(g, "title")
                    .or_else(|| text(g, "display_name"))
                    .or_else(|| text(g, "name")),
            }?;
            let mut concept = json!({ "@type": "Concept", "prefLabel": label });
            if let Some(definition) = text(g, "description") {
                concept["definition"] = json!(definition);
            }
            if let Some(scheme) = &scheme {
                concept["inScheme"] = scheme.clone();
            }
            Some(concept)
        })
        .collect();
    if concepts.is_empty() {
        return Value::UNDEFINED;
    }
    Value::from(Serde(Json::Array(concepts)))
}

/// `ckan_group_scheme(urls, fallback_title, portal_title)` — the `ConceptScheme` CKAN group
/// Concepts belong to: the portal's own group list. The portal is found from the first of
/// `urls` (the resource URL, then the input) that has CKAN's `/dataset/` path: its root is
/// everything before that, so a CKAN under a sub-path is handled, and the scheme is
/// `{"@id": "<root>/group/", "title": portal_title with {portal} replaced}`. A URL without
/// that path may point off the portal (a linked resource), so it is not used; with none,
/// the scheme is just `fallback_title`.
fn ckan_group_scheme(urls: Value, fallback_title: Value, portal_title: Value) -> Value {
    let urls = to_json(&urls);
    let portal = urls
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Json::as_str)
        .find_map(ckan_portal_root);
    let fallback = to_json(&fallback_title);
    let scheme = match (portal, to_json(&portal_title).as_str()) {
        (Some(root), Some(pattern)) if !pattern.trim().is_empty() => {
            let name = root
                .split_once("://")
                .map_or(root.as_str(), |(_, rest)| rest);
            json!({
                "@type": "ConceptScheme",
                "@id": format!("{root}/group/"),
                "title": pattern.replace("{portal}", name),
            })
        },
        _ => match fallback.as_str().map(str::trim).filter(|t| !t.is_empty()) {
            Some(title) => json!({ "@type": "ConceptScheme", "title": title }),
            None => return Value::UNDEFINED,
        },
    };
    Value::from(Serde(scheme))
}

/// The root of the CKAN portal serving `url`: scheme, host and any path before
/// `/dataset/`, without a trailing slash. None for a URL without that path.
fn ckan_portal_root(url: &str) -> Option<String> {
    let parsed = url::Url::parse(url.trim()).ok()?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return None;
    }
    let path = parsed.path();
    let prefix = &path[..path.find("/dataset/")?];
    Some(format!(
        "{}{}",
        parsed.origin().ascii_serialization(),
        prefix.trim_end_matches('/')
    ))
}

// ---------------------------------------------------------------------
// temporal
// ---------------------------------------------------------------------

/// Distinct, sorted, parsed values of `field` via the SQL backend.
fn distinct_datetimes(field: &str) -> Option<Vec<NaiveDateTime>> {
    let key = format!("distinct_dt\u{1f}{field}");
    let hit = memo_sql(&key, |backend| {
        let strs = backend.distinct_sorted_date_strings(field).ok()?;
        Some(Json::from(strs))
    })?;
    let strs: Vec<String> = serde_json::from_value(hit).ok()?;
    let mut dts = formula_helpers::parse_date_strings(&strs).ok()?;
    dts.sort_unstable();
    dts.dedup();
    Some(dts)
}

fn date_field(args: &Rest<Value>, state: &minijinja::State) -> Option<String> {
    formula_helpers::resolve_date_field(args.first(), state).ok()
}

/// Whole calendar months from `a` to `b` when both fall on the same
/// day of month at midnight (`2024-01-15 → 2024-04-15` is 3), else
/// `None`.
fn calendar_months(a: NaiveDateTime, b: NaiveDateTime) -> Option<i32> {
    if a.day() != b.day() || a.time() != b.time() {
        return None;
    }
    #[allow(clippy::cast_possible_wrap)]
    let months = (b.year() - a.year()) * 12 + (b.month() as i32 - a.month() as i32);
    (months > 0).then_some(months)
}

/// `xsd_temporal_resolution(field?)` — the temporal resolution of a
/// date/datetime column as an exact xsd:duration (`PT1M`, `PT15M`,
/// `P1D`, `P1M`, `P1Y`, ...).
///
///   * A regular series — ≥90% of gaps one calendar step of whole months on the same day (`P{n}M` /
///     `P{n}Y`, not a day count that varies from 28 to 31), or one exact interval (`PT15M`, `P1D`,
///     `P7D`) — reports its step.
///   * Anything else reports the precision its values are RECORDED at: date-only → `P1D`, whole
///     hours → `PT1H`, minutes → `PT1M`, seconds → `PT1S`, milliseconds → `PT0.001S`. For irregular
///     events the smallest gap is a sampling accident (two 311 requests 42 s apart), not the
///     resolution; the recording precision is the minimum time period the data can resolve.
///
/// Unlike the DP+ port `temporal_resolution`, sub-day values are not
/// all collapsed to `PT1H`. `field` defaults to the first detected
/// date, then datetime, column.
#[allow(clippy::cast_precision_loss)]
fn xsd_temporal_resolution(args: Rest<Value>, state: &minijinja::State) -> Value {
    let Some(field) = date_field(&args, state) else {
        return Value::UNDEFINED;
    };
    let Some(dts) = distinct_datetimes(&field) else {
        return Value::UNDEFINED;
    };
    if dts.len() < 2 {
        return Value::UNDEFINED;
    }
    let pairs: Vec<(NaiveDateTime, NaiveDateTime)> = dts
        .iter()
        .copied()
        .zip(dts.iter().copied().skip(1))
        .collect();
    if let Some(months) = pairs
        .iter()
        .map(|(a, b)| calendar_months(*a, *b))
        .collect::<Option<Vec<i32>>>()
    {
        // Month-aligned, but only a regular grid has a step: Jan, Feb,
        // Jun, Dec are month-aligned with no common interval.
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for m in &months {
            *counts.entry(*m).or_insert(0) += 1;
        }
        if let Some((step, count)) = counts
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            && count as f64 / months.len() as f64 >= PERIODICITY_MIN_SHARE
        {
            return Value::from(if step % 12 == 0 {
                format!("P{}Y", step / 12)
            } else {
                format!("P{step}M")
            });
        }
        return Value::from(recording_precision(&dts));
    }
    let gaps: Vec<i64> = pairs
        .iter()
        .map(|(a, b)| b.signed_duration_since(*a).num_milliseconds())
        .filter(|ms| *ms > 0)
        .collect();
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for g in &gaps {
        *counts.entry(*g).or_insert(0) += 1;
    }
    // Deterministic tie-break: highest count, then the smaller gap.
    if let Some((step, count)) = counts
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
        && count as f64 / gaps.len() as f64 >= PERIODICITY_MIN_SHARE
    {
        return Value::from(format_xsd_duration_ms(step));
    }
    Value::from(recording_precision(&dts))
}

/// The finest unit every value is recorded at, as an xsd:duration.
fn recording_precision(dts: &[NaiveDateTime]) -> &'static str {
    let all = |f: &dyn Fn(&NaiveDateTime) -> bool| dts.iter().all(f);
    if all(&|d| d.nanosecond() == 0 && d.second() == 0 && d.minute() == 0 && d.hour() == 0) {
        "P1D"
    } else if all(&|d| d.nanosecond() == 0 && d.second() == 0 && d.minute() == 0) {
        "PT1H"
    } else if all(&|d| d.nanosecond() == 0 && d.second() == 0) {
        "PT1M"
    } else if all(&|d| d.nanosecond() == 0) {
        "PT1S"
    } else {
        "PT0.001S"
    }
}

fn format_xsd_duration_ms(ms: i64) -> String {
    const SEC: i64 = 1_000;
    const MIN: i64 = 60 * SEC;
    const HOUR: i64 = 60 * MIN;
    const DAY: i64 = 24 * HOUR;
    if ms % DAY == 0 {
        format!("P{}D", ms / DAY)
    } else if ms % HOUR == 0 {
        format!("PT{}H", ms / HOUR)
    } else if ms % MIN == 0 {
        format!("PT{}M", ms / MIN)
    } else if ms % SEC == 0 {
        format!("PT{}S", ms / SEC)
    } else {
        let mut s = format!("PT{}.{:03}", ms / SEC, ms % SEC);
        while s.ends_with('0') {
            s.pop();
        }
        s.push('S');
        s
    }
}

/// Minimum share of consecutive gaps that must equal the dominant gap
/// before a schedule is asserted.
const PERIODICITY_MIN_SHARE: f64 = 0.9;
/// Minimum distinct dates (so at least 3 gaps) before a schedule is
/// asserted.
const PERIODICITY_MIN_DISTINCT: usize = 4;

/// `confident_accrual_periodicity(field?)` — an ISO 19115 maintenance
/// frequency code (`daily`, `weekly`, `monthly`, ...) ONLY when the
/// column's values form a regular calendar grid.
///
/// `accrualPeriodicity` is how often the dataset is *updated*; the
/// intervals between rows are only evidence of that when the rows are
/// themselves a regular series (one row per day, per month-start, a
/// daily 08:00 reading, ...). Event timestamps (311 requests,
/// inspections) have irregular gaps and say nothing about publishing
/// cadence, so the helper declines unless there are ≥4 distinct values
/// and ≥90% of consecutive gaps are the same calendar step — whole
/// months on the same day of month and time, or an EXACT whole number
/// of days (a 1 d 3 h gap is no step at all, not "1 day"). Declining is
/// silent: it is the common case. A wrong Recommended value is worse
/// than an absent one. (The DP+ port `guess_accrual_periodicity`
/// returns the modal gap unconditionally and is kept for DP+ parity.)
#[allow(clippy::cast_precision_loss)]
fn confident_accrual_periodicity(args: Rest<Value>, state: &minijinja::State) -> Value {
    const DAY_SECS: i64 = 86_400;
    let Some(field) = date_field(&args, state) else {
        return Value::UNDEFINED;
    };
    let Some(dts) = distinct_datetimes(&field) else {
        return Value::UNDEFINED;
    };
    if dts.len() < PERIODICITY_MIN_DISTINCT {
        return Value::UNDEFINED;
    }
    let steps: Vec<Option<String>> = dts
        .iter()
        .zip(dts.iter().skip(1))
        .map(|(a, b)| {
            if let Some(m) = calendar_months(*a, *b) {
                return Some(format!("M{m}"));
            }
            let gap = b.signed_duration_since(*a);
            let secs = gap.num_seconds();
            let whole = gap.subsec_nanos() == 0 && secs > 0 && secs % DAY_SECS == 0;
            whole.then(|| format!("D{}", secs / DAY_SECS))
        })
        .collect();
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for s in steps.iter().flatten() {
        *counts.entry(s.as_str()).or_insert(0) += 1;
    }
    // Deterministic tie-break: highest count, then lexical step label.
    let Some((step, count)) = counts
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(a.0)))
    else {
        return Value::UNDEFINED;
    };
    // Gaps that are no calendar step at all count against the share.
    let share = count as f64 / steps.len() as f64;
    let code = match step {
        "D1" => Some("daily"),
        "D7" => Some("weekly"),
        "D14" => Some("fortnightly"),
        "M1" => Some("monthly"),
        "M2" => Some("bimonthly"),
        "M3" => Some("quarterly"),
        "M6" => Some("biannually"),
        "M12" => Some("annually"),
        _ => None,
    };
    match code {
        Some(code) if share >= PERIODICITY_MIN_SHARE => Value::from(code),
        _ => Value::UNDEFINED,
    }
}

// ---------------------------------------------------------------------
// quality
// ---------------------------------------------------------------------

/// `quality_measurements(stats, dpp, dims, rfc4180)` — DQV quality
/// measurements for `hasQualityMeasurement`, computed from stats plus,
/// on a `--validate` run (`rfc4180` non-null), a hashed duplicate-row
/// count and the RFC 4180 / UTF-8 result of `qsv validate`.
///
/// `dims` maps measurement kinds (`completeness`, `conciseness`,
/// `syntacticValidity`) to dimension IRIs — the profile supplies them
/// from its `quality_dimensions` vocabulary so authors can swap
/// vocabularies. A kind with no IRI is omitted. Values are strings, as
/// DCAT-US v3 `QualityMeasurement.value` requires.
#[allow(clippy::cast_precision_loss)]
fn quality_measurements(stats: Value, dpp: Value, dims: Value, rfc4180: Value) -> Value {
    let (stats, dpp, dims, rfc4180) = (
        to_json(&stats),
        to_json(&dpp),
        to_json(&dims),
        to_json(&rfc4180),
    );
    let Some(cols) = stats.as_object() else {
        return Value::UNDEFINED;
    };
    let Some(rows) = row_count(&dpp).filter(|r| *r > 0) else {
        return Value::UNDEFINED;
    };
    if cols.is_empty() {
        return Value::UNDEFINED;
    }
    let dim = |k: &str| dims.get(k).and_then(Json::as_str).filter(|s| !s.is_empty());
    let measurement =
        |dimension: &str, dtype: &str, definition: &str, value: String, unit: &str| {
            let mut m = json!({
                "@type": "QualityMeasurement",
                "isMeasurementOf": {
                    "@type": "Metric",
                    "expectedDataType": dtype,
                    "inDimension": dimension,
                    "definition": definition,
                },
                "value": value,
            });
            if !unit.is_empty() {
                m["unitMeasure"] = Json::from(unit);
            }
            m
        };

    let nullcounts: Vec<u64> = cols
        .keys()
        .map(|c| {
            col_stats(&stats, c)
                .and_then(|s| s.get("nullcount"))
                .and_then(Json::as_u64)
                .unwrap_or(0)
        })
        .collect();
    let total_nulls: u64 = nullcounts.iter().sum();
    let cells = rows as f64 * cols.len() as f64;
    let mut out: Vec<Json> = Vec::new();

    if let Some(d) = dim("completeness") {
        let pct = 100.0 * (1.0 - total_nulls as f64 / cells);
        out.push(measurement(
            d,
            "xsd:decimal",
            "Percentage of non-empty cells in the dataset.",
            format!("{pct:.2}"),
            "percent",
        ));
        let empty_cols = nullcounts.iter().filter(|n| **n >= rows).count();
        out.push(measurement(
            d,
            "xsd:integer",
            "Number of columns with no non-empty values.",
            empty_cols.to_string(),
            "columns",
        ));
        let partial_cols = nullcounts.iter().filter(|n| **n > 0).count();
        out.push(measurement(
            d,
            "xsd:integer",
            "Number of columns with at least one empty value.",
            partial_cols.to_string(),
            "columns",
        ));
    }

    // The duplicate-row count is one more pass over the file, keeping a
    // 128-bit hash per distinct row (see `duplicate_row_count`), so it
    // runs only on a --validate pass — signalled by the RFC 4180 result
    // being present — never on the default path. It compares the text the
    // file holds: `001` and `1` are different rows.
    let validated = rfc4180.is_object();
    if validated
        && let Some(d) = dim("conciseness")
        && let Some(duplicates) = memo_sql("duplicate_rows", |backend| {
            backend.duplicate_row_count().ok().map(Json::from)
        })
        .and_then(|v| v.as_u64())
    {
        out.push(measurement(
            d,
            "xsd:integer",
            "Number of rows that exactly duplicate an earlier row.",
            duplicates.to_string(),
            "rows",
        ));
    }

    if let Some(d) = dim("syntacticValidity")
        && let Some(valid) = rfc4180.get("valid").and_then(Json::as_bool)
    {
        out.push(measurement(
            d,
            "xsd:boolean",
            "Whether the file is well-formed delimited text (consistent field counts and quoting) \
             encoded as UTF-8 (qsv validate).",
            valid.to_string(),
            "",
        ));
    }

    if out.is_empty() {
        Value::UNDEFINED
    } else {
        Value::from(Serde(out))
    }
}

// ---------------------------------------------------------------------
// CSVW
// ---------------------------------------------------------------------

/// Above this cardinality a column is not described as an enumeration.
const CSVW_ENUM_MAX: u64 = 10;

/// `build_csvw_schema_rich(stats, freq, headers?, rows?, dictionary?)` — a CSVW
/// `tableSchema` whose columns carry machine-checkable constraints
/// derived from stats and frequency, on top of the `build_csvw_schema`
/// shape:
///
///   * `required: true` when the column has no empty values;
///   * `datatype.minimum` / `maximum` for numbers and dates;
///   * `datatype.minLength` / `maxLength` for ASCII string columns (qsv counts bytes, CSVW counts
///     characters, so non-ASCII columns get none);
///   * `datatype.format` — an anchored regex enumerating the values — when a string column has at
///     most `CSVW_ENUM_MAX` distinct non-empty values and frequency captured all of them;
///   * table-level `primaryKey`: the first Integer/String column with no empty values whose
///     cardinality EQUALS `rows` (the rounded `uniqueness_ratio` reads 1.0 for 99,999 distinct
///     values in 100,000 rows, so it is not evidence). Omitted when `rows` is not given.
///
/// `dictionary` (the `llm.dictionary` map of column name →
/// `{label, description}` from describegpt) adds the label to a
/// column's `titles` and its description as `dc:description`, with an
/// Info note that they are LLM-generated.
///
/// Columns follow `headers` (the file's order) when given. The legacy
/// `qsv:*` per-column extensions are kept so existing consumers keep
/// working.
fn build_csvw_schema_rich(
    stats: Value,
    freq: Value,
    headers: Option<Value>,
    rows: Option<Value>,
    dictionary: Option<Value>,
) -> Value {
    let (stats, freq) = (to_json(&stats), to_json(&freq));
    let dictionary = dictionary.map(|d| to_json(&d)).filter(Json::is_object);
    let mut annotated = 0usize;
    let Some(cols) = stats.as_object() else {
        return Value::UNDEFINED;
    };
    let rows: Option<u64> = rows.and_then(|r| to_json(&r).as_u64());
    // CSVW requires schema columns in the same order as the table's.
    // The stats map arrives key-sorted (minijinja maps are ordered by
    // key), so follow `headers` — the file's column order — when given,
    // appending any stats column it does not list.
    let mut order: Vec<String> = headers
        .map(|h| to_json(&h))
        .and_then(|h| serde_json::from_value::<Vec<String>>(h).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|h| cols.contains_key(h))
        .collect();
    for k in cols.keys() {
        if !order.contains(k) {
            order.push(k.clone());
        }
    }
    let mut columns: Vec<Json> = Vec::with_capacity(cols.len());
    let mut primary_key: Option<String> = None;
    for name in &order {
        let Some(s) = col_stats(&stats, name) else {
            continue;
        };
        let ty = s.get("type").and_then(Json::as_str).unwrap_or("String");
        let base = formula_helpers::csvw_datatype_legacy(s.get("type"));
        let nullcount = s.get("nullcount").and_then(Json::as_u64).unwrap_or(0);
        let cardinality = s.get("cardinality").and_then(Json::as_u64);

        let mut datatype = serde_json::Map::new();
        datatype.insert("base".to_string(), Json::from(base));
        match ty {
            "Integer" | "Float" => {
                for (k, src) in [("minimum", "min"), ("maximum", "max")] {
                    let int = (ty == "Integer")
                        .then(|| {
                            s.get(src)
                                .and_then(Json::as_str)?
                                .trim()
                                .parse::<i64>()
                                .ok()
                        })
                        .flatten();
                    if let Some(i) = int {
                        datatype.insert(k.to_string(), Json::from(i));
                    } else if let Some(v) = num(s.get(src)) {
                        datatype.insert(k.to_string(), json!(v));
                    }
                }
            },
            "Date" | "DateTime" => {
                for (k, src) in [("minimum", "min"), ("maximum", "max")] {
                    if let Some(v) = s.get(src).and_then(Json::as_str) {
                        datatype.insert(k.to_string(), Json::from(v));
                    }
                }
            },
            "String" => {
                // qsv's length stats count UTF-8 BYTES; CSVW lengths
                // count characters. They agree only for ASCII, so a
                // non-ASCII column gets no length bounds (a `café`
                // column would otherwise demand minLength 5 of its own
                // 4-character values).
                if s.get("is_ascii").and_then(Json::as_bool) == Some(true) {
                    for (k, src) in [("minLength", "min_length"), ("maxLength", "max_length")] {
                        if let Some(v) = s.get(src).and_then(Json::as_u64) {
                            datatype.insert(k.to_string(), Json::from(v));
                        }
                    }
                }
                if let Some(re) = enumeration_regex(&freq, name, cardinality, nullcount) {
                    datatype.insert("format".to_string(), Json::from(re));
                }
            },
            _ => {},
        }

        let mut col = json!({
            "name": name,
            "titles": [name],
            "datatype": Json::Object(datatype),
            "qsv:cardinality": s.get("cardinality"),
            "qsv:nullcount": s.get("nullcount"),
            "qsv:min": s.get("min"),
            "qsv:max": s.get("max"),
        });
        if nullcount == 0 && ty != "NULL" {
            col["required"] = Json::Bool(true);
        }
        if let Some(entry) = dictionary.as_ref().and_then(|d| d.get(name)) {
            let text = |k: &str| {
                entry
                    .get(k)
                    .and_then(Json::as_str)
                    .filter(|s| !s.is_empty())
            };
            if let Some(label) = text("label").filter(|l| l != name) {
                col["titles"] = json!([name, label]);
            }
            if let Some(desc) = text("description") {
                col["dc:description"] = Json::from(desc);
            }
            annotated += 1;
        }
        columns.push(col);

        if primary_key.is_none()
            && nullcount == 0
            && matches!(ty, "Integer" | "String")
            && let (Some(c), Some(r)) = (cardinality, rows)
            && c > 1
            && c == r
        {
            primary_key = Some(name.clone());
        }
    }
    if annotated > 0 {
        push_note(format!(
            "{annotated} column title(s)/description(s) come from an LLM data dictionary (qsv \
             describegpt); verify before using"
        ));
    }
    let mut schema = json!({ "columns": columns });
    if let Some(pk) = primary_key {
        schema["primaryKey"] = Json::from(pk);
    }
    Value::from(Serde(schema))
}

/// Anchored alternation of every non-empty value of `field`, or `None`
/// when the column is not a small, fully-captured enumeration.
fn enumeration_regex(
    freq: &Json,
    field: &str,
    cardinality: Option<u64>,
    nullcount: u64,
) -> Option<String> {
    let cardinality = cardinality?;
    // qsv's cardinality counts the empty value as one distinct value;
    // the profile frequency pass drops nulls.
    let non_empty = cardinality - u64::from(nullcount > 0);
    if non_empty == 0 || non_empty > CSVW_ENUM_MAX {
        return None;
    }
    let entries = freq.get(field)?.as_array()?;
    let mut values: Vec<&str> = entries
        .iter()
        .filter(|e| e.get("rank").and_then(Json::as_f64).is_none_or(|r| r > 0.0))
        .filter_map(|e| e.get("value").and_then(Json::as_str))
        .collect();
    if values.len() as u64 != non_empty {
        return None;
    }
    values.sort_unstable();
    let mut re = String::from("^(?:");
    for (i, v) in values.iter().enumerate() {
        if i > 0 {
            re.push('|');
        }
        let _ = write!(re, "{}", regex::escape(v));
    }
    re.push_str(")$");
    Some(re)
}

// ---------------------------------------------------------------------
// language
// ---------------------------------------------------------------------

/// `detect_data_language(stats, freq)` — the ISO 639-1 code of the
/// natural language of the dataset's text values, or UNDEFINED.
///
/// Reuses describegpt's detector, which requires every disjoint chunk
/// of the sample to agree before accepting a language (whatlang's own
/// confidence is a margin between candidates, not evidence — issue
/// #4406). The sample is the frequency-ranked values of String columns
/// plus their stats min/max. Only languages with an ISO 639-1 code are
/// returned, because that is what DCAT-US v3 `language` accepts
/// (`maxLength: 2`). Builds without the `whatlang` feature (qsvdp,
/// qsvmcp) always return UNDEFINED.
fn detect_data_language(stats: Value, freq: Value) -> Value {
    #[cfg(feature = "whatlang")]
    {
        let (stats, freq) = (to_json(&stats), to_json(&freq));
        let sample = language_sample(&stats, &freq);
        if let Some((_, code3, _)) = crate::cmd::describegpt::detect_language_in_text(&sample, None)
            && let Some(code) = iso_639_3_to_1(&code3)
        {
            return Value::from(code);
        }
        Value::UNDEFINED
    }
    #[cfg(not(feature = "whatlang"))]
    {
        let _ = (stats, freq);
        Value::UNDEFINED
    }
}

#[cfg(feature = "whatlang")]
fn language_sample(stats: &Json, freq: &Json) -> String {
    const MAX_SAMPLE_BYTES: usize = 8_192;
    let Some(cols) = stats.as_object() else {
        return String::new();
    };
    let string_cols: Vec<&String> = cols
        .keys()
        .filter(|c| {
            col_stats(stats, c)
                .and_then(|s| s.get("type"))
                .and_then(Json::as_str)
                == Some("String")
        })
        .collect();
    let mut sample = String::with_capacity(4_096);
    // Each distinct value contributes once. The detector's safeguard is
    // that disjoint chunks of the sample must AGREE, and repeated text
    // defeats it: a column holding only "Central Park" (frequency entry
    // + stats min + max) yields three identical chunks that trivially
    // agree — on Spanish. De-duplicating leaves too little text to
    // chunk, so detection correctly declines.
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut push = |v: &str| {
        let v = v.trim();
        if v.is_empty()
            || sample.len() + v.len() + 1 > MAX_SAMPLE_BYTES
            || !seen.insert(v.to_string())
        {
            return;
        }
        if !sample.is_empty() {
            sample.push(' ');
        }
        sample.push_str(v);
    };
    for c in &string_cols {
        let Some(entries) = freq.get(c.as_str()).and_then(Json::as_array) else {
            continue;
        };
        // Rank 0 is frequency's aggregate placeholder — `<ALL_UNIQUE>`
        // for a column whose every value is distinct. That is exactly
        // the free-text column (comments, descriptions) that carries
        // the most language signal, so sample it from the file instead.
        let mut all_unique = false;
        for e in entries {
            if e.get("rank").and_then(Json::as_f64) == Some(0.0) {
                all_unique |= e.get("value").and_then(Json::as_str) == Some("<ALL_UNIQUE>");
                continue;
            }
            if let Some(v) = e.get("value").and_then(Json::as_str) {
                push(v);
            }
        }
        if all_unique {
            for v in sample_text_values(c) {
                push(&v);
            }
        }
    }
    for c in &string_cols {
        if let Some(s) = col_stats(stats, c) {
            for k in ["min", "max"] {
                if let Some(v) = s.get(k).and_then(Json::as_str) {
                    push(v);
                }
            }
        }
    }
    sample
}

/// Up to 20 non-empty values of `field`, in file order, via the SQL
/// backend (empty when none is installed).
#[cfg(feature = "whatlang")]
fn sample_text_values(field: &str) -> Vec<String> {
    if field.contains('"') {
        return Vec::new();
    }
    let key = format!("text_sample\u{1f}{field}");
    memo_sql(&key, |backend| {
        let sql = format!(
            r#"SELECT CAST("{field}" AS VARCHAR) AS v FROM data WHERE "{field}" IS NOT NULL LIMIT 20"#
        );
        let df = backend.query(&sql).ok()?;
        let col = df.column("v").ok()?.as_series()?.str().ok()?.clone();
        Some(Json::from(
            col.iter()
                .flatten()
                .map(str::to_string)
                .collect::<Vec<String>>(),
        ))
    })
    .and_then(|v| serde_json::from_value(v).ok())
    .unwrap_or_default()
}

/// whatlang's ISO 639-3 codes → ISO 639-1. Every language whatlang
/// detects has a 639-1 code; `None` guards against future additions.
#[cfg(feature = "whatlang")]
fn iso_639_3_to_1(code: &str) -> Option<&'static str> {
    Some(match code {
        "afr" => "af",
        "aka" => "ak",
        "amh" => "am",
        "ara" => "ar",
        "aze" => "az",
        "bel" => "be",
        "ben" => "bn",
        "bul" => "bg",
        "cat" => "ca",
        "ces" => "cs",
        "cmn" => "zh",
        "cym" => "cy",
        "dan" => "da",
        "deu" => "de",
        "ell" => "el",
        "eng" => "en",
        "epo" => "eo",
        "est" => "et",
        "fin" => "fi",
        "fra" => "fr",
        "guj" => "gu",
        "heb" => "he",
        "hin" => "hi",
        "hrv" => "hr",
        "hun" => "hu",
        "hye" => "hy",
        "ind" => "id",
        "ita" => "it",
        "jav" => "jv",
        "jpn" => "ja",
        "kan" => "kn",
        "kat" => "ka",
        "khm" => "km",
        "kor" => "ko",
        "lat" => "la",
        "lav" => "lv",
        "lit" => "lt",
        "mal" => "ml",
        "mar" => "mr",
        "mkd" => "mk",
        "mya" => "my",
        "nep" => "ne",
        "nld" => "nl",
        "nob" => "nb",
        "ori" => "or",
        "pan" => "pa",
        "pes" => "fa",
        "pol" => "pl",
        "por" => "pt",
        "ron" => "ro",
        "rus" => "ru",
        "sin" => "si",
        "slk" => "sk",
        "slv" => "sl",
        "sna" => "sn",
        "spa" => "es",
        "srp" => "sr",
        "swe" => "sv",
        "tam" => "ta",
        "tel" => "te",
        "tgl" => "tl",
        "tha" => "th",
        "tuk" => "tk",
        "tur" => "tr",
        "ukr" => "uk",
        "urd" => "ur",
        "uzb" => "uz",
        "vie" => "vi",
        "yid" => "yi",
        "zul" => "zu",
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sig_figs_rounds_without_scientific_notation() {
        assert_eq!(sig_figs(11.132, 3), "11.1");
        assert_eq!(sig_figs(1113.2, 3), "1110");
        assert_eq!(sig_figs(0.001_113_2, 3), "0.00111");
        assert_eq!(sig_figs(111_320.0, 3), "111000");
    }

    #[test]
    fn xsd_duration_formats_exactly() {
        assert_eq!(format_xsd_duration_ms(60_000), "PT1M");
        assert_eq!(format_xsd_duration_ms(15 * 60_000), "PT15M");
        assert_eq!(format_xsd_duration_ms(3_600_000), "PT1H");
        assert_eq!(format_xsd_duration_ms(86_400_000), "P1D");
        assert_eq!(format_xsd_duration_ms(7 * 86_400_000), "P7D");
        assert_eq!(format_xsd_duration_ms(1_000), "PT1S");
        assert_eq!(format_xsd_duration_ms(1_500), "PT1.5S");
        assert_eq!(format_xsd_duration_ms(250), "PT0.25S");
    }

    #[test]
    fn recording_precision_is_the_finest_unit_used() {
        let p = |v: &[&str]| {
            let dts: Vec<NaiveDateTime> = v
                .iter()
                .map(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f").unwrap())
                .collect();
            recording_precision(&dts)
        };
        assert_eq!(p(&["2024-01-01 00:00:00", "2024-01-05 00:00:00"]), "P1D");
        assert_eq!(p(&["2024-01-01 08:00:00", "2024-01-01 13:00:00"]), "PT1H");
        assert_eq!(p(&["2024-01-01 08:15:00", "2024-01-01 08:57:00"]), "PT1M");
        assert_eq!(p(&["2024-01-01 08:15:00", "2024-01-01 08:15:42"]), "PT1S");
        assert_eq!(
            p(&["2024-01-01 08:15:00.250", "2024-01-01 08:15:42"]),
            "PT0.001S"
        );
    }

    #[test]
    fn calendar_months_requires_same_day_and_time() {
        let d = |s: &str| {
            NaiveDateTime::parse_from_str(&format!("{s} 00:00:00"), "%Y-%m-%d %H:%M:%S").unwrap()
        };
        assert_eq!(calendar_months(d("2024-01-15"), d("2024-04-15")), Some(3));
        assert_eq!(calendar_months(d("2023-12-01"), d("2024-01-01")), Some(1));
        assert_eq!(calendar_months(d("2024-01-15"), d("2024-02-14")), None);
    }

    fn stats_fixture() -> Json {
        json!({
            "lat":    {"stats": {"type": "Float", "min": "42.2553", "max": "42.3806",
                                 "mean": 42.3367, "max_precision": 4, "nullcount": 0,
                                 "n_zero": 0, "cardinality": 78, "uniqueness_ratio": 0.78}},
            "lon":    {"stats": {"type": "Float", "min": "-71.1626", "max": "-71.0298",
                                 "mean": -71.07, "max_precision": 4, "nullcount": 0,
                                 "n_zero": 0, "cardinality": 77, "uniqueness_ratio": 0.77}},
            "id":     {"stats": {"type": "Integer", "min": "1", "max": "4", "nullcount": 0,
                                 "cardinality": 4, "uniqueness_ratio": 1.0}},
            "status": {"stats": {"type": "String", "min": "Closed", "max": "Open",
                                 "is_ascii": true,
                                 "min_length": 4, "max_length": 6, "nullcount": 1,
                                 "cardinality": 3, "uniqueness_ratio": 0.75}},
            "photo":  {"stats": {"type": "NULL", "nullcount": 4, "cardinality": 1}},
        })
    }

    #[test]
    fn coordinate_resolution_uses_coarser_axis() {
        let dpp = json!({"LAT_FIELD": "lat", "LON_FIELD": "lon"});
        let v = coordinate_resolution_in_meters(
            Value::from(Serde(&dpp)),
            Value::from(Serde(&stats_fixture())),
        );
        // 4 dp: 11.132 m N-S vs 11.132*cos(42.34°)=8.23 m E-W -> 11.1.
        assert_eq!(v.as_str(), Some("11.1"));
        let none = coordinate_resolution_in_meters(
            Value::from(Serde(json!({}))),
            Value::from(Serde(&stats_fixture())),
        );
        assert!(none.is_undefined());
    }

    #[test]
    fn robust_bbox_fast_path_uses_stats_without_backend() {
        formula_helpers::set_sql_backend(None);
        let dpp = json!({"LAT_FIELD": "lat", "LON_FIELD": "lon"});
        let v = robust_bbox(
            Value::from(Serde(&dpp)),
            Value::from(Serde(&stats_fixture())),
        );
        let j = to_json(&v);
        assert_eq!(
            j[0]["bbox"],
            "POLYGON((-71.1626 42.2553, -71.1626 42.3806, -71.0298 42.3806, -71.0298 42.2553, \
             -71.1626 42.2553))"
        );
    }

    #[test]
    fn quality_measurements_completeness_and_validity() {
        let dpp = json!({"RECORD_COUNT": 4});
        let dims = json!({
            "completeness": "http://www.w3.org/2016/05/ldqd#completeness",
            "syntacticValidity": "http://www.w3.org/2016/05/ldqd#syntacticValidity",
        });
        formula_helpers::set_sql_backend(None);
        let v = quality_measurements(
            Value::from(Serde(&stats_fixture())),
            Value::from(Serde(&dpp)),
            Value::from(Serde(&dims)),
            Value::from(Serde(json!({"valid": true}))),
        );
        let j = to_json(&v);
        let arr = j.as_array().unwrap();
        // 5 cols x 4 rows = 20 cells, 5 empty -> 75.00%.
        assert_eq!(arr[0]["value"], "75.00");
        assert_eq!(arr[0]["unitMeasure"], "percent");
        assert_eq!(arr[1]["value"], "1"); // photo is all-empty
        assert_eq!(arr[2]["value"], "2"); // status + photo
        // conciseness omitted: no IRI given (and no backend)
        assert_eq!(arr.len(), 4);
        assert_eq!(arr[3]["value"], "true");
        assert_eq!(
            arr[3]["isMeasurementOf"]["inDimension"],
            "http://www.w3.org/2016/05/ldqd#syntacticValidity"
        );
    }

    #[test]
    fn csvw_rich_constraints_enum_and_primary_key() {
        let freq = json!({
            "status": [
                {"value": "Open", "count": 2, "percentage": 50.0, "rank": 1.0},
                {"value": "Closed", "count": 1, "percentage": 25.0, "rank": 2.0},
            ],
        });
        let headers = json!(["status", "id", "lat", "lon", "photo"]);
        let v = build_csvw_schema_rich(
            Value::from(Serde(&stats_fixture())),
            Value::from(Serde(&freq)),
            Some(Value::from(Serde(&headers))),
            Some(Value::from(4)),
            None,
        );
        let j = to_json(&v);
        assert_eq!(j["primaryKey"], "id");
        let names: Vec<&str> = j["columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["status", "id", "lat", "lon", "photo"]);
        let col = |n: &str| {
            j["columns"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == n)
                .unwrap()
                .clone()
        };
        let status = col("status");
        assert_eq!(status["datatype"]["format"], "^(?:Closed|Open)$");
        assert_eq!(status["datatype"]["minLength"], 4);
        assert!(status.get("required").is_none());
        let id = col("id");
        assert_eq!(id["required"], true);
        assert_eq!(id["datatype"]["minimum"], 1);
        assert_eq!(id["datatype"]["maximum"], 4);
        assert!(col("photo").get("required").is_none());
    }

    #[test]
    fn csvw_rich_primary_key_needs_exact_cardinality_and_ascii_lengths() {
        let stats = json!({
            // 99,999 distinct of 100,000 rows: uniqueness_ratio rounds to 1.0.
            "id":   {"stats": {"type": "Integer", "nullcount": 0, "cardinality": 99_999,
                               "uniqueness_ratio": 1.0}},
            "word": {"stats": {"type": "String", "is_ascii": false, "nullcount": 0,
                               "min_length": 5, "max_length": 5, "cardinality": 1}},
        });
        let v = build_csvw_schema_rich(
            Value::from(Serde(&stats)),
            Value::from(Serde(json!({}))),
            None,
            Some(Value::from(100_000)),
            None,
        );
        let j = to_json(&v);
        assert!(j.get("primaryKey").is_none(), "{j}");
        let word = j["columns"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["name"] == "word")
            .unwrap();
        assert!(word["datatype"].get("minLength").is_none(), "{word}");
    }

    #[test]
    fn enumeration_regex_declines_partial_capture() {
        let freq = json!({"status": [{"value": "Open", "count": 2, "rank": 1.0}]});
        assert_eq!(enumeration_regex(&freq, "status", Some(3), 1), None);
        assert_eq!(enumeration_regex(&freq, "status", Some(20), 0), None);
    }

    #[cfg(feature = "whatlang")]
    #[test]
    fn language_sample_counts_each_value_once() {
        let stats = json!({"station": {"stats": {"type": "String", "min": "Central Park",
                                                 "max": "Central Park"}}});
        let freq = json!({"station": [{"value": "Central Park", "count": 30, "rank": 1.0}]});
        assert_eq!(language_sample(&stats, &freq), "Central Park");
        let v = detect_data_language(Value::from(Serde(&stats)), Value::from(Serde(&freq)));
        assert!(
            v.is_undefined(),
            "repeated proper noun must not be detected as a language"
        );
    }

    #[test]
    fn notes_are_deduplicated_in_order() {
        let _ = drain_notes();
        push_note("b".into());
        push_note("a".into());
        push_note("b".into());
        assert_eq!(drain_notes(), vec!["b".to_string(), "a".to_string()]);
        assert!(drain_notes().is_empty());
    }

    #[test]
    fn ckan_groups_become_labelled_concepts_in_the_scheme() {
        let groups = Value::from(Serde(json!([
            { "name": "transportation", "title": "Transportation",
              "display_name": "Transport", "description": " Roads and transit " },
            { "name": "health", "title": "", "display_name": "Health" },
            { "name": "parks", "description": "" },
            "Environment",
            { "title": "  " },
            "",
        ])));
        let scheme = json!({ "@type": "ConceptScheme", "title": "CKAN groups" });
        let got = to_json(&ckan_group_concepts(
            groups,
            Value::from(Serde(scheme.clone())),
        ));
        assert_eq!(
            got,
            json!([
                { "@type": "Concept", "prefLabel": "Transportation",
                  "definition": "Roads and transit", "inScheme": scheme },
                { "@type": "Concept", "prefLabel": "Health", "inScheme": scheme },
                { "@type": "Concept", "prefLabel": "parks", "inScheme": scheme },
                { "@type": "Concept", "prefLabel": "Environment", "inScheme": scheme },
            ])
        );
    }

    #[test]
    fn ckan_group_scheme_names_the_portal_from_a_ckan_url() {
        let scheme = |urls: Json| {
            to_json(&ckan_group_scheme(
                Value::from(Serde(urls)),
                Value::from("CKAN groups"),
                Value::from("{portal} groups"),
            ))
        };
        // an uploaded resource's URL is on the portal
        assert_eq!(
            scheme(json!([
                "https://data.wprdc.org/dataset/311/resource/76fd/download/311.csv",
                "local.csv"
            ])),
            json!({ "@type": "ConceptScheme", "@id": "https://data.wprdc.org/group/",
                    "title": "data.wprdc.org groups" })
        );
        // a CKAN under a sub-path; the resource URL is undefined, the input is the dataset
        assert_eq!(
            scheme(json!([null, "https://example.org/ckan/dataset/roads"])),
            json!({ "@type": "ConceptScheme", "@id": "https://example.org/ckan/group/",
                    "title": "example.org/ckan groups" })
        );
        // a linked resource may live off the portal: no /dataset/ path, so not used
        assert_eq!(
            scheme(json!(["https://files.example.com/roads.csv", "roads.csv"])),
            json!({ "@type": "ConceptScheme", "title": "CKAN groups" })
        );
    }

    #[test]
    fn ckan_groups_without_labels_are_undefined_and_no_scheme_means_no_in_scheme() {
        let none = Value::from(Serde(json!([{ "title": "" }, ""])));
        let scheme = Value::from(Serde(json!({ "@type": "ConceptScheme", "title": "G" })));
        assert!(ckan_group_concepts(none, scheme).is_undefined());
        let one = Value::from(Serde(json!([{ "name": "x" }])));
        assert_eq!(
            to_json(&ckan_group_concepts(one, Value::UNDEFINED)),
            json!([{ "@type": "Concept", "prefLabel": "x" }])
        );
        // no portal and no fallback title: no scheme at all
        let urls = Value::from(Serde(json!(["roads.csv"])));
        assert!(
            ckan_group_scheme(urls, Value::UNDEFINED, Value::from("{portal} groups"))
                .is_undefined()
        );
        // an emptied {portal} pattern falls back to the plain title
        let urls = Value::from(Serde(json!(["https://d.example/dataset/x"])));
        assert_eq!(
            to_json(&ckan_group_scheme(
                urls,
                Value::from("CKAN groups"),
                Value::from(" ")
            )),
            json!({ "@type": "ConceptScheme", "title": "CKAN groups" })
        );
    }
}
