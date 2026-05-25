# profile

> Extract and infer DCAT-US v3 / Croissant metadata from a CSV, optionally driven by a CKAN [scheming](https://github.com/ckan/ckanext-scheming) YAML spec (e.g. [dataset-druf.yaml](https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml)). Reuses [Datapusher+'s](https://github.com/dathere/datapusher-plus) `jinja2_helpers.py` for formula/suggestion evaluation via an embedded Python interpreter. Produces a `.metadata.json` that CKAN uploaders (qsv pro, DP+) consume to prepopulate package + resource metadata. [Requires Python 3.10 or greater](https://github.com/dathere/qsv/blob/master/docs/INTERPRETERS.md#building-qsv-with-python-feature) and the `jinja2` package.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/profile.rs](https://github.com/dathere/qsv/blob/master/src/cmd/profile.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")[📚](TableOfContents.md#legend "has lookup table support, enabling runtime \"lookups\" against local or remote reference CSVs.")[⛩️](TableOfContents.md#legend "uses Mini Jinja template engine.") [![CKAN](../images/ckan.png)](TableOfContents.md#legend "has CKAN-aware integration options.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Profile Options](#profile-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Extract and infer DCAT-3 / Croissant metadata from a CSV, optionally driven by a
CKAN scheming YAML spec.

This is the non-interactive, qsv-native counterpart to what datapusher-plus (DP+)
does in CKAN: run statistical + frequency analysis on the input, build a Jinja2
context (`package`, `resource`, `dpps`, `dppf`, `dpp`), then evaluate every
`formula` / `suggestion_formula` field declared in the scheming YAML. The
resulting `.metadata.json` carries both a CKAN-shaped block and a best-effort
DCAT-US v3 projection, ready for qsv pro and DP+ to prepopulate CKAN packages.

Helpers and filters are a native Rust port of DP+'s `jinja2_helpers.py`,
built on `minijinja`. No Python interpreter is required at runtime; the
SQL-requiring helpers (`temporal_resolution`, `guess_accrual_periodicity`)
query the input CSV directly via Polars SQL.

For an example spec file, see:  
<https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml>

For more extensive examples, see <https://github.com/dathere/qsv/blob/master/tests/test_profile.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv profile [options] [<input>]
qsv profile --help
```

<a name="profile-options"></a>

## Profile Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑spec`&nbsp; | string | CKAN scheming YAML spec file. If omitted, only the inferred `dpp` block (lat/lon/date columns, dataset stats) is emitted; no formulas are evaluated. |  |
| &nbsp;`‑‑initial‑context`&nbsp; | string | JSON file providing seed values for the package / resource dicts plus optional JSON-Pointer overrides for the final DCAT block. Replaces the older --package-meta / --resource-meta flags. Shape: { "package":  {"title": "...", ...}, "resource": {"format": "CSV", ...}, "dataset_info": { "/dcat/dct:title": "Force override" } } Each leaf value may also be wrapped as {"value": ..., "force": true} to mark it as overriding any value discovered from the URL's existing DCAT markup. Phase 4a ships the flag + dataset_info overrides; per-property force semantics land in 4b. |  |
| &nbsp;`‑‑no‑dcat`&nbsp; | flag | Skip the DCAT-US v3 projection block. |  |
| &nbsp;`‑‑no‑ckan`&nbsp; | flag | Skip the CKAN-shape block. |  |
| &nbsp;`‑‑dcat‑legacy‑license`&nbsp; | flag | Transitional: re-emit dct:license on the Dataset alongside the v3-required Distribution-level copy. Default: off (strict v3, license on Distribution only). |  |
| &nbsp;`‑‑no‑dcat‑discovery`&nbsp; | flag | Skip DCAT-markup discovery on URL inputs. Discovery sniffs HTTP Link: rel=describedBy (and, in future, sibling .metadata.json / JSON-LD <script> blocks) to use the publisher's stated metadata as a base layer. |  |
| &nbsp;`‑‑dcat‑discovery‑timeout`&nbsp; | integer | Per-request timeout for DCAT-markup discovery probes. Default: 5. |  |
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
