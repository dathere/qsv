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
//!   * SQL globals — backed by Polars SQL over the input CSV via `sql_backend::SqlBackend`. The
//!     backend is installed on the current thread by `formula_engine::evaluate_spec` for the
//!     render-pass duration; without one installed, the helpers surface a clear "no input CSV
//!     available" error rather than crashing (matches the old Python-stub behaviour of returning
//!     empty datastore results).
//!
//! Default field-name candidate lists for lat/lon detection come from
//! the old `qsv_ckan_stubs.py` defaults and `ckanext.datapusher_plus.config`
//! upstream defaults.

// minijinja's `Function`/`Filter` trait bounds require by-value args
// (`Value`, `Kwargs`, `Rest<Value>`, owned `String`) to satisfy its `ArgType`
// machinery, so clippy::needless_pass_by_value fires on every registered
// helper here even though the signatures cannot take references.
#![allow(clippy::needless_pass_by_value)]

use std::cell::RefCell;

use chrono::{DateTime, NaiveDate, NaiveDateTime};
use minijinja::{Environment, Error, ErrorKind, Value, value::Kwargs};

use super::sql_backend::SqlBackend;

/// Pick the first defined value from a chain of `Option<T>`s. Used to
/// implement Jinja2-style "positional or keyword" argument resolution
/// in helpers that DP+ documents with named parameters
/// (`format_date(value, format='%Y-%m-%d')` etc.).
fn first_some<T>(opts: [Option<T>; 2]) -> Option<T> {
    let [a, b] = opts;
    a.or(b)
}

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

    // --- filters (profile-engine additions) ---------------------------
    env.add_filter("only_if_absolute_iri", only_if_absolute_iri);
    env.add_filter("basename", basename);
    env.add_filter("file_stem", file_stem);
    env.add_filter("sanitize_iso_8601_interval", sanitize_iso_8601_interval);
    env.add_filter("format_mailto", format_mailto);

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

    // --- globals (profile-engine additions, file-aware) ---------------
    env.add_function("sha256_of", sha256_of);
    env.add_function("blake3_of", blake3_of);
    env.add_function("file_size_of", file_size_of);
    env.add_function("compress_format", compress_format);
    env.add_function("package_format", package_format);
    env.add_function("build_csvw_schema", build_csvw_schema);
    env.add_function("bbox_from_dpps", bbox_from_dpps);
    env.add_function("temporal_from_dpps", temporal_from_dpps);
    env.add_function("build_croissant_fields", build_croissant_fields);

    // --- globals (SQL-backed) -----------------------------------------
    // Backed by Polars SQL over the input CSV (see sql_backend.rs).
    // The backend is set process-wide by `formula_engine::evaluate_spec`
    // for the duration of the render pass, then cleared.
    env.add_function("temporal_resolution", temporal_resolution);
    env.add_function("guess_accrual_periodicity", guess_accrual_periodicity);

    // shared data-wrangling filters (regex, datefmt, slugify, padding, etc.)
    crate::minijinja_filters::register(env);
}

// =====================================================================
// FILTERS
// =====================================================================

/// Truncate text to `length`, appending `ellipsis` if truncation occurred.
/// `{{ package.description | truncate_with_ellipsis(10) }}` → "Hello, wo..."
/// Also accepts `length=` and `ellipsis=` keyword args for DP+ parity.
fn truncate_with_ellipsis(
    text: Value,
    length: Option<usize>,
    ellipsis: Option<String>,
    kwargs: Kwargs,
) -> Result<Value, Error> {
    let len = first_some([length, kwargs.get::<Option<usize>>("length")?]).unwrap_or(50);
    let ell = first_some([ellipsis, kwargs.get::<Option<String>>("ellipsis")?])
        .unwrap_or_else(|| "...".to_string());
    kwargs.assert_all_used()?;
    let Some(s) = text.as_str() else {
        return Ok(text);
    };
    if s.chars().count() <= len {
        return Ok(text);
    }
    let truncated: String = s.chars().take(len).collect();
    Ok(Value::from(format!("{truncated}{ell}")))
}

