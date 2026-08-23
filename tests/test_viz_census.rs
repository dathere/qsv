//! Request-level tests for `viz --geojson auto` against a mock `TIGERweb` service.
//!
//! These assert things that cannot be observed from the rendered output: which states were
//! actually requested, and that a repeated run issues NO request at all. Pointing qsv at a local
//! mock (via `QSV_CENSUS_TIGERWEB_URL`) rather than the live Census service also keeps the suite
//! hermetic — the assertions are about qsv's behavior, not the Bureau's uptime.

use std::{
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::ServerHandle, rt, web};
use serial_test::serial;

use crate::workdir::Workdir;

const BIND_HOST: &str = "127.0.0.1";

/// Read one percent-encoded query parameter. Hand-rolled rather than pulled from a crate so the
/// test does not depend on an optional dependency being enabled by the feature set under test.
fn query_param(query: &str, want: &str) -> String {
    for pair in query.split('&') {
        let Some((k, v)) = pair.split_once('=') else {
            continue;
        };
        if k != want {
            continue;
        }
        let bytes = v.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'%' if i + 2 < bytes.len() => {
                    if let Ok(b) = u8::from_str_radix(&v[i + 1..i + 3], 16) {
                        out.push(b);
                        i += 3;
                    } else {
                        out.push(bytes[i]);
                        i += 1;
                    }
                },
                b'+' => {
                    out.push(b' ');
                    i += 1;
                },
                b => {
                    out.push(b);
                    i += 1;
                },
            }
        }
        return String::from_utf8_lossy(&out).into_owned();
    }
    String::new()
}

/// What the mock observed: how many requests arrived, and every `where` clause it was asked for.
#[derive(Clone, Default)]
pub(crate) struct Observed {
    pub(crate) requests:      Arc<AtomicUsize>,
    pub(crate) where_clauses: Arc<Mutex<Vec<String>>>,
}

/// The service catalog: one ACS vintage.
async fn serve_catalog(o: web::Data<Observed>) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok().json(serde_json::json!({
        "services": [
            {"name": "TIGERweb/tigerWMS_ACS2023", "type": "MapServer"},
            {"name": "TIGERweb/tigerWMS_ACS2019", "type": "MapServer"}
        ]
    }))
}

/// Layer catalog. Deliberately uses ids that are NOT the live service's, so a test that passes
/// here could not be relying on a hardcoded id.
async fn serve_mapserver(o: web::Data<Observed>) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok().json(serde_json::json!({
        "layers": [
            {"id": 77, "name": "Counties"},
            {"id": 88, "name": "2020 Census ZIP Code Tabulation Areas"},
            {"id": 99, "name": "Census Tracts"}
        ]
    }))
}

/// Build a county feature for a GEOID, with a tiny square polygon.
fn county_feature(geoid: &str, x: f64) -> serde_json::Value {
    serde_json::json!({
        "type": "Feature",
        // AREALAND in square metres, as TIGERweb publishes it — the fetch declares that unit so
        // `--denominator-key properties.AREALAND` needs no --denominator-unit (issue #4414)
        "properties": {"GEOID": geoid, "NAME": format!("County {geoid}"), "AREALAND": 1_500_000_000_i64},
        "geometry": {
            "type": "Polygon",
            "coordinates": [[[x, 0.0], [x, 1.0], [x + 1.0, 1.0], [x + 1.0, 0.0], [x, 0.0]]]
        }
    })
}

