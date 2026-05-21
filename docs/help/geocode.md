# geocode

> Geocodes a location against an updatable local copy of the [Geonames](https://www.geonames.org/) cities & the [Maxmind GeoLite2](https://www.maxmind.com/en/geolite-free-ip-geolocation-data) databases — with caching and multi-threading, this offline path geocodes up to 360,000 records/sec! Can also geocode online (forward & reverse) via the [OpenCage](https://opencagedata.com) API, which is sequential and rate-limited.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/geocode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/geocode.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🚀](TableOfContents.md#legend "multithreaded even without an index.")[🌐](TableOfContents.md#legend "has web-aware options.")[🔣](TableOfContents.md#legend "requires UTF-8 encoded input.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")[🌎](TableOfContents.md#legend "has geospatial capabilities.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Geocode Options](#geocode-options) | [Suggest Only Options](#suggest-only-options) | [Reverse Only Option](#reverse-only-option) | [Opencage Only Options](#opencage-only-options) | [Dynamic Formatting Options](#dynamic-formatting-options) | [Cache-Prune Only Option](#cache-prune-only-option) | [Index-Update Only Options](#index-update-only-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Geocodes a location in CSV data against an updatable local copy of the Geonames cities index
and a local copy of the MaxMind GeoLite2 City database.

The Geonames cities index can be retrieved and updated using the `geocode index-*` subcommands.

The GeoLite2 City database will need to be MANUALLY downloaded from MaxMind. Though it is
free, you will need to create a MaxMind account to download the GeoIP2 Binary database (mmdb)
from <https://www.maxmind.com/en/accounts/current/geoip/downloads>.
Copy the GeoLite2-City.mmdb file to the ~/.qsv-cache/ directory or point to it using the
QSV_GEOIP2_FILENAME environment variable.

When you run the command for the first time, it will download a prebuilt Geonames cities
index from the qsv GitHub repo and use it going forward. You can operate on the local
index using the `geocode index-*` subcommands.

By default, the prebuilt index uses the Geonames Gazeteer cities15000.zip file using
English names. It contains cities with populations > 15,000 (about ~26k cities).
See <https://download.geonames.org/export/dump/> for more information.

It has twelve major subcommands:  
* suggest        - given a partial City name, return the closest City's location metadata
per the local Geonames cities index (Jaro-Winkler distance)
* suggestnow     - same as suggest, but using a partial City name from the command line,
instead of CSV data.
* reverse        - given a WGS-84 location coordinate, return the closest City's location
metadata per the local Geonames cities index.
(Euclidean distance - shortest distance "as the crow flies")
* reversenow     - sames as reverse, but using a coordinate from the command line,
instead of CSV data.
* countryinfo    - returns the country information for the ISO-3166 2-letter country code
(e.g. US, CA, MX, etc.)
* countryinfonow - same as countryinfo, but using a country code from the command line,
instead of CSV data.
* iplookup       - given an IP address or URL, return the closest City's location metadata
per the local Maxmind GeoLite2 City database.
* iplookupnow    - same as iplookup, but using an IP address or URL from the command line,
instead of CSV data.
* opencage       - ONLINE forward/reverse geocoding using the OpenCage API.
Forward-geocodes a free-form address, or reverse-geocodes a
"lat, long" coordinate. Requires an OpenCage API key.
* opencagenow    - same as opencage, but using an address/coordinate from the
command line, instead of CSV data.
* index-*        - operations to update the local Geonames cities index.
(index-check, index-update, index-load & index-reset)
* cache-*        - operations to manage the persistent on-disk OpenCage result cache.
(cache-clear, cache-prune & cache-info)

### Suggest

Suggest a Geonames city based on a partial city name. It returns the closest Geonames
city record based on the Jaro-Winkler distance between the partial city name and the
Geonames city name.

The geocoded information is formatted based on --formatstr, returning it in
'%location' format (i.e. "(lat, long)") if not specified.

Use the --new-column option if you want to keep the location column, e.g.

Geocode file.csv city column and set the geocoded value to a new column named lat_long.

```console
$ qsv geocode suggest city --new-column lat_long file.csv
```


Limit suggestions to the US, Canada and Mexico.

```console
$ qsv geocode suggest city --country us,ca,mx file.csv
```


Limit suggestions to New York State and California, with matches in New York state
having higher priority as its listed first.

```console
$ qsv geocode suggest city --country us --admin1 "New York,US.CA" file.csv
```


If we use admin1 codes, we can omit --country as it will be inferred from the admin1 code prefix.

```console
$ qsv geocode suggest city --admin1 "US.NY,US.CA" file.csv
```


Geocode file.csv city column with --formatstr=%state and set the
geocoded value a new column named state.

```console
$ qsv geocode suggest city --formatstr %state --new-column state file.csv
```


Use dynamic formatting to create a custom format.

```console
$ qsv geocode suggest city -f "{name}, {admin1}, {country} in {timezone}" file.csv
```


Using French place names. You'll need to rebuild the index with the --languages option first

```console
$ qsv geocode suggest city -f "{name}, {admin1}, {country} in {timezone}" -l fr file.csv
```


### Suggestnow

Accepts the same options as suggest, but does not require an input file.
Its default format is more verbose - "{name}, {admin1} {country}: {latitude}, {longitude}"

```console
$ qsv geocode suggestnow "New York"
```

```console
$ qsv geocode suggestnow --country US -f %cityrecord "Paris"
```

```console
$ qsv geocode suggestnow --admin1 "US:OH" "Athens"
```


### Reverse

Reverse geocode a WGS 84 coordinate to the nearest City. It returns the closest Geonames
city record based on the Euclidean distance between the coordinate and the nearest city.
It accepts "lat, long" or "(lat, long)" format.

The geocoded information is formatted based on --formatstr, returning it in
'%city-admin1' format if not specified, e.g.

Reverse geocode file.csv LatLong column. Set the geocoded value to a new column named City.

```console
$ qsv geocode reverse LatLong -c City file.csv
```


Reverse geocode file.csv LatLong column and set the geocoded value to a new column
named CityState, output to a file named file_with_citystate.csv.

```console
$ qsv geocode reverse LatLong -c CityState file.csv -o file_with_citystate.csv
```


The same as above, but get the timezone instead of the city and state.

```console
$ qsv geocode reverse LatLong -f %timezone -c tz file.csv -o file_with_tz.csv
```


### Reversenow

Accepts the same options as reverse, but does not require an input file.

```console
$ qsv geocode reversenow "40.71427, -74.00597"
```

```console
$ qsv geocode reversenow --country US -f %cityrecord "40.71427, -74.00597"
```

```console
$ qsv geocode reversenow "(39.32924, -82.10126)"
```


### Countryinfo

Returns the country information for the specified ISO-3166 2-letter country code.

```console
$ qsv geocode countryinfo country_col data.csv
```

```console
$ qsv geocode countryinfo --formatstr "%json" country_col data.csv
```

```console
$ qsv geocode countryinfo -f "%continent" country_col data.csv
```

```console
$ qsv geocode countryinfo -f "{country_name} ({fips}) in {continent}" country_col data.csv
```


### Countryinfonow

Accepts the same options as countryinfo, but does not require an input file.

```console
$ qsv geocode countryinfonow US
```

```console
$ qsv geocode countryinfonow --formatstr "%pretty-json" US
```

```console
$ qsv geocode countryinfonow -f "%continent" US
```

```console
$ qsv geocode countryinfonow -f "{country_name} ({fips}) in {continent}" US
```


### Iplookup

Given an IP address or URL, return the closest City's location metadata per the
local Geonames cities index.

```console
$ qsv geocode iplookup IP_col data.csv
```

```console
$ qsv geocode iplookup --formatstr "%json" IP_col data.csv
```

```console
$ qsv geocode iplookup -f "%cityrecord" IP_col data.csv
```


### Iplookupnow

Accepts the same options as iplookup, but does not require an input file.

```console
$ qsv geocode iplookupnow 140.174.222.253
```

```console
$ qsv geocode iplookupnow https://amazon.com
```

```console
$ qsv geocode iplookupnow --formatstr "%json" 140.174.222.253
```

```console
$ qsv geocode iplookupnow -f "%cityrecord" 140.174.222.253
```


### Opencage

Online forward or reverse geocoding using the OpenCage Geocoding API
(<https://opencagedata.com>). Unlike the suggest/reverse subcommands which use the
local Geonames index, opencage geocodes real street addresses online.

Requires an OpenCage API key. Set it with --api-key or the QSV_OPENCAGE_API_KEY
environment variable (the --api-key flag takes precedence). Get a free key at
<https://opencagedata.com/users/sign_up>.

The <column> may contain either a free-form address (forward geocoding) or a
"lat, long" / "(lat, long)" WGS-84 coordinate (reverse geocoding). The mode is
auto-detected per row; pass --reverse to force reverse geocoding.

OpenCage's Terms of Service explicitly allow caching, so results are cached in a
persistent on-disk cache (see --cache-ttl & --no-cache). Re-runs and duplicate
queries do NOT re-hit the API. The free tier allows 2,500 requests/day at 1
request/second; rows are processed sequentially and rate-limited (see --rate-limit).

The --country option, if set, restricts results to the given ISO 3166-1 alpha-2
country code(s). The --timeout, --language, --invalid-result, --new-column, --rename
and --output options behave as they do for the other subcommands.

The --formatstr option supports these OpenCage-specific formats:  
* '%+' | '%formatted'   - the OpenCage formatted address (default)
* '%lat-long'           - <latitude>, <longitude>
* '%location'           - (<latitude>, <longitude>)
* '%city'               - the city/town/village
* '%state' | '%admin1'  - the state/province
* '%county' | '%admin2' - the county
* '%country'            - the ISO 3166-1 alpha-2 country code
* '%country_name'       - the country name
* '%postcode'           - the postal code
* '%confidence'         - the OpenCage confidence score (0-10)
* '%json'               - the first OpenCage result as JSON
* '%pretty-json'        - the first OpenCage result as pretty JSON
Dynamic formatting is also supported, using dotted keys, e.g.
"{components.city}, {components.country}" or "{annotations.timezone.name}".
Available keys: formatted, lat, lng, confidence, components.<name> and
annotations.<dotted.path>.

The special "%dyncols:" format is also supported, adding multiple columns to the
output CSV. Set --formatstr to "%dyncols:" followed by a comma-delimited list of
"{col_name:key}" pairs, where key is one of the dynamic keys above, e.g.
"%dyncols: {city:components.city}, {tz:annotations.timezone.name}"
Like the other subcommands, "%dyncols:" cannot be combined with --new-column.

```console
$ qsv geocode opencage address --api-key YOURKEY file.csv
```

```console
$ qsv geocode opencage address --country us -f '%json' file.csv
```

```console
$ qsv geocode opencage coord_col --reverse -c city file.csv
```

```console
$ qsv geocode opencage address -f '{components.city}, {components.country}' file.csv
```

```console
$ qsv geocode opencage address -f '%dyncols: {city:components.city}, {pc:components.postcode}' file.csv
```


### Opencagenow

Accepts the same options as opencage, but does not require an input file.

```console
$ qsv geocode opencagenow --api-key YOURKEY "Brooklyn, NY"
```

```console
$ qsv geocode opencagenow "40.71427, -74.00597"
```

```console
$ qsv geocode opencagenow -f '%pretty-json' "Eiffel Tower, Paris"
```


INDEX-<operation>
Manage the local Geonames cities index used by the geocode command.

It has four operations:  
* check  - checks if the local Geonames index is up-to-date compared to the Geonames website.
returns the index file's metadata JSON to stdout.
* update - updates the local Geonames index with the latest changes from the Geonames website.
use this command judiciously as it downloads about ~200mb of data from Geonames
and rebuilds the index from scratch using the --languages option.
If you don't need a language other than English, use the index-load subcommand instead
as it's faster and will not download any data from Geonames.
* reset  - resets the local Geonames index to the default prebuilt, English-only Geonames cities
index (cities15000) - downloading it from the qsv GitHub repo for the current qsv version.
* load   - load a Geonames cities index from a file, making it the default index going forward.
If set to 15000, it will download the prebuilt English-only cities15000 Geonames
index rkyv file from the qsv GitHub repo for the current qsv version.

Update the Geonames cities index with the latest changes.

```console
$ qsv geocode index-update
```


Rebuild the index using the latest Geonames data w/ English, French, German & Spanish place names

```console
$ qsv geocode index-update --languages en,fr,de,es
```


Load an alternative Geonames cities index from a file, making it the default index going forward.

```console
$ qsv geocode index-load my_geonames_index.rkyv
```


CACHE-<operation>
Manage the persistent on-disk OpenCage result cache used by the opencage subcommands.
This cache is separate from the Geonames cities index and is only populated by the
opencage/opencagenow subcommands. It lives in {cache-dir}/geocode-opencage_v1.

It has three operations:  
* clear  - wipe the entire OpenCage disk cache, removing all cached results.
* prune  - delete cache entries older than the --older-than value. The value is either
an absolute date/datetime (e.g. 2025-01-31, "2025-01-31 12:00:00") or a
relative age with a unit suffix - s(econds), m(inutes), h(ours), d(ays) or
w(eeks). e.g. 30d, 2w, 48h, 90m, 3600s.
* info   - report the cache directory, entry count, on-disk size and the oldest/newest
cached entry timestamps. Emits a JSON summary to stdout.

Wipe the entire OpenCage cache.

```console
$ qsv geocode cache-clear
```


Delete cached entries older than 30 days.

```console
$ qsv geocode cache-prune --older-than 30d
```


Delete cached entries created before a specific date.

```console
$ qsv geocode cache-prune --older-than 2025-01-01
```


Show cache statistics.

```console
$ qsv geocode cache-info
```



<a name="examples"></a>

## Examples [↩](#nav)

> For US locations, you can retrieve the us_state_fips_code and us_county_fips_code fields of a US City
> to help with Census data enrichment.

```console
qsv geocode suggest city_col --country US -f \
"%dyncols: {geocoded_city_col:name}, {state_col:admin1}, {county_col:admin2},  {state_fips_code:us_state_fips_code}, {county_fips_code:us_county_fips_code}"\
input_data.csv -o output_data_with_fips.csv
```

> For US locations, you can reverse geocode the us_state_fips_code and us_county_fips_code fields of a WGS 84 coordinate
> to help with Census data enrichment. The coordinate can be in "lat, long" or "(lat, long)" format.

```console
qsv geocode reverse wgs84_coordinate_col --country US -f \
"%dyncols: {geocoded_city_col:name}, {state_col:admin1}, {county_col:admin2},  {state_fips_code:us_state_fips_code}, {county_fips_code:us_county_fips_code}"\
input_data.csv -o output_data_with_fips.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_geocode.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv geocode suggest [--formatstr=<string>] [options] <column> [<input>]
qsv geocode suggestnow [options] <location>
qsv geocode reverse [--formatstr=<string>] [options] <column> [<input>]
qsv geocode reversenow [options] <location>
qsv geocode countryinfo [options] <column> [<input>]
qsv geocode countryinfonow [options] <location>
qsv geocode iplookup [options] <column> [<input>]
qsv geocode iplookupnow [options] <location>
qsv geocode opencage [--formatstr=<string>] [options] <column> [<input>]
qsv geocode opencagenow [options] <location>
qsv geocode index-load <index-file>
qsv geocode index-check
qsv geocode index-update [--languages=<lang>] [--cities-url=<url>] [--force] [--timeout=<seconds>]
qsv geocode index-reset
qsv geocode cache-clear [options]
qsv geocode cache-prune --older-than=<val> [options]
qsv geocode cache-info [options]
qsv geocode --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The input file to read from. If not specified, reads from stdin. |
| &nbsp;`<column>`&nbsp; | The column to geocode. Used by suggest, reverse & countryinfo subcommands. For suggest, it must be a column with a City string pattern. For reverse, it must be a column using WGS 84 coordinates in "lat, long" or "(lat, long)" format. For countryinfo, it must be a column with a ISO 3166-1 alpha-2 country code. For iplookup, it must be a column with an IP address or a URL. For opencage, it may be a free-form address OR a WGS 84 coordinate. Note that you can use column selector syntax to select the column, but only the first column will be used. See `select --help` for more information. |
| &nbsp;`<location>`&nbsp; | The location to geocode for suggestnow, reversenow, countryinfonow and iplookupnow subcommands. For suggestnow, its a City string pattern. For reversenow, it must be a WGS 84 coordinate. For countryinfonow, it must be a ISO 3166-1 alpha-2 code. For iplookupnow, it must be an IP address or a URL. For opencagenow, it must be an address OR a WGS 84 coordinate. |
| &nbsp;`<index-file>`&nbsp; | The alternate geonames index file to use. It must be a .rkyv file. For convenience, if this is set to 15000, it will download the prebuilt English-only cities15000 Geonames index rkyv file from the qsv GitHub repo for the current qsv version and use it. Only used by the index-load subcommand. |

<a name="geocode-options"></a>

## Geocode Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑c,`<br>`‑‑new‑column`&nbsp; | string | Put the transformed values in a new column instead. Not valid when using the '%dyncols:' --formatstr option. |  |
| &nbsp;`‑r,`<br>`‑‑rename`&nbsp; | string | New name for the transformed column. |  |
| &nbsp;`‑‑country`&nbsp; | string | The comma-delimited, case-insensitive list of countries to filter for. Country is specified as a ISO 3166-1 alpha-2 (two-letter) country code. <https://en.wikipedia.org/wiki/ISO_3166-2> |  |

<a name="suggest-only-options"></a>

## Suggest Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑min‑score`&nbsp; | float | The minimum Jaro-Winkler distance score. | `0.8` |
| &nbsp;`‑‑admin1`&nbsp; | string | The comma-delimited, case-insensitive list of admin1s to filter for. |  |

<a name="reverse-only-option"></a>

## Reverse Only Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑k,`<br>`‑‑k_weight`&nbsp; | string | Use population-weighted distance for reverse subcommand. (i.e. nearest.distance - k * city.population) Larger values will favor more populated cities. If not set (default), the population is not used and the nearest city is returned. |  |

<a name="opencage-only-options"></a>

## Opencage Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑api‑key`&nbsp; | string | The OpenCage API key for the opencage/opencagenow subcommands. If set, it takes precedence over the QSV_OPENCAGE_API_KEY environment variable. Get a free key at <https://opencagedata.com/users/sign_up>. |  |
| &nbsp;`‑‑rate‑limit`&nbsp; | integer | Maximum number of OpenCage API requests per second. The free tier allows 1 request/second (2,500/day). | `1` |
| &nbsp;`‑‑reverse`&nbsp; | flag | Force reverse geocoding for opencage/opencagenow (treat the query as a "lat, long" WGS-84 coordinate). If not set, forward and reverse mode is auto-detected per row. |  |
| &nbsp;`‑‑no‑annotations`&nbsp; | flag | Omit OpenCage annotations (timezone, currency, etc.) from the result and from %json output. |  |
| &nbsp;`‑‑cache‑ttl`&nbsp; | integer | Time-to-live for the persistent on-disk OpenCage result cache. | `1209600` |
| &nbsp;`‑‑no‑cache`&nbsp; | flag | Disable the persistent on-disk OpenCage cache. Duplicate queries within a run are still de-duplicated. |  |

<a name="dynamic-formatting-options"></a>

## Dynamic Formatting Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑l,`<br>`‑‑language`&nbsp; | string | The language to use when geocoding. The language is specified as a ISO 639-1 code. Note that the Geonames index must have been built with the specified language using the `index-update` subcommand with the --languages option. If the language is not available, the first language in the index is used. | `en` |
| &nbsp;`‑‑invalid‑result`&nbsp; | string | The string to return when the geocode result is empty/invalid. If not set, the original value is used. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`‑b,`<br>`‑‑batch`&nbsp; | integer | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |
| &nbsp;`‑‑timeout`&nbsp; | integer | Timeout for downloading Geonames cities index. | `120` |
| &nbsp;`‑‑cache‑dir`&nbsp; | string | The directory to use for caching the Geonames cities index and the persistent on-disk OpenCage result cache. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. | `~/.qsv-cache` |

<a name="cache-prune-only-option"></a>

## Cache-Prune Only Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑older‑than`&nbsp; | string | Delete OpenCage cache entries older than this value. Accepts an absolute date/datetime (e.g. 2025-01-31) or a relative age with a unit suffix (s/m/h/d/w = seconds, minutes, hours, days or weeks; e.g. 30d, 2w, 48h). Required by the cache-prune subcommand. |  |

<a name="index-update-only-options"></a>

## Index-Update Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑languages`&nbsp; | string | The comma-delimited, case-insensitive list of languages to use when building the Geonames cities index. The languages are specified as a comma-separated list of ISO 639-2 codes. See <https://download.geonames.org/export/dump/iso-languagecodes.txt> to look up codes and <https://download.geonames.org/export/dump/alternatenames/> for the supported language files. 253 languages are currently supported. | `en` |
| &nbsp;`‑‑cities‑url`&nbsp; | string | The URL to download the Geonames cities file from. There are several available at <https://download.geonames.org/export/dump/>. cities500.zip   - cities with populations > 500; ~200k cities, 56mb cities1000.zip  - population > 1000; ~140k cities, 44mb cities5000.zip  - population > 5000; ~53k cities, 21mb cities15000.zip - population > 15000; ~26k cities, 13mb Note that the more cities are included, the larger the local index file will be, lookup times will be slower, and the search results will be different. For convenience, if this is set to 500, 1000, 5000 or 15000, it will be converted to a geonames cities URL. | `https://download.geonames.org/export/dump/cities15000.zip` |
| &nbsp;`‑‑force`&nbsp; | flag | Force update the Geonames cities index. If not set, qsv will check if there are updates available at Geonames.org before updating the index. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`‑p,`<br>`‑‑progressbar`&nbsp; | flag | Show progress bars. Will also show the cache hit rate upon completion. Not valid for stdin. |  |

---
**Source:** [`src/cmd/geocode.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/geocode.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
