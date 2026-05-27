//! Generic, YAML-driven metadata projection engine.
//!
//! Replaces the hardcoded `dcat.rs` + `catalog.rs` projection. Given a
//! `ProfileSpec` and the analysis context produced by `context.rs`,
//! `project()` evaluates each declared field's minijinja template,
//! coerces the rendered string into either a JSON value or a literal
//! string, and assembles a single JSON-LD-shaped block ready for
//! validation / output.
//!
//! Warnings collected during projection (missing required fields,
//! template render errors, vocabulary misses) flow back as
//! `ProjectionWarning` entries so the orchestrator can re-emit them in
//! the `dcat_warnings` array.

use minijinja::Environment;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use super::{
    formula_helpers,
    profile_spec::{CatalogBlock, FieldDecl, ProfileSpec, RecordSetSpec, RequiredLevel},
};
use crate::{CliError, CliResult};

// -----------------------------------------------------------------------
// Public types
// -----------------------------------------------------------------------

/// Which top-level shape the engine assembles.
///
/// `Dataset` produces the bare Dataset block (default); `Catalog` wraps
/// the Dataset inside the profile's `catalog:` envelope (used by
/// `--catalog`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionMode {
    Dataset,
    Catalog,
}

/// Severity-tagged warning surfaced during projection. The orchestrator
/// concatenates these with `dcat_validate::validate()` output so users
/// see all problems in one `dcat_warnings` array.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionWarning {
    /// JSON-LD key (`dct:title`) or full pointer (`dcat:distribution/0/dct:license`)
    pub field:    String,
    pub severity: Severity,
    pub message:  String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Required,
    Recommended,
    Optional,
    Info,
}

impl From<RequiredLevel> for Severity {
    fn from(level: RequiredLevel) -> Self {
        match level {
            RequiredLevel::Required => Severity::Required,
            RequiredLevel::Recommended => Severity::Recommended,
            RequiredLevel::Optional => Severity::Optional,
        }
    }
}

// -----------------------------------------------------------------------
// Engine entry point
// -----------------------------------------------------------------------

/// Project the analysis `ctx` through `profile`'s field declarations.
///
/// Returns the assembled JSON-LD block and the list of warnings the
/// projection produced (missing required fields, render errors, etc.).
pub fn project(
    profile: &ProfileSpec,
    ctx: &Value,
    mode: ProjectionMode,
) -> CliResult<(Value, Vec<ProjectionWarning>)> {
    let mut env = Environment::new();
    // ChainableUndefined lets templates walk through missing
    // intermediate keys (`pkg.dpp_suggestions.spatial_extent.value`)
    // without raising, so `| default("")` actually catches the miss.
    // Mirrors the legacy `dcat.rs` engine's behavior where absent
    // CKAN keys silently fall through to defaults.
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Chainable);
    formula_helpers::register(&mut env);
    register_profile_helpers(&mut env, profile);

    let mj_ctx = minijinja::Value::from_serialize(ctx);
    let mut warnings: Vec<ProjectionWarning> = Vec::new();

    // 1. Build the Dataset core. Fields with `on_dataset: false` skip.
    let mut dataset: Map<String, Value> = Map::new();
    if let Some(type_) = &profile.dataset.type_ {
        dataset.insert("@type".to_string(), Value::String(type_.clone()));
    }
    if let Some(context) = &profile.dataset.context {
        // Profile's `context:` can be a string URI (typical DCAT) or
        // a JSON-LD object (Croissant ships an inline @context map).
        dataset.insert("@context".to_string(), context.clone());
    }
    for field in &profile.dataset.fields {
        if !field.on_dataset {
            continue;
        }
        emit_field(&env, &mj_ctx, field, &mut dataset, &mut warnings);
    }

    // 2. Build Distribution[0] from `distribution:` block + any `dataset:` field with
    //    `on_distribution: true`.
    if let Some(dist_block) = &profile.distribution {
        let mut dist: Map<String, Value> = Map::new();
        if let Some(type_) = &dist_block.type_ {
            dist.insert("@type".to_string(), Value::String(type_.clone()));
        }
        for field in &dist_block.fields {
            emit_field(&env, &mj_ctx, field, &mut dist, &mut warnings);
        }
        // Pull cross-placed fields from dataset block.
        for field in &profile.dataset.fields {
            if field.on_distribution {
                emit_field(&env, &mj_ctx, field, &mut dist, &mut warnings);
            }
        }
        if !dist.is_empty() {
            // Profile may override the wrapping key (Croissant uses
            // schema.org bare `distribution`; DCAT defaults to
            // `dcat:distribution`).
            let key = dist_block.path.as_deref().unwrap_or("dcat:distribution");
            dataset.insert(key.to_string(), Value::Array(vec![Value::Object(dist)]));
        }
    }

    // 3. Croissant-style RecordSets (`recordsets:` block).
    if !profile.recordsets.is_empty() {
        let stats = ctx
            .pointer("/dpps")
            .or_else(|| ctx.pointer("/stats"))
            .cloned()
            .unwrap_or(Value::Null);
        for rs in &profile.recordsets {
            emit_recordset(&env, &mj_ctx, rs, &stats, &mut dataset, &mut warnings);
        }
    }

    let dataset_value = Value::Object(dataset);

    // 4. Catalog envelope if requested.
    let block = match mode {
        ProjectionMode::Dataset => dataset_value,
        ProjectionMode::Catalog => {
            wrap_as_catalog(&env, ctx, profile, dataset_value, &mut warnings)
        },
    };

    Ok((block, warnings))
}

