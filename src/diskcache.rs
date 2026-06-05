//! Shared disk-cache subsystem for qsv.
//!
//! This module has two tiers:
//!
//! 1. An **always-compiled** core (this top section) with the source-resolution and fetch
//!    primitives shared by `lookup.rs` (the lookup-table module used by `luau`, `validate`,
//!    `template` and `describegpt`) and the `get` command. Keeping the `ckan://` / `dathere://`
//!    resolution — including the security-critical same-origin AUTHORIZATION-token-stripping check
//!    — in one place means there is a single, audited code path rather than two.
//!
//! 2. A **`get`-feature-gated** rich cache (see the bottom of this file): a content-addressed,
//!    zstd-compressed blob store with a per-entry metadata index (BLAKE3, ETag, sizes, record
//!    count, TTL), HTTP cache semantics via `http-cache-reqwest`, auto-indexing, and `dc:` prefix
//!    resolution. This tier pulls heavier dependencies (`http-cache-reqwest`, `zstd`) so it is only
//!    compiled when the `get` feature is enabled (keeping `qsvlite` lean).

use std::{fs, path::Path, time::SystemTime};

use reqwest::blocking::{Client, Response};
use serde_json::Value;
use util::expand_tilde;

use crate::{CliError, util};

/// Default CKAN Action API base URL (datHere's public CKAN).
pub const DEFAULT_CKAN_API: &str = "https://data.dathere.com/api/3/action";

/// Resolve, create if needed, and canonicalize the qsv cache directory.
///
/// Honors the `QSV_CACHE_DIR` environment variable (which overrides
/// `cache_dir`), expands a leading `~`, and creates the directory if absent.
pub fn set_qsv_cache_dir(cache_dir: &str) -> Result<String, CliError> {
    let qsv_cache_dir = if let Ok(cache_path) = std::env::var("QSV_CACHE_DIR") {
        // if QSV_CACHE_DIR env var is set, check if it exists. If it doesn't, create it.
        if cache_path.starts_with('~') {
            // expand the tilde
            let expanded_dir = expand_tilde(&cache_path).unwrap();
            expanded_dir.to_string_lossy().to_string()
        } else {
            cache_path
        }
    } else if cache_dir.starts_with('~') {
        // expand the tilde
        let expanded_dir = expand_tilde(cache_dir).unwrap();
        expanded_dir.to_string_lossy().to_string()
    } else {
        cache_dir.to_string()
    };
    if !Path::new(&qsv_cache_dir).exists() {
        fs::create_dir_all(&qsv_cache_dir)?;
    }
    Ok(qsv_cache_dir)
}

/// The outcome of resolving a source URI's scheme prefix.
pub struct ResolvedUri {
    /// The http(s) URL to fetch (after `dathere://` / `ckan://` expansion), or
    /// the original URI for local files / already-http(s) inputs.
    pub url:                  String,
    /// True if this is a CKAN resource that still needs resource_show /
    /// resource_search resolution to obtain the actual data URL.
    pub is_ckan:              bool,
    /// True if the CKAN form was `ckan://<name>?` (resource_search) rather than
    /// `ckan://<id>` (resource_show).
    pub ckan_resource_search: bool,
}

/// Expand a source URI's scheme prefix into an http(s) URL plus CKAN flags.
///
/// - `dathere://<path>` → datHere's `qsv-lookup-tables` GitHub raw URL.
/// - `ckan://<id>`      → `<ckan_api>/resource_show?id=<id>`   (is_ckan=true).
/// - `ckan://<name>?`   → `<ckan_api>/resource_search?query=name:<name>` (is_ckan=true,
///   ckan_resource_search=true).
/// - anything else (local path, http(s)://) is returned unchanged.
pub fn resolve_uri_prefix(uri: &str, ckan_api_url: Option<&str>) -> ResolvedUri {
    if let Some(rest) = uri.strip_prefix("dathere://") {
        return ResolvedUri {
            url:                  format!(
                "https://raw.githubusercontent.com/dathere/qsv-lookup-tables/main/lookup-tables/{rest}"
            ),
            is_ckan:              false,
            ckan_resource_search: false,
        };
    }

    if let Some(rest) = uri.strip_prefix("ckan://") {
        let rest = rest.trim();
        let api = ckan_api_url.unwrap_or(DEFAULT_CKAN_API);
        if let Some(name) = rest.strip_suffix('?') {
            return ResolvedUri {
                url:                  format!("{api}/resource_search?query=name:{name}"),
                is_ckan:              true,
                ckan_resource_search: true,
            };
        }
        return ResolvedUri {
            url:                  format!("{api}/resource_show?id={rest}"),
            is_ckan:              true,
            ckan_resource_search: false,
        };
    }

    ResolvedUri {
        url:                  uri.to_string(),
        is_ckan:              false,
        ckan_resource_search: false,
    }
}

