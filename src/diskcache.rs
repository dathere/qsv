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
//!    zstd-compressed blob store with a per-entry metadata index (BLAKE3, `ETag`, sizes, record
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

/// Resolve the qsv cache directory and create it if needed.
///
/// Honors the `QSV_CACHE_DIR` environment variable (which overrides
/// `cache_dir`) and expands a leading `~`. The path is otherwise returned as
/// given (it is NOT canonicalized/made absolute), then created if absent.
pub fn set_qsv_cache_dir(cache_dir: &str) -> Result<String, CliError> {
    // `expand_tilde` returns None if the home directory can't be determined;
    // propagate a CliError rather than panicking.
    let expand = |dir: &str| -> Result<String, CliError> {
        expand_tilde(dir)
            .map(|p| p.to_string_lossy().to_string())
            .ok_or_else(|| {
                CliError::Other(format!(
                    "could not expand '~' in cache directory '{dir}' (home directory not found)"
                ))
            })
    };
    let qsv_cache_dir = if let Ok(cache_path) = std::env::var("QSV_CACHE_DIR") {
        // QSV_CACHE_DIR overrides cache_dir; create it below if it doesn't exist.
        if cache_path.starts_with('~') {
            expand(&cache_path)?
        } else {
            cache_path
        }
    } else if cache_dir.starts_with('~') {
        expand(cache_dir)?
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
    /// True if this is a CKAN resource that still needs `resource_show` /
    /// `resource_search` resolution to obtain the actual data URL.
    pub is_ckan:              bool,
    /// True if the CKAN form was `ckan://<name>?` (`resource_search`) rather than
    /// `ckan://<id>` (`resource_show`).
    pub ckan_resource_search: bool,
}

/// Expand a source URI's scheme prefix into an http(s) URL plus CKAN flags.
///
/// - `dathere://<path>` → datHere's `qsv-lookup-tables` GitHub raw URL.
/// - `ckan://<id>`      → `<ckan_api>/resource_show?id=<id>`   (`is_ckan=true`).
/// - `ckan://<name>?`   → `<ckan_api>/resource_search?query=name:<name>` (`is_ckan=true`,
///   `ckan_resource_search=true`).
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
            // URL-encode the user-supplied name so spaces / `&` / `#` etc. don't
            // produce an invalid or ambiguous query string. The `name:` field
            // prefix is CKAN query syntax and is kept literal.
            let encoded: String = url::form_urlencoded::byte_serialize(name.as_bytes()).collect();
            return ResolvedUri {
                url:                  format!("{api}/resource_search?query=name:{encoded}"),
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
    // same origin as the CKAN API. Use the *effective* API base (the default
    // when none was given) so the token isn't needlessly stripped for resources
    // hosted on the default CKAN origin.
    let ckan_url_parsed = reqwest::Url::parse(ckan_api_url.unwrap_or(DEFAULT_CKAN_API)).ok();
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

/// The lowercased outer extension of `source` (query/fragment stripped) when it
/// is a compression extension this module knows (`zip`/`sz`/`gz`/`zlib`/`zst`),
/// else `None`. The set must match [`decompress_source`]'s arms.
fn compression_ext(source: &str) -> Option<String> {
    let path_part = source.split(['?', '#']).next().unwrap_or(source);
    Path::new(path_part)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .filter(|e| matches!(e.as_str(), "zip" | "sz" | "gz" | "zlib" | "zst"))
}

/// The tabular extension of `source`'s stem (e.g. `data.tsv.gz` → `Some("tsv")`),
/// used to pick the delimiter of decompressed content. `None` when the stem has no
/// recognized tabular extension.
fn inner_tabular_ext(source: &str) -> Option<String> {
    let path_part = source.split(['?', '#']).next().unwrap_or(source);
    Path::new(Path::new(path_part).file_stem()?)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .filter(|e| matches!(e.as_str(), "csv" | "tsv" | "tab" | "ssv"))
}

/// Result of [`decompress_source`]: the (possibly) decompressed payload plus the
/// tabular extension of the inner content when determinable. `inner_ext` drives
/// delimiter selection downstream (so a decompressed `.tsv.gz` is not read as CSV).
pub struct Decompressed {
    pub bytes:     Vec<u8>,
    pub inner_ext: Option<String>,
}

/// Detect compression from `source` (a path or URL; query/fragment stripped) by its
/// lowercased outer extension and decompress `body` in memory. Bytes are returned
/// unchanged (passthrough) when no known compression extension is present.
///
/// `.zip` and `.sz` are always available (the `zip` and `snap` crates are
/// non-optional); `.gz`/`.zlib` require the `flate2` codec and `.zst` the `zstd`
/// codec, returning an actionable error when the codec is not in the build.
///
/// Lives in the always-compiled core (not the `get`-gated `rich` tier) so both the
/// `get`/diskcache ingest and the lookup-table downloader (`lookup.rs`) can share it.
pub fn decompress_source(source: &str, body: Vec<u8>) -> Result<Decompressed, CliError> {
    use std::io::Read;

    // outer extension of the source, lowercased, with any query/fragment stripped
    let path_part = source.split(['?', '#']).next().unwrap_or(source);
    let outer_ext = Path::new(path_part)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);

    let inner_ext_of_stem = || inner_tabular_ext(source);
    // Neutral, source-scoped messages: this helper is shared by `get` ingest AND
    // `luau`/`validate`/`describegpt` lookup-table downloads, so do NOT prefix with
    // a single command name.
    let io_err = |e: std::io::Error| CliError::Other(format!("failed to decompress {source}: {e}"));

    match outer_ext.as_deref() {
        Some("zip") => {
            let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&body))
                .map_err(|e| CliError::Other(format!("invalid zip archive {source}: {e}")))?;
            let (idx, inner_ext) = util::select_zip_entry(&mut archive)?;
            let mut entry = archive
                .by_index(idx)
                .map_err(|e| CliError::Other(format!("reading zip entry from {source}: {e}")))?;
            let mut out = Vec::with_capacity(entry.size() as usize);
            entry.read_to_end(&mut out).map_err(io_err)?;
            Ok(Decompressed {
                bytes: out,
                inner_ext,
            })
        },
        Some("sz") => {
            let mut out = Vec::new();
            snap::read::FrameDecoder::new(std::io::Cursor::new(&body))
                .read_to_end(&mut out)
                .map_err(io_err)?;
            Ok(Decompressed {
                bytes:     out,
                inner_ext: inner_ext_of_stem(),
            })
        },
        #[cfg(feature = "flate2")]
        Some("gz") => {
            let mut out = Vec::new();
            flate2::read::MultiGzDecoder::new(std::io::Cursor::new(&body))
                .read_to_end(&mut out)
                .map_err(io_err)?;
            Ok(Decompressed {
                bytes:     out,
                inner_ext: inner_ext_of_stem(),
            })
        },
        #[cfg(feature = "flate2")]
        Some("zlib") => {
            let mut out = Vec::new();
            flate2::read::ZlibDecoder::new(std::io::Cursor::new(&body))
                .read_to_end(&mut out)
                .map_err(io_err)?;
            Ok(Decompressed {
                bytes:     out,
                inner_ext: inner_ext_of_stem(),
            })
        },
        #[cfg(feature = "zstd")]
        Some("zst") => {
            let out = zstd::decode_all(std::io::Cursor::new(&body)).map_err(io_err)?;
            Ok(Decompressed {
                bytes:     out,
                inner_ext: inner_ext_of_stem(),
            })
        },
        #[cfg(not(feature = "flate2"))]
        Some(ext @ ("gz" | "zlib")) => Err(CliError::Other(format!(
            "cannot decompress .{ext} source {source}: this qsv build lacks the 'flate2' codec. \
             The standard qsv and qsvmcp builds include it; otherwise rebuild with a feature that \
             enables it (e.g. 'fetch', 'apply', 'get', or 'luau')."
        ))),
        #[cfg(not(feature = "zstd"))]
        Some("zst") => Err(CliError::Other(format!(
            "cannot decompress .zst source {source}: this qsv build lacks the 'zstd' codec. The \
             standard qsv and qsvmcp builds include it; otherwise rebuild with a feature that \
             enables it (e.g. 'get' or 'luau')."
        ))),
        // not a known compression extension: passthrough. Record the tabular outer
        // extension (if any) so the caller can still pick the right delimiter.
        _ => {
            let inner_ext =
                outer_ext.filter(|e| matches!(e.as_str(), "csv" | "tsv" | "tab" | "ssv"));
            Ok(Decompressed {
                bytes: body,
                inner_ext,
            })
        },
    }
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
        io::{BufWriter, Read, Write},
        path::{Path, PathBuf},
        time::SystemTime,
    };

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
    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
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
        /// HTTP `ETag`, if the origin provided one.
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
        /// Non-secret cloud store-identity config (endpoint / region / account /
        /// …) captured at fetch time for `get_cloud` sources. Scopes the same URL
        /// across different S3-compatible endpoints or accounts so they do not
        /// collide, and lets `dc:` auto-refresh rebuild the correct store.
        /// Credentials are deliberately excluded. Empty for non-cloud entries.
        #[serde(default)]
        pub cloud_identity:     Vec<(String, String)>,
        /// For `ckan://` sources, the effective CKAN Action API base URL used at
        /// fetch time (the `--ckan-api` value or its resolved default), so a
        /// `dc:` auto-refresh re-resolves against the SAME CKAN instance rather
        /// than the ambient `QSV_CKAN_API`/default. None for non-CKAN entries.
        #[serde(default)]
        pub ckan_api_url:       Option<String>,
        /// Tabular extension (`csv`/`tsv`/`tab`/`ssv`) of the decompressed content
        /// for compressed sources, so a `dc:` materialized temp gets the right
        /// extension (and thus delimiter). None for plain or unknown sources.
        #[serde(default)]
        pub inner_ext:          Option<String>,
        /// Best-effort CSV dialect/schema sniffed from the cached blob during
        /// indexing (delimiter, header, field names & types). None for
        /// non-tabular blobs, when sniffing failed, or for entries cached before
        /// this field existed (backfilled on next `ensure_indexed`).
        #[serde(default)]
        pub sniffed:            Option<SniffedDialect>,
    }

    /// CSV dialect + schema sniffed from a cached blob, surfaced by
    /// `cache-list`/`cache-info`. Captured once during `ensure_indexed`.
    #[derive(Serialize, Deserialize, Clone)]
    pub struct SniffedDialect {
        /// Field delimiter (`,` for CSV, `\t` for TSV, `;` for SSV, …).
        pub delimiter:     char,
        /// Whether the first row was detected as a header.
        pub has_header:    bool,
        /// Number of preamble rows before the tabular data begins.
        pub preamble_rows: usize,
        /// Quote character, if any was detected.
        pub quote:         Option<char>,
        /// Whether records have a varying number of fields.
        pub flexible:      bool,
        /// Whether the sampled content is valid UTF-8.
        pub is_utf8:       bool,
        /// Number of fields (columns) detected.
        pub num_fields:    usize,
        /// Detected header/field names.
        pub fields:        Vec<String>,
        /// Detected per-field types (as strings).
        pub types:         Vec<String>,
    }

    /// The on-disk record: per-entry metadata. The data blob is content-addressed
    /// and stored separately (see `store_blob`).
    #[derive(Serialize, Deserialize)]
    struct StoredEntry {
        meta: CacheEntry,
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
        /// Extra `key=value` config pairs for cloud (`get_cloud`) sources,
        /// overlaid on the `AWS_*`/`AZURE_*`/`GOOGLE_*` environment. Ignored for
        /// non-cloud sources. Empty for `dc:` auto-refresh (env-only).
        pub cloud_opts:     Vec<String>,
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
    /// Lossy (used only for cosmetic temp filenames under a content-addressed
    /// directory); alias filenames use the reversible `encode_name` instead.
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

    /// The tabular file extensions a `dc:` handle may materialize under. They
    /// select the delimiter (`csv` => comma, `tsv`/`tab` => tab, `ssv` =>
    /// semicolon), so they also scope the per-extension stats-cache blob and the
    /// per-extension temp subdir in `resolve_dc_path`.
    const TABULAR_EXTS: [&str; 4] = ["csv", "tsv", "tab", "ssv"];

    /// A temp filename for a `dc:` handle that is guaranteed to carry a known
    /// tabular extension, so `Config`'s format check accepts it. Prefers the
    /// handle's own extension, then the decompressed content's `inner_ext` (for
    /// compressed sources, where the source URI's outer extension is the codec,
    /// not the tabular format), then the cached source's, falling back to `.csv`.
    fn tabular_temp_name(name: &str, resolved_uri: &str, inner_ext: Option<&str>) -> String {
        let is_known = |p: &str| {
            Path::new(p)
                .extension()
                .and_then(|e| e.to_str())
                .map(str::to_ascii_lowercase)
                .is_some_and(|e| TABULAR_EXTS.contains(&e.as_str()))
        };
        let base = safe_name(name);
        if is_known(&base) {
            return base;
        }
        // The decompressed content's tabular extension wins over the source's
        // outer extension: a `data.tsv.gz` has resolved_uri ending in `.gz`, but
        // its inner_ext is `tsv` — so it must materialize as `.tsv`, not `.csv`.
        if let Some(ext) = inner_ext.filter(|e| TABULAR_EXTS.contains(e)) {
            return format!("{base}.{ext}");
        }
        let source = resolved_uri
            .split(['?', '#'])
            .next()
            .unwrap_or(resolved_uri);
        let ext = Path::new(source)
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .filter(|e| TABULAR_EXTS.contains(&e.as_str()))
            .unwrap_or_else(|| "csv".to_string());
        format!("{base}.{ext}")
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

    /// Path of the durable, content-addressed stats-cache blob for an entry's
    /// content AND parsing extension `ext` (csv/tsv/tab/ssv): the zstd-compressed
    /// `.stats.csv.data.jsonl` that the "smart" commands (frequency, schema,
    /// profile, ...) build via `get_stats_records`. Keyed by extension as well as
    /// content hash because the stats fields depend on the delimiter the
    /// extension selects, so `.csv` and `.tsv` aliases of the same bytes must not
    /// share a blob. Compression-agnostic, like `idx_blob_path`.
    fn stats_blob_path(root: &Path, b3: &str, ext: &str) -> PathBuf {
        blob_dir(root, b3).join(format!("{b3}.{ext}.stats.jsonl.zst"))
    }

    fn entry_path(root: &Path, keyhash: &str) -> PathBuf {
        root.join("entries").join(format!("{keyhash}.json"))
    }

    /// The canonical alias filename for a logical name: BLAKE3 of the name (a
    /// fixed 64-char, filesystem-safe key). Hashing keeps the filename bounded
    /// regardless of name length (a long name's hex would otherwise exceed the
    /// 255-byte filename limit) while staying injective in practice. The
    /// original name is preserved inside the file content (see `write_alias`).
    fn alias_path(root: &Path, name: &str) -> PathBuf {
        root.join("aliases")
            // `.as_str()`: blake3's `to_hex()` is an `ArrayString` (impls
            // `AsRef<str>`, not `AsRef<Path>`), so feed `Path::join` a `&str`.
            .join(blake3::hash(name.as_bytes()).to_hex().as_str())
    }

    /// Write an alias file: content is `"{keyhash}\n{name}"` so the original
    /// logical name is recoverable for `cache-list` display.
    fn write_alias(root: &Path, name: &str, keyhash: &str) -> CliResult<()> {
        atomic_write(
            &alias_path(root, name),
            format!("{keyhash}\n{name}").as_bytes(),
        )?;
        Ok(())
    }

    /// Parse an alias file's content into `(keyhash, original_name?)`. Legacy
    /// alias files (from earlier commits) hold only the key hash, so `name` is
    /// None for those.
    fn parse_alias(content: &str) -> (String, Option<String>) {
        let mut lines = content.lines();
        let kh = lines.next().unwrap_or_default().trim().to_string();
        let name = lines.next().map(ToString::to_string);
        (kh, name)
    }

    /// Resolve a logical name to its entry key hash via its canonical alias.
    fn alias_keyhash(root: &Path, name: &str) -> CliResult<Option<String>> {
        match fs::read_to_string(alias_path(root, name)) {
            Ok(content) => Ok(Some(parse_alias(&content).0)),
            Err(_) => Ok(None),
        }
    }

    /// A process- and call-unique token for temp filenames, so concurrent
    /// writers never collide on the same temp path.
    fn unique_token() -> String {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("{}-{n}", std::process::id())
    }

    /// Atomically write `bytes` to `path` (write to a unique temp sibling, then
    /// rename). The temp name is process+call-unique so concurrent writers to
    /// the same target don't clobber each other's temp file. `fs::rename`
    /// replaces an existing destination on both Unix and Windows.
    fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension(format!("tmp-{}", unique_token()));
        fs::write(&tmp, bytes)?;
        if let Err(e) = fs::rename(&tmp, path) {
            // Best-effort cleanup so a failed rename doesn't leave temp litter.
            let _ = fs::remove_file(&tmp);
            return Err(e);
        }
        Ok(())
    }

    fn read_zst(path: &Path) -> std::io::Result<Vec<u8>> {
        zstd::decode_all(fs::File::open(path)?)
    }

    /// Store `body` as a content-addressed (possibly compressed) blob.
    /// Returns (blake3, `compressed_size`, `uncompressed_size`).
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

    /// The compressed-output sink behind a `BlobSink` (zstd-encoding or raw),
    /// kept as an enum (rather than `Box<dyn Write>`) so `finish` can propagate a
    /// zstd flush error instead of swallowing it on drop.
    enum BlobSinkWriter {
        Zstd(zstd::stream::write::Encoder<'static, BufWriter<fs::File>>),
        Raw(BufWriter<fs::File>),
    }

    /// Streaming, content-addressed blob writer. Feed it the uncompressed byte
    /// chunks **in order** (`write`); it incrementally BLAKE3-hashes the stream
    /// and writes the (optionally zstd-compressed) bytes to a temp file. `finish`
    /// renames that temp to the content-addressed blob path `{blake3}.{ext}` and
    /// returns `(blake3, compressed_size, uncompressed_size)`.
    ///
    /// Unlike `store_blob` (which takes the whole `&[u8]`), this never holds the
    /// full object in memory — the prerequisite for ingesting large objects in
    /// bounded memory (used by both the HTTP `ingest_http` and the cloud
    /// `ingest_cloud` ranged/streaming download paths).
    struct BlobSink {
        hasher:       blake3::Hasher,
        // `Some` while ingesting; `take`n by `finish` (or by `Drop` on a failed
        // ingest) so the underlying file handle is released before the temp file
        // is renamed into place or unlinked.
        writer:       Option<BlobSinkWriter>,
        tmp:          PathBuf,
        compression:  Compression,
        uncompressed: u64,
    }

    impl BlobSink {
        fn new(root: &Path, compression: Compression) -> CliResult<Self> {
            let blobs = root.join("blobs");
            fs::create_dir_all(&blobs)?;
            let tmp = blobs.join(format!("ingest-{}.tmp", unique_token()));
            let buf = BufWriter::new(fs::File::create(&tmp)?);
            let writer = match compression {
                Compression::Zstd => {
                    BlobSinkWriter::Zstd(zstd::stream::write::Encoder::new(buf, ZSTD_LEVEL)?)
                },
                Compression::None => BlobSinkWriter::Raw(buf),
            };
            Ok(Self {
                hasher: blake3::Hasher::new(),
                writer: Some(writer),
                tmp,
                compression,
                uncompressed: 0,
            })
        }

        // Feed the next uncompressed chunk: hash it and write it (optionally
        // zstd-compressed) to the temp blob. Named `push` (not `write`) so the
        // `std::io::Write` impl below — used to stream a decoder's output into the
        // sink — doesn't collide with this inherent method.
        fn push(&mut self, chunk: &[u8]) -> std::io::Result<()> {
            self.hasher.update(chunk);
            self.uncompressed += chunk.len() as u64;
            match self.writer.as_mut() {
                Some(BlobSinkWriter::Zstd(e)) => e.write_all(chunk),
                Some(BlobSinkWriter::Raw(w)) => w.write_all(chunk),
                // Unreachable: `finish` consumes the sink, so no write follows it.
                None => Ok(()),
            }
        }

        /// Finalize the stream and atomically move it into place. On any error
        /// here the temp file is removed; if the ingest errors out *before*
        /// `finish` is ever called, `Drop` removes the partial temp instead, so a
        /// failed ingest never leaves litter in the cache.
        fn finish(mut self, root: &Path) -> CliResult<(String, u64, u64)> {
            // Take the writer so it is dropped (closing the file) here, and so
            // `Drop` sees the sink as already finalized and skips its cleanup.
            let writer = self.writer.take();
            let result = (|| -> std::io::Result<(String, u64, u64)> {
                // Flush the (zstd or plain) writer fully down to the file.
                let buf = match writer {
                    Some(BlobSinkWriter::Zstd(e)) => e.finish()?,
                    Some(BlobSinkWriter::Raw(w)) => w,
                    None => return Err(std::io::Error::other("BlobSink already finished")),
                };
                let file = buf
                    .into_inner()
                    .map_err(std::io::IntoInnerError::into_error)?;
                file.sync_all()?;
                drop(file);
                let b3 = self.hasher.finalize().to_hex().to_string();
                let size_compressed = fs::metadata(&self.tmp)?.len();
                let dest = blob_path(root, &b3, self.compression);
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::rename(&self.tmp, &dest)?;
                Ok((b3, size_compressed, self.uncompressed))
            })();
            if result.is_err() {
                let _ = fs::remove_file(&self.tmp);
            }
            Ok(result?)
        }
    }

    // Lets a streaming decoder (flate2/zstd `write::*Decoder`) write its
    // decompressed output straight into the sink, so a compressed download is
    // decoded in bounded memory instead of being fully buffered first. `flush` is
    // a no-op — the data is finalized in `finish`.
    impl std::io::Write for BlobSink {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.push(buf)?;
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Drop for BlobSink {
        fn drop(&mut self) {
            // If `finish` ran it already took the writer (and renamed or removed
            // the temp), so there is nothing to clean up. If the ingest errored
            // out before `finish`, the writer is still here: drop it first to
            // release the file handle (so Windows can unlink it), then remove the
            // partial temp so a failed/aborted download leaves no litter.
            if self.writer.take().is_some() {
                let _ = fs::remove_file(&self.tmp);
            }
        }
    }

    /// Ingest target for a download: either the streaming `BlobSink` (uncompressed
    /// sources — bounded memory) or an in-memory buffer that decompresses the whole
    /// payload on `finish` (compressed sources, where the codec needs the full
    /// payload so streaming buys nothing). Both expose the same `write`/`finish`,
    /// so the HTTP and cloud download loops stay identical regardless of mode.
    // A `write::*Decoder` that streams a download's decompressed bytes into a
    // `BlobSink` (bounded memory). Only formats that decode push-style live here;
    // `.zip` (needs random access) and `.sz` (snap has no push-decoder) stay on the
    // full-buffer `IngestSink::Buffer` path.
    enum DecodeWriter {
        #[cfg(feature = "flate2")]
        Gz(flate2::write::MultiGzDecoder<BlobSink>),
        #[cfg(feature = "flate2")]
        Zlib(flate2::write::ZlibDecoder<BlobSink>),
        #[cfg(feature = "zstd")]
        Zst(zstd::stream::write::Decoder<'static, BlobSink>),
    }

    // Boxed (it owns a `BlobSink`) so it doesn't bloat `IngestSink`.
    struct DecodeState {
        writer:    DecodeWriter,
        inner_ext: Option<String>,
    }

    enum IngestSink {
        // Boxed: `BlobSink` (with its zstd encoder) is far larger than the
        // `Buffer` variant, so box it to keep the enum small.
        Stream(Box<BlobSink>),
        // Streaming decompression into a `BlobSink` (gz/zlib/zst): bounded memory.
        Decode(Box<DecodeState>),
        // Full-buffer decompression on `finish` (zip & sz; also gz/zlib/zst when
        // their codec feature is absent — `finish` then surfaces the actionable
        // build-feature error via `decompress_source`).
        Buffer {
            source:      String,
            compression: Compression,
            buf:         Vec<u8>,
        },
    }

    impl IngestSink {
        /// Pick the ingest mode from `source`'s extension: stream-decode the
        /// push-decodable compressed formats into the blob (bounded memory),
        /// full-buffer `.zip`/`.sz`, and stream plain sources unchanged.
        fn for_source(root: &Path, compression: Compression, source: &str) -> CliResult<Self> {
            let buffer = |_root| {
                Ok(IngestSink::Buffer {
                    source: source.to_string(),
                    compression,
                    buf: Vec::new(),
                })
            };
            let decode = |writer| {
                Ok(IngestSink::Decode(Box::new(DecodeState {
                    writer,
                    inner_ext: super::inner_tabular_ext(source),
                })))
            };
            match super::compression_ext(source).as_deref() {
                #[cfg(feature = "flate2")]
                Some("gz") => decode(DecodeWriter::Gz(flate2::write::MultiGzDecoder::new(
                    BlobSink::new(root, compression)?,
                ))),
                #[cfg(feature = "flate2")]
                Some("zlib") => decode(DecodeWriter::Zlib(flate2::write::ZlibDecoder::new(
                    BlobSink::new(root, compression)?,
                ))),
                #[cfg(feature = "zstd")]
                Some("zst") => decode(DecodeWriter::Zst(zstd::stream::write::Decoder::new(
                    BlobSink::new(root, compression)?,
                )?)),
                // zip, sz, and (codec-absent) gz/zlib/zst -> full-buffer path.
                Some(_) => buffer(root),
                None => Ok(IngestSink::Stream(Box::new(BlobSink::new(
                    root,
                    compression,
                )?))),
            }
        }

        fn write(&mut self, chunk: &[u8]) -> CliResult<()> {
            use std::io::Write;
            match self {
                IngestSink::Stream(s) => s.push(chunk)?,
                IngestSink::Buffer { buf, .. } => buf.extend_from_slice(chunk),
                IngestSink::Decode(state) => match &mut state.writer {
                    #[cfg(feature = "flate2")]
                    DecodeWriter::Gz(w) => w.write_all(chunk)?,
                    #[cfg(feature = "flate2")]
                    DecodeWriter::Zlib(w) => w.write_all(chunk)?,
                    #[cfg(feature = "zstd")]
                    DecodeWriter::Zst(w) => w.write_all(chunk)?,
                },
            }
            Ok(())
        }

        /// Finalize and store the blob. Returns `(blake3, compressed_size,
        /// uncompressed_size, inner_ext)`.
        fn finish(self, root: &Path) -> CliResult<(String, u64, u64, Option<String>)> {
            match self {
                IngestSink::Stream(s) => {
                    let (b3, sc, su) = s.finish(root)?;
                    Ok((b3, sc, su, None))
                },
                IngestSink::Decode(state) => {
                    // Finalize the decoder (flushing all decompressed output into the
                    // sink), recover the BlobSink, then finalize the blob.
                    let blob = match state.writer {
                        #[cfg(feature = "flate2")]
                        DecodeWriter::Gz(w) => w.finish()?,
                        #[cfg(feature = "flate2")]
                        DecodeWriter::Zlib(w) => w.finish()?,
                        #[cfg(feature = "zstd")]
                        DecodeWriter::Zst(mut w) => {
                            // zstd's write Decoder buffers decompressed output;
                            // `into_inner` alone truncates it, so flush first to
                            // drain every remaining byte into the sink.
                            std::io::Write::flush(&mut w)?;
                            w.into_inner()
                        },
                    };
                    let (b3, sc, su) = blob.finish(root)?;
                    Ok((b3, sc, su, state.inner_ext))
                },
                IngestSink::Buffer {
                    source,
                    compression,
                    buf,
                } => {
                    let d = super::decompress_source(&source, buf)?;
                    let (b3, sc, su) = store_blob(root, &d.bytes, compression)?;
                    Ok((b3, sc, su, d.inner_ext))
                },
            }
        }
    }

    /// What a successful download yields: `(blake3, compressed_size,
    /// uncompressed_size, etag, last_modified, inner_ext)`. `None` (at the
    /// `Option` use sites) means the origin reported Not-Modified.
    type FetchedBlob = (
        String,
        u64,
        u64,
        Option<String>,
        Option<String>,
        Option<String>,
    );

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

    // The cache uses a two-level model: an *entry* (`entries/{keyhash}.json`) is
    // keyed by its cache key (the URL for HTTP, `FILE:<abs>` for local) and owns
    // the blob + metadata; a *name* (`aliases/<name>`) is a user-facing handle
    // that points at an entry's key hash. Many names may point at one entry, and
    // (via content-addressed dedup) many entries may share one blob.

    /// Write/replace the entry for its cache key and point the entry's primary
    /// name at it. Reclaims blobs/entries orphaned by the write.
    fn write_entry(root: &Path, entry: &StoredEntry) -> CliResult<()> {
        let kh = keyhash(&entry.meta.cache_key);
        let json = serde_json::to_vec(entry)
            .map_err(|e| CliError::Other(format!("get: failed to serialize cache entry: {e}")))?;

        // The entry currently stored under this same cache key (if any), so we
        // can reclaim its blob when the content or compression changed.
        let prev_same_key = load_entry_at(&entry_path(root, &kh)).ok();

        // Write the new entry JSON first so it counts toward refcounts below.
        atomic_write(&entry_path(root, &kh), &json)?;

        // (A) Repoint the primary name. If it previously pointed at a *different*
        // entry that now has no remaining names, remove that orphaned entry.
        let prev_alias_kh = alias_keyhash(root, &entry.meta.logical_name)?;
        write_alias(root, &entry.meta.logical_name, &kh)?;
        if let Some(old_kh) = prev_alias_kh
            && old_kh != kh
            && aliases_pointing_to(root, &old_kh).is_empty()
        {
            delete_entry_by_keyhash(root, &old_kh);
        }

        // (B) Same cache key, but the blob (content or compression) changed:
        // reclaim the previous blob/index when nothing else references it.
        if let Some(prev) = prev_same_key {
            let prev_blob = blob_path(root, &prev.meta.blake3, prev.meta.compression);
            let new_blob = blob_path(root, &entry.meta.blake3, entry.meta.compression);
            if prev_blob != new_blob && !blob_path_referenced(root, &prev_blob, &kh) {
                let _ = fs::remove_file(&prev_blob);
            }
            if prev.meta.blake3 != entry.meta.blake3
                && !entry_references_blob(root, &prev.meta.blake3, &kh)
            {
                let _ = fs::remove_file(idx_blob_path(root, &prev.meta.blake3));
                // Reclaim the per-extension stats-cache blobs for the old content
                // too; otherwise a refresh-to-new-content orphans them.
                for ext in TABULAR_EXTS {
                    let _ = fs::remove_file(stats_blob_path(root, &prev.meta.blake3, ext));
                }
            }
        }

        Ok(())
    }

    /// Bind a name to an existing entry (by key hash) without modifying the
    /// entry. Used when a fresh cache hit is requested under a new name: the
    /// middleware serves it from its read path so `put`/`write_entry` never runs.
    fn bind_alias(root: &Path, name: &str, keyhash: &str) -> CliResult<()> {
        write_alias(root, name, keyhash)
    }

    fn load_entry_at(path: &Path) -> CliResult<StoredEntry> {
        let bytes = fs::read(path)?;
        serde_json::from_slice(&bytes).map_err(|e| {
            CliError::Other(format!("get: corrupt cache entry {}: {e}", path.display()))
        })
    }

    fn load_entry_by_name(root: &Path, name: &str) -> CliResult<Option<StoredEntry>> {
        let Some(kh) = alias_keyhash(root, name)? else {
            return Ok(None);
        };
        let ep = entry_path(root, &kh);
        if !ep.exists() {
            return Ok(None);
        }
        Ok(Some(load_entry_at(&ep)?))
    }

    /// Alias files that currently point at the given entry key hash.
    fn aliases_pointing_to(root: &Path, keyhash: &str) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let Ok(rd) = fs::read_dir(root.join("aliases")) else {
            return out;
        };
        for de in rd.flatten() {
            if fs::read_to_string(de.path())
                .map(|s| parse_alias(&s).0 == keyhash)
                .unwrap_or(false)
            {
                out.push(de.path());
            }
        }
        out
    }

    /// True if any entry other than `except_keyhash` references the given blob
    /// (by content hash). Content-addressed dedup means a blob may be shared.
    fn entry_references_blob(root: &Path, b3: &str, except_keyhash: &str) -> bool {
        let Ok(rd) = fs::read_dir(root.join("entries")) else {
            return false;
        };
        for de in rd.flatten() {
            if de
                .path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                == Some(except_keyhash.to_string())
            {
                continue;
            }
            if let Ok(e) = load_entry_at(&de.path())
                && e.meta.blake3 == b3
            {
                return true;
            }
        }
        false
    }

    /// True if any entry other than `except_keyhash` maps to the exact blob file
    /// path (same content hash *and* compression).
    fn blob_path_referenced(root: &Path, blob: &Path, except_keyhash: &str) -> bool {
        let Ok(rd) = fs::read_dir(root.join("entries")) else {
            return false;
        };
        for de in rd.flatten() {
            if de
                .path()
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                == Some(except_keyhash.to_string())
            {
                continue;
            }
            if let Ok(e) = load_entry_at(&de.path())
                && blob_path(root, &e.meta.blake3, e.meta.compression) == blob
            {
                return true;
            }
        }
        false
    }

    /// Fully remove the entry at `keyhash`: every name pointing at it, its JSON,
    /// and its blob/index when nothing else references them. The data blob is
    /// freed on an exact path match (content hash *and* compression differ in
    /// the filename), while the index — `{blake3}.idx.zst`, compression-agnostic
    /// — is freed on a content-hash match.
    fn delete_entry_by_keyhash(root: &Path, keyhash: &str) {
        let entry = load_entry_at(&entry_path(root, keyhash)).ok();
        for ap in aliases_pointing_to(root, keyhash) {
            let _ = fs::remove_file(ap);
        }
        let _ = fs::remove_file(entry_path(root, keyhash));
        if let Some(e) = entry {
            let blob = blob_path(root, &e.meta.blake3, e.meta.compression);
            if !blob_path_referenced(root, &blob, keyhash) {
                let _ = fs::remove_file(&blob);
            }
            if !entry_references_blob(root, &e.meta.blake3, keyhash) {
                let _ = fs::remove_file(idx_blob_path(root, &e.meta.blake3));
                // Stats blobs are keyed by content hash AND parsing extension;
                // free every per-extension variant for this content.
                for ext in TABULAR_EXTS {
                    let _ = fs::remove_file(stats_blob_path(root, &e.meta.blake3, ext));
                }
            }
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

    /// Max bytes fed to the CSV sniffer. Sniffing the head is enough to detect
    /// the dialect/schema; reading a whole multi-GB blob would be wasteful.
    const SNIFF_SAMPLE_BYTES: usize = 1 << 20; // 1 MiB

    /// Best-effort sniff of a CSV blob's dialect + schema from its head. Returns
    /// None for non-tabular content or on any sniff error — never fails ingest.
    fn sniff_dialect(body: &[u8]) -> Option<SniffedDialect> {
        use csv_nose::{SampleSize, Sniffer, metadata::Quote};

        let head = &body[..body.len().min(SNIFF_SAMPLE_BYTES)];
        let metadata = Sniffer::new()
            .sample_size(SampleSize::All)
            .sniff_reader(std::io::Cursor::new(head))
            .ok()?;
        let fields: Vec<String> = metadata.fields.iter().map(ToString::to_string).collect();
        Some(SniffedDialect {
            delimiter: metadata.dialect.delimiter as char,
            has_header: metadata.dialect.header.has_header_row,
            preamble_rows: metadata.dialect.header.num_preamble_rows,
            quote: match metadata.dialect.quote {
                Quote::Some(chr) => Some(char::from(chr)),
                Quote::None => None,
            },
            flexible: metadata.dialect.flexible,
            is_utf8: metadata.dialect.is_utf8,
            num_fields: fields.len(),
            fields,
            types: metadata.types.iter().map(ToString::to_string).collect(),
        })
    }

    /// Build (and cache) the qsv index for an entry's blob, recording the exact
    /// record count. No-op if already indexed.
    fn ensure_indexed(root: &Path, entry: &mut StoredEntry) -> CliResult<()> {
        if entry.meta.indexed {
            return Ok(());
        }
        let body = read_blob(root, &entry.meta.blake3, entry.meta.compression)?;

        // Sniff the dialect/schema once from the head (best-effort).
        if entry.meta.sniffed.is_none() {
            entry.meta.sniffed = sniff_dialect(&body);
        }

        let tmp_dir = std::env::temp_dir().join("qsv-getidx");
        fs::create_dir_all(&tmp_dir)?;
        // Unique temp name per call so concurrent `get`s of the same content
        // don't race on the same temp CSV (and its sibling .idx).
        let tmp_csv = tmp_dir.join(format!(
            "{}-{}.csv",
            &entry.meta.blake3[..16],
            unique_token()
        ));
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

    fn ingest_local(opts: &GetOptions, root: &Path, path: &Path) -> CliResult<CacheEntry> {
        // Stream the (possibly compressed) local source into the content-addressed
        // blob store so the stored blob is plain CSV (auto-indexable, correct
        // record_count). Streaming gz/zlib/zst keeps memory bounded for large local
        // inputs — mirroring the remote `ingest_http`/`ingest_cloud` paths — while
        // zip/sz (and codec-absent gz/zlib/zst) still full-buffer per `IngestSink`'s
        // per-format policy. Plain sources stream through unchanged.
        use std::io::Read;
        let mut sink = IngestSink::for_source(root, opts.compression, &path.to_string_lossy())?;
        let mut reader = std::io::BufReader::new(fs::File::open(path)?);
        let mut chunk = [0u8; 64 * 1024];
        loop {
            let n = reader.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            sink.write(&chunk[..n])?;
        }
        let (b3, size_compressed, size_uncompressed, inner_ext) = sink.finish(root)?;
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
            cloud_identity: Vec::new(),
            ckan_api_url: None,
            inner_ext,
            sniffed: None,
        };
        let mut entry = StoredEntry { meta };
        write_entry(root, &entry)?;
        ensure_indexed(root, &mut entry)?;
        Ok(entry.meta)
    }

    /// True if `source` uses a cloud object-store URL scheme handled by
    /// `object_store` (`s3://`, `gs://`, `az://` and friends). Detection is
    /// compiled in even without `get_cloud` so `get_resource` can emit a precise
    /// "rebuild with --features `get_cloud`" hint rather than a generic
    /// "unsupported source" error.
    fn is_cloud_scheme(source: &str) -> bool {
        let lower = source.to_ascii_lowercase();
        const SCHEMES: [&str; 8] = [
            "s3://", "s3a://", "gs://", "az://", "adl://", "azure://", "abfs://", "abfss://",
        ];
        SCHEMES.iter().any(|s| lower.starts_with(s))
    }

    /// Build the `object_store` config option list for a cloud fetch: the
    /// `AWS_*`/`AZURE_*`/`GOOGLE_*` environment (`object_store`'s `parse_url_opts`
    /// does NOT read the environment itself), overlaid with any user
    /// `--cloud-opt key=value` pairs, which take precedence. The environment is
    /// filtered to provider prefixes so unrelated vars (e.g. a generic `ENDPOINT`
    /// or `REGION`) can't be silently picked up as store config.
    #[cfg(feature = "get_cloud")]
    fn cloud_opts_for(extra: &[String]) -> Vec<(String, String)> {
        let mut opts: Vec<(String, String)> = std::env::vars()
            .filter(|(k, _)| {
                let u = k.to_ascii_uppercase();
                u.starts_with("AWS_") || u.starts_with("AZURE_") || u.starts_with("GOOGLE_")
            })
            .collect();
        for kv in extra {
            if let Some((k, v)) = kv.split_once('=') {
                opts.push((k.trim().to_string(), v.to_string()));
            }
        }
        opts
    }

    /// True for config keys whose values are credentials/secrets, which must
    /// never be folded into the (on-disk, plaintext) cache key or persisted
    /// store identity.
    #[cfg(feature = "get_cloud")]
    fn is_secret_opt_key(key: &str) -> bool {
        const SECRET_MARKERS: [&str; 8] = [
            "secret",
            "token",
            "password",
            "credential",
            "key",
            "sas",
            "service_account",
            "application_credentials",
        ];
        SECRET_MARKERS.iter().any(|m| key.contains(m))
    }

    /// The non-secret, identity-determining subset of cloud config options,
    /// lower-cased (`object_store` treats keys case-insensitively) and reduced to a
    /// single value per key using the **same last-wins precedence `object_store`
    /// applies when building the store** (`cloud_opts_for` lists env first, then
    /// `--cloud-opt`, and `parse_url_opts` folds options so the later value
    /// overrides). The result is sorted by key (`BTreeMap` iteration order).
    ///
    /// Collapsing per key — rather than de-duping whole `(key, value)` pairs — is
    /// what keeps the cache key aligned with the *effective* store and stable
    /// across `dc:` auto-refresh: replaying the persisted identity as
    /// high-precedence `--cloud-opt` makes it win over any (even changed) ambient
    /// env var, so the recomputed key matches the original. Credentials are
    /// excluded so they never reach the on-disk cache key or the persisted entry.
    #[cfg(feature = "get_cloud")]
    fn cloud_identity_opts(opts: &[(String, String)]) -> Vec<(String, String)> {
        let mut id: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
        for (k, v) in opts {
            let lk = k.to_ascii_lowercase();
            if !is_secret_opt_key(&lk) {
                // Later occurrence wins, mirroring the store builder's fold.
                id.insert(lk, v.clone());
            }
        }
        id.into_iter().collect()
    }

    /// Build a cloud cache key that scopes `source` by its resolved store
    /// identity, so the same URL on different backing stores does not collide.
    #[cfg(feature = "get_cloud")]
    fn cloud_cache_key(source: &str, identity: &[(String, String)]) -> String {
        if identity.is_empty() {
            return format!("CLOUD:{source}");
        }
        let joined = identity
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        format!("CLOUD:{source}#{joined}")
    }

    /// Default per-part size for ranged downloads (HTTP and cloud): objects larger
    /// than this are fetched as parallel byte-ranges of this size (8 MiB). Override
    /// with `QSV_GET_PART_SIZE` (bytes).
    const DEFAULT_PART_SIZE: u64 = 8 * 1024 * 1024;
    /// Default max number of concurrent range GETs for one download (4). Override
    /// with `QSV_GET_CONCURRENCY`. Peak extra memory is roughly
    /// `concurrency * part_size`, independent of total object size.
    const DEFAULT_DL_CONCURRENCY: u64 = 4;

    /// Read a `u64` tuning value from environment variable `name`, falling back
    /// to `default` when it is unset or does not parse.
    fn env_u64(name: &str, default: u64) -> u64 {
        std::env::var(name)
            .ok()
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(default)
    }

    /// Fetch a cloud object-store source (`s3://`, `gs://`, `az://`, …) into the
    /// cache, with ETag-based conditional revalidation (parity with the HTTP
    /// path's "don't re-download unchanged" guarantee) and **streaming, parallel
    /// ranged downloads** for large objects. Mirrors `ingest_local` plus the
    /// alias/orphan-cleanup tail of `get_resource`.
    ///
    /// A single ranged GET of the first part doubles as the conditional
    /// revalidation request AND the size probe (a 206 carries the total object
    /// size via `Content-Range`), so there is no extra HEAD round-trip. Objects
    /// larger than one part are fetched as concurrent byte-ranges and streamed
    /// straight into a `BlobSink`, so peak memory is bounded (≈ `concurrency *
    /// part_size`) rather than the whole object.
    #[cfg(feature = "get_cloud")]
    fn ingest_cloud(opts: &GetOptions, root: &Path, source: &str) -> CliResult<CacheEntry> {
        use std::sync::Arc;

        use futures_util::stream::StreamExt;
        use object_store::{GetOptions as OsGetOptions, GetRange, ObjectStore, parse_url_opts};
        use url::Url;

        let url = Url::parse(source)
            .map_err(|e| CliError::Other(format!("get: invalid cloud URL '{source}': {e}")))?;
        // Scope the cache entry by the resolved store identity (endpoint /
        // region / account / …) so the same URL on different backing stores does
        // not collide, and persist that identity so a later `dc:` auto-refresh
        // rebuilds the correct store. Credentials are excluded from both.
        let all_opts = cloud_opts_for(&opts.cloud_opts);
        let identity = cloud_identity_opts(&all_opts);
        let (store, path) = parse_url_opts(&url, all_opts).map_err(|e| {
            CliError::Other(format!("get: cannot open cloud store for '{source}': {e}"))
        })?;
        // Shared across the concurrent range fetches below.
        let store = Arc::new(store);

        let cache_key = cloud_cache_key(source, &identity);
        let kh = keyhash(&cache_key);
        let name = opts.name.clone().unwrap_or_else(|| derive_name(source));

        let mut existing = load_entry_at(&entry_path(root, &kh)).ok();

        // --refresh never: serve the cached copy without touching the origin.
        if opts.refresh_policy == RefreshPolicy::Never
            && let Some(mut entry) = existing.take()
        {
            bind_alias(root, &name, &kh)?;
            ensure_indexed(root, &mut entry)?;
            let mut meta = entry.meta;
            meta.logical_name = name;
            return Ok(meta);
        }

        // --force / --refresh always -> unconditional GET; otherwise send a
        // conditional If-None-Match (on the probe) when we already hold an ETag.
        let unconditional = opts.force || opts.refresh_policy == RefreshPolicy::Always;
        let conditional_etag = if unconditional {
            None
        } else {
            existing.as_ref().and_then(|e| e.meta.etag.clone())
        };

        // Tuning knobs are environment-only so they never collide with the
        // object_store config keys that `--cloud-opt` feeds to parse_url_opts.
        let part_size = env_u64("QSV_GET_PART_SIZE", DEFAULT_PART_SIZE).max(1);
        let concurrency =
            env_u64("QSV_GET_CONCURRENCY", DEFAULT_DL_CONCURRENCY).clamp(1, 64) as usize;

        let rt = tokio::runtime::Runtime::new()?;
        // None => origin reported Not-Modified; Some => the freshly stored blob's
        // (blake3, compressed_size, uncompressed_size, etag, last_modified).
        let fetched: Option<FetchedBlob> = rt.block_on(async {
            // Probe the first part. A ranged GET returns the TOTAL object size
            // via Content-Range (object_store sets meta.size), so this single
            // request serves as both the conditional revalidation and the size
            // probe. If the origin ignores Range it replies with the full body
            // and we store it as one part.
            let probe = OsGetOptions {
                range: Some(GetRange::Bounded(0..part_size)),
                if_none_match: conditional_etag.clone(),
                ..OsGetOptions::default()
            };

            let first = match store.get_opts(&path, probe).await {
                Err(object_store::Error::NotModified { .. }) => return Ok(None),
                Ok(r) => r,
                Err(e) => {
                    return Err(CliError::Other(format!(
                        "get: fetching {source} failed: {e}"
                    )));
                },
            };
            let total = first.meta.size;
            let etag = first.meta.e_tag.clone();
            let last_modified = first.meta.last_modified.to_rfc2822();
            // Guard the remaining range fetches against the object changing
            // mid-download (the store returns a precondition error).
            let if_match = etag.clone();
            let first_bytes = first
                .bytes()
                .await
                .map_err(|e| CliError::Other(format!("get: reading {source} failed: {e}")))?;

            let mut sink = IngestSink::for_source(root, opts.compression, source)?;
            let have = first_bytes.len() as u64;
            sink.write(&first_bytes)?;
            drop(first_bytes);

            // Fetch any outstanding ranges concurrently, but consume them IN
            // ORDER (buffered) so the blob reassembles correctly while memory
            // stays bounded to ≈ concurrency * part_size.
            if have < total {
                // Generate the outstanding byte-ranges LAZILY (via from_fn,
                // consumed by `buffered`) so the range list never materializes
                // in full — it would be `total / part_size` entries for a huge
                // object with a tiny part size. `saturating_add` avoids overflow
                // on a hostile total.
                let mut next = have;
                let range_iter = std::iter::from_fn(move || {
                    if next >= total {
                        return None;
                    }
                    let start = next;
                    let end = start.saturating_add(part_size).min(total);
                    next = end;
                    Some(start..end)
                });
                let mut parts = futures_util::stream::iter(range_iter.map(|r| {
                    let store = Arc::clone(&store);
                    let path = path.clone();
                    let if_match = if_match.clone();
                    async move {
                        let o = OsGetOptions {
                            range: Some(GetRange::Bounded(r)),
                            if_match,
                            ..OsGetOptions::default()
                        };
                        store.get_opts(&path, o).await?.bytes().await
                    }
                }))
                .buffered(concurrency);

                while let Some(chunk) = parts.next().await {
                    let chunk = chunk.map_err(|e| {
                        CliError::Other(format!("get: reading {source} failed: {e}"))
                    })?;
                    sink.write(&chunk)?;
                }
            }

            let (b3, size_compressed, size_uncompressed, inner_ext) = sink.finish(root)?;
            Ok(Some((
                b3,
                size_compressed,
                size_uncompressed,
                etag,
                Some(last_modified),
                inner_ext,
            )))
        })?;

        match fetched {
            // Not-Modified: just refresh the freshness clock on the existing entry.
            None => {
                let mut entry = existing.ok_or_else(|| {
                    CliError::Other(format!(
                        "get: {source} returned not-modified but no cached entry exists"
                    ))
                })?;
                entry.meta.downloaded_at = unix_now();
                let json = serde_json::to_vec(&entry).map_err(|e| {
                    CliError::Other(format!("get: failed to serialize cache entry: {e}"))
                })?;
                atomic_write(&entry_path(root, &kh), &json)?;
            },
            // Fresh bytes: the streaming sink already stored the content-addressed
            // blob; record the entry metadata pointing at it.
            Some((b3, size_compressed, size_uncompressed, etag, last_modified, inner_ext)) => {
                let entry = StoredEntry {
                    meta: CacheEntry {
                        logical_name: name.clone(),
                        cache_key: cache_key.clone(),
                        source_uri: source.to_string(),
                        resolved_uri: source.to_string(),
                        blake3: b3,
                        etag,
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
                        cloud_identity: identity.clone(),
                        ckan_api_url: None,
                        inner_ext,
                        sniffed: None,
                    },
                };
                write_entry(root, &entry)?;
            },
        }

        // Bind the requested name to this entry and reclaim an orphaned previous
        // target (the same two-level alias bookkeeping as `get_resource`).
        let prev_target = alias_keyhash(root, &name)?;
        bind_alias(root, &name, &kh)?;
        if let Some(old) = prev_target
            && old != kh
            && aliases_pointing_to(root, &old).is_empty()
        {
            delete_entry_by_keyhash(root, &old);
        }

        let mut entry = load_entry_at(&entry_path(root, &kh))?;
        ensure_indexed(root, &mut entry)?;
        let mut meta = entry.meta;
        meta.logical_name = name;
        Ok(meta)
    }

    /// Stream a fully-resolved `http(s)` URL into the cache. Replaces the former
    /// http-cache middleware path with a unified conditional-revalidation +
    /// streaming/ranged downloader that mirrors `ingest_cloud`: the first-part
    /// ranged GET doubles as the conditional revalidation (ETag / Last-Modified)
    /// AND the size probe (a 206's `Content-Range` carries the total), so large
    /// objects stream into a `BlobSink` as parallel in-order byte-ranges (bounded
    /// memory) and small ones in a single streamed pass. Freshness is
    /// ETag/Last-Modified based — qsv's `dc:` TTL/RefreshPolicy governs staleness,
    /// so RFC 9111 `Cache-Control` max-age is intentionally not honored.
    fn ingest_http(
        opts: &GetOptions,
        root: &Path,
        final_url: &str,
        auth_token: Option<&str>,
        ckan_resource_hash: Option<String>,
        is_ckan: bool,
    ) -> CliResult<CacheEntry> {
        use futures_util::stream::StreamExt;
        use reqwest::{
            StatusCode,
            header::{
                ACCEPT_RANGES, AUTHORIZATION, CONTENT_RANGE, ETAG, IF_MATCH, IF_MODIFIED_SINCE,
                IF_NONE_MATCH, IF_UNMODIFIED_SINCE, LAST_MODIFIED, RANGE,
            },
        };

        let cache_key = format!("HTTP:{}", opts.source);
        let kh = keyhash(&cache_key);
        let name = opts.name.clone().unwrap_or_else(|| derive_name(final_url));

        let mut existing = load_entry_at(&entry_path(root, &kh)).ok();

        // --refresh never: serve the cached copy without touching the origin.
        if opts.refresh_policy == RefreshPolicy::Never
            && let Some(mut entry) = existing.take()
        {
            bind_alias(root, &name, &kh)?;
            ensure_indexed(root, &mut entry)?;
            let mut meta = entry.meta;
            meta.logical_name = name;
            return Ok(meta);
        }

        // --force / --refresh always -> unconditional; otherwise revalidate with
        // the stored ETag (If-None-Match) and/or Last-Modified (If-Modified-Since).
        let unconditional = opts.force || opts.refresh_policy == RefreshPolicy::Always;
        let cond_etag = if unconditional {
            None
        } else {
            existing.as_ref().and_then(|e| e.meta.etag.clone())
        };
        let cond_lastmod = if unconditional {
            None
        } else {
            existing.as_ref().and_then(|e| e.meta.last_modified.clone())
        };

        let part_size = env_u64("QSV_GET_PART_SIZE", DEFAULT_PART_SIZE).max(1);
        let concurrency =
            env_u64("QSV_GET_CONCURRENCY", DEFAULT_DL_CONCURRENCY).clamp(1, 64) as usize;

        let client = util::create_reqwest_async_client(
            None,
            opts.timeout_secs,
            Some(final_url.to_string()),
        )?;
        let rt = tokio::runtime::Runtime::new()?;

        // Stream a whole response body into the sink, returning the byte count.
        async fn drain(
            sink: &mut IngestSink,
            resp: reqwest::Response,
            url: &str,
        ) -> CliResult<u64> {
            use futures_util::stream::StreamExt;
            let mut have = 0u64;
            let mut stream = resp.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk
                    .map_err(|e| CliError::Other(format!("get: reading {url} failed: {e}")))?;
                have += chunk.len() as u64;
                sink.write(&chunk)?;
            }
            Ok(have)
        }

        // Parse a `Content-Range` value ("bytes START-END/TOTAL", or a "*" total
        // for an unknown size) into (start, end, total). Used to validate that
        // each follow-up range response actually carries the bytes we asked for.
        fn parse_content_range(cr: &str) -> Option<(u64, u64, Option<u64>)> {
            let rest = cr.trim().strip_prefix("bytes ")?.trim();
            let (range, total) = rest.split_once('/')?;
            let (start, end) = range.split_once('-')?;
            let start = start.trim().parse::<u64>().ok()?;
            let end = end.trim().parse::<u64>().ok()?;
            let total = total.trim();
            let total = if total == "*" {
                None
            } else {
                Some(total.parse::<u64>().ok()?)
            };
            Some((start, end, total))
        }

        // None => Not-Modified; Some => (blake3, compressed, uncompressed, etag, last_modified).
        let fetched: Option<FetchedBlob> = rt.block_on(async {
            // First-part ranged probe (also the conditional revalidation).
            let mut req = client
                .get(final_url)
                .header(RANGE, format!("bytes=0-{}", part_size - 1));
            if let Some(tok) = auth_token {
                req = req.header(AUTHORIZATION, tok);
            }
            if let Some(e) = &cond_etag {
                req = req.header(IF_NONE_MATCH, e);
            }
            if let Some(lm) = &cond_lastmod {
                req = req.header(IF_MODIFIED_SINCE, lm);
            }
            let resp = req
                .send()
                .await
                .map_err(|e| CliError::Other(format!("get: request to {final_url} failed: {e}")))?;

            let status = resp.status();
            if status == StatusCode::NOT_MODIFIED {
                return Ok(None);
            }
            if !status.is_success() {
                return Err(CliError::Other(format!(
                    "get: {final_url} returned {status}"
                )));
            }

            let headers = resp.headers().clone();
            let etag = header_str(&headers, ETAG);
            let last_modified = header_str(&headers, LAST_MODIFIED);
            let ranges_supported = status == StatusCode::PARTIAL_CONTENT
                && header_str(&headers, ACCEPT_RANGES)
                    .unwrap_or_default()
                    .to_ascii_lowercase()
                    .contains("bytes");
            // Total object size from Content-Range ("bytes 0-N/TOTAL") on a 206.
            let total: Option<u64> = if status == StatusCode::PARTIAL_CONTENT {
                header_str(&headers, CONTENT_RANGE)
                    .and_then(|cr| cr.rsplit('/').next().map(|t| t.trim().to_string()))
                    .and_then(|t| t.parse::<u64>().ok())
            } else {
                None
            };

            let mut sink = IngestSink::for_source(root, opts.compression, final_url)?;

            if ranges_supported && let Some(total) = total {
                // 206 with a known total: stream the first part, then fetch the
                // remaining bytes as parallel, in-order ranges (bounded memory).
                let have = drain(&mut sink, resp, final_url).await?;
                if have < total {
                    // Guard the follow-up ranges against the object changing
                    // mid-download. `If-Match` mandates STRONG comparison, so a
                    // weak validator (`W/"..."`) must NOT be sent there — a
                    // compliant origin answers 412. Use the strong ETag when we
                    // have one, else fall back to `If-Unmodified-Since` from the
                    // Last-Modified, else rely on the per-range Content-Range /
                    // length validation below.
                    let strong_etag = etag
                        .clone()
                        .filter(|e| !e.starts_with("W/") && !e.starts_with("w/"));
                    let guard_lastmod = if strong_etag.is_some() {
                        None
                    } else {
                        last_modified.clone()
                    };
                    // Generate the (start, end) byte-ranges LAZILY so the list
                    // never materializes in full — it would be `total /
                    // part_size` entries for a huge object with a tiny part size.
                    // `saturating_add` avoids overflow on a hostile total.
                    let mut next = have;
                    let range_iter = std::iter::from_fn(move || {
                        if next >= total {
                            return None;
                        }
                        let s = next;
                        let e = s.saturating_add(part_size).min(total);
                        next = e;
                        Some((s, e))
                    });
                    let mut parts = futures_util::stream::iter(range_iter.map(|(s, e)| {
                        let client = client.clone();
                        let url = final_url.to_string();
                        let auth = auth_token.map(ToString::to_string);
                        let if_match = strong_etag.clone();
                        let if_unmod = guard_lastmod.clone();
                        async move {
                            let mut req = client
                                .get(&url)
                                .header(RANGE, format!("bytes={s}-{}", e - 1));
                            if let Some(tok) = &auth {
                                req = req.header(AUTHORIZATION, tok);
                            }
                            if let Some(im) = &if_match {
                                req = req.header(IF_MATCH, im);
                            } else if let Some(lm) = &if_unmod {
                                req = req.header(IF_UNMODIFIED_SINCE, lm);
                            }
                            let resp = req.send().await.map_err(|err| {
                                CliError::Other(format!("get: reading {url} failed: {err}"))
                            })?;
                            // Each follow-up range MUST come back as 206 with a
                            // Content-Range that matches exactly what we asked
                            // for and a body of the requested length. Otherwise a
                            // 200 full body, a wrong slice, or a short read would
                            // be stitched in as if it were this range, silently
                            // corrupting the reassembled blob.
                            let st = resp.status();
                            if st != StatusCode::PARTIAL_CONTENT {
                                return Err(CliError::Other(format!(
                                    "get: range {s}-{} of {url} returned {st}, expected 206 \
                                     Partial Content",
                                    e - 1
                                )));
                            }
                            let cr =
                                header_str(resp.headers(), CONTENT_RANGE).ok_or_else(|| {
                                    CliError::Other(format!(
                                        "get: range {s}-{} of {url} is missing a Content-Range \
                                         header",
                                        e - 1
                                    ))
                                })?;
                            match parse_content_range(&cr) {
                                Some((rs, re, rtot))
                                    if rs == s
                                        && re == e - 1
                                        && rtot.is_none_or(|t| t == total) => {},
                                _ => {
                                    return Err(CliError::Other(format!(
                                        "get: range {s}-{} of {url} returned a mismatched \
                                         Content-Range '{cr}' (object changed mid-download?)",
                                        e - 1
                                    )));
                                },
                            }
                            let body = resp.bytes().await.map_err(|err| {
                                CliError::Other(format!("get: reading {url} failed: {err}"))
                            })?;
                            if body.len() as u64 != e - s {
                                return Err(CliError::Other(format!(
                                    "get: range {s}-{} of {url} returned {} bytes, expected {}",
                                    e - 1,
                                    body.len(),
                                    e - s
                                )));
                            }
                            Ok::<_, CliError>(body)
                        }
                    }))
                    .buffered(concurrency);
                    while let Some(chunk) = parts.next().await {
                        let chunk = chunk?;
                        sink.write(&chunk)?;
                    }
                }
            } else if status == StatusCode::PARTIAL_CONTENT {
                // 206 but the total is unknown (Content-Range "*"): the first
                // part alone is incomplete, so re-fetch the whole object in one
                // streamed pass.
                drop(resp);
                let mut req = client.get(final_url);
                if let Some(tok) = auth_token {
                    req = req.header(AUTHORIZATION, tok);
                }
                let full = req.send().await.map_err(|e| {
                    CliError::Other(format!("get: request to {final_url} failed: {e}"))
                })?;
                if !full.status().is_success() {
                    return Err(CliError::Other(format!(
                        "get: {final_url} returned {}",
                        full.status()
                    )));
                }
                drain(&mut sink, full, final_url).await?;
            } else {
                // 200 (server ignored Range / no range support): stream the full
                // body straight into the sink.
                drain(&mut sink, resp, final_url).await?;
            }

            let (b3, size_compressed, size_uncompressed, inner_ext) = sink.finish(root)?;
            Ok(Some((
                b3,
                size_compressed,
                size_uncompressed,
                etag,
                last_modified,
                inner_ext,
            )))
        })?;

        match fetched {
            // Not-Modified: refresh the freshness clock on the existing entry.
            None => {
                let mut entry = existing.ok_or_else(|| {
                    CliError::Other(format!(
                        "get: {final_url} returned not-modified but no cached entry exists"
                    ))
                })?;
                entry.meta.downloaded_at = unix_now();
                let json = serde_json::to_vec(&entry).map_err(|e| {
                    CliError::Other(format!("get: failed to serialize cache entry: {e}"))
                })?;
                atomic_write(&entry_path(root, &kh), &json)?;
            },
            // Fresh bytes: the streaming sink already stored the content-addressed
            // blob; record the entry metadata pointing at it.
            Some((b3, size_compressed, size_uncompressed, etag, last_modified, inner_ext)) => {
                let entry = StoredEntry {
                    meta: CacheEntry {
                        logical_name: name.clone(),
                        cache_key: cache_key.clone(),
                        source_uri: opts.source.clone(),
                        resolved_uri: final_url.to_string(),
                        blake3: b3,
                        etag,
                        last_modified,
                        ckan_resource_hash: ckan_resource_hash.clone(),
                        size_compressed,
                        size_uncompressed,
                        record_count: None,
                        indexed: false,
                        downloaded_at: unix_now(),
                        ttl_secs: opts.ttl_secs,
                        refresh_policy: opts.refresh_policy,
                        compression: opts.compression,
                        cloud_identity: Vec::new(),
                        // Persist the CKAN API base only for actual ckan:// sources.
                        ckan_api_url: if is_ckan {
                            opts.ckan_api_url.clone()
                        } else {
                            None
                        },
                        inner_ext,
                        sniffed: None,
                    },
                };
                write_entry(root, &entry)?;
            },
        }

        // Bind the requested name to this entry and reclaim an orphaned previous
        // target (the same two-level alias bookkeeping as `get_resource`).
        let prev_target = alias_keyhash(root, &name)?;
        bind_alias(root, &name, &kh)?;
        if let Some(old) = prev_target
            && old != kh
            && aliases_pointing_to(root, &old).is_empty()
        {
            delete_entry_by_keyhash(root, &old);
        }

        let mut entry = load_entry_at(&entry_path(root, &kh))?;
        ensure_indexed(root, &mut entry)?;
        let mut meta = entry.meta;
        meta.logical_name = name;
        Ok(meta)
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
            if is_cloud_scheme(&opts.source) {
                #[cfg(feature = "get_cloud")]
                {
                    return ingest_cloud(opts, &root, &opts.source);
                }
                #[cfg(not(feature = "get_cloud"))]
                {
                    return Err(CliError::Other(format!(
                        "get: cloud source '{}' requires cloud support. Rebuild qsv with \
                         `--features get_cloud` (already included in the distrib, qsvmcp and \
                         datapusher_plus builds).",
                        opts.source
                    )));
                }
            }
            let local_path = Path::new(&opts.source);
            if local_path.exists() {
                return ingest_local(opts, &root, local_path);
            }
            return Err(CliError::Other(format!(
                "get: unsupported source '{}'. Supported: local file, http(s)://, dathere://, \
                 ckan://, and (with the get_cloud feature) s3://, gs:// and az://. (sftp is \
                 planned for a later release.)",
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

        // Stream the resolved http(s) URL into the cache via a conditional,
        // ranged downloader (replaces the former http-cache middleware).
        ingest_http(
            opts,
            &root,
            &final_url,
            auth_token.as_deref(),
            ckan_hash,
            resolved.is_ckan,
        )
    }

    // ---- cache-bypassing CSV preview (`--sample`/`--offset`/`--random`) ----

    /// Bytes sniffed (and used to extract the header) from the start of a source.
    const PREVIEW_HEAD_BYTES: u64 = 256 * 1024;
    /// Per-probe window for `--random` ranged reads.
    const PREVIEW_RANDOM_WINDOW: u64 = 128 * 1024;
    /// Max `--random` probe attempts is `sample * this` (records may straddle
    /// windows, so allow several tries per wanted record before giving up).
    const PREVIEW_RANDOM_ATTEMPTS_MULT: u64 = 20;
    /// Cap on bytes pulled while accumulating windows for a cloud `--offset`/
    /// first-N preview, so a tiny `--sample` never drags the whole object.
    #[cfg(feature = "get_cloud")]
    const PREVIEW_MAX_FETCH: u64 = 256 * 1024 * 1024;

    /// Options for a cache-bypassing CSV preview. Unlike `GetOptions`, this never
    /// touches the cache: rows are streamed straight to `--output`/stdout.
    pub struct PreviewOptions {
        pub source:       String,
        pub sample:       u64,
        pub offset_mb:    Option<u64>,
        pub random:       bool,
        pub ckan_api_url: Option<String>,
        pub ckan_token:   Option<String>,
        pub timeout_secs: u16,
        #[cfg_attr(not(feature = "get_cloud"), allow(dead_code))]
        pub cloud_opts:   Vec<String>,
    }

    /// Total object size from a `Content-Range` value ("bytes START-END/TOTAL").
    /// None when the total is unknown ("*").
    fn parse_total_from_content_range(cr: &str) -> Option<u64> {
        let rest = cr.trim().strip_prefix("bytes ")?.trim();
        let (_, total) = rest.split_once('/')?;
        let total = total.trim();
        if total == "*" {
            None
        } else {
            total.parse().ok()
        }
    }

    /// Sniff the head buffer for its delimiter and extract the header record,
    /// returning `(delimiter, header_record)`. Defaults to comma when sniffing
    /// fails so a preview still emits something usable for an unrecognized source.
    fn sniff_preview_head(head: &[u8]) -> (u8, Option<csv::ByteRecord>) {
        // Use the sniffer only for the delimiter. Header detection on a small,
        // all-string sample is unreliable (csv-nose often reports "no header"
        // for text-only columns), so — matching qsv's default — a preview always
        // treats the first row as the header and re-attaches it.
        let delimiter = sniff_dialect(head).map_or(b',', |s| s.delimiter as u8);
        let header = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true)
            .has_headers(true)
            .from_reader(head)
            .byte_headers()
            .ok()
            .cloned();
        (delimiter, header)
    }

    /// Open the preview output sink: a file, or stdout for `-`/None.
    fn preview_writer(output: Option<&str>) -> CliResult<Box<dyn Write>> {
        let w: Box<dyn Write> = match output {
            Some(p) if p != "-" => Box::new(BufWriter::new(fs::File::create(p)?)),
            _ => Box::new(BufWriter::new(std::io::stdout())),
        };
        Ok(w)
    }

    /// Emit a header (explicit, or read from the stream) plus the first `take`
    /// data records parsed from `data`.
    fn emit_preview<R: Read>(
        data: R,
        delimiter: u8,
        data_has_header: bool,
        explicit_header: Option<&csv::ByteRecord>,
        take: u64,
        output: Option<&str>,
    ) -> CliResult<u64> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true)
            .has_headers(data_has_header)
            .from_reader(data);
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(preview_writer(output)?);
        if let Some(h) = explicit_header {
            wtr.write_byte_record(h)?;
        } else if data_has_header {
            let h = rdr.byte_headers()?.clone();
            wtr.write_byte_record(&h)?;
        }
        let mut n = 0u64;
        let mut rec = csv::ByteRecord::new();
        while n < take && rdr.read_byte_record(&mut rec)? {
            wtr.write_byte_record(&rec)?;
            n += 1;
        }
        wtr.flush()?;
        Ok(n)
    }

    /// Reservoir-sample (Algorithm R) `take` records from a single streaming pass.
    /// Used for `--random` when the source does not support ranged reads.
    fn emit_preview_reservoir<R: Read>(
        data: R,
        delimiter: u8,
        data_has_header: bool,
        take: u64,
        output: Option<&str>,
    ) -> CliResult<u64> {
        use rand::RngExt;
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true)
            .has_headers(data_has_header)
            .from_reader(data);
        let header = if data_has_header {
            Some(rdr.byte_headers()?.clone())
        } else {
            None
        };
        let mut rng = rand::rng();
        let mut reservoir: Vec<csv::ByteRecord> = Vec::new();
        let mut seen = 0u64;
        let mut rec = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut rec)? {
            if (reservoir.len() as u64) < take {
                reservoir.push(rec.clone());
            } else if take > 0 {
                let j = rng.random_range(0..=seen);
                if j < take {
                    reservoir[j as usize] = rec.clone();
                }
            }
            seen += 1;
        }
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(preview_writer(output)?);
        if let Some(h) = &header {
            wtr.write_byte_record(h)?;
        }
        for r in &reservoir {
            wtr.write_byte_record(r)?;
        }
        wtr.flush()?;
        Ok(reservoir.len() as u64)
    }

    /// Skip the (likely partial) first line of a ranged read so parsing starts on
    /// a clean record boundary.
    fn skip_partial_line<R: std::io::BufRead>(r: &mut R) -> CliResult<()> {
        let mut discard = Vec::new();
        r.read_until(b'\n', &mut discard)?;
        Ok(())
    }

    /// Dispatch a preview to the right backend (mirrors `get_resource`).
    pub fn preview_resource(opts: &PreviewOptions, output: Option<&str>) -> CliResult<()> {
        let resolved = resolve_uri_prefix(&opts.source, opts.ckan_api_url.as_deref());
        let is_http = resolved.url.to_ascii_lowercase().starts_with("http");

        if !is_http {
            if is_cloud_scheme(&opts.source) {
                #[cfg(feature = "get_cloud")]
                {
                    return preview_cloud(&opts.source, opts, output);
                }
                #[cfg(not(feature = "get_cloud"))]
                {
                    return Err(CliError::Other(format!(
                        "get: cloud source '{}' requires cloud support. Rebuild qsv with \
                         `--features get_cloud`.",
                        opts.source
                    )));
                }
            }
            let local_path = Path::new(&opts.source);
            if local_path.exists() {
                return preview_local(local_path, opts, output);
            }
            return Err(CliError::Other(format!(
                "get: unsupported source '{}' for preview. Supported: local file, http(s)://, \
                 dathere://, ckan://, and (with get_cloud) s3://, gs://, az://.",
                opts.source
            )));
        }

        let client = util::create_reqwest_blocking_client(
            None,
            opts.timeout_secs,
            Some(resolved.url.clone()),
        )?;
        let (final_url, auth) = if resolved.is_ckan {
            let ckan = resolve_ckan_resource(
                &client,
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
            (ckan.data_url, auth)
        } else {
            (resolved.url.clone(), None)
        };
        preview_http(&client, &final_url, auth.as_deref(), opts, output)
    }

    fn preview_local(path: &Path, opts: &PreviewOptions, output: Option<&str>) -> CliResult<()> {
        use std::io::{Read, Seek, SeekFrom};

        let mut head = Vec::new();
        fs::File::open(path)?
            .take(PREVIEW_HEAD_BYTES)
            .read_to_end(&mut head)?;
        let (delim, header) = sniff_preview_head(&head);

        if opts.random {
            let f = std::io::BufReader::new(fs::File::open(path)?);
            emit_preview_reservoir(f, delim, true, opts.sample, output)?;
            return Ok(());
        }
        if let Some(mb) = opts.offset_mb {
            let mut f = fs::File::open(path)?;
            f.seek(SeekFrom::Start(mb.saturating_mul(1 << 20)))?;
            let mut br = std::io::BufReader::new(f);
            skip_partial_line(&mut br)?;
            emit_preview(br, delim, false, header.as_ref(), opts.sample, output)?;
            return Ok(());
        }
        let f = std::io::BufReader::new(fs::File::open(path)?);
        emit_preview(f, delim, true, None, opts.sample, output)?;
        Ok(())
    }

    fn preview_http(
        client: &reqwest::blocking::Client,
        url: &str,
        auth: Option<&str>,
        opts: &PreviewOptions,
        output: Option<&str>,
    ) -> CliResult<()> {
        use reqwest::header::{AUTHORIZATION, CONTENT_RANGE, RANGE};

        let http_get = |range: Option<String>| -> CliResult<reqwest::blocking::Response> {
            let mut req = client.get(url);
            if let Some(r) = range {
                req = req.header(RANGE, r);
            }
            if let Some(t) = auth {
                req = req.header(AUTHORIZATION, t);
            }
            Ok(req.send()?.error_for_status()?)
        };

        // Head probe: doubles as the dialect sniff and the range-support detector.
        let probe = http_get(Some(format!("bytes=0-{}", PREVIEW_HEAD_BYTES - 1)))?;
        let supports_range = probe.status() == reqwest::StatusCode::PARTIAL_CONTENT;
        let total = probe
            .headers()
            .get(CONTENT_RANGE)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_total_from_content_range);
        let mut head = Vec::new();
        probe.take(PREVIEW_HEAD_BYTES).read_to_end(&mut head)?;
        let (delim, header) = sniff_preview_head(&head);

        if opts.random {
            if supports_range && let Some(total) = total {
                return preview_http_random(
                    &http_get,
                    delim,
                    header.as_ref(),
                    opts.sample,
                    total,
                    output,
                );
            }
            let resp = http_get(None)?;
            emit_preview_reservoir(
                std::io::BufReader::new(resp),
                delim,
                true,
                opts.sample,
                output,
            )?;
            return Ok(());
        }

        if let Some(mb) = opts.offset_mb {
            if !supports_range {
                return Err(CliError::Other(
                    "get: source does not support HTTP Range requests; --offset is unavailable. \
                     Use --sample without --offset."
                        .to_string(),
                ));
            }
            let off = mb.saturating_mul(1 << 20);
            let resp = http_get(Some(format!("bytes={off}-")))?;
            let mut br = std::io::BufReader::new(resp);
            skip_partial_line(&mut br)?;
            emit_preview(br, delim, false, header.as_ref(), opts.sample, output)?;
            return Ok(());
        }

        let resp = http_get(None)?;
        emit_preview(
            std::io::BufReader::new(resp),
            delim,
            true,
            None,
            opts.sample,
            output,
        )?;
        Ok(())
    }

    /// Approximate uniform-random sampling over a range-capable HTTP source: read
    /// a small window at random offsets, realign to a record boundary, and take
    /// the first whole record from each, until `take` records are collected.
    fn preview_http_random(
        http_get: &dyn Fn(Option<String>) -> CliResult<reqwest::blocking::Response>,
        delimiter: u8,
        header: Option<&csv::ByteRecord>,
        take: u64,
        total: u64,
        output: Option<&str>,
    ) -> CliResult<()> {
        use rand::RngExt;

        let mut rng = rand::rng();
        let mut wtr = csv::WriterBuilder::new()
            .delimiter(delimiter)
            .from_writer(preview_writer(output)?);
        if let Some(h) = header {
            wtr.write_byte_record(h)?;
        }
        let mut got = 0u64;
        let mut attempts = 0u64;
        let max_attempts = take
            .saturating_mul(PREVIEW_RANDOM_ATTEMPTS_MULT)
            .max(take + 8);
        while got < take && attempts < max_attempts {
            attempts += 1;
            let span = total.saturating_sub(1).max(1);
            let off = rng.random_range(0..span);
            let end = (off + PREVIEW_RANDOM_WINDOW).min(total).saturating_sub(1);
            let resp = http_get(Some(format!("bytes={off}-{end}")))?;
            let mut win = Vec::new();
            resp.take(PREVIEW_RANDOM_WINDOW).read_to_end(&mut win)?;
            // Realign past the first (partial) line, then take one whole record.
            let Some(nl) = win.iter().position(|&b| b == b'\n') else {
                continue;
            };
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(delimiter)
                .flexible(true)
                .has_headers(false)
                .from_reader(&win[nl + 1..]);
            let mut rec = csv::ByteRecord::new();
            if rdr.read_byte_record(&mut rec)? && !rec.is_empty() {
                wtr.write_byte_record(&rec)?;
                got += 1;
            }
        }
        wtr.flush()?;
        Ok(())
    }

    #[cfg(feature = "get_cloud")]
    fn preview_cloud(source: &str, opts: &PreviewOptions, output: Option<&str>) -> CliResult<()> {
        use object_store::{GetOptions as OsGetOptions, GetRange, ObjectStore, parse_url_opts};
        use rand::RngExt;
        use url::Url;

        let url = Url::parse(source)
            .map_err(|e| CliError::Other(format!("get: invalid cloud URL '{source}': {e}")))?;
        let all_opts = cloud_opts_for(&opts.cloud_opts);
        let (store, path) = parse_url_opts(&url, all_opts).map_err(|e| {
            CliError::Other(format!("get: cannot open cloud store for '{source}': {e}"))
        })?;
        let rt = tokio::runtime::Runtime::new()?;

        let fetch = |start: u64, end: u64| -> CliResult<Vec<u8>> {
            rt.block_on(async {
                let r = store
                    .get_opts(
                        &path,
                        OsGetOptions {
                            range: Some(GetRange::Bounded(start..end)),
                            ..OsGetOptions::default()
                        },
                    )
                    .await
                    .map_err(|e| CliError::Other(format!("get: fetching {source} failed: {e}")))?;
                let b = r
                    .bytes()
                    .await
                    .map_err(|e| CliError::Other(format!("get: reading {source} failed: {e}")))?;
                Ok::<_, CliError>(b.to_vec())
            })
        };

        // Head probe also yields the total object size.
        let (head, total) = rt.block_on(async {
            let r = store
                .get_opts(
                    &path,
                    OsGetOptions {
                        range: Some(GetRange::Bounded(0..PREVIEW_HEAD_BYTES)),
                        ..OsGetOptions::default()
                    },
                )
                .await
                .map_err(|e| CliError::Other(format!("get: fetching {source} failed: {e}")))?;
            let total = r.meta.size;
            let b = r
                .bytes()
                .await
                .map_err(|e| CliError::Other(format!("get: reading {source} failed: {e}")))?;
            Ok::<_, CliError>((b.to_vec(), total))
        })?;
        let (delim, header) = sniff_preview_head(&head);

        if opts.random {
            let mut rng = rand::rng();
            let mut wtr = csv::WriterBuilder::new()
                .delimiter(delim)
                .from_writer(preview_writer(output)?);
            if let Some(h) = &header {
                wtr.write_byte_record(h)?;
            }
            let mut got = 0u64;
            let mut attempts = 0u64;
            let max_attempts = opts
                .sample
                .saturating_mul(PREVIEW_RANDOM_ATTEMPTS_MULT)
                .max(opts.sample + 8);
            while got < opts.sample && attempts < max_attempts {
                attempts += 1;
                let span = total.saturating_sub(1).max(1);
                let off = rng.random_range(0..span);
                let end = (off + PREVIEW_RANDOM_WINDOW).min(total);
                let win = fetch(off, end)?;
                let Some(nl) = win.iter().position(|&b| b == b'\n') else {
                    continue;
                };
                let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(delim)
                    .flexible(true)
                    .has_headers(false)
                    .from_reader(&win[nl + 1..]);
                let mut rec = csv::ByteRecord::new();
                if rdr.read_byte_record(&mut rec)? && !rec.is_empty() {
                    wtr.write_byte_record(&rec)?;
                    got += 1;
                }
            }
            wtr.flush()?;
            return Ok(());
        }

        // first-N / offset: accumulate bounded windows until enough lines or EOF.
        let start = opts.offset_mb.map_or(0, |mb| mb.saturating_mul(1 << 20));
        let needed_lines = opts.sample + 2;
        let mut buf = Vec::new();
        let mut pos = start;
        while pos < total && (buf.len() as u64) < PREVIEW_MAX_FETCH {
            let end = (pos + DEFAULT_PART_SIZE).min(total);
            let chunk = fetch(pos, end)?;
            if chunk.is_empty() {
                break;
            }
            pos = end;
            buf.extend_from_slice(&chunk);
            if (buf.iter().filter(|&&b| b == b'\n').count() as u64) >= needed_lines {
                break;
            }
        }

        if start > 0 {
            let data = match buf.iter().position(|&b| b == b'\n') {
                Some(nl) => buf[nl + 1..].to_vec(),
                None => Vec::new(),
            };
            emit_preview(
                std::io::Cursor::new(data),
                delim,
                false,
                header.as_ref(),
                opts.sample,
                output,
            )?;
        } else {
            emit_preview(
                std::io::Cursor::new(buf),
                delim,
                true,
                None,
                opts.sample,
                output,
            )?;
        }
        Ok(())
    }

    // ---- glob / directory expansion (multi-fetch) ----

    /// Context for `expand_source` (cloud listing needs the same `--cloud-opt`s).
    pub struct ExpandCtx {
        #[cfg_attr(not(feature = "get_cloud"), allow(dead_code))]
        pub cloud_opts: Vec<String>,
    }

    /// True if `s` ends with a tabular extension we auto-collect from a directory.
    fn is_tabular_path(s: &str) -> bool {
        let lower = s.to_ascii_lowercase();
        [".csv", ".tsv", ".tab", ".ssv"]
            .iter()
            .any(|e| lower.ends_with(e))
    }

    /// True if `s` contains a glob metacharacter (`*`, `?` or `[`).
    fn has_glob_meta(s: &str) -> bool {
        s.contains(['*', '?', '['])
    }

    /// Expand a glob/directory `source` into concrete sources. A non-glob,
    /// non-directory source returns unchanged (as a one-element vec). Local
    /// globs/directories and (with `get_cloud`) cloud globs/prefixes expand to
    /// the matching files/objects; http/ckan/dathere/dc sources pass through.
    #[cfg_attr(not(feature = "get_cloud"), allow(unused_variables))]
    pub fn expand_source(source: &str, ctx: &ExpandCtx) -> CliResult<Vec<String>> {
        if is_cloud_scheme(source) {
            #[cfg(feature = "get_cloud")]
            if source.ends_with('/') || has_glob_meta(source) {
                return expand_cloud(source, ctx);
            }
            return Ok(vec![source.to_string()]);
        }
        // Only local filesystem paths expand; remote schemes pass through.
        let lower = source.to_ascii_lowercase();
        let remote = lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("ckan://")
            || lower.starts_with("dathere://")
            || lower.starts_with("dc:");
        if remote {
            return Ok(vec![source.to_string()]);
        }
        let path = Path::new(source);
        if path.is_dir() {
            return expand_local_dir(path);
        }
        if has_glob_meta(source) {
            return expand_local_glob(source);
        }
        Ok(vec![source.to_string()])
    }

    fn expand_local_dir(dir: &Path) -> CliResult<Vec<String>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(dir)? {
            let p = entry?.path();
            if p.is_file() && is_tabular_path(&p.to_string_lossy()) {
                out.push(p.to_string_lossy().to_string());
            }
        }
        out.sort();
        if out.is_empty() {
            return Err(CliError::Other(format!(
                "get: no tabular files (.csv/.tsv/.tab/.ssv) found in directory '{}'.",
                dir.display()
            )));
        }
        Ok(out)
    }

    fn expand_local_glob(pattern: &str) -> CliResult<Vec<String>> {
        let mut out = Vec::new();
        let paths = glob::glob(pattern)
            .map_err(|e| CliError::Other(format!("get: invalid glob '{pattern}': {e}")))?;
        for p in paths {
            let p = p.map_err(|e| CliError::Other(format!("get: glob error: {e}")))?;
            if p.is_file() {
                out.push(p.to_string_lossy().to_string());
            }
        }
        out.sort();
        if out.is_empty() {
            return Err(CliError::Other(format!(
                "get: no files matched glob '{pattern}'."
            )));
        }
        Ok(out)
    }

    /// List a cloud bucket/prefix and expand a glob (or trailing-slash directory)
    /// into concrete `scheme://host/key` object URLs.
    #[cfg(feature = "get_cloud")]
    fn expand_cloud(source: &str, ctx: &ExpandCtx) -> CliResult<Vec<String>> {
        use futures_util::stream::StreamExt;
        use object_store::{ObjectStore, parse_url_opts, path::Path as OsPath};
        use url::Url;

        let url = Url::parse(source)
            .map_err(|e| CliError::Other(format!("get: invalid cloud URL '{source}': {e}")))?;
        let scheme = url.scheme().to_string();
        let host = url.host_str().unwrap_or_default().to_string();
        let base = format!("{scheme}://{host}");
        let key = url.path().trim_start_matches('/').to_string();

        let all_opts = cloud_opts_for(&ctx.cloud_opts);
        let (store, _path) = parse_url_opts(&url, all_opts).map_err(|e| {
            CliError::Other(format!("get: cannot open cloud store for '{source}': {e}"))
        })?;

        let is_glob = has_glob_meta(&key);
        let pattern = if is_glob {
            Some(
                glob::Pattern::new(&key)
                    .map_err(|e| CliError::Other(format!("get: invalid glob '{source}': {e}")))?,
            )
        } else {
            None
        };
        // List under the literal prefix preceding the first glob metacharacter
        // (or the whole key for a trailing-slash directory).
        let literal: String = key
            .chars()
            .take_while(|c| !matches!(c, '*' | '?' | '['))
            .collect();
        let list_prefix = match literal.rfind('/') {
            Some(i) => literal[..=i].to_string(),
            None if is_glob => String::new(),
            None => literal,
        };

        let rt = tokio::runtime::Runtime::new()?;
        let results = rt.block_on(async {
            let osp = if list_prefix.is_empty() {
                None
            } else {
                Some(OsPath::from(list_prefix.as_str()))
            };
            let mut stream = store.list(osp.as_ref());
            let mut out = Vec::new();
            while let Some(meta) = stream.next().await {
                let meta = meta
                    .map_err(|e| CliError::Other(format!("get: listing {source} failed: {e}")))?;
                let loc = meta.location.as_ref().to_string();
                let keep = match &pattern {
                    Some(p) => p.matches(&loc),
                    None => is_tabular_path(&loc),
                };
                if keep {
                    out.push(format!("{base}/{loc}"));
                }
            }
            out.sort();
            Ok::<_, CliError>(out)
        })?;

        if results.is_empty() {
            return Err(CliError::Other(format!(
                "get: no objects matched '{source}'."
            )));
        }
        Ok(results)
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
                    // Re-resolve a ckan:// entry against the SAME CKAN instance it
                    // was originally fetched from (the persisted `--ckan-api`),
                    // falling back to the ambient env / default only for older
                    // entries that predate this field.
                    ckan_api_url:   entry
                        .meta
                        .ckan_api_url
                        .clone()
                        .or_else(|| std::env::var("QSV_CKAN_API").ok())
                        .or_else(|| Some(DEFAULT_CKAN_API.to_string())),
                    ckan_token:     std::env::var("QSV_CKAN_TOKEN").ok(),
                    timeout_secs:   30,
                    // Replay the persisted store identity (endpoint/region/…) so
                    // a cloud `dc:` refresh rebuilds the SAME store and resolves
                    // to the SAME cache entry. Credentials still come from the
                    // ambient environment (they are never persisted). Empty for
                    // non-cloud entries.
                    cloud_opts:     entry
                        .meta
                        .cloud_identity
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect(),
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
        // The materialized temp name carries a known tabular extension that
        // selects the delimiter (.csv => comma, .tsv/.tab => tab, .ssv =>
        // semicolon). Isolate each extension in its own subdir AND key the
        // stats-cache blob by it, so two aliases of the same bytes with
        // different extensions never share a materialized sidecar (same temp
        // stem) or a durable stats blob — either of which would cross-contaminate
        // schema/frequency results across delimiters.
        let temp_name = tabular_temp_name(
            name,
            &entry.meta.resolved_uri,
            entry.meta.inner_ext.as_deref(),
        );
        let ext = Path::new(&temp_name)
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .unwrap_or_else(|| "csv".to_string());
        let dir = std::env::temp_dir()
            .join("qsv-dc")
            .join(&entry.meta.blake3)
            .join(&ext);
        fs::create_dir_all(&dir)?;
        let csv_path = dir.join(&temp_name);

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

        // Stats-cache (persist-on-use): the `.stats.csv.data.jsonl` sidecar that
        // the "smart" commands build via `util::get_stats_records` to skip
        // recomputation. Capture a freshly-built one into a durable, content-
        // addressed blob so it survives temp-dir cleanup, and restore it when the
        // temp copy is gone or stale (CSV just rewritten). The sidecar location
        // mirrors `get_stats_records` (canonical CSV path + `with_extension`).
        let canonical_csv = fs::canonicalize(&csv_path).unwrap_or_else(|_| csv_path.clone());
        let stats_sidecar = canonical_csv.with_extension("stats.csv.data.jsonl");
        let stats_blob = stats_blob_path(&root, &entry.meta.blake3, &ext);

        // A sidecar is usable only if newer than the CSV — qsv's stats-cache
        // staleness rule (see `util::get_stats_records`).
        let sidecar_fresh = stats_sidecar.exists() && {
            let s = fs::metadata(&stats_sidecar)
                .map(|m| filetime::FileTime::from_last_modification_time(&m));
            let c = fs::metadata(&csv_path)
                .map(|m| filetime::FileTime::from_last_modification_time(&m));
            matches!((s, c), (Ok(s), Ok(c)) if s > c)
        };

        if sidecar_fresh {
            // Capture-once (keyed by content hash). A leaner cache than a later
            // consumer needs is still correct — that consumer just recomputes.
            if !stats_blob.exists()
                && let Ok(bytes) = fs::read(&stats_sidecar)
                && let Ok(zst) = zstd::encode_all(&bytes[..], ZSTD_LEVEL)
            {
                let _ = atomic_write(&stats_blob, &zst);
            }
        } else if stats_blob.exists()
            && let Ok(bytes) = read_zst(&stats_blob)
        {
            // Restore from the durable blob with a fresh mtime so it passes the
            // staleness check (written after the CSV, like the .idx above).
            let _ = atomic_write(&stats_sidecar, &bytes);
            let _ = filetime::set_file_mtime(
                &stats_sidecar,
                filetime::FileTime::from_system_time(SystemTime::now()),
            );
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
    /// List one row per cached **name** (alias), each carrying its entry's
    /// metadata with `logical_name` set to that name. Names that share an entry
    /// (e.g. two `--name`s for one URL) appear as distinct rows.
    pub fn list_entries(cache_dir: &str) -> CliResult<Vec<CacheEntry>> {
        let root = get_root(cache_dir);
        let mut out = Vec::new();
        let Ok(rd) = fs::read_dir(root.join("aliases")) else {
            return Ok(out);
        };
        for de in rd.flatten() {
            let Ok(content) = fs::read_to_string(de.path()) else {
                continue;
            };
            let (kh, name_opt) = parse_alias(&content);
            // The original name is stored in the alias file content.
            let Some(name) = name_opt else { continue };
            if let Ok(e) = load_entry_at(&entry_path(&root, &kh)) {
                let mut meta = e.meta;
                meta.logical_name = name;
                out.push(meta);
            }
        }
        out.sort_by(|a, b| a.logical_name.cmp(&b.logical_name));
        Ok(out)
    }

    /// Remove all cache names, entries and blobs. Returns the number of names
    /// removed.
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

    /// Remove entries last fetched more than `older_than_secs` ago (and all the
    /// names pointing at them). Returns the number of entries removed.
    pub fn prune(cache_dir: &str, older_than_secs: i64) -> CliResult<usize> {
        let root = get_root(cache_dir);
        let now = unix_now();
        let Ok(rd) = fs::read_dir(root.join("entries")) else {
            return Ok(0);
        };
        // Collect first, then delete, so we don't mutate the dir mid-iteration.
        let mut stale_keys = Vec::new();
        for de in rd.flatten() {
            if de.path().extension().is_some_and(|e| e == "json")
                && let Ok(e) = load_entry_at(&de.path())
                // Inclusive: `--older-than 0` prunes everything (age >= 0).
                && now.saturating_sub(e.meta.downloaded_at) >= older_than_secs
                && let Some(kh) = de.path().file_stem().map(|s| s.to_string_lossy().into_owned())
            {
                stale_keys.push(kh);
            }
        }
        for kh in &stale_keys {
            delete_entry_by_keyhash(&root, kh);
        }
        Ok(stale_keys.len())
    }

    /// Load the entry for `name`, apply `mutate`, and persist it back in place.
    /// Errors if `name` is not a cached alias.
    fn update_entry(
        cache_dir: &str,
        name: &str,
        mutate: impl FnOnce(&mut StoredEntry),
    ) -> CliResult<()> {
        let root = get_root(cache_dir);
        let kh = alias_keyhash(&root, name)?.ok_or_else(|| {
            CliError::Other(format!(
                "get: cache entry '{name}' not found. List cached names with `qsv get cache-list`."
            ))
        })?;
        let mut entry = load_entry_at(&entry_path(&root, &kh))?;
        mutate(&mut entry);
        let json = serde_json::to_vec(&entry)
            .map_err(|e| CliError::Other(format!("get: failed to serialize cache entry: {e}")))?;
        atomic_write(&entry_path(&root, &kh), &json)?;
        Ok(())
    }

    /// Set a cache entry's TTL (seconds; -1 = never expire) by name. TTL is an
    /// entry-level property, so this affects every alias pointing at the entry.
    pub fn set_ttl(cache_dir: &str, name: &str, ttl_secs: i64) -> CliResult<()> {
        update_entry(cache_dir, name, |e| e.meta.ttl_secs = ttl_secs)
    }

    /// Set a cache entry's refresh policy by name. Like TTL, this is entry-level
    /// and affects every alias pointing at the entry.
    pub fn set_policy(cache_dir: &str, name: &str, policy: RefreshPolicy) -> CliResult<()> {
        update_entry(cache_dir, name, |e| e.meta.refresh_policy = policy)
    }

    /// Verify cached blob integrity: recompute the BLAKE3 of each alias's stored
    /// (decompressed) blob and compare it to the recorded hash. Returns one
    /// `(name, ok)` pair per alias — `ok == false` means the blob is missing,
    /// unreadable, or its content no longer matches its recorded hash.
    pub fn verify(cache_dir: &str) -> CliResult<Vec<(String, bool)>> {
        let root = get_root(cache_dir);
        let entries = list_entries(cache_dir)?;
        let mut out = Vec::with_capacity(entries.len());
        for e in entries {
            let ok = match read_blob(&root, &e.blake3, e.compression) {
                Ok(body) => blake3::hash(&body).to_hex().to_string() == e.blake3,
                Err(_) => false,
            };
            out.push((e.logical_name, ok));
        }
        Ok(out)
    }
}
