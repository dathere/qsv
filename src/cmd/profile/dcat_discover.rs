//! DCAT-markup discovery for URL inputs.
//!
//! When `qsv profile <URL>` is invoked, sniff the URL for existing
//! DCAT-US v3 metadata so the projection can use the publisher's stated
//! values as a base layer. Per the plan (Phase 4c merge precedence),
//! discovered DCAT beats auto-inferred values from the CSV but loses to
//! a `--initial-context` entry with `force: true`.
//!
//! Three discovery mechanisms, tried in priority order; first one to
//! yield a `dcat:Dataset` JSON-LD object wins:
//!
//!   1. HTTP `Link: rel=describedBy` header on the CSV's HEAD response. Authoritative — the
//!      publisher explicitly points at the metadata.
//!   2. (§5.2) Sibling URLs by convention: `<url>.metadata.json`, `<url>.dcat.json`,
//!      `<dirname>/datapackage.json`, `<host>/.well-known/data.json`.
//!   3. (§5.2) JSON-LD `<script type="application/ld+json">` blocks in the URL's HTML landing page
//!      (one path segment up from the CSV).
//!
//! Each step has a tight default timeout (configurable via
//! `--dcat-discovery-timeout`). All network errors are non-fatal — the
//! discovery falls through to the next mechanism, and if all fail the
//! projection proceeds with no discovered base layer.

use std::time::Duration;

use encoding_rs::{Encoding, UTF_8};
use reqwest::{
    blocking::{Client, Response},
    header::{CONTENT_TYPE, HeaderMap, HeaderValue, LINK},
};
use serde_json::Value;

use crate::util;

/// Best-effort DCAT-markup sniff for `url`. Returns the discovered
/// `dcat:Dataset` JSON-LD object if any mechanism succeeded, else
/// `None`. Network / parse errors are swallowed and treated as
/// "nothing discovered" — this is a non-essential enrichment.
///
/// Uses qsv's shared blocking HTTP client (`util::create_reqwest_blocking_client`)
/// for consistent user-agent / compression / TLS / retry behaviour
/// with the rest of qsv (fetch, validate, describegpt, …). `timeout`
/// is converted from the caller's `Duration` to whole seconds for the
/// shared client; we round up so a fractional-second caller never gets
/// a zero-timeout client.
///
/// Three mechanisms, tried in priority order — first one to yield a
/// `dcat:Dataset` JSON-LD object wins:
///   1. `Link: rel=describedBy` header (authoritative).
///   2. Sibling URLs by convention (`.metadata.json`, `.dcat.json`, `datapackage.json`,
///      `/.well-known/data.json`).
///   3. JSON-LD `<script type="application/ld+json">` blocks in the URL's parent (landing-page)
///      HTML.
pub fn discover(url: &str, timeout: Duration) -> Option<Value> {
    let timeout_secs: u16 = timeout.as_secs().max(1).min(u16::MAX as u64) as u16;
    let client = util::create_reqwest_blocking_client(None, timeout_secs, None).ok()?;

    if let Some(via_link) = discover_via_link_header(&client, url) {
        return Some(via_link);
    }
    if let Some(via_sibling) = discover_via_sibling_urls(&client, url) {
        return Some(via_sibling);
    }
    if let Some(via_html) = discover_via_html_jsonld(&client, url) {
        return Some(via_html);
    }
    None
}

/// Cap on bytes read from a publisher-supplied DCAT-LD document.
/// The Link `rel=describedBy` target is publisher-controlled, so an
/// errant/hostile pointer at a multi-GB resource shouldn't blow up
/// qsv's memory. 4 MiB is well above any realistic DCAT-LD document
/// (Pittsburgh 311's full CKAN package JSON is ~30 KiB) while still
/// being a safe ceiling.
const DCAT_DISCOVERY_MAX_BYTES: u64 = 4 * 1024 * 1024;

