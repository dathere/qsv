//! `--locate`: name the places a dataset covers, for the DCAT-US v3
//! `spatial` Locations.
//!
//! The bounding box says WHERE the coordinates are; it does not say
//! "Boston" or "Suffolk County". With `--locate`, qsv samples the
//! densest coordinate cells (row-weighted, deterministic) and looks them
//! up with `qsv geocode`:
//!
//!   * `geonames` — the offline Geonames index (`geocode reverse`). The finest level that at least
//!     [`AGREEMENT`] of the sampled rows share — nearest place, then admin1, then country — labels
//!     the bbox Location (`prefLabel`, plus a `GeoNames` `identifier` for a place or country).
//!   * `census` — the keyless US Census Geocoder (`geocode census --reverse`). Each state and each
//!     county holding at least [`REGION_MIN_SHARE`] of the sampled rows (the heaviest state
//!     always), becomes a Location keyed by its Census GEOID.
//!
//! The lookups run once, before projection, as `qsv geocode` child
//! processes — so they share its caches, rate limiting, retries and
//! the `QSV_CENSUS_GEOCODER_URL` / `QSV_GEOCODE_INDEX_FILENAME`
//! overrides — and reach the templates as the plain-JSON `located`
//! context key. A lookup that cannot run is noted, never fatal: the
//! projection keeps its bbox.
//!
//! Region columns, in builds with the `viz` feature, supply the Census
//! Locations instead — whether or not the file has coordinates, since
//! they name every row's region where the lookup samples cells: a state column
//! (USPS code, FIPS or name; header mentioning "state") from its
//! frequency, and a county column (names or GEOIDs; header mentioning
//! "county") resolved exactly against the Census county name table,
//! within each row's state.

use std::{collections::HashMap, io::Write as _};

use serde_json::{Value, json};

use super::{
    derive::{Coords, KEEP_COORDS},
    sql_backend::SqlBackend,
};
use crate::{CliResult, util};

/// How many coordinate cells are looked up. Census single-record
/// lookups run at 5/s, so this bounds a cold `census` run to ~8s.
const SAMPLE_CELLS: usize = 40;
/// Share of the sampled rows a Geonames label must cover.
const AGREEMENT: f64 = 0.8;
/// Share of the sampled rows a Census state or county must hold.
const REGION_MIN_SHARE: f64 = 0.05;
/// At most this many county Locations.
const MAX_COUNTIES: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LocateMode {
    Geonames,
    Census,
    Both,
}

impl LocateMode {
    pub(super) fn parse(s: &str) -> CliResult<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "geonames" => Ok(Self::Geonames),
            "census" => Ok(Self::Census),
            "both" => Ok(Self::Both),
            other => fail_incorrectusage_clierror!(
                "--locate must be geonames, census or both, not {other:?}."
            ),
        }
    }

    const fn geonames(self) -> bool {
        matches!(self, Self::Geonames | Self::Both)
    }

    const fn census(self) -> bool {
        matches!(self, Self::Census | Self::Both)
    }
}

/// One sampled coordinate cell: its rounded point and how many rows
/// fall in it.
#[derive(Clone, Debug, PartialEq)]
struct Cell {
    lat:  f64,
    lon:  f64,
    rows: f64,
}

