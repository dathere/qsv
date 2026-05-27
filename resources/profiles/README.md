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
| `geoconnex`  | Geoconnex hydrologic linked-data (docs.geoconnex.us) | disabled (upstream ships SHACL via `pyshacl`); Phase 1 dataset-level only — see `resources/geoconnex/shacl/README.md` for what's covered. **Gated behind the `geoconnex` cargo feature** — bundled in `qsv` (via `distrib_features`) and as an opt-in for `qsvdp` (`-F datapusher_plus,geoconnex`); not available in `qsvlite` / `qsvmcp` |

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

## Validation

Two orthogonal validators are available on every profile:

* **`validation.enabled` (JSON Schema)** — when `true`, the engine
  validates the rendered block against the vendored GSA DCAT-US v3
  bundle under `resources/dcat-us-v3/`. Today only the DCAT-US v3
  bundle ships with qsv; any other `schema_dir` emits a heads-up
  warning. Triggered by `--validate-dcat`.
* **`validation.external`** — an out-of-process validator (e.g.
  `mlcroissant`, `pyshacl`) spawned with the rendered JSON-LD on
  disk. Runs orthogonal to JSON Schema: a profile may use either,
  both, or neither. Also gated by `--validate-dcat`.

### External validator config

| Key | Meaning |
|---|---|
| `command` | Command to spawn (resolved via `PATH`). When not found, validation emits one `Info`-severity warning and continues — projection still ships. |
| `args` | Arguments. The literal token `{file}` is replaced with the path to a tempfile holding the rendered JSON-LD. Without `{file}`, the path is appended as the last positional arg. |
| `default_severity` | Severity for each surfaced finding. One of `required` / `recommended` (default) / `optional` / `info`. |
| `label` | Friendly name used in warning messages instead of the raw `command` value. Useful when the command is e.g. `python3 -m mlcroissant ...`. |
| `install_hint` | Optional free-form text appended to the missing-binary warning (typically a one-line install command + project URL). |
| `resources` | Optional list of additional tempfile inputs to materialize before spawn. Each entry has `name` (token used in `args` as `{<name>}`), `embedded` (logical name resolved against the qsv-bundled `EMBEDDED_RESOURCES` table), and an optional `suffix` (default `.tmp`). The implicit `{file}` token (rendered JSON-LD) is always available — using `name: "file"` here is rejected at spawn time. |

A non-zero exit code surfaces one warning per non-empty stderr line
(falling back to stdout when stderr is empty). Exit 0 = empty Vec.
Findings respect `--strict-dcat`: when set, a non-`Info` external
finding fails the command the same way a JSON Schema violation does.

Croissant uses this to wire up the canonical Python validator:

```yaml
validation:
  enabled: false
  external:
    command: "mlcroissant"
    args: ["validate", "--jsonld", "{file}"]
    label: "mlcroissant"
    default_severity: "recommended"
    install_hint: "pip install mlcroissant (https://github.com/mlcommons/croissant/tree/main/python/mlcroissant)"
```

DCAT-AP v3 uses this for `pyshacl`, which needs both the rendered
JSON-LD AND the SHACL shapes file. The shapes file is vendored
under `resources/dcat-ap-v3/shacl/` and shipped embedded in the
qsv binary; the profile references it by logical name:

```yaml
validation:
  enabled: false
  external:
    command: "pyshacl"
    args: ["-s", "{shapes}", "-sf", "turtle", "-df", "json-ld", "-f", "human", "{file}"]
    label: "pyshacl"
    install_hint: "pip install pyshacl (https://github.com/RDFLib/pySHACL)"
    resources:
      - name: "shapes"
        embedded: "dcat-ap-v3-shacl-shapes"
        suffix: ".ttl"
```

Adding a new embedded resource means vendoring the file under
`resources/<slug>/` (with a sibling `README.md` documenting the
source + re-vendor procedure) and registering one
`(name, include_str!(path))` tuple in
`src/cmd/profile/external_validate.rs::EMBEDDED_RESOURCES`.

The `geoconnex` profile uses the same `pyshacl` machinery against
the Internet of Water's Geoconnex SHACL shapes (vendored under
`resources/geoconnex/shacl/`, embedded as `geoconnex-shacl-shapes`):

```yaml
validation:
  enabled: false
  external:
    command: "pyshacl"
    args: ["-s", "{shapes}", "-sf", "turtle", "-df", "json-ld", "-f", "human", "{file}"]
    label: "pyshacl"
    install_hint: "pip install pyshacl (https://github.com/RDFLib/pySHACL)"
    resources:
      - name: "shapes"
        embedded: "geoconnex-shacl-shapes"
        suffix: ".ttl"
```
