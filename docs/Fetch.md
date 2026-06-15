# Fetch command

## jaq ##

Fetch is integrated with [`jaq`](https://github.com/01mf02/jaq) (a jq clone), so you can directly parse out
values from an API's JSON response using the jq JSON Query Language.

* Selectors use jq syntax (leading `.`), e.g. `."places"[0]."place name"`.
* When `--jaq` is not used and `--new-column` is not set, the output is a JSONL file - one minified JSON
  response per line, not a CSV.
* When `--new-column` is set, the output is a CSV with the fetched/parsed value in the new column.
* A jaq selector that returns a single array value is serialized as a JSON array string,
  e.g. `[ ."a", ."b" ]` yields `["Beverly Hills","CA"]`. A selector that returns a *stream*
  of multiple values (e.g. `."a", ."b"` without the enclosing brackets) instead joins the
  values with `, `.
* On error, the cell is left blank unless `--store-error` is set, in which case the error message is stored.

## Usage Examples

__data.csv__

```csv
URL
https://api.zippopotam.us/us/90210
https://api.zippopotam.us/us/94105
https://api.zippopotam.us/us/92802
```

### Fetch the JSON response for each row

```
$ qsv fetch URL data.csv
```

The output is a JSONL file - with a minified JSON response per line, not a CSV file.

### Fetch via the `URL` column, apply a jaq selector, and put results into a new column

To generate a CSV with the parsed City and State, use the `--new-column` and `--jaq` options:

```
$ qsv fetch URL --new-column CityState \
    --jaq '[ ."places"[0]."place name",."places"[0]."state abbreviation" ]' \
    data.csv > data_with_CityState.csv
```

__data_with_CityState.csv__

```csv
URL,CityState
https://api.zippopotam.us/us/90210,"[""Beverly Hills"",""CA""]"
https://api.zippopotam.us/us/94105,"[""San Francisco"",""CA""]"
https://api.zippopotam.us/us/92802,"[""Anaheim"",""CA""]"
```

### Load a jaq selector from a file

Entering jaq selectors on the command line is error prone and can quickly become cumbersome.
Alternatively, the jaq selector can be saved and loaded from a file using the `--jaqfile` option:

```
$ qsv fetch URL --new-column CityState --jaqfile places.jaq data.csv > datatest.csv
```

### Store the error message instead of a blank value

On error, instead of a blank value, the error message can be stored via the `--store-error` flag:

```
$ qsv fetch URL data.csv --new-column City --jaq '."places"[0]."place name"' --store-error
```

### Dynamically construct URLs with `--url-template`

Instead of using a column of fully-qualified URLs, you can construct the URL for each CSV row from its
column values. Enclose column names in curly braces; non-alphanumeric characters in field names (spaces,
hyphens) are replaced with `_`.

```
$ qsv fetch \
    --url-template "https://api.geocode.earth/v1/reverse?point.lat={latitude}&point.lon={longitude}" \
    addr_data.csv -c response > enriched_addr_data.csv
```

### Fetch using custom headers (e.g. for an API key)

The `--http-header` (`-H`) option appends arbitrary `key:value` pairs to the HTTP header. Pass it multiple
times for multiple pairs:

```
$ qsv fetch URL data.csv --http-header "X-Api-Key:TEST_KEY" -H "X-Api-Secret:ABC123XYZ" -H "Accept-Language: fr-FR"
```

For more extensive examples, see the [fetch tests](https://github.com/dathere/qsv/blob/master/tests/test_fetch.rs)
and the [HTTP and Web wiki](https://github.com/dathere/qsv/wiki/HTTP-and-Web#fetch).