/// The county layer. Records the `where` clause, and returns the two Pennsylvania counties the
/// fixture uses whenever state 42 is requested.
async fn serve_county_query(o: web::Data<Observed>, req: HttpRequest) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    let where_clause = query_param(req.query_string(), "where");
    o.where_clauses.lock().unwrap().push(where_clause.clone());

    // State 06 exists as far as a geometry-free PROBE is concerned, but its geometry fetch comes
    // back empty. That split lets a test drive a candidate column that ranks well and then cannot
    // be fetched — the fall-through case in `resolve_smart_auto_geojson`. Empty rather than a 5xx
    // on purpose: a 5xx is classified TRANSIENT and must abort the whole run (a service outage is
    // not a reason to silently draw a different column's map), so it would model the wrong thing.
    // geometry-free probe vs. the real fetch — both answer from the same fixture
    // State 12 answers probes but its geometry fetch is UNWELL (5xx -> transient). Distinct from
    // state 06's empty-but-healthy answer, so the two failure classes can be told apart.
    if where_clause.starts_with("STATE IN") && where_clause.contains("12") {
        return HttpResponse::InternalServerError().body("boom");
    }
    let features = if where_clause.contains("12") {
        vec![county_feature("12086", 12.0), county_feature("12099", 14.0)]
    } else if where_clause.contains("06") {
        if where_clause.starts_with("GEOID IN") {
            vec![county_feature("06001", 8.0), county_feature("06075", 10.0)]
        } else {
            vec![]
        }
    } else if where_clause.contains("42") {
        let mut features = vec![county_feature("42003", 0.0), county_feature("42101", 2.0)];
        // a geocode-hinted Springfield MA resolves to Hampden County; serve it whenever state 25
        // rides along with 42 (the geocode-auto tests span both states in one fetch)
        if where_clause.contains("25") {
            features.push(county_feature("25013", 4.0));
        }
        features
    } else if where_clause.contains("25") {
        vec![county_feature("25013", 4.0)]
    } else {
        vec![]
    };
    HttpResponse::Ok().json(serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    }))
}

/// The ZCTA and tract layers never match this fixture's codes, so the probe rejects them.
async fn serve_empty_query(o: web::Data<Observed>, req: HttpRequest) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    o.where_clauses
        .lock()
        .unwrap()
        .push(query_param(req.query_string(), "where"));
    HttpResponse::Ok().json(serde_json::json!({
        "type": "FeatureCollection",
        "features": []
    }))
}

/// The Census DATA API (issue #4395) — a different service from TIGERweb: tabular ACS estimates,
/// returned as an array of arrays whose first row names the columns.
async fn serve_acs(o: web::Data<Observed>, req: HttpRequest) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    let q = req.query_string();
    o.where_clauses
        .lock()
        .unwrap()
        .push(format!("acs?{}", query_param(q, "for")));
    // an API key must never be required — and when one IS set, it must arrive
    let for_clause = query_param(q, "for");
    if for_clause.starts_with("county") {
        HttpResponse::Ok().json(serde_json::json!([
            ["B01003_001E", "NAME", "state", "county"],
            ["1250578", "Allegheny County, Pennsylvania", "42", "003"],
            ["1603797", "Philadelphia County, Pennsylvania", "42", "101"]
        ]))
    } else {
        HttpResponse::Ok().json(serde_json::json!([
            ["B01003_001E", "NAME", "state"],
            ["12989208", "Pennsylvania", "42"],
            ["19571216", "New York", "36"]
        ]))
    }
}

/// What the Bureau actually serves for a missing or invalid key: a 302 to `missing_key.html`,
/// which answers **HTTP 200 with HTML**. Not an error status — which is precisely why the raw
/// failure is "response is not valid JSON" rather than anything about credentials.
async fn serve_missing_key(o: web::Data<Observed>) -> HttpResponse {
    o.requests.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<html><head><title>Missing Key</title></head><body>Missing Key</body></html>")
}

async fn run_webserver(
    tx: std::sync::mpsc::Sender<Result<(ServerHandle, SocketAddr), String>>,
    observed: Observed,
) -> std::io::Result<()> {
    let server_builder = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(observed.clone()))
            .service(web::resource("/TIGERweb").to(serve_catalog))
            .service(web::resource("/TIGERweb/tigerWMS_ACS2023/MapServer").to(serve_mapserver))
            .service(web::resource("/TIGERweb/tigerWMS_ACS2019/MapServer").to(serve_mapserver))
            .service(
                web::resource("/TIGERweb/tigerWMS_ACS2023/MapServer/77/query")
                    .to(serve_county_query),
            )
            .service(
                web::resource("/TIGERweb/tigerWMS_ACS2023/MapServer/88/query")
                    .to(serve_empty_query),
            )
            .service(
                web::resource("/TIGERweb/tigerWMS_ACS2023/MapServer/99/query")
                    .to(serve_empty_query),
            )
            // the Data API lives under its own root; only 2023 is "published" here, so the
            // vintage probe has to walk back to find it
            .service(web::resource("/data/2023/acs/acs5").to(serve_acs))
            // a vintage that answers like an unkeyed request, so the key hint can be observed
            .service(web::resource("/data/2019/acs/acs5").to(serve_missing_key))
    });
    let bound = match server_builder.bind((BIND_HOST, 0)) {
        Ok(b) => b,
        Err(e) => {
            let _ = tx.send(Err(format!("bind failed: {e}")));
            return Err(e);
        },
    };
    let addr = match bound.addrs().into_iter().next() {
        Some(a) => a,
        None => {
            let _ = tx.send(Err("bind succeeded but no address reported".to_string()));
            return Err(std::io::Error::other("addrs() empty"));
        },
    };
    let server = bound.run();
    let _ = tx.send(Ok((server.handle(), addr)));
    server.await
}

