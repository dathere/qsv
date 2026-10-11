// NOTE: these tests are meant for the cities15000 dataset
use serial_test::serial;

use crate::workdir::Workdir;

#[test]
fn geocode_suggest() {
    let wrk = Workdir::new("geocode_suggest");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["(41.90059, -87.85673)"],
        svec!["(28.11085, -82.69482)"],
        svec!["(40.71427, -74.00597)"],
        svec!["(45.09413, -93.35634)"],
        svec!["(41.27621, -72.86843)"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["(40.72816, -74.07764)"],
        svec!["95.213424, 190,1234565"], // suggest expects a city name, not lat, long
        svec!["(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_select() {
    let wrk = Workdir::new("geocode_suggest_select");
    wrk.create(
        "data.csv",
        vec![
            svec!["c1", "c2", "Location"],
            svec!["1", "2", "Melrose, New York"],
            svec!["3", "4", "East Flatbush, New York"],
            svec!["5", "6", "Manhattan, New York"],
            svec!["7", "8", "Brooklyn, New York"],
            svec!["9", "10", "East Harlem, New York"],
            svec![
                "11",
                "12",
                "This is not a Location and it will not be geocoded"
            ],
            svec!["13", "14", "Jersey City, New Jersey"],
            svec!["15", "16", "95.213424, 190,1234565"], // invalid lat, long
            svec!["17", "18", "Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    // use select syntax to select the last column
    cmd.arg("suggest").arg("_").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["c1", "c2", "Location"],
        svec!["1", "2", "(41.90059, -87.85673)"],
        svec!["3", "4", "(28.11085, -82.69482)"],
        svec!["5", "6", "(40.71427, -74.00597)"],
        svec!["7", "8", "(45.09413, -93.35634)"],
        svec!["9", "10", "(41.27621, -72.86843)"],
        svec![
            "11",
            "12",
            "This is not a Location and it will not be geocoded"
        ],
        svec!["13", "14", "(40.72816, -74.07764)"],
        svec!["15", "16", "95.213424, 190,1234565"], // suggest expects a city name, not lat, long
        svec!["17", "18", "(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggestnow_default() {
    let wrk = Workdir::new("geocode_suggestnow_default");
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("Brooklyn");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Brooklyn, New York US: 40.6501, -73.94958"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_admin1_without_alternate_name_falls_back() {
    // Maputo City (MZ.11) has no `en` alternateName, so the default English index printed a blank
    // admin1 ("Maputo,  MZ") although the record carries the division. Fall back to its own name.
    let wrk = Workdir::new("geocode_admin1_without_alternate_name_falls_back");
    let mut cmd = wrk.command("geocode");
    cmd.arg("reversenow").arg("-25.9692, 32.5732");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["Location"], svec!["Maputo, Maputo City MZ"]];
    assert_eq!(got, expected);
}

#[test]
fn geocode_language_not_in_index_falls_back_and_warns() {
    // the default index is English-only: `-l FR` (any case) must still print the admin1 name, and
    // say why the names are not French
    let wrk = Workdir::new("geocode_language_not_in_index_falls_back_and_warns");
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("-l").arg("FR").arg("Paris");

    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stdout.contains("Paris, Île-de-France FR: 48.85341, 2.3488"),
        "admin1 must fall back, not blank: {stdout}"
    );
    assert!(
        stderr.contains("has no 'fr' names"),
        "missing language warning: {stderr}"
    );
}

#[test]
fn geocode_countryinfonow_language_not_in_index_falls_back() {
    let wrk = Workdir::new("geocode_countryinfonow_language_not_in_index_falls_back");
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow").arg("-l").arg("fr").arg("FR");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["Location"], svec!["France"]];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggestnow_admin1_code_lowercase() {
    // `us.pr` worked (territory path) but `us.ca` demanded --country: codes are case-insensitive
    let wrk = Workdir::new("geocode_suggestnow_admin1_code_lowercase");
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow")
        .arg("--admin1")
        .arg("us.ca")
        .arg("San Juan");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["San Juan Capistrano, California US: 33.50169, -117.66255"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_index_update_empty_languages() {
    let wrk = Workdir::new("geocode_index_update_empty_languages");
    let mut cmd = wrk.command("geocode");
    // isolate the cache: should validation ever regress, index-update must not rebuild the real
    // active index (it did, under a mutation test of parse_languages)
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("index-update").arg("--languages").arg(" , ");
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("--languages must name at least one language"));
}

#[test]
fn geocode_suggestnow_formatstr_dyncols() {
    let wrk = Workdir::new("geocode_suggestnow_formatstr_dyncols");

    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("Secaucus").args([
        "--formatstr",
        "%dyncols: {population:population}, {state:admin1}, {county:admin2}, \
         {state_fips:us_state_fips_code}, {county_fips:us_county_fips_code}, {timezone:timezone}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "Location",
            "population",
            "state",
            "county",
            "state_fips",
            "county_fips",
            "timezone",
        ],
        svec![
            "Secaucus",
            "19104",
            "New Jersey",
            "Hudson",
            "34",
            "017",
            "America/New_York"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggestnow_dyncols_county_fips_over_100() {
    // #4771: county codes >= 100 lost their first digit (US.MD.510 -> "010")
    let wrk = Workdir::new("geocode_suggestnow_dyncols_county_fips_over_100");

    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("Baltimore").args([
        "--admin1",
        "US.MD",
        "--formatstr",
        "%dyncols: {state_fips:us_state_fips_code}, {county_fips:us_county_fips_code}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location", "state_fips", "county_fips"],
        svec!["Baltimore", "24", "510"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl() {
    let wrk = Workdir::new("geocode_suggest_intl");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Île-de-France Region FR"],
        svec!["Manila, National Capital Region PH"],
        svec!["London, England GB"],
        svec!["Berlin, State of Berlin DE"],
        svec!["Moscow, Moscow RU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brasília, Federal District BR"],
        svec!["95.213424, 190,1234565"],
        svec!["Havana, La Habana Province CU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl_country_filter() {
    let wrk = Workdir::new("geocode_suggest_intl_country_filter");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--country", "US"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Texas US"],
        svec!["Manteca, California US"],
        svec!["Sterling, Virginia US"],
        svec!["Burlington, North Carolina US"],
        svec!["Moscow, Idaho US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brawley, California US"],
        svec!["95.213424, 190,1234565"],
        svec!["Savannah, Georgia US"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl_admin1_filter_error() {
    let wrk = Workdir::new("geocode_suggest_intl_admin1_filter_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--admin1", "US"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    // admin1 requires a country filter
    wrk.assert_err(&mut cmd);
}

#[test]
fn geocode_suggestnow() {
    let wrk = Workdir::new("geocode_suggestnow");

    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow")
        .arg("Paris")
        .args(["--country", "US"])
        .args(["-f", "%city-admin1-country"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["Location"], svec!["Paris, Texas US"]];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reversenow() {
    let wrk = Workdir::new("geocode_reversenow");

    let mut cmd = wrk.command("geocode");
    cmd.arg("reversenow").arg("(40.67, -73.94)").args([
        "-f",
        "{name}, {admin2} County, {admin1} - {population} {timezone}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Brooklyn, Kings County, New York - 2736074 America/New_York"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reversenow_error() {
    let wrk = Workdir::new("geocode_reversenow_error");

    let mut cmd = wrk.command("geocode");
    cmd.arg("reversenow")
        .arg("(40.67, -73.94)")
        .args(["--admin1", "New York"])
        .args([
            "-f",
            "{name}, {admin2} County, {admin1} - {population} {timezone}",
        ]);

    // reversenow does not support admin1 filter
    wrk.assert_err(&mut cmd);
}

#[test]
fn geocode_suggest_intl_admin1_filter_country_inferencing() {
    let wrk = Workdir::new("geocode_suggest_intl_admin1_filter_country_inferencing");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--admin1", "US.NJ,US.TX,US.NY"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    // admin1 requires a country filter
    // however, since all the admin1 filters are in the US
    // or more specifically, the admin1 filters have a prefix of "US."
    // the country filter is inferred to be "US"
    wrk.assert_success(&mut cmd);
}

#[test]
fn geocode_suggest_intl_multi_country_filter() {
    let wrk = Workdir::new("geocode_suggest_intl_multi_country_filter");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--country", "us,FR,ru"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Île-de-France Region FR"],
        svec!["Manteca, California US"],
        svec!["Sterling, Virginia US"],
        svec!["Burlington, North Carolina US"],
        svec!["Moscow, Moscow RU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brawley, California US"],
        svec!["95.213424, 190,1234565"],
        svec!["Savannah, Georgia US"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_filter_country_admin1() {
    let wrk = Workdir::new("geocode_suggest_filter_country_admin1");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["(41.90059, -87.85673)"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["-f", "{name}, {admin1}, {admin2} {country}"])
        .args(["--country", "US"])
        .args(["--admin1", "US.NY,New J,Metro Manila"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // --admin1 FILTERS (issue #4687): a value with no match inside the listed admin1s is left
    // unchanged, never replaced by a place elsewhere (this used to return Melrose Park, Illinois
    // and East Haven, Connecticut for New York values, and McKinney, Texas for Makati).
    let expected = vec![
        svec!["Location"],
        svec!["Melrose, New York"],
        svec!["Elmwood Park, New Jersey, Bergen County US"],
        svec!["New York, New York,  US"],
        svec!["Brooklyn, New York, Kings US"],
        svec!["East Meadow, New York, Nassau US"],
        svec!["This is not a Location and it will not be geocoded"],
        // Jersey City matched as the admin1 filter included "New J"
        // which starts_with match "New Jersey"
        svec!["Jersey City, New Jersey, Hudson US"],
        // suggest expects a city name, not lat, long
        svec!["(41.90059, -87.85673)"],
        // the Metro Manila filter cannot admit it: the country filter is US
        svec!["Makati, Metro Manila, Philippines"],
    ];
    assert_eq!(got, expected);
}

// US territories are countries of their own in Geonames (Ponce is admin1 `PR.113` of country
// `PR`), so `US.PR` used to exclude every Puerto Rico place before ranking (issue #4687).
#[test]
fn geocode_suggest_admin1_us_territory() {
    let wrk = Workdir::new("geocode_suggest_admin1_us_territory");
    wrk.create(
        "data.csv",
        vec![
            svec!["city"],
            svec!["Ponce"],
            svec!["Mayaguez"],
            svec!["Carolina"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.args(["suggest", "city", "data.csv"])
        .args(["-f", "{name}, {admin1} {country}"])
        .args(["--admin1", "US.PR"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["city"],
        svec!["Ponce, Ponce PR"],
        svec!["Mayagüez, Mayagüez PR"],
        svec!["Carolina, Carolina PR"],
    ];
    assert_eq!(got, expected);

    // an explicit --country US still admits the territory the admin1 list names
    let mut cmd = wrk.command("geocode");
    cmd.args(["suggest", "city", "data.csv"])
        .args(["-f", "{name}, {admin1} {country}"])
        .args(["--country", "US", "--admin1", "US.PR"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, expected);
}

// Every entry an admin1 code: the countries are inferred from ALL their prefixes, so a mixed
// state + territory list needs no --country (issue #4687).
#[test]
fn geocode_suggest_admin1_mixed_country_prefixes() {
    let wrk = Workdir::new("geocode_suggest_admin1_mixed_country_prefixes");
    wrk.create(
        "data.csv",
        vec![svec!["city"], svec!["Rochester"], svec!["Ponce"]],
    );
    let mut cmd = wrk.command("geocode");
    cmd.args(["suggest", "city", "data.csv"])
        .args(["-f", "{name}, {admin1} {country}"])
        .args(["--admin1", "US.NY,US.PR"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["city"],
        svec!["Rochester, New York US"],
        svec!["Ponce, Ponce PR"],
    ];
    assert_eq!(got, expected);
}

// --admin1 is strict: Inverness, FL is not in the default (cities15000) index, and the value must
// stay unchanged and be counted rather than become Inver Grove Heights, Minnesota (issue #4687).
#[test]
fn geocode_suggest_admin1_is_strict_and_reports_misses() {
    let wrk = Workdir::new("geocode_suggest_admin1_is_strict_and_reports_misses");
    wrk.create(
        "data.csv",
        vec![svec!["city"], svec!["Inverness"], svec!["Tampa"]],
    );
    let mut cmd = wrk.command("geocode");
    cmd.args(["suggest", "city", "data.csv"])
        .args(["-f", "{name}, {admin1}"])
        .args(["--admin1", "US.FL"]);
    let out = wrk.output(&mut cmd);
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout, "city\nInverness\n\"Tampa, Florida\"\n");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(
            "1 value(s) found no match within --admin1 US.FL, so they were left unchanged"
        ),
        "{stderr}"
    );
}

// A prefix match scores a flat 1.0 and ties go to population, so "Davis" used to rank San Diego
// (a name variant starting with "davis") above Davis, CA. A place whose own name IS the value now
// wins the tie, with or without --admin1 (issue #4687).
#[test]
fn geocode_suggest_prefers_exact_name() {
    let wrk = Workdir::new("geocode_suggest_prefers_exact_name");
    wrk.create("data.csv", vec![svec!["city"], svec!["Davis"]]);
    for admin1 in [None, Some("US.CA")] {
        let mut cmd = wrk.command("geocode");
        cmd.args(["suggest", "city", "data.csv"])
            .args(["-f", "{name}, {admin1}"]);
        if let Some(a) = admin1 {
            cmd.args(["--admin1", a]);
        }
        let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
        assert_eq!(
            got,
            vec![svec!["city"], svec!["Davis, California"]],
            "--admin1 {admin1:?}"
        );
    }
}

#[test]
fn geocode_suggest_invalid() {
    let wrk = Workdir::new("geocode_suggest_invalid");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["Brooklyn, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--invalid-result", "<ERROR>"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["(41.90059, -87.85673)"],
        svec!["(28.11085, -82.69482)"],
        svec!["(40.71427, -74.00597)"],
        svec!["(41.27621, -72.86843)"],
        svec!["(45.09413, -93.35634)"],
        svec!["<ERROR>"],
        svec!["(40.72816, -74.07764)"],
        svec!["<ERROR>"], // suggest expects a city name, not lat, long
        svec!["(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_dynfmt() {
    let wrk = Workdir::new("geocode_suggest_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg(
            "{latitude}:{longitude} - {name}, {admin1}:{us_state_fips_code}-{us_county_fips_code} \
             {country} {continent} {currency_code} {neighbours}",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["41.90059:-87.85673 - Melrose Park, Illinois:17-031 US NA USD CA,MX,CU"],
        svec!["28.11085:-82.69482 - East Lake, Florida:12-103 US NA USD CA,MX,CU"],
        svec!["40.71427:-74.00597 - New York, New York:36- US NA USD CA,MX,CU"],
        svec!["41.27621:-72.86843 - East Haven, Connecticut:09-170 US NA USD CA,MX,CU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["14.55027:121.03269 - Makati City, National Capital Region:- PH AS PHP "],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_pretty_json() {
    let wrk = Workdir::new("geocode_suggest_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%pretty-json")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec![
            "{\n  \"cityrecord\":{\n  \"id\": 4901868,\"name\": \"Melrose Park\",\"latitude\": \
             41.90058898925781,\"longitude\": -87.85672760009766,\"country\": {\n    \"id\": \
             6252001,\"code\": \"US\",\"name\": \"United States\"\n  },\"admin_division\": {\n    \
             \"id\": 4896861,\"code\": \"US.IL\",\"name\": \"Illinois\"\n  },\"admin2_division\": \
             {\n    \"id\": 4888671,\"code\": \"US.IL.031\",\"name\": \"Cook County\"\n  \
             },\"timezone\": \"America/Chicago\",\"names\": {\n    \"en\": \"Melrose Park\"\n  \
             },\"country_names\": {\n    \"en\": \"United States\"\n  },\"admin1_names\": {\n    \
             \"en\": \"Illinois\"\n  },\"admin2_names\": {\n    \"en\": \"Cook\"\n  \
             },\"population\": 25379\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \
             \"US\",\"iso3\": \"USA\",\"iso_numeric\": \"840\",\"fips\": \"US\",\"name\": \
             \"United States\",\"capital\": \"Washington\",\"area\": \"9629091\",\"population\": \
             327167434,\"continent\": \"NA\",\"tld\": \".us\",\"currency_code\": \
             \"USD\",\"currency_name\": \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n},\n  \
             \"us_fips_codes\":{\n  \"us_state_code\": \"IL\",\n  \"us_state_name\": \
             \"Illinois\",\n  \"us_state_fips_code\": \"17\",\n  \"us_county\": \"Cook\",\n  \
             \"us_county_fips_code\": \"031\"\n}\n}"
        ],
        svec![
            "{\n  \"cityrecord\":{\n  \"id\": 4154008,\"name\": \"East Lake\",\"latitude\": \
             28.110849380493164,\"longitude\": -82.69481658935547,\"country\": {\n    \"id\": \
             6252001,\"code\": \"US\",\"name\": \"United States\"\n  },\"admin_division\": {\n    \
             \"id\": 4155751,\"code\": \"US.FL\",\"name\": \"Florida\"\n  },\"admin2_division\": \
             {\n    \"id\": 4168618,\"code\": \"US.FL.103\",\"name\": \"Pinellas County\"\n  \
             },\"timezone\": \"America/New_York\",\"names\": null,\"country_names\": {\n    \
             \"en\": \"United States\"\n  },\"admin1_names\": {\n    \"en\": \"Florida\"\n  \
             },\"admin2_names\": {\n    \"en\": \"Pinellas\"\n  },\"population\": 30962\n},\n  \
             \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\"iso3\": \
             \"USA\",\"iso_numeric\": \"840\",\"fips\": \"US\",\"name\": \"United \
             States\",\"capital\": \"Washington\",\"area\": \"9629091\",\"population\": \
             327167434,\"continent\": \"NA\",\"tld\": \".us\",\"currency_code\": \
             \"USD\",\"currency_name\": \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n},\n  \
             \"us_fips_codes\":{\n  \"us_state_code\": \"FL\",\n  \"us_state_name\": \
             \"Florida\",\n  \"us_state_fips_code\": \"12\",\n  \"us_county\": \"Pinellas\",\n  \
             \"us_county_fips_code\": \"103\"\n}\n}"
        ],
        svec![
            "{\n  \"cityrecord\":{\n  \"id\": 5128581,\"name\": \"New York City\",\"latitude\": \
             40.714271545410156,\"longitude\": -74.00596618652344,\"country\": {\n    \"id\": \
             6252001,\"code\": \"US\",\"name\": \"United States\"\n  },\"admin_division\": {\n    \
             \"id\": 5128638,\"code\": \"US.NY\",\"name\": \"New York\"\n  },\"admin2_division\": \
             null,\"timezone\": \"America/New_York\",\"names\": {\n    \"en\": \"New York\"\n  \
             },\"country_names\": {\n    \"en\": \"United States\"\n  },\"admin1_names\": {\n    \
             \"en\": \"New York\"\n  },\"admin2_names\": null,\"population\": 8804190\n},\n  \
             \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\"iso3\": \
             \"USA\",\"iso_numeric\": \"840\",\"fips\": \"US\",\"name\": \"United \
             States\",\"capital\": \"Washington\",\"area\": \"9629091\",\"population\": \
             327167434,\"continent\": \"NA\",\"tld\": \".us\",\"currency_code\": \
             \"USD\",\"currency_name\": \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n},\n  \
             \"us_fips_codes\":{\n  \"us_state_code\": \"NY\",\n  \"us_state_name\": \"New \
             York\",\n  \"us_state_fips_code\": \"36\",\n  \"us_county\": \"\",\n  \
             \"us_county_fips_code\": null\n}\n}"
        ],
        svec![
            "{\n  \"cityrecord\":{\n  \"id\": 4833425,\"name\": \"East Haven\",\"latitude\": \
             41.27621078491211,\"longitude\": -72.8684310913086,\"country\": {\n    \"id\": \
             6252001,\"code\": \"US\",\"name\": \"United States\"\n  },\"admin_division\": {\n    \
             \"id\": 4831725,\"code\": \"US.CT\",\"name\": \"Connecticut\"\n  \
             },\"admin2_division\": {\n    \"id\": 12809023,\"code\": \"US.CT.170\",\"name\": \
             \"South Central Connecticut Planning Region\"\n  },\"timezone\": \
             \"America/New_York\",\"names\": {\n    \"en\": \"East Haven\"\n  \
             },\"country_names\": {\n    \"en\": \"United States\"\n  },\"admin1_names\": {\n    \
             \"en\": \"Connecticut\"\n  },\"admin2_names\": null,\"population\": 29257\n},\n  \
             \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\"iso3\": \
             \"USA\",\"iso_numeric\": \"840\",\"fips\": \"US\",\"name\": \"United \
             States\",\"capital\": \"Washington\",\"area\": \"9629091\",\"population\": \
             327167434,\"continent\": \"NA\",\"tld\": \".us\",\"currency_code\": \
             \"USD\",\"currency_name\": \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n},\n  \
             \"us_fips_codes\":{\n  \"us_state_code\": \"CT\",\n  \"us_state_name\": \
             \"Connecticut\",\n  \"us_state_fips_code\": \"09\",\n  \"us_county\": \"South \
             Central Connecticut Planning Region\",\n  \"us_county_fips_code\": \"170\"\n}\n}"
        ],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"],
        svec![
            "{\n  \"cityrecord\":{\n  \"id\": 1703417,\"name\": \"Makati City\",\"latitude\": \
             14.550270080566406,\"longitude\": 121.0326919555664,\"country\": {\n    \"id\": \
             1694008,\"code\": \"PH\",\"name\": \"Philippines\"\n  },\"admin_division\": {\n    \
             \"id\": 7521311,\"code\": \"PH.NCR\",\"name\": \"National Capital Region\"\n  \
             },\"admin2_division\": {\n    \"id\": 11395838,\"code\": \
             \"PH.NCR.137600000\",\"name\": \"Southern Manila District\"\n  },\"timezone\": \
             \"Asia/Manila\",\"names\": {\n    \"en\": \"Makati City\"\n  },\"country_names\": \
             {\n    \"en\": \"Philippines\"\n  },\"admin1_names\": {\n    \"en\": \"National \
             Capital Region\"\n  },\"admin2_names\": null,\"population\": 510383\n},\n  \
             \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"PH\",\"iso3\": \
             \"PHL\",\"iso_numeric\": \"608\",\"fips\": \"RP\",\"name\": \
             \"Philippines\",\"capital\": \"Manila\",\"area\": \"300000\",\"population\": \
             106651922,\"continent\": \"AS\",\"tld\": \".ph\",\"currency_code\": \
             \"PHP\",\"currency_name\": \"Peso\",\"phone\": \"63\",\"postal_code_format\": \
             \"####\",\"postal_code_regex\": \"^(\\\\d{4})$\",\"languages\": \
             \"tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,\
             yka,mta,abx\",\"geonameid\": 1694008,\"neighbours\": \"\",\"equivalent_fips_code\": \
             \"\"\n  },\"names\": {\n    \"en\": \"Philippines\"\n  },\"capital_names\": {\n    \
             \"en\": \"Manila\"\n  }\n},\n  \"us_fips_codes\":{\n  \"us_state_code\": null,\n  \
             \"us_state_name\": \"National Capital Region\",\n  \"us_state_fips_code\": null,\n  \
             \"us_county\": \"Southern Manila District\",\n  \"us_county_fips_code\": null\n}\n}"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_invalid_dynfmt() {
    let wrk = Workdir::new("geocode_suggest_invalid_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("{latitude}:{longitude} - {name}, {admin1} {invalid_field}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["Invalid dynfmt template."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt() {
    let wrk = Workdir::new("geocode_suggest_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-state-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Elmhurst, Illinois US"],
        svec!["East Lake, Florida US"],
        svec!["New York, New York US"],
        svec!["East Haven, Connecticut US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["40.71427, -74.00597"], // suggest doesn't work with lat, long
        svec!["Makati City, National Capital Region PH"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt_json() {
    let wrk = Workdir::new("geocode_suggest_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%json")
        .arg("data.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);

    let expected = r######"Location
"{""cityrecord"":{""id"":4891010,""name"":""Elmhurst"",""latitude"":41.899471282958984,""longitude"":-87.94033813476562,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":4896861,""code"":""US.IL"",""name"":""Illinois""},""admin2_division"":{""id"":4890213,""code"":""US.IL.043"",""name"":""DuPage County""},""timezone"":""America/Chicago"",""names"":{""en"":""Elmhurst""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""Illinois""},""admin2_names"":{""en"":""DuPage County""},""population"":45957}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}}, ""us_fips_codes"":{""us_state_code"":""IL"",""us_state_name"":""Illinois"",""us_state_fips_code"":""17"",""us_county"":""DuPage County"",""us_county_fips_code"":""043""}}"
"{""cityrecord"":{""id"":4154008,""name"":""East Lake"",""latitude"":28.110849380493164,""longitude"":-82.69481658935547,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":4155751,""code"":""US.FL"",""name"":""Florida""},""admin2_division"":{""id"":4168618,""code"":""US.FL.103"",""name"":""Pinellas County""},""timezone"":""America/New_York"",""names"":null,""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""Florida""},""admin2_names"":{""en"":""Pinellas""},""population"":30962}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}}, ""us_fips_codes"":{""us_state_code"":""FL"",""us_state_name"":""Florida"",""us_state_fips_code"":""12"",""us_county"":""Pinellas"",""us_county_fips_code"":""103""}}"
"{""cityrecord"":{""id"":5128581,""name"":""New York City"",""latitude"":40.714271545410156,""longitude"":-74.00596618652344,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":5128638,""code"":""US.NY"",""name"":""New York""},""admin2_division"":null,""timezone"":""America/New_York"",""names"":{""en"":""New York""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""New York""},""admin2_names"":null,""population"":8804190}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}}, ""us_fips_codes"":{""us_state_code"":""NY"",""us_state_name"":""New York"",""us_state_fips_code"":""36"",""us_county"":"""",""us_county_fips_code"":null}}"
"{""cityrecord"":{""id"":4833425,""name"":""East Haven"",""latitude"":41.27621078491211,""longitude"":-72.8684310913086,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":4831725,""code"":""US.CT"",""name"":""Connecticut""},""admin2_division"":{""id"":12809023,""code"":""US.CT.170"",""name"":""South Central Connecticut Planning Region""},""timezone"":""America/New_York"",""names"":{""en"":""East Haven""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""Connecticut""},""admin2_names"":null,""population"":29257}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}}, ""us_fips_codes"":{""us_state_code"":""CT"",""us_state_name"":""Connecticut"",""us_state_fips_code"":""09"",""us_county"":""South Central Connecticut Planning Region"",""us_county_fips_code"":""170""}}"
This is not a Location and it will not be geocoded
"40.71427, -74.00597"
"{""cityrecord"":{""id"":1703417,""name"":""Makati City"",""latitude"":14.550270080566406,""longitude"":121.0326919555664,""country"":{""id"":1694008,""code"":""PH"",""name"":""Philippines""},""admin_division"":{""id"":7521311,""code"":""PH.NCR"",""name"":""National Capital Region""},""admin2_division"":{""id"":11395838,""code"":""PH.NCR.137600000"",""name"":""Southern Manila District""},""timezone"":""Asia/Manila"",""names"":{""en"":""Makati City""},""country_names"":{""en"":""Philippines""},""admin1_names"":{""en"":""National Capital Region""},""admin2_names"":null,""population"":510383}, ""countryrecord"":{""info"":{""iso"":""PH"",""iso3"":""PHL"",""iso_numeric"":""608"",""fips"":""RP"",""name"":""Philippines"",""capital"":""Manila"",""area"":""300000"",""population"":106651922,""continent"":""AS"",""tld"":"".ph"",""currency_code"":""PHP"",""currency_name"":""Peso"",""phone"":""63"",""postal_code_format"":""####"",""postal_code_regex"":""^(\\d{4})$"",""languages"":""tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,yka,mta,abx"",""geonameid"":1694008,""neighbours"":"""",""equivalent_fips_code"":""""},""names"":{""en"":""Philippines""},""capital_names"":{""en"":""Manila""}}, ""us_fips_codes"":{""us_state_code"":null,""us_state_name"":""National Capital Region"",""us_state_fips_code"":null,""us_county"":""Southern Manila District"",""us_county_fips_code"":null}}""######;
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt_cityrecord() {
    let wrk = Workdir::new("geocode_suggest_fmt_cityrecord");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%cityrecord")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec![
            "ArchivedCitiesRecord { id: 4891010, name: \"Elmhurst\", latitude: 41.89947, \
             longitude: -87.94034, country: Some(ArchivedCountry { id: 6252001, code: \"US\", \
             name: \"United States\" }), admin_division: Some(ArchivedAdminDivision { id: \
             4896861, code: \"US.IL\", name: \"Illinois\" }), admin2_division: \
             Some(ArchivedAdminDivision { id: 4890213, code: \"US.IL.043\", name: \"DuPage \
             County\" }), timezone: \"America/Chicago\", names: Some({\"en\": \"Elmhurst\"}), \
             country_names: Some({\"en\": \"United States\"}), admin1_names: Some({\"en\": \
             \"Illinois\"}), admin2_names: Some({\"en\": \"DuPage County\"}), population: 45957 }"
        ],
        svec![
            "ArchivedCitiesRecord { id: 4154008, name: \"East Lake\", latitude: 28.11085, \
             longitude: -82.69482, country: Some(ArchivedCountry { id: 6252001, code: \"US\", \
             name: \"United States\" }), admin_division: Some(ArchivedAdminDivision { id: \
             4155751, code: \"US.FL\", name: \"Florida\" }), admin2_division: \
             Some(ArchivedAdminDivision { id: 4168618, code: \"US.FL.103\", name: \"Pinellas \
             County\" }), timezone: \"America/New_York\", names: None, country_names: \
             Some({\"en\": \"United States\"}), admin1_names: Some({\"en\": \"Florida\"}), \
             admin2_names: Some({\"en\": \"Pinellas\"}), population: 30962 }"
        ],
        svec![
            "ArchivedCitiesRecord { id: 5128581, name: \"New York City\", latitude: 40.71427, \
             longitude: -74.00597, country: Some(ArchivedCountry { id: 6252001, code: \"US\", \
             name: \"United States\" }), admin_division: Some(ArchivedAdminDivision { id: \
             5128638, code: \"US.NY\", name: \"New York\" }), admin2_division: None, timezone: \
             \"America/New_York\", names: Some({\"en\": \"New York\"}), country_names: \
             Some({\"en\": \"United States\"}), admin1_names: Some({\"en\": \"New York\"}), \
             admin2_names: None, population: 8804190 }"
        ],
        svec![
            "ArchivedCitiesRecord { id: 4833425, name: \"East Haven\", latitude: 41.27621, \
             longitude: -72.86843, country: Some(ArchivedCountry { id: 6252001, code: \"US\", \
             name: \"United States\" }), admin_division: Some(ArchivedAdminDivision { id: \
             4831725, code: \"US.CT\", name: \"Connecticut\" }), admin2_division: \
             Some(ArchivedAdminDivision { id: 12809023, code: \"US.CT.170\", name: \"South \
             Central Connecticut Planning Region\" }), timezone: \"America/New_York\", names: \
             Some({\"en\": \"East Haven\"}), country_names: Some({\"en\": \"United States\"}), \
             admin1_names: Some({\"en\": \"Connecticut\"}), admin2_names: None, population: 29257 \
             }"
        ],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["40.71427, -74.00597"],
        svec![
            "ArchivedCitiesRecord { id: 1703417, name: \"Makati City\", latitude: 14.55027, \
             longitude: 121.03269, country: Some(ArchivedCountry { id: 1694008, code: \"PH\", \
             name: \"Philippines\" }), admin_division: Some(ArchivedAdminDivision { id: 7521311, \
             code: \"PH.NCR\", name: \"National Capital Region\" }), admin2_division: \
             Some(ArchivedAdminDivision { id: 11395838, code: \"PH.NCR.137600000\", name: \
             \"Southern Manila District\" }), timezone: \"Asia/Manila\", names: Some({\"en\": \
             \"Makati City\"}), country_names: Some({\"en\": \"Philippines\"}), admin1_names: \
             Some({\"en\": \"National Capital Region\"}), admin2_names: None, population: 510383 }"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse() {
    let wrk = Workdir::new("geocode_reverse");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec![
                "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
                 geocoded."
            ],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec![
                "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should \
                 NOT be geocoded."
            ],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["The Bronx, New York US"],
        svec!["Brooklyn, New York US"],
        svec!["Manhattan, New York US"],
        svec!["Cliffside Park, New Jersey US"],
        svec!["Cliffside Park, New Jersey US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brooklyn, New York US"],
        svec!["95.213424, 190,1234565"],
        svec![
            "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT \
             be geocoded."
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring() {
    let wrk = Workdir::new("geocode_reverse_fmtstring");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-state-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["The Bronx, New York US"],
        svec!["Brooklyn, New York US"],
        svec!["Manhattan, New York US"],
        svec!["Cliffside Park, New Jersey US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-admin1-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Barcelona, Catalonia ES"],
        svec!["Amsterdam, North Holland NL"],
        svec!["Berlin, State of Berlin DE"],
        svec!["Makati City, National Capital Region PH"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl_dynfmt() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("pop: {population} tz: {timezone}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["pop: 1686208 tz: Europe/Madrid"],
        svec!["pop: 741636 tz: Europe/Amsterdam"],
        svec!["pop: 3426354 tz: Europe/Berlin"],
        svec!["pop: 510383 tz: Asia/Manila"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl_invalid_dynfmt() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl_invalid_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("pop: {population} tz: {timezone} {doesnotexistfield}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_dyncols_fmt() {
    let wrk = Workdir::new("geocode_suggest_dyncols_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args([
            "-f",
            "%dyncols: {city_col:name}, {state_col:admin1}, {county_col:admin2}, \
             {country_col:country}, {continent_col:continent}, {currency_col:currency_code}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec![
            "Location",
            "city_col",
            "state_col",
            "county_col",
            "country_col",
            "continent_col",
            "currency_col"
        ],
        svec![
            "Melrose, New York",
            "Melrose Park",
            "Illinois",
            "Cook",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "East Flatbush, New York",
            "East Lake",
            "Florida",
            "Pinellas",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "Manhattan, New York",
            "New York",
            "New York",
            "",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "Brooklyn, New York",
            "Brooklyn Park",
            "Minnesota",
            "Hennepin",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "East Harlem, New York",
            "East Haven",
            "Connecticut",
            "South Central Connecticut Planning Region",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "This is not a Location and it will not be geocoded",
            "",
            "",
            "",
            "",
            "",
            ""
        ],
        svec![
            "Jersey City, New Jersey",
            "Jersey City",
            "New Jersey",
            "Hudson",
            "US",
            "NA",
            "USD"
        ],
        svec!["95.213424, 190,1234565", "", "", "", "", "", ""],
        svec![
            "Makati, Metro Manila, Philippines",
            "Makati City",
            "National Capital Region",
            "Southern Manila District",
            "PH",
            "AS",
            "PHP"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_reverse_dyncols_fmt() {
    let wrk = Workdir::new("geocode_reverse_dyncols_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec![
                "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
                 geocoded."
            ],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec![
                "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should \
                 NOT be geocoded."
            ],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .args([
            "-f",
            "%dyncols: {city_col:name}, {tz_col:timezone}, {capital_col:capital}, \
             {pop_col:population}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location", "city_col", "tz_col", "capital_col", "pop_col"],
        svec![
            "40.812126, -73.9041813",
            "The Bronx",
            "America/New_York",
            "Washington",
            "1385108"
        ],
        svec![
            "40.66472342, -73.93867227",
            "Brooklyn",
            "America/New_York",
            "Washington",
            "2736074"
        ],
        svec![
            "(40.766672, -73.9568128)",
            "Manhattan",
            "America/New_York",
            "Washington",
            "1487536"
        ],
        svec![
            "(  40.819342, -73.9532127    )",
            "Cliffside Park",
            "America/New_York",
            "Washington",
            "24857"
        ],
        svec![
            "< 40.819342,-73.9532127 >",
            "Cliffside Park",
            "America/New_York",
            "Washington",
            "24857"
        ],
        svec![
            "This is not a Location and it will not be geocoded",
            "",
            "",
            "",
            ""
        ],
        svec![
            "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
             geocoded.",
            "Brooklyn",
            "America/New_York",
            "Washington",
            "2736074"
        ],
        svec!["95.213424, 190,1234565", "", "", "", ""],
        svec![
            "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT \
             be geocoded.",
            "",
            "",
            "",
            ""
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo() {
    let wrk = Workdir::new("geocode_countryinfo");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo").arg("Country").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec!["United States"],
        svec!["Canada"],
        svec!["Mexico"],
        svec!["United States"],
        svec!["China"],
        svec!["This is not a country and it will not be geocoded"],
        svec!["Philippines"],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"], // passed thru as its not a valid country code
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo_invalid_result() {
    let wrk = Workdir::new("geocode_countryinfo_invalid_result");
    wrk.create(
        "data.csv",
        vec![svec!["Country"], svec!["US"], svec!["ZZ"], svec![""]],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args(["--invalid-result", "<BAD>"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec!["United States"],
        svec!["<BAD>"],
        svec![""], // an empty cell is left untouched
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo_dyncols_rejected() {
    // countryinfo used to add the %dyncols headers but never the fields, so the CSV writer
    // aborted on the first row
    let wrk = Workdir::new("geocode_countryinfo_dyncols_rejected");
    wrk.create("data.csv", vec![svec!["Country"], svec!["US"]]);
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args(["-f", "%dyncols: {cap:capital}, {cont:continent}"])
        .arg("data.csv");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not supported by the countryinfo subcommands"));
    assert!(output.stdout.is_empty());
}

#[test]
#[serial]
fn geocode_suggest_dyncols_malformed_pair_rejected() {
    // a malformed pair (no colon separator) used to be silently dropped
    let wrk = Workdir::new("geocode_suggest_dyncols_malformed_pair_rejected");
    wrk.create("data.csv", vec![svec!["city"], svec!["Brooklyn"]]);
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("city")
        .args(["-f", "%dyncols: {city_col name}, {state_col:admin1}"])
        .arg("data.csv");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid '%dyncols:' pair"));
}

#[test]
#[serial]
fn geocode_suggest_dyncols_empty_rejected() {
    // an empty "%dyncols:" list used to fall back to non-dyncols mode
    let wrk = Workdir::new("geocode_suggest_dyncols_empty_rejected");
    wrk.create("data.csv", vec![svec!["city"], svec!["Brooklyn"]]);
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("city")
        .args(["-f", "%dyncols: "])
        .arg("data.csv");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("expected one or more"));
}

#[test]
#[serial]
fn geocode_countryinfo_formatstr() {
    let wrk = Workdir::new("geocode_countryinfo_formatstr");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args([
            "--formatstr",
            "{country_name} Pop: {country_population} in {continent} using {currency_name} all in \
             {area} square kms.",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec!["United States Pop: 327167434 in NA using Dollar all in 9629091 square kms."],
        svec!["Canada Pop: 37058856 in NA using Dollar all in 9984670 square kms."],
        svec!["Mexico Pop: 126190788 in NA using Peso all in 1972550 square kms."],
        svec!["United States Pop: 327167434 in NA using Dollar all in 9629091 square kms."],
        svec!["China Pop: 1411778724 in AS using Yuan Renminbi all in 9596960 square kms."],
        svec!["This is not a country and it will not be geocoded"],
        svec!["Philippines Pop: 106651922 in AS using Peso all in 300000 square kms."],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_countryinfo_formatstr_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args(["--formatstr", "%pretty-json"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);

    let expected = vec![
        svec!["Country"],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"US\",\"iso3\": \"USA\",\"iso_numeric\": \
             \"840\",\"fips\": \"US\",\"name\": \"United States\",\"capital\": \
             \"Washington\",\"area\": \"9629091\",\"population\": 327167434,\"continent\": \
             \"NA\",\"tld\": \".us\",\"currency_code\": \"USD\",\"currency_name\": \
             \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}"
        ],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"CA\",\"iso3\": \"CAN\",\"iso_numeric\": \
             \"124\",\"fips\": \"CA\",\"name\": \"Canada\",\"capital\": \"Ottawa\",\"area\": \
             \"9984670\",\"population\": 37058856,\"continent\": \"NA\",\"tld\": \
             \".ca\",\"currency_code\": \"CAD\",\"currency_name\": \"Dollar\",\"phone\": \
             \"1\",\"postal_code_format\": \"@#@ #@#\",\"postal_code_regex\": \
             \"^([ABCEGHJKLMNPRSTVXY]\\\\d[ABCEGHJKLMNPRSTVWXYZ]) \
             ?(\\\\d[ABCEGHJKLMNPRSTVWXYZ]\\\\d)$ \",\"languages\": \
             \"en-CA,fr-CA,iu\",\"geonameid\": 6251999,\"neighbours\": \
             \"US\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"Canada\"\n  \
             },\"capital_names\": {\n    \"en\": \"Ottawa\"\n  }\n}"
        ],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"MX\",\"iso3\": \"MEX\",\"iso_numeric\": \
             \"484\",\"fips\": \"MX\",\"name\": \"Mexico\",\"capital\": \"Mexico City\",\"area\": \
             \"1972550\",\"population\": 126190788,\"continent\": \"NA\",\"tld\": \
             \".mx\",\"currency_code\": \"MXN\",\"currency_name\": \"Peso\",\"phone\": \
             \"52\",\"postal_code_format\": \"#####\",\"postal_code_regex\": \
             \"^(\\\\d{5})$\",\"languages\": \"es-MX\",\"geonameid\": 3996063,\"neighbours\": \
             \"GT,US,BZ\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \
             \"Mexico\"\n  },\"capital_names\": {\n    \"en\": \"Mexico City\"\n  }\n}"
        ],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"US\",\"iso3\": \"USA\",\"iso_numeric\": \
             \"840\",\"fips\": \"US\",\"name\": \"United States\",\"capital\": \
             \"Washington\",\"area\": \"9629091\",\"population\": 327167434,\"continent\": \
             \"NA\",\"tld\": \".us\",\"currency_code\": \"USD\",\"currency_name\": \
             \"Dollar\",\"phone\": \"1\",\"postal_code_format\": \
             \"#####-####\",\"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\"languages\": \
             \"en-US,es-US,haw,fr\",\"geonameid\": 6252001,\"neighbours\": \
             \"CA,MX,CU\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \"United \
             States\"\n  },\"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}"
        ],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"CN\",\"iso3\": \"CHN\",\"iso_numeric\": \
             \"156\",\"fips\": \"CH\",\"name\": \"China\",\"capital\": \"Beijing\",\"area\": \
             \"9596960\",\"population\": 1411778724,\"continent\": \"AS\",\"tld\": \
             \".cn\",\"currency_code\": \"CNY\",\"currency_name\": \"Yuan Renminbi\",\"phone\": \
             \"86\",\"postal_code_format\": \"######\",\"postal_code_regex\": \
             \"^(\\\\d{6})$\",\"languages\": \"zh-CN,yue,wuu,dta,ug,za\",\"geonameid\": \
             1814991,\"neighbours\": \
             \"LA,BT,TJ,KZ,MN,AF,NP,MM,KG,PK,KP,RU,VN,IN\",\"equivalent_fips_code\": \"\"\n  \
             },\"names\": {\n    \"en\": \"China\"\n  },\"capital_names\": {\n    \"en\": \
             \"Beijing\"\n  }\n}"
        ],
        svec!["This is not a country and it will not be geocoded"],
        svec![
            "{\n  \"info\": {\n    \"iso\": \"PH\",\"iso3\": \"PHL\",\"iso_numeric\": \
             \"608\",\"fips\": \"RP\",\"name\": \"Philippines\",\"capital\": \"Manila\",\"area\": \
             \"300000\",\"population\": 106651922,\"continent\": \"AS\",\"tld\": \
             \".ph\",\"currency_code\": \"PHP\",\"currency_name\": \"Peso\",\"phone\": \
             \"63\",\"postal_code_format\": \"####\",\"postal_code_regex\": \
             \"^(\\\\d{4})$\",\"languages\": \
             \"tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,\
             yka,mta,abx\",\"geonameid\": 1694008,\"neighbours\": \"\",\"equivalent_fips_code\": \
             \"\"\n  },\"names\": {\n    \"en\": \"Philippines\"\n  },\"capital_names\": {\n    \
             \"en\": \"Manila\"\n  }\n}"
        ],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow() {
    let wrk = Workdir::new("geocode_countryinfonow");
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow").arg("US");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["Location"], svec!["United States"]];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow_formatstr() {
    let wrk = Workdir::new("geocode_countryinfonow_formatstr");

    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow").arg("cA").args([
        "--formatstr",
        "{country_name} Pop: {country_population} in {continent} using {currency_name} all in \
         {area} square kms.",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Canada Pop: 37058856 in NA using Dollar all in 9984670 square kms."],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_countryinfonow_formatstr_pretty_json");
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow")
        .arg("mx")
        .args(["--formatstr", "%pretty-json"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = "{\n  \"info\": {\n    \"iso\": \"MX\",\"iso3\": \"MEX\",\"iso_numeric\": \
                    \"484\",\"fips\": \"MX\",\"name\": \"Mexico\",\"capital\": \"Mexico \
                    City\",\"area\": \"1972550\",\"population\": 126190788,\"continent\": \
                    \"NA\",\"tld\": \".mx\",\"currency_code\": \"MXN\",\"currency_name\": \
                    \"Peso\",\"phone\": \"52\",\"postal_code_format\": \
                    \"#####\",\"postal_code_regex\": \"^(\\\\d{5})$\",\"languages\": \
                    \"es-MX\",\"geonameid\": 3996063,\"neighbours\": \
                    \"GT,US,BZ\",\"equivalent_fips_code\": \"\"\n  },\"names\": {\n    \"en\": \
                    \"Mexico\"\n  },\"capital_names\": {\n    \"en\": \"Mexico City\"\n  }\n}";
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup() {
    let wrk = Workdir::new("geocode_iplookup");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["8.8.8.8"],        // Google DNS
            svec!["1.1.1.1"],        // Cloudflare DNS
            svec!["208.67.222.222"], // OpenDNS
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"],        // Private IP
            svec!["127.0.0.1"],          // Localhost
            svec!["invalid-ip-address"], // Invalid IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup").arg("IP").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["IP"],
        svec!["8.8.8.8"],
        svec!["1.1.1.1"],
        svec!["San Francisco, California US"],
        svec!["This is not an IP and it will not be geocoded"],
        svec!["192.168.1.1"],        // Private IP returns as-is
        svec!["127.0.0.1"],          // Localhost returns as-is
        svec!["invalid-ip-address"], // Invalid IP returns as-is
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_formatstr() {
    let wrk = Workdir::new("geocode_iplookup_formatstr");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args(["--formatstr", "%city-state-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["IP"],
        svec!["Ashburn, Virginia US"],
        svec!["This is not an IP and it will not be geocoded"],
        svec!["192.168.1.1"], // Private IP returns as-is
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_formatstr_dynfmt() {
    let wrk = Workdir::new("geocode_iplookup_formatstr_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args([
            "--formatstr",
            "City: {name}, State: {admin1}, Country: {country} - {timezone}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["IP"],
        svec!["City: Ashburn, State: Virginia, Country: US - America/New_York"],
        svec!["This is not an IP and it will not be geocoded"],
        svec!["192.168.1.1"], // Private IP returns as-is
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_formatstr_json() {
    let wrk = Workdir::new("geocode_iplookup_formatstr_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args(["--formatstr", "%json"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // The JSON output will contain the full city record, so we just check that it contains expected
    // fields
    assert!(got[1][0].contains("\"cityrecord\""));
    assert!(got[1][0].contains("\"countryrecord\""));
    assert!(got[1][0].contains("\"us_fips_codes\""));
    assert_eq!(got[2][0], "This is not an IP and it will not be geocoded");
    assert_eq!(got[3][0], "192.168.1.1");
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_iplookup_formatstr_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args(["--formatstr", "%pretty-json"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // The pretty JSON output will contain the full city record with proper formatting
    assert!(got[1][0].contains("\n  \"cityrecord\""));
    assert!(got[1][0].contains("\n  \"countryrecord\""));
    assert_eq!(got[2][0], "This is not an IP and it will not be geocoded");
    assert_eq!(got[3][0], "192.168.1.1");
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_formatstr_cityrecord() {
    let wrk = Workdir::new("geocode_iplookup_formatstr_cityrecord");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args(["--formatstr", "%cityrecord"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // The cityrecord output will contain the full ArchivedCitiesRecord debug format
    assert!(got[1][0].contains("ArchivedCitiesRecord"));
    assert!(got[1][0].contains("name: \"Ashburn\""));
    assert!(got[1][0].contains("latitude:"));
    assert!(got[1][0].contains("longitude:"));
    assert_eq!(got[2][0], "This is not an IP and it will not be geocoded");
    assert_eq!(got[3][0], "192.168.1.1");
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_dyncols_fmt() {
    let wrk = Workdir::new("geocode_iplookup_dyncols_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"], // Private IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args([
            "-f",
            "%dyncols: {city_col:name}, {state_col:admin1}, {country_col:country}, \
             {tz_col:timezone}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["IP", "city_col", "state_col", "country_col", "tz_col"],
        svec!["3.3.3.3", "Ashburn", "Virginia", "US", "America/New_York"],
        svec![
            "This is not an IP and it will not be geocoded",
            "",
            "",
            "",
            ""
        ],
        svec!["192.168.1.1", "", "", "", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookup_invalid_result() {
    let wrk = Workdir::new("geocode_iplookup_invalid_result");
    wrk.create(
        "data.csv",
        vec![
            svec!["IP"],
            svec!["3.3.3.3"], // Amazon
            svec!["This is not an IP and it will not be geocoded"],
            svec!["192.168.1.1"],        // Private IP
            svec!["invalid-ip-address"], // Invalid IP
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookup")
        .arg("IP")
        .args(["--invalid-result", "<NO_LOCATION>"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["IP"],
        svec!["Ashburn, Virginia US"],
        svec!["<NO_LOCATION>"],
        svec!["192.168.1.1"],
        svec!["<NO_LOCATION>"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow() {
    let wrk = Workdir::new("geocode_iplookupnow");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("3.3.3.3");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Ashburn, Virginia US: 39.04372, -77.48749"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_url() {
    let wrk = Workdir::new("geocode_iplookupnow_url");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("https://nytimes.com");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["San Francisco, California US: 37.77493, -122.41942"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_formatstr() {
    let wrk = Workdir::new("geocode_iplookupnow_formatstr");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow")
        .arg("3.3.3.3")
        .args(["--formatstr", "%city-state-country"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["Location"], svec!["Ashburn, Virginia US"]];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_formatstr_dynfmt() {
    let wrk = Workdir::new("geocode_iplookupnow_formatstr_dynfmt");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("3.3.3.3").args([
        "--formatstr",
        "City: {name}, State: {admin1}, Country: {country} - {timezone}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["City: Ashburn, State: Virginia, Country: US - America/New_York"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_formatstr_json() {
    let wrk = Workdir::new("geocode_iplookupnow_formatstr_json");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow")
        .arg("3.3.3.3")
        .args(["--formatstr", "%json"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    // The JSON output will contain the full city record
    assert!(got.contains("\"cityrecord\""));
    assert!(got.contains("\"countryrecord\""));
    assert!(got.contains("\"us_fips_codes\""));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_iplookupnow_formatstr_pretty_json");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow")
        .arg("3.3.3.3")
        .args(["--formatstr", "%pretty-json"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    // The pretty JSON output will contain the full city record with proper formatting
    assert!(got.contains("\n  \"cityrecord\""));
    assert!(got.contains("\n  \"countryrecord\""));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_formatstr_cityrecord() {
    let wrk = Workdir::new("geocode_iplookupnow_formatstr_cityrecord");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow")
        .arg("3.3.3.3")
        .args(["--formatstr", "%cityrecord"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    // The cityrecord output will contain the full ArchivedCitiesRecord debug format
    assert!(got.contains("ArchivedCitiesRecord"));
    assert!(got.contains("Ashburn"));
    assert!(got.contains("latitude:"));
    assert!(got.contains("longitude:"));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_private_ip() {
    let wrk = Workdir::new("geocode_iplookupnow_private_ip");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("192.168.1.1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    // Private IP should return the IP address as-is
    assert!(got.contains("192.168.1.1"));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_localhost() {
    let wrk = Workdir::new("geocode_iplookupnow_localhost");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("127.0.0.1");

    let got: String = wrk.stdout_on_success(&mut cmd);
    // Localhost should return the IP address as-is
    assert!(got.contains("127.0.0.1"));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_invalid_ip() {
    let wrk = Workdir::new("geocode_iplookupnow_invalid_ip");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("invalid-ip-address");

    let got: String = wrk.stdout_on_success(&mut cmd);
    // Invalid IP should return the input as-is
    assert!(got.contains("invalid-ip-address"));
}

#[test]
#[serial]
#[ignore = "GeoIP2 database requires a license and is not included in the repo"]
fn geocode_iplookupnow_invalid_url() {
    let wrk = Workdir::new("geocode_iplookupnow_invalid_url");
    let mut cmd = wrk.command("geocode");
    cmd.arg("iplookupnow").arg("not-a-valid-url");

    let got: String = wrk.stdout_on_success(&mut cmd);
    // Invalid URL should return the input as-is
    assert!(got.contains("not-a-valid-url"));
}

// ---------------------------------------------------------------------------
// OpenCage online geocoding tests.
// The geocode_opencage* tests below hit the live OpenCage API, so they are
// #[ignore]'d by default. Run them with a free OpenCage API key:
//   QSV_OPENCAGE_API_KEY=<key> cargo test -F all_features geocode_opencage -- --ignored
// ---------------------------------------------------------------------------

#[test]
#[serial]
fn geocode_opencage_missing_api_key() {
    // this runs in CI: no --api-key and no QSV_OPENCAGE_API_KEY -> clean error
    let wrk = Workdir::new("geocode_opencage_missing_api_key");
    let mut cmd = wrk.command("geocode");
    cmd.env_remove("QSV_OPENCAGE_API_KEY");
    cmd.arg("opencagenow").arg("Brooklyn, NY");

    // capture a single run, then assert on both status and stderr
    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("requires an API key"));
}

#[test]
#[serial]
fn geocode_opencage_admin1_rejected() {
    // this runs in CI: --admin1 is not supported by the opencage subcommands
    let wrk = Workdir::new("geocode_opencage_admin1_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["--admin1", "US.NY"]);

    // capture a single run, then assert on both status and stderr
    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--admin1"));
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencage() {
    let wrk = Workdir::new("geocode_opencage");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["1600 Pennsylvania Ave NW, Washington DC"], // forward
            svec!["40.71427, -74.00597"],                     // reverse (auto-detected)
            svec![""],                                        // empty -> passthrough
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencage").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 4);
    assert_eq!(got[0], svec!["Location"]);
    // forward & reverse rows are geocoded (changed from the input)
    assert_ne!(got[1][0], "1600 Pennsylvania Ave NW, Washington DC");
    assert!(!got[1][0].is_empty());
    assert_ne!(got[2][0], "40.71427, -74.00597");
    assert!(!got[2][0].is_empty());
    // empty cell is passed through untouched
    assert_eq!(got[3], svec![""]);
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencagenow_forward() {
    let wrk = Workdir::new("geocode_opencagenow_forward");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["-f", "%location"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    // %location format is "(lat, long)"
    assert!(got.contains('(') && got.contains(','));
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencagenow_reverse() {
    let wrk = Workdir::new("geocode_opencagenow_reverse");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow").arg("40.71427, -74.00597");

    let got: String = wrk.stdout_on_success(&mut cmd);
    // a reverse geocode of a NYC coordinate should mention New York
    assert!(got.to_lowercase().contains("new york") || got.to_lowercase().contains("york"));
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencagenow_json() {
    let wrk = Workdir::new("geocode_opencagenow_json");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["-f", "%json"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    // %json on a now subcommand emits valid JSON (no "Location" header)
    let parsed: serde_json::Value = serde_json::from_str(got.trim()).unwrap();
    assert!(parsed.get("formatted").is_some());
    assert!(parsed.get("geometry").is_some());
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencagenow_dynfmt() {
    let wrk = Workdir::new("geocode_opencagenow_dynfmt");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["-f", "{components.country}"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    assert!(got.to_lowercase().contains("united states"));
}

#[test]
#[serial]
fn geocode_opencage_dyncols_invalid_key_rejected() {
    // this runs in CI: an invalid %dyncols: key is rejected before any API call
    let wrk = Workdir::new("geocode_opencage_dyncols_invalid_key_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["-f", "%dyncols: {city:components.city}, {bad:not_a_field}"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid '%dyncols:' key"));
}

#[test]
#[serial]
fn geocode_opencage_dyncols_bare_prefix_rejected() {
    // this runs in CI: a bare "components."/"annotations." prefix with no
    // field suffix is rejected before any API call - it has no field to
    // resolve and would otherwise silently yield an empty column. The
    // components. and annotations. branches are checked independently
    // because validation short-circuits on the first invalid key.
    for bad_key in ["components.", "annotations."] {
        let wrk = Workdir::new("geocode_opencage_dyncols_bare_prefix_rejected");
        let mut cmd = wrk.command("geocode");
        cmd.arg("opencagenow")
            .arg("Brooklyn, NY")
            .args(["--api-key", "dummy-key-for-arg-validation"])
            .args(["-f", &format!("%dyncols: {{bad:{bad_key}}}")]);

        let output = wrk.output(&mut cmd);
        assert!(
            !output.status.success(),
            "bare prefix '{bad_key}' should be rejected"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Invalid '%dyncols:' key"),
            "bare prefix '{bad_key}' should fail with \"Invalid '%dyncols:' key\", got: {stderr}"
        );
    }
}

#[test]
#[serial]
fn geocode_opencage_dyncols_new_column_rejected() {
    // this runs in CI: %dyncols: cannot be combined with --new-column
    let wrk = Workdir::new("geocode_opencage_dyncols_new_column_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["-c", "geocoded"])
        .args(["-f", "%dyncols: {city:components.city}"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--new-column"));
}

#[test]
#[serial]
fn geocode_opencage_dyncols_empty_rejected() {
    // this runs in CI: a "%dyncols:" formatstr with no col_name:key pairs is rejected
    let wrk = Workdir::new("geocode_opencage_dyncols_empty_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["-f", "%dyncols:"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("expected one or more"));
}

#[test]
#[serial]
fn geocode_opencage_dyncols_malformed_pair_rejected() {
    // this runs in CI: a malformed pair (no colon separator) is rejected, not
    // silently dropped
    let wrk = Workdir::new("geocode_opencage_dyncols_malformed_pair_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["-f", "%dyncols: {city components.city}"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid '%dyncols:' pair"));
}

#[test]
#[serial]
fn geocode_opencage_dyncols_empty_col_name_rejected() {
    // this runs in CI: a pair with an empty column name is rejected
    let wrk = Workdir::new("geocode_opencage_dyncols_empty_col_name_rejected");
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencagenow")
        .arg("Brooklyn, NY")
        .args(["--api-key", "dummy-key-for-arg-validation"])
        .args(["-f", "%dyncols: {:components.city}"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("column name is empty"));
}

#[test]
#[serial]
fn geocode_opencage_transient_failure_reported() {
    // this runs in CI without network access to OpenCage: a dead proxy turns every lookup into a
    // network (transient) error. Those used to reach only log::warn!, so the run looked clean.
    let wrk = Workdir::new("geocode_opencage_transient_failure_reported");
    wrk.create("data.csv", vec![svec!["address"], svec!["Brooklyn, NY"]]);
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencage")
        .arg("address")
        .arg("data.csv")
        .args(["--api-key", "dummy-key-for-transient-test"])
        .arg("--no-cache")
        .args(["--timeout", "5"])
        .env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("https_proxy", "http://127.0.0.1:9")
        .env_remove("NO_PROXY")
        .env_remove("no_proxy");

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("1 OpenCage lookup(s) failed"),
        "stderr: {stderr}"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Brooklyn, NY"));
}

#[test]
#[serial]
fn geocode_opencage_api_key_not_logged() {
    // OpenCage takes its key as a `key=` query parameter, and reqwest's error Display embeds the
    // full request URL — so a network error used to write the key verbatim into the qsv log.
    // Synthetic key on purpose: never put a real one in a fixture.
    const KEY: &str = "EXAMPLE0000NOTAREALKEY0000EXAMPLE";
    let wrk = Workdir::new("geocode_opencage_api_key_not_logged");
    wrk.create("data.csv", vec![svec!["address"], svec!["Brooklyn, NY"]]);
    let log_dir = wrk.path("logs");
    std::fs::create_dir_all(&log_dir).unwrap();
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencage")
        .arg("address")
        .arg("data.csv")
        .args(["--api-key", KEY])
        .arg("--no-cache")
        .args(["--timeout", "5"])
        .env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
        .env("QSV_LOG_LEVEL", "warn")
        .env("QSV_LOG_DIR", &log_dir)
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("https_proxy", "http://127.0.0.1:9")
        .env_remove("NO_PROXY")
        .env_remove("no_proxy");

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains(KEY), "stderr leaked the key: {stderr}");

    let mut log = String::new();
    for entry in std::fs::read_dir(&log_dir).unwrap() {
        log.push_str(&std::fs::read_to_string(entry.unwrap().path()).unwrap_or_default());
    }
    // the failure path was really exercised (otherwise "no key in the log" proves nothing)
    assert!(log.contains("OpenCage network error"), "log: {log}");
    assert!(!log.contains(KEY), "log leaked the key: {log}");
    assert!(log.contains("key=REDACTED"), "log: {log}");
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_opencage_dyncols() {
    let wrk = Workdir::new("geocode_opencage_dyncols");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["1600 Pennsylvania Ave NW, Washington DC"], // forward
            svec![""],                                        // empty -> empty dyncols
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("opencage").arg("Location").arg("data.csv").args([
        "-f",
        "%dyncols: {city:components.city}, {country:components.country}, {addr:formatted}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 3);
    // the input column is preserved & three dyncols columns are appended
    assert_eq!(got[0], svec!["Location", "city", "country", "addr"]);
    // geocoded row: input cell unchanged, dyncols populated
    assert_eq!(got[1][0], "1600 Pennsylvania Ave NW, Washington DC");
    assert!(!got[1][3].is_empty());
    assert!(got[1][2].to_lowercase().contains("united states"));
    // empty cell: input preserved, dyncols left empty
    assert_eq!(got[2], svec!["", "", "", ""]);
}

// ---------------------------------------------------------------------------
// cache-* subcommands manage the persistent on-disk OpenCage result cache.
// The tests below isolate the cache via QSV_CACHE_DIR pointed at the Workdir,
// so they never touch the real ~/.qsv-cache. The "no cache" tests run in CI;
// the populated-cache round-trip is #[ignore]'d as it needs a live API key.
// ---------------------------------------------------------------------------

#[test]
#[serial]
fn geocode_cache_info_no_cache() {
    // cache-info on a cache dir with no OpenCage cache -> graceful "no cache" report
    let wrk = Workdir::new("geocode_cache_info_no_cache");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("cache-info");

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No OpenCage cache found"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"exists\":false"));
}

#[test]
#[serial]
fn geocode_cache_clear_no_cache() {
    // cache-clear on an empty cache dir -> succeeds, nothing to do
    let wrk = Workdir::new("geocode_cache_clear_no_cache");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("cache-clear");
    wrk.assert_success(&mut cmd);
}

#[test]
#[serial]
fn geocode_cache_prune_missing_flag() {
    // cache-prune requires --older-than -> docopt usage error
    let wrk = Workdir::new("geocode_cache_prune_missing_flag");
    let mut cmd = wrk.command("geocode");
    cmd.arg("cache-prune");
    wrk.assert_err(&mut cmd);
}

#[test]
#[serial]
fn geocode_cache_prune_bad_older_than() {
    // an unparsable --older-than value -> clean error (validated in run(),
    // independent of whether a cache exists)
    let wrk = Workdir::new("geocode_cache_prune_bad_older_than");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("cache-prune").args(["--older-than", "not-a-date"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid --older-than"));
}

#[test]
#[serial]
fn geocode_cache_older_than_rejected_elsewhere() {
    // --older-than is only valid with cache-prune -> rejected on cache-info
    let wrk = Workdir::new("geocode_cache_older_than_rejected_elsewhere");
    let mut cmd = wrk.command("geocode");
    cmd.arg("cache-info").args(["--older-than", "30d"]);

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--older-than is only valid"));
}

#[test]
#[serial]
#[ignore = "requires an OpenCage API key (set QSV_OPENCAGE_API_KEY)"]
fn geocode_cache_roundtrip() {
    // populate the cache via opencagenow, inspect it, then clear it
    let wrk = Workdir::new("geocode_cache_roundtrip");
    let cache_dir = wrk.path("").to_string_lossy().to_string();

    let mut populate = wrk.command("geocode");
    populate
        .env("QSV_CACHE_DIR", &cache_dir)
        .arg("opencagenow")
        .arg("Brooklyn, NY");
    wrk.assert_success(&mut populate);

    let mut info = wrk.command("geocode");
    info.env("QSV_CACHE_DIR", &cache_dir).arg("cache-info");
    let info_out = wrk.output(&mut info);
    assert!(info_out.status.success());
    let info_json = String::from_utf8_lossy(&info_out.stdout);
    assert!(info_json.contains("\"exists\":true"));
    assert!(!info_json.contains("\"entry_count\":0"));

    let mut clear = wrk.command("geocode");
    clear.env("QSV_CACHE_DIR", &cache_dir).arg("cache-clear");
    wrk.assert_success(&mut clear);

    let mut info2 = wrk.command("geocode");
    info2.env("QSV_CACHE_DIR", &cache_dir).arg("cache-info");
    let info2_out = wrk.output(&mut info2);
    assert!(info2_out.status.success());
    let info2_json = String::from_utf8_lossy(&info2_out.stdout);
    assert!(info2_json.contains("\"entry_count\":0"));
}

#[test]
#[serial]
fn geocode_index_load_accepts_a_valid_index() {
    // issue #4431: index-load rejected EVERY index, including qsv's own published prebuilt,
    // because it gated on `EngineData::metadata` from `Storage::load_from`, which never populates
    // it. There was no success-path test, which is why it survived 37 releases. Covers both forms:
    // the numeric shortcut, and an explicit .rkyv path.
    let wrk = Workdir::new("geocode_index_load_accepts_a_valid_index");

    // cache dir deliberately SEPARATE from the working dir, so the working dir doubles as the
    // check that a shortcut download stages nothing next to wherever the command was run
    let cache_dir = wrk.path("cache");
    std::fs::create_dir_all(&cache_dir).unwrap();
    let cache = cache_dir.to_string_lossy().to_string();

    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", &cache);
    cmd.arg("index-load").arg("15000");
    let out = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "the prebuilt cities15000 index must load: {stderr}"
    );
    assert!(
        !stderr.contains("is invalid"),
        "a valid index must not be called invalid: {stderr}"
    );

    // the shortcut stages its payload in the CACHE dir - never beside the command's working
    // directory, which used to collect a bare `15000` and `15000.rkyv`
    for stray in ["15000", "15000.rkyv"] {
        assert!(
            !wrk.path(stray).exists(),
            "index-load must not write {stray} into the working directory"
        );
    }
    let decompressed = cache_dir.join("cities15000.rkyv");
    assert!(
        decompressed.exists(),
        "the shortcut must stage its index in the cache dir"
    );
    {
        let mut cmd_2 = wrk.command("geocode");
        cmd_2.env("QSV_CACHE_DIR", &cache);
        cmd_2.arg("index-load").arg(&decompressed);
        let out2 = wrk.output(&mut cmd_2);
        let stderr2 = String::from_utf8_lossy(&out2.stderr);
        assert!(
            out2.status.success(),
            "an explicit .rkyv index path must load: {stderr2}"
        );

        // The installed index must be BYTE-IDENTICAL to the source. Re-serializing the loaded
        // engine instead of copying the file produced an index 696 bytes shorter that could not
        // be loaded at all, while index-load still reported success - so asserting only on the
        // exit status let a corrupt install pass.
        let installed = cache_dir.join(format!(
            "qsv-{}-geocode-index.rkyv",
            env!("CARGO_PKG_VERSION")
        ));
        assert!(
            installed.exists(),
            "index-load must install the active index"
        );
        // compare CONTENT, not just length: the corruption this pins was a re-serialization, and
        // a same-sized rewrite would sail straight past a length check
        let loaded_bytes = std::fs::read(&decompressed).unwrap();
        let installed_bytes = std::fs::read(&installed).unwrap();
        assert!(
            loaded_bytes == installed_bytes,
            "the installed index must be a byte-exact copy of the one that was loaded (loaded {} \
             bytes, installed {} bytes)",
            loaded_bytes.len(),
            installed_bytes.len()
        );
    }

    // and the end that actually matters: the installed index must WORK
    wrk.create_from_string("city.csv", "city\nParis\n");
    let mut cmd_3 = wrk.command("geocode");
    cmd_3.env("QSV_CACHE_DIR", &cache);
    cmd_3.args([
        "suggest",
        "city",
        "--formatstr",
        "%dyncols: {c:name}",
        "city.csv",
    ]);
    let out3 = wrk.output(&mut cmd_3);
    let stderr3 = String::from_utf8_lossy(&out3.stderr);
    assert!(
        out3.status.success(),
        "the index index-load installed must be usable: {stderr3}"
    );
    assert!(
        String::from_utf8_lossy(&out3.stdout).contains("Paris"),
        "the installed index must resolve a well-known city"
    );
}

#[test]
#[serial]
fn geocode_index_load_invalid_shortcut() {
    // the index-load numeric shortcut only accepts a population floor qsv publishes a prebuilt
    // index for; any other number fails fast with an actionable error - no download
    let wrk = Workdir::new("geocode_index_load_invalid_shortcut");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("index-load").arg("5000");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid index shortcut")
            && stderr.contains("15000")
            && stderr.contains("1000"),
        "the error should name every supported shortcut, got: {stderr}"
    );
}

#[test]
#[serial]
fn geocode_index_load_accepts_timeout() {
    // `--timeout` must parse for index-load (it was rejected as "Invalid arguments"); an invalid
    // shortcut then fails fast, before any download
    let wrk = Workdir::new("geocode_index_load_accepts_timeout");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("index-load")
        .arg("--timeout")
        .arg("300")
        .arg("5000");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("Invalid index shortcut"),
        "--timeout must be accepted by index-load, got: {stderr}"
    );
}

#[test]
#[serial]
fn geocode_index_reset_forwards_timeout() {
    // index-reset must accept --timeout AND pass it to the index download: 0 is rejected by the
    // download's own validation, so reaching that error proves the value got there - no network.
    // The cache is isolated, as index-reset deletes the active index.
    let wrk = Workdir::new("geocode_index_reset_forwards_timeout");
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
        .env_remove("QSV_TIMEOUT");
    cmd.arg("index-reset").arg("--timeout").arg("0");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("Timeout cannot be zero"),
        "--timeout must reach the index download, got: {stderr}"
    );
}

#[test]
#[serial]
fn geocode_index_load_numeric_named_file_not_shortcut() {
    // a real local file whose name happens to be numeric (e.g. `15000.rkyv`) must be
    // loaded as a file, NOT mistaken for the `15000` download shortcut - which would
    // overwrite it. Verify the file content is left untouched.
    let wrk = Workdir::new("geocode_index_load_numeric_named_file_not_shortcut");
    let index_path = wrk.path("15000.rkyv");
    let original = b"this is not a real rkyv index";
    std::fs::write(&index_path, original).unwrap();

    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
    cmd.arg("index-load").arg(&index_path);

    // loading garbage content fails, but crucially the file must not be overwritten
    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read(&index_path).unwrap(),
        original,
        "index-load must not overwrite a real file whose name parses as a number"
    );
}

// ---------------------------------------------------------------------------
// `geoclient` tests use an in-process actix mock of the NYC Geoclient v2 API, selected with
// QSV_GEOCLIENT_API_URL, so they run offline in CI. Response shapes follow the upstream samples
// (mlipper/geoclient documentation/src/docs/asciidoc/samples): each endpoint wraps its object
// under its own name, `/search` under `results[].response`, and `/bbl`/`/bin` return only the
// `*InternalLabel` coordinates. The mock routes on `/{mode}/geoclient/v2/{endpoint}`:
//   ok    - normal behavior;
//   auth  - 401 unless the Ocp-Apim-Subscription-Key header is "SECRET";
//   flaky - 429 (Retry-After: 0) for the first two requests, then normal;
//   down  - always 503.
//   redirect - 302 to the same endpoint under `ok` (must NOT be followed: the key would go along);
//   huge  - a 5 MiB body (must be refused, not buffered).
//   opencage - an OpenCage-shaped success (the OpenCage tests point QSV_OPENCAGE_API_URL here).
//   truncate - the first response dies mid-body (must be retried), then normal.
// A search `input` containing NOMATCH is a Geosupport reject (EE/EE), RC2ONLY succeeds only on
// geosupportReturnCode2, and BADREQ gets an HTTP 400.
// ---------------------------------------------------------------------------
mod geoclient_mock {
    use std::{
        collections::HashMap,
        net::SocketAddr,
        sync::{
            Arc, Mutex,
            atomic::{AtomicUsize, Ordering},
            mpsc,
        },
        thread,
    };

    use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::ServerHandle, rt, web};
    use serde_json::json;

    // Local mock-server bind address (not production code).
    const BIND_HOST: &str = "127.0.0.1"; // DevSkim: ignore DS162092

    #[derive(Clone, Default)]
    struct State {
        requests: Arc<AtomicUsize>,
        queries:  Arc<Mutex<Vec<String>>>,
        keys:     Arc<Mutex<Vec<String>>>,
    }

    async fn serve(
        req: HttpRequest,
        path: web::Path<(String, String)>,
        query: web::Query<HashMap<String, String>>,
        state: web::Data<State>,
    ) -> HttpResponse {
        let (mode, endpoint) = path.into_inner();
        let n = state.requests.fetch_add(1, Ordering::SeqCst) + 1;
        let key = req
            .headers()
            .get("Ocp-Apim-Subscription-Key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let mut pairs: Vec<String> = query.iter().map(|(k, v)| format!("{k}={v}")).collect();
        pairs.sort();
        state
            .queries
            .lock()
            .unwrap()
            .push(format!("{endpoint}?{}", pairs.join("&")));
        state.keys.lock().unwrap().push(key.clone());

        match mode.as_str() {
            "auth" if key != "SECRET" => {
                return HttpResponse::Unauthorized()
                    .json(json!({"statusCode": 401, "message": "Access denied"}));
            },
            "flaky" if n <= 2 => {
                return HttpResponse::TooManyRequests()
                    .insert_header(("Retry-After", "0"))
                    .finish();
            },
            "down" => return HttpResponse::ServiceUnavailable().finish(),
            "redirect" => {
                return HttpResponse::Found()
                    .insert_header(("Location", format!("/ok/geoclient/v2/{endpoint}")))
                    .finish();
            },
            "truncate" if n == 1 => {
                // send the headers and a first chunk, THEN fail: failing at once makes actix drop
                // the connection before the headers, which the client sees as a
                // send error, not a mid-body one
                let body = futures::stream::unfold(0u8, |step| async move {
                    match step {
                        0 => Some((Ok(web::Bytes::from_static(b"{\"results\": [")), 1)),
                        1 => {
                            rt::time::sleep(std::time::Duration::from_millis(200)).await;
                            Some((Err(std::io::Error::other("connection dropped mid-body")), 2))
                        },
                        _ => None,
                    }
                });
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .streaming(body);
            },
            "opencage" => {
                return HttpResponse::Ok().json(json!({
                    "status": {"code": 200, "message": "OK"},
                    "results": [{
                        "formatted": "Mock Place, NY",
                        "geometry": {"lat": 40.5, "lng": -73.9},
                        "confidence": 9,
                    }],
                }));
            },
            "huge" => {
                let padding = "x".repeat(5 * 1024 * 1024);
                return HttpResponse::Ok().json(json!({ "search": { "padding": padding } }));
            },
            _ => {},
        }

        let q = |k: &str| query.get(k).cloned().unwrap_or_default();
        let input = q("input");
        if input.contains("BADREQ") {
            return HttpResponse::BadRequest().json(json!({"title": "Bad Request"}));
        }
        let (rc, rc2) = if input.contains("NOMATCH") {
            ("EE", "EE")
        } else if input.contains("RC2ONLY") {
            ("EE", "00")
        } else {
            ("00", "00")
        };
        let mut obj = json!({
            "geosupportReturnCode": rc,
            "geosupportReturnCode2": rc2,
            "bbl": "1018890001",
            "buildingIdentificationNumber": "1057127",
            "cityCouncilDistrict": "07",
            "censusTract2020": " 187  ",
        });
        let map = obj.as_object_mut().unwrap();
        match endpoint.as_str() {
            "bbl" | "bin" => {
                map.insert("latitudeInternalLabel".into(), json!(40.708659));
                map.insert("longitudeInternalLabel".into(), json!(-74.007118));
                if endpoint == "bbl" {
                    map.insert(
                        "bbl".into(),
                        json!(format!("{}{}{}", q("borough"), q("block"), q("lot"))),
                    );
                } else {
                    map.insert("buildingIdentificationNumber".into(), json!(q("bin")));
                }
            },
            _ => {
                map.insert("latitude".into(), json!(40.798502));
                map.insert("longitude".into(), json!(-73.972709));
                let echo = if endpoint == "search" {
                    input.clone()
                } else {
                    format!(
                        "{} {}, {}{}",
                        q("houseNumber"),
                        q("street"),
                        q("borough"),
                        q("zip")
                    )
                };
                map.insert("echo".into(), json!(echo));
            },
        }
        if endpoint == "search" {
            HttpResponse::Ok().json(json!({
                "status": "OK",
                "input": input,
                "results": [{"level": "0", "status": "EXACT_MATCH", "response": obj}],
            }))
        } else {
            HttpResponse::Ok().json(json!({ endpoint: obj }))
        }
    }

    async fn run_webserver(
        tx: mpsc::Sender<Result<(ServerHandle, SocketAddr), String>>,
        state: State,
    ) -> std::io::Result<()> {
        let server_builder = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/{mode}/geoclient/v2/{endpoint}", web::get().to(serve))
        })
        .workers(2);
        let bound = match server_builder.bind((BIND_HOST, 0)) {
            Ok(b) => b,
            Err(e) => {
                let _ = tx.send(Err(format!("bind failed: {e}")));
                return Err(e);
            },
        };
        let addr = bound.addrs().into_iter().next().unwrap();
        let server = bound.run();
        let _ = tx.send(Ok((server.handle(), addr)));
        server.await
    }

    pub struct GeoclientMock {
        handle: Option<ServerHandle>,
        addr:   SocketAddr,
        state:  State,
    }

    impl GeoclientMock {
        pub fn start() -> Self {
            let state = State::default();
            let server_state = state.clone();
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || rt::System::new().block_on(run_webserver(tx, server_state)));
            match rx.recv_timeout(std::time::Duration::from_secs(10)) {
                Ok(Ok((handle, addr))) => Self {
                    handle: Some(handle),
                    addr,
                    state,
                },
                Ok(Err(msg)) => panic!("geoclient mock failed to bind: {msg}"),
                Err(e) => panic!("geoclient mock did not start within 10s ({e:?})"),
            }
        }

        /// The base URL for `QSV_GEOCLIENT_API_URL` in the given mode.
        pub fn url(&self, mode: &str) -> String {
            format!("http://{}/{mode}/geoclient/v2", self.addr) // DevSkim: ignore DS137138
        }

        pub fn requests(&self) -> usize {
            self.state.requests.load(Ordering::SeqCst)
        }

        pub fn queries(&self) -> Vec<String> {
            self.state.queries.lock().unwrap().clone()
        }

        pub fn keys(&self) -> Vec<String> {
            self.state.keys.lock().unwrap().clone()
        }
    }

    impl Drop for GeoclientMock {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                rt::System::new().block_on(handle.stop(true));
            }
        }
    }
}

use geoclient_mock::GeoclientMock;

/// A `geocode geoclient` command pointed at `url`, with an isolated cache dir and fast retries.
fn geoclient_cmd(wrk: &Workdir, url: &str) -> std::process::Command {
    let mut cmd = wrk.command("geocode");
    cmd.env("QSV_GEOCLIENT_API_URL", url)
        .env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
        .env("QSV_GEOCLIENT_RETRY_BASE_MS", "1")
        .env_remove("QSV_GEOCLIENT_API_KEY");
    cmd
}

#[test]
#[serial]
fn geocode_geoclient_search() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_search");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["314 W 100 St Manhattan"],
            svec![""],
            svec!["NOMATCH somewhere"],
            svec!["RC2ONLY 280 Riverside Dr"],
            // same location, different case and spacing: one request, from the dedupe
            svec!["314 w 100 st   manhattan"],
        ],
    );
    let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .args(["--api-key", "SECRET"])
        .args(["--invalid-result", "<NONE>"])
        .arg("--no-cache");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["40.798502, -73.972709"],
        svec![""],
        svec!["<NONE>"],
        svec!["40.798502, -73.972709"],
        svec!["40.798502, -73.972709"],
    ];
    assert_eq!(got, expected);
    assert_eq!(mock.requests(), 3, "queries: {:?}", mock.queries());
    // the key travels in the header, never in the URL
    assert!(
        mock.keys().iter().all(|k| k == "SECRET"),
        "{:?}",
        mock.keys()
    );
    assert!(mock.queries().iter().all(|q| !q.contains("SECRET")));
    assert!(
        mock.queries()
            .contains(&"search?input=314 W 100 St Manhattan".to_string()),
        "{:?}",
        mock.queries()
    );
}

#[test]
#[serial]
fn geocode_geoclient_bbl_dyncols() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_bbl_dyncols");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "bbl"],
            svec!["a", "1000670001"],
            svec!["b", "1-67-1"],
            svec!["c", "not a bbl"],
            svec!["d", "6000670001"],
            // 10 bytes, but not 10 digits: must not panic slicing through the multi-byte char
            svec!["e", "é12345678"],
        ],
    );
    let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
    cmd.arg("geoclient")
        .arg("bbl")
        .arg("data.csv")
        .args(["--endpoint", "bbl"])
        .args([
            "-f",
            "%dyncols: {lat:latitude}, {lng:longitude}, {gbbl:bbl}, {tract:censusTract2020}",
        ])
        .arg("--no-cache");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let hit = |id: &'static str, bbl: &'static str| {
        svec![id, bbl, "40.708659", "-74.007118", "1000670001", "187"]
    };
    let expected = vec![
        svec!["id", "bbl", "lat", "lng", "gbbl", "tract"],
        hit("a", "1000670001"),
        // the delimited form is normalized to the same request
        hit("b", "1-67-1"),
        svec!["c", "not a bbl", "", "", "", ""],
        svec!["d", "6000670001", "", "", "", ""],
        svec!["e", "é12345678", "", "", "", ""],
    ];
    assert_eq!(got, expected);
    assert_eq!(
        mock.queries(),
        vec!["bbl?block=00067&borough=1&lot=0001".to_string()]
    );
}

#[test]
#[serial]
fn geocode_geoclient_address_params() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_address_params");
    wrk.create(
        "data.csv",
        vec![
            svec!["HouseNo", "Street", "Boro"],
            svec!["314", "West 100 Street", "Manhattan"],
            // a missing required value: no request, --invalid-result
            svec!["120", "", "Brooklyn"],
        ],
    );
    // no key: a self-hosted Geoclient (any non-default URL) needs none
    let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
    cmd.arg("geoclient")
        .arg("Street")
        .arg("data.csv")
        .args(["--endpoint", "address"])
        .args([
            "--params",
            "houseNumber={HouseNo},street={Street},borough={Boro}",
        ])
        .args(["-c", "geo"])
        .args(["-f", "{echo} -> {bbl}"])
        .args(["--invalid-result", "<NONE>"])
        .arg("--no-cache");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["HouseNo", "Street", "Boro", "geo"],
        svec![
            "314",
            "West 100 Street",
            "Manhattan",
            "314 West 100 Street, Manhattan -> 1018890001"
        ],
        svec!["120", "", "Brooklyn", "<NONE>"],
    ];
    assert_eq!(got, expected);
    assert_eq!(mock.requests(), 1);
    assert_eq!(mock.keys(), vec![String::new()]);
}

#[test]
#[serial]
fn geocode_geoclient_bad_key_is_fatal() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_bad_key_is_fatal");
    wrk.create(
        "data.csv",
        vec![svec!["Location"], svec!["a"], svec!["b"], svec!["c"]],
    );
    let mut cmd = geoclient_cmd(&wrk, &mock.url("auth"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .args(["--api-key", "WRONG"])
        .args(["--jobs", "1"])
        .arg("--no-cache");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("HTTP 401"), "stderr: {stderr}");
    // the gateway's own reason is passed through
    assert!(stderr.contains("Access denied"), "stderr: {stderr}");
    assert!(stderr.contains("api-portal.nyc.gov"), "stderr: {stderr}");
    // aborted on the first response instead of failing every row
    assert_eq!(mock.requests(), 1);
}

#[test]
#[serial]
fn geocode_geoclient_retries_429() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_retries_429");
    wrk.create(
        "data.csv",
        vec![svec!["Location"], svec!["Empire State Building"]],
    );
    let mut cmd = geoclient_cmd(&wrk, &mock.url("flaky"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .arg("--no-cache");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, vec![svec!["Location"], svec!["40.798502, -73.972709"]]);
    // two 429s, then the success
    assert_eq!(mock.requests(), 3);
}

#[test]
#[serial]
fn geocode_geoclient_circuit_breaker() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_circuit_breaker");
    let mut rows = vec![svec!["Location"]];
    for i in 0..15 {
        rows.push(vec![format!("{i} Broadway Manhattan")]);
    }
    wrk.create("data.csv", rows);
    let mut cmd = geoclient_cmd(&wrk, &mock.url("down"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .args(["--jobs", "1"])
        .arg("--no-cache");

    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("10 Geoclient requests in a row failed"),
        "stderr: {stderr}"
    );
    // 10 locations x (1 try + 3 retries), and nothing after the breaker trips
    assert_eq!(mock.requests(), 40);
}

#[test]
#[serial]
fn geocode_geoclient_bad_request_reported() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_bad_request_reported");
    wrk.create(
        "data.csv",
        vec![svec!["Location"], svec!["BADREQ x"], svec!["Wall St"]],
    );
    let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .arg("--no-cache");

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.replace("\r\n", "\n"),
        "Location\nBADREQ x\n\"40.798502, -73.972709\"\n"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("1 Geoclient lookup(s) were rejected"),
        "stderr: {stderr}"
    );
}

#[test]
#[serial]
fn geocode_geoclient_disk_cache() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_disk_cache");
    wrk.create(
        "data.csv",
        vec![svec!["Location"], svec!["Wall St"], svec!["NOMATCH x"]],
    );
    let run = |format: &str| {
        let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
        cmd.arg("geoclient")
            .arg("Location")
            .arg("data.csv")
            .args(["-f", format]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        got
    };
    let first = run("%+");
    assert_eq!(first[1], svec!["40.798502, -73.972709"]);
    assert_eq!(mock.requests(), 2);
    // a different --formatstr is served from the cache (it stores the raw result), and the
    // cached non-match stays a non-match
    let second = run("%bbl");
    assert_eq!(
        second,
        vec![svec!["Location"], svec!["1018890001"], svec!["NOMATCH x"]]
    );
    assert_eq!(mock.requests(), 2);

    let mut info = wrk.command("geocode");
    info.env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
        .arg("cache-info");
    let output = wrk.output(&mut info);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(r#""provider":"Geoclient""#) && stdout.contains(r#""exists":true"#),
        "stdout: {stdout}"
    );
}

#[test]
#[serial]
fn geocode_geoclient_order_preserved_concurrently() {
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_order_preserved_concurrently");
    let mut rows = vec![svec!["Location"]];
    for i in 0..40 {
        rows.push(vec![format!("{i} Broadway Manhattan")]);
    }
    wrk.create("data.csv", rows.clone());
    let mut cmd = geoclient_cmd(&wrk, &mock.url("ok"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .args(["--jobs", "4"])
        .args(["--batch", "7"])
        .args(["-f", "%dyncols: {echo:echo}"])
        .arg("--no-cache");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected: Vec<Vec<String>> = std::iter::once(svec!["Location", "echo"])
        .chain(
            rows.iter()
                .skip(1)
                .map(|r| vec![r[0].clone(), r[0].clone()]),
        )
        .collect();
    assert_eq!(got, expected);
    assert_eq!(mock.requests(), 40);
}

#[test]
#[serial]
fn geocode_geoclient_arg_validation() {
    let wrk = Workdir::new("geocode_geoclient_arg_validation");
    wrk.create("data.csv", vec![svec!["Location"], svec!["Wall St"]]);
    // a dead URL: every case below must fail before any request is made
    let dead = "http://127.0.0.1:9/geoclient/v2";
    let fails_with = |args: &[&str], url: Option<&str>, needle: &str| {
        let mut cmd = wrk.command("geocode");
        cmd.args(args)
            .env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string())
            .env_remove("QSV_GEOCLIENT_API_KEY")
            .env_remove("QSV_OPENCAGE_API_KEY");
        match url {
            Some(url) => cmd.env("QSV_GEOCLIENT_API_URL", url),
            None => cmd.env_remove("QSV_GEOCLIENT_API_URL"),
        };
        let output = wrk.output(&mut cmd);
        assert!(!output.status.success(), "{args:?} should fail");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(needle),
            "{args:?}: expected {needle:?} in stderr: {stderr}"
        );
    };

    // the public API needs a key
    fails_with(
        &["geoclient", "Location", "data.csv"],
        None,
        "requires a subscription key",
    );
    fails_with(
        &["geoclient", "Location", "data.csv", "--endpoint", "zip"],
        Some(dead),
        "Invalid --endpoint",
    );
    fails_with(
        &["geoclient", "Location", "data.csv", "--endpoint", "address"],
        Some(dead),
        "requires --params",
    );
    fails_with(
        &[
            "geoclient",
            "Location",
            "data.csv",
            "--endpoint",
            "place",
            "--params",
            "name={Location}",
        ],
        Some(dead),
        "requires the borough or zip parameter",
    );
    fails_with(
        &[
            "geoclient",
            "Location",
            "data.csv",
            "--endpoint",
            "place",
            "--params",
            "Name={Location},zip=10005",
        ],
        Some(dead),
        "Invalid --params name",
    );
    fails_with(
        &[
            "geoclient",
            "Location",
            "data.csv",
            "--params",
            "returnPolicy={Location}",
        ],
        Some(dead),
        "literal options",
    );
    fails_with(
        &[
            "geoclient",
            "Location",
            "data.csv",
            "--endpoint",
            "place",
            "--params",
            "name={Nope},zip=10005",
        ],
        Some(dead),
        "not in the input headers",
    );
    fails_with(
        &["geoclient", "Location", "data.csv", "--reverse"],
        Some(dead),
        "--reverse is not supported by the geoclient",
    );
    fails_with(
        &["geoclient", "Location", "data.csv", "--country", "US"],
        Some(dead),
        "--country is not supported by the geoclient",
    );
    fails_with(
        &["geoclient", "Location", "data.csv", "--rate-limit", "0"],
        Some(dead),
        "--rate-limit must be >= 1",
    );
    fails_with(
        &["geoclientnow", "--endpoint", "address", "Wall St"],
        Some(dead),
        "search, bin and bbl endpoints only",
    );
    fails_with(
        &[
            "opencage",
            "Location",
            "data.csv",
            "--api-key",
            "dummy",
            "--endpoint",
            "bbl",
        ],
        None,
        "only valid with the geoclient subcommands",
    );
    // an empty column selection is a usage error, not a panic
    fails_with(
        &["geoclient", "!Location", "data.csv"],
        Some(dead),
        "does not match any column",
    );
    fails_with(
        &["opencage", "!Location", "data.csv", "--api-key", "dummy"],
        None,
        "does not match any column",
    );
}

#[test]
#[serial]
#[ignore = "requires a NYC Geoclient subscription key (set QSV_GEOCLIENT_API_KEY)"]
fn geocode_geoclient_live() {
    let wrk = Workdir::new("geocode_geoclient_live");
    wrk.create(
        "data.csv",
        vec![svec!["Location"], svec!["350 5th Ave Manhattan"]],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .args(["-f", "%bbl"])
        .arg("--no-cache")
        .env_remove("QSV_GEOCLIENT_API_URL");
    // on_success: a rejected key must fail with qsv's message, not an index panic
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // Shape-only assertions: the exact values come from live city data and were not pinned when
    // this was written. A Manhattan BBL is 10 digits starting with 1.
    let bbl = &got[1][0];
    assert!(
        bbl.len() == 10 && bbl.starts_with('1') && bbl.bytes().all(|b| b.is_ascii_digit()),
        "{got:?}"
    );

    let mut cmd = wrk.command("geocode");
    cmd.arg("geoclientnow")
        .args(["--endpoint", "bbl", "-f", "%bin"])
        .arg(bbl)
        .arg("--no-cache")
        .env_remove("QSV_GEOCLIENT_API_URL");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // a Manhattan BIN is 7 digits starting with 1
    let bin = &got[1][0];
    assert!(
        bin.len() == 7 && bin.starts_with('1') && bin.bytes().all(|b| b.is_ascii_digit()),
        "{got:?}"
    );
}

#[test]
#[serial]
fn geocode_geoclient_no_redirects_no_huge_bodies() {
    // reqwest keeps custom headers on a cross-host redirect, so following one would hand the
    // subscription key to another host: redirects must not be followed. And a response far beyond
    // Geoclient's few KB must be refused rather than buffered.
    let wrk = Workdir::new("geocode_geoclient_no_redirects_no_huge_bodies");
    wrk.create("data.csv", vec![svec!["Location"], svec!["Wall St"]]);
    for mode in ["redirect", "huge"] {
        let mock = GeoclientMock::start();
        let mut cmd = geoclient_cmd(&wrk, &mock.url(mode));
        cmd.arg("geoclient")
            .arg("Location")
            .arg("data.csv")
            .args(["--api-key", "SECRET"])
            .arg("--no-cache");
        let output = wrk.output(&mut cmd);
        assert!(output.status.success(), "{mode}");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(
            stdout.replace("\r\n", "\n"),
            "Location\nWall St\n",
            "{mode}"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("1 Geoclient lookup(s) failed"),
            "{mode}: {stderr}"
        );
        // the redirect target (the `ok` route) was never requested
        assert_eq!(mock.requests(), 1, "{mode}: {:?}", mock.queries());
    }
}

#[test]
#[serial]
fn geocode_geoclient_retries_truncated_body() {
    // a connection that drops in the middle of a 200's body is retried, not given up on
    let mock = GeoclientMock::start();
    let wrk = Workdir::new("geocode_geoclient_retries_truncated_body");
    wrk.create("data.csv", vec![svec!["Location"], svec!["Wall St"]]);
    let mut cmd = geoclient_cmd(&wrk, &mock.url("truncate"));
    cmd.arg("geoclient")
        .arg("Location")
        .arg("data.csv")
        .arg("--no-cache");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, vec![svec!["Location"], svec!["40.798502, -73.972709"]]);
    assert_eq!(mock.requests(), 2);
}

#[test]
#[serial]
fn geocode_opencage_caps_response_size() {
    // OpenCage responses are a few KB; a 5 MiB body (or a compression bomb) must be refused
    // rather than buffered in full. The `opencage` mode is the positive control: it proves
    // QSV_OPENCAGE_API_URL really points the request at the mock.
    let wrk = Workdir::new("geocode_opencage_caps_response_size");
    wrk.create("data.csv", vec![svec!["Location"], svec!["Brooklyn, NY"]]);
    for (mode, expected_stdout, failed) in [
        ("opencage", "Location\n\"Mock Place, NY\"\n", false),
        ("huge", "Location\n\"Brooklyn, NY\"\n", true),
    ] {
        let mock = GeoclientMock::start();
        let mut cmd = wrk.command("geocode");
        cmd.arg("opencage")
            .arg("Location")
            .arg("data.csv")
            .args(["--api-key", "dummy-key"])
            .arg("--no-cache")
            .env("QSV_OPENCAGE_API_URL", format!("{}/search", mock.url(mode)))
            .env("QSV_CACHE_DIR", wrk.path("").to_string_lossy().to_string());
        let output = wrk.output(&mut cmd);
        assert!(output.status.success(), "{mode}");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.replace("\r\n", "\n"), expected_stdout, "{mode}");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            stderr.contains("1 OpenCage lookup(s) failed"),
            failed,
            "{mode}: {stderr}"
        );
        assert_eq!(mock.requests(), 1, "{mode}");
    }
}
