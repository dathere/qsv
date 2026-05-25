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
| `access_restriction`, `use_restriction`, `cui_restriction` | `dcat-us:*Restriction` | ✅ today   |

## Per-property `force` semantics (Phase 4b)

Any leaf value above may also be written as:

```json
"title": {"value": "Authoritative", "force": true}
```

`force: true` marks the value as **overriding** any value discovered from
the URL input's existing DCAT markup (Phase 3b's `--no-dcat-discovery`
opposite). Plain values (or `force: false`) only fill gaps where no
discovered value was found.

Wrapper detection is opt-in by shape: only objects with exactly the two
keys `value` and `force` are treated as wrappers; anything else stays a
plain value, so structured fields like `contact_point: {fn, hasEmail}`
do not collide with the wrapper sentinel.

## `dataset_info` — JSON-Pointer escape hatch

The `dataset_info` map applies after every other source has been merged.
Each key is an RFC 6901 JSON Pointer relative to the **whole output**:

```json
"dataset_info": {
  "/dcat/dct:title":                 "Force override",
  "/dcat/dcat:distribution/0/dct:license": "https://opendatacommons.org/licenses/by/1-0/"
}
```

Use this when:

* You need to override a deep path that doesn't have a dedicated
  `package` / `resource` slot.
* You're scripting against the output and need a last-write-wins hatch.
* You want to set a value the projection wouldn't otherwise compute
  (custom JSON-LD extension keys, vendor namespaces, etc.).

Missing parent objects are auto-created. Non-object intermediate scalars
get replaced. Failures (non-pointer keys, malformed escape sequences)
are silently skipped — this is best-effort, not enforcement.

## DCAT-US v3 spec references

* Spec landing page: <https://resources.data.gov/resources/dcat-us3/>
* v1.1 → v3 migration guide:
  <https://resources.data.gov/resources/dcat-us-3-migration/>
* Authoritative JSON Schema 2020-12 definitions + examples (Phase 6
  vendors these for `--validate-dcat`):
  <https://github.com/GSA/dcat-us/tree/main/jsonschema>
