# schema

> Infer either a [JSON Schema Validation Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-validation) ([Example](https://github.com/dathere/qsv/blob/master/resources/test/311_Service_Requests_from_2010_to_Present-2022-03-04.csv.schema.json)) or [Polars Schema](https://docs.pola.rs/user-guide/lazy/schemas/) ([Example](https://github.com/dathere/qsv/blob/master/resources/test/NYC_311_SR_2010-2020-sample-1M.pschema.json)) from CSV data. In JSON Schema Validation mode, it produces a `.schema.json` file replete with inferred data type & domain/range validation rules derived from [`stats`](../../README.md#stats_deeplink). Uses multithreading to go faster if an index is present. See [`validate`](../../README.md#validate_deeplink) command to use the generated JSON Schema to validate if similar CSVs comply with the schema. With the `--polars` option, it produces a `.pschema.json` file that all polars commands (`sqlp`, `joinp` & `pivotp`) use to determine the data type of each column & to optimize performance. Both schemas are editable and can be fine-tuned. For JSON Schema, to refine the inferred validation rules. For Polars Schema, to change the inferred Polars data types.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/schema.rs](https://github.com/dathere/qsv/blob/master/src/cmd/schema.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="uses additional memory proportional to the cardinality of the columns in the CSV.">😣</abbr><abbr title="multithreaded and/or faster when an index (📇) is available.">🏎️</abbr><abbr title="has powerful column selector support. See `select` for syntax.">👆</abbr><abbr title="&quot;automagical&quot; commands that uses stats and/or frequency tables to work &quot;smarter&quot; & &quot;faster&quot;.">🪄</abbr><abbr title="command powered/accelerated by  vectorized query engine.">🐻‍❄️</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Schema Options](#schema-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generate JSON Schema or Polars Schema (with the `--polars` option) from CSV data.

### JSON Schema Validation:

This command derives a JSON Schema Validation (Draft 2020-12) file from CSV data,
including validation rules based on data type and input data domain/range.
<https://json-schema.org/draft/2020-12/json-schema-validation.html>

Running `validate` command on original input CSV with generated schema
should not flag any invalid records.

The intended workflow is to use the `schema` command to generate a JSON schema file
from representative CSV data, fine-tune the JSON schema file as needed, and then use
the `validate` command to validate other CSV data with the same structure using the
generated JSON schema.

After manually fine-tuning the JSON schema file, note that you can also use the
`validate` command to validate the JSON Schema file as well:

```console
$ qsv validate schema manually-tuned-jsonschema.json
```


The generated JSON schema file has `.schema.json` suffix appended. For example,
for input `mydata.csv`, the generated JSON schema is `mydata.csv.schema.json`.

If piped from stdin, the schema file will be `stdin.csv.schema.json` and
a `stdin.csv` file will be created with stdin's contents as well.

Note that `stdin.csv` will be overwritten if it already exists.

Schema generation can be a compute-intensive process, especially for large CSV files.
To speed up generation, the `schema` command will reuse a `stats.csv.data.jsonl` file if it
exists and is current (i.e. stats generated with --cardinality and --infer-dates options).
Otherwise, it will run the `stats` command to generate the `stats.csv.data.jsonl` file first,
and then use that to generate the schema file.

### Polars Schema:

When the "polars" feature is enabled, the `--polars` option will generate a Polars schema
instead of a JSON Schema. The generated Polars schema will be written to a file with the
`.pschema.json` suffix appended to the input file stem.

The Polars schema is a JSON object that describes the schema of a CSV file. When present,
the `sqlp`, `joinp`, and `pivotp` commands will use the Polars schema to read the CSV file
instead of inferring the schema from the CSV data. Not only does this allow these commands to
skip schema inferencing which may fail when the inferencing sample is too low, it also allows
Polars to optimize the query and gives the user the option to tailor the schema to their specific
query needs (e.g. using a Decimal type with explicit precision and scale instead of a Float type).

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_schema.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv schema [options] [<input>]
qsv schema --help
```

<a name="schema-options"></a>

## Schema Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--enum-threshold`&nbsp; | string | Cardinality threshold for adding enum constraints. Enum constraints are compiled for String & Integer types. | `50` |
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Ignore case when compiling unique values for enum constraints. Do note however that the `validate` command is case-sensitive when validating against enum constraints. |  |
| &nbsp;`--strict-dates`&nbsp; | flag | Enforce Internet Datetime format (RFC-3339) for detected date/datetime columns. Otherwise, even if columns are inferred as date/datetime, they are set to type "string" in the schema instead of "date" or "date-time". |  |
| &nbsp;`--strict-formats`&nbsp; | flag | Enforce JSON Schema format constraints for detected email, hostname, and IP address columns. When enabled, String fields are checked against email, hostname, IPv4, and IPv6 formats. Format constraints are only added if ALL unique values in the field match the detected format. |  |
| &nbsp;`--pattern-columns`&nbsp; | string | Select columns to derive regex pattern constraints. That is, this will create a regular expression that matches all values for each specified column. Columns are selected using `select` syntax (see `qsv select --help` for details). |  |
| &nbsp;`--dates-whitelist`&nbsp; | string | The case-insensitive patterns to look for when shortlisting fields for date inference. i.e. if the field's name has any of these patterns, it is shortlisted for date inferencing. Set to "all" to inspect ALL fields for date/datetime types. | `date,time,due,open,close,created` |
| &nbsp;`--prefer-dmy`&nbsp; | flag | Prefer to parse dates in dmy format. Otherwise, use mdy format. |  |
| &nbsp;`--force`&nbsp; | flag | Force recomputing cardinality and unique values even if stats cache file exists and is current. |  |
| &nbsp;`--stdout`&nbsp; | flag | Send generated JSON schema file to stdout instead. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of using the default filename. For JSON Schema, the default is <input>.schema.json. For Polars schema, the default is <input>.pschema.json. |  |
| &nbsp;`--polars`&nbsp; | flag | Infer a Polars schema instead of a JSON Schema. This option is only available if the `polars` feature is enabled. The generated Polars schema will be written to a file with the `.pschema.json` suffix appended to the input filename. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Namely, it will be processed with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/schema.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/schema.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