/// Format numbers with thousands separator and decimal places.
/// `{{ value | format_number }}` → "1,234,567.89"
/// Also accepts a `decimals=` keyword arg for DP+ parity.
fn format_number(value: Value, decimals: Option<usize>, kwargs: Kwargs) -> Result<Value, Error> {
    let d = first_some([decimals, kwargs.get::<Option<usize>>("decimals")?]).unwrap_or(2);
    kwargs.assert_all_used()?;
    let Some(n) = value_to_f64(&value) else {
        return Ok(value); // pass through if not numeric
    };
    Ok(Value::from(format_thousands(n, d)))
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
/// parse / format failure. Accepts `format=` as a keyword arg for DP+
/// parity; `format` defaults to `%Y-%m-%d`.
fn format_date(value: Value, format: Option<String>, kwargs: Kwargs) -> Result<Value, Error> {
    let fmt = first_some([format, kwargs.get::<Option<String>>("format")?])
        .unwrap_or_else(|| "%Y-%m-%d".to_string());
    kwargs.assert_all_used()?;
    if value.is_none() || value.is_undefined() {
        return Ok(Value::from_serialize(&Option::<String>::None));
    }
    let Some(s) = value.as_str() else {
        return Ok(value);
    };
    // Try a few common ISO 8601 shapes; fall back to original on failure.
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(Value::from(dt.format(&fmt).to_string()));
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Ok(Value::from(ndt.format(&fmt).to_string()));
    }
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Ok(Value::from(nd.format(&fmt).to_string()));
    }
    Ok(value)
}

/// Calculate percentage. Raises an error if `whole` is zero or inputs
/// aren't numeric — mirrors DP+'s `ValueError` behavior.
fn calculate_percentage(part: Value, whole: Value) -> Result<f64, Error> {
    let p = value_to_f64(&part).ok_or_else(|| value_err("Error calculating percentage"))?;
    let w = value_to_f64(&whole).ok_or_else(|| value_err("Error calculating percentage"))?;
    if w == 0.0 {
        return Err(value_err("Whole value is zero"));
    }
    Ok((p / w) * 100.0)
}

/// Format a range of values: `format_range(min, max, separator=" to ")`.
/// Accepts `separator=` as a keyword arg for DP+ parity.
fn format_range(
    min_val: Value,
    max_val: Value,
    separator: Option<String>,
    kwargs: Kwargs,
) -> Result<Value, Error> {
    let sep = first_some([separator, kwargs.get::<Option<String>>("separator")?])
        .unwrap_or_else(|| " to ".to_string());
    kwargs.assert_all_used()?;
    Ok(Value::from(format!("{min_val}{sep}{max_val}")))
}

