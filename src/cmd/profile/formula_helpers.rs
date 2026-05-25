//! Rust port of DP+'s `jinja2_helpers.py` for `qsv profile`'s formula
//! engine.
//!
//! Each public registration function takes a `minijinja::Environment` and
//! adds every filter / global DP+ exposes to scheming-YAML formulas. The
//! helpers mirror DP+ semantics byte-for-byte where possible so a formula
//! that worked in DP+'s `FormulaProcessor` keeps working under qsv profile.
//!
//! Helpers are grouped into:
//!
//!   * filters     — applied via the `|` pipe
//!   * globals     — invoked by name
//!   * SQL globals — currently short-circuit to an error (matches the old Python stub behavior of
//!     returning empty datastore results); Phase 0c will wire them to Polars SQL over the input
//!     CSV.
//!
//! Default field-name candidate lists for lat/lon detection come from
//! the old `qsv_ckan_stubs.py` defaults and `ckanext.datapusher_plus.config`
//! upstream defaults.

use std::cell::RefCell;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use minijinja::{Environment, Error, ErrorKind, Value};

use super::sql_backend::SqlBackend;

thread_local! {
    /// Per-thread handle to the SQL backend, set by `evaluate_spec`
    /// before rendering and cleared after. Thread-local (not static
    /// RwLock) so concurrent tests / future parallel-profile runs can't
    /// race each other when setting and clearing the backend.
    static SQL_BACKEND: RefCell<Option<SqlBackend>> = const { RefCell::new(None) };
}

/// Install the SQL backend on the current thread for the duration of
/// an `evaluate_spec` call. Pass `None` to clear.
pub fn set_sql_backend(backend: Option<SqlBackend>) {
    SQL_BACKEND.with(|cell| *cell.borrow_mut() = backend);
}

/// Borrow the installed SQL backend on the current thread, if any.
fn with_sql_backend<R>(f: impl FnOnce(&SqlBackend) -> Result<R, Error>) -> Result<R, Error> {
    SQL_BACKEND.with(|cell| {
        let borrowed = cell.borrow();
        let backend = borrowed.as_ref().ok_or_else(|| {
            value_err(
                "no input CSV available for SQL-backed helper (stdin input or backend not \
                 installed)",
            )
        })?;
        f(backend)
    })
}

/// Register every filter + global on the environment. Called once per
/// `evaluate_spec` invocation by `formula_engine::evaluate_spec`.
pub fn register(env: &mut Environment) {
    // --- filters ------------------------------------------------------
    env.add_filter("truncate_with_ellipsis", truncate_with_ellipsis);
    env.add_filter("format_number", format_number);
    env.add_filter("format_bytes", format_bytes);
    env.add_filter("format_date", format_date);
    env.add_filter("calculate_percentage", calculate_percentage);
    env.add_filter("format_range", format_range);
    env.add_filter("format_coordinates", format_coordinates);

    // --- globals (pure / non-SQL) -------------------------------------
    env.add_function("calculate_bbox_area", calculate_bbox_area);
    env.add_function("spatial_extent_wkt", spatial_extent_wkt);
    env.add_function(
        "spatial_extent_feature_collection",
        spatial_extent_feature_collection,
    );
    env.add_function("get_frequency_top_values", get_frequency_top_values);
    env.add_function("map_tags_to_themes", map_tags_to_themes);
    env.add_function("spatial_resolution_in_meters", spatial_resolution_in_meters);
    env.add_function("get_column_null_percentage", get_column_null_percentage);
    env.add_function("get_column_stats", get_column_stats);

    // --- globals (SQL-backed) -----------------------------------------
    // Backed by Polars SQL over the input CSV (see sql_backend.rs).
    // The backend is set process-wide by `formula_engine::evaluate_spec`
    // for the duration of the render pass, then cleared.
    env.add_function("temporal_resolution", temporal_resolution);
    env.add_function("guess_accrual_periodicity", guess_accrual_periodicity);
}

// =====================================================================
// FILTERS
// =====================================================================

/// Truncate text to `length`, appending `ellipsis` if truncation occurred.
/// `{{ package.description | truncate_with_ellipsis(10) }}` → "Hello, wo..."
fn truncate_with_ellipsis(text: Value, length: Option<usize>, ellipsis: Option<String>) -> Value {
    let Some(s) = text.as_str() else {
        return text;
    };
    let len = length.unwrap_or(50);
    let ell = ellipsis.unwrap_or_else(|| "...".to_string());
    if s.chars().count() <= len {
        return text;
    }
    let truncated: String = s.chars().take(len).collect();
    Value::from(format!("{truncated}{ell}"))
}

