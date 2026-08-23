//! Tests for `viz --geojson auto` with a `--locations` column of county NAMES, resolved against
//! the Census's own name table for the vintage in play (issue #4417 Part B).
//!
//! Deliberately NOT gated on the `geocode` feature. That is the point of the mechanism: the
//! boundary service already knows what its counties are called, so a county-name column needs no
//! Geonames index, no 22 MB download, and no `geocode` build. `TIGERweb` is mocked through the
//! same `QSV_CENSUS_TIGERWEB_URL` seam `test_viz_census` uses, so there is no new network seam
//! either.

use serial_test::serial;

use crate::{test_viz_census::with_mock_tigerweb, workdir::Workdir};

/// A `viz choropleth --geojson auto` command wired to the mock service and an isolated cache.
fn county_cmd(wrk: &Workdir, base: &str, csv: &str) -> std::process::Command {
    let cache_dir = wrk.path("qsv-cache");
    std::fs::create_dir_all(&cache_dir).expect("create cache dir");
    let mut cmd = wrk.command("viz");
    cmd.args([
        "choropleth",
        csv,
        "--locations",
        "county",
        "--value",
        "cases",
        "--geojson",
        "auto",
    ])
    .env("QSV_CENSUS_TIGERWEB_URL", base)
    .env("QSV_CACHE_DIR", cache_dir);
    cmd
}

/// The acceptance criterion: a county-NAME column resolves to county boundaries with nothing
/// supplied but the CSV — no --geocode, no index, no explicit --geojson.
#[test]
#[serial]
fn viz_county_names_resolve() {
    let wrk = Workdir::new("viz_county_names_resolve");
    wrk.create_from_string(
        "c.csv",
        "county,cases\nAllegheny County,10\nPhiladelphia County,20\nAllegheny County,5\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "county names failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(html.contains("42003"), "Allegheny missing from trace");
        assert!(html.contains("42101"), "Philadelphia missing from trace");
        assert!(
            stderr.contains("resolved from --locations county names"),
            "provenance not reported: {stderr}"
        );
    });
}

/// `BASENAME` is a first-class key, so the bare spelling resolves identically once the column is
/// routed down this path. Both spellings live in ONE map under ONE ambiguity rule, which is what
/// keeps a bare name that collides across states from silently resolving.
///
/// A bare-name column carries no county suffix to route on, so it needs `--region-state` to
/// declare intent — asserted here, because the alternative (resolve-and-see-what-sticks) would let
/// a column of CITY names partially match county BASENAMEs and render the wrong geography.
#[test]
#[serial]
fn viz_county_names_bare_basename_resolves() {
    let wrk = Workdir::new("viz_county_names_bare_basename_resolves");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny,PA,10\nPhiladelphia,PA,20\n",
    );
    with_mock_tigerweb(|base, _observed| {
        // without --region-state a bare-name column is not routed here at all, and says so
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        assert!(!out.status.success(), "bare names should not auto-route");
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains("If they are COUNTY names"),
            "the county-name route should be named: {stderr}"
        );

        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "bare basenames failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(html.contains("42003") && html.contains("42101"), "{stderr}");
    });
}

/// A name several counties carry, with no state to resolve it, is REFUSED — not resolved to an
/// arbitrary state and not counted as a soft drop. 422 county names are shared nationally,
/// covering 52% of counties and skewing populous, so a soft drop would clear the coverage gate
/// while silently omitting half the map.
#[test]
#[serial]
fn viz_county_names_ambiguous_without_state_errors() {
    let wrk = Workdir::new("viz_county_names_ambiguous_without_state_errors");
    wrk.create_from_string(
        "c.csv",
        "county,cases\nAllegheny County,10\nWashington County,20\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        assert!(!out.status.success(), "ambiguous names should be refused");
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains("carried by more than one county"),
            "cause not named: {stderr}"
        );
        assert!(
            stderr.contains("--region-state"),
            "remedy not named: {stderr}"
        );
        // the report must name the actual candidates, not just a count
        assert!(
            stderr.contains("42125") && stderr.contains("24043"),
            "candidates not shown: {stderr}"
        );
    });
}

/// With a state column, the same ambiguous name places correctly. All three spellings a state
/// column may hold are accepted.
#[test]
#[serial]
fn viz_county_names_state_column_disambiguates() {
    for (label, state_value) in [("usps", "PA"), ("fips", "42"), ("name", "Pennsylvania")] {
        let wrk = Workdir::new(&format!("viz_county_names_state_{label}"));
        wrk.create_from_string(
            "c.csv",
            &format!("county,state,cases\nWashington County,{state_value},20\n"),
        );
        with_mock_tigerweb(|base, _observed| {
            let mut cmd = county_cmd(&wrk, base, "c.csv");
            cmd.args(["--region-state", "state"]);
            let out = wrk.output(&mut cmd);
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            assert!(
                out.status.success(),
                "state spelling {label} failed: {stderr}"
            );
            let html = String::from_utf8_lossy(&out.stdout).to_string();
            assert!(
                html.contains("42125"),
                "state spelling {label} placed the wrong county: {stderr}"
            );
            assert!(
                !html.contains("24043"),
                "state spelling {label} leaked the Maryland county: {stderr}"
            );
        });
    }
}

/// A BASENAME two counties in the SAME state share — Maryland's Baltimore County vs Baltimore
/// city — cannot be separated by a state hint, so it stays ambiguous rather than being guessed.
/// This is the case a two-tier scheme that pre-pruned BASENAME by within-state uniqueness would
/// have silently resolved.
#[test]
#[serial]
fn viz_county_names_within_state_collision_is_not_guessed() {
    let wrk = Workdir::new("viz_county_names_within_state_collision_is_not_guessed");
    wrk.create_from_string("c.csv", "county,state,cases\nBaltimore,MD,20\n");
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        assert!(
            !out.status.success(),
            "a within-state collision must not be guessed"
        );
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains("24005") && stderr.contains("24510"),
            "both candidates should be named: {stderr}"
        );
    });
}

