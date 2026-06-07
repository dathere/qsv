# get

> Get tabular data from local files, URLs (http/https & `dathere://`) & [CKAN](https://ckan.org) (`ckan://`) into a managed, queryable disk cache - with conditional revalidation (ETag/Last-Modified), transparent [zstd](https://github.com/facebook/zstd) compression, [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) hashing & automatic indexing. Cached resources are reusable by ANY qsv command via the `dc:` prefix (e.g. `qsv stats dc:data.csv`), with stale entries auto-refreshed. Efficiently seeds `luau` lookup tables, `validate` dynamicEnum reference data & speeds up Datapusher+ harvesting.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/get.rs](https://github.com/dathere/qsv/blob/master/src/cmd/get.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🌐](TableOfContents.md#legend "has web-aware options.") [![CKAN](../images/ckan.png)](TableOfContents.md#legend "has CKAN-aware integration options.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Get Options](#get-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Get tabular data from various sources into a managed, queryable disk cache.

`get` fetches a resource once, stores it compressed (zstd) and content-addressed
(BLAKE3) in the qsv cache, auto-builds a qsv index for it (for instant random
access & exact record counts), and records rich metadata (ETag, Last-Modified,
sizes, record count, TTL). Re-fetches send a conditional request
(ETag/Last-Modified) so unchanged resources are revalidated, not re-downloaded.
Large remote resources stream into the cache as parallel byte-ranges (tune with
the QSV_GET_PART_SIZE and QSV_GET_CONCURRENCY env vars).

Once cached, a resource can be read by ANY qsv command using the `dc:` prefix,
e.g. `qsv stats dc:data.csv`. Stale `dc:` entries are auto-refreshed.

Supported sources:  
local file path
http:// or https:// URL
dathere://<path>          datHere qsv-lookup-tables repo
ckan://<id>               a CKAN resource by id
ckan://<name>?            a CKAN resource by name (resource_search)
s3://<bucket>/<key>       AWS S3 / S3-compatible       (get_cloud feature)
gs://<bucket>/<key>       Google Cloud Storage         (get_cloud feature)
az://<container>/<key>    Azure Blob Storage           (get_cloud feature)
Cloud credentials are read from the standard AWS_*/AZURE_*/GOOGLE_* environment
variables (and IAM roles); use --cloud-opt for one-off overrides such as region
or endpoint. (sftp:// is planned for a later release.)


<a name="examples"></a>

## Examples [↩](#nav)

Fetch a CSV into the cache and read it back with another command:  
```console
qsv get https://example.com/data.csv --name data.csv
```

```console
qsv stats dc:data.csv
```

Seed a CKAN reference table:  
```console
qsv get "ckan://covid-vaccinations?" --name vax.csv
```

Fetch from cloud object storage (requires the get_cloud feature):  
```console
qsv get s3://my-bucket/data.csv --name data.csv
```

```console
qsv get gs://my-bucket/data.csv --cloud-opt skip_signature=true
```

Show what's in the cache, then prune old entries:  
```console
qsv get cache-list
```

```console
qsv get cache-prune --older-than=30d
```

Verify cached blob integrity, then retune an entry's TTL & policy:  
```console
qsv get cache-list --verify
```

```console
qsv get cache-set-ttl data.csv --ttl=86400
```

```console
qsv get cache-set-policy data.csv --refresh=never
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_get.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv get cache-list [--verify] [options]
qsv get cache-info [options]
qsv get cache-clear [options]
qsv get cache-prune --older-than=<val> [options]
qsv get cache-set-ttl <name> --ttl=<secs> [options]
qsv get cache-set-policy <name> --refresh=<policy> [options]
qsv get [--cloud-opt <kv>...] [options] <source>...
qsv get --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<source>`&nbsp; | One or more sources to fetch into the cache. |
| &nbsp;`<name>`&nbsp; | For cache-set-ttl / cache-set-policy: the cached logical name (`dc:` handle) to modify. |

<a name="get-options"></a>

## Get Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑name`&nbsp; | string | Logical cache name (the `dc:` handle) for the fetched entry. Defaults to the source's terminal path segment. Ignored when multiple sources are given. |  |
| &nbsp;`‑‑ttl`&nbsp; | integer | Per-entry time-to-live in seconds. -1 = never expire. Also the value applied by cache-set-ttl. | `2419200` |
| &nbsp;`‑‑refresh`&nbsp; | string | Staleness policy for `dc:` use: on-stale, always or never. Also the value applied by cache-set-policy. | `on-stale` |
| &nbsp;`‑‑compress`&nbsp; | string | Transparent blob compression: zstd or none. | `zstd` |
| &nbsp;`‑‑force`&nbsp; | flag | Re-fetch even if a fresh cached copy exists. |  |
| &nbsp;`‑‑cloud‑opt`&nbsp; | string | Extra cloud object-store config as a `key=value` pair (repeatable), e.g. region=us-east-1 or skip_signature=true. Overrides the AWS_*/AZURE_*/GOOGLE_* environment. (get_cloud only) |  |
| &nbsp;`‑‑ckan‑api`&nbsp; | string | CKAN Action API base URL. Overrides the QSV_CKAN_API env var. | `https://data.dathere.com/api/3/action` |
| &nbsp;`‑‑ckan‑token`&nbsp; | string | CKAN API token. Overrides the QSV_CKAN_TOKEN env var. |  |
| &nbsp;`‑‑timeout`&nbsp; | integer | HTTP request timeout in seconds. | `30` |
| &nbsp;`‑‑older‑than`&nbsp; | string | For cache-prune: remove entries older than this age. Accepts seconds, or a value with an s/m/h/d/w suffix (e.g. 3600, 90m, 30d, 2w). |  |
| &nbsp;`‑‑json`&nbsp; | flag | For cache-list/cache-info: output JSON instead of a table. |  |
| &nbsp;`‑‑verify`&nbsp; | flag | For cache-list: recompute each cached blob's BLAKE3 and report OK/FAIL per name (exits non-zero on any failure). |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑‑cache‑dir`&nbsp; | string | The qsv cache directory. Overrides the QSV_CACHE_DIR env var. | `~/.qsv-cache` |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | For a single <source>, also write the fetched (decompressed) data to <file> (use `-` for stdout). |  |
| &nbsp;`‑q,`<br>`‑‑quiet`&nbsp; | flag | Do not print progress/summary messages to stderr. |  |

---
**Source:** [`src/cmd/get.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/get.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
