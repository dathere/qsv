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
///
/// The `geoconnex` profile is gated behind the `geoconnex` cargo
/// feature: present in qsv (default via `distrib_features`) and as an
/// opt-in for qsvdp (`-F datapusher_plus,geoconnex`), absent from
/// qsvlite / qsvmcp. The two `cfg`'d table definitions below differ
/// only by that one tuple; users who hit `--profile geoconnex` on a
/// build without the feature get the standard "unknown profile" error
/// from `load()` listing the actually-bundled names.
#[cfg(not(feature = "geoconnex"))]
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

#[cfg(feature = "geoconnex")]
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
    (
        "geoconnex",
        include_str!("../../../resources/profiles/geoconnex.yaml"),
    ),
];

/// Returns the names of all embedded profiles, in declaration order.
/// Used for help text and error-message suggestions.
pub fn list_embedded() -> Vec<&'static str> {
    EMBEDDED.iter().map(|(n, _)| *n).collect()
}

impl ProfileSpec {
    /// Translate a CKAN-side pointer (e.g. `/package/title`) to its
    /// target counterpart (e.g. `/projection/dct:title`) per the profile's
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

    /// Where this profile was loaded from. NOT deserialized — set
    /// by `load()` after parsing. Direct `load_from_str` callers
    /// (tests, in-memory construction) get the `Embedded` default
    /// so unit tests don't trip the orchestrator's untrusted-source
    /// gate; real CLI usage always goes through `load()`, which
    /// overrides this with the resolved source.
    #[serde(skip)]
    pub source: ProfileSource,
}

// -----------------------------------------------------------------------
// Sub-blocks
// -----------------------------------------------------------------------

/// One CKAN→target pointer mapping. Both pointers are RFC 6901-ish; the
/// CKAN side is rooted under `/package/...` or `/resource/...` and the
/// target side is a JSON pointer into the emitted projection block. All
/// bundled profiles share the `/projection/...` root regardless of their
/// JSON-LD vocabulary (e.g. `/projection/dct:title` for DCAT-US v3,
/// `/projection/name` for Croissant, `/projection/schema:name` for
/// Geoconnex).
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
    /// Optional out-of-process validator (e.g. `mlcroissant`,
    /// `pyshacl`). Runs orthogonal to the built-in JSON-Schema
    /// validator gated by `enabled`: a profile may set
    /// `enabled: false` (no JSON Schema) but still configure
    /// `external` for vocabulary-specific validation. When the
    /// configured `command` isn't on `PATH`, validation degrades
    /// gracefully to a single `Severity::Info` warning rather than
    /// failing the projection.
    #[serde(default)]
    pub external:                  Option<ExternalValidator>,
}

/// Out-of-process validator config. The command is spawned with the
/// rendered JSON-LD written to a tempfile; the file path is
/// substituted for the literal `{file}` token in `args`, or appended
/// as the last argument when no `{file}` token is present.
///
/// Additional input files (e.g. SHACL shapes for `pyshacl`) can be
/// declared via `resources`. Each entry is looked up in
/// `external_validate::EMBEDDED_RESOURCES` (a compile-time
/// `include_str!`-bundled set), written to a tempfile with the
/// declared suffix, and substituted for `{<name>}` in `args`.
/// Custom YAML profiles can reference any embedded name but
/// cannot register new ones (the set is fixed at qsv release
/// time, matching how `EMBEDDED` profiles are fixed).
///
/// A non-zero exit code is treated as "validation failed" — each
/// non-empty stderr line becomes one `ProjectionWarning` with
/// severity `default_severity`. Exit code zero is treated as
/// success regardless of stdout/stderr content.
#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)]
pub struct ExternalValidator {
    /// Command to spawn. Resolved via `PATH`. When the command
    /// isn't found, validation emits a single `Info`-severity
    /// warning ("`<command>` not installed; skipped validation")
    /// rather than failing — keeps profiles that rely on an
    /// optional Python tool usable in plain-Rust environments.
    pub command:          String,
    /// Arguments. The literal token `{file}` is replaced with the
    /// tempfile path holding the rendered JSON-LD. When no `{file}`
    /// token is present, the path is appended as the last argument.
    /// Named tokens declared in `resources` (e.g. `{shapes}`) are
    /// substituted with the corresponding tempfile path.
    #[serde(default)]
    pub args:             Vec<String>,
    /// Severity assigned to each surfaced finding. One of
    /// `"required"`, `"recommended"` (default), `"optional"`,
    /// `"info"`. Case-insensitive.
    #[serde(default)]
    pub default_severity: Option<String>,
    /// Optional label used in warning messages instead of the raw
    /// command name. Lets a profile show "mlcroissant" even when
    /// the command is actually `python3 -m mlcroissant ...`.
    #[serde(default)]
    pub label:            Option<String>,
    /// Optional install hint appended to the missing-binary
    /// warning. Free-form text — typically a short install
    /// command and/or a URL pointing at the validator's homepage.
    /// E.g. "pip install mlcroissant (https://github.com/mlcommons/croissant)".
    #[serde(default)]
    pub install_hint:     Option<String>,
    /// Additional input files materialized as tempfiles before
    /// spawn. Each entry's `embedded` name resolves against the
    /// `external_validate::EMBEDDED_RESOURCES` table; its content
    /// is written to a tempfile with the declared `suffix` and
    /// substituted for `{<name>}` in `args`. Names must NOT be
    /// `"file"` (reserved for the implicit JSON-LD tempfile).
    #[serde(default)]
    pub resources:        Vec<ExternalValidatorResource>,
}

