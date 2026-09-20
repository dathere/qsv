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

Run every step in the SAME shell: step 1 sets `$COMMIT` and step 3
needs it. Each step anchors its own `cd` to the repo root so the steps
do not depend on where the previous one left you.

1. Pick the new upstream commit and capture its full SHA:

   ```bash
   COMMIT=$(curl -s https://api.github.com/repos/GSA/dcat-us/commits/main | jq -r .sha)
   echo "$COMMIT"
   ```

2. Re-fetch every schema at the new commit. Names must match the
   filenames already present under `definitions/`:

   ```bash
   cd "$(git rev-parse --show-toplevel)/resources/dcat-us-v3/definitions"
   for f in *.json; do
     curl -sSfL -o "$f" \
       "https://raw.githubusercontent.com/GSA/dcat-us/${COMMIT}/jsonschema/definitions/$f"
   done
   ```

   If upstream has added new definition files, add them to the
   loop above. If a file was renamed or removed, mirror that here
   too — the manifest must list every file that exists on disk.

3. Re-fetch the conformance fixtures at the SAME commit. These
   drive the `gsa_conformance` tests, and a fixture left at an
   older commit than the schema it is validated against quietly
   stops testing what it claims to:

   ⚠️ The pin comes from `$COMMIT` set in step 1, **not** from
   `MANIFEST.json` — the manifest still holds the OLD commit until
   step 4, so reading it here would fetch new schemas against old
   fixtures and then label both with the new SHA. That is exactly
   the drift these fixtures exist to catch.

   ```bash
   cd "$(git rev-parse --show-toplevel)/resources/dcat-us-v3"
   PIN="$COMMIT" python3 - <<'FIXTURES'
   import json, os, urllib.request, pathlib
   pin = os.environ.get('PIN', '').strip()
   if not pin:
       raise SystemExit('PIN is empty - run step 1 first, in the same shell.')
   for cls in ['Dataset', 'Catalog', 'Distribution']:
       for kind in ['good', 'bad']:
           api = (f"https://api.github.com/repos/GSA/dcat-us/contents/"
                  f"jsonschema/examples/{cls}/{kind}?ref={pin}")
           entries = json.load(urllib.request.urlopen(api))
           d = pathlib.Path('examples') / cls / kind
           d.mkdir(parents=True, exist_ok=True)
           seen = set()
           for e in entries:
               if not e['name'].endswith('.json'):
                   continue
               raw = (f"https://raw.githubusercontent.com/GSA/dcat-us/{pin}/"
                      f"jsonschema/examples/{cls}/{kind}/{e['name']}")
               data = urllib.request.urlopen(raw).read()
               json.loads(data)          # fail loudly on a truncated/HTML body
               (d / e['name']).write_bytes(data)
               seen.add(e['name'])
           for stale in set(p.name for p in d.glob('*.json')) - seen:
               (d / stale).unlink()      # upstream removed it
               print('removed stale fixture:', cls, kind, stale)
   FIXTURES
   ```

   Only these three classes are vendored — see the note on scope in
   `dcat_validate::gsa_conformance`.

4. Regenerate `MANIFEST.json`:

   ```bash
   cd "$(git rev-parse --show-toplevel)/resources/dcat-us-v3"
   python3 - <<'MANIFEST'
   import hashlib, json, pathlib
   m = json.load(open('MANIFEST.json'))
   def h(p):
       return {'path': p, 'sha256': hashlib.sha256(open(p, 'rb').read()).hexdigest()}
   m['files'] = [h(f'definitions/{f.name}')
                 for f in sorted(pathlib.Path('definitions').glob('*.json'))]
   m['examples'] = [h(p.as_posix())
                    for p in sorted(pathlib.Path('examples').glob('**/*.json'))]
   json.dump(m, open('MANIFEST.json', 'w'), indent=2)
   open('MANIFEST.json', 'a').write('\n')
   print('files:', len(m['files']), 'examples:', len(m['examples']))
   MANIFEST
   ```

   Then update `commit`, `commit_url` and `fetched_at` by hand. Leave
   `upstream`, `schema_draft`, and `entry_points` alone unless upstream
   restructures. Both `files` and `examples` are hashed by the pin test.

5. Update `THIRD_PARTY_NOTICES.md`. The pin appears there in **two**
   spellings — a 7-char form in the summary table and the full SHA in
   the detail section — and `third_party_notices_pin_matches_the_manifest`
   asserts both against `MANIFEST.json`.

6. Run the test suite:

   ```bash
   cargo test --test tests -F profile,feature_capable -- test_profile::dcat_us_v3_bundle_pin
   cargo test --bin qsv -F profile cmd::profile::
   ```

   Expect `gsa_conformance` to be the first thing that breaks if upstream
   tightened or relaxed a constraint: a `good` fixture failing means qsv is
   now stricter than the schema authors intend, a `bad` fixture passing means
   it is weaker.

7. If new fields landed upstream that qsv should emit, file a
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
