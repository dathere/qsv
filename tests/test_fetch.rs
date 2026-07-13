use serial_test::serial;

use crate::workdir::Workdir;

#[test]
fn fetch_simple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["  https://api.zippopotam.us/us/90210      "],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802      "],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--rate-limit")
        .arg("2");

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"country":"United States","country abbreviation":"US","post code":"90210","places":[{"place name":"Beverly Hills","longitude":"-118.4065","latitude":"34.0901","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"94105","places":[{"place name":"San Francisco","longitude":"-122.3892","latitude":"37.7864","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"92802","places":[{"place name":"Anaheim","longitude":"-117.9228","latitude":"33.8085","state":"California","state abbreviation":"CA"}]}"#;
    assert_eq!(got, expected);
}

#[test]
#[ignore = "Temporarily skip this as zippotam.us API is in flux"]
fn fetch_simple_pretty_json() {
    let wrk = Workdir::new("fetch_simple_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["  https://api.zippopotam.us/us/90210      "],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802      "],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--rate-limit")
        .arg("2")
        .arg("--pretty")
        .args(["--new-column", "pretty_response"]);

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"URL,pretty_response
https://api.zippopotam.us/us/99999,
https://api.zippopotam.us/us/90210,"{
  ""country"": ""United States"",
  ""country abbreviation"": ""US"",
  ""post code"": ""90210"",
  ""places"": [
    {
      ""place name"": ""Beverly Hills"",
      ""longitude"": ""-118.4065"",
      ""latitude"": ""34.0901"",
      ""state"": ""California"",
      ""state abbreviation"": ""CA""
    }
  ]
}"
https://api.zippopotam.us/us/94105,"{
  ""country"": ""United States"",
  ""country abbreviation"": ""US"",
  ""post code"": ""94105"",
  ""places"": [
    {
      ""place name"": ""San Francisco"",
      ""longitude"": ""-122.3892"",
      ""latitude"": ""37.7864"",
      ""state"": ""California"",
      ""state abbreviation"": ""CA""
    }
  ]
}"
https://api.zippopotam.us/us/92802,"{
  ""country"": ""United States"",
  ""country abbreviation"": ""US"",
  ""post code"": ""92802"",
  ""places"": [
    {
      ""place name"": ""Anaheim"",
      ""longitude"": ""-117.9228"",
      ""latitude"": ""33.8085"",
      ""state"": ""California"",
      ""state abbreviation"": ""CA""
    }
  ]
}""#;
    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_new_col() {
    let wrk = Workdir::new("fetch_simple_new_col");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col2", "col3"],
            svec!["https://api.zippopotam.us/us/99999", "a", "1"],
            svec!["  https://api.zippopotam.us/us/90210      ", "b", "2"],
            svec!["https://api.zippopotam.us/us/94105", "c", "3"],
            svec!["https://api.zippopotam.us/us/92802      ", "d", "4"],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json", "Scott Adams", "42"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("response")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--rate-limit")
        .arg("2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["URL", "col2", "col3", "response"],
        svec!["https://api.zippopotam.us/us/99999", "a", "1", ""],
        svec![
            "https://api.zippopotam.us/us/90210",
            "b",
            "2",
            "{\"country\":\"United States\",\"country abbreviation\":\"US\",\"post \
             code\":\"90210\",\"places\":[{\"place name\":\"Beverly \
             Hills\",\"longitude\":\"-118.4065\",\"latitude\":\"34.0901\",\"state\":\"California\"\
             ,\"state abbreviation\":\"CA\"}]}"
        ],
        svec![
            "https://api.zippopotam.us/us/94105",
            "c",
            "3",
            "{\"country\":\"United States\",\"country abbreviation\":\"US\",\"post \
             code\":\"94105\",\"places\":[{\"place name\":\"San \
             Francisco\",\"longitude\":\"-122.3892\",\"latitude\":\"37.7864\",\"state\":\"\
             California\",\"state abbreviation\":\"CA\"}]}"
        ],
        svec![
            "https://api.zippopotam.us/us/92802",
            "d",
            "4",
            "{\"country\":\"United States\",\"country abbreviation\":\"US\",\"post \
             code\":\"92802\",\"places\":[{\"place \
             name\":\"Anaheim\",\"longitude\":\"-117.9228\",\"latitude\":\"33.8085\",\"state\":\"\
             California\",\"state abbreviation\":\"CA\"}]}"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_report() {
    let wrk = Workdir::new("fetch_simple_report");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/07094"],
            svec!["  https://api.zippopotam.us/us/90210      "],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802      "],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv").arg("--report").arg("short");

    let mut cmd = wrk.command("index");
    cmd.arg("data.csv.fetch-report.tsv");

    let mut cmd = wrk.command("select");
    cmd.arg("url,status,cache_hit,retries,response")
        .arg(wrk.load_test_file("data.csv.fetch-report.tsv"));

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["url", "status", "cache_hit", "retries", "response"],
        svec![
            "https://api.zippopotam.us/us/07094",
            "200",
            "0",
            "5",
            r#"{"post code":"07094","country":"United States","country abbreviation":"US","places":[{"place name":"Secaucus","longitude":"-74.0634","state":"New Jersey","state abbreviation":"NJ","latitude":"40.791"}]}"#
        ],
        svec![
            "https://api.zippopotam.us/us/90210",
            "200",
            "0",
            "0",
            r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#
        ],
        svec![
            "https://api.zippopotam.us/us/94105",
            "200",
            "0",
            "0",
            r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#
        ],
        svec![
            "https://api.zippopotam.us/us/92802",
            "200",
            "0",
            "0",
            r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#
        ],
        // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json", "200", "0", "0", r#"{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_url_template() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["zip code"],
            svec!["99999"],
            svec!["  90210   "],
            svec!["94105  "],
            svec!["92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("--url-template")
        .arg("https://api.zippopotam.us/us/{zip_code}")
        .arg("--store-error")
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"country":"United States","country abbreviation":"US","post code":"90210","places":[{"place name":"Beverly Hills","longitude":"-118.4065","latitude":"34.0901","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"94105","places":[{"place name":"San Francisco","longitude":"-122.3892","latitude":"37.7864","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"92802","places":[{"place name":"Anaheim","longitude":"-117.9228","latitude":"33.8085","state":"California","state abbreviation":"CA"}]}"#;
    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_redis() {
    // if there is no local redis server, skip fetch_simple_redis test
    let redis_client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    if redis_client.get_connection().is_err() {
        return;
    }

    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["  https://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
            svec!["thisisnotaurl"],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--redis-cache")
        .arg("--rate-limit")
        .arg("2");

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"country":"United States","country abbreviation":"US","post code":"90210","places":[{"place name":"Beverly Hills","longitude":"-118.4065","latitude":"34.0901","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"94105","places":[{"place name":"San Francisco","longitude":"-122.3892","latitude":"37.7864","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"92802","places":[{"place name":"Anaheim","longitude":"-117.9228","latitude":"33.8085","state":"California","state abbreviation":"CA"}]}
{"errors":[{"title":"Invalid URL","detail":"relative URL without a base"}]}"#;

    assert_eq!(got, expected);
}

#[test]
fn fetch_simple_diskcache() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/99999"],
            svec!["https://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
            svec!["thisisnotaurl"],
            // svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );

    // use an isolated, per-test disk-cache directory inside the workdir (which is
    // uniquely named per test and auto-removed on drop). Being fresh each run, the
    // redb-file assertion below genuinely verifies THIS run created the cache -
    // without deleting any shared/global temp path.
    use std::fs;
    let temp_dir = wrk.path("dcache");
    fs::create_dir_all(&temp_dir).unwrap();
    let dc_dir = temp_dir.as_os_str().to_str().unwrap();

    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--disk-cache")
        .args(["--disk-cache-dir", dc_dir])
        .args(["--rate-limit", "2"]);

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"country":"United States","country abbreviation":"US","post code":"90210","places":[{"place name":"Beverly Hills","longitude":"-118.4065","latitude":"34.0901","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"94105","places":[{"place name":"San Francisco","longitude":"-122.3892","latitude":"37.7864","state":"California","state abbreviation":"CA"}]}
{"country":"United States","country abbreviation":"US","post code":"92802","places":[{"place name":"Anaheim","longitude":"-117.9228","latitude":"33.8085","state":"California","state abbreviation":"CA"}]}
{"errors":[{"title":"Invalid URL","detail":"relative URL without a base"}]}"#;

    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);

    // cached v3 uses redb (a single file), not sled (a directory): the on-disk
    // cache is `{name}_v{DISK_FILE_VERSION}.redb`, not `{name}_v1/conf`.
    assert!(temp_dir.join("fetch_v3.redb").exists());

    let mut cmd_2 = wrk.command("fetch");
    cmd_2
        .arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--disk-cache")
        .args(["--disk-cache-dir", dc_dir])
        .args(["--report", "short"]);

    let got = wrk.stdout::<String>(&mut cmd_2);
    assert_eq!(got, expected);

    // sleep for a bit to make sure the cache is written to disk
    std::thread::sleep(std::time::Duration::from_secs(2));

    let fetchreport = wrk.read_to_string("data.csv.fetch-report.tsv").unwrap();
    wrk.create_from_string("no-elapsed.tsv", &fetchreport);

    // remove the elapsed_ms column from the report as this is not deterministic
    let mut cmd3 = wrk.command("select");
    cmd3.arg("!elapsed_ms").arg("no-elapsed.tsv");

    let fetchreport_noelapsed = wrk.stdout::<String>(&mut cmd3);
    // read the output file and compare it with the expected output
    // Note: error rows (404 and invalid URL) show cache_hit=0 because non-200
    // responses are evicted from the disk cache when --cache-error is not set.
    // Successful (200) rows are cached and show cache_hit=1.
    assert_eq!(
        fetchreport_noelapsed,
        r#"url,status,cache_hit,retries,response
https://api.zippopotam.us/us/99999,404,0,5,"{""errors"":[{""title"":""HTTP ERROR"",""detail"":""HTTP ERROR 404 - Not Found""}]}"
https://api.zippopotam.us/us/90210,200,1,0,"{""country"":""United States"",""country abbreviation"":""US"",""post code"":""90210"",""places"":[{""place name"":""Beverly Hills"",""longitude"":""-118.4065"",""latitude"":""34.0901"",""state"":""California"",""state abbreviation"":""CA""}]}"
https://api.zippopotam.us/us/94105,200,1,0,"{""country"":""United States"",""country abbreviation"":""US"",""post code"":""94105"",""places"":[{""place name"":""San Francisco"",""longitude"":""-122.3892"",""latitude"":""37.7864"",""state"":""California"",""state abbreviation"":""CA""}]}"
https://api.zippopotam.us/us/92802,200,1,0,"{""country"":""United States"",""country abbreviation"":""US"",""post code"":""92802"",""places"":[{""place name"":""Anaheim"",""longitude"":""-117.9228"",""latitude"":""33.8085"",""state"":""California"",""state abbreviation"":""CA""}]}"
thisisnotaurl,404,0,0,"{""errors"":[{""title"":""Invalid URL"",""detail"":""relative URL without a base""}]}""#
    );
}

#[test]
fn fetch_jaq_single() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["thisisnotaurl"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jaq")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["https://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["https://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["thisisnotaurl", ""],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn fetch_jaq_single_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["https://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jaqfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jaq_single.jaq"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["https://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["https://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jaqfile_doesnotexist_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"], // DevSkim: ignore DS137138
            svec!["http://api.zippopotam.us/us/94105"], // DevSkim: ignore DS137138
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jaqfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/doesnotexist.jaq"
        ))
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetch_jaq_jaqfile_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["https://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--jaqfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jaq_single.jaq"
        ))
        .arg("--jaq")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("Invalid arguments."));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetch_jaq_multiple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"], // DevSkim: ignore DS137138
            svec!["http://api.zippopotam.us/us/94105"], // DevSkim: ignore DS137138
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jaq")
        .arg(r#"[ ."places"[0]."place name", ."places"[0]."state abbreviation" ]"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec![
            "http://api.zippopotam.us/us/90210", // DevSkim: ignore DS137138
            "[\"Beverly Hills\",\"CA\"]"
        ],
        svec![
            "http://api.zippopotam.us/us/94105", // DevSkim: ignore DS137138
            "[\"San Francisco\",\"CA\"]"
        ],
        svec!["https://api.zippopotam.us/us/92802", "[\"Anaheim\",\"CA\"]"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jaq_multiple_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"], // DevSkim: ignore DS137138
            svec!["http://api.zippopotam.us/us/94105"], // DevSkim: ignore DS137138
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jaqfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jaq_multiple.jaq"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec![
            "http://api.zippopotam.us/us/90210", // DevSkim: ignore DS137138
            "[\"Beverly Hills\",\"CA\"]"
        ],
        svec![
            "http://api.zippopotam.us/us/94105", // DevSkim: ignore DS137138
            "[\"San Francisco\",\"CA\"]"
        ],
        svec!["https://api.zippopotam.us/us/92802", "[\"Anaheim\",\"CA\"]"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetch_custom_header() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("-H")
        .arg(" X-Api-Key :  DEMO_KEY")
        .arg("-H")
        .arg("X-Api-Secret :ABC123XYZ")
        .arg("--jaq")
        .arg(r#"[ ."headers"."X-Api-Key", ."headers"."X-Api-Secret" ]"#)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.stdout::<String>(&mut cmd);
    let expected = "[\"DEMO_KEY\",\"ABC123XYZ\"]";
    assert_eq!(got, expected);
}

#[test]
fn fetch_custom_invalid_header_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg("X-Api-\tSecret :ABC123XYZ") // embedded tab is not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid header name"));

    wrk.assert_err(&mut cmd);
}
#[test]
fn fetch_custom_invalid_user_agent_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--user-agent")
        // ð, è and \n are invalid characters for header values
        .arg("Mðzilla/5.0\n (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxvèrsion")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid user-agent"));

    wrk.assert_err(&mut cmd);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetch_custom_user_agent() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--user-agent")
        .arg("Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.stdout::<String>(&mut cmd);
    assert!(got.contains(
        "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion"
    ));
    wrk.assert_success(&mut cmd);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetch_user_agent() {
    let wrk = Workdir::new("fetch_user_agent");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    // the default user agent should contain the name of the qsv command used,
    // in this case "fetch"
    assert!(got.contains("; fetch; "));
    wrk.assert_success(&mut cmd);
}

#[test]
fn fetch_custom_invalid_value_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]], // DevSkim: ignore DS137138
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg("X-Api-Secret :ABC123\r\nXYZ") // non-visible ascii not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid header value"));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetchpost_custom_invalid_header_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("-H")
        .arg("X-Api-\tSecret :ABC123XYZ") // non-visible ascii not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid header name"));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetchpost_custom_invalid_value_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("--http-header")
        .arg("X-Api-Secret :ABC123\r\nXYZ") // non-visible ascii not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid header value"));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetchpost_custom_invalid_user_agent_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("1,2")
        .arg("--user-agent")
        // ð, è and \n are invalid characters for header values
        .arg("Mðzilla/5.0\n (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxvèrsion")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("usage error: Invalid user-agent"));

    wrk.assert_err(&mut cmd);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_custom_user_agent() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("1,2")
        .arg("--user-agent")
        .arg("Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.stdout::<String>(&mut cmd);
    assert!(got.contains(
        "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion"
    ));
    wrk.assert_success(&mut cmd);
}

use std::{net::SocketAddr, sync::mpsc, thread};

use actix_web::{
    App, HttpRequest, HttpServer, Responder, Result, dev::ServerHandle, middleware, rt, web,
};
use serde::Serialize;
#[derive(Serialize)]
struct MyObj {
    fullname: String,
}

async fn index() -> impl Responder {
    "Hello world!"
}

/// handler with path parameters like `/user/{name}/`
/// returns Smurf fullname in JSON format
async fn get_fullname(req: HttpRequest, name: web::Path<String>) -> Result<impl Responder> {
    println!("{req:?}");

    let obj = MyObj {
        fullname: format!("{name} Smurf"),
    };

    Ok(web::Json(obj))
}

// Bind to 127.0.0.1 with an OS-assigned ephemeral port. Hardcoded ports
// (this suite previously used 8081) collide on macOS CI runners with peer
// integration-test binaries / lingering TIME_WAIT sockets and produce flaky
// "Address already in use" failures. Each test reads the actual SocketAddr
// from the channel and builds URLs against it via a local closure.
const FETCH_TEST_BIND_HOST: &str = "127.0.0.1";

/// start an Actix Webserver with Rate Limiting via Governor.
/// Sends `Ok((handle, addr))` on success or `Err(msg)` on bind failure so
/// tests fail fast with a clear error instead of timing out on `recv`.
async fn run_webserver(
    tx: mpsc::Sender<std::result::Result<(ServerHandle, SocketAddr), String>>,
) -> std::io::Result<()> {
    use actix_governor::{Governor, GovernorConfigBuilder};

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every 250 ms (4 qps)
    let governor_conf = GovernorConfigBuilder::default()
        .milliseconds_per_request(250)
        .burst_size(7)
        .finish()
        .unwrap();

    // server is server controller type, `dev::ServerHandle`
    let server_builder = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(Governor::new(&governor_conf))
            .service(web::resource("/user/{name}").route(web::get().to(get_fullname)))
            .service(web::resource("/").to(index))
    });

    // Port 0 -> OS picks an unused ephemeral port; addrs() then reports it.
    let bound = match server_builder.bind((FETCH_TEST_BIND_HOST, 0)) {
        Ok(b) => b,
        Err(e) => {
            let _ = tx.send(Err(format!("bind failed: {e}")));
            return Err(e);
        },
    };

    let addr = match bound.addrs().into_iter().next() {
        Some(a) => a,
        None => {
            let _ = tx.send(Err("bind succeeded but no address was reported".to_string()));
            return Err(std::io::Error::other(
                "actix HttpServer::addrs() returned empty",
            ));
        },
    };

    let server = bound.run();

    // send server controller + actual bound addr to main thread
    let _ = tx.send(Ok((server.handle(), addr)));

    // run future
    server.await
}