/// Format numbers with thousands separator and decimal places.
/// `{{ value | format_number }}` → "1,234,567.89"
fn format_number(value: Value, decimals: Option<usize>) -> Value {
    let Some(n) = value_to_f64(&value) else {
        return value; // pass through if not numeric
    };
    let d = decimals.unwrap_or(2);
    Value::from(format_thousands(n, d))
}

/// Format byte sizes into human-readable form.
/// `{{ size | format_bytes }}` → "1.5 GB"
fn format_bytes(value: Value) -> Value {
    let Some(mut n) = value_to_f64(&value) else {
        return value;
    };
    let units = ["B", "KB", "MB", "GB", "TB"];
    for unit in units {
        if n < 1024.0 {
            return Value::from(format!("{n:.1} {unit}"));
        }
        n /= 1024.0;
    }
    Value::from(format!("{n:.1} PB"))
}

/// Format dates in a specified strftime-style format. Currently supports
/// the most common DP+ formats; falls back to the original string on any
/// parse / format failure. `format` defaults to `%Y-%m-%d`.
fn format_date(value: Value, format: Option<String>) -> Value {
    if value.is_none() || value.is_undefined() {
        return Value::from_serialize(&Option::<String>::None);
    }
    let Some(s) = value.as_str() else {
        return value;
    };
    let fmt = format.unwrap_or_else(|| "%Y-%m-%d".to_string());
    // Try a few common ISO 8601 shapes; fall back to original on failure.
    use chrono::{DateTime, NaiveDate, NaiveDateTime};
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Value::from(dt.format(&fmt).to_string());
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Value::from(ndt.format(&fmt).to_string());
    }
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Value::from(nd.format(&fmt).to_string());
    }
    value
}

/// Calculate percentage. Raises an error if `whole` is zero or inputs
/// aren't numeric — mirrors DP+'s ValueError behavior.
fn calculate_percentage(part: Value, whole: Value) -> Result<f64, Error> {
    let p = value_to_f64(&part).ok_or_else(|| value_err("Error calculating percentage"))?;
    let w = value_to_f64(&whole).ok_or_else(|| value_err("Error calculating percentage"))?;
    if w == 0.0 {
        return Err(value_err("Whole value is zero"));
    }
    Ok((p / w) * 100.0)
}

/// Format a range of values: `format_range(min, max, separator=" to ")`
fn format_range(min_val: Value, max_val: Value, separator: Option<String>) -> Value {
    let sep = separator.unwrap_or_else(|| " to ".to_string());
    Value::from(format!("{min_val}{sep}{max_val}"))
}

/// Format coordinates as `40.712800°N, 74.006000°W`.
fn format_coordinates(lat: Value, lon: Value, precision: Option<usize>) -> Result<String, Error> {
    let lat_f =
        value_to_f64(&lat).ok_or_else(|| value_err("format_coordinates: lat must be numeric"))?;
    let lon_f =
        value_to_f64(&lon).ok_or_else(|| value_err("format_coordinates: lon must be numeric"))?;
    let p = precision.unwrap_or(6);
    let lat_dir = if lat_f >= 0.0 { 'N' } else { 'S' };
    let lon_dir = if lon_f >= 0.0 { 'E' } else { 'W' };
    Ok(format!(
        "{:.*}°{}, {:.*}°{}",
        p,
        lat_f.abs(),
        lat_dir,
        p,
        lon_f.abs(),
        lon_dir
    ))
}

// =====================================================================
// GLOBALS — pure / non-SQL
// =====================================================================

