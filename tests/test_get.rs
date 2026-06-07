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

// Mock-server request counters: `body_sends` counts 200 responses that carry a
// body; `revalidations` counts 304 (Not Modified) responses. Tracking 304s
// separately lets a test prove a conditional request actually reached the server
// and revalidated — distinct from `resolve_dc_path`'s fallback-to-stale path,
// which makes no successful request yet still serves the cached data.
#[derive(Clone)]
struct Counters {
    body_sends:    Arc<AtomicUsize>,
    revalidations: Arc<AtomicUsize>,
}

// Parse an HTTP `Range: bytes=START-END` header into inclusive byte offsets,
// clamped to `len`. Returns None for an unsatisfiable/unsupported spec (caller
// then serves the full body). Only the single-range `bytes=a-b` / `bytes=a-`
// forms are handled — all that object_store's ranged GETs send.
fn parse_byte_range(header: &str, len: usize) -> Option<(usize, usize)> {
    let spec = header.trim().strip_prefix("bytes=")?;
    let (s, e) = spec.split_once('-')?;
    let start: usize = s.trim().parse().ok()?;
    let end = if e.trim().is_empty() {
        len.saturating_sub(1)
    } else {
        e.trim().parse::<usize>().ok()?.min(len.saturating_sub(1))
    };
    if start > end || start >= len {
        return None;
    }
    Some((start, end))
}

// Serve `body` with single-range support: a `Range` request yields 206 +
// Content-Range (so object_store can drive a multipart/ranged download), any
// other request yields the full 200 body. Both count as a body send.
fn ranged_response(
    body: &[u8],
    etag: &str,
    req: &HttpRequest,
    counters: &Counters,
) -> HttpResponse {
    if let Some(rng) = req.headers().get("range").and_then(|v| v.to_str().ok())
        && let Some((start, end)) = parse_byte_range(rng, body.len())
    {
        counters.body_sends.fetch_add(1, Ordering::SeqCst);
        return HttpResponse::PartialContent()
            .insert_header(("etag", etag.to_string()))
            .insert_header(("accept-ranges", "bytes"))
            .insert_header((
                "content-range",
                format!("bytes {start}-{end}/{}", body.len()),
            ))
            .content_type("text/csv")
            .body(body[start..=end].to_vec());
    }
    counters.body_sends.fetch_add(1, Ordering::SeqCst);
    HttpResponse::Ok()
        .insert_header(("etag", etag.to_string()))
        .content_type("text/csv")
        .body(body.to_vec())
}

// A multi-part-sized CSV body (and its ETag) for exercising the cloud ranged
// download path: ~500 deterministic rows, large enough to span many parts when
// QSV_GET_PART_SIZE is set small. `value` = id*7 so reassembly can be checked.
const BIG_ETAG: &str = "\"big-v1\"";

fn big_csv() -> String {
    let mut s = String::from("id,name,value\n");
    for i in 0..500 {
        s.push_str(&format!("{i},row-{i},{}\n", i * 7));
    }
    s
}

async fn serve_big(c: web::Data<Counters>, req: HttpRequest) -> HttpResponse {
    ranged_response(big_csv().as_bytes(), BIG_ETAG, &req, &c)
}

// A request handler that serves STATES_CSV with an ETag and honors
// conditional GETs: a matching `If-None-Match` yields 304 (counted as a
// revalidation, NOT a body send), so a test can assert that a second
// `qsv get` revalidated rather than re-downloaded.
async fn serve_states(c: web::Data<Counters>, req: HttpRequest) -> HttpResponse {
    if let Some(inm) = req.headers().get("if-none-match")
        && inm.to_str().unwrap_or_default() == ETAG
    {
        c.revalidations.fetch_add(1, Ordering::SeqCst);
        return HttpResponse::NotModified()
            .insert_header(("etag", ETAG))
            .finish();
    }
    // Range-aware (object_store's cloud GETs request a range and require a 206).
    ranged_response(STATES_CSV.as_bytes(), ETAG, &req, &c)
}

// A `states` endpoint that also advertises Cache-Control: max-age. With the
// unified conditional downloader, a re-fetch sends If-None-Match and gets a 304
// (no re-download), so the "served from cache, not re-downloaded" assertions
// still hold via revalidation rather than an RFC9111 fresh hit.
async fn serve_states_fresh(c: web::Data<Counters>, req: HttpRequest) -> HttpResponse {
    if let Some(inm) = req.headers().get("if-none-match")
        && inm.to_str().unwrap_or_default() == ETAG
    {
        c.revalidations.fetch_add(1, Ordering::SeqCst);
        return HttpResponse::NotModified()
            .insert_header(("etag", ETAG))
            .finish();
    }
    ranged_response(STATES_CSV.as_bytes(), ETAG, &req, &c)
}

