//! Opt-in JSON Schema validation of the emitted dcat block.
//!
//! Activated by `--validate`; with `--strict` the command
//! fails on any schema error instead of appending to the
//! `projection_warnings` array.
//!
//! ## Schema bundle
//!
//! The validator runs against the upstream GSA DCAT-US v3 JSON
//! Schema bundle, vendored under `resources/dcat-us-v3/`. See
//! `resources/dcat-us-v3/README.md` for pinning + refresh
//! procedure and `resources/dcat-us-v3/MANIFEST.json` for the
//! exact commit + per-file SHA-256 hashes.
//!
//! Two qsv-authored overlay schemas wrap the GSA definitions:
//!
//! * `qsv-overlay-dataset.json` (`allOf`-refs `definitions/Dataset.json`) — entry-point when the
//!   emitted block is a Dataset.
//! * `qsv-overlay-catalog.json` (`allOf`-refs `definitions/Catalog.json`) — entry-point when
//!   `--catalog` produced a Catalog envelope.
//!
//! Both overlays document the `dcat-us:*` namespace extensions
//! (`dcat-us:bureauCode`, `dcat-us:programCode`) that the GSA
//! bundle itself does not define.
//!
//! ## CURIE/IRI bridge
//!
//! The GSA bundle validates against **unprefixed** keys
//! (`title`, `contactPoint`, `fn`) — JSON-LD-expanded local
//! names. `dcat::build` emits the JSON-LD-**compact** form with
//! CURIE prefixes (`dct:title`, `dcat:contactPoint`, `vcard:fn`)
//! for interop with CKAN, data.gov, and other downstream
//! consumers. The two forms are bridged transparently at
//! validation time by [`curie::strip_curies`], which returns a
//! deep copy of the dcat block with known prefixes stripped from
//! object keys. The emitted JSON on disk is unchanged.

use std::sync::OnceLock;

use jsonschema::Validator;
use serde_json::{Value, json};

use super::{
    profile_spec::ProfileSpec,
    projection::{ProjectionWarning, Severity},
};

// -----------------------------------------------------------------------------
// Static bundle
// -----------------------------------------------------------------------------

/// Vendored GSA bundle, embedded at compile time. The `name` field is
/// the lowercase canonical identifier the GSA schemas use in their
/// `$id` segments (e.g. Dataset.json declares
/// `$id: ".../definitions/dataset"`). The retriever normalizes
/// incoming URIs to that lowercased form before lookup, so $refs
/// from either inside the GSA bundle (lowercase, no `.json`) or
/// from our overlay (`definitions/Dataset.json`) both resolve.
struct SchemaEntry {
    name: &'static str,
    json: &'static str,
}