/// Format coordinates as `40.712800°N, 74.006000°W`. Accepts
/// `precision=` as a keyword arg for DP+ parity.
fn format_coordinates(
    lat: Value,
    lon: Value,
    precision: Option<usize>,
    kwargs: Kwargs,
) -> Result<String, Error> {
    let p = first_some([precision, kwargs.get::<Option<usize>>("precision")?]).unwrap_or(6);
    kwargs.assert_all_used()?;
    let lat_f =
        value_to_f64(&lat).ok_or_else(|| value_err("format_coordinates: lat must be numeric"))?;
    let lon_f =
        value_to_f64(&lon).ok_or_else(|| value_err("format_coordinates: lon must be numeric"))?;
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
    let mid_lat = f64::midpoint(coords.min_lat, coords.max_lat);
    let width = (coords.max_lon - coords.min_lon).abs().to_radians() * mid_lat.to_radians().cos();
    let height = (coords.max_lat - coords.min_lat).abs().to_radians();
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

/// Convert a bounding box to a named `GeoJSON` `FeatureCollection` string.
/// DP+'s docstring documents this with three named slots, and the
/// examples show both positional and keyword call styles:
///
///   * `spatial_extent_feature_collection("Name", bbox, "manual")`
///   * `spatial_extent_feature_collection(name="X", bbox=[...], feature_type="m")`
///
/// Both shapes are supported. minijinja's `Function` impls don't route
/// `Rest<Value> + Kwargs` cleanly (Rest swallows the kwargs container),
/// so we accept `Rest<Value>` only and detect a trailing kwargs-shaped
/// `Value` ourselves via `Kwargs::extract`.
fn spatial_extent_feature_collection(
    args: minijinja::value::Rest<Value>,
    state: &minijinja::State,
) -> Result<String, Error> {
    // Separate positional args from a trailing kwargs container.
    // `Kwargs::try_from(Value)` succeeds only on the synthetic kwargs
    // marker minijinja emits for `name=...` syntax — which lets us
    // tell apart `func([1,2,3,4])` (a real bbox list as the only
    // positional) from `func(bbox=[1,2,3,4])` (kwargs).
    let (positional, kwargs): (&[Value], Option<Kwargs>) = match args.last() {
        Some(last) => match Kwargs::try_from(last.clone()) {
            Ok(kw) => (&args[..args.len() - 1], Some(kw)),
            Err(_) => (&args[..], None),
        },
        None => (&args[..], None),
    };

    // ---- name -------------------------------------------------------
    let mut name = positional
        .first()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_else(|| "Inferred Spatial Extent".to_string());
    if let Some(kw) = kwargs.as_ref()
        && let Some(v) = kw.get::<Option<String>>("name")?
    {
        name = v;
    }

    // ---- bbox -------------------------------------------------------
    let mut bbox: Option<[f64; 4]> = None;
    let mut bbox_supplied = false;
    if let Some(v) = positional.get(1)
        && !v.is_none()
        && !v.is_undefined()
    {
        bbox = Some(extract_bbox(v)?);
        bbox_supplied = true;
    }
    if let Some(kw) = kwargs.as_ref()
        && let Some(v) = kw.get::<Option<Value>>("bbox")?
        && !v.is_none()
        && !v.is_undefined()
    {
        bbox = Some(extract_bbox(&v)?);
        bbox_supplied = true;
    }

    // ---- feature_type ----------------------------------------------
    let mut feature_type = positional
        .get(2)
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_else(|| {
            if bbox_supplied {
                "calculated".to_string()
            } else {
                "inferred".to_string()
            }
        });
    if let Some(kw) = kwargs.as_ref()
        && let Some(v) = kw.get::<Option<String>>("feature_type")?
    {
        feature_type = v;
    }
    if let Some(kw) = kwargs {
        kw.assert_all_used()?;
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

/// Convert a minijinja Value (expected: list of 4 numerics) into a
/// `[f64; 4]` bbox. Used by both the positional and kwarg paths of
/// `spatial_extent_feature_collection` — iterating via `try_iter` (not
/// serde) handles mixed int/float arrays that DP+'s Python helpers
/// accept uniformly.
fn extract_bbox(v: &Value) -> Result<[f64; 4], Error> {
    let collected: Vec<Value> = v
        .try_iter()
        .map_err(|e| value_err(&format!("bbox must be iterable: {e}")))?
        .collect();
    if collected.len() != 4 {
        return Err(value_err("Invalid bounding box"));
    }
    let mut arr = [0_f64; 4];
    for (i, vv) in collected.iter().enumerate() {
        arr[i] = value_to_f64(vv).ok_or_else(|| value_err("bbox must be a list of 4 numerics"))?;
    }
    Ok(arr)
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
        return Ok(field_stats
            .get_attr(name)
            .unwrap_or_else(|_| Value::from(0)));
    }
    // try list-of-stat-names
    if let Some(names) = deserialize_value::<Vec<String>>(stat_arg) {
        let mut out = std::collections::BTreeMap::<String, Value>::new();
        for n in names {
            let v = field_stats.get_attr(&n).unwrap_or_else(|_| Value::from(0));
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
/// Parse ISO 8601 date / datetime / RFC3339 strings into `NaiveDateTime`
/// for interval math. Sub-second precision is preserved; date-only
/// strings get midnight as the time component.
///
/// Patterns tried in order, mirroring what Python's
/// `datetime.fromisoformat()` accepts (which DP+'s helpers used):
///   * `%Y-%m-%dT%H:%M:%S%.f` — datetime with fractional seconds (T)
///   * `%Y-%m-%d %H:%M:%S%.f` — datetime with fractional seconds (space)
///   * `%Y-%m-%dT%H:%M:%S`    — datetime, second precision (T)
///   * `%Y-%m-%d %H:%M:%S`    — datetime, second precision (space)
///   * `%Y-%m-%d`             — bare date → midnight
///   * RFC 3339               — with timezone (normalized to UTC)
fn parse_date_strings(strs: &[String]) -> Result<Vec<NaiveDateTime>, Error> {
    let mut out = Vec::with_capacity(strs.len());
    for s in strs {
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
            out.push(dt);
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
            out.push(dt);
        } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
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
/// Most-frequent value, with ties broken by first-seen order to match
/// Python's `Counter(...).most_common(1)[0]` semantics. The earlier
/// implementation used a `HashMap`, which made tie-breaking
/// nondeterministic and could change `guess_accrual_periodicity`
/// results between runs when multiple intervals are equally common.
/// Walking the input once to assign each unique value a stable index,
/// then `max_by_key` over (count, -`first_index`), preserves first-seen
/// order without needing an external `IndexMap` dep.
fn mode(values: &[i64]) -> Option<i64> {
    use std::collections::HashMap;
    let mut first_seen: HashMap<i64, usize> = HashMap::new();
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for (i, &v) in values.iter().enumerate() {
        first_seen.entry(v).or_insert(i);
        *counts.entry(v).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(v, c)| {
            // Higher count wins; on a tie, the value first seen (lower
            // first_seen index) wins — invert the index sign by
            // comparing with a `Reverse`-style trick so the SMALLEST
            // index beats larger ones.
            (c, std::cmp::Reverse(first_seen[&v]))
        })
        .map(|(v, _)| v)
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
///   1. `resource.dpp_spatial_extent.coordinates` (`BoundingBox` geometry)
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
        .is_some_and(|v| v.is_true());
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
    let formatted = format!("{n:.decimals$}");
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

// =========================================================================
// Profile-engine helpers (added for YAML-driven projection)
// =========================================================================

/// `only_if_absolute_iri` filter — passes the value through if it parses
/// as an absolute IRI with http/https/ftp/ftps/file scheme; otherwise
/// returns minijinja's undefined sentinel so `| default(...)` chains
/// can route to a fallback. Used by `dcat:landingPage`, `dcat:accessURL`,
/// and similar IRI-typed slots to reject bare strings.
fn only_if_absolute_iri(value: &str) -> minijinja::Value {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return minijinja::Value::UNDEFINED;
    }
    let lower = trimmed.to_ascii_lowercase();
    for prefix in ["http://", "https://", "ftp://", "ftps://", "file://"] {
        if lower.starts_with(prefix) {
            return minijinja::Value::from(trimmed.to_string());
        }
    }
    minijinja::Value::UNDEFINED
}

/// `basename` filter — returns the final path segment of `value`. Used
/// to derive a `dct:title` fallback from the URL's last segment.
fn basename(value: &str) -> String {
    std::path::Path::new(value)
        .file_name()
        .and_then(|s| s.to_str())
        .map_or_else(|| value.to_string(), str::to_string)
}

/// `file_stem` filter — returns the basename minus its extension. Used
/// to derive a tempfile-stem-aware `dcat:Distribution.dct:title`.
fn file_stem(value: &str) -> String {
    std::path::Path::new(value)
        .file_stem()
        .and_then(|s| s.to_str())
        .map_or_else(|| value.to_string(), str::to_string)
}

/// `sanitize_iso_8601_interval` filter — rejects interval / repeating
/// ISO 8601 syntax (`R/P1Y`, `2024-01-01/2024-06-30`). Useful for
/// `dct:modified` / `dct:issued` slots where a pure instant is expected;
/// instead of forwarding a malformed interval, returns Jinja undefined
/// so the field is suppressed.
fn sanitize_iso_8601_interval(value: &str) -> minijinja::Value {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return minijinja::Value::UNDEFINED;
    }
    if trimmed.starts_with("R/") || trimmed.starts_with('R') && trimmed.contains("/P") {
        return minijinja::Value::UNDEFINED;
    }
    if trimmed.contains('/') && !trimmed.starts_with("http") {
        // Likely interval form like 2024-01-01/2024-06-30.
        return minijinja::Value::UNDEFINED;
    }
    minijinja::Value::from(trimmed.to_string())
}

/// `format_mailto` filter — trims whitespace and prepends `mailto:` if
/// missing. Returns minijinja undefined for empty input so chained
/// `| default` fallbacks work.
fn format_mailto(value: &str) -> minijinja::Value {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return minijinja::Value::UNDEFINED;
    }
    if trimmed.to_ascii_lowercase().starts_with("mailto:") {
        minijinja::Value::from(trimmed.to_string())
    } else {
        minijinja::Value::from(format!("mailto:{trimmed}"))
    }
}

/// `sha256_of` global — streaming SHA-256 of a local file as lowercase
/// hex. Returns minijinja undefined on read failure (best-effort).
fn sha256_of(path: &str) -> minijinja::Value {
    use std::{fmt::Write as _, io::Read};

    use sha2::{Digest, Sha256};
    let Ok(mut file) = std::fs::File::open(path) else {
        return minijinja::Value::UNDEFINED;
    };
    let mut hasher = Sha256::new();
    #[allow(clippy::large_stack_arrays)]
    let mut buf = [0u8; 65536];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => hasher.update(&buf[..n]),
            Err(_) => return minijinja::Value::UNDEFINED,
        }
    }
    let digest = hasher.finalize();
    // Manually hex-encode the digest bytes: sha2 0.11 returns
    // `hybrid_array::Array<u8, _>`, which does not implement `LowerHex`
    // (so `format!("{digest:x}")` no longer compiles). Iterating
    // works on both 0.10 (GenericArray derefs to slice) and 0.11.
    let mut hex = String::with_capacity(digest.len() * 2);
    for b in &digest {
        let _ = write!(&mut hex, "{b:02x}");
    }
    minijinja::Value::from(hex)
}

/// `blake3_of` global — BLAKE3 digest of a local file as lowercase hex.
/// Uses the qsv-wide blake3 dep with mmap+rayon for large-file speed.
fn blake3_of(path: &str) -> minijinja::Value {
    let mut hasher = blake3::Hasher::new();
    if hasher.update_mmap_rayon(path).is_err() {
        return minijinja::Value::UNDEFINED;
    }
    let digest = hasher.finalize();
    minijinja::Value::from(digest.to_hex().to_string())
}

/// `file_size_of` global — byte length of a local file as a string.
/// Matches the GSA `dcat:byteSize` convention (number serialized as
/// string). Returns minijinja undefined on stat failure.
fn file_size_of(path: &str) -> minijinja::Value {
    match std::fs::metadata(path) {
        Ok(m) => minijinja::Value::from(m.len().to_string()),
        Err(_) => minijinja::Value::UNDEFINED,
    }
}

/// `compress_format` global — IANA media type for single-file
/// compressors derived from the path's extension. Returns minijinja
/// undefined when the extension is unrecognized.
// `lower` is already ASCII-lowercased, so the case-sensitivity lint is moot;
// the `match ()` guard idiom intentionally uses `_` arms over `ends_with`.
#[allow(
    clippy::case_sensitive_file_extension_comparisons,
    clippy::ignored_unit_patterns
)]
fn compress_format(path: &str) -> minijinja::Value {
    let lower = path.to_ascii_lowercase();
    let m = match () {
        _ if lower.ends_with(".gz") => "application/gzip",
        _ if lower.ends_with(".zst") => "application/zstd",
        _ if lower.ends_with(".bz2") => "application/x-bzip2",
        _ if lower.ends_with(".xz") => "application/x-xz",
        _ if lower.ends_with(".br") => "application/brotli",
        _ => return minijinja::Value::UNDEFINED,
    };
    minijinja::Value::from(m.to_string())
}

