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
//! ## No CURIE bridge
//!
//! Earlier revisions emitted JSON-LD-compact, CURIE-prefixed keys
//! (`dct:title`, `dcat:contactPoint`) and stripped the prefixes in
//! memory before validating, because the GSA bundle keys everything
//! unprefixed. That bridge is gone: DCAT-US v3 is plain JSON
//! validated by JSON Schema — GSA removed the JSON-LD context and
//! the SHACL shapes upstream — so the projection now emits the same
//! unprefixed keys the schemas declare and is validated verbatim.
//!
//! The only prefixed keys left are deliberate qsv extensions, listed
//! in [`EXTENSION_KEYS`].
//!
//! ## Why the unknown-key lint exists
//!
//! No schema in the GSA bundle sets `additionalProperties`, so an
//! unknown key — a typo, a stale property upstream has since removed,
//! or a key under the wrong namespace — validates silently. The lint
//! in [`lint_unknown_keys`] closes that hole; it is the only thing
//! that catches a misnamed key, and it is how the pre-conformance
//! `dcat:describedBy` / `dcat:versionNotes` / dead `@context` bugs
//! would have been caught. Upstream ships the same idea as
//! `jsonschema/check_undefined_fields.py`.

use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};

use jsonschema::Validator;
use serde_json::Value;

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

    // The emitted block is already keyed the way the schemas are, so
    // it is validated verbatim — no CURIE stripping (see module docs).
    let is_catalog = block.get("@type").and_then(Value::as_str).is_some_and(|t| {
        t.eq_ignore_ascii_case("dcat:Catalog") || t.eq_ignore_ascii_case("Catalog")
    });

    let validator = if is_catalog {
        catalog_validator()
    } else {
        dataset_validator()
    };

    let mut findings = match validator {
        Ok(v) => v
            .iter_errors(block)
            .map(|err| {
                let path = err.instance_path().to_string();
                let field = path.trim_start_matches('/').to_string();
                let kind = format!("{:?}", err.kind());
                ProjectionWarning {
                    field,
                    severity: classify_severity(&kind, &path),
                    message: format!("{err}"),
                }
            })
            .collect(),
        Err(e) => vec![ProjectionWarning {
            field:    "dcat_validate".to_string(),
            severity: Severity::Required,
            message:  e.to_string(),
        }],
    };

    findings.extend(lint_unknown_keys(block, is_catalog));
    findings
}

/// Report emitted keys the target schema does not declare.
///
/// The bundle never sets `additionalProperties`, so unknown keys pass
/// validation silently — this is the only check that catches a typo,
/// a property upstream has removed, or a key under the wrong
/// namespace.
///
/// ## Scope
///
/// Deliberately bounded to the node types qsv actually constructs:
/// the root (Catalog or Dataset), each Dataset inside a Catalog's
/// `dataset` array, and each Distribution inside a Dataset's
/// `distribution` array. It does NOT descend into nested value
/// objects (`publisher`, `contactPoint`, `landingPage`, the
/// restriction objects) — those are `$ref`-typed and already shape-
/// checked by the schema's own `anyOf`/`$ref` machinery, which does
/// report violations. The gap this closes is specifically *unknown
/// top-level property names*, which nothing else reports.
fn lint_unknown_keys(block: &Value, is_catalog: bool) -> Vec<ProjectionWarning> {
    let mut out = Vec::new();
    if is_catalog {
        check_node(block, "catalog", "", &mut out);
        if let Some(datasets) = block.get("dataset").and_then(Value::as_array) {
            for (i, ds) in datasets.iter().enumerate() {
                check_node(ds, "dataset", &format!("dataset/{i}/"), &mut out);
                lint_distributions(ds, &format!("dataset/{i}/"), &mut out);
            }
        }
    } else {
        check_node(block, "dataset", "", &mut out);
        lint_distributions(block, "", &mut out);
    }
    out
}

fn lint_distributions(dataset: &Value, prefix: &str, out: &mut Vec<ProjectionWarning>) {
    if let Some(dists) = dataset.get("distribution").and_then(Value::as_array) {
        for (i, d) in dists.iter().enumerate() {
            check_node(
                d,
                "distribution",
                &format!("{prefix}distribution/{i}/"),
                out,
            );
        }
    }
}

/// Compare one node's keys against `definitions/<def_name>.json`.
fn check_node(node: &Value, def_name: &str, path_prefix: &str, out: &mut Vec<ProjectionWarning>) {
    let Some(obj) = node.as_object() else {
        return;
    };
    let Some(known) = declared_properties(def_name) else {
        return;
    };
    for key in obj.keys() {
        if known.contains(key.as_str()) || EXTENSION_KEYS.contains(&key.as_str()) {
            continue;
        }
        out.push(ProjectionWarning {
            field:    format!("{path_prefix}{key}"),
            severity: Severity::Recommended,
            message:  format!(
                "`{key}` is not a DCAT-US v3 {def_name} property and is not a known qsv \
                 extension. It will be ignored by data.gov harvesters. Check the spelling, or \
                 whether the property was removed upstream."
            ),
        });
    }
}