/// Start the mock, run `f` with its base URL, then shut it down.
pub(crate) fn with_mock_tigerweb<F: FnOnce(&str, &Observed)>(f: F) {
    let observed = Observed::default();
    let server_observed = observed.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let handle =
        thread::spawn(move || rt::System::new().block_on(run_webserver(tx, server_observed)));
    let (server_handle, addr) = rx
        .recv()
        .expect("mock server thread died")
        .expect("mock server failed to bind");

    let base = format!("http://{BIND_HOST}:{}", addr.port());
    f(&base, &observed);

    rt::System::new().block_on(server_handle.stop(true));
    let _ = handle.join();
}

// `--geojson auto` must request ONLY the states the data names. A nationwide fetch is the failure
// this guards: the ZCTA layer alone runs to hundreds of MB, and a national county layer is a
// pointless download for a two-county dataset. Also proves the layer id is read from the catalog
// rather than hardcoded — the mock serves Counties as layer 77, which the live service never uses.
#[test]
#[serial]
fn viz_geojson_auto_scopes_the_fetch_to_the_states_present() {
    let wrk = Workdir::new("viz_geojson_auto_scopes_the_fetch_to_the_states_present");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,10\n42101,20\n");

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "pa.csv",
            "--locations",
            "fips",
            "--value",
            "cases",
            "--location-mode",
            "geojson-id",
            "--geojson",
            "auto",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env(
            "QSV_CACHE_DIR",
            wrk.path("boundary-cache").to_string_lossy().to_string(),
        );
        let out = wrk.output(&mut cmd);
        assert!(out.status.success(), "auto resolution failed");

        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(!clauses.is_empty(), "no query reached the service");

        // the geometry fetch is state-scoped, and to state 42 ONLY
        let state_scoped: Vec<&String> = clauses
            .iter()
            .filter(|c| c.starts_with("STATE IN"))
            .collect();
        assert!(
            !state_scoped.is_empty(),
            "no state-scoped county query was issued, got: {clauses:?}"
        );
        for clause in state_scoped {
            assert_eq!(
                clause, "STATE IN ('42')",
                "fetch was not scoped to the data's states"
            );
        }
        // nothing may ask for every feature
        assert!(
            !clauses
                .iter()
                .any(|c| c.trim() == "1=1" || c.trim().is_empty()),
            "a nationwide (unfiltered) query was issued: {clauses:?}"
        );
    });
}

// The issue's "second run = zero network" criterion, asserted rather than timed: the second
// invocation must not reach the service at all — not even for the service catalog.
#[test]
#[serial]
fn viz_geojson_auto_second_run_makes_no_requests() {
    let wrk = Workdir::new("viz_geojson_auto_second_run_makes_no_requests");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,10\n42101,20\n");
    let cache = wrk.path("boundary-cache").to_string_lossy().to_string();

    with_mock_tigerweb(|base, observed| {
        let run = || {
            let mut cmd = wrk.command("viz");
            cmd.args([
                "choropleth",
                "pa.csv",
                "--locations",
                "fips",
                "--value",
                "cases",
                "--location-mode",
                "geojson-id",
                "--geojson",
                "auto",
            ])
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache);
            let out = wrk.output(&mut cmd);
            assert!(out.status.success());
        };

        run();
        let after_first = observed.requests.load(Ordering::SeqCst);
        assert!(after_first > 0, "the first run should have fetched");

        run();
        assert_eq!(
            observed.requests.load(Ordering::SeqCst),
            after_first,
            "the second run must be served entirely from cache"
        );
    });
}

