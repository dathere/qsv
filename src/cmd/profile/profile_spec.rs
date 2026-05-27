//! Profile YAML parser for the projection engine.
//!
//! A `ProfileSpec` is a single YAML file declaring how a target metadata
//! standard (DCAT-US v3, DCAT-AP v3, Croissant, agency-specific) is
//! projected from the qsv analysis context. It carries:
//!
//! * named vocabularies (string lookup tables for license / frequency / language / datatype IRIs)
//! * a CKAN→target pointer mapping table (replaces the legacy hardcoded `CKAN_TO_DCAT` constant)
//! * a `dataset:` and `distribution:` block listing field declarations, each carrying a minijinja
//!   template + cardinality / placement directives
//! * an optional `catalog:` envelope spec (used when `--catalog` is set)
//! * `validation:` (schema bundle path + entry points + CURIE strip list)
//! * `discovery_merge:` rules (which keys never overwrite, default strategy)
//! * optional `recordsets:` block (Croissant-specific: one `cr:RecordSet` per Dataset with one
//!   `cr:Field` per CSV column)
//!
//! Embedded profiles ship under `resources/profiles/<name>.yaml` and are
//! resolved via `load(arg)` ahead of file paths (so `--profile
//! dcat-us-v3` always finds the bundled copy).

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{CliError, CliResult};

// -----------------------------------------------------------------------
// Embedded profiles
// -----------------------------------------------------------------------

/// Canonical profile YAMLs shipped with the binary. Resolution order in
/// `load()`: embedded by name (case-insensitive), then file path on
/// disk. Adding an entry here makes that profile available via
/// `--profile <name>` without an external file.
///
/// Profiles ship under `resources/profiles/<name>.yaml`. Each profile is
/// validated at load time (template syntax checked, vocabulary references
/// resolved) so a malformed bundled profile causes a hard error rather
/// than silently producing degraded output.
pub const EMBEDDED: &[(&str, &str)] = &[
    (
        "dcat-us-v3",
        include_str!("../../../resources/profiles/dcat-us-v3.yaml"),
    ),
    (
        "dcat-ap-v3",
        include_str!("../../../resources/profiles/dcat-ap-v3.yaml"),
    ),
    (
        "croissant",
        include_str!("../../../resources/profiles/croissant.yaml"),
    ),
];

/// Returns the names of all embedded profiles, in declaration order.
/// Used for help text and error-message suggestions.
pub fn list_embedded() -> Vec<&'static str> {
    EMBEDDED.iter().map(|(n, _)| *n).collect()
}

impl ProfileSpec {
    /// Translate a CKAN-side pointer (e.g. `/package/title`) to its
    /// target counterpart (e.g. `/dcat/dct:title`) per the profile's
    /// `field_mappings:` table. Returns `None` when the CKAN pointer
    /// isn't mapped (the legacy `ckan_to_dcat::translate_ckan_ptr`
    /// behavior). Used by `context.rs::collect_forced_paths` for the
    /// force-override pathway.
    pub fn translate_ckan_ptr(&self, ckan_ptr: &str) -> Option<&str> {
        self.field_mappings
            .iter()
            .find(|m| m.ckan == ckan_ptr)
            .map(|m| m.target.as_str())
    }
}

// -----------------------------------------------------------------------
// Top-level spec
// -----------------------------------------------------------------------

/// Parsed contents of a profile YAML file.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // some fields are surfaced via output.profile_meta only
pub struct ProfileSpec {
    pub name:      String,
    #[serde(default)]
    pub version:   Option<String>,
    #[serde(default)]
    pub about:     Option<String>,
    #[serde(default, rename = "about_url")]
    pub about_url: Option<String>,

    /// Named lookup tables. Templates access these via `lookup("table",
    /// key)`. Values may be strings or arbitrary JSON objects; the
    /// helper returns the raw value as a minijinja `Value`.
    #[serde(default)]
    pub vocabularies: Map<String, Value>,

    /// CKAN-pointer → target-pointer entries. Used by force-override
    /// path (`profile.rs::apply_force_overrides`) and by the warning
    /// filter to map raw CKAN paths into the projection's address space.
    /// Declaration order matters: later entries override earlier ones
    /// when two CKAN pointers resolve to the same target pointer.
    #[serde(default)]
    pub field_mappings: Vec<FieldMapping>,