/// A CKAN resource resolved to its actual data URL.
pub struct CkanResource {
    /// The URL of the actual resource data (e.g. the CSV file).
    pub data_url:      String,
    /// Whether the CKAN bearer token may be sent when fetching `data_url`
    /// (true only when the resource is same-origin as the CKAN API).
    pub send_auth:     bool,
    /// The CKAN-reported resource hash, if present (used for staleness checks).
    pub resource_hash: Option<String>,
}

/// Resolve a CKAN `resource_show` / `resource_search` URL to the actual data URL.
///
/// For `resource_search=true`, `url` is a `resource_search?query=name:<name>`
/// endpoint; the first result's id is resolved to a `resource_show`. For
/// `resource_search=false`, `url` is a `resource_show?id=<id>` endpoint directly.
///
/// `resource_show` returns JSON with the actual data URL inside `result.url`.
/// The CKAN bearer token is sent on the metadata calls, but is **stripped**
/// before fetching the data URL when that URL is on a different origin than the
/// CKAN API — CKAN admins can register external resource URLs, and the token
/// must not leak to third-party hosts. Fail-secure: any parse failure or missing
/// host is treated as cross-origin. Origin comparison covers scheme + host +
/// port (RFC 6454).
pub fn resolve_ckan_resource(
    client: &Client,
    url: &str,
    resource_search: bool,
    ckan_api_url: Option<&str>,
    ckan_token: Option<&str>,
) -> Result<CkanResource, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(token) = ckan_token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(token)?,
        );
    }

    let resource_show_uri: String = if resource_search {
        let resource_search_result = client
            .get(url)
            .headers(headers.clone())
            .send()?
            .error_for_status()?
            .text()?;
        let resource_search_json: Value = serde_json::from_str(&resource_search_result)?;
        let resource_id = resource_search_json["result"]["results"][0]["id"]
            .as_str()
            .ok_or("Cannot find resource name")?;
        format!(
            "{}/resource_show?id={}",
            ckan_api_url.unwrap_or(DEFAULT_CKAN_API),
            resource_id
        )
    } else {
        url.to_string()
    };

    let resource_show_result = client
        .get(&resource_show_uri)
        .headers(headers.clone())
        .send()?
        .error_for_status()?
        .text()?;
    let resource_show_json: Value = serde_json::from_str(&resource_show_result)?;

    let data_url = resource_show_json["result"]["url"]
        .as_str()
        .ok_or("Cannot get resource URL from resource_show JSON response")?
        .to_string();

    // CKAN exposes a per-resource content hash; capture it when present (empty
    // strings are treated as absent).
    let resource_hash = resource_show_json["result"]["hash"]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);

    // Same-origin check: only keep the auth token if the data URL is on the
    // same origin as the CKAN API.
    let ckan_url_parsed = ckan_api_url.and_then(|u| reqwest::Url::parse(u).ok());
    let resource_url_parsed = reqwest::Url::parse(&data_url).ok();
    let send_auth = match (ckan_url_parsed.as_ref(), resource_url_parsed.as_ref()) {
        (Some(a), Some(b)) => {
            a.host_str().is_some()
                && a.host_str() == b.host_str()
                && a.scheme() == b.scheme()
                && a.port_or_known_default() == b.port_or_known_default()
        },
        _ => false,
    };

    Ok(CkanResource {
        data_url,
        send_auth: send_auth && ckan_token.is_some(),
        resource_hash,
    })
}

/// Perform a blocking GET with an optional `If-Modified-Since` conditional
/// header (RFC 7231 §7.1.1.1 IMF-fixdate format) and optional bearer token.
pub fn http_get_conditional(
    client: &Client,
    url: &str,
    if_modified_since: Option<SystemTime>,
    auth_token: Option<&str>,
) -> Result<Response, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(modified) = if_modified_since {
        let last_modified: chrono::DateTime<chrono::Utc> = modified.into();
        // chrono's to_rfc2822() emits "+0000" instead of "GMT", which strict
        // origins/CDNs may reject and re-serve the full body.
        let ims = last_modified
            .format("%a, %d %b %Y %H:%M:%S GMT")
            .to_string();
        headers.insert(
            reqwest::header::IF_MODIFIED_SINCE,
            reqwest::header::HeaderValue::from_str(&ims)?,
        );
    }

    if let Some(token) = auth_token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(token)?,
        );
    }

    client.get(url).headers(headers).send().map_err(Into::into)
}

// ============================================================================
// get-feature-gated rich cache
// ============================================================================
#[cfg(feature = "get")]
pub use rich::*;

#[cfg(feature = "get")]
mod rich {
    use std::{
        fs,
        io::Read,
        path::{Path, PathBuf},
        sync::{Arc, Mutex},
        time::SystemTime,
    };

