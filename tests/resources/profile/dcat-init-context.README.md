# `--initial-context` template

This file shows every input slot that the `qsv profile --initial-context`
flag reads. Use it as a starting point: `cp dcat-init-context.json
my-dataset.json`, edit, then run `qsv profile my.csv --initial-context
my-dataset.json`.

The top-level keys are:

| Key            | Purpose                                                       |
|----------------|---------------------------------------------------------------|
| `package`      | CKAN-shaped seed for the dataset block (Dataset-level DCAT).  |
| `resource`     | CKAN-shaped seed for the resource block (Distribution-level). |
| `dataset_info` | Free-form JSON-Pointer overrides into the final output.       |

## `package` fields the projection reads

Most fields mirror their CKAN names; v3 additions were added in Phase 5
and are all live in the current build.

| Field                          | DCAT-US v3 slot                              | Status     |
|--------------------------------|----------------------------------------------|------------|
| `title`                        | `dct:title` (mandatory)                      | ✅ today   |
| `notes`                        | `dct:description` (mandatory)                | ✅ today   |
| `name`                         | `dct:identifier` (mandatory)                 | ✅ today   |
| `publisher` (or `author`)      | `dct:publisher`                              | ✅ today   |
| `license_id` (or `license`)    | Distribution-level `dct:license`             | ✅ today   |
| `metadata_modified`            | `dct:modified` (discrete date only)          | ✅ today   |
| `metadata_created`             | `dct:issued`                                 | ✅ today   |
| `created`                      | `dct:created` (distinct from issued / modified) | ✅ today   |
| `version`                      | `dcat:version`                               | ✅ today   |
| `versionNotes`                 | `dcat:versionNotes`                          | ✅ today   |
| `tags[]`                       | `dcat:keyword`                               | ✅ today   |
| `groups[]`                     | `dcat:theme`                                 | ✅ today   |
| `language`                     | `dct:language` (normalized to ISO 639-1)     | ✅ today   |
| `contact_point` `{fn, hasEmail}` | `dcat:contactPoint` (**mandatory**)        | ✅ today   |
| `bureauCode`, `programCode`    | `dcat-us:bureauCode` / `dcat-us:programCode` | ✅ today   |
| `accrualPeriodicity`           | `dct:accrualPeriodicity`                     | ✅ today   |
| `accessRights`                 | `dct:accessRights`                           | ✅ today   |
| `rights`                       | `dct:rights`                                 | ✅ today   |
| `landing_page`                 | `dcat:landingPage`                           | ✅ today   |
| `describedBy`                  | `dcat:describedBy`                           | ✅ today   |
| `purpose`, `scopeNote`, `liabilityStatement` | `dcat-us:*`                    | ✅ today   |
| `inSeries`                     | `dcat:inSeries`                              | ✅ today   |

## `resource` fields the projection reads

| Field                 | DCAT-US v3 slot                             | Status     |
|-----------------------|---------------------------------------------|------------|
| `name`                | Distribution `dct:title`                    | ✅ today   |
| `description`         | Distribution `dct:description`              | ✅ today   |
| `format`              | Distribution `dct:format`                   | ✅ today   |
| `url`                 | Distribution `dcat:downloadURL` (IRI only)  | ✅ today   |
| `license_id` / `license` | Distribution `dct:license`               | ✅ today   |
| `last_modified`       | Distribution `dct:modified`                 | ✅ today   |
| `accessURL`           | Distribution `dcat:accessURL`               | ✅ today   |
| `rights`              | Distribution `dct:rights`                   | ✅ today   |
| `language`            | Distribution `dct:language` (Distribution-level; falls back to `package.language`) | ✅ today   |
| `conformsTo`          | Distribution `dct:conformsTo` (array of `dct:Standard` — string IRI or `{@id, ...}` object) | ✅ today   |
| `access_restriction`, `use_restriction`, `cui_restriction` | `dcat-us:*Restriction` | ✅ today   |

### Distribution fields qsv computes automatically

These do not need an `--initial-context` slot — qsv populates them from
the input bytes during the projection:

| Field                            | Source                                       |
|----------------------------------|----------------------------------------------|
| `dcat:byteSize`                  | `fs::metadata(input).len()` (emitted as string per GSA schema) |
| `dcat:checksum`                  | SHA-256 over the materialized file payload (`spdx:Checksum`)   |
| `dcat:compressFormat`            | derived from extension (`.gz` → `application/gzip`, etc.)      |
| `dcat:packageFormat`             | derived from extension (`.zip`, `.tar*` → `application/zip` / `x-tar`) |
| `dcat:mediaType`                 | always `text/csv`                            |
| `dcat:spatialResolutionInMeters` | from the spec's `spatial_resolution_in_meters` suggestion (via `dpp_suggestions`) |
| `csvw:tableSchema`               | per-column stats from `qsv stats` (cardinality, nullcount, datatype, min, max) |