/// The `located` context value: `primary` keys merged into the bbox
/// Location, `regions` appended after it, and `notes` for
/// `projection_warnings`. `Value::Null` when there is nothing to add.
pub(super) fn locate(
    mode: LocateMode,
    backend: &SqlBackend,
    dpp: &Value,
    dppf: &Value,
    headers: &[String],
) -> Value {
    let mut notes: Vec<String> = Vec::new();
    let mut primary = serde_json::Map::new();
    let mut regions: Vec<Value> = Vec::new();

    // A state or county column names every row's region exactly, where
    // the coordinate lookup samples only the densest cells: when one
    // resolves, it supplies the Census Locations even if the file also
    // has coordinates (which still give the bbox, centroid and Geonames
    // label).
    let (column_found, mut column_notes) = if mode.census() {
        column_regions(backend, dppf, headers)
    } else {
        (Vec::new(), Vec::new())
    };
    let from_columns = !column_found.is_empty();

    let coords = Coords::from_dpp(dpp);
    let has_coordinates = coords.is_some();
    if let Some(coords) = coords {
        match sample_cells(backend, &coords) {
            Some((cells, kept, centroid)) if !cells.is_empty() => {
                primary.insert("centroid".to_string(), centroid);
                let sampled: f64 = cells.iter().map(|c| c.rows).sum();
                let coverage = 100.0 * sampled / kept;
                if mode.geonames() {
                    match spawn_lookup(&cells, &geonames_args()) {
                        Ok(rows) => {
                            if let Some(label) = geonames_label(&cells, &rows) {
                                primary.extend(label);
                            } else {
                                notes.push(format!(
                                    "--locate geonames: no place, admin1 or country is shared by \
                                     {:.0}% of the sampled rows, so the bounding box is not \
                                     labelled",
                                    AGREEMENT * 100.0
                                ));
                            }
                        },
                        Err(e) => notes.push(format!("--locate geonames: lookup failed ({e})")),
                    }
                }
                if mode.census() && !from_columns {
                    match spawn_lookup(&cells, &census_args()) {
                        Ok(rows) => {
                            let (found, mut more) = census_regions(&cells, &rows);
                            if found.is_empty() {
                                notes.push(
                                    "--locate census: no sampled coordinate lies in a US Census \
                                     state"
                                        .to_string(),
                                );
                            }
                            regions.extend(found);
                            notes.append(&mut more);
                        },
                        Err(e) => notes.push(format!(
                            "--locate census: US Census Geocoder lookup failed ({e}); no Census \
                             regions were added"
                        )),
                    }
                }
                if mode.geonames() || !from_columns {
                    notes.push(format!(
                        "--locate: looked up the {} densest coordinate cells, covering \
                         {coverage:.1}% of the rows with valid coordinates",
                        cells.len()
                    ));
                }
            },
            _ => notes.push(format!(
                "--locate: no valid {} to look up",
                coords.describe()
            )),
        }
    } else if !mode.census() {
        notes.push("--locate geonames: no latitude/longitude columns detected".to_string());
    }
    // without coordinates the column notes always explain the outcome;
    // with them, only when the columns supplied the Locations
    if from_columns || !has_coordinates {
        regions.extend(column_found);
        notes.append(&mut column_notes);
    }

    if primary.is_empty() && regions.is_empty() && notes.is_empty() {
        return Value::Null;
    }
    json!({ "primary": primary, "regions": regions, "notes": notes })
}

/// The [`SAMPLE_CELLS`] most populated ~110 m cells (coordinates rounded
/// to 3 decimals), the number of rows with valid coordinates, and their
/// centroid as a GeoJSON Point. Invalid coordinates — out of range, or
/// the (0, 0) placeholder — are excluded, as for the bbox. Ties order by
/// the rounded point, so the sample is deterministic.
fn sample_cells(backend: &SqlBackend, coords: &Coords) -> Option<(Vec<Cell>, f64, Value)> {
    // Read every column as text and cast explicitly: schema inference
    // over the first rows types `42, 42, ... 42.36` as an integer column
    // and the scan then fails on the first decimal.
    let backend = backend.clone().with_all_strings();
    let (table, keep) = (coords.table(), KEEP_COORDS);
    let f64_at = |df: &polars::prelude::DataFrame, c: &str, i: usize| -> Option<f64> {
        let col = df.column(c).ok()?;
        let cast = col.cast(&polars::prelude::DataType::Float64).ok()?;
        cast.f64().ok()?.get(i)
    };

    let totals = backend
        .query(&format!(
            "SELECT COUNT(*) AS kept, AVG(la) AS clat, AVG(lo) AS clon FROM {table} WHERE {keep}"
        ))
        .ok()?;
    let kept = f64_at(&totals, "kept", 0)?;
    if kept <= 0.0 {
        return Some((Vec::new(), 0.0, Value::Null));
    }
    let round6 = |v: f64| (v * 1e6).round() / 1e6;
    let centroid = json!({
        "type": "Point",
        "coordinates": [round6(f64_at(&totals, "clon", 0)?), round6(f64_at(&totals, "clat", 0)?)],
    });

    let df = backend
        .query(&format!(
            "SELECT plat, plon, COUNT(*) AS n FROM (SELECT ROUND(la, 3) AS plat, ROUND(lo, 3) AS \
             plon FROM {table} WHERE {keep}) AS s GROUP BY plat, plon ORDER BY n DESC, plat, plon \
             LIMIT {SAMPLE_CELLS}"
        ))
        .ok()?;
    let cells = (0..df.height())
        .filter_map(|i| {
            Some(Cell {
                lat:  f64_at(&df, "plat", i)?,
                lon:  f64_at(&df, "plon", i)?,
                rows: f64_at(&df, "n", i)?,
            })
        })
        .collect();
    Some((cells, kept, centroid))
}

const POINT_COLUMN: &str = "qsv_locate_point";

fn geonames_args() -> Vec<String> {
    vec![
        "reverse".to_string(),
        POINT_COLUMN.to_string(),
        "--formatstr".to_string(),
        "%dyncols: {gid:id}, {place:name}, {admin1:admin1}, {cc:country}, {country:country_name}, \
         {cgid:country_geonameid}"
            .to_string(),
    ]
}