/// Wrap a Dataset block inside the profile's Catalog envelope.
///
/// Companion to `project(..., Dataset)` — call this after
/// `discovery_merge::merge` has applied so the discovered metadata
/// lands on the inner Dataset (Roborev #2490 finding #1), not on the
/// outer Catalog envelope. The returned Catalog block carries its own
/// `@context` (mirrored from the Dataset block) so the envelope is
/// itself valid JSON-LD.
///
/// Re-uses the existing `wrap_as_catalog` helper which already
/// understands `profile.catalog`. We build a fresh minijinja
/// environment because the helper only needs it for the optional
/// `title_template` + any catalog-only fields. `analysis_ctx` flows
/// through so catalog templates can reach the same `pkg`/`res`/`stats`
/// values the Dataset templates see, plus an injected `inner` (the
/// rendered Dataset block) for template-driven inheritance.
pub fn wrap_in_catalog_envelope(
    profile: &ProfileSpec,
    dataset: Value,
    analysis_ctx: &Value,
) -> CliResult<(Value, Vec<ProjectionWarning>)> {
    let mut env = Environment::new();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Chainable);
    formula_helpers::register(&mut env);
    register_profile_helpers(&mut env, profile);
    let mut warnings: Vec<ProjectionWarning> = Vec::new();
    let block = wrap_as_catalog(&env, analysis_ctx, profile, dataset, &mut warnings);
    Ok((block, warnings))
}

// -----------------------------------------------------------------------
// Field emission
// -----------------------------------------------------------------------

fn emit_field(
    env: &Environment,
    ctx: &minijinja::Value,
    field: &FieldDecl,
    out: &mut Map<String, Value>,
    warnings: &mut Vec<ProjectionWarning>,
) {
    // Optional emit_when guard.
    if let Some(guard) = &field.emit_when
        && !render_truthy(env, ctx, guard)
    {
        return;
    }

    let rendered = match render_to_string(env, ctx, &field.template) {
        Ok(s) => s,
        Err(e) => {
            warnings.push(ProjectionWarning {
                field:    field.path.clone(),
                severity: Severity::Required,
                message:  format!("template render error: {e}"),
            });
            return;
        },
    };

    let trimmed = rendered.trim();
    if trimmed.is_empty() {
        if let Some(default) = &field.default {
            out.insert(field.path.clone(), default.clone());
            return;
        }
        if let Some(level) = field.required_level {
            warnings.push(ProjectionWarning {
                field:    field.path.clone(),
                severity: Severity::from(level),
                message:  format!("missing {}", field.path),
            });
        }
        return;
    }

    let value = coerce_json_or_string(&rendered);
    out.insert(field.path.clone(), value);
}