    /// Schema validation strategy. When `enabled: false` (DCAT-AP v3,
    /// Croissant), `validate()` returns an empty warning list.
    #[serde(default)]
    pub validation: Validation,

    /// Dataset-level projection (the main output block).
    pub dataset: DatasetBlock,

    /// Per-Distribution projection. Each Distribution shares the same
    /// field declarations; profiles that need per-resource specialization
    /// use `for_each_column:` or template-side conditionals.
    #[serde(default)]
    pub distribution: Option<DistributionBlock>,

    /// `--catalog` envelope spec. When absent, `--catalog` falls back to
    /// the legacy minimal `dcat:Catalog` shell.
    #[serde(default)]
    pub catalog: Option<CatalogBlock>,

    /// Discovery-merge policy. When unset, defaults to
    /// `fill-if-absent` with `@context` / `@type` protected.
    #[serde(default)]
    pub discovery_merge: DiscoveryMerge,

    /// Optional Croissant-specific block: one entry per RecordSet to
    /// emit. Each entry expands via `for_each_column` into one `cr:Field`
    /// per stats column.
    #[serde(default)]
    pub recordsets: Vec<RecordSetSpec>,

    /// Any other top-level keys preserved verbatim so user-authored
    /// profiles can round-trip vendor extensions.
    #[serde(flatten)]
    pub extras: Map<String, Value>,
}

// -----------------------------------------------------------------------
// Sub-blocks
// -----------------------------------------------------------------------

