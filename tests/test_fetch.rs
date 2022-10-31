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
            svec!["  http://api.zippopotam.us/us/90210      "],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802      "],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv").arg("--store-error");

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}
{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}
{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}
{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#;
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
            svec!["  http://api.zippopotam.us/us/90210      ", "b", "2"],
            svec!["https://api.zippopotam.us/us/94105", "c", "3"],
            svec!["http://api.zippopotam.us/us/92802      ", "d", "4"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json", "Scott Adams", "42"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("response")
        .arg("data.csv")
        .arg("--store-error");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["URL", "col2", "col3", "response"],
        svec!["https://api.zippopotam.us/us/99999", "a", "1", ""],
        svec!["http://api.zippopotam.us/us/90210", "b", "2", r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#],
        svec!["https://api.zippopotam.us/us/94105", "c", "3", r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#],
        svec!["http://api.zippopotam.us/us/92802", "d", "4", r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#],
        svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json", "Scott Adams", "42", r#"{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#],
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
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
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
        svec!["https://api.zippopotam.us/us/07094", "200", "0", "5", r#"{"post code":"07094","country":"United States","country abbreviation":"US","places":[{"place name":"Secaucus","longitude":"-74.0634","state":"New Jersey","state abbreviation":"NJ","latitude":"40.791"}]}"#],
        svec!["https://api.zippopotam.us/us/90210", "200", "0", "0", r#"{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}"#],
        svec!["https://api.zippopotam.us/us/94105", "200", "0", "0", r#"{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}"#],
        svec!["https://api.zippopotam.us/us/92802", "200", "0", "0", r#"{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#],
        svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json", "200", "0", "0", r#"{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#],
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
{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}
{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}
{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}"#;

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
            svec!["  http://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802"],
            svec!["thisisnotaurl"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("data.csv")
        .arg("--store-error")
        .arg("--redis");

    let got = wrk.stdout::<String>(&mut cmd);

    let expected = r#"{"errors":[{"title":"HTTP ERROR","detail":"HTTP ERROR 404 - Not Found"}]}
{"post code":"90210","country":"United States","country abbreviation":"US","places":[{"place name":"Beverly Hills","longitude":"-118.4065","state":"California","state abbreviation":"CA","latitude":"34.0901"}]}
{"post code":"94105","country":"United States","country abbreviation":"US","places":[{"place name":"San Francisco","longitude":"-122.3892","state":"California","state abbreviation":"CA","latitude":"37.7864"}]}
{"post code":"92802","country":"United States","country abbreviation":"US","places":[{"place name":"Anaheim","longitude":"-117.9228","state":"California","state abbreviation":"CA","latitude":"33.8085"}]}
{"errors":[{"title":"Invalid URL","detail":"relative URL without a base"}]}
{"head":{"vars":["dob"]},"results":{"bindings":[{"dob":{"datatype":"http://www.w3.org/2001/XMLSchema#dateTime","type":"literal","value":"1952-03-11T00:00:00Z"}}]}}"#;

    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_single() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["thisisnotaurl"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jql")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["thisisnotaurl", ""],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_single_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_single.jql"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jqlfile_doesnotexist_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/doesnotexist.jql"
        ))
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetch_jql_jqlfile_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://api.zippopotam.us/us/90210"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_single.jql"
        ))
        .arg("--jql")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("Invalid arguments."));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetch_jql_multiple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jql")
        .arg(r#""places"[0]."place name","places"[0]."state abbreviation""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills, CA"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco, CA"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim, CA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql_multiple_file() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("CityState")
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/fetch_jql_multiple.jql"
        ))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "CityState"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills, CA"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco, CA"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim, CA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_custom_header() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg(" X-Api-Key :  DEMO_KEY")
        .arg("--http-header")
        .arg("X-Api-Secret :ABC123XYZ")
        .arg("--jql")
        .arg(r#""headers"."X-Api-Key","headers"."X-Api-Secret""#)
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    let expected = "DEMO_KEY, ABC123XYZ";
    assert_eq!(got, expected);
}

#[test]
fn fetch_custom_invalid_header_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg("X-Api-\tSecret :ABC123XYZ") // embedded tab is not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("Invalid header name"));

    wrk.assert_err(&mut cmd);
}
#[test]
fn fetch_custom_invalid_user_agent_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--user-agent")
        .arg("Mozilla/5.0\t (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion") // embedded tab is not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    //#assert!(got.starts_with("Invalid header name"));

    wrk.assert_err(&mut cmd);
}

#[test]
fn fetch_custom_user_agent() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--user-agent")
        .arg("Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion")
        .arg("data.csv");

    let got = wrk.stdout::<String>(&mut cmd);
    let expected =
        "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion";
    wrk.assert_success(&mut cmd);
}

#[test]
fn fetch_custom_invalid_value_error() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![svec!["URL"], svec!["http://httpbin.org/get"]],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--http-header")
        .arg("X-Api-Secret :ABC123\r\nXYZ") // non-visible ascii not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("Invalid header value"));

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
        .arg("--http-header")
        .arg("X-Api-\tSecret :ABC123XYZ") // non-visible ascii not valid
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("Invalid header name"));

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
    assert!(got.starts_with("Invalid header value"));

    wrk.assert_err(&mut cmd);
}

use std::{sync::mpsc, thread};

use actix_web::{
    dev::ServerHandle, middleware, rt, web, App, HttpRequest, HttpServer, Responder, Result,
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
    println!("{:?}", req);

    let obj = MyObj {
        fullname: format!("{} Smurf", name),
    };

    Ok(web::Json(obj))
}

// convenience macros for changing test ip/port to use
macro_rules! test_server {
    () => {
        "127.0.0.1:8081"
    };
}

macro_rules! test_url {
    ($api_parm:expr) => {
        concat!("http://", test_server!(), "/", $api_parm)
    };
}

/// start an Actix Webserver with Rate Limiting via Governor
async fn run_webserver(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    use actix_governor::{Governor, GovernorConfigBuilder};

    // Allow bursts with up to five requests per IP address
    // and replenishes one element every 250 ms (4 qps)
    let governor_conf = GovernorConfigBuilder::default()
        .per_millisecond(250)
        .burst_size(7)
        .finish()
        .unwrap();

    // server is server controller type, `dev::ServerHandle`
    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(Governor::new(&governor_conf))
            .service(web::resource("/user/{name}").route(web::get().to(get_fullname)))
            .service(web::resource("/").to(index))
    })
    .bind(test_server!())?
    .run();

    // send server controller to main thread
    let _ = tx.send(server.handle());

    // run future
    server.await
}