// A cache entry is keyed on the inputs, so a DIFFERENT dataset must not be served the previous
// one's boundaries — the failure mode a key built from a summary (rather than the scope itself)
// would have introduced.
#[test]
#[serial]
fn viz_geojson_auto_different_codes_refetch() {
    let wrk = Workdir::new("viz_geojson_auto_different_codes_refetch");
    wrk.create_from_string("a.csv", "fips,cases\n42003,10\n42101,20\n");
    wrk.create_from_string("b.csv", "fips,cases\n42003,10\n");
    let cache = wrk.path("boundary-cache").to_string_lossy().to_string();

    with_mock_tigerweb(|base, observed| {
        let run = |file: &str| {
            let mut cmd = wrk.command("viz");
            cmd.args([
                "choropleth",
                file,
                "--locations",
                "fips",
                "--value",
                "cases",
                "--location-mode",
                "geojson-id",
                "--geojson",
                "auto",
            ])
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache);
            let out = wrk.output(&mut cmd);
            assert!(out.status.success());
        };

        run("a.csv");
        let after_first = observed.requests.load(Ordering::SeqCst);
        run("b.csv");
        assert!(
            observed.requests.load(Ordering::SeqCst) > after_first,
            "a different code set must not be served the previous dataset's cache entry"
        );
    });
}

/// A dictionary tagging `fips` as a county-FIPS region column — what `viz smart` reads to find its
/// region column, since a concept only ever comes from a dictionary.
const COUNTY_DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;

// The headline of issue #4416: `viz smart --geojson auto` with no boundary asset supplied. The
// region column is not named on the command line — it comes from the dictionary — so this can only
// work if resolution is DEFERRED until after stats and column semantics exist.
#[test]
#[serial]
fn viz_smart_geojson_auto_resolves_from_the_dictionary_region_column() {
    let wrk = Workdir::new("viz_smart_geojson_auto_resolves_from_the_dictionary_region_column");
    wrk.create_from_string(
        "pa.csv",
        "fips,cases\n42003,10\n42003,20\n42101,30\n42101,40\n",
    );
    wrk.create_from_string("dict.schema.json", COUNTY_DICT);

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "pa.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env(
                "QSV_CACHE_DIR",
                wrk.path("boundary-cache").to_string_lossy().to_string(),
            );
        let out = wrk.output(&mut cmd);
        assert!(out.status.success(), "deferred auto resolution failed");
        let html = String::from_utf8_lossy(&out.stdout);

        // a region choropleth keyed off the fetched boundaries, not a bare frequency bar
        assert!(
            html.contains(r#""featureidkey":"properties.GEOID""#),
            "expected the auto-resolved feature-id key: {html}"
        );
        assert!(
            html.contains("count by fips"),
            "expected the summary choropleth panel: {html}"
        );

        // the fetch was scoped to the state the data names, exactly as on the --locations path
        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(
            clauses.iter().any(|c| c == "STATE IN ('42')"),
            "fetch was not scoped to the data's states: {clauses:?}"
        );
    });
}

// The zero-network criterion on the smart path. With a single region-code candidate there is
// nothing to choose between, so no probe is issued for the choice at all and the second run is
// served entirely from the boundary cache.
#[test]
#[serial]
fn viz_smart_geojson_auto_second_run_makes_no_requests() {
    let wrk = Workdir::new("viz_smart_geojson_auto_second_run_makes_no_requests");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,10\n42101,20\n42101,30\n");
    wrk.create_from_string("dict.schema.json", COUNTY_DICT);
    let cache = wrk.path("boundary-cache").to_string_lossy().to_string();

    with_mock_tigerweb(|base, observed| {
        let run = || {
            let mut cmd = wrk.command("viz");
            cmd.args(["smart", "pa.csv", "--geojson", "auto", "--dictionary"])
                .arg(wrk.path("dict.schema.json"))
                .env("QSV_CENSUS_TIGERWEB_URL", base)
                .env("QSV_CACHE_DIR", &cache);
            let out = wrk.output(&mut cmd);
            assert!(out.status.success());
        };

        run();
        let after_first = observed.requests.load(Ordering::SeqCst);
        assert!(after_first > 0, "the first run should have fetched");

        run();
        assert_eq!(
            observed.requests.load(Ordering::SeqCst),
            after_first,
            "the second run must be served entirely from cache"
        );
    });
}