/// One named tempfile resource for an `ExternalValidator`. The
/// content is sourced from a compile-time `include_str!`-bundled
/// table (`external_validate::EMBEDDED_RESOURCES`) so custom YAML
/// profiles can reference SHACL shapes / shape catalogs / etc. by
/// name without re-shipping the underlying bytes. The framework
/// validates `embedded` resolves at spawn time and emits a
/// `Required`-severity warning otherwise.
#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)]
pub struct ExternalValidatorResource {
    /// Token name. Used in `args` as `{<name>}`. Reserved value:
    /// `"file"` (would shadow the implicit JSON-LD path).
    pub name:     String,
    /// Logical embedded-resource identifier. Resolved against
    /// `external_validate::EMBEDDED_RESOURCES` at spawn time.
    pub embedded: String,
    /// File suffix for the materialized tempfile (e.g. `.ttl`,
    /// `.jsonld`, `.xml`). Default `.tmp`. The suffix helps
    /// validators that key off file extension to pick a parser.
    #[serde(default)]
    pub suffix:   Option<String>,
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
    /// minijinja template producing the catalog's title. The inner
    /// Dataset is exposed as `inner["..."]` (e.g.  `inner["dct:title"]`
    /// for DCAT, `inner["schema:name"]` for schema.org-rooted profiles),
    /// and the analysis ctx (`pkg`/`res`/`stats`/`dpp`/...) is available
    /// too. When unset, the title falls back to the legacy "Catalog of
    /// <inner title>" convention which reads from whichever key the
    /// profile declares as `title_key`.
    #[serde(default)]
    pub title_template:       Option<String>,
    /// JSON-LD key under which the catalog title is emitted on the
    /// envelope. Defaults to `dct:title` (DCAT-shaped). Schema.org-
    /// rooted profiles (Croissant, Geoconnex) should set this to
    /// `schema:name` so the envelope doesn't carry DCAT terms its
    /// `@context` doesn't define.
    #[serde(default)]
    pub title_key:            Option<String>,
    /// JSON-LD key under which the inner Dataset is wrapped on the
    /// envelope. Defaults to `dcat:dataset` (DCAT-shaped). Schema.org-
    /// rooted profiles should set this to `schema:dataset` for
    /// envelope/context consistency.
    #[serde(default)]
    pub dataset_key:          Option<String>,
    /// Top-level Dataset keys to copy onto the Catalog envelope
    /// verbatim (typical: `dct:publisher`). For renaming,
    /// transforming, or conditional inheritance use a `fields[]`
    /// entry whose template references `inner["<key>"]` instead;
    /// catalog templates have full access to that binding plus the
    /// analysis ctx.
    #[serde(default)]
    pub inherit_from_dataset: Vec<String>,
    /// The `dct:conformsTo` target IRI.
    #[serde(default)]
    pub conforms_to:          Option<String>,
    /// Catalog-only / template-driven fields. Each entry is a
    /// regular `FieldDecl`; templates render with `inner` (the
    /// rendered Dataset block) injected on top of the analysis ctx,
    /// so you can do `{{ inner["dct:publisher"] | tojson }}` for
    /// transform-style inheritance.
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

/// Tag identifying where the profile was loaded from. Set by
/// `load()`; used by the orchestrator to gate trust-sensitive
/// features (e.g. spawning external validators).
///
/// The `Embedded` default is for tests / direct `load_from_str`
/// callers that bypass `load()`. Real CLI invocations always go
/// through `load()`, which sets the source explicitly.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProfileSource {
    /// Bundled with the qsv binary (`EMBEDDED` table). Trusted at
    /// build time; safe to spawn declared validators without an
    /// explicit per-invocation opt-in.
    #[default]
    Embedded,
    /// Loaded from a YAML file on disk. Treated as untrusted: the
    /// orchestrator requires an explicit CLI opt-in
    /// (`--allow-external-validator`) before spawning any binary
    /// the file declares.
    FilePath,
}

