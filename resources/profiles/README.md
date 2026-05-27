# qsv profile — bundled YAML profiles

Each `<name>.yaml` here is a **projection profile** read by
`qsv profile --profile <name>`. The bundled set is resolved first; if no
embedded name matches, `--profile` falls back to treating its argument as
a file path on disk.

Bundled today:

| Name | Target standard | Validation |
|---|---|---|
| `dcat-us-v3` | DCAT-US v3 (resources.data.gov) | JSON Schema (vendored GSA bundle under `resources/dcat-us-v3/`) |
| `dcat-ap-v3` | DCAT-AP v3 (data.europa.eu) | disabled (upstream ships SHACL, not JSON Schema) |
| `croissant`  | Croissant 1.0 (mlcommons.org)  | disabled (no published JSON Schema) |

## Authoring a custom profile

Copy any bundled profile, edit the `vocabularies` / `field_mappings` /
`dataset` / `distribution` / `catalog` blocks, then point
`--profile path/to/your.yaml` at it. The schema is documented inline in
`src/cmd/profile/profile_spec.rs`.

## Versioning contract

The shipped YAMLs are a stable downstream contract: DataPusher+ and other
metadata consumers depend on the wire shape these profiles produce. When
editing a bundled profile:

1. Bump the `version:` field semver-style.
2. The Dataset/Distribution wire shape change must be additive (new
   keys, never renames or removals) unless a major version bump.
3. Re-run the golden parity tests (`tests/resources/profile/golden/`)
   and update the goldens in the same commit if the shape change is
   intentional.

## Field-decl directives

Every entry under `dataset.fields[]`, `distribution.fields[]`, and
`catalog.fields[]` carries:

| Key | Meaning |
|---|---|
| `path` | JSON-LD key emitted into the projection (e.g. `dct:title`). |
| `template` | Minijinja expression rendered against the analysis context. Strings starting with `{` are JSON-parsed; otherwise treated as literal strings. |
| `required_level` | `required` / `recommended` / `optional` — drives `ProjectionWarning` severity when empty. |
| `on_dataset` | Emit on the Dataset block? Default `true`. |
| `on_distribution` | Emit on each Distribution? Default `false`. |
| `emit_when` | Optional guard template; field skipped when this renders falsy/empty. |
| `default` | Literal fallback when the main template renders empty (suppresses the warning). |
| `for_each_column` | Croissant-style expansion: emit one entry per stats column. |

## Catalog envelope

When `--catalog` is set, the Dataset block is wrapped inside the
profile's `catalog:` block. Two paths are available for surfacing
Dataset values on the outer Catalog envelope:

* **`inherit_from_dataset: [key, key, ...]`** — verbatim copy. Each
  listed Dataset key is copied (same name, same value) onto the
  Catalog. Cheapest path; suitable when you just want to expose
  `dct:publisher` or similar at the catalog level.
* **`catalog.fields[]`** — template-driven. Each entry is a regular
  `FieldDecl` (same directives as dataset/distribution fields), but
  its template gets access to an extra `inner` binding holding the
  rendered Dataset block. Use this for rename, conditional copy, or
  value transformation:

```yaml
catalog:
  inherit_from_dataset: ["dct:publisher"]   # verbatim
  fields:
    - path: "dcat:contactPoint"
      template: '{{ inner["dcat:contactPoint"] | tojson }}'
      emit_when: '{{ inner["dcat:contactPoint"] is defined }}'
    - path: "qsv:catalogId"
      template: 'cat-{{ pkg.id }}'
```

The catalog `title_template` also receives both bindings, so titles
can mix analysis vars and Dataset values:
`'{{ pkg.publisher }} — Catalog of {{ inner["dct:title"] }}'`.

## Discovery merge

When the caller provides publisher-side DCAT (via `--initial-context` or
auto-discovered from CKAN), `discovery_merge` controls how that metadata
folds into the qsv-inferred projection.

| Key | Meaning |
|---|---|
| `enabled` | Master switch. `false` skips merging entirely. |
| `never_overwrite` | Top-level keys protected from any overlay (typical: `@context`, `@type`, `dcat:distribution`). |
| `default_strategy` | `fill-if-absent` (default — inferred wins on conflict), `overlay-array` (append publisher elements to inferred arrays), or `never`. |
| `distribution_merge` | Optional per-element merge for the distribution array. See below. |

### Per-distribution merging

By default the `dcat:distribution` array is `never_overwrite`'d — qsv's
inferred distributions are canonical for the local data. Setting
`distribution_merge.enabled: true` bypasses that protection for the
array key and walks each publisher Distribution, matching it to an
inferred one by `identity_keys` (first non-empty match wins). Matched
fields flow into the inferred record via `field_strategy`.

| Key | Meaning |
|---|---|
| `enabled` | Master switch for per-element merging. `false` (default) preserves the legacy "publisher distributions dropped" behavior. |
| `array_key` | The top-level key holding the distribution array. Default `dcat:distribution`. |
| `identity_keys` | Ordered list of fields used to match a publisher Distribution against an inferred one. Empty list disables matching. |
| `field_strategy` | How to merge fields within a matched pair: `fill-if-absent` (default) / `overlay-array` / `never`. |
| `append_unmatched` | When `true`, publisher distributions that match no inferred entry are appended. Default `false` (silently dropped). |

DCAT-US v3 and DCAT-AP v3 enable this with `dcat:downloadURL` →
`dcat:accessURL` → `@id` as the identity-key priority. Croissant
disables `discovery_merge` entirely.