// One entry per file under `resources/dcat-us-v3/definitions/`.
// Sorted by `name` so audit-by-eye matches the directory listing.
const BUNDLE: &[SchemaEntry] = &[
    SchemaEntry {
        name: "accessrestriction",
        json: include_str!("../../../resources/dcat-us-v3/definitions/AccessRestriction.json"),
    },
    SchemaEntry {
        name: "activity",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Activity.json"),
    },
    SchemaEntry {
        name: "address",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Address.json"),
    },
    SchemaEntry {
        name: "agent",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Agent.json"),
    },
    SchemaEntry {
        name: "attribution",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Attribution.json"),
    },
    SchemaEntry {
        name: "cuirestriction",
        json: include_str!("../../../resources/dcat-us-v3/definitions/CUIRestriction.json"),
    },
    SchemaEntry {
        name: "catalog",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Catalog.json"),
    },
    SchemaEntry {
        name: "catalogrecord",
        json: include_str!("../../../resources/dcat-us-v3/definitions/CatalogRecord.json"),
    },
    SchemaEntry {
        name: "checksum",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Checksum.json"),
    },
    SchemaEntry {
        name: "concept",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Concept.json"),
    },
    SchemaEntry {
        name: "conceptscheme",
        json: include_str!("../../../resources/dcat-us-v3/definitions/ConceptScheme.json"),
    },
    SchemaEntry {
        name: "dataservice",
        json: include_str!("../../../resources/dcat-us-v3/definitions/DataService.json"),
    },
    SchemaEntry {
        name: "dataset",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Dataset.json"),
    },
    SchemaEntry {
        name: "datasetseries",
        json: include_str!("../../../resources/dcat-us-v3/definitions/DatasetSeries.json"),
    },
    SchemaEntry {
        name: "distribution",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Distribution.json"),
    },
    SchemaEntry {
        name: "document",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Document.json"),
    },
    SchemaEntry {
        name: "identifier",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Identifier.json"),
    },
    SchemaEntry {
        name: "kind",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Kind.json"),
    },
    SchemaEntry {
        name: "location",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Location.json"),
    },
    SchemaEntry {
        name: "metric",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Metric.json"),
    },
    SchemaEntry {
        name: "organization",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Organization.json"),
    },
    SchemaEntry {
        name: "periodoftime",
        json: include_str!("../../../resources/dcat-us-v3/definitions/PeriodOfTime.json"),
    },
    SchemaEntry {
        name: "qualitymeasurement",
        json: include_str!("../../../resources/dcat-us-v3/definitions/QualityMeasurement.json"),
    },
    SchemaEntry {
        name: "relationship",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Relationship.json"),
    },
    SchemaEntry {
        name: "standard",
        json: include_str!("../../../resources/dcat-us-v3/definitions/Standard.json"),
    },
    SchemaEntry {
        name: "userestriction",
        json: include_str!("../../../resources/dcat-us-v3/definitions/UseRestriction.json"),
    },
];

/// Dataset entry-point overlay (`allOf`-refs `definitions/Dataset.json`).
const DATASET_OVERLAY_JSON: &str =
    include_str!("../../../resources/dcat-us-v3/qsv-overlay-dataset.json");

/// Catalog entry-point overlay (`allOf`-refs `definitions/Catalog.json`).
const CATALOG_OVERLAY_JSON: &str =
    include_str!("../../../resources/dcat-us-v3/qsv-overlay-catalog.json");

// -----------------------------------------------------------------------------
// (No retriever needed — the registry is pre-populated in `build_validator`.)
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Compiled validators (lazy, one-shot)
// -----------------------------------------------------------------------------

static DATASET_VALIDATOR: OnceLock<Result<Validator, String>> = OnceLock::new();
static CATALOG_VALIDATOR: OnceLock<Result<Validator, String>> = OnceLock::new();

fn dataset_validator() -> Result<&'static Validator, &'static str> {
    DATASET_VALIDATOR
        .get_or_init(|| build_validator(DATASET_OVERLAY_JSON))
        .as_ref()
        .map_err(String::as_str)
}

fn catalog_validator() -> Result<&'static Validator, &'static str> {
    CATALOG_VALIDATOR
        .get_or_init(|| build_validator(CATALOG_OVERLAY_JSON))
        .as_ref()
        .map_err(String::as_str)
}

fn build_validator(entry_json: &str) -> Result<Validator, String> {
    let entry_schema: Value =
        serde_json::from_str(entry_json).map_err(|e| format!("malformed overlay schema: {e}"))?;

    // Pre-populate a referencing::Registry with every vendored GSA
    // definition at its canonical `$id` URI. Once built, the
    // registry resolves all internal cross-refs without ever needing
    // to call a Retrieve impl — the referencing crate disallows
    // lazy retrieval after the registry is prepared, so up-front
    // registration is the only way to validate against the bundle.
    let mut builder = jsonschema::Registry::new();
    for e in BUNDLE {
        let val: Value = serde_json::from_str(e.json)
            .map_err(|err| format!("malformed vendored schema `{}`: {err}", e.name))?;
        let uri = format!(
            "https://resources.data.gov/dcat-us/3.0.0/definitions/{}",
            e.name
        );
        builder = builder
            .add(uri, val)
            .map_err(|err| format!("could not register `{}` in registry: {err}", e.name))?;
    }
    let registry = builder
        .prepare()
        .map_err(|e| format!("could not prepare DCAT-US v3 registry: {e}"))?;

    jsonschema::options()
        .with_registry(&registry)
        .build(&entry_schema)
        .map_err(|e| format!("could not compile DCAT-US v3 schema bundle: {e}"))
}

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