/// Approximate area of a bounding box in square kilometers.
/// Args optional — falls back to context-derived bbox.
fn calculate_bbox_area(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<f64, Error> {
    let coords = if args.len() == 4 {
        BBoxCoords {
            min_lon: value_to_f64(&args[0])
                .ok_or_else(|| value_err("calculate_bbox_area: min_lon not numeric"))?,
            min_lat: value_to_f64(&args[1])
                .ok_or_else(|| value_err("calculate_bbox_area: min_lat not numeric"))?,
            max_lon: value_to_f64(&args[2])
                .ok_or_else(|| value_err("calculate_bbox_area: max_lon not numeric"))?,
            max_lat: value_to_f64(&args[3])
                .ok_or_else(|| value_err("calculate_bbox_area: max_lat not numeric"))?,
        }
    } else {
        bbox_from_context(state)?
    };
    let earth_radius = 6371_f64; // km
    let mid_lat = (coords.min_lat + coords.max_lat) / 2.0;
    let width = (coords.max_lon - coords.min_lon).abs() * std::f64::consts::PI / 180.0
        * mid_lat.to_radians().cos();
    let height = (coords.max_lat - coords.min_lat).abs() * std::f64::consts::PI / 180.0;
    Ok(width * height * earth_radius * earth_radius)
}

/// Convert min/max WGS84 coordinates to a WKT polygon. Falls back to
/// the context's inferred lat/lon fields if no args provided.
///
/// Matches DP+'s exact format string including the `SRID=4326;` prefix,
/// since downstream CKAN consumers parse on that literal.
fn spatial_extent_wkt(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<String, Error> {
    let coords = if args.len() == 4 {
        BBoxCoords {
            min_lon: value_to_f64(&args[0])
                .ok_or_else(|| value_err("spatial_extent_wkt: min_lon not numeric"))?,
            min_lat: value_to_f64(&args[1])
                .ok_or_else(|| value_err("spatial_extent_wkt: min_lat not numeric"))?,
            max_lon: value_to_f64(&args[2])
                .ok_or_else(|| value_err("spatial_extent_wkt: max_lon not numeric"))?,
            max_lat: value_to_f64(&args[3])
                .ok_or_else(|| value_err("spatial_extent_wkt: max_lat not numeric"))?,
        }
    } else {
        bbox_from_context(state)?
    };
    let BBoxCoords {
        min_lon,
        min_lat,
        max_lon,
        max_lat,
    } = coords;
    Ok(format!(
        "SRID=4326;POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, \
         {max_lon} {min_lat}, {min_lon} {min_lat}))"
    ))
}

/// Convert a bounding box to a named GeoJSON FeatureCollection string.
fn spatial_extent_feature_collection(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<String, Error> {
    // Positional: (name, bbox, feature_type)
    let mut name = "Inferred Spatial Extent".to_string();
    let mut bbox: Option<[f64; 4]> = None;
    let mut feature_type = "inferred".to_string();

    if let Some(v) = args.first()
        && let Some(s) = v.as_str()
    {
        name = s.to_string();
    }
    if let Some(v) = args.get(1)
        && !v.is_none()
        && !v.is_undefined()
    {
        let arr: Vec<f64> = deserialize_value(v).ok_or_else(|| {
            value_err("spatial_extent_feature_collection: bbox must be a list of 4 floats")
        })?;
        if arr.len() != 4 {
            return Err(value_err("Invalid bounding box"));
        }
        bbox = Some([arr[0], arr[1], arr[2], arr[3]]);
        feature_type = "calculated".to_string();
    }
    if let Some(v) = args.get(2)
        && let Some(s) = v.as_str()
    {
        feature_type = s.to_string();
    }

    let coords = match bbox {
        Some(b) => BBoxCoords {
            min_lon: b[0],
            min_lat: b[1],
            max_lon: b[2],
            max_lat: b[3],
        },
        None => bbox_from_context(state)?,
    };
    let BBoxCoords {
        min_lon,
        min_lat,
        max_lon,
        max_lat,
    } = coords;
    Ok(format!(
        r#"{{"type": "FeatureCollection", "features": [{{"type": "Feature", "properties": {{"name": "{name}", "type": "{feature_type}"}}, "geometry": {{"type": "Polygon", "coordinates": [[[{min_lon},{min_lat}], [{min_lon},{max_lat}], [{max_lon},{max_lat}], [{max_lon},{min_lat}], [{min_lon},{min_lat}]]]}}}}]}}"#
    ))
}

/// Get the top values for a field from frequency data (`dppf`).
fn get_frequency_top_values(field: String, state: &minijinja::State) -> Result<Value, Error> {
    let dppf = state
        .lookup("dppf")
        .ok_or_else(|| value_err("No frequency data found"))?;
    if dppf.is_none() || dppf.is_undefined() {
        return Err(value_err("No frequency data found"));
    }
    let entry = dppf
        .get_attr(&field)
        .map_err(|_| value_err("Field not found in frequency data"))?;
    if entry.is_undefined() {
        return Err(value_err("Field not found in frequency data"));
    }
    Ok(entry)
}

/// Map CKAN tags to DCAT theme URIs via a small static lookup. Matches
/// the upstream DP+ stub mapping; future expansion will load from a
/// remote/reference resource.
fn map_tags_to_themes(state: &minijinja::State) -> Result<Value, Error> {
    let package = state.lookup("package").unwrap_or(Value::UNDEFINED);
    let tags_v = package.get_attr("tags").unwrap_or(Value::UNDEFINED);

    let tags: Vec<String> = if let Some(s) = tags_v.as_str() {
        s.split(',').map(|t| t.trim().to_string()).collect()
    } else if tags_v.is_undefined() || tags_v.is_none() {
        Vec::new()
    } else {
        match deserialize_value::<Vec<serde_json::Value>>(&tags_v) {
            Some(arr) => arr
                .into_iter()
                .filter_map(|v| match v {
                    serde_json::Value::String(s) => Some(s),
                    serde_json::Value::Object(o) => {
                        o.get("name").and_then(|n| n.as_str().map(str::to_string))
                    },
                    _ => None,
                })
                .collect(),
            None => Vec::new(),
        }
    };

    let themes: Vec<&'static str> = tags
        .iter()
        .filter_map(|t| match t.to_lowercase().as_str() {
            "climate" => Some("https://data.gov/themes/climate"),
            "health" => Some("https://data.gov/themes/health"),
            "transportation" => Some("https://data.gov/themes/transportation"),
            _ => None,
        })
        .collect();

    if themes.is_empty() {
        Ok(Value::from_serialize(&Option::<Vec<String>>::None))
    } else {
        Ok(Value::from_serialize(&themes))
    }
}

/// Diagonal of a bounding box in meters via the Haversine formula.
fn spatial_resolution_in_meters(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<f64, Error> {
    // DP+ signature: bbox=(min_lat, max_lat, min_lon, max_lon)
    let (min_lat, max_lat, min_lon, max_lon) = if let Some(first) = args.first()
        && !first.is_none()
        && !first.is_undefined()
    {
        let arr: Vec<f64> = deserialize_value(first).ok_or_else(|| {
            value_err("spatial_resolution_in_meters: bbox must be a list of 4 floats")
        })?;
        if arr.len() != 4 {
            return Err(value_err("bbox must have 4 elements"));
        }
        (arr[0], arr[1], arr[2], arr[3])
    } else {
        let c = bbox_from_context(state)?;
        (c.min_lat, c.max_lat, c.min_lon, c.max_lon)
    };

    let radius_m = 6_371_000_f64;
    let phi1 = min_lat.to_radians();
    let phi2 = max_lat.to_radians();
    let dphi = (max_lat - min_lat).to_radians();
    let dlambda = (max_lon - min_lon).to_radians();
    let a = (dphi / 2.0).sin().powi(2) + phi1.cos() * phi2.cos() * (dlambda / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    Ok(radius_m * c)
}

/// Compute the percentage of nulls in a field, reading from `dpps[col].stats.nullcount`
/// and `dpp.RECORD_COUNT`.
fn get_column_null_percentage(column_name: String, state: &minijinja::State) -> Result<f64, Error> {
    if column_name.is_empty() {
        return Err(value_err("Column name is required"));
    }
    let dpps = state.lookup("dpps").unwrap_or(Value::UNDEFINED);
    let field_stats = dpps
        .get_attr(&column_name)
        .unwrap_or(Value::UNDEFINED)
        .get_attr("stats")
        .unwrap_or(Value::UNDEFINED);
    let nullcount = field_stats
        .get_attr("nullcount")
        .ok()
        .and_then(|v| value_to_f64(&v))
        .unwrap_or(0.0);
    let dpp = state.lookup("dpp").unwrap_or(Value::UNDEFINED);
    let record_count = dpp
        .get_attr("RECORD_COUNT")
        .ok()
        .and_then(|v| value_to_f64(&v))
        .unwrap_or(0.0);
    if record_count == 0.0 {
        Ok(0.0)
    } else {
        Ok((nullcount / record_count) * 100.0)
    }
}

/// Get statistics for a column.
/// `get_column_stats(col)` → full stats dict
/// `get_column_stats(col, "min")` → single stat value
/// `get_column_stats(col, ["min", "max"])` → dict of requested stats
fn get_column_stats(
    column_name: String,
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<Value, Error> {
    if column_name.is_empty() {
        return Err(value_err("Column name is required"));
    }
    let dpps = state.lookup("dpps").unwrap_or(Value::UNDEFINED);
    let field_stats = dpps
        .get_attr(&column_name)
        .unwrap_or(Value::UNDEFINED)
        .get_attr("stats")
        .unwrap_or(Value::UNDEFINED);
    if field_stats.is_undefined() || field_stats.is_none() {
        return Err(value_err(&format!(
            "No stats found for column {column_name}"
        )));
    }
    let Some(stat_arg) = args.first() else {
        return Ok(field_stats);
    };
    if let Some(name) = stat_arg.as_str() {
        return Ok(field_stats.get_attr(name).unwrap_or(Value::from(0)));
    }
    // try list-of-stat-names
    if let Some(names) = deserialize_value::<Vec<String>>(stat_arg) {
        let mut out = std::collections::BTreeMap::<String, Value>::new();
        for n in names {
            let v = field_stats.get_attr(&n).unwrap_or(Value::from(0));
            out.insert(n, v);
        }
        return Ok(Value::from_serialize(&out));
    }
    Ok(field_stats)
}

// =====================================================================
// GLOBALS — SQL-backed (Polars over the input CSV)
// =====================================================================

/// `temporal_resolution(date_field?)` — minimum interval between
/// consecutive sorted unique values of `date_field` in the input CSV,
/// expressed as an ISO 8601 duration. Mirrors DP+'s thresholds:
///     < 1 day  → "PT1H"
///     == 1 day → "P1D"
///     <= 31 d  → "P{n}D"
///     <= 366 d → "P{n//30}M"
///     else     → "P{n//365}Y"
/// If `date_field` is omitted, falls back to the first entry in
/// `dpp.DATE_FIELDS` then `dpp.DATETIME_FIELDS`.
fn temporal_resolution(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<String, Error> {
    let field = resolve_date_field(args.first(), state)?;
    with_sql_backend(|backend| {
        let strs = backend
            .distinct_sorted_date_strings(&field)
            .map_err(|e| value_err(&format!("temporal_resolution: {e}")))?;
        if strs.len() < 2 {
            return Err(value_err("Not enough records to get temporal resolution"));
        }
        let dates = parse_date_strings(&strs)?;
        let intervals = day_intervals(&dates);
        let min_days = *intervals
            .iter()
            .min()
            .ok_or_else(|| value_err("No intervals found"))?;
        Ok(format_temporal_resolution(min_days))
    })
}

/// `guess_accrual_periodicity(date_field?)` — most common interval
/// between consecutive sorted unique date values, expressed as an
/// ISO 8601 repeating duration: `R/P{n}{unit}`.
fn guess_accrual_periodicity(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<String, Error> {
    let field = resolve_date_field(args.first(), state)?;
    with_sql_backend(|backend| {
        let strs = backend
            .distinct_sorted_date_strings(&field)
            .map_err(|e| value_err(&format!("guess_accrual_periodicity: {e}")))?;
        if strs.len() < 2 {
            return Err(value_err("Not enough records to guess accrual periodicity"));
        }
        let dates = parse_date_strings(&strs)?;
        let intervals = day_intervals(&dates);
        let most_common = mode(&intervals).ok_or_else(|| value_err("No intervals found"))?;
        Ok(format_accrual_periodicity(most_common))
    })
}

/// Resolve a date-field name from an optional first-positional arg,
/// falling back to the first entry of `dpp.DATE_FIELDS` then
/// `dpp.DATETIME_FIELDS` in the template context.
fn resolve_date_field(arg: Option<&Value>, state: &minijinja::State) -> Result<String, Error> {
    if let Some(v) = arg
        && let Some(s) = v.as_str()
        && !s.is_empty()
    {
        return Ok(s.to_string());
    }
    let dpp = state.lookup("dpp").unwrap_or(Value::UNDEFINED);
    for key in ["DATE_FIELDS", "DATETIME_FIELDS"] {
        let arr = dpp.get_attr(key).unwrap_or(Value::UNDEFINED);
        if let Some(name) =
            deserialize_value::<Vec<String>>(&arr).and_then(|v| v.into_iter().next())
        {
            return Ok(name);
        }
    }
    Err(value_err("No date or datetime fields found"))
}

/// Parse ISO 8601 date / datetime / RFC3339 strings into NaiveDateTime
/// for interval math. Sub-second precision is preserved; date-only
/// strings get midnight as the time component.
fn parse_date_strings(strs: &[String]) -> Result<Vec<NaiveDateTime>, Error> {
    let mut out = Vec::with_capacity(strs.len());
    for s in strs {
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
            out.push(dt);
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
            out.push(dt);
        } else if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            out.push(d.and_hms_opt(0, 0, 0).unwrap());
        } else if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            out.push(dt.naive_utc());
        } else {
            return Err(value_err(&format!("could not parse date: {s}")));
        }
    }
    Ok(out)
}

/// Whole-day intervals between consecutive (already-sorted) datetimes.
/// Matches DP+'s `(dates[i+1] - dates[i]).days` semantics — sub-day
/// differences truncate to zero, surfaced as "PT1H" by the formatter.
fn day_intervals(dates: &[NaiveDateTime]) -> Vec<i64> {
    let mut out = Vec::with_capacity(dates.len().saturating_sub(1));
    for w in dates.windows(2) {
        out.push((w[1] - w[0]).num_days());
    }
    out
}

/// Most-frequent value (with ties broken by first-seen order).
fn mode(values: &[i64]) -> Option<i64> {
    use std::collections::HashMap;
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, c)| c).map(|(v, _)| v)
}

fn format_temporal_resolution(min_days: i64) -> String {
    if min_days < 1 {
        "PT1H".to_string()
    } else if min_days == 1 {
        "P1D".to_string()
    } else if min_days <= 31 {
        format!("P{min_days}D")
    } else if min_days <= 366 {
        format!("P{}M", min_days / 30)
    } else {
        format!("P{}Y", min_days / 365)
    }
}

fn format_accrual_periodicity(most_common: i64) -> String {
    if most_common == 1 {
        "R/P1D".to_string()
    } else if most_common <= 31 {
        format!("R/P{most_common}D")
    } else if most_common <= 366 {
        format!("R/P{}M", most_common / 30)
    } else {
        format!("R/P{}Y", most_common / 365)
    }
}

// =====================================================================
// Shared helpers
// =====================================================================

#[derive(Debug, Clone, Copy)]
struct BBoxCoords {
    min_lon: f64,
    min_lat: f64,
    max_lon: f64,
    max_lat: f64,
}

/// Resolve a bounding box from the template context, with the same
/// precedence DP+ uses:
///   1. `resource.dpp_spatial_extent.coordinates` (BoundingBox geometry)
///   2. `dpp.LAT_FIELD` / `dpp.LON_FIELD` → `dpps[field].stats.{min,max}`
///
/// Raises a ValueError-equivalent if no lat/lon fields are found.
fn bbox_from_context(state: &minijinja::State) -> Result<BBoxCoords, Error> {
    // Try resource.dpp_spatial_extent first
    let resource = state.lookup("resource").unwrap_or(Value::UNDEFINED);
    let extent = resource
        .get_attr("dpp_spatial_extent")
        .unwrap_or(Value::UNDEFINED);
    if !extent.is_undefined() && !extent.is_none() {
        let ext_type = extent.get_attr("type").unwrap_or(Value::UNDEFINED);
        if ext_type.as_str() != Some("BoundingBox") {
            return Err(value_err("Spatial extent is not a BoundingBox"));
        }
        let coords = extent
            .get_attr("coordinates")
            .map_err(|_| value_err("BoundingBox missing coordinates"))?;
        let cs: Vec<Vec<f64>> = deserialize_value(&coords)
            .ok_or_else(|| value_err("BoundingBox coordinates malformed"))?;
        if cs.len() != 2 || cs[0].len() != 2 || cs[1].len() != 2 {
            return Err(value_err("BoundingBox coordinates malformed"));
        }
        return Ok(BBoxCoords {
            min_lon: cs[0][0],
            min_lat: cs[0][1],
            max_lon: cs[1][0],
            max_lat: cs[1][1],
        });
    }

    // Fall back to inferred LAT/LON fields
    let dpp = state.lookup("dpp").unwrap_or(Value::UNDEFINED);
    let no_lat_lon = dpp
        .get_attr("NO_LAT_LON_FIELDS")
        .ok()
        .map(|v| v.is_true())
        .unwrap_or(false);
    if no_lat_lon {
        return Err(value_err("No latitude or longitude fields found"));
    }
    let lat_field = dpp
        .get_attr("LAT_FIELD")
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .ok_or_else(|| value_err("No latitude or longitude fields found"))?;
    let lon_field = dpp
        .get_attr("LON_FIELD")
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .ok_or_else(|| value_err("No latitude or longitude fields found"))?;
    let dpps = state.lookup("dpps").unwrap_or(Value::UNDEFINED);
    let stat = |field: &str| -> Result<(f64, f64), Error> {
        let s = dpps
            .get_attr(field)
            .unwrap_or(Value::UNDEFINED)
            .get_attr("stats")
            .unwrap_or(Value::UNDEFINED);
        let mn = s
            .get_attr("min")
            .ok()
            .and_then(|v| value_to_f64(&v))
            .ok_or_else(|| value_err(&format!("stats.min missing for {field}")))?;
        let mx = s
            .get_attr("max")
            .ok()
            .and_then(|v| value_to_f64(&v))
            .ok_or_else(|| value_err(&format!("stats.max missing for {field}")))?;
        Ok((mn, mx))
    };
    let (min_lat, max_lat) = stat(&lat_field)?;
    let (min_lon, max_lon) = stat(&lon_field)?;
    Ok(BBoxCoords {
        min_lon,
        min_lat,
        max_lon,
        max_lat,
    })
}

/// Best-effort coercion of a minijinja Value to f64 — handles ints,
/// floats, and numeric strings (DP+'s Python helpers all called
/// `float(...)` on inputs liberally).
fn value_to_f64(v: &Value) -> Option<f64> {
    if let Ok(n) = TryInto::<f64>::try_into(v.clone()) {
        return Some(n);
    }
    if let Some(s) = v.as_str() {
        return s.parse::<f64>().ok();
    }
    None
}

fn value_err(msg: &str) -> Error {
    Error::new(ErrorKind::InvalidOperation, msg.to_string())
}

/// Round-trip a minijinja `Value` through `serde_json` to a typed Rust
/// value. Returns `None` if the value can't be serialized or doesn't
/// deserialize into `T`. Used in lieu of `ViaDeserialize`, which is only
/// usable as a function-parameter wrapper, not an ad-hoc conversion.
fn deserialize_value<T: serde::de::DeserializeOwned>(v: &Value) -> Option<T> {
    let json = serde_json::to_value(v).ok()?;
    serde_json::from_value(json).ok()
}

/// US-locale thousands-separated float format, e.g. `format_thousands(1234567.89, 2) ==
/// "1,234,567.89"`.
fn format_thousands(n: f64, decimals: usize) -> String {
    let formatted = format!("{n:.*}", decimals);
    let (int_part, dec_part) = match formatted.find('.') {
        Some(idx) => (&formatted[..idx], &formatted[idx..]),
        None => (formatted.as_str(), ""),
    };
    let (sign, digits) = if let Some(stripped) = int_part.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", int_part)
    };
    let mut with_commas = String::new();
    let len = digits.len();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            with_commas.push(',');
        }
        with_commas.push(c);
    }
    format!("{sign}{with_commas}{dec_part}")
}