/// Helper for `fetch_ratelimit` / `fetch_complex_url_template`: spawn the
/// webserver thread, wait up to 10s for the bind to either succeed (returning
/// `(handle, addr)`) or fail, panicking with a clear message otherwise.
fn start_fetch_webserver() -> (ServerHandle, SocketAddr) {
    let (tx, rx) = mpsc::channel();
    println!("START Webserver ");
    thread::spawn(move || {
        let server_future = run_webserver(tx);
        rt::System::new().block_on(server_future)
    });
    match rx.recv_timeout(std::time::Duration::from_secs(10)) {
        Ok(Ok(payload)) => payload,
        Ok(Err(msg)) => panic!("test webserver failed to bind: {msg}"),
        Err(e) => panic!("test webserver did not start within 10s ({e:?})"),
    }
}

#[test]
#[serial]
fn fetch_ratelimit() {
    // start webserver with rate limiting (OS-assigned ephemeral port)
    let (server_handle, addr) = start_fetch_webserver();
    // svec! only accepts &'static str, so we need a String-based row helper
    // for any row that contains a runtime-built URL.
    let url_row = |path: &str| vec![format!("http://{addr}/{path}")];
    let url_pair = |path: &str, name: &str| vec![format!("http://{addr}/{path}"), name.to_string()];

    // proceed with usual unit test
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            url_row("user/Smurfette"),
            url_row("user/Papa"),
            url_row("user/Clumsy"),
            url_row("user/Brainy"),
            url_row("user/Grouchy"),
            url_row("user/Hefty"),
            url_row("user/Greedy"),
            url_row("user/Jokey"),
            url_row("user/Chef"),
            url_row("user/Vanity"),
            url_row("user/Handy"),
            url_row("user/Scaredy"),
            url_row("user/Tracker"),
            url_row("user/Sloppy"),
            url_row("user/Harmony"),
            url_row("user/Painter"),
            url_row("user/Poet"),
            url_row("user/Farmer"),
            url_row("user/Natural"),
            url_row("user/Snappy"),
            url_row("user/The quick brown fox jumped over the lazy dog by the zigzag quarry site"),
        ],
    );

    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jaq")
        .arg(r#"."fullname""#)
        .arg("--rate-limit")
        .arg("4")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "Fullname"],
        url_pair("user/Smurfette", "Smurfette Smurf"),
        url_pair("user/Papa", "Papa Smurf"),
        url_pair("user/Clumsy", "Clumsy Smurf"),
        url_pair("user/Brainy", "Brainy Smurf"),
        url_pair("user/Grouchy", "Grouchy Smurf"),
        url_pair("user/Hefty", "Hefty Smurf"),
        url_pair("user/Greedy", "Greedy Smurf"),
        url_pair("user/Jokey", "Jokey Smurf"),
        url_pair("user/Chef", "Chef Smurf"),
        url_pair("user/Vanity", "Vanity Smurf"),
        url_pair("user/Handy", "Handy Smurf"),
        url_pair("user/Scaredy", "Scaredy Smurf"),
        url_pair("user/Tracker", "Tracker Smurf"),
        url_pair("user/Sloppy", "Sloppy Smurf"),
        url_pair("user/Harmony", "Harmony Smurf"),
        url_pair("user/Painter", "Painter Smurf"),
        url_pair("user/Poet", "Poet Smurf"),
        url_pair("user/Farmer", "Farmer Smurf"),
        url_pair("user/Natural", "Natural Smurf"),
        url_pair("user/Snappy", "Snappy Smurf"),
        url_pair(
            "user/The quick brown fox jumped over the lazy dog by the zigzag quarry site",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site Smurf",
        ),
    ];
    assert_eq!(got, expected);

    // init stop webserver and wait until server gracefully exit
    println!("STOPPING Webserver");
    rt::System::new().block_on(server_handle.stop(true));
}