/// `package_format` global — IANA media type for archive containers
/// derived from the path's extension. Returns minijinja undefined when
/// the extension is unrecognized.
// `lower` is already ASCII-lowercased, and `.tar.gz`/`.tgz` double-extension
// matching cannot be expressed via `Path::extension`, so the `ends_with`
// guards stay; the `match ()` idiom intentionally uses `_` arms.
#[allow(
    clippy::case_sensitive_file_extension_comparisons,
    clippy::ignored_unit_patterns
)]
fn package_format(path: &str) -> minijinja::Value {
    let lower = path.to_ascii_lowercase();
    let m = match () {
        _ if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") => "application/gzip",
        _ if lower.ends_with(".tar") => "application/x-tar",
        _ if lower.ends_with(".zip") => "application/zip",
        _ if lower.ends_with(".7z") => "application/x-7z-compressed",
        _ => return minijinja::Value::UNDEFINED,
    };
    minijinja::Value::from(m.to_string())
}

/// `build_csvw_schema` global — walks a stats map (column-name → stats
/// blob, the shape produced by `context.rs::build_dpps`) and emits a
/// `{ columns: [...] }` shape suitable for `csvw:tableSchema`. Each
/// column entry carries `name`, `titles`, `datatype`, plus the
/// `qsv:cardinality`, `qsv:nullcount`, `qsv:min`, `qsv:max` extensions
/// that the legacy `dcat.rs::build_distribution` emitted.
///
/// The datatype mapping uses the legacy `csvw_datatype` rule directly
/// so wire-shape parity with the pre-YAML engine is preserved.
fn build_csvw_schema(stats: minijinja::Value) -> minijinja::Value {
    // Serialize the minijinja Value once into a JSON Value so we can
    // walk it with native serde_json calls (works for both object and
    // array shapes; recordset templates iterate via index instead).
    let stats_json: serde_json::Value = match serde_json::to_value(&stats) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let Some(cols) = stats_json.as_object() else {
        return minijinja::Value::UNDEFINED;
    };
    let columns: Vec<serde_json::Value> = cols
        .iter()
        .map(|(name, blob)| {
            // Legacy convention: per-column blob may carry a nested
            // `stats` sub-object (Schema mode) or be flat (DPPS mode).
            let stats_obj = blob.get("stats").unwrap_or(blob);
            serde_json::json!({
                "name":             name,
                "titles":           [name],
                "datatype":         csvw_datatype_legacy(stats_obj.get("type")),
                "qsv:cardinality":  stats_obj.get("cardinality"),
                "qsv:nullcount":    stats_obj.get("nullcount"),
                "qsv:min":          stats_obj.get("min"),
                "qsv:max":          stats_obj.get("max"),
            })
        })
        .collect();
    minijinja::Value::from_serialize(serde_json::json!({ "columns": columns }))
}