impl ProfileSource {
    /// True when the profile shipped with the qsv binary.
    pub fn is_embedded(self) -> bool {
        matches!(self, Self::Embedded)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct DiscoveryMerge {
    /// When `false`, discovery-merge is a no-op for this profile.
    #[serde(default = "default_true")]
    pub enabled:            bool,
    /// Top-level keys never overwritten by discovered metadata.
    /// Defaults include `@context`, `@type`, and the distribution array.
    #[serde(default)]
    pub never_overwrite:    Vec<String>,
    /// Strategy applied to keys not in `never_overwrite`.
    /// Possible values: `fill-if-absent` (default), `overlay-array`,
    /// `never`.
    #[serde(default)]
    pub default_strategy:   Option<String>,
    /// Optional per-element merge config for the distribution array.
    /// When `Some(cfg)` with `cfg.enabled = true`, the distribution
    /// key bypasses the `never_overwrite` early-return and each
    /// publisher Distribution is matched against an inferred
    /// Distribution by identity keys (e.g. `dcat:accessURL`),
    /// allowing publisher metadata (titles, rights, formats) to
    /// flow into the inferred record without losing qsv's stats.
    /// See `DistributionMerge` for details.
    #[serde(default)]
    pub distribution_merge: Option<DistributionMerge>,
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
            enabled:            true,
            never_overwrite:    vec![
                "@context".to_string(),
                "@type".to_string(),
                "dcat:distribution".to_string(),
            ],
            default_strategy:   Some("fill-if-absent".to_string()),
            distribution_merge: None,
        }
    }
}

/// Per-element merge config for the distribution array. When
/// `enabled = true`, the discovery-merge engine bypasses the outer
/// `never_overwrite` rule for `array_key` and walks the discovered
/// distribution array element-by-element, matching each publisher
/// Distribution against the inferred array via `identity_keys`.
///
/// A typical config for DCAT profiles:
/// ```yaml
/// distribution_merge:
///   enabled: true
///   array_key: "dcat:distribution"
///   identity_keys: ["dcat:downloadURL", "dcat:accessURL", "@id"]
///   field_strategy: fill-if-absent
///   append_unmatched: false
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)]
pub struct DistributionMerge {
    /// Master switch. `false` (default) preserves the legacy
    /// "inferred-only" behavior.
    #[serde(default)]
    pub enabled:          bool,
    /// Top-level key that holds the distribution array. Defaults
    /// to `dcat:distribution`. Croissant-style profiles can point
    /// this at `distribution` (schema.org bare-name).
    #[serde(default)]
    pub array_key:        Option<String>,
    /// Ordered list of fields used to match a publisher distribution
    /// against an inferred one. The first identity key with a
    /// non-empty value on both sides triggers the match. Empty
    /// `identity_keys` disables matching (every publisher dist is
    /// treated as unmatched).
    #[serde(default)]
    pub identity_keys:    Vec<String>,
    /// Strategy for merging fields within a matched pair. Same
    /// values as `DiscoveryMerge::default_strategy`: `fill-if-absent`
    /// (default), `overlay-array`, or `never`.
    #[serde(default)]
    pub field_strategy:   Option<String>,
    /// When `true`, publisher distributions that don't match any
    /// inferred distribution are appended to the array. Default
    /// `false` — qsv's inferred distributions are usually canonical
    /// for the local data, and unmatched publisher entries often
    /// describe resources we don't see (mirrors, alternate formats).
    #[serde(default)]
    pub append_unmatched: bool,
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
/// The returned profile is `dry_compile`-validated (every template
/// is parsed by minijinja so a malformed YAML fails here rather
/// than mid-projection) and has its `source` field stamped with
/// the resolution path — `Embedded` for bundled profiles, `FilePath`
/// for on-disk YAML. The orchestrator reads `source` to gate
/// trust-sensitive features such as external validator spawn.
/// Tests + low-level callers that want raw parsing without
/// compilation can still use `load_from_str` directly (those keep
/// the `Embedded` default).
pub fn load(arg: &str) -> CliResult<ProfileSpec> {
    let needle = arg.to_ascii_lowercase();
    let (mut profile, source) = if let Some((_, raw)) = EMBEDDED
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(&needle))
    {
        (load_from_str(raw, arg)?, ProfileSource::Embedded)
    } else {
        let path = std::path::Path::new(arg);
        if path.is_file() {
            let raw = std::fs::read_to_string(path).map_err(|e| {
                CliError::Other(format!("could not read --profile file `{arg}`: {e}"))
            })?;
            (load_from_str(&raw, arg)?, ProfileSource::FilePath)
        } else {
            return Err(CliError::Other(format!(
                "unknown --profile `{arg}`; embedded profiles: [{}]. To use a file path, point at \
                 an existing .yaml file.",
                list_embedded().join(", "),
            )));
        }
    };
    profile.source = source;
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
    fn embedded_dcat_ap_v3_parses_and_dry_compiles() {
        let spec = load("dcat-ap-v3").expect("embedded load");
        assert_eq!(spec.name, "dcat-ap-v3");
        // Built-in JSON-Schema validator stays off; DCAT-AP ships
        // SHACL upstream and the external validator below wires it
        // up to pyshacl.
        assert!(!spec.validation.enabled);
        assert!(spec.discovery_merge.enabled);
        assert!(!spec.dataset.fields.is_empty());
        assert!(spec.distribution.is_some());
        assert!(spec.catalog.is_some());
        assert_eq!(spec.field_mappings.len(), 28);
        // EU theme vocab is the AP-specific addition.
        assert!(spec.vocabularies.contains_key("eu_theme"));
        assert!(spec.vocabularies.contains_key("license_iri"));
        assert!(spec.vocabularies.contains_key("accrual_periodicity"));
        assert!(spec.vocabularies.contains_key("iso_639_1"));
        assert!(spec.vocabularies.contains_key("csvw_datatype"));
        // External pyshacl validator is configured and references
        // the embedded SHACL shapes by name.
        let external = spec
            .validation
            .external
            .as_ref()
            .expect("dcat-ap-v3 should declare external pyshacl validator");
        assert_eq!(external.command, "pyshacl");
        let shapes_resource = external
            .resources
            .iter()
            .find(|r| r.name == "shapes")
            .expect("shapes resource must be declared");
        assert_eq!(shapes_resource.embedded, "dcat-ap-v3-shacl-shapes");
        assert_eq!(shapes_resource.suffix.as_deref(), Some(".ttl"));
        super::super::projection::dry_compile(&spec).expect("dry_compile");
    }