## `force` semantics — three sources, one precedence

Any leaf value in `package`, `resource`, or `dataset_info` may be written
as a `{value, force: true}` wrapper:

```json
"title": {"value": "Authoritative", "force": true}
```

A forced leaf is applied **last**, beating every prior step in the
following pipeline (low → high precedence):

1. **Inferred** values from the DCAT projection (stats + dpp +
   formulas).
2. **Discovered** publisher DCAT (Link-header / sibling URL /
   JSON-LD `<script>` blocks).
3. **`dataset_info`** plain pointer entries (escape hatch into the
   final output).
4. **Forced leaves** (this row) — `{value, force: true}` wins
   unconditionally over 1, 2, and 3.

### How package / resource force flags route to DCAT

The CKAN field name is translated to its DCAT JSON-Pointer counterpart
via a built-in mapping table (`src/cmd/profile/ckan_to_dcat.rs`). The
most common entries:

| CKAN pointer           | DCAT pointer                                  |
|------------------------|-----------------------------------------------|
| `/package/title`       | `/dcat/dct:title`                             |
| `/package/notes`       | `/dcat/dct:description`                       |
| `/package/version`     | `/dcat/dcat:version`                          |
| `/package/bureauCode`  | `/dcat/dcat-us:bureauCode`                    |
| `/resource/url`        | `/dcat/dcat:distribution/0/dcat:downloadURL`  |
| `/resource/format`     | `/dcat/dcat:distribution/0/dct:format`        |
| `/resource/conformsTo` | `/dcat/dcat:distribution/0/dct:conformsTo`    |

CKAN slots without a DCAT counterpart (e.g. `package.scheming_version`)
silently drop their `force` flag — a documented no-op rather than a
translation error.

### Wrapper detection rules

Wrapper detection is opt-in by **shape**: only objects with exactly the
two keys `value` and `force` (and `force` is a bool) are treated as
wrappers. Anything else stays a plain value, so structured fields like
`contact_point: {fn, hasEmail}` do not collide with the wrapper sentinel.

`force: false` strips the wrapper but applies no force — equivalent to
just providing the inner value directly.

## `dataset_info` — JSON-Pointer escape hatch

The `dataset_info` map applies after every other source has been merged.
Each key is an RFC 6901 JSON Pointer relative to the **whole output**:

```json
"dataset_info": {
  "/dcat/dct:title":                 "Force override",
  "/dcat/dcat:distribution/0/dct:license": "https://opendatacommons.org/licenses/by/1-0/",
  "/dcat/dct:modified": {"value": "2024-12-31T23:59:59Z", "force": true}
}
```

Use this when:

* You need to override a deep path that doesn't have a dedicated
  `package` / `resource` slot.
* You're scripting against the output and need a last-write-wins hatch.
* You want to set a value the projection wouldn't otherwise compute
  (custom JSON-LD extension keys, vendor namespaces, etc.).

`dataset_info` entries can also use the `{value, force: true}` wrapper
— in which case they merge into the unified force-overrides list and
beat every other source (including non-forced `dataset_info` entries).

Missing parent objects are auto-created. Non-object intermediate scalars
get replaced. Failures (non-pointer keys, malformed escape sequences)
are silently skipped — this is best-effort, not enforcement.

## `--catalog` mode

With `--catalog`, the emitted `dcat` block is wrapped inside a
`dcat:Catalog` envelope (`Catalog{dataset:[...]}`) suitable for
federation harvesters (data.gov, CKAN ingest). The Catalog inherits
the enclosed Dataset's title (prefixed with `Catalog of `) and
publisher. All `dataset_info` and force overrides apply to the
Dataset BEFORE the Catalog wrap — pointer paths like `/dcat/dct:title`
target the inner Dataset, not the Catalog envelope.

`--validate-dcat --catalog` runs the Catalog overlay schema
(`resources/dcat-us-v3/qsv-overlay-catalog.json`) which enforces
Catalog-level required keys on the envelope.

## DCAT-US v3 spec references

* Spec landing page: <https://resources.data.gov/resources/dcat-us3/>
* v1.1 → v3 migration guide:
  <https://resources.data.gov/resources/dcat-us-3-migration/>
* Authoritative JSON Schema 2020-12 definitions + examples
  (vendored under `resources/dcat-us-v3/` for `--validate-dcat`):
  <https://github.com/GSA/dcat-us/tree/main/jsonschema>