fn census_args() -> Vec<String> {
    vec![
        "census".to_string(),
        POINT_COLUMN.to_string(),
        "--reverse".to_string(),
        "--formatstr".to_string(),
        "%dyncols: {st:flat.States.GEOID}, {stname:flat.States.NAME}, {co:flat.Counties.GEOID}, \
         {coname:flat.Counties.NAME}"
            .to_string(),
    ]
}

/// Run `qsv geocode <args> <points.csv>` over the sampled cells and
/// return one map of the added columns per cell, in cell order.
fn spawn_lookup(cells: &[Cell], args: &[String]) -> Result<Vec<HashMap<String, String>>, String> {
    let mut points = tempfile::Builder::new()
        .prefix("qsv-profile-locate")
        .suffix(".csv")
        .tempfile()
        .map_err(|e| e.to_string())?;
    let mut body = format!("{POINT_COLUMN}\n");
    for c in cells {
        body.push_str(&format!("\"{}, {}\"\n", c.lat, c.lon));
    }
    points
        .write_all(body.as_bytes())
        .and_then(|()| points.flush())
        .map_err(|e| e.to_string())?;
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let (stdout, stderr) = util::run_qsv_cmd(
        "geocode",
        &argv,
        &points.path().to_string_lossy(),
        &format!("qsv profile: ran `geocode {}` for --locate", args[0]),
    )
    .map_err(|_| format!("`qsv geocode {}` did not complete", args[0]))?;
    // census leaves a lookup that failed after its retries looking like a
    // no-match, and says so only on stderr. Read that as a failure, or an
    // outage would be reported as "no point lies in a US state".
    if stderr.contains("failed after retries") {
        return Err(format!(
            "`qsv geocode {}`: some lookups failed after retries",
            args[0]
        ));
    }
    let rows = parse_lookup_csv(&stdout);
    if rows.len() == cells.len() {
        Ok(rows)
    } else {
        Err(format!(
            "`qsv geocode {}` returned {} rows for {} points",
            args[0],
            rows.len(),
            cells.len()
        ))
    }
}

fn parse_lookup_csv(csv_text: &str) -> Vec<HashMap<String, String>> {
    let mut rdr = csv::ReaderBuilder::new().from_reader(csv_text.as_bytes());
    let Ok(headers) = rdr.headers().cloned() else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(Result::ok)
        .map(|rec| {
            headers
                .iter()
                .zip(rec.iter())
                .map(|(h, v)| (h.to_string(), v.trim().to_string()))
                .collect()
        })
        .collect()
}

fn gazetteer(title: &str, iri: &str) -> Value {
    json!({ "@type": "ConceptScheme", "title": title, "@id": iri })
}

