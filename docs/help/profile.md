# profile

> Extract, derive & infer metadata from a CSV (local path or URL) - using the statistical profile of a dataset, mapped and driven by a metadata [scheming](https://github.com/ckan/ckanext-scheming) YAML spec, with optional CKAN/DCAT metadata discovery for URL inputs. This enables [FAIRification](https://www.go-fair.org/fair-principles/fairification-process/) at scale.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/profile.rs](https://github.com/dathere/qsv/blob/master/src/cmd/profile.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")[📚](TableOfContents.md#legend "has lookup table support, enabling runtime \"lookups\" against local or remote reference CSVs.")[⛩️](TableOfContents.md#legend "uses Mini Jinja template engine.") [![CKAN](../images/ckan.png)](TableOfContents.md#legend "has CKAN-aware integration options.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Profile Options](#profile-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Extract, derive & infer metadata from a CSV (local path or URL) - using the statistical profile of a
dataset, mapped and driven by a metadata scheming YAML spec. CKAN/DCAT metadata is optionally
discovered and ingested as a base layer when the input is a URL with DCAT markup.

This is the non-interactive, qsv-native FAIRification counterpart to what datapusher-plus (DP+)
does in CKAN: run statistical + frequency analysis on the input, build a Jinja2 context with the results,
then evaluate Jinja2 formulae/suggestions using this context as declared in the scheming YAML.
The resulting `.metadata.json` carries both a CKAN-shaped block and a best-effort DCAT v3
projection (starting with DCAT-US v3), DP+ to prepopulate CKAN packages.

Helpers and filters are a native Rust port of DP+'s `jinja2_helpers.py`, built on `minijinja`.

For an example spec file, see:  
<https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml>

For more extensive examples, see <https://github.com/dathere/qsv/blob/master/tests/test_profile.rs>.


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

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑spec`&nbsp; | string | CKAN scheming YAML spec file. If omitted, only the inferred `dpp` block (lat/lon/date columns, dataset stats) is emitted; no formulas are evaluated. |  |
| &nbsp;`‑‑initial‑context`&nbsp; | string | JSON file providing seed values for the package / resource dicts plus optional JSON-Pointer overrides for the final DCAT block. Replaces the older --package-meta / --resource-meta flags. Top-level keys: `package`, `resource`, `dataset_info`. Each leaf value may be wrapped as {"value": ..., "force": true} to mark it as overriding any value discovered from URL DCAT markup AND any value qsv inferred. Force is honored across all three subtrees: dataset_info entries override their target path verbatim; package / resource entries route through the active profile's `field_mappings:` table (e.g. `package.title force=true` lands at `/dcat/dct:title`, beating inference and discovery). Forced values for slots the profile does not surface are silently dropped (no-op). See tests/resources/profile/dcat-init-context.README.md for a fully-populated example. |  |
| &nbsp;`‑‑no‑dcat`&nbsp; | flag | Skip the DCAT-US v3 projection block. |  |
| &nbsp;`‑‑no‑ckan`&nbsp; | flag | Skip the CKAN-shape block. |  |
| &nbsp;`‑‑dcat‑legacy‑license`&nbsp; | flag | Transitional: re-emit dct:license on the Dataset alongside the v3-required Distribution-level copy. Default: off (strict v3, license on Distribution only). |  |
| &nbsp;`‑‑no‑dcat‑discovery`&nbsp; | flag | Skip DCAT-markup discovery on URL inputs. Discovery sniffs HTTP Link: rel=describedBy (and, in future, sibling .metadata.json / JSON-LD <script> blocks) to use the publisher's stated metadata as a base layer. |  |
| &nbsp;`‑‑dcat‑discovery‑timeout`&nbsp; | integer | Per-request timeout for DCAT-markup discovery probes. Default: 5. |  |
| &nbsp;`‑‑validate‑dcat`&nbsp; | flag | Validate the emitted dcat block against the vendored GSA DCAT-US v3 JSON Schema bundle (see resources/dcat-us-v3/). Catches missing mandatory fields, cardinality issues, and shape violations across the full v3 spec. Violations append to dcat_warnings by default. |  |
| &nbsp;`‑‑strict‑dcat`&nbsp; | flag | With --validate-dcat, fail the command on any schema violation instead of warning. |  |
| &nbsp;`‑‑catalog`&nbsp; | flag | Wrap the emitted DCAT-US v3 Dataset inside a dcat:Catalog envelope (Catalog{dataset:[...]}). Useful for federation harvesters (data.gov, CKAN ingest) that expect Catalog-shaped top-level metadata. Default: off (Dataset-only, backwards-compatible). |  |
| &nbsp;`‑‑profile`&nbsp; | string | Metadata projection profile to use. Embedded names: dcat-us-v3 (default), dcat-ap-v3, croissant. A path to a custom YAML profile is also accepted; embedded names always win over same-named files. See resources/profiles/README.md for the schema and authoring guide. |  |
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
