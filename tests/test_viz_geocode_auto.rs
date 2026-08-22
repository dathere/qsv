//! Tests for `viz --geojson auto --geocode`: a `--locations` column of city/place NAMES
//! forward-geocoded to US county FIPS, then resolved against the mock `TIGERweb` service
//! (issue #4417 Part A).
//!
//! Hermetic on BOTH network seams: `TIGERweb` is mocked via `QSV_CENSUS_TIGERWEB_URL` (shared
//! with `test_viz_census`), and the Geonames index is built AT TEST TIME from the tiny committed
//! TSVs under `resources/test/geonames-mini/` — never downloaded, and never committed as a
//! prebuilt `.rkyv` (the rkyv layout is coupled to the geosuggest-core version; building the
//! fixture with the Cargo.lock-unified crate removes that coupling by construction). The index
//! file must exist before a run, or qsv's first-use path would download the real prebuilt.

use serial_test::serial;

use crate::{test_viz_census::with_mock_tigerweb, workdir::Workdir};

/// The index filename each test points `QSV_GEOCODE_INDEX_FILENAME` at.
const MINI_INDEX: &str = "qsv-geocode-mini.rkyv";

/// Build the mini Geonames index into `<wrk cache dir>/qsv-geocode-mini.rkyv` and return the
/// cache dir path (shared with the boundary cache, which lives in a subdirectory of it).
fn build_mini_geocode_index(wrk: &Workdir) -> String {
    let cache_dir = wrk.path("qsv-cache");
    std::fs::create_dir_all(&cache_dir).expect("create cache dir");
    let mini =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/geonames-mini");
    let index_data = geosuggest_core::index::IndexData::new_from_files(
        geosuggest_core::index::SourceFileOptions {
            cities:           mini.join("cities.txt"),
            // localized names, like the published prebuilt (built English-only): without them
            // admin1/country names come back empty and the hint-rejection exemplars degrade to
            // bare ISO codes
            names:            Some(mini.join("alternateNames.txt")),
            countries:        Some(mini.join("countryInfo.txt")),
            admin1_codes:     Some(mini.join("admin1CodesASCII.txt")),
            admin2_codes:     Some(mini.join("admin2Codes.txt")),
            filter_languages: vec!["en"],
        },
    )
    .expect("parse mini Geonames fixture");
    let engine_data =
        geosuggest_core::EngineData::try_from(index_data).expect("serialize mini index");
    // Write the storage's `<4-byte metadata len BE><metadata><payload>` framing BY HAND:
    // `Storage::dump_to` without geosuggest's `tracing` feature writes the metadata LENGTH but
    // not the metadata bytes (a misplaced `#[cfg]`), shifting the payload and making the index
    // unloadable — the very corruption qsv issue #4433 works around by never re-serializing.
    let metadata = rkyv::to_bytes::<rkyv::rancor::Error>(&engine_data.metadata)
        .expect("serialize mini index metadata");
    let mut bytes: Vec<u8> = Vec::with_capacity(4 + metadata.len() + engine_data.data.len());
    bytes.extend_from_slice(&u32::try_from(metadata.len()).unwrap().to_be_bytes());
    bytes.extend_from_slice(&metadata);
    bytes.extend_from_slice(&engine_data.data);
    std::fs::write(cache_dir.join(MINI_INDEX), bytes).expect("write mini index");
    cache_dir.to_string_lossy().to_string()
}

/// A `viz choropleth --geojson auto --geocode` command wired to the mock service, the mini
/// index, and `cache_dir`.
fn geocode_auto_cmd(
    wrk: &Workdir,
    base: &str,
    cache_dir: &str,
    csv: &str,
) -> std::process::Command {
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        csv,
        "--locations",
        "city",
        "--value",
        "cases",
        "--geojson",
        "auto",
        "--geocode",
    ])
    .env("QSV_CENSUS_TIGERWEB_URL", base)
    .env("QSV_CACHE_DIR", cache_dir)
    .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
    cmd
}