/// The finest Geonames level that [`AGREEMENT`] of the sampled rows
/// share: their nearest place, else their admin1, else their country.
/// Row-weighted, so a few scattered outlying cells cannot outvote the
/// dense core — and a dataset split between two places is labelled by
/// what they have in common, never by one of them.
fn geonames_label(
    cells: &[Cell],
    rows: &[HashMap<String, String>],
) -> Option<serde_json::Map<String, Value>> {
    let field = |r: &HashMap<String, String>, k: &str| r.get(k).cloned().unwrap_or_default();
    let total: f64 = cells.iter().map(|c| c.rows).sum();
    if total <= 0.0 {
        return None;
    }
    let winner = |key: &dyn Fn(&HashMap<String, String>) -> Option<String>| -> Option<String> {
        let mut weights: Vec<(String, f64)> = Vec::new();
        for (cell, row) in cells.iter().zip(rows) {
            if let Some(k) = key(row) {
                match weights.iter_mut().find(|(w, _)| *w == k) {
                    Some((_, w)) => *w += cell.rows,
                    None => weights.push((k, cell.rows)),
                }
            }
        }
        weights
            .into_iter()
            .filter(|(_, w)| *w / total >= AGREEMENT)
            .map(|(k, _)| k)
            .next()
    };
    let sample_of = |pred: &dyn Fn(&HashMap<String, String>) -> bool| {
        rows.iter().find(|r| pred(r)).cloned().unwrap_or_default()
    };
    let scheme = gazetteer("GeoNames", "https://www.geonames.org/");

    let non_empty = |s: String| (!s.is_empty()).then_some(s);
    if let Some(gid) = winner(&|r| non_empty(field(r, "gid"))) {
        let r = sample_of(&|r| field(r, "gid") == gid);
        let label = [
            field(&r, "place"),
            field(&r, "admin1"),
            field(&r, "country"),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
        let mut out = serde_json::Map::new();
        out.insert("prefLabel".into(), json!(label));
        out.insert(
            "identifier".into(),
            json!(format!("https://sws.geonames.org/{gid}/")),
        );
        out.insert("inScheme".into(), scheme);
        return Some(out);
    }
    let admin1_key = |r: &HashMap<String, String>| {
        let (a, c) = (field(r, "admin1"), field(r, "cc"));
        (!a.is_empty() && !c.is_empty()).then(|| format!("{c}\u{1f}{a}"))
    };
    if let Some(key) = winner(&admin1_key) {
        let r = sample_of(&|r| admin1_key(r).as_deref() == Some(key.as_str()));
        let mut out = serde_json::Map::new();
        out.insert(
            "prefLabel".into(),
            json!(format!("{}, {}", field(&r, "admin1"), field(&r, "country"))),
        );
        out.insert("inScheme".into(), scheme);
        return Some(out);
    }
    if let Some(cc) = winner(&|r| non_empty(field(r, "cc"))) {
        let r = sample_of(&|r| field(r, "cc") == cc);
        let mut out = serde_json::Map::new();
        let name = non_empty(field(&r, "country")).unwrap_or(cc);
        out.insert("prefLabel".into(), json!(name));
        if let Some(cgid) = non_empty(field(&r, "cgid")) {
            out.insert(
                "identifier".into(),
                json!(format!("https://sws.geonames.org/{cgid}/")),
            );
        }
        out.insert("inScheme".into(), scheme);
        return Some(out);
    }
    None
}

fn census_scheme() -> Value {
    gazetteer(
        "U.S. Census Bureau geographic identifiers (GEOID)",
        "https://www.census.gov/programs-surveys/geography/guidance/geo-identifiers.html",
    )
}

fn region_location(label: &str, geoid: &str) -> Value {
    json!({
        "@type": "Location",
        "prefLabel": label,
        "identifier": geoid,
        "inScheme": census_scheme(),
    })
}

/// Sum `rows` per key, ordered by weight (heaviest first), then key.
fn ranked(weights: HashMap<String, (String, f64)>) -> Vec<(String, String, f64)> {
    let mut v: Vec<(String, String, f64)> = weights
        .into_iter()
        .map(|(k, (label, w))| (k, label, w))
        .collect();
    v.sort_by(|a, b| b.2.total_cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
    v
}

/// State and county Locations from the Census lookups, row-weighted.
/// A region needs [`REGION_MIN_SHARE`] of the sampled rows (the
/// heaviest state always qualifies), so a stray mis-geocoded row does
/// not add a state; counties are capped at [`MAX_COUNTIES`].
fn census_regions(cells: &[Cell], rows: &[HashMap<String, String>]) -> (Vec<Value>, Vec<String>) {
    let total: f64 = cells.iter().map(|c| c.rows).sum();
    let mut states: HashMap<String, (String, f64)> = HashMap::new();
    let mut counties: HashMap<String, (String, f64)> = HashMap::new();
    for (cell, row) in cells.iter().zip(rows) {
        let get = |k: &str| row.get(k).map(String::as_str).unwrap_or_default();
        let (st, stname) = (get("st"), get("stname"));
        if st.is_empty() {
            continue;
        }
        states
            .entry(st.to_string())
            .or_insert_with(|| (stname.to_string(), 0.0))
            .1 += cell.rows;
        let (co, coname) = (get("co"), get("coname"));
        if !co.is_empty() {
            let label = if stname.is_empty() {
                coname.to_string()
            } else {
                format!("{coname}, {stname}")
            };
            counties
                .entry(co.to_string())
                .or_insert_with(|| (label, 0.0))
                .1 += cell.rows;
        }
    }
    let mut out = Vec::new();
    let mut notes = Vec::new();
    if total <= 0.0 {
        return (out, notes);
    }
    for (i, (geoid, name, w)) in ranked(states).into_iter().enumerate() {
        if i == 0 || w / total >= REGION_MIN_SHARE {
            out.push(region_location(&name, &geoid));
        }
    }
    let qualifying: Vec<_> = ranked(counties)
        .into_iter()
        .filter(|(.., w)| *w / total >= REGION_MIN_SHARE)
        .collect();
    if qualifying.len() > MAX_COUNTIES {
        notes.push(format!(
            "--locate census: {} counties each hold at least {:.0}% of the sampled rows; only the \
             {MAX_COUNTIES} largest are listed",
            qualifying.len(),
            REGION_MIN_SHARE * 100.0
        ));
    }
    for (geoid, label, _) in qualifying.into_iter().take(MAX_COUNTIES) {
        out.push(region_location(&label, &geoid));
    }
    (out, notes)
}

/// Census Locations from a file's region columns, which `locate` prefers
/// over sampled coordinates whenever they resolve: states from a state column (see
/// [`state_column`]), counties from a county column (see [`county_column_regions`]). With no state
/// column, the states are those of the resolved counties.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
fn column_regions(
    backend: &SqlBackend,
    dppf: &Value,
    headers: &[String],
) -> (Vec<Value>, Vec<String>) {
    let state = state_column(dppf, headers);
    // a state column narrows every row's county even when too few of its
    // values are US states to list states from it directly
    let narrow = state.as_ref().map(|(header, ..)| header.as_str());
    let mut notes = Vec::new();

    // the first county-like column that qualifies; a `county_status`
    // before the real `County` must not hide it
    let mut rejected = Vec::new();
    let mut found = None;
    for county in headers
        .iter()
        .filter(|h| h.to_ascii_lowercase().contains("county"))
    {
        match county_column_regions(backend, county, narrow) {
            Ok(regions) => {
                found = Some(regions);
                break;
            },
            Err(note) => rejected.push(note),
        }
    }
    let (mut states, counties) = match found {
        Some((states, counties, mut more)) => {
            notes.append(&mut more);
            (states, counties)
        },
        None => {
            notes.append(&mut rejected);
            (HashMap::new(), Vec::new())
        },
    };
    if let Some((header, from_column, resolved)) = state
        && resolved >= STATE_COLUMN_MIN
    {
        states = from_column;
        notes.insert(
            0,
            format!("--locate census: states taken from the `{header}` column"),
        );
    }
    if states.is_empty() && counties.is_empty() && notes.is_empty() {
        notes.push(
            "--locate census: no latitude/longitude, state or county column detected".to_string(),
        );
    }
    let total: f64 = states.values().map(|(_, w)| w).sum();
    let mut out: Vec<Value> = ranked(states)
        .into_iter()
        .enumerate()
        .filter(|(i, (.., w))| *i == 0 || *w / total >= REGION_MIN_SHARE)
        .map(|(_, (geoid, name, _))| region_location(&name, &geoid))
        .collect();
    out.extend(counties);
    (out, notes)
}

/// Share of a state column's listed values that must be US states for
/// the column to list the dataset's states.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
const STATE_COLUMN_MIN: f64 = 0.9;
/// The column whose header mentions "state" and the largest share of
/// whose listed frequency (the top values qsv kept) is a USPS code, FIPS
/// code or state name: its header, its states weighted by row count,
/// and that share. Any share above zero makes it a state column, so it
/// narrows every row's county to that row's state — a mostly non-US
/// column then declines county inference rather than matching its rows
/// nationwide — while a header such as `Statement Date`, with no US
/// state at all, is not one.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
fn state_column(
    dppf: &Value,
    headers: &[String],
) -> Option<(String, HashMap<String, (String, f64)>, f64)> {
    use crate::cmd::viz_census::{state_geoid_for_any_spelling, state_name_for_geoid};

    let mut best: Option<(String, HashMap<String, (String, f64)>, f64)> = None;
    for header in headers {
        if !header.to_ascii_lowercase().contains("state") {
            continue;
        }
        let Some(entries) = dppf.get(header).and_then(Value::as_array) else {
            continue;
        };
        let mut listed = 0.0;
        let mut states: HashMap<String, (String, f64)> = HashMap::new();
        for e in entries {
            let value = e.get("value").and_then(Value::as_str).unwrap_or_default();
            let count = e.get("count").and_then(Value::as_f64).unwrap_or(0.0);
            // the frequency summary's own buckets, not data values
            if value.starts_with("Other (") || value.starts_with('<') || value.is_empty() {
                continue;
            }
            listed += count;
            if let Some(geoid) = state_geoid_for_any_spelling(value)
                && let Some(name) = state_name_for_geoid(&geoid)
            {
                states
                    .entry(geoid)
                    .or_insert_with(|| (name.to_string(), 0.0))
                    .1 += count;
            }
        }
        let resolved: f64 = states.values().map(|(_, w)| w).sum();
        if listed <= 0.0 {
            continue;
        }
        let share = resolved / listed;
        if share > 0.0 && best.as_ref().is_none_or(|(.., s)| share > *s) {
            best = Some((header.clone(), states, share));
        }
    }
    best
}

/// `part` as a percentage of `whole`, rounded DOWN to one decimal so a
/// near-total share never reads as 100%, and a sliver never as 0%.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
fn share(part: f64, whole: f64) -> String {
    // `+ 0.0` turns the -0.0 an empty f64 sum yields into 0.0
    let pct = (1000.0 * part / whole).floor() / 10.0 + 0.0;
    if pct < 0.1 && part > 0.0 {
        "under 0.1%".to_string()
    } else {
        format!("{pct:.1}%")
    }
}

/// Share of a county column's rows that must name a US county before
/// any county Location is trusted.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
const COUNTY_MIN_RESOLVED: f64 = 0.5;

/// County Locations from a county column: names ("Allegheny County",
/// "Allegheny") or 5-digit GEOIDs, resolved EXACTLY against the Census
/// county name table `viz` uses — never fuzzily, which turns a county
/// name into a real but wrong county. With a state column, each row's
/// county is resolved within its own row's state (one GROUP BY over the
/// pair), so `King, WA` is King County, Washington; a name several
/// counties share stays unresolved and is noted, never guessed.
///
/// Returns the states of the resolved counties (row-weighted), the
/// county Locations holding at least [`REGION_MIN_SHARE`] of the rows
/// (at most [`MAX_COUNTIES`]), and notes. `Err` is a note: the column
/// could not be read or the name table could not be fetched.
#[cfg(all(feature = "viz", feature = "feature_capable"))]
fn county_column_regions(
    backend: &SqlBackend,
    county: &str,
    state: Option<&str>,
) -> Result<(HashMap<String, (String, f64)>, Vec<Value>, Vec<String>), String> {
    use crate::cmd::viz_census::{
        CountyLookup, county_name_table, state_geoid_for_any_spelling, state_name_for_geoid,
    };

    if county.contains('"') || state.is_some_and(|s| s.contains('"')) {
        return Err(format!(
            "--locate census: the `{county}` column name cannot be queried; no county Locations \
             were added"
        ));
    }
    let backend = backend.clone().with_all_strings();
    let sql = match state {
        Some(st) => format!(
            r#"SELECT "{st}" AS s, "{county}" AS c, COUNT(*) AS n FROM data WHERE "{county}" IS NOT NULL GROUP BY "{st}", "{county}""#
        ),
        None => format!(
            r#"SELECT "{county}" AS c, COUNT(*) AS n FROM data WHERE "{county}" IS NOT NULL GROUP BY "{county}""#
        ),
    };
    let df = backend.query(&sql).map_err(|e| {
        format!(
            "--locate census: could not read the `{county}` column ({e}); no county Locations \
             were added"
        )
    })?;
    let table = county_name_table(None).map_err(|e| {
        format!(
            "--locate census: the Census county name table is unavailable ({e}); no county \
             Locations were added"
        )
    })?;

    let text_at = |c: &str, i: usize| -> Option<String> {
        let col = df.column(c).ok()?;
        let cast = col.cast(&polars::prelude::DataType::String).ok()?;
        cast.str().ok()?.get(i).map(|s| s.trim().to_string())
    };
    let count_at = |i: usize| -> f64 {
        df.column("n")
            .ok()
            .and_then(|c| c.cast(&polars::prelude::DataType::Float64).ok())
            .and_then(|c| c.f64().ok().and_then(|f| f.get(i)))
            .unwrap_or(0.0)
    };

    let mut total = 0.0;
    let mut counties: HashMap<String, (String, f64)> = HashMap::new();
    let mut ambiguous: HashMap<String, (String, f64)> = HashMap::new();
    for i in 0..df.height() {
        let Some(name) = text_at("c", i).filter(|s| !s.is_empty()) else {
            continue;
        };
        let n = count_at(i);
        total += n;
        let state_fips = match state {
            Some(_) => match text_at("s", i)
                .as_deref()
                .and_then(state_geoid_for_any_spelling)
            {
                Some(fips) => Some(fips),
                // a row whose state is not a US state cannot name a US county
                None => continue,
            },
            None => None,
        };
        let geoid = if name.len() == 5
            && name.bytes().all(|b| b.is_ascii_digit())
            && table.name_for_geoid(&name).is_some()
            && state_fips.as_deref().is_none_or(|f| name.starts_with(f))
        {
            Some(name.clone())
        } else {
            match table.resolve(&name, state_fips.as_deref()) {
                CountyLookup::Resolved(geoid) => Some(geoid),
                CountyLookup::Ambiguous(_) => {
                    ambiguous
                        .entry(name.to_ascii_lowercase())
                        .or_insert_with(|| (name.clone(), 0.0))
                        .1 += n;
                    None
                },
                CountyLookup::NoMatch | CountyLookup::StateMismatch(_) => None,
            }
        };
        if let Some(geoid) = geoid {
            let label = table.name_for_geoid(&geoid).unwrap_or(&geoid).to_string();
            counties.entry(geoid).or_insert_with(|| (label, 0.0)).1 += n;
        }
    }

    let resolved: f64 = counties.values().map(|(_, w)| w).sum();
    let unclear: f64 = ambiguous.values().map(|(_, w)| w).sum();
    let ambiguity = (unclear > 0.0).then(|| {
        let examples: Vec<String> = ranked(ambiguous)
            .into_iter()
            .take(3)
            .map(|(_, name, _)| format!("`{name}`"))
            .collect();
        format!(
            "{} of the `{county}` values name more than one county{} (e.g. {})",
            share(unclear, total),
            if state.is_none() {
                " and there is no state column to tell them apart"
            } else {
                ""
            },
            examples.join(", ")
        )
    });
    if total <= 0.0 || resolved / total < COUNTY_MIN_RESOLVED {
        return Err(format!(
            "--locate census: only {} of the `{county}` values name a US county{}; no county \
             Locations were added",
            if total > 0.0 {
                share(resolved, total)
            } else {
                "0%".to_string()
            },
            ambiguity
                .as_deref()
                .map(|a| format!(" ({a})"))
                .unwrap_or_default()
        ));
    }

    let mut notes = vec![format!(
        "--locate census: counties taken from the `{county}` column ({} of its values resolved \
         against the ACS{} county names)",
        share(resolved, total),
        table.vintage
    )];
    if let Some(a) = ambiguity {
        notes.push(format!("--locate census: {a}; they were left out"));
    }

    let mut states: HashMap<String, (String, f64)> = HashMap::new();
    for (geoid, (_, w)) in &counties {
        let Some(fips) = geoid.get(..2) else {
            continue;
        };
        if let Some(name) = state_name_for_geoid(fips) {
            states
                .entry(fips.to_string())
                .or_insert_with(|| (name.to_string(), 0.0))
                .1 += w;
        }
    }
    let qualifying: Vec<_> = ranked(counties)
        .into_iter()
        .filter(|(.., w)| *w / total >= REGION_MIN_SHARE)
        .collect();
    if qualifying.len() > MAX_COUNTIES {
        notes.push(format!(
            "--locate census: {} counties each hold at least {:.0}% of the rows; only the \
             {MAX_COUNTIES} largest are listed",
            qualifying.len(),
            REGION_MIN_SHARE * 100.0
        ));
    }
    let out = qualifying
        .into_iter()
        .take(MAX_COUNTIES)
        .map(|(geoid, name, _)| {
            let label = geoid
                .get(..2)
                .and_then(state_name_for_geoid)
                .map_or_else(|| name.clone(), |st| format!("{name}, {st}"));
            region_location(&label, &geoid)
        })
        .collect();
    Ok((states, out, notes))
}

#[cfg(not(all(feature = "viz", feature = "feature_capable")))]
fn column_regions(
    _backend: &SqlBackend,
    _dppf: &Value,
    _headers: &[String],
) -> (Vec<Value>, Vec<String>) {
    (
        Vec::new(),
        vec![
            "--locate census: no latitude/longitude columns detected (state and county columns \
             are only resolved in builds with the `viz` feature)"
                .to_string(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(rows: f64) -> Cell {
        Cell {
            lat: 42.0,
            lon: -71.0,
            rows,
        }
    }

    fn row(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    fn boston(gid: &str, place: &str) -> HashMap<String, String> {
        row(&[
            ("gid", gid),
            ("place", place),
            ("admin1", "Massachusetts"),
            ("cc", "US"),
            ("country", "United States"),
            ("cgid", "6252001"),
        ])
    }

    #[test]
    fn geonames_labels_the_place_most_rows_share() {
        let cells = [cell(90.0), cell(5.0), cell(5.0)];
        let rows = [
            boston("4930956", "Boston"),
            boston("4931972", "Cambridge"),
            boston("4930956", "Boston"),
        ];
        let label = geonames_label(&cells, &rows).unwrap();
        assert_eq!(label["prefLabel"], "Boston, Massachusetts, United States");
        assert_eq!(label["identifier"], "https://sws.geonames.org/4930956/");
    }

    #[test]
    fn geonames_split_between_places_falls_back_to_admin1() {
        // 60/40 between two places: neither reaches 80%, their state does
        let cells = [cell(60.0), cell(40.0)];
        let rows = [boston("4930956", "Boston"), boston("4931972", "Cambridge")];
        let label = geonames_label(&cells, &rows).unwrap();
        assert_eq!(label["prefLabel"], "Massachusetts, United States");
        assert!(label.get("identifier").is_none());
    }

    #[test]
    fn geonames_weights_by_rows_not_cells() {
        // one dense cell outweighs three sparse ones
        let cells = [cell(97.0), cell(1.0), cell(1.0), cell(1.0)];
        let mut rows = vec![boston("4930956", "Boston")];
        for _ in 0..3 {
            rows.push(boston("4931972", "Cambridge"));
        }
        let label = geonames_label(&cells, &rows).unwrap();
        assert_eq!(label["prefLabel"], "Boston, Massachusetts, United States");
    }

    #[test]
    fn geonames_country_level_uses_the_country_geonameid() {
        let cells = [cell(50.0), cell(50.0)];
        let mut ny = boston("5128581", "New York City");
        ny.insert("admin1".into(), "New York".into());
        let rows = [boston("4930956", "Boston"), ny];
        let label = geonames_label(&cells, &rows).unwrap();
        assert_eq!(label["prefLabel"], "United States");
        assert_eq!(label["identifier"], "https://sws.geonames.org/6252001/");
    }

    #[test]
    fn geonames_no_shared_country_is_no_label() {
        let cells = [cell(50.0), cell(50.0)];
        let mut paris = boston("2988507", "Paris");
        paris.insert("cc".into(), "FR".into());
        let rows = [boston("4930956", "Boston"), paris];
        assert!(geonames_label(&cells, &rows).is_none());
    }

    fn county(st: &str, stname: &str, co: &str, coname: &str) -> HashMap<String, String> {
        row(&[
            ("st", st),
            ("stname", stname),
            ("co", co),
            ("coname", coname),
        ])
    }

    #[test]
    fn census_regions_drop_a_stray_state_and_small_counties() {
        let cells = [cell(80.0), cell(17.0), cell(2.0), cell(1.0)];
        let rows = [
            county("25", "Massachusetts", "25025", "Suffolk County"),
            county("25", "Massachusetts", "25017", "Middlesex County"),
            county("25", "Massachusetts", "25021", "Norfolk County"),
            county("33", "New Hampshire", "33011", "Hillsborough County"),
        ];
        let (regions, _) = census_regions(&cells, &rows);
        let labels: Vec<&str> = regions
            .iter()
            .map(|r| r["prefLabel"].as_str().unwrap())
            .collect();
        assert_eq!(
            labels,
            [
                "Massachusetts",
                "Suffolk County, Massachusetts",
                "Middlesex County, Massachusetts"
            ]
        );
        assert_eq!(regions[1]["identifier"], "25025");
        assert_eq!(regions[1]["inScheme"]["@type"], "ConceptScheme");
    }

    #[test]
    fn census_regions_skip_unmatched_points() {
        let cells = [cell(10.0), cell(90.0)];
        let rows = [
            county("25", "Massachusetts", "25025", "Suffolk County"),
            row(&[("st", "")]),
        ];
        let (regions, _) = census_regions(&cells, &rows);
        // the heaviest matched state is always kept; the county needs 5% of ALL sampled rows
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0]["identifier"], "25");
    }

    #[test]
    fn census_regions_cap_counties() {
        let cells: Vec<Cell> = (0..12).map(|_| cell(1.0)).collect();
        let rows: Vec<_> = (0..12)
            .map(|i| county("36", "New York", &format!("36{i:03}"), &format!("C{i}")))
            .collect();
        let (regions, notes) = census_regions(&cells, &rows);
        assert_eq!(regions.len(), 1 + MAX_COUNTIES);
        assert!(notes[0].contains("12 counties"), "{notes:?}");
    }

    #[test]
    fn parse_lookup_csv_keeps_quoted_points() {
        let rows = parse_lookup_csv("qsv_locate_point,st\n\"42.1, -71.1\",25\n\"0.5, 0.5\",\n");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["st"], "25");
        assert_eq!(rows[1]["st"], "");
    }

    #[test]
    fn locate_mode_parses_case_insensitively() {
        assert_eq!(LocateMode::parse("Census").unwrap(), LocateMode::Census);
        assert!(LocateMode::parse("tiger").is_err());
    }

    #[cfg(all(feature = "viz", feature = "feature_capable"))]
    #[test]
    fn state_column_needs_a_state_header_and_resolvable_values() {
        let dppf = json!({
            "code":  [{"value": "MA", "count": 50}, {"value": "NY", "count": 50}],
            "state": [
                {"value": "MA", "count": 70},
                {"value": "New York", "count": 27},
                {"value": "Other (3)", "count": 3}
            ],
        });
        let headers = ["code".to_string(), "state".to_string()];
        let (regions, notes) = column_regions(&SqlBackend::new("unused.csv"), &dppf, &headers);
        let ids: Vec<&str> = regions
            .iter()
            .map(|r| r["identifier"].as_str().unwrap())
            .collect();
        assert_eq!(ids, ["25", "36"]);
        assert_eq!(regions[1]["prefLabel"], "New York");
        assert!(notes[0].contains("`state`"), "{notes:?}");

        // island areas resolve too: an 11% Guam share must not sink the column
        let territory =
            json!({ "state": [{"value": "GU", "count": 11}, {"value": "MA", "count": 89}] });
        let (regions, _) = column_regions(
            &SqlBackend::new("unused.csv"),
            &territory,
            &["state".to_string()],
        );
        let ids: Vec<&str> = regions
            .iter()
            .map(|r| r["identifier"].as_str().unwrap())
            .collect();
        assert_eq!(ids, ["25", "66"]);
        assert_eq!(regions[1]["prefLabel"], "Guam");

        let unresolved =
            json!({ "state": [{"value": "open", "count": 9}, {"value": "MA", "count": 1}] });
        let (regions, _) = column_regions(
            &SqlBackend::new("unused.csv"),
            &unresolved,
            &["state".to_string()],
        );
        assert!(regions.is_empty());
    }
}
