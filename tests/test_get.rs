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
// Local mock-server bind address for these tests (not production code).
const BIND_HOST: &str = "127.0.0.1"; // DevSkim: ignore DS162092

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

// Like `serve_states` but explicitly fresh (Cache-Control: max-age), so a
// second request within the window is served from cache WITHOUT revalidation
// (no `put` on the manager) — the path that previously failed when a cache hit
// was requested under a different logical name.
async fn serve_states_fresh(counter: web::Data<Arc<AtomicUsize>>) -> HttpResponse {
    counter.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok()
        .insert_header(("etag", ETAG))
        .insert_header(("cache-control", "max-age=3600"))
        .content_type("text/csv")
        .body(STATES_CSV)
}

// A second fresh endpoint with DIFFERENT (1-row) content, for testing that a
// fresh cache hit repoints a name that previously pointed at another entry.
async fn serve_one_fresh(counter: web::Data<Arc<AtomicUsize>>) -> HttpResponse {
    counter.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok()
        .insert_header(("etag", "\"one-v1\""))
        .insert_header(("cache-control", "max-age=3600"))
        .content_type("text/csv")
        .body("name,abbr\nFoo,FO\n")
}

async fn run_webserver(
    tx: mpsc::Sender<Result<(ServerHandle, SocketAddr), String>>,
    counter: Arc<AtomicUsize>,
) -> std::io::Result<()> {
    let server_builder = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .service(web::resource("/states.csv").to(serve_states))
            .service(web::resource("/states_fresh.csv").to(serve_states_fresh))
            .service(web::resource("/one_fresh.csv").to(serve_one_fresh))
            // Path-style S3 object: object_store issues `GET /{bucket}/{key}`
            // against the endpoint override. Reuses the ETag/304 handler so the
            // cloud path can assert revalidation just like the HTTP path.
            .service(web::resource("/test-bucket/states.csv").to(serve_states))
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
        // Plain HTTP to the in-process test mock server (not production code).
        format!("http://{}/{path}", self.addr) // DevSkim: ignore DS137138
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
                    // content blobs are `{b3}.zst` (zstd) or `{b3}.raw` (none);
                    // exclude the `{b3}.idx.zst` index blobs.
                    let is_content = (name.ends_with(".zst") && !name.ends_with(".idx.zst"))
                        || name.ends_with(".raw");
                    if is_content {
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
#[serial]
fn get_http_same_url_different_name() {
    // Regression: a fresh cache hit requested under a NEW logical name used to
    // fail because no alias was created for that name (the manager's `put` is
    // never called on a fresh hit).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_http_same_url_different_name");
    let cache_dir = wrk.path("qsvcache");
    let url = server.url("states_fresh.csv");

    let mut a = wrk.command("get");
    a.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "a.csv"])
        .arg(&url);
    wrk.assert_success(&mut a);

    // same URL, different logical name -> must succeed, served from cache
    let mut b = wrk.command("get");
    b.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "b.csv"])
        .arg(&url);
    wrk.assert_success(&mut b);
    assert_eq!(
        server.body_sends(),
        1,
        "second name should reuse the cached body, not re-download"
    );

    // both dc: handles resolve to the cached data
    for name in ["a.csv", "b.csv"] {
        let mut c = wrk.command("count");
        c.env("QSV_CACHE_DIR", &cache_dir).arg(format!("dc:{name}"));
        let got: String = wrk.stdout(&mut c);
        assert_eq!(got, "4", "dc:{name} should resolve to the cached data");
    }

    // cache-list shows BOTH names (each alias is its own row), sharing one blob
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("a.csv") && stdout.contains("b.csv"),
        "cache-list should show both names:\n{stdout}"
    );
    assert_eq!(
        count_content_blobs(&cache_dir),
        1,
        "both names should share a single content blob"
    );
}