/// A name that exists, but not in the state given for it, is reported as such rather than as an
/// unknown name — the distinction tells a typo'd county from a typo'd state.
#[test]
#[serial]
fn viz_county_names_state_mismatch_is_named() {
    let wrk = Workdir::new("viz_county_names_state_mismatch_is_named");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nHampden County,PA,20\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains("do not exist in the state given for them"),
            "mismatch cause not named: {stderr}"
        );
        assert!(stderr.contains("MA"), "actual state not named: {stderr}");
    });
}

/// The warm-run invariant: the name table and the boundaries are both cached, so a repeat command
/// contacts the service zero times.
#[test]
#[serial]
fn viz_county_names_second_run_makes_no_requests() {
    let wrk = Workdir::new("viz_county_names_second_run_makes_no_requests");
    wrk.create_from_string("c.csv", "county,cases\nAllegheny County,10\n");
    with_mock_tigerweb(|base, observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        assert!(out.status.success());
        assert!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst) > 0,
            "the first run should have fetched"
        );
        observed
            .requests
            .store(0, std::sync::atomic::Ordering::SeqCst);
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        assert!(out.status.success());
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            0,
            "a warm run must make no requests"
        );
    });
}

/// `--region-state` is refused with `--geocode` rather than silently ignored: it disambiguates
/// COUNTY names, which the geocode engine cannot resolve at all.
#[test]
#[serial]
fn viz_county_names_region_state_rejects_geocode() {
    let wrk = Workdir::new("viz_county_names_region_state_rejects_geocode");
    wrk.create_from_string("c.csv", "county,state,cases\nAllegheny County,PA,10\n");
    let mut cmd = county_cmd(&wrk, "http://127.0.0.1:1", "c.csv");
    cmd.args(["--region-state", "state", "--geocode"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        stderr.contains("--region-state disambiguates COUNTY names"),
        "wrong error: {stderr}"
    );
}

/// `--region-state` on `viz smart` is refused too — there the state column comes from the data
/// dictionary, so accepting the flag would validate it and then ignore it.
#[test]
#[serial]
fn viz_county_names_region_state_rejects_smart() {
    let wrk = Workdir::new("viz_county_names_region_state_rejects_smart");
    wrk.create_from_string("c.csv", "county,state,cases\nAllegheny County,PA,10\n");
    let mut cmd = wrk.command("viz");
    cmd.args(["smart", "c.csv", "--region-state", "state"]);
    let out = wrk.output(&mut cmd);
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        stderr.contains("only applies to `viz choropleth`"),
        "wrong error: {stderr}"
    );
}

/// A column of names that name no county names the OTHER path rather than dead-ending.
#[test]
#[serial]
fn viz_county_names_unknown_names_point_at_geocode() {
    let wrk = Workdir::new("viz_county_names_unknown_names_point_at_geocode");
    wrk.create_from_string(
        "c.csv",
        "county,cases\nErewhon County,10\nAtlantis County,20\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        assert!(!out.status.success());
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains("match no county in this vintage"),
            "cause not named: {stderr}"
        );
        assert!(
            stderr.contains("--geocode"),
            "the city-name path should be named: {stderr}"
        );
    });
}

/// The dictionary-driven smart path: a `geo.county` column holding NAMES resolves, taking its
/// state from the `geo.state` column the dictionary names.
#[test]
#[serial]
fn viz_smart_county_name_column_resolves() {
    const DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "county": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county" } },
    "state": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.state" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;
    let wrk = Workdir::new("viz_smart_county_name_column_resolves");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nAllegheny County,PA,15\nWashington \
         County,PA,20\nWashington County,PA,25\n",
    );
    wrk.create_from_string("dict.schema.json", DICT);
    let cache_dir = wrk.path("qsv-cache");
    std::fs::create_dir_all(&cache_dir).expect("create cache dir");
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "c.csv", "--geojson", "auto", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "smart county names failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("42003") && html.contains("42125"),
            "counties missing from the Data Schematic: {stderr}"
        );
    });
}

/// The two multi-state drop causes are reported SEPARATELY, because their remedies differ.
///
/// `Washington County` names a different county in each of PA and MD, so it can never be keyed by
/// bare name — the remedy is a FIPS column. `Hampden County` resolves in MA but does not exist in
/// PA, so the remedy is fixing that one name/state pairing. Collapsing both into "occurs in more
/// than one state" would hand the wrong instruction to the second case.
#[test]
#[serial]
fn viz_county_names_separates_divergent_from_mixed() {
    let wrk = Workdir::new("viz_county_names_separates_divergent_from_mixed");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nPhiladelphia County,PA,15\nBaltimore \
         County,MD,18\nAllegheny,PA,12\nPhiladelphia,PA,14\nWashington County,PA,20\n\
         Washington County,MD,30\nHampden County,MA,40\nHampden County,PA,50\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "run failed: {stderr}");
        assert!(
            stderr.contains("name a DIFFERENT county in each of the states"),
            "divergent cause not named: {stderr}"
        );
        assert!(
            stderr.contains("resolve under one of the states given for them but not another"),
            "mixed cause not named: {stderr}"
        );
        // reported in the DATA's spelling, not the lowercased match key
        assert!(
            stderr.contains("Washington County") && stderr.contains("Hampden County"),
            "names not quoted as written: {stderr}"
        );
    });
}