fn emit_recordset(
    env: &Environment,
    ctx: &minijinja::Value,
    rs: &RecordSetSpec,
    stats: &Value,
    out: &mut Map<String, Value>,
    warnings: &mut Vec<ProjectionWarning>,
) {
    if !rs.for_each_column {
        // Whole-template render, no expansion.
        match render_to_string(env, ctx, &rs.template) {
            Ok(s) if !s.trim().is_empty() => {
                set_by_simple_path(out, &rs.path, coerce_json_or_string(&s));
            },
            Ok(_) => {},
            Err(e) => warnings.push(ProjectionWarning {
                field:    rs.path.clone(),
                severity: Severity::Recommended,
                message:  format!("recordset render error: {e}"),
            }),
        }
        return;
    }

    // Expand per stats column.
    let Some(columns) = stats.as_array() else {
        return;
    };
    let mut entries: Vec<Value> = Vec::with_capacity(columns.len());
    for col in columns {
        let scoped = json!({ "column": col });
        let mj_scoped = minijinja::Value::from_serialize(&scoped);
        match render_to_string(env, &mj_scoped, &rs.template) {
            Ok(s) if !s.trim().is_empty() => {
                entries.push(coerce_json_or_string(&s));
            },
            Ok(_) => {},
            Err(e) => warnings.push(ProjectionWarning {
                field:    rs.path.clone(),
                severity: Severity::Recommended,
                message:  format!("recordset column render error: {e}"),
            }),
        }
    }
    if !entries.is_empty() {
        set_by_simple_path(out, &rs.path, Value::Array(entries));
    }
}

// -----------------------------------------------------------------------
// Catalog envelope
// -----------------------------------------------------------------------

pub fn wrap_as_catalog(
    env: &Environment,
    analysis_ctx: &Value,
    profile: &ProfileSpec,
    dataset: Value,
    warnings: &mut Vec<ProjectionWarning>,
) -> Value {
    let envelope = profile.catalog.as_ref();
    let cat_type = envelope
        .and_then(|c| c.type_.clone())
        .unwrap_or_else(|| "dcat:Catalog".to_string());

    // Build a ctx that exposes the inner Dataset alongside the regular
    // analysis vars (pkg/res/stats/dpp/etc.). Catalog templates use
    // this so they can pull values from the rendered Dataset via
    // `inner["dct:title"]` while still seeing the underlying source
    // (`pkg.title`, etc.). The ctx clone is cheap (serde_json::Value
    // is reference-counted for strings under the hood) and constructed
    // once per catalog wrap.
    let ctx_with_inner = build_ctx_with_inner(analysis_ctx, &dataset);
    let mj_with_inner = minijinja::Value::from_serialize(&ctx_with_inner);

    // Title derived from the catalog template if any; otherwise fall back
    // to the legacy "Catalog of <dct:title>" convention. The title
    // template now sees the same ctx_with_inner as catalog.fields, so
    // it can reference pkg/res/stats in addition to inner[...] (a
    // backward-compatible superset — legacy templates that only used
    // `inner` keep working unchanged).
    let title = if let Some(CatalogBlock {
        title_template: Some(tpl),
        ..
    }) = envelope
    {
        match render_to_string(env, &mj_with_inner, tpl) {
            Ok(s) if !s.trim().is_empty() => s,
            _ => legacy_catalog_title(&dataset),
        }
    } else {
        legacy_catalog_title(&dataset)
    };

    let mut catalog = serde_json::Map::new();

    // Roborev #2490: the Catalog envelope carries CURIE keys
    // (`dct:title`, `dct:conformsTo`, `dcat:dataset`, plus any
    // inherited Dataset keys) so it needs its own `@context`.
    // Without one, the envelope itself isn't valid JSON-LD.
    // Source the context from the profile's dataset block — the inner
    // Dataset still keeps its own `@context` for self-containment, so
    // the redundancy is intentional (JSON-LD allows nested
    // re-declaration).
    if let Some(context) = &profile.dataset.context {
        catalog.insert("@context".to_string(), context.clone());
    }
    catalog.insert("@type".to_string(), Value::String(cat_type));
    catalog.insert("dct:title".to_string(), Value::String(title));

    if let Some(envelope) = envelope
        && let Some(conforms_to) = &envelope.conforms_to
    {
        catalog.insert(
            "dct:conformsTo".to_string(),
            json!({ "@type": "dct:Standard", "@id": conforms_to }),
        );
    }

    // Inherit declared keys from the Dataset (verbatim copy by key).
    // For template-driven inheritance — renaming, conditional copy,
    // transforming values — use a `catalog.fields[]` entry instead;
    // those render with `inner` exposed so they can do
    // `{{ inner["dct:publisher"] | tojson }}` etc.
    if let Some(envelope) = envelope {
        for key in &envelope.inherit_from_dataset {
            if let Some(v) = dataset.get(key) {
                catalog.insert(key.clone(), v.clone());
            }
        }

        // Additional catalog-only / template-driven fields. Render
        // with the ctx_with_inner so templates can reference both
        // the analysis vars (`pkg`, `res`, `stats`, `dpp`) and the
        // rendered Dataset (`inner["..."]`).
        for field in &envelope.fields {
            emit_field(env, &mj_with_inner, field, &mut catalog, warnings);
        }
    }

    catalog.insert("dcat:dataset".to_string(), Value::Array(vec![dataset]));

    Value::Object(catalog)
}