#[test]
#[serial]
fn get_force_refetches_shared_entry() {
    // Regression: --force must re-fetch from the origin even when the URL-keyed
    // entry is still referenced by another name (removing only the requested
    // alias would leave the entry to be served as a fresh hit).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_force_refetches_shared_entry");
    let cache_dir = wrk.path("qsvcache");
    let url = server.url("states_fresh.csv");

    for name in ["a.csv", "b.csv"] {
        let mut c = wrk.command("get");
        c.env("QSV_CACHE_DIR", &cache_dir)
            .args(["--name", name])
            .arg(&url);
        wrk.assert_success(&mut c);
    }
    assert_eq!(
        server.body_sends(),
        1,
        "two names share one downloaded body"
    );

    // --force on a name whose entry is shared by b.csv must still hit the origin
    let mut f = wrk.command("get");
    f.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "a.csv", "--force"])
        .arg(&url);
    wrk.assert_success(&mut f);
    assert_eq!(
        server.body_sends(),
        2,
        "--force should re-download even when the entry is shared by another name"
    );
}

#[test]
#[serial]
fn get_fresh_hit_repoints_stale_name() {
    // Regression: requesting a URL under a name that already points at a
    // DIFFERENT entry, where the URL is a fresh cache hit, must repoint the name
    // to the fetched entry instead of keeping the old (stale) data.
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_fresh_hit_repoints_stale_name");
    let cache_dir = wrk.path("qsvcache");
    let url_a = server.url("states_fresh.csv"); // 4 rows
    let url_b = server.url("one_fresh.csv"); // 1 row

    // x.csv -> url_a (4 rows)
    let mut g1 = wrk.command("get");
    g1.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg(&url_a);
    wrk.assert_success(&mut g1);

    // cache url_b under a different name so the next fetch of it is a fresh hit
    let mut g2 = wrk.command("get");
    g2.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "y.csv"])
        .arg(&url_b);
    wrk.assert_success(&mut g2);

    // now point x.csv at url_b: served fresh (no put), x.csv must repoint
    let mut g3 = wrk.command("get");
    g3.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg(&url_b);
    wrk.assert_success(&mut g3);

    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:x.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(
        got, "1",
        "x.csv should now resolve to url_b's data, not the stale entry"
    );
}

#[test]
fn get_compression_variant_blob_reclaimed() {
    // Regression: deleting an entry must free the exact blob file (content hash
    // AND compression), not assume any entry with the same hash shares it.
    let wrk = Workdir::new("get_compression_variant_blob_reclaimed");
    wrk.create_from_string("a.csv", STATES_CSV);
    wrk.create_from_string("b.csv", STATES_CSV); // identical content to a.csv
    wrk.create_from_string("c.csv", "name,abbr\nFoo,FO\n"); // different content
    let cache_dir = wrk.path("qsvcache");

    // same content under two cache keys, two compression variants (.zst, .raw)
    let mut g1 = wrk.command("get");
    g1.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv", "--compress", "zstd"])
        .arg("a.csv");
    wrk.assert_success(&mut g1);
    let mut g2 = wrk.command("get");
    g2.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "y.csv", "--compress", "none"])
        .arg("b.csv");
    wrk.assert_success(&mut g2);

    // reuse x.csv for different content -> orphans the .zst variant of the
    // shared hash; the .raw variant (y.csv) must survive.
    let mut g3 = wrk.command("get");
    g3.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("c.csv");
    wrk.assert_success(&mut g3);

    // remaining: b.csv's .raw + c.csv's blob == 2 (the orphaned .zst is gone)
    assert_eq!(
        count_content_blobs(&cache_dir),
        2,
        "the orphaned compression variant should be reclaimed by exact path"
    );
}

