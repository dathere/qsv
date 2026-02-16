# geocode

> Geocodes a location against an updatable local copy of the [Geonames](https://www.geonames.org/) cities & the [Maxmind GeoLite2](https://www.maxmind.com/en/geolite-free-ip-geolocation-data) databases. With caching and multi-threading, it geocodes up to 360,000 records/sec!

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/geocode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/geocode.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.">🧠</abbr><abbr title="has web-aware options.">🌐</abbr><abbr title="multithreaded even without an index.">🚀</abbr><abbr title="requires UTF-8 encoded input.">🔣</abbr><abbr title="has powerful column selector support. See `select` for syntax.">👆</abbr><abbr title="has geospatial capabilities.">🌎</abbr>

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Geocode Options](#geocode-options) | [Suggest Only Options](#suggest-only-options) | [Reverse Only Option](#reverse-only-option) | [Dynamic Formatting Options](#dynamic-formatting-options) | [Index-update Only Options](#index-update-only-options) | [Common Options](#common-options)

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

It has seven major subcommands:
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
* index-*        - operations to update the local Geonames cities index.
(index-check, index-update, index-load & index-reset)

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
If set to 500, 1000, 5000 or 15000, it will download the corresponding English-only
Geonames index rkyv file from the qsv GitHub repo for the current qsv version.

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
qsv geocode index-load <index-file>
qsv geocode index-check
qsv geocode index-update [--languages=<lang>] [--cities-url=<url>] [--force] [--timeout=<seconds>]
qsv geocode index-reset
qsv geocode --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The input file to read from. If not specified, reads from stdin. |
| &nbsp;`<column>`&nbsp; | The column to geocode. Used by suggest, reverse & countryinfo subcommands. For suggest, it must be a column with a City string pattern. For reverse, it must be a column using WGS 84 coordinates in "lat, long" or "(lat, long)" format. For countryinfo, it must be a column with a ISO 3166-1 alpha-2 country code. For iplookup, it must be a column with an IP address or a URL. Note that you can use column selector syntax to select the column, but only the first column will be used. See `select --help` for more information. |
| &nbsp;`<location>`&nbsp; | The location to geocode for suggestnow, reversenow, countryinfonow and iplookupnow subcommands. For suggestnow, its a City string pattern. For reversenow, it must be a WGS 84 coordinate. For countryinfonow, it must be a ISO 3166-1 alpha-2 code. For iplookupnow, it must be an IP address or a URL. |
| &nbsp;`<index-file>`&nbsp; | The alternate geonames index file to use. It must be a .rkyv file. For convenience, if this is set to 500, 1000, 5000 or 15000, it will download the corresponding English-only Geonames index rkyv file from the qsv GitHub repo for the current qsv version and use it. Only used by the index-load subcommand. |

<a name="geocode-options"></a>

## Geocode Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-c,`<br>`--new-column`&nbsp; | string | Put the transformed values in a new column instead. Not valid when using the '%dyncols:' --formatstr option. |  |
| &nbsp;`-r,`<br>`--rename`&nbsp; | string | New name for the transformed column. |  |
| &nbsp;`--country`&nbsp; | string | The comma-delimited, case-insensitive list of countries to filter for. Country is specified as a ISO 3166-1 alpha-2 (two-letter) country code. <https://en.wikipedia.org/wiki/ISO_3166-2> |  |

<a name="suggest-only-options"></a>

## Suggest Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--min-score`&nbsp; | string | The minimum Jaro-Winkler distance score. | `0.8` |
| &nbsp;`--admin1`&nbsp; | string | The comma-delimited, case-insensitive list of admin1s to filter for. |  |

<a name="reverse-only-option"></a>

## Reverse Only Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-k,`<br>`--k_weight`&nbsp; | string | Use population-weighted distance for reverse subcommand. (i.e. nearest.distance - k * city.population) Larger values will favor more populated cities. If not set (default), the population is not used and the nearest city is returned. |  |

<a name="dynamic-formatting-options"></a>

## Dynamic Formatting Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-l,`<br>`--language`&nbsp; | string | The language to use when geocoding. The language is specified as a ISO 639-1 code. Note that the Geonames index must have been built with the specified language using the `index-update` subcommand with the --languages option. If the language is not available, the first language in the index is used. | `en` |
| &nbsp;`--invalid-result`&nbsp; | string | The string to return when the geocode result is empty/invalid. If not set, the original value is used. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |
| &nbsp;`--timeout`&nbsp; | string | Timeout for downloading Geonames cities index. | `120` |
| &nbsp;`--cache-dir`&nbsp; | string | The directory to use for caching the Geonames cities index. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. | `~/.qsv-cache` |

<a name="index-update-only-options"></a>

## Index-update Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--languages`&nbsp; | string | The comma-delimited, case-insensitive list of languages to use when building the Geonames cities index. The languages are specified as a comma-separated list of ISO 639-2 codes. See <https://download.geonames.org/export/dump/iso-languagecodes.txt> to look up codes and <https://download.geonames.org/export/dump/alternatenames/> for the supported language files. 253 languages are currently supported. | `en` |
| &nbsp;`--cities-url`&nbsp; | string | The URL to download the Geonames cities file from. There are several available at <https://download.geonames.org/export/dump/>. cities500.zip   - cities with populations > 500; ~200k cities, 56mb cities1000.zip  - population > 1000; ~140k cities, 44mb cities5000.zip  - population > 5000; ~53k cities, 21mb cities15000.zip - population > 15000; ~26k cities, 13mb Note that the more cities are included, the larger the local index file will be, lookup times will be slower, and the search results will be different. For convenience, if this is set to 500, 1000, 5000 or 15000, it will be converted to a geonames cities URL. | `https://download.geonames.org/export/dump/cities15000.zip` |
| &nbsp;`--force`&nbsp; | flag | Force update the Geonames cities index. If not set, qsv will check if there are updates available at Geonames.org before updating the index. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Will also show the cache hit rate upon completion. Not valid for stdin. |  |

---
**Source:** [`src/cmd/geocode.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/geocode.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
