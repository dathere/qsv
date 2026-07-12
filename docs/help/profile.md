# profile

> Extract, derive & infer metadata from a CSV (local path or URL) - using the statistical profile of a dataset, mapped and driven by a configurable metadata [scheming](https://github.com/ckan/ckanext-scheming) YAML spec ([DCAT-US v3](https://resources.data.gov/resources/dcat-us3/), [DCAT-AP v3](https://semiceu.github.io/DCAT-AP/releases/3.0.0/) and [Croissant 1.1](https://docs.mlcommons.org/croissant/docs/croissant-spec-1.1.html) bundled; [Geoconnex](https://docs.geoconnex.us/reference/overview) when built with the `geoconnex` feature), with optional CKAN/DCAT metadata discovery for URL inputs. This enables [FAIRification](https://www.go-fair.org/fair-principles/fairification-process/) at scale.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/profile.rs](https://github.com/dathere/qsv/blob/master/src/cmd/profile.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")[📚](TableOfContents.md#legend "has lookup table support, enabling runtime \"lookups\" against local or remote reference CSVs.")[⛩️](TableOfContents.md#legend "uses MiniJinja template engine.") [![CKAN](../images/ckan.png)](TableOfContents.md#legend "has CKAN-aware integration options.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Profile Options](#profile-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Profile a CSV (local path or URL) and emit a `.metadata.json` file carrying
five top-level blocks:  

`dpp`        — inferred dataset signals: lat/lon/date columns, file size,
row count, encoding, etc. (the legacy datapusher-plus
inference block).
`stats`      — per-column summary statistics from `qsv stats`.
`frequency`  — per-column value counts from `qsv frequency`.
`ckan`       — a CKAN-shaped block (package + resources) that
datapusher-plus consumes to prepopulate CKAN packages.
`projection` — the dataset re-expressed in the active profile's metadata
vocabulary. Default is DCAT-US v3; bundled alternates are
dcat-ap-v3 (EU portals), croissant (ML/AI registries) and
geoconnex (water-data federations). Consumable directly by
data.gov harvesters, EU DCAT-AP catalogs, mlcommons /
Hugging Face / Kaggle, and Internet of Water tooling.

Behind the scenes qsv runs the same statistical + frequency analysis
datapusher-plus (DP+) runs in CKAN, builds a Jinja2 evaluation context from
the results, and — when an optional CKAN scheming YAML spec is supplied —
evaluates the spec's `formula` / `suggestion_formula` templates against that
context. Jinja2 helpers and filters are a native Rust port of DP+'s
`jinja2_helpers.py`, built on `minijinja`.

When the input is a URL whose response carries DCAT markup (HTTP
`Link: rel=describedBy`), qsv discovers the publisher's stated metadata and
merges it as a base layer beneath the inferred projection.

For an example CKAN scheming YAML spec, see:  
<https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml>

For more extensive examples, see <https://github.com/dathere/qsv/blob/master/tests/test_profile.rs>.
See also <https://github.com/dathere/qsv/wiki/Metadata-Profiling>


<a name="examples"></a>

## Examples [↩](#nav)

> Quick: dpp/stats/frequency + default DCAT-US v3 projection.

```console
qsv profile data.csv
```

> Pipe stdin; output defaults to `stdin.metadata.json`.

```console
cat data.csv | qsv profile
```

> URL input: discover the publisher's DCAT markup and merge it as a base layer.

```console
qsv profile https://data.example.gov/datasets/sample.csv
```

> Seed publisher/contact info from a JSON file; write to a chosen output path.

```console
qsv profile data.csv --initial-context publisher.json -o data.metadata.json
```

> data.gov-style harvest: validate against DCAT-US v3 JSON Schema, abort on
> violations, wrap in a Catalog envelope.

```console
qsv profile data.csv --validate --strict --catalog -o data.metadata.json
```

> DCAT-AP v3 for EU data portals; `pyshacl` validates the bundled SHACL shapes.

```console
qsv profile open-data.csv --profile dcat-ap-v3 --validate --strict
```

> Croissant JSON-LD for an ML dataset; `mlcroissant` validates the output.

```console
qsv profile train.csv --profile croissant --validate -o train.croissant.json
```

> Geoconnex JSON-LD for hydrologic data (qsv built with the `geoconnex` feature).

```console
qsv profile gages.csv --profile geoconnex --validate --strict
```

> Evaluate a CKAN scheming spec: Jinja2 formulas compute spatial/temporal
> extents, accrual periodicity, and other derived fields.

```console
qsv profile data.csv --spec dataset-druf.yaml -o data.metadata.json
```

> CKAN-only output: drop the `projection` block, keep dpp/stats/frequency/ckan.

```console
qsv profile data.csv --no-projection --spec dataset-druf.yaml
```

> Custom YAML profile from disk (embedded names always win over same-named
> files, so use a non-clashing name for custom profiles).

```console
qsv profile data.csv --profile ./my-org-dcat.yaml --validate
```


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv profile [options] [<input>]
qsv profile --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | Path or URL to the CSV to profile. When `-` or omitted, reads from stdin. When the URL has DCAT markup, qsv will attempt to discover and ingest it as a base layer of metadata (unless --no-dcat-discovery is set). See --no-dcat-discovery and --dcat-discovery-timeout for details and opt-out. |

<a name="profile-options"></a>

## Profile Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑spec`&nbsp; | string | CKAN scheming YAML spec file. If omitted, only the inferred `dpp` block (lat/lon/date columns, dataset stats) is emitted; no formulas are evaluated. |  |
| &nbsp;`‑‑initial‑context`&nbsp; | string | JSON file providing seed values for the package / resource dicts plus optional JSON-Pointer overrides for the final projection block. Replaces the older --package-meta / --resource-meta flags. Top-level keys: `package`, `resource`, `dataset_info`. Each leaf value may be wrapped as {"value": ..., "force": true} to mark it as overriding any value discovered from URL DCAT markup AND any value qsv inferred. Force is honored across all three subtrees: dataset_info entries override their target path verbatim; package / resource entries route through the active profile's `field_mappings:` table (e.g. `package.title force=true` lands at `/projection/dct:title`, beating inference and discovery). Forced values for slots the profile does not surface are silently dropped (no-op). See tests/resources/profile/dcat-init-context.README.md for a fully-populated example. |  |
| &nbsp;`‑‑no‑projection`&nbsp; | flag | Skip the metadata projection block (dcat/croissant/ geoconnex, depending on the active profile). |  |
| &nbsp;`‑‑no‑ckan`&nbsp; | flag | Skip the CKAN-shape block. |  |
| &nbsp;`‑‑croissant‑frequency`&nbsp; | flag | Embed per-column value-frequency distributions in the metadata projection. The croissant profile renders them as inline cr:RecordSets (one `<col>-frequency` RecordSet of {value, count, percentage} rows per column), per the spec's "distribution of values is a statistic on the field" guidance. Off by default (keeps the projection compact); the raw counts always remain in the top-level `frequency` block regardless. Other bundled profiles ignore this flag. |  |
| &nbsp;`‑‑dcat‑legacy‑license`&nbsp; | flag | Transitional: re-emit dct:license on the Dataset alongside the v3-required Distribution-level copy. Default: off (strict v3, license on Distribution only). |  |
| &nbsp;`‑‑no‑dcat‑discovery`&nbsp; | flag | Skip DCAT-markup discovery on URL inputs. Discovery sniffs HTTP Link: rel=describedBy (and, in future, sibling .metadata.json / JSON-LD <script> blocks) to use the publisher's stated metadata as a base layer. |  |
| &nbsp;`‑‑dcat‑discovery‑timeout`&nbsp; | integer | Per-request timeout for DCAT-markup discovery probes. Default: 5. |  |
| &nbsp;`‑‑validate`&nbsp; | flag | Validate the emitted projection block against the active profile's declared validators. For dcat-us-v3 that's the vendored GSA JSON Schema bundle (see resources/dcat-us-v3/); for dcat-ap-v3 / geoconnex it's pyshacl over the bundled SHACL shapes; for croissant it's mlcroissant. Catches missing mandatory fields, cardinality issues, and shape violations. Violations append to projection_warnings by default. |  |
| &nbsp;`‑‑strict`&nbsp; | flag | With --validate, fail the command on JSON Schema violations or non-Info external- validator findings (Required/Recommended severities) instead of just warning. Note: RFC4180 structural failures from `qsv validate` (emitted when a spec declares `validators`) are always appended as warnings, regardless of this flag. |  |
| &nbsp;`‑‑allow‑external‑validator`&nbsp; | flag | Opt in to spawning the validator binary declared by `validation.external` when the profile was loaded from an arbitrary YAML file. Bundled profiles (dcat-us-v3, dcat-ap-v3, croissant, geoconnex) always run their declared external validators because the profile content is vetted at qsv release time. Without this flag, file-loaded profiles emit a Recommended-severity warning instead of running the binary, so an untrusted YAML can't silently execute arbitrary commands. Default: off. |  |
| &nbsp;`‑‑catalog`&nbsp; | flag | Wrap the emitted DCAT-US v3 Dataset inside a dcat:Catalog envelope (Catalog{dataset:[...]}). Useful for federation harvesters (data.gov, CKAN ingest) that expect Catalog-shaped top-level metadata. Default: off (Dataset-only, backwards-compatible). |  |
| &nbsp;`‑‑profile`&nbsp; | string | Metadata projection profile to use. Embedded names: dcat-us-v3 (default), dcat-ap-v3, croissant; geoconnex (when built with the `geoconnex` feature — qsv default; qsvdp opt-in via -F datapusher_plus,geoconnex). A path to a custom YAML profile is also accepted; embedded names always win over same-named files. See resources/profiles/README.md for the schema and authoring guide. |  |
| &nbsp;`‑‑force`&nbsp; | flag | Force recomputing cardinality and unique values even if a stats cache file exists. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | The number of jobs to run in parallel for the underlying stats/frequency passes. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Output JSON path. Default: <input>.metadata.json. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Namely, it will be processed with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. |  |
| &nbsp;`‑‑memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/profile.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/profile.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