    use http_cache::{CacheManager, CacheMode, HttpResponse};
    use http_cache_reqwest::{Cache, HttpCache, HttpCacheOptions};
    use http_cache_semantics::CachePolicy;
    use reqwest_middleware::ClientBuilder;
    use serde::{Deserialize, Serialize};

    use super::{DEFAULT_CKAN_API, resolve_ckan_resource, resolve_uri_prefix, set_qsv_cache_dir};
    use crate::{CliError, CliResult, config::Config, util};

    /// zstd compression level for cached blobs (good speed/ratio for tabular text).
    const ZSTD_LEVEL: i32 = 3;
    /// Default qsv cache dir (used by `dc:` resolution when none is otherwise set).
    const DEFAULT_CACHE_DIR: &str = "~/.qsv-cache";

    /// How a cache entry decides when to revalidate against its source.
    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
    #[serde(rename_all = "kebab-case")]
    pub enum RefreshPolicy {
        /// Revalidate only when the entry is older than its TTL (the default).
        #[default]
        OnStale,
        /// Always re-fetch.
        Always,
        /// Never revalidate; serve the cached copy regardless of age.
        Never,
    }

    impl RefreshPolicy {
        /// Parse a `--refresh` flag value (`on-stale` | `always` | `never`).
        pub fn parse(s: &str) -> CliResult<Self> {
            match s.to_ascii_lowercase().as_str() {
                "on-stale" | "onstale" | "stale" => Ok(Self::OnStale),
                "always" => Ok(Self::Always),
                "never" => Ok(Self::Never),
                other => Err(CliError::Other(format!(
                    "invalid --refresh policy '{other}' (expected on-stale, always or never)"
                ))),
            }
        }
    }

    /// Transparent on-disk compression for cached blobs.
    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
    #[serde(rename_all = "lowercase")]
    pub enum Compression {
        /// No compression.
        None,
        /// zstd-compressed (the default).
        #[default]
        Zstd,
    }

    impl Compression {
        /// Parse a `--compress` flag value (`zstd` | `none`).
        pub fn parse(s: &str) -> CliResult<Self> {
            match s.to_ascii_lowercase().as_str() {
                "zstd" | "zst" => Ok(Self::Zstd),
                "none" | "off" | "no" => Ok(Self::None),
                other => Err(CliError::Other(format!(
                    "invalid --compress algo '{other}' (expected zstd or none)"
                ))),
            }
        }