/// Pull the `charset` parameter out of a `Content-Type` header.
/// e.g. `text/html; charset=windows-1252` -> `Some("windows-1252")`.
///
/// A deliberate hand parse rather than promoting `mime` to a direct dependency:
/// the grammar needed here is one parameter with optional quoting.
fn charset_from_headers(headers: &HeaderMap) -> Option<String> {
    let content_type = headers.get(CONTENT_TYPE)?.to_str().ok()?;
    content_type
        .split(';')
        .skip(1)
        .filter_map(|param| param.split_once('='))
        .find(|(name, _)| name.trim().eq_ignore_ascii_case("charset"))
        .map(|(_, value)| value.trim().trim_matches('"').to_string())
}

/// Read at most `DCAT_DISCOVERY_MAX_BYTES` from `reader`.
///
/// Deliberately not `Response::text()`: these bodies are publisher-controlled and
/// `.text()` is unbounded, so the cap has to be applied at the reader. That is also
/// why the charset handling below is open-coded instead of delegating to
/// `text_with_charset()`.
fn read_capped(reader: impl std::io::Read) -> Option<Vec<u8>> {
    let mut limited = std::io::Read::take(reader, DCAT_DISCOVERY_MAX_BYTES);
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut limited, &mut buf).ok()?;
    Some(buf)
}

/// Decode a capped body to text, honoring the publisher's declared `charset`.
///
/// This mirrors what `reqwest::Response::text_with_charset` does internally: a
/// `charset` that `encoding_rs` recognizes wins, anything else falls back to UTF-8,
/// and `Encoding::decode` BOM-sniffs (so a UTF-8/UTF-16 BOM both selects the
/// encoding and is stripped, which matters because `serde_json` chokes on a BOM).
///
/// Decoding is lossy in every branch. The predecessor here was `read_to_string`,
/// which hard-errored on two inputs a discovery probe actually meets - a non-UTF-8
/// body, and a *valid* UTF-8 body whose byte cap sliced a multi-byte codepoint -
/// and since every caller swallows the error with `.ok()?`, both silently aborted
/// discovery with no diagnostic.
///
/// What lossiness does NOT fix: a body truncated by the cap is still truncated, and
/// `serde_json::from_str` on it still fails. It only stops truncation from being a
/// *second*, earlier failure mode.
///
/// Note that a declared charset is honored on JSON bodies too, even though JSON is
/// UTF-8 by RFC 8259, so a server sending `application/json; charset=iso-8859-1` is
/// taken at its word. That is deliberate: it is exactly what `text_with_charset`
/// does for every other HTTP body in qsv, and diverging here would be the surprise.
fn decode_capped(buf: &[u8], charset: Option<&str>) -> String {
    let encoding = charset
        .and_then(|label| Encoding::for_label(label.as_bytes()))
        .unwrap_or(UTF_8);
    encoding.decode(buf).0.into_owned()
}

/// Read a capped body and decode it per the response's own `Content-Type` charset.
fn read_capped_lossy(response: Response) -> Option<String> {
    // the charset has to come off the headers before `response` is moved into the reader
    let charset = charset_from_headers(response.headers());
    let buf = read_capped(response)?;
    Some(decode_capped(&buf, charset.as_deref()))
}

/// Issue a HEAD against `url`; if the response carries a
/// `Link: <iri>; rel="describedBy"` header, follow the IRI and parse
/// its body as JSON-LD. Returns the parsed `dcat:Dataset` value, if
/// any. Errors at every step are swallowed (return `None`).
///
/// The followed body is capped at `DCAT_DISCOVERY_MAX_BYTES` because
/// the Link target is publisher-controlled; reading the whole thing
/// via `.text()` would expose qsv to memory exhaustion if it points
/// at a large resource.
fn discover_via_link_header(client: &Client, url: &str) -> Option<Value> {
    let head = client.head(url).send().ok()?;
    let link_header = head.headers().get(LINK)?;
    let describedby_iri = parse_describedby_iri(link_header)?;

    let resolved = resolve_relative(url, &describedby_iri).unwrap_or(describedby_iri);

    let response = client.get(&resolved).send().ok()?;
    let body = read_capped_lossy(response)?;
    let json: Value = serde_json::from_str(&body).ok()?;
    extract_dcat_dataset(&json)
}

