# Vendored Geoconnex SHACL shapes

`geoconnex.ttl` is the canonical SHACL shapes file shipped by the
[Internet of Water](https://internetofwater.org/) project's
[nabu](https://github.com/internetofwater/nabu) crawler/validator.
It defines seven `NodeShape`s covering organizations, datasets,
variables, distributions, and hydrologic places — see
<https://docs.geoconnex.us/reference/data-formats/shacl_shape> for
the human-readable overview (note: the docs page can lag the .ttl
by a release; trust the .ttl when in doubt).

## Source

* Upstream: <https://github.com/internetofwater/nabu>
* Path: `shacl_validator/shapes/geoconnex.ttl`
* Raw URL: <https://raw.githubusercontent.com/internetofwater/nabu/main/shacl_validator/shapes/geoconnex.ttl>
* License: Apache-2.0
* Pinned commit: `e5d6ad390a2cf9b0272676757713b1bf1757f75b` (2026-05-26)

## Why embedded

qsv `include_str!`s this file into the `qsv` binary so the
`geoconnex` profile's `validation.external` block can spawn
`pyshacl` against the rendered JSON-LD without requiring users to
download or locate the shapes file separately. See
`src/cmd/profile/external_validate.rs::EMBEDDED_RESOURCES`.

## Phase 1 coverage

The bundled `geoconnex` profile (`resources/profiles/geoconnex.yaml`)
projects dataset-level metadata only: `DatasetShape`, `ProviderShape`,
`PublisherShape`, and `DistributionShape`. The row-per-feature
`LocationOrientedShape` (with mandatory `gsp:hasGeometry` /
`gsp:asWKT` and per-row stable `@id`) is deferred to a follow-up that
introduces a `for_each_row` projection mode + WKT synthesis from
lat/lon columns. `VariableShape` and `MeasurementMethodShape` only
fire when a Dataset emits `schema:variableMeasured` /
`schema:measurementMethod`; the Phase 1 profile does not, so those
shapes don't trip on the inferred output.

## Re-vendoring procedure

```bash
curl -sSL -o resources/geoconnex/shacl/geoconnex.ttl \
  https://raw.githubusercontent.com/internetofwater/nabu/main/shacl_validator/shapes/geoconnex.ttl
```

If the upstream evolves (e.g. adds variable-shape constraints qsv
should emit), bump the pinned commit SHA above and re-run
`cargo test profile -F all_features` to catch any drift. The
`embedded` key in `resources/profiles/geoconnex.yaml` doesn't need
to change — it's a logical name resolved against
`EMBEDDED_RESOURCES`, not a path.
