# Vendored DCAT-US v3 JSON Schema bundle

These JSON Schema 2020-12 definitions are vendored verbatim from
[GSA/dcat-us](https://github.com/GSA/dcat-us). They drive the
optional schema validation pass exposed by `qsv profile`'s
`--validate` and `--strict` flags.

## Pin

`MANIFEST.json` is the source of truth for what is vendored:

| Field          | Meaning                                                          |
|----------------|------------------------------------------------------------------|
| `upstream`     | GSA repository URL                                               |
| `commit`       | Git commit SHA the bundle was fetched at                         |
| `commit_url`   | Browse-link to the exact directory on GitHub                     |
| `fetched_at`   | Date the snapshot was taken                                      |
| `schema_draft` | JSON Schema draft URI                                            |
| `entry_points` | Recommended root schemas (Dataset for dataset-only, Catalog for federation) |
| `files`        | Each vendored file with its SHA-256 content hash                 |

A unit test (`tests/test_profile.rs::dcat_us_v3_bundle_pin_manifest_matches_files`) re-hashes every file
on every CI run and fails if any SHA in `MANIFEST.json` does not
match the file on disk. Silent edits are blocked.

## Refresh procedure

1. Pick the new upstream commit and capture its full SHA:

   ```bash
   COMMIT=$(curl -s https://api.github.com/repos/GSA/dcat-us/commits/main | jq -r .sha)
   echo "$COMMIT"
   ```

2. Re-fetch every schema at the new commit. Names must match the
   filenames already present under `definitions/`:

   ```bash
   cd resources/dcat-us-v3/definitions
   for f in *.json; do
     curl -sSfL -o "$f" \
       "https://raw.githubusercontent.com/GSA/dcat-us/${COMMIT}/jsonschema/definitions/$f"
   done
   ```

   If upstream has added new definition files, add them to the
   loop above. If a file was renamed or removed, mirror that here
   too — the manifest must list every file that exists on disk.

3. Regenerate `MANIFEST.json`:

   ```bash
   cd resources/dcat-us-v3
   python3 -c "
   import hashlib, json, os
   defs = sorted(os.listdir('definitions'))
   files = [{'path': f'definitions/{f}',
             'sha256': hashlib.sha256(open(f'definitions/{f}','rb').read()).hexdigest()}
            for f in defs]
   print(json.dumps({'files': files}, indent=2))
   "
   ```

   Update `commit`, `commit_url`, `fetched_at`, and `files` in
   `MANIFEST.json`. Leave `upstream`, `schema_draft`, and
   `entry_points` alone unless upstream restructures.

4. Run the test suite:

   ```bash
   cargo test --test tests -F profile,feature_capable -- test_profile::dcat_us_v3_bundle_pin
   cargo test --bin qsv -F profile cmd::profile::
   ```

5. If new fields landed upstream that qsv should emit, file a
   follow-up — extending coverage is a separate change from
   refreshing the pin.

## Unprefixed keys

The GSA bundle declares property names without CURIE prefixes
(`title`, not `dct:title`), and so does qsv. DCAT-US v3 is plain
JSON validated by JSON Schema — GSA removed the JSON-LD context
and the SHACL shapes upstream in April 2026 ("Move aside outdated
files and emphasize the JSON Schema"), and the canonical examples
under `jsonschema/examples/` carry no `@context`.

Earlier versions of qsv emitted the JSON-LD-compact, CURIE-prefixed
form and stripped the prefixes in memory before validating. That
bridge is gone: the projection is now validated verbatim, exactly
as written to disk, against the same schemas data.gov's own
validator at <https://harvest.data.gov/validate/> runs.

The only prefixed keys qsv still emits are deliberate extensions:
`dcat-us:bureauCode`, `dcat-us:programCode`, `dcat-us:accessLevel`,
`qsv:sourcePath` and `csvw:tableSchema`. The first three are **not**
DCAT-US v3 properties — they appear in none of the 26 definitions —
but agencies still need them for OMB M-13-13 / Project Open Data,
and the v1.1 → v3 migration guide explicitly says to keep
`accessLevel` during the transition ("the v3.0 schema will not
reject it"). They are listed in `dcat_validate::EXTENSION_KEYS` so
the unknown-key lint tolerates them.

## Validation is stricter than the bundle alone

No schema in the bundle sets `additionalProperties`, so an unknown
key — a typo, or a property upstream has since removed — validates
silently. Two qsv-side additions close that:

* an **unknown-key lint** reports emitted keys the target definition does not declare, scoped to
  the node types qsv constructs (root, each Dataset in a Catalog, each Distribution);
* **format assertion** is switched on. JSON Schema 2020-12 treats `format` as an annotation by
  default, so without this a malformed `date-time` passes here while data.gov rejects it.

Warning severity is derived from the bundle's own
`requirementLevel` annotations, resolved against the class that owns
the path (`modified` is Recommended on Dataset but Mandatory on
CatalogRecord — a flat name index would conflate them).

## Why the bundle has no top-level entry-point

The upstream README directs consumers to validate against
`definitions/Dataset.json` for individual datasets or
`definitions/Catalog.json` for federated catalogs. We do the
same — `dcat_validate.rs::validate` picks the right entry-point by
inspecting `@type` on the emitted block.

Two overlay schemas in this directory layer qsv-specific
extensions on top of the vendored GSA bundle:

* `qsv-overlay-dataset.json` — `allOf`-wraps `definitions/Dataset.json`
  and adds property definitions for `dcat-us:bureauCode` and
  `dcat-us:programCode` (the M-13-13 OMB codes that the GSA v3
  bundle itself does not define). Used as the validator entry-point
  when the emitted `@type` is `Dataset`.
* `qsv-overlay-catalog.json` — `allOf`-wraps `definitions/Catalog.json`
  with no current additions. Used as the validator entry-point when
  `--catalog` is set. Reserved for future Catalog-level extensions.

Both overlays inherit the GSA bundle's permissive
`additionalProperties` behavior, so any other `dcat-us:*` key is
still accepted by the schema — but it will be reported by the
unknown-key lint unless it is on the extension allowlist.

## Licensing

The vendored schemas are © General Services Administration and
distributed under the upstream repository's license (see the
`LICENSE` file in [github.com/GSA/dcat-us](https://github.com/GSA/dcat-us)).
qsv adds no copyright claim over the vendored content.