#[test]
fn get_refetch_changed_content_reclaims_blob() {
    // Regression: re-fetching the same source (same cache key) after its content
    // changed must reclaim the previous blob rather than orphan it.
    let wrk = Workdir::new("get_refetch_changed_content_reclaims_blob");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut g1 = wrk.command("get");
    g1.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut g1);

    // change the source content, then re-fetch under the same name
    wrk.create_from_string("src.csv", "name,abbr\nFoo,FO\n");
    let mut g2 = wrk.command("get");
    g2.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut g2);

    // the previous blob must be reclaimed -> exactly one content blob remains
    assert_eq!(
        count_content_blobs(&cache_dir),
        1,
        "re-fetch with changed content should reclaim the old blob"
    );

    // dc:x.csv reflects the new content (1 data row)
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:x.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "1");
}

#[test]
fn get_name_reuse_replaces_entry() {
    // Regression: reusing a logical name for a different source must not leave
    // the old entry/blob orphaned (duplicate names, inflated metadata).
    let wrk = Workdir::new("get_name_reuse_replaces_entry");
    wrk.create_from_string("first.csv", STATES_CSV);
    wrk.create_from_string("second.csv", "name,abbr\nFoo,FO\n");
    let cache_dir = wrk.path("qsvcache");

    let mut g1 = wrk.command("get");
    g1.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("first.csv");
    wrk.assert_success(&mut g1);

    // reuse the same name for a different source/content
    let mut g2 = wrk.command("get");
    g2.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("second.csv");
    wrk.assert_success(&mut g2);

    // the old (now-orphaned) blob must be gone -> exactly one content blob
    assert_eq!(
        count_content_blobs(&cache_dir),
        1,
        "name reuse should remove the orphaned old blob"
    );

    // dc:x.csv now reflects the new source (1 data row, not 4)
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:x.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "1");

    // and the name appears exactly once in cache-list
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        stdout.matches("x.csv").count(),
        1,
        "x.csv should appear once in cache-list:\n{stdout}"
    );
}

#[test]
fn get_alias_names_do_not_collide() {
    // Regression: logical names that a lossy sanitizer would map to the same
    // on-disk filename (e.g. "a b.csv" vs "a_b.csv") must not collide.
    let wrk = Workdir::new("get_alias_names_do_not_collide");
    wrk.create_from_string("src1.csv", STATES_CSV); // 4 rows
    wrk.create_from_string("src2.csv", "name,abbr\nFoo,FO\n"); // 1 row
    let cache_dir = wrk.path("qsvcache");

    let mut g1 = wrk.command("get");
    g1.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "a b.csv"])
        .arg("src1.csv");
    wrk.assert_success(&mut g1);
    let mut g2 = wrk.command("get");
    g2.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "a_b.csv"])
        .arg("src2.csv");
    wrk.assert_success(&mut g2);

    // each dc: handle resolves to its OWN content (no collision)
    let mut c1 = wrk.command("count");
    c1.env("QSV_CACHE_DIR", &cache_dir).arg("dc:a b.csv");
    let got1: String = wrk.stdout(&mut c1);
    assert_eq!(got1, "4");
    let mut c2 = wrk.command("count");
    c2.env("QSV_CACHE_DIR", &cache_dir).arg("dc:a_b.csv");
    let got2: String = wrk.stdout(&mut c2);
    assert_eq!(got2, "1");

    // cache-list shows both ORIGINAL names (decoded from the reversible alias)
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("a b.csv") && stdout.contains("a_b.csv"),
        "cache-list should show both original names:\n{stdout}"
    );
}

