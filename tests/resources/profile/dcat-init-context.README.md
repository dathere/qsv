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
| `title`                        | `title` (mandatory)                      | ✅ today   |
| `notes`                        | `description` (mandatory)                | ✅ today   |
| `name`                         | `identifier` (mandatory)                 | ✅ today   |
| `publisher` (or `author`)      | `publisher`                              | ✅ today   |
| `license_id` (or `license`)    | Distribution-level `license`             | ✅ today   |
| `metadata_modified`            | `modified` (discrete date only)          | ✅ today   |
| `metadata_created`             | `issued`                                 | ✅ today   |
| `created`                      | `created` (distinct from issued / modified) | ✅ today   |
| `version`                      | `version`                               | ✅ today   |
| `versionNotes`                 | `versionNotes`                          | ✅ today   |
| `tags[]`                       | `keyword`                               | ✅ today   |
| `groups[]`                     | `theme`                                 | ✅ today   |
| `language`                     | `language` (normalized to ISO 639-1)     | ✅ today   |
| `contact_point` `{fn, hasEmail}` | `contactPoint` (**mandatory**)        | ✅ today   |
| `bureauCode`, `programCode`    | `dcat-us:bureauCode` / `dcat-us:programCode` (qsv extension — see note) | ✅ today   |
| `accrualPeriodicity`           | `accrualPeriodicity`                     | ✅ today   |
| `accessRights`                 | `accessRights`                           | ✅ today   |
| `rights`                       | `rights`                                 | ✅ today   |
| `landing_page`                 | `landingPage`                           | ✅ today   |
| `describedBy`                  | `describedBy`                           | ✅ today   |
| `purpose`, `scopeNote`, `liabilityStatement` | `purpose` / `scopeNote` / `liabilityStatement` | ✅ today   |

## `resource` fields the projection reads

| Field                 | DCAT-US v3 slot                             | Status     |
|-----------------------|---------------------------------------------|------------|
| `name`                | Distribution `title`                    | ✅ today   |
| `description`         | Distribution `description`              | ✅ today   |
| `format`              | Distribution `format`                   | ✅ today   |
| `url`                 | Distribution `downloadURL` (IRI only)  | ✅ today   |
| `license_id` / `license` | Distribution `license`               | ✅ today   |
| `last_modified`       | Distribution `modified`                 | ✅ today   |
| `accessURL`           | Distribution `accessURL`               | ✅ today   |
| `rights`              | Distribution `rights`                   | ✅ today   |
| `language`            | Distribution `language` (Distribution-level; falls back to `package.language`) | ✅ today   |
| `conformsTo`          | Distribution `conformsTo` (array of `Standard` — string IRI or `{@id, ...}` object) | ✅ today   |
| `access_restriction`, `use_restriction`, `cui_restriction` | `accessRestriction` / `useRestriction` / `cuiRestriction` (the first two are arrays) | ✅ today   |

### Distribution fields qsv computes automatically

These do not need an `--initial-context` slot — qsv populates them from
the input bytes during the projection:

| Field                            | Source                                       |
|----------------------------------|----------------------------------------------|
| `byteSize`                  | `fs::metadata(input).len()` (emitted as string per GSA schema) |
| `checksum`                  | SHA-256 over the materialized file payload (`Checksum`)   |
| `compressFormat`            | derived from extension (`.gz` → `application/gzip`, etc.)      |
| `packageFormat`             | derived from extension (`.zip`, `.tar*` → `application/zip` / `x-tar`) |
| `mediaType`                 | always `text/csv`                            |
| `spatialResolutionInMeters` | from the spec's `spatial_resolution_in_meters` suggestion (via `dpp_suggestions`) |
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
via the active profile's `field_mappings:` table (see
`resources/profiles/dcat-us-v3.yaml`; the lookup itself is
`ProfileSpec::translate_ckan_ptr`). The most common entries:

| CKAN pointer           | Projection pointer                                 |
|------------------------|----------------------------------------------------|
| `/package/title`       | `/projection/title`                            |
| `/package/notes`       | `/projection/description`                      |
| `/package/version`     | `/projection/version`                         |
| `/package/bureauCode`  | `/projection/dcat-us:bureauCode`                   |
| `/resource/url`        | `/projection/distribution/0/downloadURL` |
| `/resource/format`     | `/projection/distribution/0/format`       |
| `/resource/conformsTo` | `/projection/distribution/0/conformsTo`   |

CKAN slots without a projection counterpart (e.g. `package.scheming_version`)
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
  "/projection/title":                 "Force override",
  "/projection/distribution/0/license": "https://opendatacommons.org/licenses/by/1-0/",
  "/projection/modified": {"value": "2024-12-31T23:59:59Z", "force": true}
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

With `--catalog`, the emitted projection block is wrapped inside a
`Catalog` envelope (`Catalog{dataset:[...]}`) suitable for
federation harvesters (data.gov, CKAN ingest). The Catalog inherits
the enclosed Dataset's title (prefixed with `Catalog of `) and
publisher.

In `--catalog` mode the inner Dataset is nested at
`/projection/dataset/0/...` (or `/projection/schema:dataset/0/...`
for schema.org-rooted profiles like Geoconnex). `dataset_info` and
force-value overrides apply to the *full output* after the Catalog
wrap, so a bare `/projection/title` writes to the Catalog
envelope itself; to override an inner-Dataset slot use the nested
path, e.g. `/projection/dataset/0/title`. Discovery-merge
force-protection still operates on the pre-wrap Dataset, so a
forced `/projection/title` does correctly shield the Dataset's
title from being overwritten by discovered publisher metadata even
under `--catalog`.

`--validate --catalog` runs the Catalog overlay schema
(`resources/dcat-us-v3/qsv-overlay-catalog.json`) which enforces
Catalog-level required keys on the envelope.

## DCAT-US v3 spec references

* Spec landing page: <https://resources.data.gov/resources/dcat-us3/>
* v1.1 → v3 migration guide:
  <https://resources.data.gov/resources/dcat-us-3-migration/>
* Authoritative JSON Schema 2020-12 definitions + examples
  (vendored under `resources/dcat-us-v3/` for `--validate`):
  <https://github.com/GSA/dcat-us/tree/main/jsonschema>

## A note on serialization and the `dcat-us:` keys

DCAT-US v3 is plain JSON with **unprefixed** keys and no `@context` —
GSA removed the JSON-LD context upstream in April 2026. The slots in
the tables above are therefore bare names (`title`, not `dct:title`),
and JSON-Pointer overrides in `dataset_info` must use those bare names
too: `/projection/title`, not `/projection/dct:title`.

Three keys are still emitted prefixed, and they are the exception
rather than the rule:

* `dcat-us:bureauCode`
* `dcat-us:programCode`
* `dcat-us:accessLevel`

**None of these is a DCAT-US v3 property** — they appear in none of
the 26 vendored definitions. qsv keeps emitting them because agencies
still need them for OMB M-13-13 / Project Open Data, and because the
v1.1 → v3 migration guide explicitly says to retain `accessLevel`
during the transition: "the v3.0 schema will not reject it". Treat
them as qsv extensions, not as conformance surface.

`inSeries` used to be accepted here. GSA removed it from `Dataset.json`,
so it is no longer emitted; supplying it now has no effect.