// Candidate selection has to be honest: with more than one region-tagged column, the boundaries
// cannot be fetched for an arbitrary one and the columns then scored against them — for a
// code-set-scoped layer that set is DERIVED from the column it was fetched for, so whichever
// column was picked would score perfectly by construction. The decoy here is tagged as a region
// column (the only way in) and holds well-formed 5-digit codes that simply do not exist, so only
// a geometry-free probe of BOTH columns can tell them apart.
#[test]
#[serial]
fn viz_smart_geojson_auto_decoy_region_column_does_not_win() {
    let wrk = Workdir::new("viz_smart_geojson_auto_decoy_region_column_does_not_win");
    wrk.create_from_string(
        "pa.csv",
        "decoy,fips,cases\n99998,42003,10\n99998,42003,20\n99999,42101,30\n99999,42101,40\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "decoy": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.zip_code" } },
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "pa.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env(
                "QSV_CACHE_DIR",
                wrk.path("boundary-cache").to_string_lossy().to_string(),
            );
        let out = wrk.output(&mut cmd);
        assert!(out.status.success(), "auto resolution failed");
        let html = String::from_utf8_lossy(&out.stdout);

        // the real column drove the fetch AND the panel
        assert!(
            html.contains("count by fips"),
            "the decoy column should not have won: {html}"
        );

        // the decoy's codes were PROBED (that is how it lost) but never fetched geometry for:
        // no state-scoped query may name a state the decoy implies
        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(
            clauses.iter().any(|c| c.contains("99998")),
            "the decoy column was never probed, so it did not lose on evidence: {clauses:?}"
        );
        for clause in clauses.iter().filter(|c| c.starts_with("STATE IN")) {
            assert_eq!(
                clause, "STATE IN ('42')",
                "geometry was fetched for something other than the winning column"
            );
        }
    });
}

// A candidate can rank first and still be unusable: ranking is a geometry-free probe, so a column
// whose codes exist but whose boundary FETCH fails (a service error, an unpublished layer, a
// sampled head that flatters a bad tail) is only discovered afterwards. Committing to the winner
// would abort the run with a perfectly good sibling column unexamined — on this path the column
// choice is qsv's, not the user's, so a bad guess must cost another attempt, not the Data
// Schematic. Here `ca` probes clean (state 06 answers GEOID probes) but its geometry fetch 500s.
// (roborev 4255.)
#[test]
#[serial]
fn viz_smart_geojson_auto_falls_through_to_the_next_candidate() {
    let wrk = Workdir::new("viz_smart_geojson_auto_falls_through_to_the_next_candidate");
    // `ca` sits at column 0, so on the exact probe tie (2/2 each) the stable sort ranks it first.
    wrk.create_from_string(
        "two.csv",
        "ca,fips,cases\n06001,42003,10\n06001,42003,20\n06075,42101,30\n06075,42101,40\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "ca": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "two.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env(
                "QSV_CACHE_DIR",
                wrk.path("boundary-cache").to_string_lossy().to_string(),
            );
        let out = wrk.output(&mut cmd);
        assert!(
            out.status.success(),
            "a failing top candidate must not abort the run: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let html = String::from_utf8_lossy(&out.stdout);
        assert!(
            html.contains("count by fips"),
            "the surviving sibling column should have carried the map: {html}"
        );

        // the failed attempt really happened — otherwise this passes for the wrong reason
        // (`ca` never ranking first at all)
        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(
            clauses.iter().any(|c| c == "STATE IN (\'06\')"),
            "the top candidate was never fetched, so nothing fell through: {clauses:?}"
        );
        assert!(
            clauses.iter().any(|c| c == "STATE IN (\'42\')"),
            "the surviving candidate was never fetched: {clauses:?}"
        );
    });
}

