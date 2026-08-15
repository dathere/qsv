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
struct Observed {
    requests:      Arc<AtomicUsize>,
    where_clauses: Arc<Mutex<Vec<String>>>,
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
        "properties": {"GEOID": geoid, "NAME": format!("County {geoid}")},
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

    // geometry-free probe vs. the real fetch — both answer from the same fixture
    let features = if where_clause.contains("42") {
        vec![county_feature("42003", 0.0), county_feature("42101", 2.0)]
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
fn with_mock_tigerweb<F: FnOnce(&str, &Observed)>(f: F) {
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