// The Part A happy path: nothing supplied but the CSV — city names forward-geocode to their
// containing counties (Pittsburgh -> 42003, Philadelphia -> 42101), the fetch is scoped to the
// counties' state, and the trace keys by county FIPS.
#[test]
#[serial]
fn viz_geojson_auto_geocode_resolves_city_names() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_resolves_city_names");
    wrk.create_from_string(
        "cities.csv",
        "city,cases\nPittsburgh,10\nPhiladelphia,20\nPittsburgh,5\n",
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "geocoded auto failed: {stderr}");

        // the fetch is scoped to the state the geocoded counties belong to
        let clauses = observed.where_clauses.lock().unwrap().clone();
        let state_scoped: Vec<&String> = clauses
            .iter()
            .filter(|c| c.starts_with("STATE IN"))
            .collect();
        assert!(
            state_scoped.iter().all(|c| c.as_str() == "STATE IN ('42')"),
            "fetch was not scoped to the geocoded counties' state: {clauses:?}"
        );
        assert!(
            !state_scoped.is_empty(),
            "no state-scoped query: {clauses:?}"
        );

        // the trace keys by county FIPS, not by the city names
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("42003"),
            "Allegheny county missing from trace"
        );
        assert!(
            html.contains("42101"),
            "Philadelphia county missing from trace"
        );

        // the provenance names the lineage: these codes were geocoded, not supplied
        assert!(
            stderr.contains("forward-geocoded from --locations place names"),
            "provenance does not mention geocoding: {stderr}"
        );
    });
}

// The --geocode-admin1 hint column disambiguates a name that exists in many states: without the
// hint the fixture's Springfield IL (more populous) wins; the US.MA hint forces Hampden County MA.
#[test]
#[serial]
fn viz_geojson_auto_geocode_admin1_hint_disambiguates() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_admin1_hint_disambiguates");
    wrk.create_from_string(
        "cities.csv",
        "city,state,cases\nSpringfield,US.MA,10\nPittsburgh,US.PA,20\n",
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
        cmd.args(["--geocode-admin1", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            out.status.success(),
            "hinted geocoded auto failed: {stderr}"
        );

        // the fetch spans BOTH hinted states — proving Springfield resolved to MA (25), not IL (17)
        let clauses = observed.where_clauses.lock().unwrap().clone();
        let state_scoped: Vec<&String> = clauses
            .iter()
            .filter(|c| c.starts_with("STATE IN"))
            .collect();
        assert!(
            state_scoped
                .iter()
                .all(|c| c.contains("'25'") && c.contains("'42'") && !c.contains("'17'")),
            "hint did not steer Springfield to Massachusetts: {clauses:?}"
        );
        assert!(
            !state_scoped.is_empty(),
            "no state-scoped query: {clauses:?}"
        );

        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("25013"),
            "Hampden County MA missing from trace"
        );
    });
}

// A name the mini index cannot resolve is reported with the per-cause breakdown AND the denser
// prebuilt remedy — an under-resolving index must read as an instruction, not a sparse map.
#[test]
#[serial]
fn viz_geojson_auto_geocode_reports_unresolved_names() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_reports_unresolved_names");
    wrk.create_from_string(
        "cities.csv",
        "city,cases\nPittsburgh,10\nPhiladelphia,20\nErewhon,30\n",
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        // 2 of 3 resolve, above the coverage gate: the map still renders, with the note
        assert!(
            out.status.success(),
            "partially-resolved run failed: {stderr}"
        );
        assert!(
            stderr.contains("forward-geocoded 2 of 3"),
            "resolution count missing: {stderr}"
        );
        assert!(
            stderr.contains("matched no known place"),
            "no-match cause missing: {stderr}"
        );
        assert!(
            stderr.contains("qsv geocode index-load 1000"),
            "denser-index remedy missing: {stderr}"
        );
    });
}

// A match outside the implicit `us` hint (London -> GB) is a hint REJECTION: reported with where
// the match actually was, never silently dropped, and never charted as a wrong county.
#[test]
#[serial]
fn viz_geojson_auto_geocode_reports_hint_rejections() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_reports_hint_rejections");
    wrk.create_from_string(
        "cities.csv",
        "city,cases\nPittsburgh,10\nPhiladelphia,20\nLondon,30\n",
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            out.status.success(),
            "run with a rejected name failed: {stderr}"
        );
        assert!(
            stderr.contains("matched a place outside the hint"),
            "hint-rejection cause missing: {stderr}"
        );
        assert!(
            stderr.contains("London matched England, GB"),
            "rejection exemplar missing: {stderr}"
        );
        // a rejection is not an index-resolution problem, so the remedy must NOT be advertised
        assert!(
            !stderr.contains("index-load 1000"),
            "denser-index remedy wrongly advertised for a hint rejection: {stderr}"
        );
    });
}