// A second endpoint with DIFFERENT (1-row) content + its own ETag, for testing
// that revalidating a name repoints it to the fetched entry.
const ONE_ETAG: &str = "\"one-v1\"";
const ONE_CSV: &[u8] = b"name,abbr\nFoo,FO\n";
async fn serve_one_fresh(c: web::Data<Counters>, req: HttpRequest) -> HttpResponse {
    if let Some(inm) = req.headers().get("if-none-match")
        && inm.to_str().unwrap_or_default() == ONE_ETAG
    {
        c.revalidations.fetch_add(1, Ordering::SeqCst);
        return HttpResponse::NotModified()
            .insert_header(("etag", ONE_ETAG))
            .finish();
    }
    ranged_response(ONE_CSV, ONE_ETAG, &req, &c)
}

async fn run_webserver(
    tx: mpsc::Sender<Result<(ServerHandle, SocketAddr), String>>,
    counters: Counters,
) -> std::io::Result<()> {
    let server_builder = HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(counters.clone()))
            .service(web::resource("/states.csv").to(serve_states))
            .service(web::resource("/states_fresh.csv").to(serve_states_fresh))
            .service(web::resource("/one_fresh.csv").to(serve_one_fresh))
            // A larger object for the HTTP ranged/streaming download test.
            .service(web::resource("/big.csv").to(serve_big))
            // Path-style S3 object: object_store issues `GET /{bucket}/{key}`
            // against the endpoint override. Reuses the ETag/304 handler so the
            // cloud path can assert revalidation just like the HTTP path.
            .service(web::resource("/test-bucket/states.csv").to(serve_states));
        // A larger path-style object for the cloud ranged/multipart download
        // test (only built with get_cloud).
        #[cfg(feature = "get_cloud")]
        let app = app.service(web::resource("/test-bucket/big.csv").to(serve_big));
        app
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
    handle:   Option<ServerHandle>,
    addr:     SocketAddr,
    counters: Counters,
}