/// §5.2: probe sibling URLs by convention for DCAT-shaped metadata.
/// Four candidates (in order):
///   1. `<url>.metadata.json`     — qsv profile's own output naming
///   2. `<url>.dcat.json`         — common DCAT-JSON convention
///   3. `<dirname>/datapackage.json` — Frictionless Data Package spec
///   4. `<host>/.well-known/data.json` — DCAT-US site catalog
///
/// First HTTP 2xx response that parses as JSON-LD and yields a
/// `dcat:Dataset` wins. Bodies are capped at `DCAT_DISCOVERY_MAX_BYTES`
/// because the targets are publisher-controlled.
fn discover_via_sibling_urls(client: &Client, url: &str) -> Option<Value> {
    for candidate in sibling_candidates(url) {
        if let Some(found) = fetch_json_and_extract(client, &candidate) {
            return Some(found);
        }
    }
    None
}

/// Build the four sibling-URL candidates for `url`. Skips entries we
/// can't construct (e.g. a URL with no path → no `dirname`).
///
/// All candidates are built via `url::Url` parsing so that query strings
/// (`?token=...`) and fragments (`#...`) on the input URL do NOT get
/// concatenated into the appended suffix — without this, an input like
/// `snapshot.csv?token=abc` would yield `snapshot.csv?token=abc.metadata.json`,
/// which the server would interpret as a GET on `snapshot.csv` with a
/// `token=abc.metadata.json` query value and return the CSV body, not the
/// sibling JSON.
fn sibling_candidates(url: &str) -> Vec<String> {
    let mut out = Vec::with_capacity(4);

    let Ok(parsed) = url::Url::parse(url) else {
        // Unparseable URL: fall back to textual append. Best-effort —
        // anything that doesn't parse as a URL is unlikely to be fetched
        // successfully anyway, but we keep the slots populated.
        out.push(format!("{url}.metadata.json"));
        out.push(format!("{url}.dcat.json"));
        return out;
    };

    // <url>.metadata.json and <url>.dcat.json: append the suffix to the
    // *path*, and clear query/fragment so they don't leak into the
    // appended portion.
    let base_path = parsed.path().to_string();
    for suffix in [".metadata.json", ".dcat.json"] {
        let mut candidate = parsed.clone();
        candidate.set_path(&format!("{base_path}{suffix}"));
        candidate.set_query(None);
        candidate.set_fragment(None);
        out.push(candidate.to_string());
    }

    // <dirname>/datapackage.json: replace last segment with
    // "datapackage.json". When the URL is just "/" we still want
    // "/datapackage.json".
    let dirname = std::path::Path::new(&base_path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("/");
    let dirname_with_slash = if dirname.ends_with('/') {
        dirname.to_string()
    } else {
        format!("{dirname}/")
    };
    let mut datapackage = parsed.clone();
    datapackage.set_path(&format!("{dirname_with_slash}datapackage.json"));
    datapackage.set_query(None);
    datapackage.set_fragment(None);
    out.push(datapackage.to_string());

    // <host>/.well-known/data.json
    let mut well_known = parsed;
    well_known.set_path("/.well-known/data.json");
    well_known.set_query(None);
    well_known.set_fragment(None);
    out.push(well_known.to_string());

    out
}

/// §5.2: JSON-LD `<script type="application/ld+json">` sniff. Fetches
/// the URL's parent (one path segment up) and looks for embedded
/// JSON-LD blocks. CKAN / data.gov / open-data portals typically host
/// the dataset landing page one level above the raw CSV download.
fn discover_via_html_jsonld(client: &Client, url: &str) -> Option<Value> {
    let parsed = url::Url::parse(url).ok()?;
    let parent_path = std::path::Path::new(parsed.path())
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("/");
    let parent_path = if parent_path.ends_with('/') {
        parent_path.to_string()
    } else {
        format!("{parent_path}/")
    };
    let mut parent = parsed.clone();
    parent.set_path(&parent_path);
    parent.set_query(None);
    parent.set_fragment(None);

    let response = client.get(parent.as_ref()).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let content_type_is_html = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|s| s.to_ascii_lowercase().contains("html"));

    let body = read_capped_lossy(response)?;

    // Cheap looks-like-html sniff so we don't spend cycles scanning a
    // PDF or binary blob that happened to be served with no Content-Type.
    let body_starts_html = {
        let head = body.trim_start();
        let lower = head
            .get(..16)
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();
        lower.starts_with("<!doctype html") || lower.starts_with("<html")
    };
    if !content_type_is_html && !body_starts_html {
        return None;
    }

    extract_jsonld_blocks(&body)
}

