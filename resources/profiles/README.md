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
