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
        // one closure, invoked twice — the shape `viz_geojson_auto_second_run_makes_no_requests`
        // already uses, so the double-run check sees two distinct executions rather than one
        // command whose output is asserted across runs
        let run = || {
            let mut cmd = county_cmd(&wrk, base, "c.csv");
            let out = wrk.output(&mut cmd);
            assert!(
                out.status.success(),
                "run failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        };

        run();
        let after_first = observed.requests.load(std::sync::atomic::Ordering::SeqCst);
        assert!(after_first > 0, "the first run should have fetched");

        run();
        assert_eq!(
            observed.requests.load(std::sync::atomic::Ordering::SeqCst),
            after_first,
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

/// ISSUE #4481: a county name occurring in SEVERAL states now resolves per row, instead of being
/// dropped because the alias map could only hold one code per bare name.
///
/// This is the case the whole change exists for. `Washington County` is carried by 30 counties;
/// with a state column, PA's rows must chart under `42125` and MD's under `24043` — and before
/// #4481 BOTH were dropped, taking the correctly-resolved rows with them.
#[test]
#[serial]
fn viz_county_names_same_name_two_states_both_resolve() {
    let wrk = Workdir::new("viz_county_names_same_name_two_states_both_resolve");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nWashington County,PA,20\nWashington \
         County,MD,30\nBaltimore County,MD,40\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "run failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        for geoid in ["42003", "42125", "24043", "24005"] {
            assert!(html.contains(geoid), "{geoid} missing from trace: {stderr}");
        }
        // nothing was dropped, so no drop report at all
        assert!(
            !stderr.contains("omitted from the map"),
            "nothing should have been dropped: {stderr}"
        );
    });
}

/// ISSUE #4481, the honesty half: a name that resolves under one of its states and NOT another
/// costs only the failing PAIR. The resolving rows chart; the failing rows are reported.
///
/// Before, the whole name was dropped — so `Hampden County, MA` was collateral damage of a
/// mistyped `Hampden County, PA`.
#[test]
#[serial]
fn viz_county_names_mixed_outcome_costs_only_the_failing_pair() {
    let wrk = Workdir::new("viz_county_names_mixed_outcome_costs_only_the_failing_pair");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nPhiladelphia County,PA,20\nHampden \
         County,MA,40\nHampden County,PA,50\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        cmd.args(["--region-state", "state"]);
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(out.status.success(), "run failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        // the MA pair charts...
        assert!(html.contains("25013"), "Hampden MA missing: {stderr}");
        // ...and the PA pair is reported by its own cause, not swept into a name-level drop
        assert!(
            stderr.contains("do not exist in the state given for them"),
            "mismatch cause not named: {stderr}"
        );
        assert!(stderr.contains("MA"), "actual state not named: {stderr}");
    });
}

/// REGRESSION (roborev 4409): a county-NAME column must survive the smart trial order even when
/// another region-code candidate is present.
///
/// County-name slots cannot be probed — their values normalize to nothing, so `score_candidates`
/// returns `None` and the ranking's `filter_map` used to drop them outright. With two code
/// candidates (a digitless `geo.county` and a `geo.state`), that meant the county path was never
/// tried and the run died with "none of this dataset's region columns hold values that resolve".
/// The earlier smart test could not catch this: its state column was constant, so only ONE code
/// candidate survived `cardinality >= 2` and the no-probe short-circuit hid the bug.
#[test]
#[serial]
fn viz_smart_county_names_survive_a_competing_code_candidate() {
    const DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "county": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county" } },
    "state": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.state" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;
    let wrk = Workdir::new("viz_smart_county_names_survive_a_competing_code_candidate");
    // the state column VARIES, so it is a second region-code candidate and the probe actually runs
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nAllegheny County,PA,10\nAllegheny County,PA,12\nPhiladelphia \
         County,PA,20\nBaltimore County,MD,30\nWashington County,MD,40\nWashington County,MD,42\n",
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
        assert!(
            out.status.success(),
            "the county-name candidate was dropped from the trial order: {stderr}"
        );
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("42003") && html.contains("24043"),
            "counties missing from the Data Schematic: {stderr}"
        );
    });
}

/// REGRESSION (roborev 4409): the smallest mixed county column — one suffixed name and one bare —
/// is still recognized as county data. A strict majority rule failed the guard's own stated
/// intent ("one bare value must not disable the guard") on exactly this shape.
#[test]
#[serial]
fn viz_county_names_two_value_mixed_column_is_county_data() {
    let wrk = Workdir::new("viz_county_names_two_value_mixed_column_is_county_data");
    wrk.create_from_string(
        "c.csv",
        "county,cases\nAllegheny County,10\nPhiladelphia,20\n",
    );
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = county_cmd(&wrk, base, "c.csv");
        let out = wrk.output(&mut cmd);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            out.status.success(),
            "half-suffixed column failed: {stderr}"
        );
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("42003") && html.contains("42101"),
            "both counties should resolve: {stderr}"
        );
    });
}