impl GetWebServer {
    fn start() -> Self {
        let counters = Counters {
            body_sends:    Arc::new(AtomicUsize::new(0)),
            revalidations: Arc::new(AtomicUsize::new(0)),
        };
        let server_counters = counters.clone();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || rt::System::new().block_on(run_webserver(tx, server_counters)));
        match rx.recv_timeout(std::time::Duration::from_secs(10)) {
            Ok(Ok((handle, addr))) => Self {
                handle: Some(handle),
                addr,
                counters,
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

    // Number of 200 responses that carried a body (actual downloads).
    fn body_sends(&self) -> usize {
        self.counters.body_sends.load(Ordering::SeqCst)
    }

    // Number of 304 (Not Modified) responses — i.e. successful conditional
    // revalidations against the server. Only the get_cloud refresh test asserts
    // on this, so it is gated to avoid a dead-code warning in non-cloud builds.
    #[cfg(feature = "get_cloud")]
    fn revalidations(&self) -> usize {
        self.counters.revalidations.load(Ordering::SeqCst)
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
fn get_http_ranged_download() {
    // A large HTTP object with a tiny part size is fetched as many concurrent
    // byte-ranges, then reassembled byte-for-byte. Exercises the unified
    // streaming/ranged downloader on the HTTP path (the former http-cache
    // middleware buffered the whole body).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_http_ranged_download");
    let cache_dir = wrk.path("qsvcache");
    let url = server.url("big.csv");
    let outfile = wrk.path("big_out.csv");

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .env("QSV_GET_PART_SIZE", "64") // tiny parts -> many ranges
        .env("QSV_GET_CONCURRENCY", "4")
        .args(["--name", "big.csv"])
        .args(["--output", outfile.to_str().unwrap()])
        .arg(&url);
    wrk.assert_success(&mut g);

    assert!(
        server.body_sends() > 1,
        "a 64-byte part size should fan the HTTP download out into many ranged GETs, got {}",
        server.body_sends()
    );

    // reassembled bytes match the origin exactly (proves correct in-order assembly)
    let out = std::fs::read_to_string(&outfile).unwrap();
    assert_eq!(
        out,
        big_csv(),
        "reassembled --output bytes must match origin"
    );

    // and it's usable via the dc: prefix
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:big.csv");
    assert_eq!(wrk.stdout::<String>(&mut count), "500");
}

#[test]
#[serial]
fn get_http_entry_has_no_ckan_api_metadata() {
    // Regression (roborev #2760): a plain http:// source must NOT persist a CKAN
    // API URL in its metadata — `ckan_api_url` is for ckan:// entries only, even
    // though `--ckan-api` carries a default value for every invocation.
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_http_entry_has_no_ckan_api_metadata");
    let cache_dir = wrk.path("qsvcache");
    let url = server.url("states.csv");

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "states.csv"])
        .arg(&url);
    wrk.assert_success(&mut g);

    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir)
        .args(["cache-list", "--json"]);
    let out = wrk.output(&mut list);
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"ckan_api_url\": null"),
        "plain http entry must record a null ckan_api_url:\n{stdout}"
    );
    assert!(
        !stdout.contains("data.dathere.com"),
        "plain http entry must not persist the default CKAN API URL:\n{stdout}"
    );
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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

    // cache-list shows both ORIGINAL names (read from the alias file content,
    // which stores the logical name; the alias filename itself is a BLAKE3 hash)
    let mut list = wrk.command("get");
    list.env("QSV_CACHE_DIR", &cache_dir).arg("cache-list");
    let out = wrk.output(&mut list);
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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
    assert!(
        out.status.success(),
        "cache-list exited non-zero:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
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

// Regression: commands that route inputs through `util::process_input` (cat,
// slice, join, …) must honor the `dc:` prefix rather than reject it as a missing
// file — previously only `Config::new` resolved `dc:`, so process_input's raw
// `path.exists()` check failed for "dc:…". Uses a local get (no network) to
// isolate the dc: read path.
#[test]
#[serial]
fn get_dc_works_with_process_input_commands() {
    let wrk = Workdir::new("get_dc_works_with_process_input_commands");
    let cache_dir = wrk.path("qsvcache");
    let src = wrk.path("src.csv");
    std::fs::write(&src, "id,name\n0,a\n1,b\n2,c\n").unwrap();

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "src.csv"])
        .arg(src.to_str().unwrap());
    wrk.assert_success(&mut g);

    // `cat` routes inputs through process_input -> must echo the dc: content
    let mut cat = wrk.command("cat");
    cat.env("QSV_CACHE_DIR", &cache_dir)
        .args(["rows", "dc:src.csv"]);
    assert_eq!(wrk.stdout::<String>(&mut cat), "id,name\n0,a\n1,b\n2,c");

    // `slice` likewise (and uses the materialized sibling .idx for --index)
    let mut slice = wrk.command("slice");
    slice
        .env("QSV_CACHE_DIR", &cache_dir)
        .args(["--index", "1"])
        .arg("dc:src.csv");
    assert_eq!(wrk.stdout::<String>(&mut slice), "id,name\n1,b");

    // `count` (which already resolved dc: via Config::new) still works
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:src.csv");
    assert_eq!(wrk.stdout::<String>(&mut count), "3");
}

// Recursively check whether `dir` contains any file whose name ends with `suffix`.
fn dir_has_file_suffix(dir: &Path, suffix: &str) -> bool {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            if dir_has_file_suffix(&p, suffix) {
                return true;
            }
        } else if p
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(suffix))
        {
            return true;
        }
    }
    false
}