    #[test]
    fn embedded_croissant_parses_and_dry_compiles() {
        let spec = load("croissant").expect("embedded load");
        assert_eq!(spec.name, "croissant");
        // Croissant relies on the mlcommons Python validator; built-in
        // JSON-Schema validator + discovery merge are both disabled.
        assert!(!spec.validation.enabled);
        assert!(!spec.discovery_merge.enabled);
        // External validator is configured (mlcroissant).
        let external = spec
            .validation
            .external
            .as_ref()
            .expect("croissant should declare external validator");
        assert_eq!(external.command, "mlcroissant");
        assert!(external.args.iter().any(|a| a == "validate"));
        assert!(external.args.iter().any(|a| a.contains("{file}")));
        assert!(!spec.dataset.fields.is_empty());
        assert!(spec.distribution.is_some());
        assert!(spec.catalog.is_some());
        // Croissant is the only profile that uses the recordsets block
        // (per-column cr:Field expansion).
        assert!(!spec.recordsets.is_empty());
        assert_eq!(spec.field_mappings.len(), 16);
        assert!(spec.vocabularies.contains_key("license_iri"));
        assert!(spec.vocabularies.contains_key("croissant_datatype"));
        super::super::projection::dry_compile(&spec).expect("dry_compile");
    }

    /// Parametric CI gate: every entry in `EMBEDDED` must parse cleanly
    /// and `dry_compile` without error. Adding a new profile to the
    /// bundle automatically inherits this coverage — no per-profile
    /// test edit required.
    #[test]
    fn all_embedded_profiles_parse_and_dry_compile() {
        assert!(
            !EMBEDDED.is_empty(),
            "EMBEDDED must contain at least one profile"
        );
        for (name, _) in EMBEDDED {
            let spec = load(name)
                .unwrap_or_else(|e| panic!("embedded profile `{name}` failed to load: {e}"));
            assert_eq!(
                spec.name.to_ascii_lowercase(),
                name.to_ascii_lowercase(),
                "embedded profile `{name}` reports mismatched spec.name `{}`",
                spec.name
            );
            // Every shipped profile must have a non-empty dataset block;
            // a profile with no dataset fields would produce empty output.
            assert!(
                !spec.dataset.fields.is_empty(),
                "embedded profile `{name}` has no dataset fields"
            );
            super::super::projection::dry_compile(&spec)
                .unwrap_or_else(|e| panic!("embedded profile `{name}` failed dry_compile: {e}"));
        }
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