/// REGRESSION (roborev 4411): a pinned NON-county layer must not be served county boundaries.
///
/// The county-name resolver always fetches counties, so honoring a digitless `geo.county` column
/// under `--geojson census:zcta` would quietly draw counties instead of the geography the user
/// pinned. `city_candidates` has always been gated on the layer this way; the county-name slots
/// introduced by the previous fix were not, which reintroduced the silent-wrong-geography class
/// this whole feature exists to close.
#[test]
#[serial]
fn viz_smart_county_names_do_not_hijack_a_pinned_layer() {
    const DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "county": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;
    let wrk = Workdir::new("viz_smart_county_names_do_not_hijack_a_pinned_layer");
    wrk.create_from_string(
        "c.csv",
        "county,cases\nAllegheny County,10\nAllegheny County,12\nPhiladelphia County,20\n",
    );
    wrk.create_from_string("dict.schema.json", DICT);
    let cache_dir = wrk.path("qsv-cache");
    std::fs::create_dir_all(&cache_dir).expect("create cache dir");
    with_mock_tigerweb(|base, _observed| {
        let mut cmd = wrk.command("viz");
        cmd.args(["smart", "c.csv", "--geojson", "census:zcta", "--dictionary"])
            .arg(wrk.path("dict.schema.json"))
            .env("QSV_CENSUS_TIGERWEB_URL", base)
            .env("QSV_CACHE_DIR", &cache_dir);
        let out = wrk.output(&mut cmd);
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            !html.contains("42003") && !html.contains("42101"),
            "a ZCTA pin was served county boundaries: {stderr}"
        );
        assert!(
            !out.status.success(),
            "no ZCTA resolves here, so the run must fail rather than substitute counties: {stderr}"
        );
    });
}

/// ISSUE #4481: the data-viewer drawer must filter by the qualifier too.
///
/// Once two features share a region spelling, a criterion on the region column alone would show
/// Maryland's rows under Pennsylvania's polygon — the same bare-name confusion one layer up. The
/// emitted chrome must name the qualifier column and carry its raw spellings.
#[test]
#[serial]
fn viz_smart_county_names_region_filter_is_qualified() {
    const DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "county": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county" } },
    "state": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.state" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;
    let wrk = Workdir::new("viz_smart_county_names_region_filter_is_qualified");
    wrk.create_from_string(
        "c.csv",
        "county,state,cases\nWashington County,PA,10\nWashington County,PA,12\nWashington \
         County,MD,30\nWashington County,MD,32\nAllegheny County,PA,40\nAllegheny County,PA,42\n",
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
        assert!(out.status.success(), "smart run failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        // both same-named counties charted
        assert!(
            html.contains("42125") && html.contains("24043"),
            "both Washingtons should chart: {stderr}"
        );
        // the drawer filter names the STATE column (index 1), not -1
        assert!(
            html.contains("var QUAL_COL = 1;"),
            "region filter was not qualified: {stderr}"
        );
        // and carries each feature's raw qualifier spellings
        assert!(
            html.contains(r#""42125":["PA"]"#) || html.contains(r#""42125": ["PA"]"#),
            "qualifier spellings missing for 42125: {stderr}"
        );
    });
}

/// A region-CODE column publishes no aliases, so the drawer filter must stay unqualified — the
/// qualified path must not leak into the ordinary case.
#[test]
#[serial]
fn viz_smart_code_column_region_filter_is_unqualified() {
    const DICT: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "fips": { "type": "string", "x-qsv": { "qsv_type": "String", "role": "dimension", "concept": "geo.county_fips" } },
    "cases": { "type": "number", "x-qsv": { "qsv_type": "Integer", "role": "measure", "concept": "measure.amount" } }
  }
}"#;
    let wrk = Workdir::new("viz_smart_code_column_region_filter_is_unqualified");
    wrk.create_from_string(
        "c.csv",
        "fips,cases\n42003,10\n42003,12\n42101,20\n42101,22\n",
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
        assert!(out.status.success(), "smart run failed: {stderr}");
        let html = String::from_utf8_lossy(&out.stdout).to_string();
        assert!(
            html.contains("var QUAL_COL = -1;"),
            "a code column must not acquire a qualifier: {stderr}"
        );
    });
}