#[test]
#[serial]
fn fetch_complex_url_template() {
    // start webserver with rate limiting (OS-assigned ephemeral port)
    let (server_handle, addr) = start_fetch_webserver();

    // proceed with usual unit test
    let wrk = Workdir::new("fetch_complex_template");
    wrk.create(
        "data.csv",
        vec![
            svec!["first name", "color"],
            svec!["Smurfette", "blue"],
            svec!["Papa", "blue"],
            svec!["Clumsy", "blue"],
            svec!["Brainy", "blue"],
            svec!["Grouchy", "blue"],
            svec!["Hefty", "blue"],
            svec!["Greedy", "green"],
            svec!["Jokey", "blue"],
            svec!["Chef", "blue"],
            svec!["Vanity", "blue"],
            svec!["Handy", "blue"],
            svec!["Scaredy", "black"],
            svec!["Tracker", "blue"],
            svec!["Sloppy", "blue"],
            svec!["Harmony", "blue"],
            svec!["Painter", "multicolor"],
            svec!["Poet", "blue"],
            svec!["Farmer", "blue"],
            svec!["Natural", "blue"],
            svec!["Snappy", "blue"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("--url-template")
        .arg(format!("http://{addr}/user/{{first_name}}%20{{color}}"))
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jaq")
        .arg(r#"."fullname""#)
        .arg("--rate-limit")
        .arg("4")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["first name", "color", "Fullname"],
        svec!["Smurfette", "blue", "Smurfette blue Smurf"],
        svec!["Papa", "blue", "Papa blue Smurf"],
        svec!["Clumsy", "blue", "Clumsy blue Smurf"],
        svec!["Brainy", "blue", "Brainy blue Smurf"],
        svec!["Grouchy", "blue", "Grouchy blue Smurf"],
        svec!["Hefty", "blue", "Hefty blue Smurf"],
        svec!["Greedy", "green", "Greedy green Smurf"],
        svec!["Jokey", "blue", "Jokey blue Smurf"],
        svec!["Chef", "blue", "Chef blue Smurf"],
        svec!["Vanity", "blue", "Vanity blue Smurf"],
        svec!["Handy", "blue", "Handy blue Smurf"],
        svec!["Scaredy", "black", "Scaredy black Smurf"],
        svec!["Tracker", "blue", "Tracker blue Smurf"],
        svec!["Sloppy", "blue", "Sloppy blue Smurf"],
        svec!["Harmony", "blue", "Harmony blue Smurf"],
        svec!["Painter", "multicolor", "Painter multicolor Smurf"],
        svec!["Poet", "blue", "Poet blue Smurf"],
        svec!["Farmer", "blue", "Farmer blue Smurf"],
        svec!["Natural", "blue", "Natural blue Smurf"],
        svec!["Snappy", "blue", "Snappy blue Smurf"],
    ];

    assert_eq!(got, expected);

    // init stop webserver and wait until server gracefully exit
    println!("STOPPING Webserver");
    rt::System::new().block_on(server_handle.stop(true));
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_simple_test() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
            svec!["https://httpbin.org/post", "c", "666", "true"],
            svec!["https://httpbin.org/post", "d", "33", "true"],
            svec!["https://httpbin.org/post", "e", "0", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("--new-column")
        .arg("response")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let mut got_parsed: Vec<Vec<String>> = Vec::new();
    let mut record_parsed: Vec<String> = Vec::new();

    for record in got {
        record_parsed.clear();
        record_parsed.push(record[1].to_string());
        record_parsed.push(record[2].to_string());
        record_parsed.push(record[3].to_string());
        record_parsed.push(record[4].to_string());

        got_parsed.push(record_parsed.clone());
    }

    let expected = vec![
        svec!["col1", "number col", "bool_col", "response"],
        svec![
            "a",
            "42",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"a\",\"number col\":\"42\"}"
        ],
        svec![
            "b",
            "3.14",
            "false",
            "{\"bool_col\":\"false\",\"col1\":\"b\",\"number col\":\"3.14\"}"
        ],
        svec![
            "c",
            "666",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"c\",\"number col\":\"666\"}"
        ],
        svec![
            "d",
            "33",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"d\",\"number col\":\"33\"}"
        ],
        svec![
            "e",
            "0",
            "false",
            "{\"bool_col\":\"false\",\"col1\":\"e\",\"number col\":\"0\"}"
        ],
    ];

    assert_eq!(got_parsed, expected);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_simple_diskcache() {
    let wrk = Workdir::new("fetchpost_diskcache");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
            svec!["https://httpbin.org/post", "c", "666", "true"],
            svec!["https://httpbin.org/post", "d", "33", "true"],
            svec!["https://httpbin.org/post", "e", "0", "false"],
        ],
    );

    // use an isolated, per-test disk-cache directory inside the workdir (which is
    // uniquely named per test and auto-removed on drop). Being fresh each run, the
    // redb-file assertion below genuinely verifies THIS run created the cache -
    // without deleting any shared/global temp path.
    use std::fs;
    let temp_dir = wrk.path("fp_dcache");
    fs::create_dir_all(&temp_dir).unwrap();
    let dc_dir = temp_dir.as_os_str().to_str().unwrap();

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("--new-column")
        .arg("response")
        .arg("--disk-cache")
        .args(["--disk-cache-dir", dc_dir])
        .args(["--rate-limit", "2"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let mut got_parsed: Vec<Vec<String>> = Vec::new();
    let mut record_parsed: Vec<String> = Vec::new();

    for record in got {
        record_parsed.clear();
        record_parsed.push(record[1].to_string());
        record_parsed.push(record[2].to_string());
        record_parsed.push(record[3].to_string());
        record_parsed.push(record[4].to_string());

        got_parsed.push(record_parsed.clone());
    }

    let expected = vec![
        svec!["col1", "number col", "bool_col", "response"],
        svec![
            "a",
            "42",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"a\",\"number col\":\"42\"}"
        ],
        svec![
            "b",
            "3.14",
            "false",
            "{\"bool_col\":\"false\",\"col1\":\"b\",\"number col\":\"3.14\"}"
        ],
        svec![
            "c",
            "666",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"c\",\"number col\":\"666\"}"
        ],
        svec![
            "d",
            "33",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"d\",\"number col\":\"33\"}"
        ],
        svec![
            "e",
            "0",
            "false",
            "{\"bool_col\":\"false\",\"col1\":\"e\",\"number col\":\"0\"}"
        ],
    ];

    assert_eq!(got_parsed, expected);

    // cached v3 uses redb (a single file), not sled (a directory): the on-disk
    // cache is `{name}_v{DISK_FILE_VERSION}.redb`, not `{name}_v1/conf`.
    assert!(temp_dir.join("fetchpost_v3.redb").exists());

    // let mut cmd_2 = wrk.command("fetchpost");
    // cmd.arg("URL")
    //     .arg("bool_col,col1,number col")
    //     .arg("--jaq")
    //     .arg(r#""form""#)
    //     .arg("--new-column")
    //     .arg("response")
    //     .arg("--disk-cache")
    //     .args(&["--disk-cache-dir", dc_dir])
    //     .args(&["--rate-limit", "2"])

    //     // .args(&["--report", "short"])
    //     .arg("data.csv");

    // let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2);

    // let mut got_parsed2: Vec<Vec<String>> = Vec::new();
    // let mut record_parsed2: Vec<String> = Vec::new();

    // for record in got {
    //     record_parsed2.clear();
    //     record_parsed2.push(record[1].to_string());
    //     record_parsed2.push(record[2].to_string());
    //     record_parsed2.push(record[3].to_string());
    //     record_parsed2.push(record[4].to_string());

    //     got_parsed2.push(record_parsed2.clone());
    // }
    // assert_eq!(got_parsed2, expected);

    // // sleep for a bit to make sure the cache is written to disk
    // std::thread::sleep(std::time::Duration::from_secs(2));

    // let fetchpostreport = wrk.read_to_string("data.csv.fetchpost-report.tsv");
    // wrk.create_from_string("no-elapsed.tsv", &fetchpostreport);

    // // remove the elapsed_ms column from the report as this is not deterministic
    // let mut cmd3 = wrk.command("select");
    // cmd3.arg("!elapsed_ms").arg("no-elapsed.tsv");

    // let fetchreport_noelapsed = wrk.stdout::<String>(&mut cmd3);
    // // read the output file and compare it with the expected output
    // assert_eq!(
    //     fetchreport_noelapsed,
    //     r#"url,status,cache_hit,retries,response
    // https://api.zippopotam.us/us/99999,404,1,5,"{""errors"":[{""title"":""HTTP ERROR"",""detail"":""HTTP ERROR 404 - Not Found""}]}"
    // https://api.zippopotam.us/us/90210,200,1,0,"{""post code"":""90210"",""country"":""United States"",""country abbreviation"":""US"",""places"":[{""place name"":""Beverly Hills"",""longitude"":""-118.4065"",""state"":""California"",""state abbreviation"":""CA"",""latitude"":""34.0901""}]}"
    // https://api.zippopotam.us/us/94105,200,1,0,"{""post code"":""94105"",""country"":""United States"",""country abbreviation"":""US"",""places"":[{""place name"":""San Francisco"",""longitude"":""-122.3892"",""state"":""California"",""state abbreviation"":""CA"",""latitude"":""37.7864""}]}"
    // https://api.zippopotam.us/us/92802,200,0,0,"{""post code"":""92802"",""country"":""United States"",""country abbreviation"":""US"",""places"":[{""place name"":""Anaheim"",""longitude"":""-117.9228"",""state"":""California"",""state abbreviation"":""CA"",""latitude"":""33.8085""}]}"
    // thisisnotaurl,404,0,0,"{""errors"":[{""title"":""Invalid URL"",""detail"":""relative URL
    // without a base""}]}""# );
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_compress_test() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
            svec!["https://httpbin.org/post", "c", "666", "true"],
            svec!["https://httpbin.org/post", "d", "33", "true"],
            svec!["https://httpbin.org/post", "e", "0", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("--new-column")
        .arg("response")
        .arg("--compress")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let mut got_parsed: Vec<Vec<String>> = Vec::new();
    let mut record_parsed: Vec<String> = Vec::new();

    for record in got {
        record_parsed.clear();
        record_parsed.push(record[1].to_string());
        record_parsed.push(record[2].to_string());
        record_parsed.push(record[3].to_string());
        record_parsed.push(record[4].to_string());

        got_parsed.push(record_parsed.clone());
    }

    // this garbled response is actually expected, as httpbin.org does not
    // decompress compressed requests so it doesn't get zip-bombed.
    // so it just echoed back the gzipped request body.
    // https://github.com/postmanlabs/httpbin/issues/577#issuecomment-875814469
    // but if this was sent to an internal server that did decompress, it would work.
    // The garbled response varies in exact byte representation across jaq versions,
    // so we just validate the structure: 6 rows (header + 5 data), 4 columns each,
    // and the response column starts with '{' (garbled form data parsed as object).
    assert_eq!(got_parsed.len(), 6);
    assert_eq!(
        got_parsed[0],
        svec!["col1", "number col", "bool_col", "response"]
    );
    let expected_cols = [
        ("a", "42", "true"),
        ("b", "3.14", "false"),
        ("c", "666", "true"),
        ("d", "33", "true"),
        ("e", "0", "false"),
    ];
    for (i, (col1, num, boolcol)) in expected_cols.iter().enumerate() {
        let row = &got_parsed[i + 1];
        assert_eq!(row.len(), 4);
        assert_eq!(&row[0], col1);
        assert_eq!(&row[1], num);
        assert_eq!(&row[2], boolcol);
        assert!(
            row[3].starts_with('{'),
            "row {i} response should be garbled object, got: {}",
            row[3]
                .char_indices()
                .nth(50)
                .map_or(&row[3][..], |(i, _)| &row[3][..i])
        );
    }
}

#[test]
fn fetchpost_jaqfile_doesnotexist_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1", "number col", "bool_col"],
            svec!["https://httpbin.org/post", "a", "42", "true"],
            svec!["https://httpbin.org/post", "b", "3.14", "false"],
            svec!["https://httpbin.org/post", "c", "666", "true"],
            svec!["https://httpbin.org/post", "d", "33", "true"],
            svec!["https://httpbin.org/post", "e", "0", "false"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("bool_col,col1,number col")
        .arg("--jaqfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/doesnotexist.jaq"
        ))
        .arg("--new-column")
        .arg("response")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_literalurl_test() {
    let wrk = Workdir::new("fetch_literalurl_test");
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "number col", "bool_col"],
            svec!["a", "42", "true"],
            svec!["b", "3.14", "false"],
            svec!["c", "666", "true"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("https://httpbin.org/post")
        .arg("bool_col,col1,number col")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("--new-column")
        .arg("response")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let mut got_parsed: Vec<Vec<String>> = Vec::new();
    let mut record_parsed: Vec<String> = Vec::new();

    for record in got {
        record_parsed.clear();
        record_parsed.push(record[0].to_string());
        record_parsed.push(record[1].to_string());
        record_parsed.push(record[2].to_string());
        record_parsed.push(record[3].to_string());

        got_parsed.push(record_parsed.clone());
    }

    let expected = vec![
        svec!["col1", "number col", "bool_col", "response"],
        svec![
            "a",
            "42",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"a\",\"number col\":\"42\"}"
        ],
        svec![
            "b",
            "3.14",
            "false",
            "{\"bool_col\":\"false\",\"col1\":\"b\",\"number col\":\"3.14\"}"
        ],
        svec![
            "c",
            "666",
            "true",
            "{\"bool_col\":\"true\",\"col1\":\"c\",\"number col\":\"666\"}"
        ],
    ];

    assert_eq!(got_parsed, expected);
}

#[test]
fn fetchpost_simple_report() {
    let wrk = Workdir::new("fetchpost_simple_report");
    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "number_col", "bool_col"],
            svec!["a", "42", "true"],
            svec!["b", "3.14", "false"],
            svec!["c", "666", "true"],
        ],
    );
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("https://httpbin.org/post")
        .arg("bool_col,col1,number_col")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("--new-column")
        .arg("response")
        .arg("--report")
        .arg("short")
        .arg("data.csv");

    let mut cmd = wrk.command("index");
    cmd.arg("data.csv.fetchpost-report.tsv");

    let mut cmd = wrk.command("select");
    cmd.arg("url,form,status,cache_hit,retries,response")
        .arg(wrk.load_test_file("data.csv.fetchpost-report.tsv"));

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["url", "form", "status", "cache_hit", "retries", "response"],
        svec![
            "https://httpbin.org/post",
            "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number_col\": \
             String(\"42\")}",
            "200",
            "0",
            "0",
            "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number_col\": \
             String(\"42\")}"
        ],
        svec![
            "https://httpbin.org/post",
            "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number_col\": \
             String(\"3.14\")}",
            "200",
            "0",
            "0",
            "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number_col\": \
             String(\"3.14\")}"
        ],
        svec![
            "https://httpbin.org/post",
            "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number_col\": \
             String(\"666\")}",
            "200",
            "0",
            "0",
            "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number_col\": \
             String(\"666\")}"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_payload_template() {
    let wrk = Workdir::new("fetchpost_tpl");
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name", "last_name", "age", "city"],
            svec!["John", "Smith", "35", "New York"],
            svec!["Jane", "Doe", "28", "Los Angeles"],
            svec!["Bob", "Jones", "42", "Chicago"],
        ],
    );

    // Create template file
    wrk.create_from_string(
        "payload.tpl",
        r#"{
    "firstName": "{{ first_name }}",
    "lastName": "{{ last_name }}",
    "age": {{ age }},
    "city": "{{ city }}"
}"#,
    );

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("https://httpbin.org/post")
        .arg("--payload-tpl")
        .arg("payload.tpl")
        .arg("--new-column")
        .arg("response")
        .arg("--jaq")
        .arg(r#"."data""#)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["first_name", "last_name", "age", "city", "response"],
        svec![
            "John",
            "Smith",
            "35",
            "New York",
            r#"{"firstName":"John","lastName":"Smith","age":35,"city":"New York"}"#
        ],
        svec![
            "Jane",
            "Doe",
            "28",
            "Los Angeles",
            r#"{"firstName":"Jane","lastName":"Doe","age":28,"city":"Los Angeles"}"#
        ],
        svec![
            "Bob",
            "Jones",
            "42",
            "Chicago",
            r#"{"firstName":"Bob","lastName":"Jones","age":42,"city":"Chicago"}"#
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_payload_template_with_globals() {
    let wrk = Workdir::new("fetchpost_tpl");
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name", "last_name", "age", "city"],
            svec!["John", "Smith", "35", "New York"],
            svec!["Jane", "Doe", "28", "Los Angeles"],
            svec!["Bob", "Jones", "42", "Chicago"],
        ],
    );

    // Create template file
    wrk.create_from_string(
        "payload.tpl",
        r#"{
    "firstName": "{{ first_name }}",
    "lastName": "{{ last_name }}",
    "age": {{ age }},
    "dog_age": {{ age|int * qsv_g.dog_years_multiplier|int }},
    "cat_age": {{ age|int * qsv_g.cat_years_multiplier|int }},
    "city": "{{ city }}"
}"#,
    );

    // Create globals JSON file
    wrk.create_from_string(
        "globals.json",
        r#"{
        "dog_years_multiplier": "7",
        "cat_years_multiplier": "14"
    }"#,
    );

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("https://httpbin.org/post")
        .arg("--payload-tpl")
        .arg("payload.tpl")
        .args(["--globals-json", "globals.json"])
        .arg("--new-column")
        .arg("response")
        .arg("--jaq")
        .arg(r#"."data""#)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["first_name", "last_name", "age", "city", "response"],
        svec![
            "John",
            "Smith",
            "35",
            "New York",
            r#"{"firstName":"John","lastName":"Smith","age":35,"dog_age":245,"cat_age":490,"city":"New York"}"#
        ],
        svec![
            "Jane",
            "Doe",
            "28",
            "Los Angeles",
            r#"{"firstName":"Jane","lastName":"Doe","age":28,"dog_age":196,"cat_age":392,"city":"Los Angeles"}"#
        ],
        svec![
            "Bob",
            "Jones",
            "42",
            "Chicago",
            r#"{"firstName":"Bob","lastName":"Jones","age":42,"dog_age":294,"cat_age":588,"city":"Chicago"}"#
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_payload_template_with_report() {
    let wrk = Workdir::new("fetchpost_tpl_report");
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name", "last_name", "age", "city"],
            svec!["John", "Smith", "35", "New York"],
            svec!["Jane", "Doe", "28", "Los Angeles"],
            svec!["Bob", "Jones", "42", "Chicago"],
        ],
    );

    // Create template file
    wrk.create_from_string(
        "payload.tpl",
        r#"{
    "firstName": "{{ first_name }}",
    "lastName": "{{ last_name }}",
    "age": {{ age }},
    "city": "{{ city }}"
}"#,
    );

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("https://httpbin.org/post")
        .arg("--payload-tpl")
        .arg("payload.tpl")
        .arg("--new-column")
        .arg("response")
        .arg("--jaq")
        .arg(r#"."data""#)
        .arg("--report")
        .arg("short")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["first_name", "last_name", "age", "city", "response"],
        svec![
            "John",
            "Smith",
            "35",
            "New York",
            r#"{"firstName":"John","lastName":"Smith","age":35,"city":"New York"}"#
        ],
        svec![
            "Jane",
            "Doe",
            "28",
            "Los Angeles",
            r#"{"firstName":"Jane","lastName":"Doe","age":28,"city":"Los Angeles"}"#
        ],
        svec![
            "Bob",
            "Jones",
            "42",
            "Chicago",
            r#"{"firstName":"Bob","lastName":"Jones","age":42,"city":"Chicago"}"#
        ],
    ];

    assert_eq!(got, expected);

    let report = wrk.read_to_string("data.csv.fetchpost-report.tsv").unwrap();
    assert!(!report.is_empty());
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_with_headers() {
    let wrk = Workdir::new("fetchpost_headers");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1"],
            svec!["https://httpbin.org/post", "test"],
        ],
    );

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("col1")
        .arg("--http-header")
        .arg("X-Test-Header:test123")
        .arg("--http-header")
        .arg("X-Another-Header:abc")
        .arg("--jaq")
        .arg(r#"."headers""#)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.stdout::<String>(&mut cmd);
    assert!(got.contains("X-Test-Header"));
    assert!(got.contains("test123"));
    assert!(got.contains("X-Another-Header"));
    assert!(got.contains("abc"));
}

#[test]
fn fetchpost_disk_cache() {
    let wrk = Workdir::new("fetchpost_disk");

    // Create temp dir for cache
    use std::{env, fs};
    let temp_dir = env::temp_dir().join("fp_dcache_test");
    fs::create_dir_all(&temp_dir).unwrap();
    let dc_dir = temp_dir.as_os_str().to_str().unwrap();

    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "col1"],
            svec!["https://httpbin.org/post", "test"],
        ],
    );

    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("col1")
        .arg("--disk-cache")
        .arg("--disk-cache-dir")
        .arg(dc_dir)
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("data.csv");

    // First request should not be cached
    let got1 = wrk.stdout::<String>(&mut cmd);

    // Second request should be cached
    let got2 = wrk.stdout::<String>(&mut cmd);

    assert_eq!(&got1, &got2);

    // Clean up
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn fetchpost_content_type() {
    let wrk = Workdir::new("fetchpost_content_type");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL", "message"],
            svec!["https://httpbin.org/post", "Hello World"],
        ],
    );

    // Create template file
    wrk.create_from_string("payload.tpl", "Greeting: {{ message }}");

    // Test plain text content type
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("--payload-tpl")
        .arg("payload.tpl")
        .arg("--content-type")
        .arg("text/plain")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.stdout::<String>(&mut cmd);
    assert!(got.starts_with(
        r#"{"args":{},"data":"\"Greeting: Hello World\"","files":{},"form":{},"headers":{"#
    ));

    // Create JSON template file
    wrk.create_from_string(
        "jsonpayload.tpl",
        r#"{
    "URL": "{{ URL }}",
    "Message": "{{ message }}"
}"#,
    );

    // Test custom JSON content type
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("--payload-tpl")
        .arg("jsonpayload.tpl")
        .arg("--content-type")
        .arg("application/json")
        .arg("--jaq")
        .arg(r#"."json""#)
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    assert_eq!(
        got,
        r#"{"Message":"Hello World","URL":"https://httpbin.org/post"}"#
    );

    // Test form data content type
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("message")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    assert_eq!(got, r#"{"message":"Hello World"}"#);
}

#[test]
#[ignore = "flaky: depends on httpbin.org external service"]
fn test_fetchpost_column_list_globals() {
    let wrk = Workdir::new("fetchpost");
    wrk.create_from_string(
        "data.csv",
        "URL,message\nhttps://httpbin.org/post,Hello World\n",
    );

    // Create globals JSON file
    wrk.create_from_string(
        "globals.json",
        r#"{
    "api_key": "secret123",
    "user_id": "user456"
}"#,
    );

    // Test form data with globals
    let mut cmd = wrk.command("fetchpost");
    cmd.arg("URL")
        .arg("message")
        .arg("--globals-json")
        .arg("globals.json")
        .arg("--jaq")
        .arg(r#"."form""#)
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    assert_eq!(
        got,
        r#"{"api_key":"secret123","message":"Hello World","user_id":"user456"}"#
    );
}

#[test]
fn test_fetch_jaq_invalid_json() {
    let wrk = Workdir::new("fetch_jaq_invalid");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec![
                "<!doctype html><html lang=\"en\"><meta charset=utf-8><title>shortest \
                 html5</title>"
            ],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("result")
        .arg("--jaq")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "result"],
        svec![
            "<!doctype html><html lang=\"en\"><meta charset=utf-8><title>shortest html5</title>",
            ""
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn test_fetch_jaq_invalid_selector() {
    let wrk = Workdir::new("fetch_jaq_invalid_selector");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["https://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("result")
        .arg("--jaq")
        .arg(r#"."place"[0]."place name""#) // Invalid selector
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "result"],
        svec!["https://api.zippopotam.us/us/90210", ""],
    ];

    assert_eq!(got, expected);
}

#[test]
fn test_fetch_jaq_number() {
    let wrk = Workdir::new("fetch_jaq_number");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["https://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("longitude")
        .arg("--jaq")
        .arg(r#"."places"[0]."longitude""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "longitude"],
        svec!["https://api.zippopotam.us/us/90210", "-118.4065"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn test_fetch_jaq_array() {
    let wrk = Workdir::new("fetch_jaq_array");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["https://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("coordinates")
        .arg("--jaq")
        .arg(r#"[ ."places"[0]."longitude", ."places"[0]."latitude" ]"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "coordinates"],
        svec![
            "https://api.zippopotam.us/us/90210",
            "[\"-118.4065\",\"34.0901\"]"
        ],
    ];

    assert_eq!(got, expected);
}