// =====================================================================
// Tests
// =====================================================================
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn render(env: &Environment, src: &str, ctx: serde_json::Value) -> Result<String, Error> {
        let tmpl = env.template_from_str(src)?;
        tmpl.render(Value::from_serialize(&ctx))
    }

    fn build_env() -> Environment<'static> {
        let mut env = Environment::new();
        register(&mut env);
        env
    }

    #[test]
    fn format_thousands_handles_negatives_and_small_numbers() {
        assert_eq!(format_thousands(1234567.89, 2), "1,234,567.89");
        assert_eq!(format_thousands(-1234.5, 1), "-1,234.5");
        assert_eq!(format_thousands(0.0, 0), "0");
        assert_eq!(format_thousands(999.999, 2), "1,000.00");
    }

    #[test]
    fn truncate_with_ellipsis_filter() {
        let env = build_env();
        let out = render(
            &env,
            "{{ 'Hello, world!' | truncate_with_ellipsis(5) }}",
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "Hello...");
    }

    #[test]
    fn format_number_filter() {
        let env = build_env();
        let out = render(
            &env,
            "{{ value | format_number }}",
            json!({"value": 1234567.89}),
        )
        .unwrap();
        assert_eq!(out, "1,234,567.89");
    }

    #[test]
    fn format_bytes_filter() {
        let env = build_env();
        let out = render(
            &env,
            "{{ value | format_bytes }}",
            json!({"value": 1610612736_u64}), // 1.5 GiB
        )
        .unwrap();
        assert_eq!(out, "1.5 GB");
    }

    #[test]
    fn calculate_percentage_filter_handles_zero_whole() {
        let env = build_env();
        let err = render(&env, "{{ 5 | calculate_percentage(0) }}", json!({})).unwrap_err();
        let msg = format!("{err}");
        assert!(msg.contains("Whole value is zero"), "got: {msg}");
    }

    #[test]
    fn spatial_extent_wkt_with_explicit_coords() {
        let env = build_env();
        let out = render(
            &env,
            "{{ spatial_extent_wkt(-180, -90, 180, 90) }}",
            json!({}),
        )
        .unwrap();
        assert_eq!(
            out,
            "SRID=4326;POLYGON((-180 -90, -180 90, 180 90, 180 -90, -180 -90))"
        );
    }

    #[test]
    fn spatial_extent_wkt_uses_lat_lon_fields_from_context() {
        let env = build_env();
        let ctx = json!({
            "dpp": {
                "LAT_FIELD": "latitude",
                "LON_FIELD": "longitude",
                "NO_LAT_LON_FIELDS": false
            },
            "dpps": {
                "latitude":  {"stats": {"min": 40.0, "max": 41.0}},
                "longitude": {"stats": {"min": -75.0, "max": -73.0}}
            }
        });
        let out = render(&env, "{{ spatial_extent_wkt() }}", ctx).unwrap();
        assert!(out.contains("SRID=4326;POLYGON(("), "got: {out}");
        assert!(out.contains("-75 40"), "got: {out}");
        assert!(out.contains("-73 41"), "got: {out}");
    }

    #[test]
    fn spatial_extent_wkt_errors_when_no_lat_lon() {
        let env = build_env();
        let ctx = json!({"dpp": {"NO_LAT_LON_FIELDS": true}});
        let err = render(&env, "{{ spatial_extent_wkt() }}", ctx).unwrap_err();
        assert!(format!("{err}").contains("No latitude or longitude fields found"));
    }

    #[test]
    fn get_column_null_percentage_zero_record_count() {
        let env = build_env();
        let out = render(
            &env,
            "{{ get_column_null_percentage('id') }}",
            json!({"dpp": {"RECORD_COUNT": 0}, "dpps": {"id": {"stats": {"nullcount": 5}}}}),
        )
        .unwrap();
        assert_eq!(out, "0.0");
    }

    #[test]
    fn get_column_null_percentage_normal() {
        let env = build_env();
        let out = render(
            &env,
            "{{ get_column_null_percentage('id') }}",
            json!({"dpp": {"RECORD_COUNT": 100}, "dpps": {"id": {"stats": {"nullcount": 25}}}}),
        )
        .unwrap();
        assert_eq!(out, "25.0");
    }

    #[test]
    fn format_coordinates_filter() {
        // Registered as a filter (matches DP+'s @jinja2_filter decorator),
        // so the lat value gets piped in as the first arg.
        let env = build_env();
        let out = render(
            &env,
            "{{ 40.7128 | format_coordinates(-74.006, 4) }}",
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "40.7128°N, 74.0060°W");
    }

    #[test]
    fn temporal_resolution_errors_without_backend() {
        // Without a SQL backend installed (e.g. stdin input), the helper
        // surfaces a clear error instead of crashing.
        super::set_sql_backend(None);
        let env = build_env();
        let err = render(&env, "{{ temporal_resolution('d') }}", json!({})).unwrap_err();
        assert!(
            format!("{err}").contains("no input CSV available"),
            "got: {err}"
        );
    }

    #[test]
    fn temporal_resolution_daily_dates() {
        use std::io::Write;
        let mut f = tempfile::Builder::new()
            .prefix("tr-daily")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        f.write_all(b"id,date\n1,2024-01-01\n2,2024-01-02\n3,2024-01-03\n4,2024-01-04\n")
            .unwrap();
        f.flush().unwrap();
        super::set_sql_backend(Some(super::SqlBackend::new(f.path())));
        let env = build_env();
        let out = render(&env, "{{ temporal_resolution('date') }}", json!({})).unwrap();
        super::set_sql_backend(None);
        assert_eq!(out, "P1D");
    }

    #[test]
    fn guess_accrual_periodicity_weekly_cadence() {
        use std::io::Write;
        let mut f = tempfile::Builder::new()
            .prefix("ap-weekly")
            .suffix(".csv")
            .tempfile()
            .unwrap();
        f.write_all(
            b"id,date\n1,2024-01-01\n2,2024-01-08\n3,2024-01-15\n4,2024-01-22\n5,2024-01-29\n",
        )
        .unwrap();
        f.flush().unwrap();
        super::set_sql_backend(Some(super::SqlBackend::new(f.path())));
        let env = build_env();
        let out = render(&env, "{{ guess_accrual_periodicity('date') }}", json!({})).unwrap();
        super::set_sql_backend(None);
        assert_eq!(out, "R/P7D");
    }

    #[test]
    fn format_temporal_resolution_thresholds() {
        use super::format_temporal_resolution as f;
        assert_eq!(f(0), "PT1H");
        assert_eq!(f(1), "P1D");
        assert_eq!(f(7), "P7D");
        assert_eq!(f(31), "P31D");
        assert_eq!(f(60), "P2M"); // 60/30 = 2
        assert_eq!(f(366), "P12M"); // boundary: 366/30=12
        assert_eq!(f(730), "P2Y"); // 730/365 = 2
    }

    #[test]
    fn format_accrual_periodicity_thresholds() {
        use super::format_accrual_periodicity as f;
        assert_eq!(f(1), "R/P1D");
        assert_eq!(f(7), "R/P7D");
        assert_eq!(f(31), "R/P31D");
        assert_eq!(f(90), "R/P3M");
        assert_eq!(f(365), "R/P12M");
        assert_eq!(f(730), "R/P2Y");
    }
}