// When nothing resolves there are no counties to fetch: a hard error carrying the breakdown,
// before any TIGERweb request.
#[test]
#[serial]
fn viz_geojson_auto_geocode_nothing_resolves_errors() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_nothing_resolves_errors");
    wrk.create_from_string("cities.csv", "city,cases\nErewhon,10\nBrigadoon,20\n");
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(!out.status.success(), "resolving nothing must fail");
        assert!(
            stderr.contains("resolved none of the 2 distinct"),
            "hard error missing the resolution count: {stderr}"
        );
        assert!(
            stderr.contains("qsv geocode index-load 1000"),
            "denser-index remedy missing from the hard error: {stderr}"
        );
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            0,
            "no boundary request may be issued when nothing geocoded"
        );
    });
}

// Without --geocode, an all-names column is pre-empted BEFORE any network with the actionable
// suggestion — the dead end turned into the exact next command.
#[test]
#[serial]
fn viz_geojson_auto_without_geocode_suggests_it() {
    let wrk = Workdir::new("viz_geojson_auto_without_geocode_suggests_it");
    wrk.create_from_string("cities.csv", "city,cases\nPittsburgh,10\nPhiladelphia,20\n");
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "cities.csv",
            "--locations",
            "city",
            "--value",
            "cases",
            "--geojson",
            "auto",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env("QSV_CACHE_DIR", &cache_dir)
        .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            !out.status.success(),
            "an all-names column without --geocode must fail"
        );
        assert!(
            stderr.contains("add --geocode"),
            "the error does not suggest --geocode: {stderr}"
        );
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            0,
            "the pre-empt must fire before any network request"
        );
    });
}

// The geocoded path caches by the SYNTHESIZED county FIPS: a second identical run geocodes
// locally and reuses the cached boundaries, issuing zero requests.
#[test]
#[serial]
fn viz_geojson_auto_geocode_second_run_makes_no_requests() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_second_run_makes_no_requests");
    wrk.create_from_string("cities.csv", "city,cases\nPittsburgh,10\nPhiladelphia,20\n");
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let run = || {
            let mut cmd = geocode_auto_cmd(&wrk, base, &cache_dir, "cities.csv");
            let out = wrk.output(&mut cmd);
            assert!(
                out.status.success(),
                "geocoded auto failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        run();
        let after_first = observed.requests.load(std::sync::atomic::Ordering::SeqCst);
        assert!(after_first > 0, "first run must reach the mock service");
        run();
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            after_first,
            "second run must issue no requests at all"
        );
    });
}

// --geocode synthesizes COUNTY FIPS, so a pinned non-county layer cannot be served; refuse
// rather than fetch a geography the codes cannot match. No index or network needed to decide.
#[test]
#[serial]
fn viz_geojson_auto_geocode_rejects_non_county_layer() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_rejects_non_county_layer");
    wrk.create_from_string("cities.csv", "city,cases\nPittsburgh,10\n");
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "cities.csv",
            "--locations",
            "city",
            "--geojson",
            "census:zcta",
            "--geocode",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env("QSV_CACHE_DIR", &cache_dir)
        .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(!out.status.success(), "census:zcta + --geocode must fail");
        assert!(
            stderr.contains("census:county"),
            "the error does not name the layers --geocode can serve: {stderr}"
        );
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            0,
            "the layer rejection must fire before any network request"
        );
    });
}

// Reverse geocoding returns the NEAREST populated place, not the containing county — the very
// approximation issue #4417 rules out — so lat/lon is rejected on this path, with the reason.
#[test]
#[serial]
fn viz_geojson_auto_geocode_rejects_latlon() {
    let wrk = Workdir::new("viz_geojson_auto_geocode_rejects_latlon");
    wrk.create_from_string(
        "points.csv",
        "city,lat,lon,cases\nPittsburgh,40.44,-79.99,10\n",
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "points.csv",
            "--locations",
            "city",
            "--lat",
            "lat",
            "--lon",
            "lon",
            "--geojson",
            "auto",
            "--geocode",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env("QSV_CACHE_DIR", &cache_dir)
        .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            !out.status.success(),
            "--lat/--lon + --geojson auto --geocode must fail"
        );
        assert!(
            stderr.contains("NEAREST populated place"),
            "the rejection does not explain the nearest-place trap: {stderr}"
        );
    });
}

/// Dictionary tagging a lone `geo.city` column — the smart-path acceptance case: nothing
/// supplied but the CSV and its dictionary.
const CITY_DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "city": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.city" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;