/// Builds a catalog-scope rendering context by cloning the analysis
/// JSON ctx and inserting `inner` (the rendered Dataset block). Falls
/// back to a fresh object when the analysis ctx is not an object
/// (e.g. tests that pass `json!({})` and shouldn't see a panic).
fn build_ctx_with_inner(analysis_ctx: &Value, dataset: &Value) -> Value {
    match analysis_ctx {
        Value::Object(map) => {
            let mut out = map.clone();
            out.insert("inner".to_string(), dataset.clone());
            Value::Object(out)
        },
        _ => json!({ "inner": dataset }),
    }
}

fn legacy_catalog_title(dataset: &Value) -> String {
    dataset
        .get("dct:title")
        .and_then(Value::as_str)
        .map(|t| format!("Catalog of {t}"))
        .unwrap_or_else(|| "qsv profile catalog".to_string())
}

// -----------------------------------------------------------------------
// Template helpers
// -----------------------------------------------------------------------

fn render_to_string(
    env: &Environment,
    ctx: &minijinja::Value,
    src: &str,
) -> Result<String, minijinja::Error> {
    env.template_from_str(src)?.render(ctx)
}

fn render_truthy(env: &Environment, ctx: &minijinja::Value, src: &str) -> bool {
    match render_to_string(env, ctx, src) {
        Ok(s) => {
            let t = s.trim();
            !t.is_empty() && t != "false" && t != "False" && t != "0" && t != "None"
        },
        Err(_) => false,
    }
}

/// Render result coercion: a string that starts with `{` or `[` is
/// parsed as JSON; anything else is taken as a literal string.
pub fn coerce_json_or_string(rendered: &str) -> Value {
    let trimmed = rendered.trim();
    if (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
    {
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            return v;
        }
    }
    Value::String(rendered.to_string())
}

/// Walk a `/`-rooted simple path and set the leaf. Currently supports
/// only direct top-level keys; nested paths fall back to flat insertion
/// at the path (escape-free).
fn set_by_simple_path(out: &mut Map<String, Value>, path: &str, value: Value) {
    let key = path.trim_start_matches('/');
    out.insert(key.to_string(), value);
}

// -----------------------------------------------------------------------
// Profile-aware minijinja helpers (lookup, field_mapping)
// -----------------------------------------------------------------------

/// Register the two profile-aware helpers (`lookup`, `field_mapping`)
/// that need access to the `ProfileSpec` instance. The remaining 11
/// helpers in §4 of the plan are profile-agnostic and live directly in
/// `formula_helpers::register`.
fn register_profile_helpers(env: &mut Environment, profile: &ProfileSpec) {
    let vocabs = profile.vocabularies.clone();
    env.add_function(
        "lookup",
        move |table: &str, key: &str| -> minijinja::Value {
            lookup_in_vocab(&vocabs, table, key).unwrap_or(minijinja::Value::UNDEFINED)
        },
    );
    let mappings = profile.field_mappings.clone();
    env.add_function("field_mapping", move |ckan_ptr: &str| -> minijinja::Value {
        mappings
            .iter()
            .find(|m| m.ckan == ckan_ptr)
            .map(|m| minijinja::Value::from(m.target.clone()))
            .unwrap_or(minijinja::Value::UNDEFINED)
    });
}

fn lookup_in_vocab(
    vocabs: &Map<String, Value>,
    table: &str,
    key: &str,
) -> Option<minijinja::Value> {
    let entry = vocabs.get(table)?.as_object()?;
    // Case-insensitive lookup, with RFC 5646 subtag stripping for
    // language tables.
    let key_lower = key.to_ascii_lowercase();
    let primary = key.split('-').next().unwrap_or(key).to_ascii_lowercase();
    let candidates: [&str; 3] = [key, &key_lower, &primary];
    for candidate in candidates {
        if let Some(v) = entry.get(candidate) {
            return Some(minijinja::Value::from_serialize(v));
        }
        // Case-insensitive scan for non-canonical capitalization.
        if let Some((_k, v)) = entry
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(candidate))
        {
            return Some(minijinja::Value::from_serialize(v));
        }
    }
    None
}

