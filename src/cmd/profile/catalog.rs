//! Optional DCAT-US v3 `Catalog` wrapper.
//!
//! When the user passes `--catalog`, `wrap_as_catalog` envelopes the
//! Dataset projection inside a `dcat:Catalog` so the output is
//! immediately consumable by federation harvesters (data.gov,
//! CKAN ingest, multi-agency aggregators) that expect the
//! `Catalog { dcat:dataset: [...] }` shape.
//!
//! Identity fields on the Catalog (`dct:title`, `dct:publisher`,
//! `dct:conformsTo`) are pulled from the enclosed Dataset so the
//! envelope never disagrees with the metadata it carries.
//!
//! Why no `dct:modified` on the Catalog envelope: a single-CSV
//! `qsv profile` run has no independent catalog-level modification
//! date — the only meaningful mtime belongs to the inner Dataset.
//! Emitting one anyway would either duplicate the Dataset's date
//! (redundant) or invent a wall-clock value (unreliable). Users
//! who DO have catalog-level metadata can drop one in via
//! `--initial-context.dataset_info` pointer overrides; the
//! `apply_pointer_overrides` step runs after `wrap_as_catalog` so
//! pointer paths like `/dcat/dct:modified` land on the Catalog
//! envelope as expected.

use serde_json::{Value, json};

/// Standards declaration value reused on Catalog envelopes. Mirrors
/// the form `dcat::add_governance` already emits on the Dataset.
const DCAT_US_V3_STANDARD: &str = "https://resources.data.gov/resources/dcat-us3/";

/// Wrap an emitted Dataset block inside a DCAT-US v3 Catalog envelope.
///
/// The Catalog inherits the Dataset's title (prefixed with "Catalog of
/// " for clarity), publisher, and conformance declaration. The
/// original Dataset is moved into a one-element `dcat:dataset` array
/// unchanged — downstream consumers see exactly the same Dataset
/// metadata they would have without `--catalog`.
#[must_use]
pub fn wrap_as_catalog(dataset: Value) -> Value {
    let title = dataset
        .get("dct:title")
        .and_then(Value::as_str)
        .map(|t| format!("Catalog of {t}"))
        .unwrap_or_else(|| "qsv profile catalog".to_string());

    let publisher = dataset.get("dct:publisher").cloned();

    let mut catalog = json!({
        "@type":     "dcat:Catalog",
        "dct:title": title,
        "dct:conformsTo": {
            "@type": "dct:Standard",
            "@id":   DCAT_US_V3_STANDARD,
        },
        "dcat:dataset": [dataset],
    });

    if let Some(p) = publisher
        && let Some(obj) = catalog.as_object_mut()
    {
        obj.insert("dct:publisher".to_string(), p);
    }

    catalog
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn wrap_as_catalog_populates_required_keys() {
        let ds = json!({
            "@type":     "dcat:Dataset",
            "dct:title": "Pittsburgh 311",
            "dct:publisher": {
                "@type":     "foaf:Organization",
                "foaf:name": "City of Pittsburgh",
            },
        });
        let cat = wrap_as_catalog(ds);
        assert_eq!(cat["@type"], json!("dcat:Catalog"));
        assert_eq!(cat["dct:title"], json!("Catalog of Pittsburgh 311"));
        assert_eq!(cat["dct:conformsTo"]["@type"], json!("dct:Standard"));
        assert!(cat["dcat:dataset"].is_array());
        assert_eq!(
            cat["dcat:dataset"].as_array().unwrap().len(),
            1,
            "exactly one dataset inside the envelope"
        );
        assert_eq!(
            cat["dct:publisher"]["foaf:name"],
            json!("City of Pittsburgh")
        );
    }

    #[test]
    fn wrap_as_catalog_preserves_inner_dataset_unchanged() {
        let ds = json!({
            "@type":          "dcat:Dataset",
            "dct:title":      "Foo",
            "dct:identifier": "id-123",
            "dcat:distribution": [
                {"@type": "dcat:Distribution", "dct:format": "CSV"}
            ],
            "dcat-us:bureauCode": ["015:11"],
        });
        let cat = wrap_as_catalog(ds.clone());
        assert_eq!(
            cat["dcat:dataset"][0], ds,
            "inner dataset must be byte-identical to the input"
        );
    }

    #[test]
    fn wrap_as_catalog_handles_dataset_without_publisher() {
        // Some test fixtures (and stdin inputs without
        // --initial-context) won't have a publisher. The Catalog
        // envelope should still build cleanly — `dct:publisher` is
        // optional on Catalog per GSA's Catalog.json.
        let ds = json!({"@type": "dcat:Dataset", "dct:title": "Anon"});
        let cat = wrap_as_catalog(ds);
        assert!(cat.get("dct:publisher").is_none());
        assert_eq!(cat["dct:title"], json!("Catalog of Anon"));
    }

    #[test]
    fn wrap_as_catalog_falls_back_when_dataset_has_no_title() {
        // The DCAT projection always emits dct:title (it's mandatory)
        // but the wrapper should not panic if a caller passes a
        // malformed Dataset.
        let ds = json!({"@type": "dcat:Dataset"});
        let cat = wrap_as_catalog(ds);
        assert_eq!(cat["dct:title"], json!("qsv profile catalog"));
    }

    #[test]
    fn wrap_as_catalog_does_not_emit_dct_modified() {
        // Documented behavior: no independent catalog-level mtime
        // for a single-CSV run. Users layer one in via dataset_info
        // pointer overrides if desired.
        let ds = json!({
            "@type":         "dcat:Dataset",
            "dct:title":     "Foo",
            "dct:modified":  "2024-01-15T00:00:00Z",
        });
        let cat = wrap_as_catalog(ds);
        assert!(
            cat.get("dct:modified").is_none(),
            "Catalog envelope must NOT inherit Dataset's dct:modified"
        );
    }
}
