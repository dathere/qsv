//! CURIE → unprefixed key transformation for DCAT-US v3 validation.
//!
//! `dcat::build` emits the JSON-LD-compact form with CURIE-prefixed
//! object keys (`dct:title`, `dcat:contactPoint`, `vcard:fn`) so the
//! output is friendly to CKAN, data.gov, and other downstream
//! consumers. The vendored GSA DCAT-US v3 JSON Schema bundle under
//! `resources/dcat-us-v3/` validates against **unprefixed** keys
//! (`title`, `contactPoint`, `fn`).
//!
//! This module bridges the two forms transparently at validation
//! time. `strip_curies` returns a deep copy of a JSON value with every
//! known-prefix CURIE key (`<prefix>:<localname>`) replaced by its
//! `<localname>`. Only object KEYS are rewritten; string VALUES are
//! left intact so that e.g. `"@type": "dcat:Dataset"` stays literal
//! (the GSA bundle does not constrain `@type` values via `enum` or
//! `const`, so the literal CURIE form passes validation).
//!
//! The `dcat-us:` namespace is intentionally NOT stripped. GSA's
//! bundle does not define `dcat-us:bureauCode` / `dcat-us:programCode`;
//! qsv's `qsv-overlay-dataset.json` documents them as additional
//! properties. Leaving the prefix in place lets the overlay match
//! the keys exactly.
//!
//! The emitted JSON on disk is unchanged — `strip_curies` operates on
//! an in-memory clone passed to the validator.

use serde_json::{Map, Value};

/// CURIE prefixes whose `<prefix>:<localname>` keys collapse to
/// `<localname>` for validation. Matches the namespaces qsv profile
/// emits in `dcat.rs`. Prefixes are checked in declaration order;
/// the first match wins. Keep ordering stable — `dcat-us:` is NOT
/// in this list and must not be added without also reshaping the
/// validator overlay.
const STRIPPABLE_PREFIXES: &[&str] = &[
    "dct:",
    "dcat:",
    "foaf:",
    "vcard:",
    "skos:",
    "spdx:",
    "xsd:",
    "geosparql:",
    "csvw:",
    "locn:",
    "qsv:",
];

/// JSON-LD reserved keys that must never be rewritten — they aren't
/// CURIEs even though they start with `@`. Listed here for
/// completeness; the prefix match logic in `strip_key` already
/// rejects them because none start with one of `STRIPPABLE_PREFIXES`.
const _JSONLD_KEYS: &[&str] = &["@context", "@id", "@type", "@graph"];

/// Return a deep copy of `v` with every object key whose CURIE
/// prefix matches one in `STRIPPABLE_PREFIXES` replaced by the
/// unprefixed local name. Arrays and string values are unchanged.
///
/// Allocations: this is a clone-and-rewrite, not an in-place mutation,
/// because the validator needs ownership and the emitted block (the
/// one written to disk) must keep the compact form.
pub fn strip_curies(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut out = Map::with_capacity(map.len());
            for (k, child) in map {
                let new_key = strip_key(k);
                out.insert(new_key, strip_curies(child));
            }
            Value::Object(out)
        },
        Value::Array(items) => Value::Array(items.iter().map(strip_curies).collect()),
        _ => v.clone(),
    }
}