// -----------------------------------------------------------------------
// Compile-time validation (called by load() for fail-fast on bad embeds)
// -----------------------------------------------------------------------

/// Best-effort syntax check of every template in `profile`. Returns
/// `Ok(())` on success; surfaces the first compile error otherwise.
///
/// Called by `profile_spec::load` so a malformed embedded profile
/// fails at binary startup, not deep inside a render. Checks both the
/// main `template` AND the optional `emit_when` guard on every field
/// (dataset, distribution, catalog), plus the catalog title template
/// and every recordset entry. A guard with a syntax error would
/// otherwise silently render-fail in `render_truthy` and drop the
/// field at projection time (Roborev #2495).
pub fn dry_compile(profile: &ProfileSpec) -> CliResult<()> {
    let mut env = Environment::new();
    formula_helpers::register(&mut env);
    register_profile_helpers(&mut env, profile);

    let check = |label: &str, src: &str| -> CliResult<()> {
        env.template_from_str(src).map(|_| ()).map_err(|e| {
            CliError::Other(format!(
                "profile `{}`: template at `{}` failed to compile: {e}",
                profile.name, label
            ))
        })
    };

    for field in &profile.dataset.fields {
        check(&field.path, &field.template)?;
        if let Some(guard) = &field.emit_when {
            check(&format!("{} (emit_when)", field.path), guard)?;
        }
    }
    if let Some(dist) = &profile.distribution {
        for field in &dist.fields {
            check(&format!("distribution/{}", field.path), &field.template)?;
            if let Some(guard) = &field.emit_when {
                check(&format!("distribution/{} (emit_when)", field.path), guard)?;
            }
        }
    }
    if let Some(cat) = &profile.catalog {
        if let Some(tpl) = &cat.title_template {
            check("catalog/title_template", tpl)?;
        }
        for field in &cat.fields {
            check(&format!("catalog/{}", field.path), &field.template)?;
            if let Some(guard) = &field.emit_when {
                check(&format!("catalog/{} (emit_when)", field.path), guard)?;
            }
        }
    }
    for rs in &profile.recordsets {
        check(&format!("recordset/{}", rs.path), &rs.template)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::cmd::profile::profile_spec::load_from_str;

    const MINI_PROFILE: &str = r#"
name: mini
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "{{ pkg.title }}"
      required_level: required
    - path: dct:description
      template: "{{ pkg.notes }}"
      required_level: recommended
"#;

    #[test]
    fn project_emits_known_fields() {
        let profile = load_from_str(MINI_PROFILE, "test").unwrap();
        let ctx = json!({ "pkg": { "title": "Hello", "notes": "World" } });
        let (out, warns) = project(&profile, &ctx, ProjectionMode::Dataset).unwrap();
        assert_eq!(out.get("dct:title").and_then(Value::as_str), Some("Hello"));
        assert_eq!(
            out.get("dct:description").and_then(Value::as_str),
            Some("World")
        );
        assert!(warns.is_empty(), "unexpected warnings: {warns:?}");
    }

    #[test]
    fn required_level_emits_warning_when_empty() {
        let profile = load_from_str(MINI_PROFILE, "test").unwrap();
        let ctx = json!({ "pkg": {} });
        let (_out, warns) = project(&profile, &ctx, ProjectionMode::Dataset).unwrap();
        let titles: Vec<_> = warns.iter().map(|w| (&*w.field, w.severity)).collect();
        assert!(titles.contains(&("dct:title", Severity::Required)));
        assert!(titles.contains(&("dct:description", Severity::Recommended)));
    }

    #[test]
    fn coerce_json_string_parses_object() {
        let v = coerce_json_or_string("{\"@type\": \"dcat:Distribution\"}");
        assert_eq!(
            v.get("@type").and_then(Value::as_str),
            Some("dcat:Distribution")
        );
    }

    #[test]
    fn coerce_plain_string_returns_string() {
        let v = coerce_json_or_string("hello world");
        assert_eq!(v.as_str(), Some("hello world"));
    }

    #[test]
    fn emit_when_false_skips() {
        let yaml = r#"
name: emit-when-test
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:modified
      template: "2024-01-15"
      emit_when: "{{ pkg.modified is defined }}"
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({ "pkg": {} });
        let (out, _) = project(&profile, &ctx, ProjectionMode::Dataset).unwrap();
        assert!(out.get("dct:modified").is_none());
    }

    #[test]
    fn catalog_mode_wraps_dataset() {
        let yaml = r#"
name: catalog-test
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "Hello"
catalog:
  type: dcat:Catalog
  inherit_from_dataset: []
  conforms_to: "https://example.com/spec"
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({});
        let (out, _) = project(&profile, &ctx, ProjectionMode::Catalog).unwrap();
        assert_eq!(
            out.get("@type").and_then(Value::as_str),
            Some("dcat:Catalog")
        );
        assert_eq!(
            out.get("dct:title").and_then(Value::as_str),
            Some("Catalog of Hello")
        );
        assert!(out.get("dcat:dataset").and_then(Value::as_array).is_some());
    }

    #[test]
    fn catalog_field_template_can_reference_inner_dataset() {
        // Template-driven inheritance: a catalog.fields[] entry can
        // reference `inner["..."]` to pull values from the rendered
        // Dataset block, supporting rename / conditional copy /
        // transform that the static `inherit_from_dataset` list
        // cannot express.
        let yaml = r#"
name: catalog-inherit-template
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "Hello"
    - path: dct:publisher
      template: '{"@type":"foaf:Organization","foaf:name":"GSA"}'
catalog:
  type: dcat:Catalog
  inherit_from_dataset: []
  fields:
    # Rename: emit dct:publisher from the Dataset under the
    # dcat:contactPoint key on the Catalog.
    - path: "dcat:contactPoint"
      template: '{{ inner["dct:publisher"] | tojson }}'
    # Conditional: emit a derived catalog-only key only when the
    # Dataset has a title.
    - path: "qsv:datasetTitle"
      template: '{{ inner["dct:title"] }}'
      emit_when: '{{ inner["dct:title"] is defined }}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({});
        let (out, _) = project(&profile, &ctx, ProjectionMode::Catalog).unwrap();
        // Renamed key carries the Dataset's publisher object.
        let cp = out.get("dcat:contactPoint").expect("contactPoint");
        assert_eq!(
            cp.get("foaf:name").and_then(Value::as_str),
            Some("GSA"),
            "template-driven inheritance must surface nested fields"
        );
        // Conditional inheritance fires when inner has the key.
        assert_eq!(
            out.get("qsv:datasetTitle").and_then(Value::as_str),
            Some("Hello")
        );
    }

    #[test]
    fn catalog_field_template_sees_analysis_ctx_alongside_inner() {
        // ctx_with_inner is the union of the analysis ctx and
        // `inner`, so catalog templates can mix both (e.g. emit a
        // catalog-only ID from pkg + a rendered Dataset value).
        let yaml = r#"
name: catalog-mixed-ctx
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "{{ pkg.title | default('Untitled') }}"
catalog:
  type: dcat:Catalog
  inherit_from_dataset: []
  fields:
    - path: "qsv:catalogId"
      template: 'cat-{{ pkg.id }}-{{ inner["dct:title"] }}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({ "pkg": { "id": "42", "title": "NYC 311" } });
        let (out, _) = project(&profile, &ctx, ProjectionMode::Catalog).unwrap();
        assert_eq!(
            out.get("qsv:catalogId").and_then(Value::as_str),
            Some("cat-42-NYC 311"),
            "catalog template must see both pkg.* and inner[...]"
        );
    }

    #[test]
    fn catalog_field_emit_when_against_inner_skips_empty() {
        // An emit_when guard that consults `inner` must skip the
        // field when the guard renders falsy (the Dataset's key is
        // missing).
        let yaml = r#"
name: catalog-guard
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "T"
catalog:
  type: dcat:Catalog
  inherit_from_dataset: []
  fields:
    - path: "qsv:hasLicense"
      template: "yes"
      emit_when: '{{ inner["dct:license"] is defined }}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let (out, _) = project(&profile, &json!({}), ProjectionMode::Catalog).unwrap();
        assert!(
            out.get("qsv:hasLicense").is_none(),
            "emit_when against inner[missing] must skip"
        );
    }

    #[test]
    fn catalog_title_template_can_reference_analysis_ctx() {
        // Backward-compatible superset: the title template now sees
        // pkg/res/stats in addition to `inner`. Legacy templates that
        // only used `inner` keep working — covered by
        // dcat_us_v3_golden_parity_catalog. This test confirms the
        // additional context is reachable.
        let yaml = r#"
name: catalog-title-ctx
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "Inner Title"
catalog:
  type: dcat:Catalog
  title_template: '{{ pkg.publisher }} — Catalog of {{ inner["dct:title"] }}'
  inherit_from_dataset: []
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({ "pkg": { "publisher": "Acme" } });
        let (out, _) = project(&profile, &ctx, ProjectionMode::Catalog).unwrap();
        assert_eq!(
            out.get("dct:title").and_then(Value::as_str),
            Some("Acme — Catalog of Inner Title")
        );
    }

    #[test]
    fn lookup_helper_resolves_vocab_entry() {
        let yaml = r#"
name: lookup-test
vocabularies:
  license_iri:
    cc-by: "http://creativecommons.org/licenses/by/4.0/"
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:license
      template: '{{ lookup("license_iri", "cc-by") }}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let (out, _) = project(&profile, &json!({}), ProjectionMode::Dataset).unwrap();
        assert_eq!(
            out.get("dct:license").and_then(Value::as_str),
            Some("http://creativecommons.org/licenses/by/4.0/")
        );
    }

    #[test]
    fn lookup_missing_returns_undefined() {
        let yaml = r#"
name: lookup-missing
vocabularies:
  license_iri:
    cc-by: "http://creativecommons.org/licenses/by/4.0/"
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:license
      template: '{{ lookup("license_iri", "nonexistent") | default("fallback") }}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let (out, _) = project(&profile, &json!({}), ProjectionMode::Dataset).unwrap();
        assert_eq!(
            out.get("dct:license").and_then(Value::as_str),
            Some("fallback")
        );
    }

    #[test]
    fn for_each_column_emits_one_entry_per_stats_column() {
        let yaml = r#"
name: croissant-test
dataset:
  type: sc:Dataset
recordsets:
  - path: cr:RecordSet
    for_each_column: true
    template: '{"@type": "cr:Field", "name": "{{ column.field }}"}'
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let ctx = json!({
            "dpps": [
                { "field": "id" },
                { "field": "name" },
                { "field": "value" }
            ]
        });
        let (out, _) = project(&profile, &ctx, ProjectionMode::Dataset).unwrap();
        let arr = out.get("cr:RecordSet").and_then(Value::as_array).unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].get("name").and_then(Value::as_str), Some("id"));
        assert_eq!(arr[1].get("name").and_then(Value::as_str), Some("name"));
        assert_eq!(arr[2].get("name").and_then(Value::as_str), Some("value"));
    }

    #[test]
    fn dry_compile_rejects_malformed_distribution_emit_when() {
        // Roborev #2495 finding #1: dry_compile now extends the
        // emit_when syntax check to distribution + catalog field
        // guards. A typo in a distribution emit_when must fail at
        // load time, not silently render-fail to false at
        // projection time.
        let yaml = r#"
name: bad-dist-guard
dataset:
  type: dcat:Dataset
distribution:
  type: dcat:Distribution
  fields:
    - path: dcat:downloadURL
      template: "https://example.com/x.csv"
      emit_when: "{{ pkg.foo | }}"
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let err = dry_compile(&profile).expect_err("malformed emit_when must fail");
        let msg = err.to_string();
        assert!(
            msg.contains("distribution/dcat:downloadURL (emit_when)"),
            "error must locate distribution emit_when, got: {msg}",
        );
    }

    #[test]
    fn dry_compile_rejects_malformed_catalog_emit_when() {
        // Roborev #2495 finding #1: same coverage for catalog fields.
        let yaml = r#"
name: bad-catalog-guard
dataset:
  type: dcat:Dataset
catalog:
  type: dcat:Catalog
  fields:
    - path: dct:license
      template: "https://example.com/license"
      emit_when: "{% if foo %"
"#;
        let profile = load_from_str(yaml, "test").unwrap();
        let err = dry_compile(&profile).expect_err("malformed catalog emit_when must fail");
        let msg = err.to_string();
        assert!(
            msg.contains("catalog/dct:license (emit_when)"),
            "error must locate catalog emit_when, got: {msg}",
        );
    }
}
