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
//!   2. (Phase 3b' follow-up) sibling URLs by convention (`<url>.metadata.json`, `<url>.dcat.json`,
//!      `<dirname>/datapackage.json`, `<host>/.well-known/data.json`).
//!   3. (Phase 3b'' follow-up) JSON-LD `<script type="application/ld+json">` blocks in the URL's
//!      HTML landing page.
//!
//! Each step has a tight default timeout (configurable via
//! `--dcat-discovery-timeout`). All network errors are non-fatal — the
//! discovery falls through to the next mechanism, and if all fail the
//! projection proceeds with no discovered base layer.

use std::time::Duration;

use reqwest::{
    blocking::Client,
    header::{HeaderValue, LINK},
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
pub fn discover(url: &str, timeout: Duration) -> Option<Value> {
    let timeout_secs: u16 = timeout.as_secs().max(1).min(u16::MAX as u64) as u16;
    let client = util::create_reqwest_blocking_client(None, timeout_secs, None).ok()?;

    if let Some(via_link) = discover_via_link_header(&client, url) {
        return Some(via_link);
    }

    // Future: sibling URL probing + HTML script-tag JSON-LD extraction.
    None
}

/// Cap on bytes read from a publisher-supplied DCAT-LD document.
/// The Link `rel=describedBy` target is publisher-controlled, so an
/// errant/hostile pointer at a multi-GB resource shouldn't blow up
/// qsv's memory. 4 MiB is well above any realistic DCAT-LD document
/// (Pittsburgh 311's full CKAN package JSON is ~30 KiB) while still
/// being a safe ceiling.
const DCAT_DISCOVERY_MAX_BYTES: u64 = 4 * 1024 * 1024;

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
    let mut limited = std::io::Read::take(response, DCAT_DISCOVERY_MAX_BYTES);
    let mut body = String::new();
    std::io::Read::read_to_string(&mut limited, &mut body).ok()?;
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
}