// The other half of the fall-through rule: a TRANSIENT failure is a statement about the service,
// not about the column, so it must abort rather than quietly promote the next candidate. Silently
// drawing a different column's map because one fetch timed out would make the output depend on
// network weather — the same run would produce different maps on different days, with nothing
// said. It must also stay classified as a network failure rather than being rewrapped as a usage
// error. (roborev 4257.)
#[test]
#[serial]
fn viz_smart_geojson_auto_transient_failure_aborts_rather_than_switching_columns() {
    let wrk = Workdir::new(
        "viz_smart_geojson_auto_transient_failure_aborts_rather_than_switching_columns",
    );
    // `fl` sits at column 0, so on the exact probe tie it ranks first; its geometry fetch 5xxs.
    wrk.create_from_string(
        "two.csv",
        "fl,fips,cases\n12086,42003,10\n12086,42003,20\n12099,42101,30\n12099,42101,40\n",
    );
    wrk.create_from_string(
        "dict.schema.json",
        r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "fl": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
            "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
          }
        }"#,
    );

    with_mock_tigerweb(|base, observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "two.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env(
                "QSV_CACHE_DIR",
                wrk.path("boundary-cache").to_string_lossy().to_string(),
            );
        let out = wrk.output(&mut cmd);
        assert!(
            !out.status.success(),
            "a service outage must not be papered over with another column's map"
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("Census request to") && stderr.contains("failed"),
            "the outage must be reported as itself, not as a usage error: {stderr}"
        );

        // and the run stopped there — the sibling column was never fetched
        let clauses = observed.where_clauses.lock().unwrap().clone();
        assert!(
            !clauses.iter().any(|c| c == "STATE IN (\'42\')"),
            "the run continued to another column after a transient failure: {clauses:?}"
        );
    });
}

// issue #4414's acceptance case, reachable only because #4416 landed: auto-fetched boundaries
// carry land area in square METRES, so without a unit declaration the density map reads
// "per 1,000,000,000 AREALAND". `fetch_layer` stamps the unit onto the document it builds, so the
// user passes no unit flag at all. The declaration rides in the GeoJSON rather than beside it
// precisely so it survives the cache: asserted here by running twice, warm the second time.
#[test]
#[serial]
fn viz_geojson_auto_declares_its_area_unit_and_the_cache_keeps_it() {
    let wrk = Workdir::new("viz_geojson_auto_declares_its_area_unit_and_the_cache_keeps_it");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,600\n42003,700\n42101,800\n");
    let cache = wrk.path("boundary-cache").to_string_lossy().to_string();

    with_mock_tigerweb(|base, observed| {
        let run = || {
            let mut cmd = wrk.command("viz");
            cmd.args([
                "choropleth",
                "pa.csv",
                "--locations",
                "fips",
                "--value",
                "cases",
                "--location-mode",
                "geojson-id",
                "--geojson",
                "auto",
                "--denominator-key",
                "properties.AREALAND",
            ])
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache);
            let out = wrk.output(&mut cmd);
            assert!(out.status.success(), "auto + area denominator failed");
            String::from_utf8_lossy(&out.stdout).into_owned()
        };

        let cold = run();
        assert!(
            cold.contains("km²") && !cold.contains("AREALAND:"),
            "the fetch should declare its own unit, with no --denominator-unit passed: {cold}"
        );
        let after_first = observed.requests.load(Ordering::SeqCst);

        // the same command, served from cache, must label the map identically — a declaration
        // stored beside the document rather than in it would make run 2 say "AREALAND"
        let warm = run();
        assert_eq!(
            observed.requests.load(Ordering::SeqCst),
            after_first,
            "the second run should have been served from cache"
        );
        let rate_phrase = |h: &str| {
            h.split("per ")
                .filter(|s| s.starts_with("km²") || s.starts_with("1,"))
                .map(|s| s.chars().take(24).collect::<String>())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            rate_phrase(&cold),
            rate_phrase(&warm),
            "a warm cache labelled the same map differently"
        );
    });
}

