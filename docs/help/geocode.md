# geocode

> Geocodes a location against an updatable local copy of the [Geonames](https://www.geonames.org/) cities & the [Maxmind GeoLite2](https://www.maxmind.com/en/geolite-free-ip-geolocation-data) databases — with caching and multi-threading, this offline path geocodes up to 360,000 records/sec! Can also geocode online (forward & reverse) via the [OpenCage](https://opencagedata.com) geocoder, and geocode New York City addresses, BBLs and BINs via NYC's [Geoclient](https://github.com/mlipper/geoclient) API, which also returns NYC attributes such as council districts and census tracts.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/geocode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/geocode.rs)** | [📇](TableOfContents.md#legend "uses an index when available.")[🧠](TableOfContents.md#legend "expensive operations are memoized with available inter-session Redis/Disk caching for fetch commands.")[🚀](TableOfContents.md#legend "multithreaded even without an index.")[🌐](TableOfContents.md#legend "has web-aware options.")[🔣](TableOfContents.md#legend "requires UTF-8 encoded input.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")[🌎](TableOfContents.md#legend "has geospatial capabilities.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Geocode Options](#geocode-options) | [Suggest Only Options](#suggest-only-options) | [Reverse Only Option](#reverse-only-option) | [Opencage & Geoclient Options](#opencage-&-geoclient-options) | [Opencage Only Options](#opencage-only-options) | [Geoclient Only Options](#geoclient-only-options) | [Dynamic Formatting Options](#dynamic-formatting-options) | [Cache-Prune Only Option](#cache-prune-only-option) | [Index-Update Only Options](#index-update-only-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Geocodes a location in CSV data using one of four geocoders:  
* Geonames  - OFFLINE. Matches partial city names, WGS-84 coordinates and country codes
against a local, updatable index of Geonames cities, downloaded
automatically on first use. Multithreaded and cached.
* GeoIP2    - OFFLINE. Locates IP addresses and URLs using a local copy of the MaxMind
GeoLite2 City database, which you download manually.
* OpenCage  - ONLINE. Worldwide forward geocoding of free-form street addresses and
reverse geocoding of coordinates, using the OpenCage API. Requires an
OpenCage API key.
* Geoclient - ONLINE. Geocodes New York City addresses, BBLs, BINs, intersections and
places using NYC's Geoclient API, returning NYC-specific attributes such
as council districts and census tracts. Requires a free NYC API key.

The major subcommands are:  
* suggest      - (Geonames) given a partial city name, return the closest city.
* reverse      - (Geonames) given a WGS-84 coordinate, return the nearest city.
* countryinfo  - (Geonames) given an ISO 3166-1 alpha-2 country code, return the
country's information.
* iplookup     - (GeoIP2) given an IP address or URL, return its city.
* opencage     - (OpenCage) forward or reverse geocode an address or coordinate.
* geoclient    - (Geoclient) geocode a New York City location.
* index-*      - manage the local Geonames cities index.
(index-check, index-update, index-load & index-reset)
* cache-*      - manage the persistent on-disk OpenCage and Geoclient result caches.
(cache-clear, cache-prune & cache-info)

Each geocoding subcommand has a "now" variant - suggestnow, reversenow, countryinfonow,
iplookupnow, opencagenow and geoclientnow - that geocodes a single location given on the
command line instead of a CSV column.

### Geonames
The suggest, reverse and countryinfo subcommands use a local Geonames cities index. Two
prebuilt, English-only indexes, built from the Geonames Gazetteer cities files, are
distributed on the qsv GitHub repo:  

* cities15000 - cities with populations > 15,000 (about ~26k cities). The default.
* cities1000  - cities with populations > 1,000 (about ~140k cities). Resolves small
towns the default omits, at ~4x the index size.

The first time you run one of these subcommands, qsv downloads the default cities15000
index and uses it going forward. To switch to cities1000, run `qsv geocode index-load 1000`.
See <https://download.geonames.org/export/dump/> for more information.
Use the index-* operations (see "index-* operations" below) to update the index, rebuild it
with other languages or more cities, or load another one.

#### suggest
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


#### suggestnow
Accepts the same options as suggest, but does not require an input file.
Its default format is more verbose - "{name}, {admin1} {country}: {latitude}, {longitude}"

```console
$ qsv geocode suggestnow "New York"
```

```console
$ qsv geocode suggestnow --country US -f %cityrecord "Paris"
```

```console
$ qsv geocode suggestnow --admin1 "US.OH" "Athens"
```


#### reverse
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


#### reversenow
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


#### countryinfo
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


#### countryinfonow
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


#### index-* operations
Manage the local Geonames cities index used by the Geonames and GeoIP2 subcommands.

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
If set to a published population floor (15000 or 1000), it will download that
prebuilt English-only Geonames index rkyv file from the qsv GitHub repo for the
current qsv version. cities1000 (~140k cities) resolves small towns the default
cities15000 (~26k) omits, at ~4x the index size.

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


### GeoIP2
The iplookup subcommand locates an IP address - or the IP address a URL's host resolves
to - using a local copy of the MaxMind GeoLite2 City database, then returns that city's
record from the local Geonames cities index. It therefore supports the same --formatstr
formats as the Geonames subcommands. If the IP address cannot be located, or its city is
not in the Geonames index (e.g. a small town not in the default cities15000 index), the
IP address itself is returned.

The GeoLite2 City database will need to be MANUALLY downloaded from MaxMind. Though it is
free, you will need to create a MaxMind account to download the GeoIP2 Binary database (mmdb)
from <https://www.maxmind.com/en/accounts/current/geoip/downloads>.
Copy the GeoLite2-City.mmdb file to the ~/.qsv-cache/ directory or point to it using the
QSV_GEOIP2_FILENAME environment variable.

#### iplookup
Given an IP address or URL, return its city's location metadata. The default format is
'%city-admin1-country' (e.g. "San Francisco, California US").

```console
$ qsv geocode iplookup IP_col data.csv
```

```console
$ qsv geocode iplookup --formatstr "%json" IP_col data.csv
```

```console
$ qsv geocode iplookup -f "%cityrecord" IP_col data.csv
```


#### iplookupnow
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


### OpenCage
Online forward or reverse geocoding using the OpenCage Geocoding API
(<https://opencagedata.com>). Unlike the suggest/reverse subcommands which use the
local Geonames index, opencage geocodes real street addresses online.

Requires an OpenCage API key. Set it with --api-key or the QSV_OPENCAGE_API_KEY
environment variable (the --api-key flag takes precedence). Get a free key at
<https://opencagedata.com/users/sign_up>.

OpenCage's Terms of Service explicitly allow caching, so results are cached in a
persistent on-disk cache (see --cache-ttl & --no-cache). Re-runs and duplicate
queries do NOT re-hit the API. The free tier allows 2,500 requests/day at 1
request/second; rows are processed sequentially and rate-limited (see --rate-limit).

#### opencage
The <column> may contain either a free-form address (forward geocoding) or a
"lat, long" / "(lat, long)" WGS-84 coordinate (reverse geocoding). The mode is
auto-detected per row; pass --reverse to force reverse geocoding.

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


#### opencagenow
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


### Geoclient
Online geocoding of New York City locations using NYC's Geoclient v2 API
(<https://api-portal.nyc.gov>), a REST wrapper around the NYC Department of City
Planning's Geosupport system. Besides coordinates, it returns NYC-specific attributes
such as the BBL, BIN, community district, city council district, census tracts, NTA
and ZIP code. It only geocodes locations in the five boroughs.

The public API requires a free subscription key: sign up at <https://api-portal.nyc.gov>
and subscribe to the "Geoclient V2 User" product. Set the key with --api-key or the
QSV_GEOCLIENT_API_KEY environment variable (--api-key takes precedence).
To use a self-hosted Geoclient (<https://github.com/mlipper/geoclient>), set
QSV_GEOCLIENT_API_URL to its base URL (e.g. <http://localhost:8080/geoclient/v2>) -
no key is needed then.

The public API allows 100 requests/second per IP, 2,500 requests/minute and 500,000
requests/day. Requests are sent concurrently (see --jobs) behind a rate limiter that
defaults to 40 requests/second, to stay under the per-minute cap (see --rate-limit).
Requests that are rate-limited or fail on the server are retried up to 3 times. If 10
requests in a row still fail, the run is aborted, as the quota is likely exhausted.
Results are cached in a persistent on-disk cache (see --cache-ttl & --no-cache), and
duplicate locations are only looked up once.

#### geoclient
The --endpoint option selects the kind of lookup:  
* search       - (default) a free-form NYC location in <column>, e.g.
"314 W 100 St Manhattan", "Empire State Building" or a BBL/BIN.
* bin          - a Building Identification Number in <column>.
* bbl          - a Borough-Block-Lot in <column>, either as 10 digits (1000670001)
or delimited (1-67-1, 1/00067/0001).
* normalize    - a street name in <column>, returned in Geosupport's normalized
form ("w 100 st" -> "WEST  100 STREET"). It only standardizes the
name: it does not check that the street exists. By default, numbers
are padded for sorting ("WEST    1 STREET"), the same spacing as the
street names in other Geoclient results; to compare with
single-spaced data, add --params format=C ("WEST 1 STREET").
* streetcode   - a Geosupport street code in <column> (the borough digit plus a 5-,
7- or 10-digit code: 6, 8 or 11 digits, e.g. 11361001), returned as
the street's name. Other values are left unchanged, not sent.
* address, addresspoint, intersection, place, blockface - structured lookups whose
inputs come from several columns, mapped with --params.

For the structured endpoints, --params maps Geoclient parameters to input columns
("name={Column}") or to fixed values ("name=value"), comma-separated. Parameter names
are case-sensitive:  
* address, addresspoint - houseNumber, street, and borough or zip
* intersection - crossStreetOne, crossStreetTwo, borough
(optional: boroughCrossStreetTwo, compassDirection)
* place        - name, and borough or zip
* blockface    - onStreet, crossStreetOne, crossStreetTwo, borough
(optional: boroughCrossStreetOne, boroughCrossStreetTwo, compassDirection)
The borough may be a name (Manhattan), an abbreviation (MN) or a number (1-5).
With a structured endpoint, <column> is not sent to the API: it is only the column that
is replaced with the result (or that --rename applies to). For search, --params may
set fixed search options, e.g. "exactMatchMaxLevel=5". For normalize and streetcode,
it may set format - S (sort, the default) pads numbers so names sort numerically
("WEST    1 STREET"), C (compact) does not ("WEST 1 STREET") - and length (4-32,
default 32), which abbreviates names to fit (length=10: "W 100 ST").

A location that Geoclient cannot geocode is left unchanged (or set to --invalid-result).
So is a search that matches in MORE than one borough - typically a street address
without its borough, e.g. "100 Broadway" exists in Manhattan, Brooklyn, Queens and
Staten Island - rather than silently picking one; add the borough or ZIP code to the
input. (If one of the matches is an exact match, it is used.) A warning reports how
many rows were ambiguous.

Why a location did not geocode is available through %dyncols keys starting with
"_reject." - filled only on those rows, e.g. {why:_reject.message} gives Geosupport's
reason (e.g. "ADDRESS NUMBER OUT OF RANGE") or, for an ambiguous search, the boroughs
matched - marked "[street only]" where only the street's address range matched, not a
property. Other useful keys are _reject.reasonCode, _reject.message2 (the second
message of address and place lookups) and _reject._resultCount. With geoclientnow,
the JSON formats print the reject under "_reject". So that rejected searches carry
their reason, search sends returnRejections=true unless it is set in --params.

The --formatstr option supports these Geoclient-specific formats:  
* '%+' | '%lat-long'    - <latitude>, <longitude> (default). For normalize and
streetcode, which return no coordinates: the street name
(firstStreetNameNormalized).
* '%location'           - (<latitude>, <longitude>)
* '%bbl'                - the BBL
* '%bin'                - the BIN (buildingIdentificationNumber)
* '%normalized-address' - the address on one line, single-spaced, e.g.
"314 WEST 100 STREET, NEW YORK, NY 10025". Empty unless
the result has a house number (address and search results).
* '%nyc-districts'      - 11 columns, as with "%dyncols:": borough,
community_district, council_district, assembly_district,
state_senate_district, congressional_district,
police_precinct, school_district, nta2020,
census_tract2020 and zip_code. Filled for address,
intersection, place and search results; bbl, bin and
addresspoint results only have the borough.
* '%json'               - the full Geoclient result as JSON
* '%pretty-json'        - the full Geoclient result as pretty JSON
Dynamic formatting is also supported, using any Geoclient result field, e.g.
"{bbl} - council district {cityCouncilDistrict}". Commonly used fields include
latitude, longitude, bbl, buildingIdentificationNumber, communityDistrict,
cityCouncilDistrict, censusTract2020, nta2020, zipCode, xCoordinate, yCoordinate and
uspsPreferredCityName. A field missing from a result renders empty.
Two fields are added by qsv: _resultCount (how many results geocoded successfully)
and, for search, _level (the Geosupport search level of the result used).
For bbl and bin lookups, latitude/longitude are the building's internal label point.

The "%dyncols:" format is also supported, e.g.
"%dyncols: {bbl:bbl}, {council:cityCouncilDistrict}, {tract:censusTract2020}"
"%dyncols: {lat:latitude}, {lng:longitude}, {why:_reject.message}"

```console
$ qsv geocode geoclient address --api-key YOURKEY nyc_311.csv
```

```console
$ qsv geocode geoclient bbl --endpoint bbl -f '%dyncols: {lat:latitude}, {lng:longitude}' pluto.csv
```

```console
$ qsv geocode geoclient Street --endpoint address -c latlong --params "houseNumber={HouseNo},street={Street},borough={Borough}" file.csv
```


#### geoclientnow
Accepts the same options as geoclient (search, bin, bbl, normalize and streetcode
endpoints only), but does not require an input file.

```console
$ qsv geocode geoclientnow "350 5th Ave Manhattan"
```

```console
$ qsv geocode geoclientnow --endpoint bbl -f '%pretty-json' 1008350041
```

```console
$ qsv geocode geoclientnow --endpoint normalize "w 100 st"
```


### OpenCage & Geoclient result caches

#### cache-* operations
Manage the persistent on-disk result caches used by the opencage and geoclient
subcommands. They are separate from the Geonames cities index, and live in
{cache-dir}/geocode-opencage_v1 and {cache-dir}/geocode-geoclient_v1.
Each operation applies to both caches; cache-info prints one JSON line per cache.

It has three operations:  
* clear  - wipe the entire disk caches, removing all cached results.
* prune  - delete cache entries older than the --older-than value. The value is either
an absolute date/datetime (e.g. 2025-01-31, "2025-01-31 12:00:00") or a
relative age with a unit suffix - s(econds), m(inutes), h(ours), d(ays) or
w(eeks). e.g. 30d, 2w, 48h, 90m, 3600s.
* info   - report the cache directory, entry count, on-disk size and the oldest/newest
cached entry timestamps. Emits a JSON summary to stdout.

Wipe the entire OpenCage and Geoclient caches.

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

See also <https://github.com/dathere/qsv/wiki/Geospatial#geocode>

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
qsv geocode geoclient [--formatstr=<string>] [options] <column> [<input>]
qsv geocode geoclientnow [options] <location>
qsv geocode index-load [--timeout=<seconds>] <index-file>
qsv geocode index-check
qsv geocode index-update [--languages=<lang>] [--cities-url=<url>] [--force] [--timeout=<seconds>]
qsv geocode index-reset [--timeout=<seconds>]
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
| &nbsp;`<column>`&nbsp; | The column to geocode. Used by suggest, reverse & countryinfo subcommands. For suggest, it must be a column with a City string pattern. For reverse, it must be a column using WGS 84 coordinates in "lat, long" or "(lat, long)" format. For countryinfo, it must be a column with a ISO 3166-1 alpha-2 country code. For iplookup, it must be a column with an IP address or a URL. For opencage, it may be a free-form address OR a WGS 84 coordinate. For geoclient, it is the NYC location, BIN, BBL, street name or street code (see --endpoint). Note that you can use column selector syntax to select the column, but only the first column will be used. See `select --help` for more information. |
| &nbsp;`<location>`&nbsp; | The location to geocode for suggestnow, reversenow, countryinfonow and iplookupnow subcommands. For suggestnow, its a City string pattern. For reversenow, it must be a WGS 84 coordinate. For countryinfonow, it must be a ISO 3166-1 alpha-2 code. For iplookupnow, it must be an IP address or a URL. For opencagenow, it must be an address OR a WGS 84 coordinate. For geoclientnow, it is a NYC location, BIN, BBL, street name or street code. |
| &nbsp;`<index-file>`&nbsp; | The alternate geonames index file to use. It must be a .rkyv file. For convenience, if this is set to a published population floor (15000 or 1000), it will download that prebuilt English-only Geonames index rkyv file from the qsv GitHub repo for the current qsv version and use it. Only used by the index-load subcommand. |

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
| &nbsp;`‑‑admin1`&nbsp; | string | The comma-delimited, case-insensitive list of admin1s to filter for. Only a place inside one of them is returned. A value with no match inside them is left unchanged (or set to the invalid-result value), and the number of such values is reported on stderr. |  |

<a name="reverse-only-option"></a>

## Reverse Only Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑k,`<br>`‑‑k_weight`&nbsp; | string | Use population-weighted distance for reverse subcommand. (i.e. nearest.distance - k * city.population) Larger values will favor more populated cities. If not set (default), the population is not used and the nearest city is returned. |  |

<a name="opencage-&-geoclient-options"></a>

## Opencage & Geoclient Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑api‑key`&nbsp; | string | The API key for the opencage and geoclient subcommands. If set, it takes precedence over the QSV_OPENCAGE_API_KEY (opencage) or QSV_GEOCLIENT_API_KEY (geoclient) environment variable. Get a free OpenCage key at <https://opencagedata.com/users/sign_up>, or a Geoclient subscription key at <https://api-portal.nyc.gov>. |  |
| &nbsp;`‑‑rate‑limit`&nbsp; | integer | Maximum number of API requests per second. Defaults to 1 for opencage (its free tier allows 1 request/second, 2,500/day) and 40 for geoclient (the public API allows 100/second per IP but only 2,500/minute, so 41.6/second sustained, and 500,000/day). |  |
| &nbsp;`‑‑cache‑ttl`&nbsp; | integer | Time-to-live for the persistent on-disk result cache. A value of 0 disables time-based expiration (entries are cached indefinitely). Use --no-cache to disable caching entirely. | `1209600` |
| &nbsp;`‑‑no‑cache`&nbsp; | flag | Disable the persistent on-disk result cache. Duplicate queries within a run are still de-duplicated (for geoclient, within each --batch). |  |

<a name="opencage-only-options"></a>

## Opencage Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑reverse`&nbsp; | flag | Force reverse geocoding for opencage/opencagenow (treat the query as a "lat, long" WGS-84 coordinate). If not set, forward and reverse mode is auto-detected per row. |  |
| &nbsp;`‑‑no‑annotations`&nbsp; | flag | Omit OpenCage annotations (timezone, currency, etc.) from the result and from %json output. |  |

<a name="geoclient-only-options"></a>

## Geoclient Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑endpoint`&nbsp; | string | The Geoclient lookup: search (default), bin, bbl, normalize, streetcode, address, addresspoint, intersection, place or blockface. See GEOCLIENT above. |  |
| &nbsp;`‑‑params`&nbsp; | string | Comma-separated Geoclient parameters for the structured endpoints, each "name={Column}" or "name=value", e.g. "houseNumber={HouseNo},street={Street},borough={Boro}". For search, only fixed options are allowed, e.g. "exactMatchMaxLevel=5". |  |

<a name="dynamic-formatting-options"></a>

## Dynamic Formatting Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑l,`<br>`‑‑language`&nbsp; | string | The language to use when geocoding. The language is specified as a ISO 639-1 code. Note that the Geonames index must have been built with the specified language using the `index-update` subcommand with the --languages option. If the language is not available, the first language in the index is used. | `en` |
| &nbsp;`‑‑invalid‑result`&nbsp; | string | The string to return when the geocode result is empty/invalid. If not set, the original value is used. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. For geoclient, it is the number of requests in flight at once. |  |
| &nbsp;`‑b,`<br>`‑‑batch`&nbsp; | integer | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |
| &nbsp;`‑‑timeout`&nbsp; | integer | Timeout for downloading Geonames cities index. Also applies to each Geonames source file downloaded when rebuilding the index with index-update, and to the prebuilt index downloaded by index-load and index-reset - raise it on slow connections. For opencage and geoclient, it is the timeout for each API request. | `120` |
| &nbsp;`‑‑cache‑dir`&nbsp; | string | The directory to use for caching the Geonames cities index and the persistent on-disk OpenCage and Geoclient result caches. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. The index file is named for the qsv release, so each upgrade downloads a new one. When it does, qsv deletes the index files of all but the 5 most recent releases found in this directory. | `~/.qsv-cache` |

<a name="cache-prune-only-option"></a>

## Cache-Prune Only Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑older‑than`&nbsp; | string | Delete OpenCage and Geoclient cache entries older than this value. Accepts an absolute date/datetime (e.g. 2025-01-31) or a relative age with a unit suffix (s/m/h/d/w = seconds, minutes, hours, days or weeks; e.g. 30d, 2w, 48h). Required by the cache-prune subcommand. |  |

<a name="index-update-only-options"></a>

## Index-Update Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑languages`&nbsp; | string | The comma-delimited, case-insensitive list of languages to use when building the Geonames cities index. The languages are specified as Geonames language codes - the ISO 639-1 2-letter code where one exists (e.g. en,fr,de,es), else the ISO 639-2/3 code. See <https://download.geonames.org/export/dump/iso-languagecodes.txt> to look up codes and <https://download.geonames.org/export/dump/alternatenames/> for the supported language files. 253 languages are currently supported. If not set, the languages the current index was built with are used (en if there is no current index). Setting it to a different list than the current index's forces a rebuild. |  |
| &nbsp;`‑‑cities‑url`&nbsp; | string | The URL to download the Geonames cities file from. There are several available at <https://download.geonames.org/export/dump/>. cities500.zip   - cities with populations > 500; ~200k cities, 56mb cities1000.zip  - population > 1000; ~140k cities, 44mb cities5000.zip  - population > 5000; ~53k cities, 21mb cities15000.zip - population > 15000; ~26k cities, 13mb Note that the more cities are included, the larger the local index file will be, lookup times will be slower, and the search results will be different. For convenience, if this is set to 500, 1000, 5000 or 15000, it will be converted to a geonames cities URL. If not set, the cities file the current index was built from is used (cities15000.zip if there is no current index). Setting it to a different URL than the current index's forces a rebuild. |  |
| &nbsp;`‑‑force`&nbsp; | flag | Force update the Geonames cities index. If not set, qsv will check if there are updates available at Geonames.org before updating the index. With --force, the index is built even if there is no current index. |  |

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