/// Mirror of the legacy `dcat::csvw_datatype` — maps qsv stats type
/// strings to CSVW datatype IRIs. Kept here so the helper is
/// self-contained and `dcat.rs` can be deleted in Stage 4 cleanup.
fn csvw_datatype_legacy(t: Option<&serde_json::Value>) -> &'static str {
    #[allow(clippy::match_same_arms)]
    match t.and_then(serde_json::Value::as_str) {
        Some("Integer") => "integer",
        Some("Float") => "double",
        Some("Boolean") => "boolean",
        Some("Date") => "date",
        Some("DateTime") => "dateTime",
        Some("NULL") => "string",
        _ => "string",
    }
}

/// `bbox_from_dpps` global — derives a `dct:Location` array from the
/// inferred LAT/LON column metadata in `dpp` and the per-column
/// min/max in `stats`. Returns an array suitable for the
/// `dct:spatial` slot (v3 cardinality 0..*), or undefined when no
/// lat/lon columns were detected. Mirrors the legacy
/// `dcat::bbox_from_dpps` so wire-shape parity is preserved.
fn bbox_from_dpps(dpp: minijinja::Value, stats: minijinja::Value) -> minijinja::Value {
    let dpp_json: serde_json::Value = match serde_json::to_value(&dpp) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let stats_json: serde_json::Value = match serde_json::to_value(&stats) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let lat = dpp_json
        .get("LAT_FIELD")
        .and_then(serde_json::Value::as_str);
    let lon = dpp_json
        .get("LON_FIELD")
        .and_then(serde_json::Value::as_str);
    let (Some(lat), Some(lon)) = (lat, lon) else {
        return minijinja::Value::UNDEFINED;
    };
    let lookup = |field: &str, key: &str| -> Option<f64> {
        let blob = stats_json.get(field)?;
        let inner = blob.get("stats").unwrap_or(blob);
        let v = inner.get(key)?;
        #[allow(clippy::cast_precision_loss)]
        v.as_f64()
            .or_else(|| v.as_i64().map(|i| i as f64))
            .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
    };
    let (Some(min_lon), Some(max_lon), Some(min_lat), Some(max_lat)) = (
        lookup(lon, "min"),
        lookup(lon, "max"),
        lookup(lat, "min"),
        lookup(lat, "max"),
    ) else {
        return minijinja::Value::UNDEFINED;
    };
    let polygon = format!(
        "POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, {max_lon} \
         {min_lat}, {min_lon} {min_lat}))"
    );
    minijinja::Value::from_serialize(serde_json::json!([{
        "@type": "dct:Location",
        "dcat:bbox": polygon,
    }]))
}