// issue #4395: `--denominator census` fetches per-region population from the Census Data API and
// divides by it, so a raw-count map becomes a per-capita rate with nothing supplied. Asserted
// against the mock rather than the live Bureau: what matters is that qsv asks for the right
// geography, joins on FIPS, states which release it divided by, and does not ask twice.
#[test]
#[serial]
fn viz_denominator_census_fetches_population_and_states_its_release() {
    let wrk = Workdir::new("viz_denominator_census_fetches_population_and_states_its_release");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,600\n42003,700\n42101,800\n");
    let cache = wrk.path("census-cache").to_string_lossy().to_string();

    with_mock_tigerweb(|base, observed| {
        let run = || {
            let mut cmd = wrk.command("viz");
            cmd.args([
                "choropleth",
                "pa.csv",
                "--locations",
                "fips",
                "--location-mode",
                "geojson-id",
                "--geojson",
                "auto",
                "--denominator",
                "census",
            ])
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CENSUS_API_URL", format!("{base}/data"))
            .env("QSV_CACHE_DIR", &cache);
            let out = wrk.output(&mut cmd);
            assert!(
                out.status.success(),
                "census denominator failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8_lossy(&out.stdout).into_owned()
        };

        let html = run();
        // a per-capita rate, named in people rather than by a raw field name
        assert!(
            html.contains("residents"),
            "expected a per-capita rate: {html}"
        );
        // the release is stated — a rate means something different against a different vintage
        assert!(
            html.contains("ACS") && html.contains("B01003"),
            "the ACS release must be stated beneath the map: {html}"
        );
        // it asked for COUNTY population, because the codes are 5-digit county FIPS
        let asked = observed.where_clauses.lock().unwrap().clone();
        assert!(
            asked.iter().any(|c| c.starts_with("acs?county")),
            "expected a county-level ACS request, got: {asked:?}"
        );

        // and a repeated command makes no further request — the Data Schematic's offline promise
        let after_first = observed.requests.load(Ordering::SeqCst);
        run();
        assert_eq!(
            observed.requests.load(Ordering::SeqCst),
            after_first,
            "the second run must be served entirely from cache"
        );
    });
}

// `--denominator census` is accepted on `viz smart` too (issue #4395), which the pre-existing gate
// rejected for every `--denominator` value. The rule it enforces — a denominator COLUMN must be
// constant within a region, so it can only be read where a region key is read from the same row —
// simply does not reach a source qsv fetches itself.
#[test]
#[serial]
fn viz_smart_accepts_the_census_denominator() {
    let wrk = Workdir::new("viz_smart_accepts_the_census_denominator");
    wrk.create_from_string(
        "pa.csv",
        "fips,cases\n42003,600\n42003,700\n42101,800\n42101,900\n",
    );
    wrk.create_from_string("dict.schema.json", COUNTY_DICT);

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "smart",
            "pa.csv",
            "--geojson",
            "auto",
            "--denominator",
            "census",
            "--dictionary",
        ])
        .arg(wrk.path("dict.schema.json"))
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env("QSV_CENSUS_API_URL", format!("{base}/data"))
        .env(
            "QSV_CACHE_DIR",
            wrk.path("census-cache").to_string_lossy().to_string(),
        );
        let out = wrk.output(&mut cmd);
        assert!(
            out.status.success(),
            "`viz smart --denominator census` was rejected: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let html = String::from_utf8_lossy(&out.stdout);
        assert!(
            html.contains("residents"),
            "expected a rate panel beside the count panel: {html}"
        );
        assert!(
            html.contains("ACS"),
            "the rate panel must carry the release it divided by: {html}"
        );
    });
}

// A denominator column NAMED "census" is not a way to opt out: the value is reserved, and the help
// text says so. This pins the reservation rather than leaving it to prose.
#[test]
#[ignore = "Census API is down so this test fails with a network error. Re-enable when the API is \
            back up."]
fn viz_denominator_census_is_reserved() {
    let wrk = Workdir::new("viz_denominator_census_is_reserved");
    wrk.create_from_string("rg.csv", "region,census,val\nAB,100,10\nCD,200,20\n");

    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        "rg.csv",
        "--locations",
        "region",
        "--value",
        "val",
        "--location-mode",
        "geojson-id",
        "--geojson",
        "auto",
        "--denominator",
        "census",
    ])
    .env("QSV_CENSUS_API_URL", "http://127.0.0.1:1/data");
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    // it tried to RESOLVE the region codes as Census geographies rather than reading the column
    assert!(
        stderr.contains("county FIPS") || stderr.contains("state codes"),
        "the reserved value should have been treated as a source, not a column: {stderr}"
    );
}

