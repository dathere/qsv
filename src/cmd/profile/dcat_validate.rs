//! Opt-in JSON Schema validation of the emitted dcat block.
//!
//! Activated by `--validate-dcat`; with `--strict-dcat` the command
//! fails on any schema error instead of appending to the
//! `dcat_warnings` array.
//!
//! ## Schema scope
//!
//! Phase 6 ships an **embedded minimal v3 schema** that enforces only
//! the DCAT-US v3 mandatory keys (the bar from the spec landing page
//! at <https://resources.data.gov/resources/dcat-us3/>). It catches
//! the common "I forgot to set --initial-context.package.contact_point"
//! class of mistake without requiring users to download the full GSA
//! schema bundle.
//!
//! **Follow-up:** vendor the full GSA dcat-us JSON Schema suite from
//! <https://github.com/GSA/dcat-us/tree/main/jsonschema> under
//! `resources/dcat-us-v3/` pinned to a specific upstream commit SHA,
//! and switch `embedded_minimal_schema()` to load the vendored bundle
//! with `$ref` resolution. The Validator construction pattern at the
//! bottom of this file already mirrors the one used in
//! `src/cmd/validate.rs:2378` for an easy swap.

use jsonschema::Validator;
use serde_json::{Value, json};

use super::dcat::{DcatWarning, Severity};

/// Validate `dcat_block` against the embedded minimal DCAT-US v3
/// schema. Returns one `DcatWarning` per violation; an empty Vec
/// indicates the block passed the minimal bar.
pub fn validate_dataset(dcat_block: &Value) -> Vec<DcatWarning> {
    let schema = embedded_minimal_schema();
    let validator = match Validator::options().build(&schema) {
        Ok(v) => v,
        Err(e) => {
            return vec![DcatWarning {
                field:    "dcat_validate".to_string(),
                severity: Severity::Required,
                message:  format!("could not compile embedded DCAT-US v3 schema: {e}"),
            }];
        },
    };
    validator
        .iter_errors(dcat_block)
        .map(|err| DcatWarning {
            field:    err
                .instance_path()
                .to_string()
                .trim_start_matches('/')
                .to_string(),
            severity: classify_severity(&err),
            message:  format!("{err}"),
        })
        .collect()
}

/// Inferred severity for a schema validation error. Errors keyed off
/// `required` (missing mandatory field) get `Required`; everything else
/// is `Recommended`. The full GSA bundle has explicit cardinality
/// metadata that future commits can use to refine this.
fn classify_severity(err: &jsonschema::ValidationError<'_>) -> Severity {
    let kind_str = format!("{:?}", err.kind());
    if kind_str.contains("Required") {
        Severity::Required
    } else {
        Severity::Recommended
    }
}

/// Embedded minimal JSON Schema 2020-12 covering the v3 mandatory
/// fields per the spec landing page. Deliberately permissive on
/// recommended fields — those are surfaced by the in-projection
/// warning helpers (`add_contact_point`, `add_us_codes`, etc.) which
/// give richer guidance than a schema error can.
fn embedded_minimal_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type":    "object",
        "required": [
            "@type",
            "dct:title",
            "dct:description",
            "dct:identifier",
            "dct:publisher",
            "dcat:contactPoint",
            "dct:conformsTo",
            "dcat:distribution"
        ],
        "properties": {
            "@type": {
                "type":  "string",
                "const": "dcat:Dataset"
            },
            "dct:title":       { "type": "string", "minLength": 1 },
            "dct:description": { "type": "string", "minLength": 1 },
            "dct:identifier":  { "type": "string", "minLength": 1 },
            "dct:publisher": {
                "type":     "object",
                "required": ["@type", "foaf:name"]
            },
            "dcat:contactPoint": {
                "type":     "object",
                "required": ["@type", "vcard:fn", "vcard:hasEmail"],
                "properties": {
                    "vcard:hasEmail": {
                        "type":    "string",
                        "pattern": "^mailto:"
                    }
                }
            },
            "dct:conformsTo": {
                "type":     "object",
                "required": ["@type", "@id"],
                "properties": {
                    "@type": { "type": "string", "const": "dct:Standard" }
                }
            },
            "dct:spatial":      { "type": "array" },
            "dct:temporal":     { "type": "array" },
            "dcat:distribution": { "type": "array", "minItems": 1 }
        }
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn minimal_passing_dataset() -> Value {
        json!({
            "@context": "https://doi-do.github.io/dcat-us/context.jsonld",
            "@type":    "dcat:Dataset",
            "dct:title":       "X",
            "dct:description": "Y",
            "dct:identifier":  "demo",
            "dct:publisher":   {"@type": "foaf:Agent", "foaf:name": "Agency"},
            "dcat:contactPoint": {
                "@type":          "vcard:Individual",
                "vcard:fn":       "Jane",
                "vcard:hasEmail": "mailto:jane@example.gov"
            },
            "dct:conformsTo": {
                "@type": "dct:Standard",
                "@id":   "https://resources.data.gov/resources/dcat-us3/"
            },
            "dcat:distribution": [{"@type": "dcat:Distribution"}]
        })
    }

    #[test]
    fn minimal_passing_dataset_returns_no_warnings() {
        let warnings = validate_dataset(&minimal_passing_dataset());
        assert!(
            warnings.is_empty(),
            "expected no warnings, got: {warnings:#?}"
        );
    }

    #[test]
    fn missing_contact_point_surfaces_required_warning() {
        let mut ds = minimal_passing_dataset();
        ds.as_object_mut().unwrap().remove("dcat:contactPoint");
        let warnings = validate_dataset(&ds);
        assert!(!warnings.is_empty(), "expected at least one warning");
        let cp = warnings
            .iter()
            .find(|w| w.message.contains("contactPoint"))
            .expect("expected contactPoint warning");
        assert!(
            matches!(cp.severity, Severity::Required),
            "expected Required severity, got: {:?}",
            cp.severity
        );
    }

    #[test]
    fn bad_contact_point_email_surfaces_warning() {
        let mut ds = minimal_passing_dataset();
        ds.pointer_mut("/dcat:contactPoint/vcard:hasEmail")
            .map(|v| *v = json!("jane@example.gov")); // missing mailto:
        let warnings = validate_dataset(&ds);
        assert!(
            warnings
                .iter()
                .any(|w| w.message.to_lowercase().contains("mailto")
                    || w.message.contains("pattern")),
            "expected mailto/pattern violation, got: {warnings:#?}"
        );
    }

    #[test]
    fn wrong_type_for_dataset_fails() {
        let mut ds = minimal_passing_dataset();
        ds["@type"] = json!("dcat:Catalog");
        let warnings = validate_dataset(&ds);
        assert!(
            !warnings.is_empty(),
            "expected validation to reject @type: dcat:Catalog"
        );
    }

    #[test]
    fn missing_distribution_array_fails_minimum() {
        let mut ds = minimal_passing_dataset();
        ds.as_object_mut().unwrap().remove("dcat:distribution");
        let warnings = validate_dataset(&ds);
        // minItems: 1 on dcat:distribution + we removed the key — at
        // least one warning expected (varies by jsonschema version).
        assert!(
            !warnings.is_empty(),
            "expected at least one warning for missing distribution"
        );
    }
}
