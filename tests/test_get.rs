use std::{
    net::SocketAddr,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::ServerHandle, rt, web};
use serial_test::serial;

use crate::workdir::Workdir;

const STATES_CSV: &str = "name,abbr\nAlabama,AL\nAlaska,AK\nArizona,AZ\nArkansas,AR\n";
const ETAG: &str = "\"states-v1\"";
const BIND_HOST: &str = "127.0.0.1";

// A request handler that serves STATES_CSV with an ETag and honors
// conditional GETs: a matching `If-None-Match` yields 304 (and does NOT
// increment the body-send counter), so a test can assert that a second
// `qsv get` revalidated rather than re-downloaded.
async fn serve_states(counter: web::Data<Arc<AtomicUsize>>, req: HttpRequest) -> HttpResponse {
    if let Some(inm) = req.headers().get("if-none-match")
        && inm.to_str().unwrap_or_default() == ETAG
    {
        return HttpResponse::NotModified()
            .insert_header(("etag", ETAG))
            .finish();
    }
    counter.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok()
        .insert_header(("etag", ETAG))
        .content_type("text/csv")
        .body(STATES_CSV)
}

async fn run_webserver(
    tx: mpsc::Sender<Result<(ServerHandle, SocketAddr), String>>,
    counter: Arc<AtomicUsize>,
) -> std::io::Result<()> {
    let server_builder = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .service(web::resource("/states.csv").to(serve_states))
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

struct GetWebServer {
    handle:  Option<ServerHandle>,
    addr:    SocketAddr,
    counter: Arc<AtomicUsize>,
}

impl GetWebServer {
    fn start() -> Self {
        let counter = Arc::new(AtomicUsize::new(0));
        let server_counter = counter.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || rt::System::new().block_on(run_webserver(tx, server_counter)));
        match rx.recv_timeout(std::time::Duration::from_secs(10)) {
            Ok(Ok((handle, addr))) => Self {
                handle: Some(handle),
                addr,
                counter,
            },
            Ok(Err(msg)) => panic!("test webserver failed to bind: {msg}"),
            Err(e) => panic!("test webserver did not start within 10s ({e:?})"),
        }
    }

    fn url(&self, path: &str) -> String {
        let path = path.strip_prefix('/').unwrap_or(path);
        format!("http://{}/{path}", self.addr)
    }

    fn body_sends(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

impl Drop for GetWebServer {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            rt::System::new().block_on(handle.stop(true));
        }
    }
}

// Count actual content blobs (excludes the `.idx.zst` index blobs) in the
// cache, to assert content-addressed dedup.
fn count_content_blobs(cache_dir: &Path) -> usize {
    let blobs = cache_dir.join("get").join("blobs");
    let mut n = 0;
    if let Ok(walk) = std::fs::read_dir(&blobs) {
        for shard1 in walk.flatten() {
            let Ok(s2) = std::fs::read_dir(shard1.path()) else {
                continue;
            };
            for shard2 in s2.flatten() {
                let Ok(files) = std::fs::read_dir(shard2.path()) else {
                    continue;
                };
                for f in files.flatten() {
                    let name = f.file_name().to_string_lossy().to_string();
                    if name.ends_with(".zst") && !name.ends_with(".idx.zst") {
                        n += 1;
                    }
                }
            }
        }
    }
    n
}

#[test]
fn get_local_file_and_dc_read() {
    let wrk = Workdir::new("get_local_file_and_dc_read");
    wrk.create_from_string("states_src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    // fetch the local file into the cache
    let mut cmd = wrk.command("get");
    cmd.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "states.csv"])
        .arg("states_src.csv");
    wrk.assert_success(&mut cmd);

    // cache-list should mention the entry
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("states.csv"),
        "cache-list missing entry:\n{stdout}"
    );

    // read it back via the dc: prefix; count must be 4 data rows
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:states.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "4");
}

#[test]
fn get_local_dedup_shares_blob() {
    let wrk = Workdir::new("get_local_dedup_shares_blob");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    for name in ["one.csv", "two.csv"] {
        let mut cmd = wrk.command("get");
        cmd.env("QSV_CACHE_DIR", &cache_dir)
            .args(["--name", name])
            .arg("src.csv");
        wrk.assert_success(&mut cmd);
    }

    // identical content under two logical names -> exactly one content blob
    assert_eq!(
        count_content_blobs(&cache_dir),
        1,
        "content-addressed dedup should store a single blob for identical content"
    );
}

#[test]
#[serial]
fn get_http_etag_revalidation() {
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_http_etag_revalidation");
    let cache_dir = wrk.path("qsvcache");
    let url = server.url("states.csv");

    // first fetch downloads the body
    let mut first = wrk.command("get");
    first
        .env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "states.csv"])
        .arg(&url);
    wrk.assert_success(&mut first);
    assert_eq!(server.body_sends(), 1, "first get should download the body");

    // second fetch must NOT re-download (served fresh or revalidated via 304)
    let mut second = wrk.command("get");
    second
        .env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "states.csv"])
        .arg(&url);
    wrk.assert_success(&mut second);
    assert_eq!(
        server.body_sends(),
        1,
        "second get should not re-download the body (ETag revalidation)"
    );

    // and the cached data is correct & indexed (dc: count)
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:states.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "4");
}

#[test]
fn get_cache_clear() {
    let wrk = Workdir::new("get_cache_clear");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut cmd = wrk.command("get");
    cmd.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut cmd);

    let mut clear = wrk.command("get");
    clear.env("QSV_CACHE_DIR", &cache_dir).arg("cache-clear");
    wrk.assert_success(&mut clear);

    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("empty"),
        "cache should be empty after clear:\n{stdout}"
    );
}

#[test]
fn get_cache_prune_all() {
    let wrk = Workdir::new("get_cache_prune_all");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut cmd = wrk.command("get");
    cmd.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut cmd);

    // prune everything older than 0 seconds (i.e. everything)
    let mut prune = wrk.command("get");
    prune
        .env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-prune", "--older-than", "0s"]);
    wrk.assert_success(&mut prune);

    assert_eq!(
        count_content_blobs(&cache_dir),
        0,
        "prune should have removed the blob"
    );
}