/// Scan `html` for `<script type="application/ld+json">` blocks (any
/// attribute order, case-insensitive type), parse each as JSON, and
/// return the first one that yields a `dcat:Dataset`. Pure-string
/// scan — no HTML parser dep — because we only need to locate well-
/// formed script tags; malformed pages just yield no match.
fn extract_jsonld_blocks(html: &str) -> Option<Value> {
    let lower = html.to_ascii_lowercase();
    let mut cursor = 0;
    while let Some(rel_open) = lower[cursor..].find("<script") {
        let abs_open = cursor + rel_open;
        // Find the end of the opening tag.
        let abs_gt = match lower[abs_open..].find('>') {
            Some(i) => abs_open + i,
            None => break,
        };
        let opening_attrs = &lower[abs_open..abs_gt];
        if opening_attrs.contains("application/ld+json") {
            let body_start = abs_gt + 1;
            if let Some(rel_close) = lower[body_start..].find("</script") {
                let body_end = body_start + rel_close;
                let block = html[body_start..body_end].trim();
                if let Ok(parsed) = serde_json::from_str::<Value>(block)
                    && let Some(dataset) = extract_dcat_dataset(&parsed)
                {
                    return Some(dataset);
                }
            }
        }
        cursor = abs_gt + 1;
    }
    None
}