/// `temporal_from_dpps` global — derives a `dct:PeriodOfTime` array,
/// one entry per inferred date column. Returns undefined when no
/// date columns were detected. Mirrors the legacy
/// `dcat::temporal_from_dpps` (v3 cardinality 0..*).
fn temporal_from_dpps(dpp: minijinja::Value, stats: minijinja::Value) -> minijinja::Value {
    let dpp_json: serde_json::Value = match serde_json::to_value(&dpp) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let stats_json: serde_json::Value = match serde_json::to_value(&stats) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let Some(dates) = dpp_json
        .get("DATE_FIELDS")
        .and_then(serde_json::Value::as_array)
    else {
        return minijinja::Value::UNDEFINED;
    };
    let mut out: Vec<serde_json::Value> = Vec::new();
    for field_v in dates {
        let Some(field) = field_v.as_str() else {
            continue;
        };
        let blob = stats_json
            .get(field)
            .and_then(|b| b.get("stats").or(Some(b)));
        let Some(start) = blob.and_then(|b| b.get("min")).and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(end) = blob.and_then(|b| b.get("max")).and_then(|v| v.as_str()) else {
            continue;
        };
        out.push(serde_json::json!({
            "@type":          "dct:PeriodOfTime",
            "dcat:startDate": start,
            "dcat:endDate":   end,
        }));
    }
    if out.is_empty() {
        return minijinja::Value::UNDEFINED;
    }
    minijinja::Value::from_serialize(out)
}