/// Property names declared by a vendored definition, plus the JSON-LD
/// node keywords every class permits.
fn declared_properties(def_name: &str) -> Option<&'static HashSet<String>> {
    static INDEX: OnceLock<HashMap<String, HashSet<String>>> = OnceLock::new();
    INDEX
        .get_or_init(|| {
            let mut idx = HashMap::new();
            for e in BUNDLE {
                let Ok(schema) = serde_json::from_str::<Value>(e.json) else {
                    continue;
                };
                let mut names: HashSet<String> = schema
                    .get("properties")
                    .and_then(Value::as_object)
                    .map(|p| p.keys().cloned().collect())
                    .unwrap_or_default();
                names.insert("@id".to_string());
                names.insert("@type".to_string());
                idx.insert(e.name.to_string(), names);
            }
            idx
        })
        .get(def_name)
}

/// Prefixed keys qsv emits on purpose.
///
/// `dcat-us:bureauCode` / `dcat-us:programCode` / `dcat-us:accessLevel`
/// are NOT DCAT-US v3 properties — they are confirmed absent from all
/// 26 definitions. They are kept because agencies still need them for
/// OMB M-13-13 / Project Open Data, and because the v1.1 -> v3
/// migration guide (Step 5) explicitly says to keep `accessLevel`
/// during the transition: "the v3.0 schema will not reject it".
///
/// `qsv:sourcePath` and `csvw:tableSchema` are qsv's own additions.
/// `csvw:tableSchema`'s interior is arbitrary qsv-shaped JSON, which
/// is why the lint never descends into nested value objects.
const EXTENSION_KEYS: &[&str] = &[
    "dcat-us:bureauCode",
    "dcat-us:programCode",
    "dcat-us:accessLevel",
    "qsv:sourcePath",
    "csvw:tableSchema",
];

/// Map a validation error onto a severity using the GSA bundle's own
/// `requirementLevel` annotations rather than the shape of the
/// jsonschema error.
///
/// The old classifier substring-matched the Debug-formatted error
/// kind for "Required", which collapsed every `type`, `enum`,
/// `pattern` and `anyOf` failure to `Recommended` regardless of how
/// important the property was. The bundle already states the answer
/// per property, so use it.
///
/// The instance-path-to-property mapping is not total, so the chain
/// is explicit:
///
/// 1. A `required` violation reports the *parent* object as its instance path, not the missing key,
///    so the property name is parsed out of the error kind and looked up directly.
/// 2. Otherwise walk the instance path from the leaf toward the root and take the first segment
///    that names a property carrying a `requirementLevel`. This skips array indices and
///    intermediate anonymous objects.
/// 3. No match falls back to `Recommended` — never silently `Optional`/`Info`, which would hide a
///    real finding.
fn classify_severity(kind_str: &str, instance_path: &str) -> Severity {
    // (1) `required` violations name the missing property in the kind.
    if kind_str.contains("Required") {
        if let Some(prop) = required_property_from_kind(kind_str)
            && let Some(level) = requirement_level(&prop)
        {
            return level;
        }
        // A mandatory key is missing but the bundle gave us no level:
        // a missing `required` entry is Required by definition.
        return Severity::Required;
    }

    // (2) walk leaf -> root for the nearest property with a level.
    for seg in instance_path.split('/').rev() {
        if seg.is_empty() || seg.parse::<usize>().is_ok() {
            continue;
        }
        if let Some(level) = requirement_level(seg) {
            return level;
        }
    }

    // (3) fallback.
    Severity::Recommended
}

/// Pull the missing property name out of a Debug-formatted
/// `ValidationErrorKind::Required { property: String("title") }`.
fn required_property_from_kind(kind_str: &str) -> Option<String> {
    let start = kind_str.find("String(\"")? + "String(\"".len();
    let rest = &kind_str[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// `requirementLevel` for a property name, as declared anywhere in
/// the vendored bundle.
///
/// The index is flat (property name, not class-qualified). That is
/// deliberate: DCAT-US reuses property names consistently across
/// classes at the same level — `title` is Mandatory on Dataset and
/// Recommended on Distribution — so a flat index can disagree with
/// the class actually being validated. Conflicts resolve to the
/// STRONGER level, which keeps `--strict` conservative: it may
/// over-report, never under-report.
fn requirement_level(property: &str) -> Option<Severity> {
    static INDEX: OnceLock<HashMap<String, Severity>> = OnceLock::new();
    INDEX
        .get_or_init(|| {
            let mut idx: HashMap<String, Severity> = HashMap::new();
            for e in BUNDLE {
                let Ok(schema) = serde_json::from_str::<Value>(e.json) else {
                    continue;
                };
                let Some(props) = schema.get("properties").and_then(Value::as_object) else {
                    continue;
                };
                for (name, decl) in props {
                    let Some(level) = decl.get("requirementLevel").and_then(Value::as_str) else {
                        continue;
                    };
                    let sev = match level {
                        "Mandatory" => Severity::Required,
                        "Recommended" => Severity::Recommended,
                        _ => Severity::Optional,
                    };
                    idx.entry(name.clone())
                        .and_modify(|cur| {
                            if stronger(sev, *cur) {
                                *cur = sev;
                            }
                        })
                        .or_insert(sev);
                }
            }
            idx
        })
        .get(property)
        .copied()
}

/// Ordering helper for the "conflicts resolve to the stronger level"
/// rule in [`requirement_level`].
fn stronger(a: Severity, b: Severity) -> bool {
    fn rank(s: Severity) -> u8 {
        match s {
            Severity::Required => 3,
            Severity::Recommended => 2,
            Severity::Optional => 1,
            Severity::Info => 0,
        }
    }
    rank(a) > rank(b)
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