        fn ext(self) -> &'static str {
            match self {
                Self::Zstd => "zst",
                Self::None => "raw",
            }
        }
    }

    /// Per-entry metadata recorded in the cache index, surfaced by `cache-list`.
    #[derive(Serialize, Deserialize, Clone)]
    pub struct CacheEntry {
        /// The `dc:` handle / alias for this entry.
        pub logical_name:       String,
        /// The http-cache key (or a synthetic key for non-HTTP sources). Internal.
        pub cache_key:          String,
        /// The original source as given to `get` (http(s)://, ckan://, file path, …).
        pub source_uri:         String,
        /// The final URL/path actually fetched after prefix/CKAN resolution.
        pub resolved_uri:       String,
        /// BLAKE3 hex of the uncompressed bytes (== blob filename stem).
        pub blake3:             String,
        /// HTTP ETag, if the origin provided one.
        pub etag:               Option<String>,
        /// HTTP Last-Modified, if the origin provided one.
        pub last_modified:      Option<String>,
        /// CKAN resource hash, when the source was `ckan://`.
        pub ckan_resource_hash: Option<String>,
        /// On-disk (compressed) blob size in bytes.
        pub size_compressed:    u64,
        /// Original (uncompressed) size in bytes.
        pub size_uncompressed:  u64,
        /// Exact record count from the auto-built index (None if not indexed).
        pub record_count:       Option<u64>,
        /// Whether a `{blake3}.idx.zst` index blob is stored.
        pub indexed:            bool,
        /// Unix seconds when the entry was last (re)fetched.
        pub downloaded_at:      i64,
        /// Per-entry TTL in seconds. -1 = never expire.
        pub ttl_secs:           i64,
        /// Refresh policy for `dc:` staleness handling.
        pub refresh_policy:     RefreshPolicy,
        /// On-disk compression for the blob.
        pub compression:        Compression,
    }

    /// The on-disk record: metadata plus (for HTTP sources) the data needed to
    /// reconstruct an `http-cache` response for revalidation.
    #[derive(Serialize, Deserialize)]
    struct StoredEntry {
        meta: CacheEntry,
        http: Option<HttpStored>,
    }

    #[derive(Serialize, Deserialize)]
    struct HttpStored {
        /// The cached HTTP response with its body emptied (body lives in the blob).
        response: HttpResponse,
        policy:   CachePolicy,
    }

    /// Options for fetching a resource into the cache.
    pub struct GetOptions {
        pub source:         String,
        pub name:           Option<String>,
        /// Already-resolved cache directory (see `set_qsv_cache_dir`).
        pub cache_dir:      String,
        pub ttl_secs:       i64,
        pub refresh_policy: RefreshPolicy,
        pub compression:    Compression,
        pub force:          bool,
        pub ckan_api_url:   Option<String>,
        pub ckan_token:     Option<String>,
        pub timeout_secs:   u16,
    }

    type BoxError = http_cache::BoxError;

    fn box_err<E: std::fmt::Display>(e: E) -> BoxError {
        Box::new(std::io::Error::other(e.to_string()))
    }

    fn unix_now() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or_default()
    }

    fn keyhash(cache_key: &str) -> String {
        blake3::hash(cache_key.as_bytes()).to_hex().to_string()
    }

    /// Sanitize a logical name into a safe single-path-segment filename while
    /// preserving its extension (so `dc:` temp files keep delimiter detection).
    fn safe_name(name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    fn get_root(cache_dir: &str) -> PathBuf {
        Path::new(cache_dir).join("get")
    }

    fn blob_dir(root: &Path, b3: &str) -> PathBuf {
        root.join("blobs").join(&b3[0..2]).join(&b3[2..4])
    }

    fn blob_path(root: &Path, b3: &str, compression: Compression) -> PathBuf {
        blob_dir(root, b3).join(format!("{b3}.{}", compression.ext()))
    }

    fn idx_blob_path(root: &Path, b3: &str) -> PathBuf {
        blob_dir(root, b3).join(format!("{b3}.idx.zst"))
    }

    fn entry_path(root: &Path, keyhash: &str) -> PathBuf {
        root.join("entries").join(format!("{keyhash}.json"))
    }

    fn alias_path(root: &Path, name: &str) -> PathBuf {
        root.join("aliases").join(safe_name(name))
    }

    /// Atomically write `bytes` to `path` (write to a temp sibling, then rename).
    fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension(format!(
            "tmp-{}",
            &blake3::hash(path.to_string_lossy().as_bytes()).to_hex()[..8]
        ));
        fs::write(&tmp, bytes)?;
        fs::rename(&tmp, path)?;
        Ok(())
    }

    fn read_zst(path: &Path) -> std::io::Result<Vec<u8>> {
        zstd::decode_all(fs::File::open(path)?)
    }

    /// Store `body` as a content-addressed (possibly compressed) blob.
    /// Returns (blake3, compressed_size, uncompressed_size).
    fn store_blob(
        root: &Path,
        body: &[u8],
        compression: Compression,
    ) -> std::io::Result<(String, u64, u64)> {
        let b3 = blake3::hash(body).to_hex().to_string();
        let path = blob_path(root, &b3, compression);
        let bytes = match compression {
            Compression::Zstd => zstd::encode_all(body, ZSTD_LEVEL)?,
            Compression::None => body.to_vec(),
        };
        let size_compressed = bytes.len() as u64;
        atomic_write(&path, &bytes)?;
        Ok((b3, size_compressed, body.len() as u64))
    }

    fn read_blob(root: &Path, b3: &str, compression: Compression) -> std::io::Result<Vec<u8>> {
        let path = blob_path(root, b3, compression);
        match compression {
            Compression::Zstd => read_zst(&path),
            Compression::None => {
                let mut v = Vec::new();
                fs::File::open(path)?.read_to_end(&mut v)?;
                Ok(v)
            },
        }
    }

    fn write_entry(root: &Path, entry: &StoredEntry) -> CliResult<()> {
        let kh = keyhash(&entry.meta.cache_key);
        let json = serde_json::to_vec(entry)
            .map_err(|e| CliError::Other(format!("get: failed to serialize cache entry: {e}")))?;
        // Write the new entry JSON first so it counts toward the blob refcount
        // during the orphan cleanup below (content-addressed dedup means the new
        // and old entries may share a blob).
        atomic_write(&entry_path(root, &kh), &json)?;

        // If this logical name previously pointed to a *different* entry, that
        // old entry is now orphaned (reachable only via cache-list/prune/clear)
        // and would duplicate the name with stale metadata. Remove it so logical
        // names stay unique.
        let ap = alias_path(root, &entry.meta.logical_name);
        if let Ok(old_kh) = fs::read_to_string(&ap) {
            let old_kh = old_kh.trim();
            if old_kh != kh
                && let Ok(old) = load_entry_at(&entry_path(root, old_kh))
            {
                delete_entry_files(root, &old.meta);
            }
        }

        atomic_write(&ap, kh.as_bytes())?;
        Ok(())
    }

    fn load_entry_at(path: &Path) -> CliResult<StoredEntry> {
        let bytes = fs::read(path)?;
        serde_json::from_slice(&bytes).map_err(|e| {
            CliError::Other(format!("get: corrupt cache entry {}: {e}", path.display()))
        })
    }

    fn load_entry_by_name(root: &Path, name: &str) -> CliResult<Option<StoredEntry>> {
        let ap = alias_path(root, name);
        if !ap.exists() {
            return Ok(None);
        }
        let kh = fs::read_to_string(&ap)?.trim().to_string();
        let ep = entry_path(root, &kh);
        if !ep.exists() {
            return Ok(None);
        }
        Ok(Some(load_entry_at(&ep)?))
    }

    fn delete_entry_files(root: &Path, entry: &CacheEntry) {
        let _ = fs::remove_file(entry_path(root, &keyhash(&entry.cache_key)));
        let _ = fs::remove_file(alias_path(root, &entry.logical_name));
        // Only remove the blob/idx if no other entry references the same content.
        if !blob_referenced(root, &entry.blake3, &entry.cache_key) {
            let _ = fs::remove_file(blob_path(root, &entry.blake3, entry.compression));
            let _ = fs::remove_file(idx_blob_path(root, &entry.blake3));
        }
    }

    /// True if any *other* entry references the given blob (content-addressed
    /// dedup means a blob may be shared by multiple logical names).
    fn blob_referenced(root: &Path, b3: &str, except_cache_key: &str) -> bool {
        let entries_dir = root.join("entries");
        let Ok(rd) = fs::read_dir(&entries_dir) else {
            return false;
        };
        for de in rd.flatten() {
            if let Ok(e) = load_entry_at(&de.path())
                && e.meta.blake3 == b3
                && e.meta.cache_key != except_cache_key
            {
                return true;
            }
        }
        false
    }

    fn delete_entry_by_name(root: &Path, name: &str) -> CliResult<bool> {
        if let Some(entry) = load_entry_by_name(root, name)? {
            delete_entry_files(root, &entry.meta);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn header_str(
        headers: &reqwest::header::HeaderMap,
        name: reqwest::header::HeaderName,
    ) -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(ToString::to_string)
    }

    fn derive_name(url: &str) -> String {
        let trimmed = url.split(['?', '#']).next().unwrap_or(url);
        let seg = trimmed.rsplit('/').find(|s| !s.is_empty()).unwrap_or(url);
        if seg.is_empty() {
            "cached.csv".to_string()
        } else {
            seg.to_string()
        }
    }

    /// Build (and cache) the qsv index for an entry's blob, recording the exact
    /// record count. No-op if already indexed.
    fn ensure_indexed(root: &Path, entry: &mut StoredEntry) -> CliResult<()> {
        if entry.meta.indexed {
            return Ok(());
        }
        let body = read_blob(root, &entry.meta.blake3, entry.meta.compression)?;

        let tmp_dir = std::env::temp_dir().join("qsv-getidx");
        fs::create_dir_all(&tmp_dir)?;
        let tmp_csv = tmp_dir.join(format!("{}.csv", &entry.meta.blake3[..16]));
        fs::write(&tmp_csv, &body)?;

        let tmp_csv_str = tmp_csv.to_string_lossy().to_string();
        let rconf = Config::new(Some(&tmp_csv_str)).no_headers(false);

        // Auto-indexing is best-effort: a non-CSV blob (or one that fails to
        // index) is still cached and usable, just without random access.
        let record_count = match util::create_index_for_file(&tmp_csv, &rconf) {
            Ok(()) => {
                let count = util::count_rows(&rconf).unwrap_or_default();
                let idx_src = util::idx_path(&tmp_csv);
                if let Ok(idx_bytes) = fs::read(&idx_src) {
                    let idx_zst = zstd::encode_all(&idx_bytes[..], ZSTD_LEVEL)?;
                    atomic_write(&idx_blob_path(root, &entry.meta.blake3), &idx_zst)?;
                    entry.meta.indexed = true;
                }
                let _ = fs::remove_file(&idx_src);
                Some(count)
            },
            Err(e) => {
                log::warn!("get: could not auto-index {}: {e}", entry.meta.logical_name);
                None
            },
        };
        let _ = fs::remove_file(&tmp_csv);

        entry.meta.record_count = record_count;
        write_entry(root, entry)?;
        Ok(())
    }

    /// The custom `http-cache` manager: persists response bodies into the
    /// content-addressed zstd blob store and metadata into the JSON entry index.
    #[derive(Clone)]
    struct QsvCacheManager {
        root:               PathBuf,
        source_uri:         String,
        logical_name:       String,
        ttl_secs:           i64,
        refresh_policy:     RefreshPolicy,
        compression:        Compression,
        ckan_resource_hash: Option<String>,
        // The cache key the middleware last operated on (in get OR put), shared
        // with `get_resource` so it can recover the entry on a fresh cache hit
        // (where `put` is never called and no alias for the requested name is
        // created).
        observed_key:       Arc<Mutex<Option<String>>>,
    }

    impl CacheManager for QsvCacheManager {
        async fn get(
            &self,
            cache_key: &str,
        ) -> http_cache::Result<Option<(HttpResponse, CachePolicy)>> {
            if let Ok(mut g) = self.observed_key.lock() {
                *g = Some(cache_key.to_string());
            }
            let ep = entry_path(&self.root, &keyhash(cache_key));
            if !ep.exists() {
                return Ok(None);
            }
            let entry = load_entry_at(&ep).map_err(box_err)?;
            let Some(httpstored) = entry.http else {
                return Ok(None);
            };
            let body = read_blob(&self.root, &entry.meta.blake3, entry.meta.compression)
                .map_err(box_err)?;
            let mut response = httpstored.response;
            response.body = body;
            Ok(Some((response, httpstored.policy)))
        }

        async fn put(
            &self,
            cache_key: String,
            res: HttpResponse,
            policy: CachePolicy,
        ) -> http_cache::Result<HttpResponse> {
            if let Ok(mut g) = self.observed_key.lock() {
                *g = Some(cache_key.clone());
            }
            let (b3, size_compressed, size_uncompressed) =
                store_blob(&self.root, &res.body, self.compression).map_err(box_err)?;

            let parts = res.parts().map_err(box_err)?;
            let etag = header_str(&parts.headers, reqwest::header::ETAG);
            let last_modified = header_str(&parts.headers, reqwest::header::LAST_MODIFIED);

            let meta = CacheEntry {
                logical_name: self.logical_name.clone(),
                cache_key,
                source_uri: self.source_uri.clone(),
                resolved_uri: res.url.to_string(),
                blake3: b3,
                etag,
                last_modified,
                ckan_resource_hash: self.ckan_resource_hash.clone(),
                size_compressed,
                size_uncompressed,
                record_count: None,
                indexed: false,
                downloaded_at: unix_now(),
                ttl_secs: self.ttl_secs,
                refresh_policy: self.refresh_policy,
                compression: self.compression,
            };

            let mut stored_response = res.clone();
            stored_response.body = Vec::new();
            let entry = StoredEntry {
                meta,
                http: Some(HttpStored {
                    response: stored_response,
                    policy,
                }),
            };
            write_entry(&self.root, &entry).map_err(box_err)?;
            Ok(res)
        }

        async fn delete(&self, cache_key: &str) -> http_cache::Result<()> {
            let ep = entry_path(&self.root, &keyhash(cache_key));
            if let Ok(entry) = load_entry_at(&ep) {
                delete_entry_files(&self.root, &entry.meta);
            }
            Ok(())
        }
    }

    fn ingest_local(opts: &GetOptions, root: &Path, path: &Path) -> CliResult<CacheEntry> {
        let body = fs::read(path)?;
        let (b3, size_compressed, size_uncompressed) = store_blob(root, &body, opts.compression)?;
        let abs = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let name = opts
            .name
            .clone()
            .unwrap_or_else(|| derive_name(&abs.to_string_lossy()));
        let last_modified = fs::metadata(path)
            .and_then(|m| m.modified())
            .ok()
            .map(|mt| {
                let dt: chrono::DateTime<chrono::Utc> = mt.into();
                dt.to_rfc2822()
            });
        let meta = CacheEntry {
            logical_name: name,
            cache_key: format!("FILE:{}", abs.display()),
            // Store the canonicalized absolute path as the source so a later
            // stale `dc:` refresh re-reads the right file regardless of the
            // working directory it runs from (the originally-given path may be
            // relative).
            source_uri: abs.to_string_lossy().to_string(),
            resolved_uri: abs.to_string_lossy().to_string(),
            blake3: b3,
            etag: None,
            last_modified,
            ckan_resource_hash: None,
            size_compressed,
            size_uncompressed,
            record_count: None,
            indexed: false,
            downloaded_at: unix_now(),
            ttl_secs: opts.ttl_secs,
            refresh_policy: opts.refresh_policy,
            compression: opts.compression,
        };
        let mut entry = StoredEntry { meta, http: None };
        write_entry(root, &entry)?;
        ensure_indexed(root, &mut entry)?;
        Ok(entry.meta)
    }

    /// Fetch (or revalidate) `opts.source` into the cache, returning its metadata.
    pub fn get_resource(opts: &GetOptions) -> CliResult<CacheEntry> {
        let root = get_root(&opts.cache_dir);
        fs::create_dir_all(root.join("entries"))?;
        fs::create_dir_all(root.join("aliases"))?;
        fs::create_dir_all(root.join("blobs"))?;

        let resolved = resolve_uri_prefix(&opts.source, opts.ckan_api_url.as_deref());
        let is_http = resolved.url.to_ascii_lowercase().starts_with("http");

        if !is_http {
            let local_path = Path::new(&opts.source);
            if local_path.exists() {
                return ingest_local(opts, &root, local_path);
            }
            return Err(CliError::Other(format!(
                "get: unsupported source '{}'. Supported: local file, http(s)://, dathere://, \
                 ckan://. (s3/az/gs cloud storage and sftp are planned for a later release.)",
                opts.source
            )));
        }

        // Resolve CKAN resources to their actual data URL (and auth decision).
        let blocking_client = util::create_reqwest_blocking_client(
            None,
            opts.timeout_secs,
            Some(resolved.url.clone()),
        )?;
        let (final_url, auth_token, ckan_hash) = if resolved.is_ckan {
            let ckan = resolve_ckan_resource(
                &blocking_client,
                &resolved.url,
                resolved.ckan_resource_search,
                opts.ckan_api_url.as_deref(),
                opts.ckan_token.as_deref(),
            )
            .map_err(|e| CliError::Other(format!("get: CKAN resolution failed: {e}")))?;
            let auth = if ckan.send_auth {
                opts.ckan_token.clone()
            } else {
                None
            };
            (ckan.data_url, auth, ckan.resource_hash)
        } else {
            (resolved.url.clone(), None, None)
        };

        let name = opts.name.clone().unwrap_or_else(|| derive_name(&final_url));

        // `--force` / `--refresh always` drop any existing entry so we re-fetch.
        if opts.force || opts.refresh_policy == RefreshPolicy::Always {
            let _ = delete_entry_by_name(&root, &name);
        }

        let observed_key = Arc::new(Mutex::new(None));
        let manager = QsvCacheManager {
            root:               root.clone(),
            source_uri:         opts.source.clone(),
            logical_name:       name.clone(),
            ttl_secs:           opts.ttl_secs,
            refresh_policy:     opts.refresh_policy,
            compression:        opts.compression,
            ckan_resource_hash: ckan_hash,
            observed_key:       observed_key.clone(),
        };
        let mode = match opts.refresh_policy {
            RefreshPolicy::Never => CacheMode::ForceCache,
            _ => CacheMode::Default,
        };

        let async_client =
            util::create_reqwest_async_client(None, opts.timeout_secs, Some(final_url.clone()))?;
        let client = ClientBuilder::new(async_client)
            .with(Cache(HttpCache {
                mode,
                manager,
                options: HttpCacheOptions::default(),
            }))
            .build();

        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let mut req = client.get(&final_url);
            if let Some(tok) = &auth_token {
                req = req.header(reqwest::header::AUTHORIZATION, tok);
            }
            let resp = req
                .send()
                .await
                .map_err(|e| CliError::Other(format!("get: request to {final_url} failed: {e}")))?;
            resp.error_for_status()
                .map_err(|e| CliError::Other(format!("get: {final_url} returned {e}")))?;
            Ok::<(), CliError>(())
        })?;

        let mut entry = match load_entry_by_name(&root, &name)? {
            Some(e) => e,
            None => {
                // Fresh cache hit: the response was served from `get`'s read path
                // without calling `put`, so no alias was created for the
                // requested name. Recover the entry via the cache key the
                // middleware just operated on and bind the requested name to it.
                let observed = observed_key.lock().ok().and_then(|g| g.clone());
                let mut e = observed
                    .as_deref()
                    .map(|k| entry_path(&root, &keyhash(k)))
                    .filter(|p| p.exists())
                    .map(|p| load_entry_at(&p))
                    .transpose()?
                    .ok_or_else(|| {
                        CliError::Other(format!(
                            "get: no cache entry for '{name}' after fetching {final_url}"
                        ))
                    })?;
                e.meta.logical_name = name.clone();
                write_entry(&root, &e)?;
                e
            },
        };
        ensure_indexed(&root, &mut entry)?;
        Ok(entry.meta)
    }

    /// Resolve a `dc:<name>` input path to a usable (decompressed) CSV file path,
    /// auto-refreshing the entry if stale. Also materializes the sibling `.idx`.
    pub fn resolve_dc_path(name: &str) -> CliResult<PathBuf> {
        let cache_dir = set_qsv_cache_dir(DEFAULT_CACHE_DIR)?;
        let root = get_root(&cache_dir);

        let mut entry = load_entry_by_name(&root, name)?.ok_or_else(|| {
            CliError::Other(format!(
                "dc: cache entry '{name}' not found. Fetch it first, e.g. `qsv get <source> \
                 --name {name}`."
            ))
        })?;

        // qsv-level staleness: refresh from the original source when past TTL.
        if entry.meta.refresh_policy != RefreshPolicy::Never && entry.meta.ttl_secs >= 0 {
            let age = unix_now().saturating_sub(entry.meta.downloaded_at);
            if age >= entry.meta.ttl_secs {
                let refresh_opts = GetOptions {
                    source:         entry.meta.source_uri.clone(),
                    name:           Some(name.to_string()),
                    cache_dir:      cache_dir.clone(),
                    ttl_secs:       entry.meta.ttl_secs,
                    refresh_policy: entry.meta.refresh_policy,
                    compression:    entry.meta.compression,
                    force:          false,
                    ckan_api_url:   std::env::var("QSV_CKAN_API")
                        .ok()
                        .or_else(|| Some(DEFAULT_CKAN_API.to_string())),
                    ckan_token:     std::env::var("QSV_CKAN_TOKEN").ok(),
                    timeout_secs:   30,
                };
                // Best-effort: on refresh failure, fall back to the stale copy.
                if get_resource(&refresh_opts).is_ok()
                    && let Some(refreshed) = load_entry_by_name(&root, name)?
                {
                    entry = refreshed;
                }
            }
        }

        let body = read_blob(&root, &entry.meta.blake3, entry.meta.compression)?;
        let dir = std::env::temp_dir().join("qsv-dc").join(&entry.meta.blake3);
        fs::create_dir_all(&dir)?;
        let csv_path = dir.join(safe_name(name));

        let need_write = !csv_path.exists()
            || fs::metadata(&csv_path).map(|m| m.len()).unwrap_or(0)
                != entry.meta.size_uncompressed;
        if need_write {
            atomic_write(&csv_path, &body)?;
        }

        // Materialize the sibling .idx (written after the CSV so its mtime is
        // not older than the CSV's, satisfying qsv's index-staleness check).
        let idx_blob = idx_blob_path(&root, &entry.meta.blake3);
        if idx_blob.exists() {
            let idx_dst = util::idx_path(&csv_path);
            if need_write || !idx_dst.exists() {
                let idx_bytes = read_zst(&idx_blob)?;
                atomic_write(&idx_dst, &idx_bytes)?;
                let _ = filetime::set_file_mtime(
                    &idx_dst,
                    filetime::FileTime::from_system_time(SystemTime::now()),
                );
            }
        }

        Ok(csv_path)
    }

    /// Write a cached entry's (decompressed) bytes to `output` (a file path, or
    /// `-` / `None` for stdout).
    pub fn write_output(cache_dir: &str, name: &str, output: Option<&str>) -> CliResult<()> {
        let root = get_root(cache_dir);
        let entry = load_entry_by_name(&root, name)?
            .ok_or_else(|| CliError::Other(format!("get: no cached entry named '{name}'")))?;
        let body = read_blob(&root, &entry.meta.blake3, entry.meta.compression)?;
        match output {
            Some(p) if p != "-" => atomic_write(Path::new(p), &body)?,
            _ => {
                use std::io::Write;
                std::io::stdout().write_all(&body)?;
            },
        }
        Ok(())
    }

    /// List all cache entries (for `cache-list` / `cache-info`).
    pub fn list_entries(cache_dir: &str) -> CliResult<Vec<CacheEntry>> {
        let entries_dir = get_root(cache_dir).join("entries");
        let mut out = Vec::new();
        let Ok(rd) = fs::read_dir(&entries_dir) else {
            return Ok(out);
        };
        for de in rd.flatten() {
            if de.path().extension().is_some_and(|e| e == "json")
                && let Ok(e) = load_entry_at(&de.path())
            {
                out.push(e.meta);
            }
        }
        out.sort_by(|a, b| a.logical_name.cmp(&b.logical_name));
        Ok(out)
    }

    /// Remove all cache entries and blobs. Returns the number of entries removed.
    pub fn clear(cache_dir: &str) -> CliResult<usize> {
        let root = get_root(cache_dir);
        let count = list_entries(cache_dir)?.len();
        for sub in ["entries", "aliases", "blobs"] {
            let p = root.join(sub);
            if p.exists() {
                fs::remove_dir_all(&p)?;
            }
        }
        Ok(count)
    }

    /// Remove entries last fetched more than `older_than_secs` ago. Returns the
    /// number of entries removed.
    pub fn prune(cache_dir: &str, older_than_secs: i64) -> CliResult<usize> {
        let root = get_root(cache_dir);
        let now = unix_now();
        let mut removed = 0;
        for meta in list_entries(cache_dir)? {
            // Inclusive: `--older-than 0` prunes everything (age >= 0).
            if now.saturating_sub(meta.downloaded_at) >= older_than_secs {
                delete_entry_files(&root, &meta);
                removed += 1;
            }
        }
        Ok(removed)
    }
}