/// Strip the first matching CURIE prefix from `key`. Returns the
/// original key when no prefix matches (including JSON-LD `@`-keys
/// and the deliberately-preserved `dcat-us:` namespace).
fn strip_key(key: &str) -> String {
    for prefix in STRIPPABLE_PREFIXES {
        if let Some(local) = key.strip_prefix(prefix) {
            return local.to_string();
        }
    }
    key.to_string()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn strips_known_prefixes_from_object_keys() {
        let input = json!({
            "dct:title":       "Foo",
            "dcat:keyword":    ["a", "b"],
            "foaf:name":       "Bob",
            "vcard:fn":        "Bob B.",
        });
        let stripped = strip_curies(&input);
        assert_eq!(
            stripped,
            json!({
                "title":   "Foo",
                "keyword": ["a", "b"],
                "name":    "Bob",
                "fn":      "Bob B.",
            })
        );
    }

    #[test]
    fn leaves_dcat_us_namespace_alone() {
        // The qsv-overlay-dataset.json schema matches dcat-us:* keys
        // verbatim, so the prefix must not be stripped.
        let input = json!({
            "dct:title":              "Foo",
            "dcat-us:bureauCode":     ["015:11"],
            "dcat-us:programCode":    ["015:001"],
        });
        let stripped = strip_curies(&input);
        assert_eq!(
            stripped,
            json!({
                "title":                "Foo",
                "dcat-us:bureauCode":   ["015:11"],
                "dcat-us:programCode":  ["015:001"],
            })
        );
    }

    #[test]
    fn leaves_jsonld_reserved_keys_alone() {
        let input = json!({
            "@context":  "https://example.gov/c",
            "@id":       "https://example.gov/d/1",
            "@type":     "dcat:Dataset",
            "dct:title": "Foo",
        });
        let stripped = strip_curies(&input);
        assert_eq!(
            stripped,
            json!({
                "@context":  "https://example.gov/c",
                "@id":       "https://example.gov/d/1",
                "@type":     "dcat:Dataset",
                "title":     "Foo",
            })
        );
    }

    #[test]
    fn does_not_touch_string_values_even_when_they_look_like_curies() {
        // @type carries a CURIE-shaped string but it's a VALUE, not a
        // KEY — must pass through untouched. Same for any other
        // string value that happens to look like "dct:something".
        let input = json!({
            "@type":          "dcat:Dataset",
            "dct:conformsTo": {
                "@type": "dct:Standard",
                "@id":   "https://resources.data.gov/resources/dcat-us3/",
            },
            "dct:description": "this mentions dct:title in prose",
        });
        let stripped = strip_curies(&input);
        assert_eq!(stripped["@type"], json!("dcat:Dataset"));
        assert_eq!(stripped["conformsTo"]["@type"], json!("dct:Standard"));
        assert_eq!(
            stripped["description"],
            json!("this mentions dct:title in prose")
        );
    }

    #[test]
    fn recurses_into_nested_objects_and_arrays() {
        let input = json!({
            "dcat:contactPoint": {
                "@type":          "vcard:Individual",
                "vcard:fn":       "Bob",
                "vcard:hasEmail": "mailto:bob@example.gov",
            },
            "dcat:distribution": [
                {"@type": "dcat:Distribution", "dct:title": "A"},
                {"@type": "dcat:Distribution", "dct:title": "B"},
            ],
        });
        let stripped = strip_curies(&input);
        assert_eq!(stripped["contactPoint"]["fn"], json!("Bob"));
        assert_eq!(
            stripped["contactPoint"]["hasEmail"],
            json!("mailto:bob@example.gov")
        );
        assert_eq!(stripped["distribution"][0]["title"], json!("A"));
        assert_eq!(stripped["distribution"][1]["title"], json!("B"));
    }

    #[test]
    fn leaves_unprefixed_keys_untouched() {
        // If someone already passes an unprefixed block (e.g. a raw
        // GSA example fixture) the function is a no-op.
        let input = json!({
            "title":       "Foo",
            "description": "Bar",
            "publisher":   {"@type": "Organization", "name": "Agency"},
        });
        let stripped = strip_curies(&input);
        assert_eq!(stripped, input);
    }

    #[test]
    fn leaves_unknown_prefix_alone() {
        // A CURIE-shaped key with a namespace we do NOT enumerate
        // (e.g. "schema:" or "ex:") must pass through. Otherwise we'd
        // silently rename keys the user intended as opaque.
        let input = json!({
            "schema:identifier": "abc",
            "ex:custom":         "xyz",
        });
        let stripped = strip_curies(&input);
        assert_eq!(stripped, input);
    }
}
