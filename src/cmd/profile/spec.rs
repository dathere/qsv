//! Parser for CKAN [scheming](https://github.com/ckan/ckanext-scheming) YAML
//! specification files (e.g. DP+'s `dataset-druf.yaml`).
//!
//! Scheming specs are intentionally open-ended — CKAN sites bolt on their own
//! `preset`, `validators`, `dpp_*`, and other keys. To keep this parser
//! useful across spec variants, we only type the keys we need to *act* on
//! (the field name and the Jinja2 formula keys) and preserve everything else
//! verbatim in an `extras` map so it round-trips into the output.

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::CliResult;

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // some fields are preserved for round-tripping into the output rather than read in Rust
pub struct Spec {
    #[serde(default)]
    pub scheming_version: Option<u32>,
    #[serde(default)]
    pub dataset_type:     Option<String>,
    #[serde(default)]
    pub about:            Option<String>,
    #[serde(default)]
    pub about_url:        Option<String>,
    #[serde(default)]
    pub dataset_fields:   Vec<Field>,
    #[serde(default)]
    pub resource_fields:  Vec<Field>,
    /// Any other top-level keys (display_group_order, draft_fields_required, ...)
    /// are preserved here so the output can round-trip them.
    #[serde(flatten)]
    pub extras:           Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // `label` and `extras` are surfaced via the spec output rather than read in Rust
pub struct Field {
    /// Field identifier (CKAN-side column name). Required for normal fields;
    /// `None` for the synthetic "page-break" entries that only carry a
    /// `start_form_page` block.
    #[serde(default)]
    pub field_name: Option<String>,

    /// Optional human-readable label.
    #[serde(default)]
    pub label: Option<String>,

    /// Jinja2 template that, when evaluated, becomes the *value* of this field
    /// on package/resource creation or update.
    #[serde(default)]
    pub formula: Option<String>,

    /// Jinja2 template that, when evaluated, becomes a *suggestion* the user
    /// can accept/modify. Collected under the package-level `dpp_suggestions`
    /// JSON object.
    #[serde(default)]
    pub suggestion_formula: Option<String>,

    /// All other keys (preset, validators, choices, form_*, start_form_page,
    /// dpp_*, …) preserved verbatim.
    #[serde(flatten)]
    pub extras: Map<String, Value>,
}

impl Field {
    /// True when the entry is a real field (has a `field_name`), as opposed to
    /// a page-break / form-section entry. Only real fields participate in
    /// formula evaluation and JSON output.
    #[inline]
    pub fn is_real(&self) -> bool {
        self.field_name.is_some()
    }
}

/// Parse a scheming YAML file from disk.
pub fn load_from_path(path: &str) -> CliResult<Spec> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| crate::CliError::Other(format!("could not read --spec file `{path}`: {e}")))?;
    load_from_str(&raw, path)
}

/// Parse a scheming YAML from an in-memory string. `source_label` is used only
/// for error messages.
pub fn load_from_str(yaml: &str, source_label: &str) -> CliResult<Spec> {
    yaml_serde::from_str::<Spec>(yaml).map_err(|e| {
        crate::CliError::Other(format!("could not parse --spec file `{source_label}`: {e}"))
    })
}

/// Convenience: return all real (non page-break) dataset fields.
impl Spec {
    pub fn real_dataset_fields(&self) -> impl Iterator<Item = &Field> {
        self.dataset_fields.iter().filter(|f| f.is_real())
    }

    pub fn real_resource_fields(&self) -> impl Iterator<Item = &Field> {
        self.resource_fields.iter().filter(|f| f.is_real())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// dataset-druf.yaml has one `suggestion_formula` on `spatial_extent`,
    /// the rest of the fields are non-formula.
    const DRUF_SAMPLE: &str = r#"
scheming_version: 2
dataset_type: dataset
about: test
draft_fields_required: false
display_group_order: ["Basic Info"]
dataset_fields:
  - start_form_page:
      title: Basic Info
      description: required
    field_name: title
    label: Title
    preset: title
    required: True
  - field_name: spatial_extent
    label: Spatial Extent
    validators: scheming_required
    suggestion_formula: '{{ spatial_extent_wkt() }}'
resource_fields:
  - field_name: url
    label: URL
    preset: resource_url_upload
"#;

    #[test]
    fn parses_druf_sample() {
        let spec = load_from_str(DRUF_SAMPLE, "<test>").expect("parse");
        assert_eq!(spec.scheming_version, Some(2));
        assert_eq!(spec.dataset_type.as_deref(), Some("dataset"));
        // page-break entry + 2 real fields = 2 entries (the start_form_page
        // collapses into the title entry's extras, since YAML merges keys at
        // the same map level — confirmed by the fixture).
        assert_eq!(spec.dataset_fields.len(), 2);
        assert_eq!(spec.real_dataset_fields().count(), 2);

        // spatial_extent must carry its suggestion_formula.
        let se = spec
            .dataset_fields
            .iter()
            .find(|f| f.field_name.as_deref() == Some("spatial_extent"))
            .expect("spatial_extent field");
        assert_eq!(
            se.suggestion_formula.as_deref(),
            Some("{{ spatial_extent_wkt() }}")
        );
        assert!(se.formula.is_none());

        // resource fields parsed.
        assert_eq!(spec.real_resource_fields().count(), 1);

        // top-level extras preserved (display_group_order, draft_fields_required).
        assert!(spec.extras.contains_key("display_group_order"));
        assert!(spec.extras.contains_key("draft_fields_required"));
    }

    /// Vendored copy of dathere/datapusher-plus@main:
    ///   ckanext/datapusher_plus/dataset-druf.yaml
    /// kept under tests/resources/profile/.
    const REAL_DRUF: &str = include_str!("../../../tests/resources/profile/dataset-druf.yaml");

    #[test]
    fn parses_real_dataset_druf_fixture() {
        let spec = load_from_str(REAL_DRUF, "dataset-druf.yaml").expect("parse");
        assert_eq!(spec.scheming_version, Some(2));
        assert_eq!(spec.dataset_type.as_deref(), Some("dataset"));

        // Counts locked to the vendored snapshot. dataset_fields has two
        // entries that carry a `start_form_page` header alongside their
        // field_name (title, update_type) — those still count as real fields.
        assert_eq!(spec.real_dataset_fields().count(), 9);
        assert_eq!(spec.real_resource_fields().count(), 8);

        // The one suggestion_formula in the real fixture.
        let se = spec
            .dataset_fields
            .iter()
            .find(|f| f.field_name.as_deref() == Some("spatial_extent"))
            .expect("spatial_extent field");
        assert_eq!(
            se.suggestion_formula.as_deref(),
            Some("{{ spatial_extent_wkt() }}")
        );

        // dpp_locale on the resource side carries no formula, just docs/extras.
        let locale = spec
            .resource_fields
            .iter()
            .find(|f| f.field_name.as_deref() == Some("dpp_locale"))
            .expect("dpp_locale field");
        assert!(locale.formula.is_none());
        assert!(locale.suggestion_formula.is_none());
        assert!(locale.extras.contains_key("form_placeholder"));
    }
}