#[test]
#[serial]
fn fetch_ratelimit() {
    // start webserver with rate limiting
    let (tx, rx) = mpsc::channel();

    println!("START Webserver ");
    thread::spawn(move || {
        let server_future = run_webserver(tx);
        rt::System::new().block_on(server_future)
    });

    let server_handle = rx.recv().expect("test webserver error");

    // proceed with usual unit test
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec![test_url!("user/Smurfette")],
            svec![test_url!("user/Papa")],
            svec![test_url!("user/Clumsy")],
            svec![test_url!("user/Brainy")],
            svec![test_url!("user/Grouchy")],
            svec![test_url!("user/Hefty")],
            svec![test_url!("user/Greedy")],
            svec![test_url!("user/Jokey")],
            svec![test_url!("user/Chef")],
            svec![test_url!("user/Vanity")],
            svec![test_url!("user/Handy")],
            svec![test_url!("user/Scaredy")],
            svec![test_url!("user/Tracker")],
            svec![test_url!("user/Sloppy")],
            svec![test_url!("user/Harmony")],
            svec![test_url!("user/Painter")],
            svec![test_url!("user/Poet")],
            svec![test_url!("user/Farmer")],
            svec![test_url!("user/Natural")],
            svec![test_url!("user/Snappy")],
            svec![test_url!(
                "user/The quick brown fox jumped over the lazy dog by the zigzag quarry site"
            )],
        ],
    );

    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jql")
        .arg(r#"."fullname""#)
        .arg("--rate-limit")
        .arg("4")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "Fullname"],
        svec![test_url!("user/Smurfette"), "Smurfette Smurf"],
        svec![test_url!("user/Papa"), "Papa Smurf"],
        svec![test_url!("user/Clumsy"), "Clumsy Smurf"],
        svec![test_url!("user/Brainy"), "Brainy Smurf"],
        svec![test_url!("user/Grouchy"), "Grouchy Smurf"],
        svec![test_url!("user/Hefty"), "Hefty Smurf"],
        svec![test_url!("user/Greedy"), "Greedy Smurf"],
        svec![test_url!("user/Jokey"), "Jokey Smurf"],
        svec![test_url!("user/Chef"), "Chef Smurf"],
        svec![test_url!("user/Vanity"), "Vanity Smurf"],
        svec![test_url!("user/Handy"), "Handy Smurf"],
        svec![test_url!("user/Scaredy"), "Scaredy Smurf"],
        svec![test_url!("user/Tracker"), "Tracker Smurf"],
        svec![test_url!("user/Sloppy"), "Sloppy Smurf"],
        svec![test_url!("user/Harmony"), "Harmony Smurf"],
        svec![test_url!("user/Painter"), "Painter Smurf"],
        svec![test_url!("user/Poet"), "Poet Smurf"],
        svec![test_url!("user/Farmer"), "Farmer Smurf"],
        svec![test_url!("user/Natural"), "Natural Smurf"],
        svec![test_url!("user/Snappy"), "Snappy Smurf"],
        svec![
            test_url!(
                "user/The quick brown fox jumped over the lazy dog by the zigzag quarry site"
            ),
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site Smurf"
        ],
    ];
    assert_eq!(got, expected);

    // init stop webserver and wait until server gracefully exit
    println!("STOPPING Webserver");
    rt::System::new().block_on(server_handle.stop(true));
}