/// Validate `block` against the profile's declared schema bundle.
/// Returns one `ProjectionWarning` per violation; an empty Vec
/// indicates the block is conformant.
///
/// When `profile.validation.enabled == false`, validation is a no-op
/// (returns an empty Vec). Non-DCAT profiles use this to opt out of
/// JSON-Schema validation entirely.
///
/// Only the vendored DCAT-US v3 GSA bundle is supported today. The
/// profile is expected to declare `schema_dir: resources/dcat-us-v3/`
/// (matching `EMBEDDED_SCHEMA_DIR`); any other `schema_dir` produces a
/// single `Recommended`-severity warning explaining that arbitrary
/// schema-bundle loading is a queued follow-up (Roborev #2490 finding
/// #6). The embedded validators are still used when the profile points
/// at the bundled directory so DCAT-US v3 validation behaves
/// unchanged.
pub fn validate(profile: &ProfileSpec, block: &Value) -> Vec<ProjectionWarning> {
    // The embedded validators hold the vendored GSA DCAT-US v3
    // bundle. A profile whose `schema_dir` matches this directory
    // path gets validated against the embedded validators; anything
    // else short-circuits with a heads-up warning rather than
    // silently misvalidating against the wrong schema.
    const EMBEDDED_SCHEMA_DIR: &str = "resources/dcat-us-v3/";

    if !profile.validation.enabled {
        return Vec::new();
    }

    if let Some(dir) = &profile.validation.schema_dir
        && !dir.is_empty()
        && dir.trim_end_matches('/') != EMBEDDED_SCHEMA_DIR.trim_end_matches('/')
    {
        return vec![ProjectionWarning {
            field:    "dcat_validate".to_string(),
            severity: Severity::Recommended,
            message:  format!(
                "profile `{}` declares schema_dir=`{}` but qsv currently only ships the embedded \
                 GSA DCAT-US v3 bundle at `{EMBEDDED_SCHEMA_DIR}`. JSON Schema validation against \
                 custom bundles is a queued follow-up; the projection was emitted but not \
                 validated.",
                profile.name, dir
            ),
        }];
    }

    let prefixes: Vec<&str> = profile
        .validation
        .strippable_curie_prefixes
        .iter()
        .map(String::as_str)
        .collect();
    let stripped = strip_curies(block, &prefixes);
    let is_catalog = block.get("@type").and_then(Value::as_str).is_some_and(|t| {
        t.eq_ignore_ascii_case("dcat:Catalog") || t.eq_ignore_ascii_case("Catalog")
    });

    let validator = if is_catalog {
        catalog_validator()
    } else {
        dataset_validator()
    };

    match validator {
        Ok(v) => v
            .iter_errors(&stripped)
            .map(|err| {
                let path = err.instance_path().to_string();
                let field = path.trim_start_matches('/').to_string();
                let kind = format!("{:?}", err.kind());
                ProjectionWarning {
                    field,
                    severity: classify_severity(&kind),
                    message: format!("{err}"),
                }
            })
            .collect(),
        Err(e) => vec![ProjectionWarning {
            field:    "dcat_validate".to_string(),
            severity: Severity::Required,
            message:  e.to_string(),
        }],
    }
}

/// Deep clone `v` with every object key whose CURIE prefix matches
/// one in `prefixes` replaced by the unprefixed local name. The
/// validator needs ownership and the emitted block (the one written
/// to disk) must keep the compact form. Inlined here so the legacy
/// `curie.rs` module can be deleted.
fn strip_curies(v: &Value, prefixes: &[&str]) -> Value {
    use serde_json::Map;
    match v {
        Value::Object(map) => {
            let mut out = Map::with_capacity(map.len());
            for (k, child) in map {
                let new_key = strip_curie_key(k, prefixes);
                out.insert(new_key, strip_curies(child, prefixes));
            }
            Value::Object(out)
        },
        Value::Array(items) => {
            Value::Array(items.iter().map(|c| strip_curies(c, prefixes)).collect())
        },
        _ => v.clone(),
    }
}