// The issue's acceptance criterion on the smart path: a city-name column resolves to county
// boundaries with nothing supplied but the CSV (and its dictionary) — forward-geocoded, fetched,
// and charted keyed by county FIPS.
#[test]
#[serial]
fn viz_smart_geojson_auto_geocode_city_column_resolves() {
    let wrk = Workdir::new("viz_smart_geojson_auto_geocode_city_column_resolves");
    wrk.create_from_string(
        "cities.csv",
        "city,cases\nPittsburgh,10\nPittsburgh,20\nPhiladelphia,30\nPhiladelphia,40\n",
    );
    wrk.create_from_string("dict.schema.json", CITY_DICT);
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "cities.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir)
            .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "smart geocoded auto failed: {stderr}");

        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains(r#""featureidkey":"properties.GEOID""#),
            "expected the auto-resolved feature-id key"
        );
        assert!(
            html.contains("count by city"),
            "expected the summary choropleth panel keyed by the city column"
        );
        assert!(
            stderr.contains("forward-geocoded from place names"),
            "provenance does not carry the geocode lineage: {stderr}"
        );
        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(
            clauses.iter().any(|c| c == "STATE IN ('42')"),
            "fetch was not scoped to the geocoded counties' state: {clauses:?}"
        );
    });
}

// A real code column must always beat a city column: codes are exact, geocoding is a guess.
// The city column here would resolve fine — it must simply never be consulted.
#[test]
#[serial]
fn viz_smart_geojson_auto_geocode_code_column_beats_city() {
    let wrk = Workdir::new("viz_smart_geojson_auto_geocode_code_column_beats_city");
    wrk.create_from_string(
        "mixed.csv",
        "fips,city,cases\n42003,Pittsburgh,10\n42003,Pittsburgh,20\n42101,Philadelphia,30\n42101,\
         Philadelphia,40\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "city": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.city" } },
            "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "mixed.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir)
            .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "smart run failed: {stderr}");

        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("count by fips"),
            "the code column must key the panel"
        );
        // no geocoding happened: the provenance carries no geocode lineage
        assert!(
            !stderr.contains("forward-geocoded"),
            "geocoding ran although a code column resolved: {stderr}"
        );
    });
}

// The fall-through inherited from issue #4416, extended across kinds: when every CODE candidate
// fails (here a decoy of well-formed but nonexistent codes), a city column is the next guess —
// qsv chose the column, so a bad guess costs another attempt, not the Data Schematic.
#[test]
#[serial]
fn viz_smart_geojson_auto_geocode_falls_through_to_city() {
    let wrk = Workdir::new("viz_smart_geojson_auto_geocode_falls_through_to_city");
    wrk.create_from_string(
        "mixed.csv",
        "decoy,city,cases\n99998,Pittsburgh,10\n99998,Pittsburgh,20\n99999,Philadelphia,30\n99999,\
         Philadelphia,40\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "decoy": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.zip_code" } },
            "city": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.city" } },
            "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "mixed.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir)
            .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            out.status.success(),
            "fall-through to the city column failed: {stderr}"
        );

        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("count by city"),
            "the city column must key the panel after the decoy failed"
        );
        assert!(
            stderr.contains("forward-geocoded from place names"),
            "provenance does not carry the geocode lineage: {stderr}"
        );
    });
}

// The inverse fall-through: a city column whose names geocode to nothing degrades to the
// existing failure report — never a panic, never a hard geocode error mid-candidate.
#[test]
#[serial]
fn viz_smart_geojson_auto_geocode_unresolvable_city_degrades() {
    let wrk = Workdir::new("viz_smart_geojson_auto_geocode_unresolvable_city_degrades");
    wrk.create_from_string(
        "cities.csv",
        "city,cases\nErewhon,10\nErewhon,20\nBrigadoon,30\nBrigadoon,40\n",
    );
    wrk.create_from_string("dict.schema.json", CITY_DICT);
    let cache_dir = build_mini_geocode_index(&wrk);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "cities.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir)
            .env("QSV_GEOCODE_INDEX_FILENAME", MINI_INDEX);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            !out.status.success(),
            "an unresolvable city column must fail the auto resolve"
        );
        assert!(
            stderr.contains("resolved none of the 2 distinct place names"),
            "the failure does not carry the geocode diagnosis: {stderr}"
        );
    });
}