#[test]
#[serial]
fn fetch_complex_url_template() {
    // start webserver with rate limiting
    let (tx, rx) = mpsc::channel();

    println!("START Webserver ");
    thread::spawn(move || {
        let server_future = run_webserver(tx);
        rt::System::new().block_on(server_future)
    });

    let server_handle = rx.recv().unwrap();

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
        .arg(concat!(
            "http://",
            test_server!(),
            "/user/{first_name}%20{color}"
        ))
        .arg("--new-column")
        .arg("Fullname")
        .arg("--jql")
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
        .arg("--jql")
        .arg(r#""form""#)
        .arg("--new-column")
        .arg("response")
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
        svec!["a", "42", "true", "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number col\": String(\"42\")}"],
        svec!["b", "3.14", "false", "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number col\": String(\"3.14\")}"],
        svec!["c", "666", "true", "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number col\": String(\"666\")}"],
        svec!["d", "33", "true", "{\"bool_col\": String(\"true\"), \"col1\": String(\"d\"), \"number col\": String(\"33\")}"],
        svec!["e", "0", "false", "{\"bool_col\": String(\"false\"), \"col1\": String(\"e\"), \"number col\": String(\"0\")}"],
    ];

    assert_eq!(got_parsed, expected);
}

#[test]
fn fetchpost_jqlfile_doesnotexist_error() {
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
        .arg("--jqlfile")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/doesnotexist.jql"
        ))
        .arg("--new-column")
        .arg("response")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
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
        .arg("--jql")
        .arg(r#""form""#)
        .arg("--new-column")
        .arg("response")
        .arg("data.csv");

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
        svec!["a", "42", "true", "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number col\": String(\"42\")}"],
        svec!["b", "3.14", "false", "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number col\": String(\"3.14\")}"],
        svec!["c", "666", "true", "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number col\": String(\"666\")}"],
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
        .arg("--jql")
        .arg(r#""form""#)
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
        svec!["https://httpbin.org/post", "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number_col\": String(\"42\")}", "200", "0", "0", "{\"bool_col\": String(\"true\"), \"col1\": String(\"a\"), \"number_col\": String(\"42\")}"],
        svec!["https://httpbin.org/post", "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number_col\": String(\"3.14\")}", "200", "0", "0", "{\"bool_col\": String(\"false\"), \"col1\": String(\"b\"), \"number_col\": String(\"3.14\")}"],
        svec!["https://httpbin.org/post", "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number_col\": String(\"666\")}", "200", "0", "0", "{\"bool_col\": String(\"true\"), \"col1\": String(\"c\"), \"number_col\": String(\"666\")}"],
    ];

    assert_eq!(got, expected);
}