/// Shared GET-and-extract helper for sibling-URL probing.
/// Returns the first `dcat:Dataset` shape it finds, or None on any
/// network/parse/non-2xx failure.
fn fetch_json_and_extract(client: &Client, url: &str) -> Option<Value> {
    let response = client.get(url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body = read_capped_lossy(response)?;
    let json: Value = serde_json::from_str(&body).ok()?;
    extract_dcat_dataset(&json)
}

/// Parse the first `rel="describedBy"` IRI out of an RFC 5988 Link
/// header value. RFC 8288 mandates link-target enclosure in `<...>`
/// and `rel` matching that is case-insensitive on the value.
fn parse_describedby_iri(header: &HeaderValue) -> Option<String> {
    let raw = header.to_str().ok()?;
    for link in raw.split(',') {
        let link = link.trim();
        let Some((iri_part, params)) = link.split_once(';') else {
            continue;
        };
        let iri = iri_part.trim();
        let iri = iri.strip_prefix('<')?.strip_suffix('>')?.trim().to_string();
        // Walk semicolon-separated params looking for rel=describedBy
        // (case-insensitive). Per RFC 8288 params are name=value; value
        // may be DQUOTE'd.
        for p in params.split(';') {
            let p = p.trim();
            let Some((k, v)) = p.split_once('=') else {
                continue;
            };
            if !k.trim().eq_ignore_ascii_case("rel") {
                continue;
            }
            let v = v.trim().trim_matches('"').to_ascii_lowercase();
            // `rel` may carry multiple space-separated tokens
            if v.split_whitespace().any(|tok| tok == "describedby") {
                return Some(iri);
            }
        }
    }
    None
}

/// Resolve a possibly-relative IRI against the URL we sniffed from.
/// Returns the absolute form, or `None` if neither input parses.
fn resolve_relative(base: &str, maybe_relative: &str) -> Option<String> {
    let base_u = url::Url::parse(base).ok()?;
    base_u.join(maybe_relative).ok().map(|u| u.to_string())
}

/// Pull a `dcat:Dataset` object out of a JSON-LD document. Accepts
/// either:
///   * a single object whose `@type` mentions `dcat:Dataset`
///   * an `@graph` array containing one
///   * a bare object that already looks dataset-shaped (has `dct:title` or `dcat:keyword`) —
///     best-effort fallback for non-conforming publishers
fn extract_dcat_dataset(doc: &Value) -> Option<Value> {
    if let Some(arr) = doc.get("@graph").and_then(|v| v.as_array()) {
        for entry in arr {
            if is_dcat_dataset(entry) {
                return Some(entry.clone());
            }
        }
    }
    if is_dcat_dataset(doc) {
        return Some(doc.clone());
    }
    // Fallback: looks dataset-shaped even without @type.
    if doc.get("dct:title").is_some() || doc.get("dcat:keyword").is_some() {
        return Some(doc.clone());
    }
    None
}

fn is_dcat_dataset(v: &Value) -> bool {
    // The full IRI form below uses the http:// scheme because the W3C
    // DCAT namespace is published with it (https://www.w3.org/ns/dcat
    // resolves the same docs but the canonical type IRI is the http
    // form). DevSkim suppression for rule DS137138 (non-TLS URL).
    let Some(ty) = v.get("@type") else {
        return false;
    };
    if let Some(s) = ty.as_str() {
        return s.eq_ignore_ascii_case("dcat:Dataset")
            || s.eq_ignore_ascii_case("http://www.w3.org/ns/dcat#Dataset"); // DevSkim: ignore DS137138
    }
    if let Some(arr) = ty.as_array() {
        return arr.iter().any(|t| {
            t.as_str().is_some_and(|s| {
                s.eq_ignore_ascii_case("dcat:Dataset")
                    || s.eq_ignore_ascii_case("http://www.w3.org/ns/dcat#Dataset") // DevSkim: ignore DS137138
            })
        });
    }
    false
}

#[cfg(test)]
mod tests {
    use reqwest::header::HeaderValue;
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_describedby_simple() {
        let h = HeaderValue::from_static(r#"<https://x/meta.json>; rel="describedBy""#);
        assert_eq!(
            parse_describedby_iri(&h),
            Some("https://x/meta.json".to_string())
        );
    }

    #[test]
    fn parse_describedby_case_insensitive() {
        let h = HeaderValue::from_static(r#"<https://x/meta.json>; REL="DescribedBy""#);
        assert_eq!(
            parse_describedby_iri(&h),
            Some("https://x/meta.json".to_string())
        );
    }

    #[test]
    fn parse_describedby_among_multiple_links() {
        let h = HeaderValue::from_static(
            r#"<https://x/canon>; rel="canonical", <https://x/m.json>; rel="describedBy", <https://x/up>; rel="up""#,
        );
        assert_eq!(
            parse_describedby_iri(&h),
            Some("https://x/m.json".to_string())
        );
    }

    #[test]
    fn parse_describedby_multi_token_rel() {
        // RFC 8288 allows multiple space-separated rel tokens.
        let h = HeaderValue::from_static(r#"<https://x/m.json>; rel="alternate describedBy""#);
        assert_eq!(
            parse_describedby_iri(&h),
            Some("https://x/m.json".to_string())
        );
    }

    #[test]
    fn parse_describedby_returns_none_when_absent() {
        let h = HeaderValue::from_static(r#"<https://x/c>; rel="canonical""#);
        assert_eq!(parse_describedby_iri(&h), None);
    }

    #[test]
    fn resolve_relative_handles_absolute_and_relative() {
        assert_eq!(
            resolve_relative("https://x.gov/dir/data.csv", "meta.json"),
            Some("https://x.gov/dir/meta.json".to_string())
        );
        assert_eq!(
            resolve_relative("https://x.gov/dir/data.csv", "/meta.json"),
            Some("https://x.gov/meta.json".to_string())
        );
        assert_eq!(
            resolve_relative("https://x.gov/dir/data.csv", "https://other.gov/m.json"),
            Some("https://other.gov/m.json".to_string())
        );
    }

    // ---- capped read + charset-aware decode -------------------------------

    #[test]
    fn charset_from_headers_reads_the_content_type_parameter() {
        let mut h = HeaderMap::new();
        h.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=windows-1252"),
        );
        assert_eq!(charset_from_headers(&h), Some("windows-1252".to_string()));
    }

    #[test]
    fn charset_from_headers_tolerates_quotes_case_and_spacing() {
        let mut h = HeaderMap::new();
        h.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(r#"text/html ; Charset = "ISO-8859-1""#),
        );
        assert_eq!(charset_from_headers(&h), Some("ISO-8859-1".to_string()));
    }

    #[test]
    fn charset_from_headers_is_none_without_a_parameter() {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        assert_eq!(charset_from_headers(&h), None);
        assert_eq!(charset_from_headers(&HeaderMap::new()), None);
    }

    #[test]
    fn decode_capped_honors_a_declared_legacy_charset() {
        // The whole point of the fix: "Café" in windows-1252 is 0xE9, which is not
        // valid UTF-8. With the publisher's declaration honored it round-trips;
        // without it, the title silently became "Caf<U+FFFD>".
        let src: &[u8] = &[b'C', b'a', b'f', 0xE9];
        assert_eq!(decode_capped(src, Some("windows-1252")), "Café");
        assert_eq!(decode_capped(src, None), "Caf\u{FFFD}");
    }

    #[test]
    fn decode_capped_ignores_an_unusable_charset_label() {
        // Encoding::for_label() returns None for junk; we fall back to UTF-8 rather
        // than guessing. windows-1252 bytes are used so the assertion discriminates -
        // an implementation that guessed latin-1 would yield "Café", not U+FFFD, and
        // an ASCII fixture would pass under either.
        let src: &[u8] = &[b'C', b'a', b'f', 0xE9];
        assert_eq!(decode_capped(src, Some("not-an-encoding")), "Caf\u{FFFD}");
    }

    #[test]
    fn decode_capped_strips_a_utf8_bom() {
        // serde_json::from_str fails on a leading U+FEFF, so the BOM must go.
        let mut src = vec![0xEF, 0xBB, 0xBF];
        src.extend_from_slice(br#"{"@type":"dcat:Dataset"}"#);
        assert_eq!(decode_capped(&src, None), r#"{"@type":"dcat:Dataset"}"#);
        assert_eq!(
            decode_capped(&src, Some("utf-8")),
            r#"{"@type":"dcat:Dataset"}"#
        );
    }

    #[test]
    fn decode_capped_survives_a_body_truncated_mid_codepoint() {
        // A dangling lead byte used to kill an otherwise-fine read via
        // read_to_string(); it now degrades to U+FFFD.
        let src: &[u8] = &[b'a', 0xC3];
        assert_eq!(decode_capped(src, None), "a\u{FFFD}");
    }

    #[test]
    fn read_capped_stops_at_the_byte_cap_slicing_a_codepoint() {
        // Covers the read/decode seam: the cap lands between the two bytes of "é"
        // (0xC3 0xA9), so read_capped hands back a buffer ending in a dangling lead
        // byte. This exact shape is what the old read_to_string() died on, so the
        // fixture has to straddle the real cap - a short buffer would not prove it.
        let cap = DCAT_DISCOVERY_MAX_BYTES as usize;
        let mut src = vec![b'a'; cap - 1];
        src.extend_from_slice(&[0xC3, 0xA9]);
        assert!(
            std::str::from_utf8(&src).is_ok(),
            "fixture must itself be valid UTF-8"
        );

        let got = read_capped(src.as_slice()).expect("capped read must not fail");
        assert_eq!(got.len(), cap, "read_capped must stop exactly at the cap");
        assert_eq!(
            got[cap - 1],
            0xC3,
            "the cap must actually slice the codepoint"
        );

        let decoded = decode_capped(&got, None);
        assert_eq!(decoded.len(), cap - 1 + "\u{FFFD}".len());
        assert!(decoded.ends_with('\u{FFFD}'));
    }

    #[test]
    fn read_capped_passes_through_a_short_body() {
        let src = r#"{"@type":"dcat:Dataset","dct:title":"café"}"#;
        assert_eq!(read_capped(src.as_bytes()).as_deref(), Some(src.as_bytes()));
    }

    #[test]
    fn extract_dataset_from_bare_object() {
        let doc = json!({"@type": "dcat:Dataset", "dct:title": "X"});
        assert_eq!(extract_dcat_dataset(&doc), Some(doc.clone()));
    }

    #[test]
    fn extract_dataset_from_graph_array() {
        let target = json!({"@type": "dcat:Dataset", "dct:title": "wanted"});
        let doc = json!({
            "@context": "https://...",
            "@graph": [
                {"@type": "dcat:Catalog", "dct:title": "skip"},
                target.clone(),
                {"@type": "dcat:Distribution", "dct:title": "also skip"},
            ]
        });
        assert_eq!(extract_dcat_dataset(&doc), Some(target));
    }

    #[test]
    fn extract_dataset_recognizes_full_iri_type() {
        // W3C DCAT canonical type IRI — http scheme is the published
        // identifier. DevSkim: ignore DS137138
        let doc = json!({"@type": "http://www.w3.org/ns/dcat#Dataset", "dct:title": "X"});
        assert!(extract_dcat_dataset(&doc).is_some());
    }

    #[test]
    fn extract_dataset_handles_type_array() {
        // Leading entry is a placeholder type IRI for the test;
        // DevSkim: ignore DS137138
        let doc = json!({"@type": ["http://example/Thing", "dcat:Dataset"], "dct:title": "X"});
        assert!(extract_dcat_dataset(&doc).is_some());
    }

    #[test]
    fn extract_dataset_falls_back_on_shape() {
        // No @type, but has dct:title — accept best-effort.
        let doc = json!({"dct:title": "X", "dcat:keyword": ["a", "b"]});
        assert!(extract_dcat_dataset(&doc).is_some());
    }

    #[test]
    fn extract_dataset_rejects_unrelated_object() {
        let doc = json!({"foo": "bar"});
        assert_eq!(extract_dcat_dataset(&doc), None);
    }

    // §5.2: sibling-URL candidate construction.
    #[test]
    fn sibling_candidates_for_typical_csv_url() {
        let c = sibling_candidates("https://x.gov/data/snapshot.csv");
        assert_eq!(c.len(), 4);
        assert_eq!(c[0], "https://x.gov/data/snapshot.csv.metadata.json");
        assert_eq!(c[1], "https://x.gov/data/snapshot.csv.dcat.json");
        assert_eq!(c[2], "https://x.gov/data/datapackage.json");
        assert_eq!(c[3], "https://x.gov/.well-known/data.json");
    }

    #[test]
    fn sibling_candidates_strips_query_and_fragment_from_all_candidates() {
        // All four candidates are built via `url::Url` parsing, so the
        // input URL's query string and fragment are dropped — they would
        // otherwise be appended *after* the `.metadata.json` /
        // `.dcat.json` suffix (e.g. `snapshot.csv?token=abc.metadata.json`),
        // which servers interpret as a GET on the CSV with a polluted
        // query value rather than the sibling JSON.
        let c = sibling_candidates("https://x.gov/data/snapshot.csv?token=abc#frag");
        assert_eq!(c[0], "https://x.gov/data/snapshot.csv.metadata.json");
        assert_eq!(c[1], "https://x.gov/data/snapshot.csv.dcat.json");
        assert_eq!(c[2], "https://x.gov/data/datapackage.json");
        assert_eq!(c[3], "https://x.gov/.well-known/data.json");
    }

    #[test]
    fn sibling_candidates_for_url_with_no_path() {
        // Host-only URL: parent of "/" is None per Path::parent, so
        // datapackage.json gets a "/" prefix as the fallback dirname.
        let c = sibling_candidates("https://x.gov/");
        assert_eq!(c[0], "https://x.gov/.metadata.json");
        assert_eq!(c[1], "https://x.gov/.dcat.json");
        assert_eq!(c[2], "https://x.gov/datapackage.json");
        assert_eq!(c[3], "https://x.gov/.well-known/data.json");
    }

    // §5.2: JSON-LD <script> block extraction.
    #[test]
    fn extract_jsonld_finds_dcat_dataset_in_script_tag() {
        let html = r#"<!doctype html>
<html><head>
  <script type="application/ld+json">
    {"@type": "dcat:Dataset", "dct:title": "Pittsburgh 311"}
  </script>
</head><body></body></html>"#;
        let found = extract_jsonld_blocks(html).expect("should find dataset");
        assert_eq!(
            found.pointer("/dct:title").and_then(|v| v.as_str()),
            Some("Pittsburgh 311"),
        );
    }

    #[test]
    fn extract_jsonld_handles_mixed_case_type_attribute() {
        // Spec is case-insensitive for the type attribute.
        let html = r#"<html><script type="Application/LD+JSON">
{"@type":"dcat:Dataset","dct:title":"Mixed Case Wins"}
</script></html>"#;
        let found = extract_jsonld_blocks(html).expect("mixed-case should match");
        assert_eq!(
            found.pointer("/dct:title").and_then(|v| v.as_str()),
            Some("Mixed Case Wins"),
        );
    }

    #[test]
    fn extract_jsonld_walks_past_non_dataset_blocks() {
        // First block is a WebSite, second is the Dataset we want.
        let html = r#"<html>
<script type="application/ld+json">{"@type":"WebSite","name":"Site"}</script>
<script type="application/ld+json">{"@type":"dcat:Dataset","dct:title":"Real One"}</script>
</html>"#;
        let found = extract_jsonld_blocks(html).expect("dataset should win over website");
        assert_eq!(
            found.pointer("/dct:title").and_then(|v| v.as_str()),
            Some("Real One"),
        );
    }

    #[test]
    fn extract_jsonld_returns_none_when_no_match() {
        let html = "<html><script type=\"application/javascript\">var x = 1;</script></html>";
        assert_eq!(extract_jsonld_blocks(html), None);
    }

    #[test]
    fn extract_jsonld_skips_unrelated_script_tags() {
        // Only non-ld+json scripts present → no match.
        let html = "<html><script>alert(1)</script></html>";
        assert_eq!(extract_jsonld_blocks(html), None);
    }

    #[test]
    fn extract_jsonld_accepts_graph_envelope() {
        // JSON-LD often wraps multiple typed nodes in @graph.
        let html = r#"<html><script type="application/ld+json">
{"@context":"https://schema.org","@graph":[
  {"@type":"WebSite","name":"Site"},
  {"@type":"dcat:Dataset","dct:title":"From Graph"}
]}
</script></html>"#;
        let found = extract_jsonld_blocks(html).expect("graph-wrapped dataset should match");
        assert_eq!(
            found.pointer("/dct:title").and_then(|v| v.as_str()),
            Some("From Graph"),
        );
    }
}