/// `build_croissant_fields` global — walks a stats map (column-name →
/// stats blob) and emits a flat array of `cr:Field` objects suitable
/// for Croissant's `recordSet[0].field` slot. The datatype maps
/// through the profile's `croissant_datatype` vocabulary (sc:Text /
/// sc:Integer / sc:Float / sc:Date / sc:DateTime / sc:Boolean / sc:URL).
///
/// Distinct from `build_csvw_schema`: that helper emits CSVW-shaped
/// `{columns: [...]}` for DCAT's `csvw:tableSchema`; this one emits
/// a top-level array of Field entries for Croissant's
/// `cr:RecordSet.field`.
fn build_croissant_fields(stats: minijinja::Value) -> minijinja::Value {
    let stats_json: serde_json::Value = match serde_json::to_value(&stats) {
        Ok(v) => v,
        Err(_) => return minijinja::Value::UNDEFINED,
    };
    let Some(cols) = stats_json.as_object() else {
        return minijinja::Value::UNDEFINED;
    };
    let fields: Vec<serde_json::Value> = cols
        .iter()
        .map(|(name, blob)| {
            let stats_obj = blob.get("stats").unwrap_or(blob);
            let qsv_type = stats_obj
                .get("type")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("String");
            // Map via Croissant atomic types; default sc:Text.
            let datatype = match qsv_type {
                "Integer" => "sc:Integer",
                "Float" => "sc:Float",
                "Boolean" => "sc:Boolean",
                "Date" => "sc:Date",
                "DateTime" => "sc:DateTime",
                "URL" => "sc:URL",
                _ => "sc:Text",
            };
            serde_json::json!({
                "@type":              "cr:Field",
                "@id":                format!("main-table/{name}"),
                "name":               name,
                "description":        format!("Column `{name}` from the source CSV."),
                "dataType":           datatype,
                "qsv:cardinality":    stats_obj.get("cardinality"),
                "qsv:nullcount":      stats_obj.get("nullcount"),
            })
        })
        .collect();
    minijinja::Value::from_serialize(fields)
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

    /// Copilot PR #3901 finding (mode() determinism): when multiple
    /// values share the highest count, the first-seen value must win
    /// for reproducible guess_accrual_periodicity output across runs.
    #[test]
    fn mode_tie_broken_by_first_seen_order() {
        use super::mode;
        // 1 and 7 both appear twice. The first occurrence of 1 is at
        // index 0, of 7 is at index 1 — so 1 must win, deterministically.
        assert_eq!(mode(&[1, 7, 7, 1, 3]), Some(1));
        // 5 first appears at index 0, 2 at index 2 — both count==2; 5 wins.
        assert_eq!(mode(&[5, 5, 2, 2]), Some(5));
        // Unique max wins regardless of position.
        assert_eq!(mode(&[3, 3, 3, 1, 1]), Some(3));
        // Empty input returns None.
        assert_eq!(mode(&[]), None);
    }

    /// Copilot PR #3901 finding (fractional-second ISO 8601 datetimes):
    /// DP+'s `datetime.fromisoformat` accepts these and our parser
    /// must too — otherwise temporal_resolution /
    /// guess_accrual_periodicity hard-fail on CSVs with sub-second
    /// timestamps.
    #[test]
    fn parse_date_strings_accepts_fractional_seconds() {
        use super::parse_date_strings;
        let inputs = vec![
            "2024-01-01T12:34:56.789".to_string(),
            "2024-01-02 12:34:56.789".to_string(),
            "2024-01-03T12:34:56".to_string(),
            "2024-01-04".to_string(),
        ];
        let parsed = parse_date_strings(&inputs).expect("all four shapes must parse");
        assert_eq!(parsed.len(), 4);
        // Spot-check the fractional-second values reach millisecond precision.
        assert_eq!(
            parsed[0].format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-01-01T12:34:56.789"
        );
        assert_eq!(
            parsed[1].format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            "2024-01-02T12:34:56.789"
        );
    }
    /// Roborev finding 2439#6: DP+ helpers must accept Jinja2 keyword
    /// arguments alongside positional ones.
    #[test]
    fn helpers_accept_keyword_args() {
        let env = build_env();

        // format_date with format= kwarg
        let out = render(
            &env,
            r#"{{ '2024-01-15' | format_date(format='%B %d, %Y') }}"#,
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "January 15, 2024");

        // truncate_with_ellipsis with length= + ellipsis= kwargs
        let out = render(
            &env,
            r#"{{ 'Hello, world!' | truncate_with_ellipsis(length=5, ellipsis='…') }}"#,
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "Hello…");

        // format_number with decimals= kwarg.
        // 1234.6 (not 1234.5 — Rust's f64 → string uses banker's
        // rounding, so .5 rounds to even and would yield 1,234).
        let out = render(
            &env,
            r#"{{ 1234.6 | format_number(decimals=0) }}"#,
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "1,235");

        // format_coordinates with precision= kwarg (lat piped in as filter)
        let out = render(
            &env,
            r#"{{ 40.7128 | format_coordinates(-74.006, precision=2) }}"#,
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "40.71°N, 74.01°W");

        // format_range is registered as a filter (matches DP+'s
        // @jinja2_filter decorator), so the min value pipes in first.
        let out = render(
            &env,
            r#"{{ 1 | format_range(10, separator=' through ') }}"#,
            json!({}),
        )
        .unwrap();
        assert_eq!(out, "1 through 10");

        // spatial_extent_feature_collection with name=, bbox=, feature_type=
        let out = render(
            &env,
            r#"{{ spatial_extent_feature_collection(name='Custom', bbox=[-180,-90,180,90], feature_type='manual') }}"#,
            json!({}),
        )
        .unwrap();
        assert!(out.contains(r#""name": "Custom""#), "got: {out}");
        assert!(out.contains(r#""type": "manual""#), "got: {out}");
        assert!(out.contains("-180,-90"), "got: {out}");
    }

    /// Roborev finding 2440#3: positional calls to
    /// `spatial_extent_feature_collection` (the DP+ docstring's other
    /// example syntax) must keep working alongside kwargs.
    #[test]
    fn spatial_extent_feature_collection_positional() {
        let env = build_env();
        // Three positional args: name, bbox, feature_type
        let out = render(
            &env,
            r#"{{ spatial_extent_feature_collection("PosName", [-180,-90,180,90], "manual") }}"#,
            json!({}),
        )
        .unwrap();
        assert!(out.contains(r#""name": "PosName""#), "got: {out}");
        assert!(out.contains(r#""type": "manual""#), "got: {out}");
        assert!(out.contains("-180,-90"), "got: {out}");
    }

    /// Mixed positional + kwargs: name positional, bbox + feature_type
    /// as kwargs. Verifies the trailing-Value-as-kwargs detection
    /// doesn't mis-classify a real positional Value as a kwargs marker.
    #[test]
    fn spatial_extent_feature_collection_mixed_pos_and_kw() {
        let env = build_env();
        let out = render(
            &env,
            r#"{{ spatial_extent_feature_collection("Mix", bbox=[-1,-1,1,1], feature_type="m") }}"#,
            json!({}),
        )
        .unwrap();
        assert!(out.contains(r#""name": "Mix""#), "got: {out}");
        assert!(out.contains(r#""type": "m""#), "got: {out}");
    }
}