// The `--denominator census` exception is for `viz smart` ONLY. Exempting it from the
// non-choropleth check everywhere let a mistyped subcommand pass validation and then silently
// ignore the flag — worse than the error it replaced, which at least said so. And on `viz smart`
// it needs a --geojson, or there is no region map for a rate to become. (roborev 4264.)
#[test]
fn viz_denominator_census_is_rejected_where_nothing_can_use_it() {
    let wrk = Workdir::new("viz_denominator_census_is_rejected_where_nothing_can_use_it");
    wrk.create_from_string("t.csv", "a,b\nx,1\ny,2\n");

    // a subcommand that has no region map at all
    let mut cmd = wrk.command("viz");
    cmd.args([
        "bar",
        "t.csv",
        "--x",
        "a",
        "--y",
        "b",
        "--denominator",
        "census",
    ]);
    let out = wrk.output(&mut cmd);
    assert!(
        !out.status.success(),
        "`viz bar --denominator census` must not silently ignore the flag"
    );
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("only applies to `viz choropleth`"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // `viz smart` builds its region choropleth only when a boundary set is in play
    wrk.create_from_string("fips.csv", "fips,cases\n42003,10\n42101,20\n");
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "fips.csv", "--denominator", "census"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("needs a --geojson"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// The key hint has to be observable, or the wrapper that centralizes it can be quietly unwound.
// This is the path that motivated it: a PINNED vintage skips the newest-vintage probe entirely, so
// before the wrapper existed it surfaced the bare "response is not valid JSON" — verified by hand
// against the live Bureau, and unpinned by any test until now. (roborev 4266.)
#[test]
#[serial]
fn viz_denominator_census_reports_a_key_problem_rather_than_bad_json() {
    let wrk = Workdir::new("viz_denominator_census_reports_a_key_problem_rather_than_bad_json");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,600\n42101,800\n");

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "pa.csv",
            "--locations",
            "fips",
            "--location-mode",
            "geojson-id",
            "--geojson",
            "auto",
            // pinned, so the vintage probe (where the hint used to live) never runs
            "--denominator",
            "census@2019",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        .env("QSV_CENSUS_API_URL", format!("{base}/data"))
        .env("QSV_CENSUS_API_KEY", "test-key-not-a-real-credential")
        .env(
            "QSV_CACHE_DIR",
            wrk.path("census-cache").to_string_lossy().to_string(),
        );
        let out = wrk.output(&mut cmd);
        assert!(!out.status.success());
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("QSV_CENSUS_API_KEY") && stderr.contains("key_signup"),
            "an HTML answer must be diagnosed as a key problem, not as bad JSON: {stderr}"
        );
    });
}

// The redaction, end to end rather than as a helper: a network failure quotes the request URL, and
// reqwest's error Display carries the whole query string — which is how a real key reached stderr
// in the first place. The unit test covers the function; this covers the wiring.
#[test]
#[serial]
fn viz_denominator_census_never_prints_the_api_key() {
    let wrk = Workdir::new("viz_denominator_census_never_prints_the_api_key");
    wrk.create_from_string("pa.csv", "fips,cases\n42003,600\n42101,800\n");

    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args([
            "choropleth",
            "pa.csv",
            "--locations",
            "fips",
            "--location-mode",
            "geojson-id",
            "--geojson",
            "auto",
            "--denominator",
            "census",
        ])
        .env("QSV_CENSUS_TIGERWEB_URL", base)
        // a port nothing is listening on: the failure quotes the URL, key and all
        .env("QSV_CENSUS_API_URL", "http://127.0.0.1:1/data")
        .env("QSV_CENSUS_API_KEY", "sup3rs3cret-census-key-value")
        .env(
            "QSV_CACHE_DIR",
            wrk.path("census-cache").to_string_lossy().to_string(),
        );
        let out = wrk.output(&mut cmd);
        assert!(!out.status.success());
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !stderr.contains("sup3rs3cret"),
            "the API key must never reach stderr: {stderr}"
        );
        assert!(
            stderr.contains("key=REDACTED"),
            "the redaction should be visible where the key was: {stderr}"
        );
    });
}
