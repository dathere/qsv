# fetch

> Send/Fetch data to/from web services for every row using **HTTP Get**. Comes with HTTP/2 adaptive flow control, jaq JSON query language support, dynamic throttling (RateLimit) & caching with available persistent caching using Redis or a disk-cache.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fetch.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fetch.rs)** | 📇🧠🌐

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Fetch Options](#fetch-options) | [Caching Options](#caching-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Send/Fetch data to/from web services for every row using HTTP Get.

Fetch is integrated with `jaq` (a jq clone) to directly parse out values from an API JSON response.
(See <https://github.com/01mf02/jaq> for more info on how to use the jaq JSON Query Language)

CACHE OPTIONS:
Fetch caches responses to minimize traffic and maximize performance. It has four
mutually-exclusive caching options:

1. In memory cache (the default)
2. Disk cache
3. Redis cache
4. No cache

In memory Cache:
In memory cache is the default and is used if no caching option is set.
It uses a non-persistent, in-memory, 2 million entry Least Recently Used (LRU)
cache for each fetch session. To change the maximum number of entries in the cache,
set the --mem-cache-size option.

Disk Cache:
For persistent, inter-session caching, a DiskCache can be enabled with the --disk-cache flag.
By default, it will store the cache in the directory ~/.qsv/cache/fetch, with a cache expiry
Time-to-Live (TTL) of 2,419,200 seconds (28 days), and cache hits NOT refreshing the TTL
of cached values.

Set the --disk-cache-dir option and the environment variables QSV_DISKCACHE_TTL_SECS and
QSV_DISKCACHE_TTL_REFRESH to change default DiskCache settings.

Redis Cache:
Another persistent, inter-session cache option is a Redis cache enabled with the --redis flag.
By default, it will connect to a local Redis instance at redis://127.0.0.1:6379/1,
with a cache expiry Time-to-Live (TTL) of 2,419,200 seconds (28 days),
and cache hits NOT refreshing the TTL of cached values.

Set the environment variables QSV_REDIS_CONNSTR, QSV_REDIS_TTL_SECONDS and
QSV_REDIS_TTL_REFRESH to change default Redis settings.

If you don't want responses to be cached at all, use the --no-cache flag.

NETWORK OPTIONS:
Fetch recognizes RateLimit and Retry-After headers and dynamically throttles requests
to be as fast as allowed. The --rate-limit option sets the maximum number of queries per second
(QPS) to be made. The default is 0, which means to go as fast as possible, automatically
throttling as required, based on rate-limit and retry-after response headers.

To use a proxy, set the environment variables HTTP_PROXY, HTTPS_PROXY or ALL_PROXY
(e.g. export HTTPS_PROXY=socks5://127.0.0.1:1086).

```console
qsv fetch supports brotli, gzip and deflate automatic decompression for improved throughput
```

and performance, preferring brotli over gzip over deflate.

It automatically upgrades its connection to the much faster and more efficient HTTP/2 protocol
with adaptive flow control if the server supports it.
See <https://www.cloudflare.com/learning/performance/http2-vs-http1.1/> and
<https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518> for more info.

URL OPTIONS:
<url-column> needs to be a fully qualified URL path. Alternatively, you can dynamically
construct URLs for each CSV record with the --url-template option (see Examples below).

EXAMPLES USING THE URL-COLUMN ARGUMENT:

data.csv
### URL

<https://api.zippopotam.us/us/90210>
<https://api.zippopotam.us/us/94105>
<https://api.zippopotam.us/us/92802>

Given the data.csv above, fetch the JSON response.

```console
$ qsv fetch URL data.csv
```


Note the output will be a JSONL file - with a minified JSON response per line, not a CSV file.

Now, if we want to generate a CSV file with the parsed City and State, we use the
new-column and jaq options.

```console
$ qsv fetch URL --new-column CityState --jaq '[ ."places"[0]."place name",."places"[0]."state abbreviation" ]' \
data.csv > data_with_CityState.csv
```


data_with_CityState.csv
URL, CityState,
<https://api.zippopotam.us/us/90210>, "[\"Beverly Hills\",\"CA\"]"
<https://api.zippopotam.us/us/94105>, "[\"San Francisco\",\"CA\"]"
<https://api.zippopotam.us/us/92802>, "[\"Anaheim\",\"CA\"]"

As you can see, entering jaq selectors on the command line is error prone and can quickly become cumbersome.
Alternatively, the jaq selector can be saved and loaded from a file using the --jaqfile option.

```console
$ qsv fetch URL --new-column CityState --jaqfile places.jaq data.csv > datatest.csv
```


EXAMPLES USING THE --URL-TEMPLATE OPTION:

Instead of using hardcoded URLs, you can also dynamically construct the URL for each CSV row using CSV column
values in that row.

Exanple 1:
For example, we have a CSV with four columns and we want to geocode against the geocode.earth API that expects
latitude and longitude passed as URL parameters.

addr_data.csv
location, description, latitude, longitude
Home, "house is not a home when there's no one there", 40.68889829703977, -73.99589368107037
X, "marks the spot", 40.78576117777992, -73.96279560368552
work, "moolah", 40.70692672280804, -74.0112264146281
school, "exercise brain", 40.72916494539206, -73.99624185993626
gym, "exercise muscles", 40.73947342617386, -73.99039923885411

Geocode addresses in addr_data.csv, pass the latitude and longitude fields and store
the response in a new column called response into enriched_addr_data.csv.

```console
$ qsv fetch --url-template "https://api.geocode.earth/v1/reverse?point.lat={latitude}&point.lon={longitude}" \
addr_data.csv -c response > enriched_addr_data.csv
```


Example 2:
Geocode addresses in addresses.csv, pass the "street address" and "zip-code" fields
and use jaq to parse placename from the JSON response into a new column in addresses_with_placename.csv.
Note how field name non-alphanumeric characters (space and hyphen) in the url-template were replaced with _.

```console
$ qsv fetch --jaq '."features"[0]."properties", ."name"' addresses.csv -c placename --url-template \
"https://api.geocode.earth/v1/search/structured?address={street_address}&postalcode={zip_code}" \
> addresses_with_placename.csv
```


USING THE HTTP-HEADER OPTION:

The --http-header option allows you to append arbitrary key value pairs (a valid pair is a key and value
separated by a colon) to the HTTP header (to authenticate against an API, pass custom header fields, etc.).
Note that you can pass as many key-value pairs by using --http-header option repeatedly. For example:

```console
$ qsv fetch URL data.csv --http-header "X-Api-Key:TEST_KEY" -H "X-Api-Secret:ABC123XYZ" -H "Accept-Language: fr-FR"
```


For more extensive examples, see <https://github.com/dathere/qsv/blob/master/tests/test_fetch.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv fetch [<url-column> | --url-template <template>] [--jaq <selector> | --jaqfile <file>] [--http-header <k:v>...] [options] [<input>]
qsv fetch --help
```

<a name="fetch-options"></a>

## Fetch Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--url-template`&nbsp; | string | URL template to use. Use column names enclosed with curly braces to insert the CSV data for a record. Mutually exclusive with url-column. |  |
| &nbsp;`-c,`<br>`--new-column`&nbsp; | string | Put the fetched values in a new column. Specifying this option results in a CSV. Otherwise, the output is in JSONL format. |  |
| &nbsp;`--jaq`&nbsp; | string | Apply jaq selector to API returned JSON value. Mutually exclusive with --jaqfile, |  |
| &nbsp;`--jaqfile`&nbsp; | string | Load jaq selector from file instead. Mutually exclusive with --jaq. |  |
| &nbsp;`--pretty`&nbsp; | flag | Prettify JSON responses. Otherwise, they're minified. If the response is not in JSON format, it's passed through. Note that --pretty requires the --new-column option. |  |
| &nbsp;`--rate-limit`&nbsp; | string | Rate Limit in Queries Per Second (max: 1000). Note that fetch dynamically throttles as well based on rate-limit and retry-after response headers. Set to 0 to go as fast as possible, automatically throttling as required. CAUTION: Only use zero for APIs that use RateLimit and/or Retry-After headers, otherwise your fetch job may look like a Denial Of Service attack. Even though zero is the default, this is mitigated by --max-errors having a default of 10. | `0` |
| &nbsp;`--timeout`&nbsp; | string | Timeout for each URL request. | `30` |
| &nbsp;`-H,`<br>`--http-header`&nbsp; | string | Append custom header(s) to the HTTP header. Pass multiple key-value pairs by adding this option multiple times, once for each pair. The key and value should be separated by a colon. |  |
| &nbsp;`--max-retries`&nbsp; | string | Maximum number of retries per record before an error is raised. | `5` |
| &nbsp;`--max-errors`&nbsp; | string | Maximum number of errors before aborting. Set to zero (0) to continue despite errors. | `10` |
| &nbsp;`--store-error`&nbsp; | flag | On error, store error code/message instead of blank value. |  |
| &nbsp;`--cookies`&nbsp; | flag | Allow cookies. |  |
| &nbsp;`--user-agent`&nbsp; | string | Specify custom user agent. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND. Try to follow the syntax here - <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent> |  |
| &nbsp;`--report`&nbsp; | string | Creates a report of the fetch job. The report has the same name as the input file with the ".fetch-report" suffix. There are two kinds of report - d for "detailed" & s for "short". The detailed report has the same columns as the input CSV with six additional columns - qsv_fetch_url, qsv_fetch_status, qsv_fetch_cache_hit, qsv_fetch_retries, qsv_fetch_elapsed_ms & qsv_fetch_response. The short report only has the six columns without the "qsv_fetch_" prefix. | `none` |

<a name="caching-options"></a>

## Caching Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--no-cache`&nbsp; | flag | Do not cache responses. |  |
| &nbsp;`--mem-cache-size`&nbsp; | string | Maximum number of entries in the in-memory LRU cache. | `2000000` |
| &nbsp;`--disk-cache`&nbsp; | flag | Use a persistent disk cache for responses. The cache is stored in the directory specified by --disk-cache-dir. If the directory does not exist, it will be created. If the directory exists, it will be used as is. It has a default Time To Live (TTL)/lifespan of 28 days and cache hits do not refresh the TTL of cached values. Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH env vars to change DiskCache settings. |  |
| &nbsp;`--disk-cache-dir`&nbsp; | string | The directory <dir> to store the disk cache. Note that if the directory does not exist, it will be created. If the directory exists, it will be used as is, and will not be flushed. This option allows you to maintain several disk caches for different fetch jobs (e.g. one for geocoding, another for weather, etc.) | `~/.qsv/cache/fetch` |
| &nbsp;`--redis-cache`&nbsp; | flag | Use Redis to cache responses. It connects to "redis://127.0.0.1:6379/1" with a connection pool size of 20, with a TTL of 28 days, and a cache hit NOT renewing an entry's TTL. Adjust the QSV_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE, QSV_REDIS_TTL_SECONDS & QSV_REDIS_TTL_REFRESH env vars respectively to change Redis settings. This option is ignored if the --disk-cache option is enabled. |  |
| &nbsp;`--cache-error`&nbsp; | flag | Cache error responses even if a request fails. If an identical URL is requested, the cached error is returned. Otherwise, the fetch is attempted again for --max-retries. |  |
| &nbsp;`--flush-cache`&nbsp; | flag | Flush all the keys in the current cache on startup. This only applies to Disk and Redis caches. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Namely, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Will also show the cache hit rate upon completion. Not valid for stdin. |  |

---
**Source:** [`src/cmd/fetch.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fetch.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
