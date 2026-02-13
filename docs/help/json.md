# json

> Convert JSON array to CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/json.rs](https://github.com/dathere/qsv/blob/master/src/cmd/json.rs)** | 👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [JSON Options](#json-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Convert JSON to CSV.

The JSON data is expected to be non-empty and non-nested as either:

1. An array of objects where:
A. All objects are non-empty, have non-empty and unique keys, and the same keys are in each object.
B. Values are not objects or arrays.
2. An object where values are not objects or arrays and the object is as described above.

Objects with duplicate keys are not recommended as only one key and its values may be used.

If your JSON data is not in the expected format and/or is nested or complex, try using
the --jaq option to pass a jq-like filter before parsing with the above constraints.
Learn more about jaqhere: <https://github.com/01mf02/jaq>

As an example, say we have the following JSON data in a file fruits.json:

[
{
"fruit": "apple",
"price": 2.50,
"calories": 95
},
{
"fruit": "banana",
"price": 3.00,
"calories": 105
}
]

To convert it to CSV format run:

```console
$ qsv json fruits.json
```


And the following is printed to the terminal:

fruit,price,calories
apple,2.5,95
banana,3.0,105

IMPORTANT:
The order of the columns in the CSV file will be the same as the order of the keys in the first JSON object.
The order of the rows in the CSV file will be the same as the order of the objects in the JSON array.

Additional keys not present in the first JSON object will be appended as additional columns in the
output CSV in the order they appear.

For example, say we have the following JSON data in a file fruits2.json:

[
{
"fruit": "apple",
"cost": 1.75,
"price": 2.50,
"calories": 95
},
{
"fruit": "mangosteen",
"price": 5.00,
"calories": 56
},
{
"fruit": "starapple",
"rating": 9,
"price": 4.50,
"calories": 95,
},
{
"fruit": "banana",
"price": 3.00,
"calories": 105
}
]

If we run the following command:

```console
$ qsv json fruits2.json | qsv table
```


The output CSV will have the following columns:

fruit       cost  price  calories  rating
apple       1.75  2.5    95
mangosteen        5.0    56
starapple         4.5    95        9
banana            3.0    105

Note that the "rating" column is added as an additional column in the output CSV,
though it appears as the 2nd column in the third JSON object for "starapple".

If you want to select/reorder/drop columns in the output CSV, use the --select option, for example:

```console
$ qsv json fruits.json --select price,fruit
```


The following is printed to the terminal:

price,fruit
2.5,apple
3.0,banana

Note: Trailing zeroes in decimal numbers after the decimal are truncated (2.50 becomes 2.5).

If the JSON data was provided using stdin then either use - or do not provide a file path.
For example you may copy the JSON data above to your clipboard then run:

```console
$ qsv clipboard | qsv json
```


Again, when JSON data is nested or complex, try using the --jaq option and provide a filter value.

For example we have a .json file with a "data" key and the value being the same array as before:

{
"data": [...]
}

We may run the following to select the JSON file and convert the nested array to CSV:

```console
$ qsv prompt -F json | qsv json --jaq .data
```


For more examples, see <https://github.com/dathere/qsv/blob/master/tests/test_json.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv json [options] [<input>]
qsv json --help
```

<a name="json-options"></a>

## JSON Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--jaq`&nbsp; | string | Filter JSON data using jaq syntax (<https://github.com/01mf02/jaq>), which is identical to the popular JSON command-line tool - jq. <https://jqlang.github.io/jq/> Note that the filter is applied BEFORE converting JSON to CSV |  |
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select, reorder or drop columns for output. Otherwise, all the columns will be output in the same order as the first object's keys in the JSON data. See 'qsv select --help' for the full syntax. Note however that <cols> NEED to be a comma-delimited list of column NAMES and NOT column INDICES. | `1-` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/json.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/json.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