/// One CKAN→target pointer mapping. Both pointers are RFC 6901-ish; the
/// CKAN side is rooted under `/package/...` or `/resource/...` and the
/// target side under the projection's address space (e.g. `/dcat/...`
/// for DCAT-US v3, top-level keys for Croissant).
#[derive(Debug, Clone, Deserialize)]
pub struct FieldMapping {
    pub ckan:   String,
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)] // surface fields not yet consumed reserved for future expansion
pub struct Validation {
    #[serde(default)]
    pub enabled:                   bool,
    #[serde(default)]
    pub schema_dir:                Option<String>,
    #[serde(default)]
    pub entry_dataset:             Option<String>,
    #[serde(default)]
    pub entry_catalog:             Option<String>,
    /// CURIE prefixes whose `<prefix>:<localname>` keys collapse to
    /// `<localname>` before schema validation. Mirrors the legacy
    /// `STRIPPABLE_PREFIXES` const.
    #[serde(default)]
    pub strippable_curie_prefixes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct DatasetBlock {
    #[serde(default, rename = "type")]
    pub type_:   Option<String>,
    /// JSON-LD `@context`. A string is emitted as a single context
    /// URI (typical DCAT shape); an object is emitted verbatim
    /// (Croissant ships a multi-key context with `@vocab` + prefix
    /// shorthands).
    #[serde(default)]
    pub context: Option<Value>,
    #[serde(default)]
    pub fields:  Vec<FieldDecl>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct DistributionBlock {
    #[serde(default, rename = "type")]
    pub type_:  Option<String>,
    /// JSON-LD key under which the Distribution array is emitted.
    /// Default: `dcat:distribution`. Croissant uses bare
    /// `distribution` (schema.org's @vocab resolves it).
    #[serde(default)]
    pub path:   Option<String>,
    #[serde(default)]
    pub fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct CatalogBlock {
    #[serde(default, rename = "type")]
    pub type_:                Option<String>,
    /// minijinja template producing the catalog's `dct:title`. The
    /// inner Dataset is available as `inner["..."]` for inheritance.
    #[serde(default)]
    pub title_template:       Option<String>,
    /// Top-level Dataset keys to copy into the Catalog envelope
    /// (typical: `dct:publisher`).
    #[serde(default)]
    pub inherit_from_dataset: Vec<String>,
    /// The `dct:conformsTo` target IRI.
    #[serde(default)]
    pub conforms_to:          Option<String>,
    /// Extra catalog-only fields beyond the title/conformsTo/dataset
    /// trio.
    #[serde(default)]
    pub fields:               Vec<FieldDecl>,
}

/// One field declaration inside `dataset:` / `distribution:` / `catalog:`.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct FieldDecl {
    /// JSON-LD key emitted into the projection (e.g. `dct:title`).
    pub path:            String,
    /// Minijinja expression evaluated against the analysis context.
    /// Strings starting with `{` are parsed as JSON; otherwise treated
    /// as literal strings.
    pub template:        String,
    /// `required` / `recommended` / `optional`. Empty render with no
    /// `default` produces a `ProjectionWarning` with matching severity.
    #[serde(default)]
    pub required_level:  Option<RequiredLevel>,
    /// Placement override: emit on the Dataset block? Default true.
    #[serde(default = "default_true")]
    pub on_dataset:      bool,
    /// Placement override: emit on each Distribution? Default false.
    #[serde(default)]
    pub on_distribution: bool,
    /// Guard expression. When this template renders falsy/empty, the
    /// field is skipped without emitting a warning.
    #[serde(default)]
    pub emit_when:       Option<String>,
    /// Literal value to emit when the main template renders empty.
    /// Skips the warning.
    #[serde(default)]
    pub default:         Option<Value>,
    /// Per-stats-column expansion (Croissant `cr:Field`). When set, the
    /// template runs once per stats column; the column-record is
    /// exposed as `column.*` in scope.
    #[serde(default)]
    pub for_each_column: bool,
}

fn default_true() -> bool {
    true
}

/// Required-level tag for `FieldDecl`. Determines warning severity when
/// the rendered value is empty.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequiredLevel {
    Required,
    Recommended,
    Optional,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct DiscoveryMerge {
    /// When `false`, discovery-merge is a no-op for this profile.
    #[serde(default = "default_true")]
    pub enabled:          bool,
    /// Top-level keys never overwritten by discovered metadata.
    /// Defaults include `@context`, `@type`, and the distribution array.
    #[serde(default)]
    pub never_overwrite:  Vec<String>,
    /// Strategy applied to keys not in `never_overwrite`.
    /// Possible values: `fill-if-absent` (default), `overlay-array`,
    /// `never`.
    #[serde(default)]
    pub default_strategy: Option<String>,
}

// Hand-rolled Default so callers that build a ProfileSpec without
// deserializing (e.g. tests, in-memory construction) see the same
// "fill-if-absent enabled" behavior documented for omitted
// `discovery_merge:` blocks. The serde `default = "default_true"`
// annotation only fires during deserialization; #[derive(Default)]
// would leave `enabled: false` which silently disables merging.
impl Default for DiscoveryMerge {
    fn default() -> Self {
        Self {
            enabled:          true,
            never_overwrite:  vec![
                "@context".to_string(),
                "@type".to_string(),
                "dcat:distribution".to_string(),
            ],
            default_strategy: Some("fill-if-absent".to_string()),
        }
    }
}

/// Croissant-specific RecordSet declaration. The engine emits one
/// `cr:RecordSet` per entry; if `for_each_column` is set, each field in
/// the RecordSet expands to one `cr:Field` per CSV column.
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct RecordSetSpec {
    /// JSON-LD path under the dataset where this RecordSet is emitted.
    pub path:            String,
    /// If true, generate one entry per stats column. Each entry runs
    /// `template` with `column.*` in scope.
    #[serde(default)]
    pub for_each_column: bool,
    /// Minijinja template producing the field/RecordSet body.
    pub template:        String,
}

// -----------------------------------------------------------------------
// Loading
// -----------------------------------------------------------------------

/// Resolve `arg` to a parsed `ProfileSpec`. Lookup order:
///
/// 1. Embedded by case-insensitive name (`dcat-us-v3` matches `DCAT-US-v3`, `Dcat-Us-V3`, etc.).
///    The case-insensitive match is deliberate so users don't have to remember the canonical
///    capitalization.
/// 2. File path on disk. Relative paths are resolved against the current working directory.
///
/// Returns a `CliError::Other` with a helpful list of bundled names when
/// neither lookup succeeds.
///
/// The returned profile is also `dry_compile`-validated — every
/// template inside the profile is parsed by minijinja so a malformed
/// embedded YAML (or a user-supplied file with a typo) fails at
/// `load` time rather than mid-projection. This matches the doc
/// claim on the `EMBEDDED` constant. Tests + low-level call sites
/// that want raw parsing without compilation can still use
/// `load_from_str` directly.
pub fn load(arg: &str) -> CliResult<ProfileSpec> {
    let needle = arg.to_ascii_lowercase();
    let profile = if let Some((_, raw)) = EMBEDDED
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(&needle))
    {
        load_from_str(raw, arg)?
    } else {
        let path = std::path::Path::new(arg);
        if path.is_file() {
            let raw = std::fs::read_to_string(path).map_err(|e| {
                CliError::Other(format!("could not read --profile file `{arg}`: {e}"))
            })?;
            load_from_str(&raw, arg)?
        } else {
            return Err(CliError::Other(format!(
                "unknown --profile `{arg}`; embedded profiles: [{}]. To use a file path, point at \
                 an existing .yaml file.",
                list_embedded().join(", "),
            )));
        }
    };
    // Validate every template at load time — malformed profiles
    // surface here, not deep inside a render. Errors are wrapped to
    // include the profile name + label.
    super::projection::dry_compile(&profile)?;
    Ok(profile)
}

/// Parse a profile YAML from an in-memory string. `source_label` is used
/// only for error messages.
pub fn load_from_str(yaml: &str, source_label: &str) -> CliResult<ProfileSpec> {
    yaml_serde::from_str::<ProfileSpec>(yaml)
        .map_err(|e| CliError::Other(format!("could not parse --profile `{source_label}`: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL_YAML: &str = r#"
name: test-profile
version: 0.1.0
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "{{ pkg.title }}"
      required_level: required
"#;

    #[test]
    fn parses_minimal_yaml() {
        let spec = load_from_str(MINIMAL_YAML, "<test>").expect("parse");
        assert_eq!(spec.name, "test-profile");
        assert_eq!(spec.version.as_deref(), Some("0.1.0"));
        assert_eq!(spec.dataset.fields.len(), 1);
        assert_eq!(spec.dataset.fields[0].path, "dct:title");
        assert!(matches!(
            spec.dataset.fields[0].required_level,
            Some(RequiredLevel::Required)
        ));
    }

    #[test]
    fn embedded_resolves_case_insensitively() {
        let cases = ["dcat-us-v3", "DCAT-US-v3", "Dcat-Us-V3"];
        for c in cases {
            let spec = load(c).expect(c);
            assert_eq!(spec.name.to_ascii_lowercase(), "dcat-us-v3");
        }
    }

    #[test]
    fn embedded_dcat_us_v3_parses_and_dry_compiles() {
        let spec = load("dcat-us-v3").expect("embedded load");
        assert_eq!(spec.name, "dcat-us-v3");
        assert!(spec.validation.enabled);
        assert!(!spec.dataset.fields.is_empty());
        assert!(spec.distribution.is_some());
        assert!(spec.catalog.is_some());
        assert_eq!(spec.field_mappings.len(), 53);
        // Vocabularies populated.
        assert!(spec.vocabularies.contains_key("license_iri"));
        assert!(spec.vocabularies.contains_key("accrual_periodicity"));
        assert!(spec.vocabularies.contains_key("iso_639_1"));
        assert!(spec.vocabularies.contains_key("csvw_datatype"));
        // Templates all compile.
        super::super::projection::dry_compile(&spec).expect("dry_compile");
    }

    #[test]
    fn unknown_name_errors_with_list() {
        let err = load("not-a-real-profile").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unknown --profile"));
        assert!(msg.contains("dcat-us-v3"));
        assert!(msg.contains("croissant"));
    }

    #[test]
    fn malformed_yaml_errors_with_location() {
        let yaml = r#"
name: bad
dataset:
  fields:
    - path: dct:title
      template: : invalid
"#;
        let err = load_from_str(yaml, "test").unwrap_err();
        assert!(err.to_string().contains("could not parse --profile"));
    }

    #[test]
    fn file_path_resolves() {
        // Write a temp file and load it via `load()`.
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("custom-profile.yaml");
        std::fs::write(&path, MINIMAL_YAML).expect("write");
        let spec = load(path.to_str().unwrap()).expect("load");
        assert_eq!(spec.name, "test-profile");
    }

    #[test]
    fn list_embedded_returns_known_profiles() {
        let names = list_embedded();
        assert!(names.contains(&"dcat-us-v3"));
        assert!(names.contains(&"dcat-ap-v3"));
        assert!(names.contains(&"croissant"));
    }
}