fn strip_curie_key(key: &str, prefixes: &[&str]) -> String {
    for p in prefixes {
        if let Some(local) = key.strip_prefix(p) {
            return local.to_string();
        }
    }
    key.to_string()
}

/// Classify a `jsonschema::ValidationError` kind as Required or
/// Recommended. Errors keyed off `Required` (missing mandatory
/// property in a vendored schema's `required` array) get
/// `Severity::Required`; everything else (pattern mismatches,
/// type mismatches, enum violations on Recommended fields) gets
/// `Severity::Recommended`.
fn classify_severity(kind_str: &str) -> Severity {
    if kind_str.contains("Required") {
        Severity::Required
    } else {
        Severity::Recommended
    }
}

// -----------------------------------------------------------------------------
// Minimal-schema fallback (kept for offline / future use)
// -----------------------------------------------------------------------------

/// Hand-written minimal v3 schema covering only the mandatory keys
/// from the spec landing page. Used by older call sites and as a
/// safety net for diagnostics — `validate_dataset_or_catalog`
/// defaults to the vendored GSA bundle.
///
/// Validates against the JSON-LD-compact (CURIE-prefixed) keys
/// `dcat::build` emits — does NOT need `curie::strip_curies`.
#[allow(dead_code)]
pub(super) fn embedded_minimal_schema() -> Value {
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
            "dcat:distribution",
        ],
        "properties": {
            "@type":           {"type": "string", "const": "dcat:Dataset"},
            "dct:title":       {"type": "string", "minLength": 1},
            "dct:description": {"type": "string", "minLength": 1},
            "dct:identifier":  {"type": "string", "minLength": 1},
            "dct:publisher": {
                "type":     "object",
                "required": ["@type", "foaf:name"],
            },
            "dcat:contactPoint": {
                "type":     "object",
                "required": ["@type", "vcard:fn", "vcard:hasEmail"],
                "properties": {
                    "vcard:hasEmail": {"type": "string", "pattern": "^mailto:"},
                },
            },
            "dct:conformsTo": {
                "type":     "array",
                "minItems": 1,
                "items": {
                    "type":     "object",
                    "required": ["@type", "@id"],
                    "properties": {
                        "@type": {"type": "string", "const": "dct:Standard"},
                    },
                },
            },
            "dcat:distribution": {"type": "array", "minItems": 1},
        },
    })
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn minimal_valid_dataset() -> Value {
        // Every mandatory v3 field populated, in JSON-LD-compact form
        // (what dcat::build emits). Used as the golden seed for tests
        // that mutate one field at a time.
        json!({
            "@type":          "dcat:Dataset",
            "dct:title":      "Test Dataset",
            "dct:description": "A test dataset.",
            "dct:identifier": "test-id-001",
            "dct:publisher": {
                "@type":     "foaf:Organization",
                "foaf:name": "Test Agency",
            },
            "dcat:contactPoint": {
                "@type":          "vcard:Individual",
                "vcard:fn":       "Test Contact",
                "vcard:hasEmail": "mailto:test@example.gov",
            },
            "dct:conformsTo": [{
                "@type": "dct:Standard",
                "@id":   "https://resources.data.gov/resources/dcat-us3/",
            }],
            "dcat:distribution": [{
                "@type":            "dcat:Distribution",
                "dct:title":        "CSV",
                "dcat:downloadURL": "https://example.gov/d.csv",
            }],
        })
    }

    #[test]
    fn bundle_validator_compiles_for_dataset_overlay() {
        // First-call latency check: building the validator with $ref
        // resolution through BundleRetriever must succeed.
        assert!(
            dataset_validator().is_ok(),
            "dataset overlay must compile cleanly against the vendored bundle"
        );
    }

    #[test]
    fn bundle_validator_compiles_for_catalog_overlay() {
        assert!(
            catalog_validator().is_ok(),
            "catalog overlay must compile cleanly against the vendored bundle"
        );
    }

    #[test]
    fn minimal_dataset_passes_full_bundle() {
        // Regression guard for the CURIE-strip bridge: a fully-populated
        // minimal dataset in compact form must validate clean after
        // curie::strip_curies. If this starts failing, either the bundle
        // was refreshed with new mandatory fields or the curie module
        // is mis-mapping a prefix.
        let profile = super::super::profile_spec::load("dcat-us-v3").unwrap();
        let warnings = validate(&profile, &minimal_valid_dataset());
        assert!(
            warnings.is_empty(),
            "minimal valid dataset must pass: warnings = {warnings:#?}",
        );
    }

    #[test]
    fn dropping_publisher_yields_required_severity() {
        // Sanity: removing one of the five GSA-mandatory fields must
        // surface as Severity::Required (not Recommended).
        let mut ds = minimal_valid_dataset();
        ds.as_object_mut().unwrap().remove("dct:publisher");
        let profile = super::super::profile_spec::load("dcat-us-v3").unwrap();
        let warnings = validate(&profile, &ds);
        assert!(!warnings.is_empty());
        assert!(
            warnings.iter().any(|w| w.severity == Severity::Required),
            "missing publisher must produce a Required-severity warning, got: {warnings:#?}",
        );
    }

    #[test]
    fn dcat_us_extensions_pass_through_overlay() {
        // dcat-us:bureauCode and dcat-us:programCode must validate
        // clean — the overlay defines the shape; the GSA bundle's
        // permissive additionalProperties default does the rest.
        let mut ds = minimal_valid_dataset();
        let obj = ds.as_object_mut().unwrap();
        obj.insert("dcat-us:bureauCode".to_string(), json!(["015:11"]));
        obj.insert("dcat-us:programCode".to_string(), json!(["015:001"]));
        let profile = super::super::profile_spec::load("dcat-us-v3").unwrap();
        let warnings = validate(&profile, &ds);
        assert!(
            warnings.is_empty(),
            "dcat-us:* extensions must validate clean, got: {warnings:#?}",
        );
    }

    #[test]
    fn dispatches_to_catalog_validator_when_type_is_catalog() {
        // Constructing a Catalog envelope and validating it. The only
        // mandatory key per GSA Catalog.json is `dataset` — which is
        // `dcat:dataset` in compact form. Should pass.
        let cat = json!({
            "@type":     "dcat:Catalog",
            "dct:title": "Test Catalog",
            "dcat:dataset": [minimal_valid_dataset()],
        });
        let profile = super::super::profile_spec::load("dcat-us-v3").unwrap();
        let warnings = validate(&profile, &cat);
        assert!(
            warnings.is_empty(),
            "minimal valid catalog must pass: warnings = {warnings:#?}",
        );
    }

    #[test]
    fn classify_severity_flags_required_kind() {
        assert_eq!(
            classify_severity("Required(\"contactPoint\")"),
            Severity::Required
        );
        assert_eq!(classify_severity("Pattern(...)"), Severity::Recommended);
        assert_eq!(classify_severity("Enum(...)"), Severity::Recommended);
    }

    #[test]
    fn bundle_carries_every_referenced_definition() {
        // Smoke test: each name we expect Dataset.json / Catalog.json
        // to $ref must appear in BUNDLE. Guards against accidentally
        // pruning an entry during a refresh — if the build_validator
        // call returns a "missing" reference error on prepare(), the
        // bundle is incomplete relative to what the GSA schemas
        // require.
        for expected in &[
            "concept",
            "agent",
            "kind",
            "distribution",
            "location",
            "identifier",
            "checksum",
            "standard",
            "periodoftime",
        ] {
            assert!(
                BUNDLE.iter().any(|e| e.name == *expected),
                "BUNDLE must contain definition `{expected}`",
            );
        }
    }
}