#[test]
fn get_long_logical_name() {
    // Regression: a long logical name must still cache. A hex-encoded alias
    // filename (2x length) would exceed the 255-byte filename limit; the hashed
    // alias filename keeps it bounded.
    let wrk = Workdir::new("get_long_logical_name");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");
    let long_name = format!("{}.csv", "n".repeat(200));

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", &long_name])
        .arg("src.csv");
    wrk.assert_success(&mut g);

    let mut count = wrk.command("count");
    count
        .env("QSV_CACHE_DIR", &cache_dir)
        .arg(format!("dc:{long_name}"));
    let got: String = wrk.stdout(&mut count);
    assert_eq!(
        got, "4",
        "a long logical name should cache and resolve via dc:"
    );
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

// Overwrite the first content blob found with `bytes`, simulating on-disk
// corruption. Returns true if a content blob was found and overwritten.
fn corrupt_first_content_blob(cache_dir: &Path, bytes: &[u8]) -> bool {
    let blobs = cache_dir.join("get").join("blobs");
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
                    let is_content = (name.ends_with(".zst") && !name.ends_with(".idx.zst"))
                        || name.ends_with(".raw");
                    if is_content {
                        std::fs::write(f.path(), bytes).unwrap();
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[test]
fn get_cache_set_ttl() {
    let wrk = Workdir::new("get_cache_set_ttl");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    // two logical names for the same local file share one entry
    for name in ["a.csv", "b.csv"] {
        let mut g = wrk.command("get");
        g.env("QSV_CACHE_DIR", &cache_dir)
            .args(["--name", name])
            .arg("src.csv");
        wrk.assert_success(&mut g);
    }

    let mut set = wrk.command("get");
    set.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-set-ttl", "a.csv", "--ttl", "12345"]);
    wrk.assert_success(&mut set);

    // TTL is an entry-level property -> both shared aliases reflect the change
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-list", "--json"]);
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        stdout.matches("\"ttl_secs\": 12345").count(),
        2,
        "both aliases of the shared entry should show the new TTL:\n{stdout}"
    );
}

#[test]
fn get_cache_set_policy() {
    let wrk = Workdir::new("get_cache_set_policy");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut g);

    let mut set = wrk.command("get");
    set.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-set-policy", "x.csv", "--refresh", "never"]);
    wrk.assert_success(&mut set);

    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-list", "--json"]);
    let out = wrk.output(&mut list);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"refresh_policy\": \"never\""),
        "refresh policy should be updated to never:\n{stdout}"
    );

    // an unknown name is an error
    let mut bad = wrk.command("get");
    bad.env("QSV_CACHE_DIR", &cache_dir).args([
        "cache-set-policy",
        "nope.csv",
        "--refresh",
        "always",
    ]);
    wrk.assert_err(&mut bad);
}

#[test]
fn get_cache_verify_ok() {
    let wrk = Workdir::new("get_cache_verify_ok");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut g);

    let mut v = wrk.command("get");
    v.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-list", "--verify"]);
    let out = wrk.output(&mut v);
    assert!(
        out.status.success(),
        "verify of a healthy cache should succeed"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("OK") && !stdout.contains("FAIL"),
        "verify should report OK:\n{stdout}"
    );
}

#[test]
fn get_cache_verify_detects_corruption() {
    let wrk = Workdir::new("get_cache_verify_detects_corruption");
    wrk.create_from_string("src.csv", STATES_CSV);
    let cache_dir = wrk.path("qsvcache");

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .arg("src.csv");
    wrk.assert_success(&mut g);

    assert!(
        corrupt_first_content_blob(&cache_dir, b"not a valid zstd blob"),
        "expected a content blob to corrupt"
    );

    let mut v = wrk.command("get");
    v.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-list", "--verify"]);
    let out = wrk.output(&mut v);
    assert!(
        !out.status.success(),
        "verify must exit non-zero when a blob fails its integrity check"
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("FAIL"),
        "verify should report FAIL for the corrupted blob:\n{stdout}"
    );
}

// Build a `get` command that targets the mock S3 endpoint via path-style,
// anonymous (skip_signature) access over plain HTTP.
#[cfg(feature = "get_cloud")]
fn s3_get_cmd(
    wrk: &Workdir,
    cache_dir: &Path,
    endpoint: &str,
    name: &str,
) -> std::process::Command {
    let mut c = wrk.command("get");
    c.env("QSV_CACHE_DIR", cache_dir)
        .args(["--name", name])
        .args(["--cloud-opt", &format!("aws_endpoint={endpoint}")])
        .args(["--cloud-opt", "aws_region=us-east-1"])
        .args(["--cloud-opt", "aws_allow_http=true"])
        .args(["--cloud-opt", "aws_skip_signature=true"])
        .arg("s3://test-bucket/states.csv");
    c
}

