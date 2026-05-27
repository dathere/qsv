# Vendored DCAT-AP v3 SHACL shapes

`dcat-ap-SHACL.ttl` is the canonical SHACL shapes file shipped by
[SEMIC.eu](https://semiceu.github.io/DCAT-AP/releases/3.0.0/) with
the DCAT-AP v3.0.0 release.

## Source

* Upstream: <https://github.com/SEMICeu/DCAT-AP>
* Release: `releases/3.0.0/shacl/dcat-ap-SHACL.ttl`
* Raw URL: <https://raw.githubusercontent.com/SEMICeu/DCAT-AP/master/releases/3.0.0/shacl/dcat-ap-SHACL.ttl>

## Why embedded

qsv `include_str!`s this file into the `qsv` binary so the
`dcat-ap-v3` profile's `validation.external` block can spawn
`pyshacl` against the rendered JSON-LD without requiring users to
download or locate the shapes file separately. See
`src/cmd/profile/external_validate.rs::EMBEDDED_RESOURCES`.

## Re-vendoring procedure

```bash
curl -sSL -o resources/dcat-ap-v3/shacl/dcat-ap-SHACL.ttl \
  https://raw.githubusercontent.com/SEMICeu/DCAT-AP/master/releases/3.0.0/shacl/dcat-ap-SHACL.ttl
```

If SEMICeu publishes a new patch release (e.g. 3.0.1, 3.0.2), bump
the version subdirectory in the URL and re-run. The `embedded` key
in `resources/profiles/dcat-ap-v3.yaml` doesn't need to change —
it's a logical name resolved against `EMBEDDED_RESOURCES`, not a
path.