// Regression + feature (get §2.2): commands that use util::get_stats_records
// (frequency, schema, ...) must (a) work on dc: inputs at all — they previously
// errored because the raw "dc:" string failed canonicalize — and (b) have the
// .stats.csv.data.jsonl sidecar they build captured into a durable, content-
// addressed blob so it survives temp-dir cleanup.
#[test]
#[serial]
fn get_dc_stats_cache_for_smart_commands() {
    let wrk = Workdir::new("get_dc_stats_cache_for_smart_commands");
    let cache_dir = wrk.path("qsvcache");
    let src = wrk.path("cats.csv");
    std::fs::write(&src, "id,cat\n1,a\n2,b\n3,a\n4,c\n5,a\n6,b\n").unwrap();

    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .args(["--name", "cats.csv"])
        .arg(src.to_str().unwrap());
    wrk.assert_success(&mut g);

    // (a) frequency works on dc: and is cardinality-aware (the per-value counts
    //     come from the stats-cache path that get_stats_records drives).
    let mut freq = wrk.command("frequency");
    freq.env("QSV_CACHE_DIR", &cache_dir).arg("dc:cats.csv");
    let got = wrk.stdout::<String>(&mut freq);
    assert!(
        got.contains("cat,a,3"),
        "frequency dc: should count cat=a 3x, got:\n{got}"
    );

    // (b) schema is another get_stats_records consumer; it must succeed too.
    let mut schema = wrk.command("schema");
    schema.env("QSV_CACHE_DIR", &cache_dir).arg("dc:cats.csv");
    wrk.assert_success(&mut schema);

    // (c) the stats-cache sidecar was captured into a durable blob under the
    //     entry's content-addressed blob dir (cache root has a `get` subdir).
    let blobs = cache_dir.join("get").join("blobs");
    assert!(
        dir_has_file_suffix(&blobs, ".stats.jsonl.zst"),
        "expected a captured .stats.jsonl.zst blob under {}",
        blobs.display()
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
fn get_s3_multipart_ranged_download() {
    // Force a tiny part size so the ~8KB object is fetched as many concurrent
    // byte-ranges, then assert the blob reassembled in the correct order
    // (boundary + middle rows exact) and that it actually fanned out (>1 GET).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_multipart_ranged_download");
    let cache_dir = wrk.path("qsvcache");
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    let outfile = wrk.path("big_out.csv");
    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .env("QSV_GET_PART_SIZE", "64") // tiny parts -> many ranges
        .env("QSV_GET_CONCURRENCY", "4")
        .args(["--name", "big.csv"])
        .args(["--cloud-opt", &format!("aws_endpoint={endpoint}")])
        .args(["--cloud-opt", "aws_region=us-east-1"])
        .args(["--cloud-opt", "aws_allow_http=true"])
        .args(["--cloud-opt", "aws_skip_signature=true"])
        .args(["--output", outfile.to_str().unwrap()])
        .arg("s3://test-bucket/big.csv");
    wrk.assert_success(&mut g);

    assert!(
        server.body_sends() > 1,
        "a 64-byte part size should fan the download out into many ranged GETs, got {}",
        server.body_sends()
    );

    // The fetched (decompressed) bytes must match the origin byte-for-byte: this
    // proves the concurrently-fetched parts were streamed back into the blob in
    // the correct order (a scrambled reassembly would diverge here).
    let out = std::fs::read_to_string(&outfile).unwrap();
    assert_eq!(
        out,
        big_csv(),
        "reassembled --output bytes must match origin"
    );

    // and it's usable via the dc: prefix
    let mut count = wrk.command("count");
    count.env("QSV_CACHE_DIR", &cache_dir).arg("dc:big.csv");
    assert_eq!(wrk.stdout::<String>(&mut count), "500");
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

#[cfg(feature = "get_cloud")]
#[test]
#[serial]
fn get_s3_env_cloud_opt_override_refresh_stable() {
    // Regression (roborev #2757): when the same key comes from both the
    // environment and --cloud-opt, the persisted identity must collapse to the
    // effective (last-wins) value, matching the store object_store builds. A
    // stale dc: refresh — even under a CHANGED ambient env var — must then
    // resolve to the SAME cache entry (revalidate, not re-download).
    let server = GetWebServer::start();
    let wrk = Workdir::new("get_s3_env_cloud_opt_override_refresh_stable");
    let cache_dir = wrk.path("qsvcache");
    let endpoint = format!("http://{}", server.addr); // DevSkim: ignore DS137138

    // env says us-east-1, --cloud-opt overrides to eu-west-1 (the effective
    // store). --ttl 0 makes the entry immediately stale so the dc: read refreshes.
    let mut g = wrk.command("get");
    g.env("QSV_CACHE_DIR", &cache_dir)
        .env("AWS_REGION", "us-east-1")
        .args(["--name", "r.csv", "--ttl", "0"])
        .args(["--cloud-opt", &format!("aws_endpoint={endpoint}")])
        .args(["--cloud-opt", "aws_region=eu-west-1"])
        .args(["--cloud-opt", "aws_allow_http=true"])
        .args(["--cloud-opt", "aws_skip_signature=true"])
        .arg("s3://test-bucket/states.csv");
    wrk.assert_success(&mut g);
    assert_eq!(server.body_sends(), 1, "initial fetch downloads once");

    // stale dc: read with a DIFFERENT ambient region; the persisted identity
    // (replayed as high-precedence --cloud-opt) must win, keeping the same key.
    let mut count = wrk.command("count");
    count
        .env("QSV_CACHE_DIR", &cache_dir)
        .env("AWS_REGION", "ap-south-1")
        .arg("dc:r.csv");
    let got: String = wrk.stdout(&mut count);
    assert_eq!(got, "4", "dc:r.csv should resolve to the cached data");

    // The refresh must have made a *successful conditional request* (one 304).
    // This is what proves the identity replay rebuilt the right store and hit the
    // same entry — `resolve_dc_path` silently falls back to the stale copy on
    // refresh failure, which would still yield count 4 and body_sends 1 but make
    // zero successful requests.
    assert_eq!(
        server.revalidations(),
        1,
        "stale dc: refresh must issue exactly one conditional (304) revalidation request"
    );
    assert_eq!(
        server.body_sends(),
        1,
        "stale refresh must revalidate the SAME entry (304), not re-download under a changed env \
         var"
    );
}