#[cfg(feature = "get_cloud")]
#[test]
#[serial]
fn get_s3_fetch_and_dc_read() {
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_fetch_and_dc_read");
    let cache_dir = wrk.path("qsvcache");
    // Plain HTTP to the in-process mock acting as an S3 endpoint (not prod).
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    let mut g = s3_get_cmd(&wrk, &cache_dir, &endpoint, "states.csv");
    wrk.assert_success(&mut g);
    assert_eq!(
        server.body_sends(),
        1,
        "first s3 get should download the body"
    );

    // read the cached object back via the dc: prefix (offline)
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:states.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "4");
}

#[cfg(feature = "get_cloud")]
#[test]
#[serial]
fn get_s3_etag_revalidation() {
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_etag_revalidation");
    let cache_dir = wrk.path("qsvcache");
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    let mut first = s3_get_cmd(&wrk, &cache_dir, &endpoint, "states.csv");
    wrk.assert_success(&mut first);
    assert_eq!(
        server.body_sends(),
        1,
        "first s3 get should download the body"
    );

    // second fetch must revalidate via If-None-Match -> 304, no re-download
    let mut second = s3_get_cmd(&wrk, &cache_dir, &endpoint, "states.csv");
    wrk.assert_success(&mut second);
    assert_eq!(
        server.body_sends(),
        1,
        "second s3 get should revalidate (304), not re-download the body"
    );
}

#[cfg(feature = "get_cloud")]
#[test]
#[serial]
fn get_s3_missing_object_errors_cleanly() {
    // A cloud object that doesn't exist (404 from the store) must fail with a
    // clean CliError (non-zero exit), not a panic. Hitting the live mock with an
    // unregistered key yields a deterministic, fast 404 (no retry storm).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_missing_object_errors_cleanly");
    let cache_dir = wrk.path("qsvcache");
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "x.csv"])
        .args(["--cloud-opt", &format!("aws_endpoint={endpoint}")])
        .args(["--cloud-opt", "aws_region=us-east-1"])
        .args(["--cloud-opt", "aws_allow_http=true"])
        .args(["--cloud-opt", "aws_skip_signature=true"])
        .arg("s3://test-bucket/does-not-exist.csv");
    wrk.assert_err(&mut g);
}

#[cfg(feature = "get_cloud")]
#[test]
#[serial]
fn get_s3_identity_scopes_cache_key() {
    // Regression (roborev #2756): the same s3:// URL against different store
    // identities (here, region) must NOT share a cache entry. Each is fetched
    // independently; a collision would make the second a 304 revalidation that
    // serves the first store's data (body_sends would stay 1).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_identity_scopes_cache_key");
    let cache_dir = wrk.path("qsvcache");
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    let get_region = |name: &str, region: &str| {
        let mut c = wrk.command("get");
        c.env("QSV_CACHE_DIR", &cache_dir)
            .args(["--name", name])
            .args(["--cloud-opt", &format!("aws_endpoint={endpoint}")])
            .args(["--cloud-opt", &format!("aws_region={region}")])
            .args(["--cloud-opt", "aws_allow_http=true"])
            .args(["--cloud-opt", "aws_skip_signature=true"])
            .arg("s3://test-bucket/states.csv");
        c
    };

    let mut a = get_region("us.csv", "us-east-1");
    wrk.assert_success(&mut a);
    let mut b = get_region("eu.csv", "eu-west-1");
    wrk.assert_success(&mut b);

    assert_eq!(
        server.body_sends(),
        2,
        "different store identities must not share a cache entry (no cross-store revalidation)"
    );
}
