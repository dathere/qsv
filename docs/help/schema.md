# schema

📇😣🏎️👆🪄🐻‍❄️

> Infer either a JSON Schema Validation Draft 2020-12 (Example) or Polars Schema (Example) from CSV data. In JSON Schema Validation mode, it produces a `.schema.json` file replete with inferred data type & domain/range validation rules derived from `stats`. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. With the `--polars` option, it produces a `.pschema.json` file that all polars commands (`sqlp`, `joinp` & `pivotp`) use to determine the data type of each column & to optimize performance. Both schemas are editable and can be fine-tuned. For JSON Schema, to refine the inferred validation rules. For Polars Schema, to change the inferred Polars data types.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/schema.rs](https://github.com/dathere/qsv/blob/master/src/cmd/schema.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Schema Options](#schema-options) | [Common Options](#common-options)

## Description [↩](#nav)

Generate JSON Schema or Polars Schema (with the `--polars` option) from CSV data.

### Json Schema Validation:

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


## Usage [↩](#nav)

```console
qsv schema [options] [<input>]
qsv schema --help
```

## Schema Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--enum-threshold` | string | Cardinality threshold for adding enum constraints. Enum constraints are compiled for String & Integer types. | `50` |
| `-i, --ignore-case` | flag | Ignore case when compiling unique values for enum constraints. Do note however that the `validate` command is case-sensitive when validating against enum constraints. |  |
| `--strict-dates` | flag | Enforce Internet Datetime format (RFC-3339) for detected date/datetime columns. Otherwise, even if columns are inferred as date/datetime, they are set to type "string" in the schema instead of "date" or "date-time". |  |
| `--strict-formats` | flag | Enforce JSON Schema format constraints for detected email, hostname, and IP address columns. When enabled, String fields are checked against email, hostname, IPv4, and IPv6 formats. Format constraints are only added if ALL unique values in the field match the detected format. |  |
| `--pattern-columns` | string | Select columns to derive regex pattern constraints. That is, this will create a regular expression that matches all values for each specified column. Columns are selected using `select` syntax (see `qsv select --help` for details). |  |
| `--dates-whitelist` | string | The case-insensitive patterns to look for when shortlisting fields for date inference. i.e. if the field's name has any of these patterns, it is shortlisted for date inferencing. Set to "all" to inspect ALL fields for date/datetime types. | `date,time,due,open,close,created` |
| `--prefer-dmy` | flag | Prefer to parse dates in dmy format. Otherwise, use mdy format. |  |
| `--force` | flag | Force recomputing cardinality and unique values even if stats cache file exists and is current. |  |
| `--stdout` | flag | Send generated JSON schema file to stdout instead. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| `-o, --output` | string | Write output to <file> instead of using the default filename. For JSON Schema, the default is <input>.schema.json. For Polars schema, the default is <input>.pschema.json. |  |
| `--polars` | flag | Infer a Polars schema instead of a JSON Schema. This option is only available if the `polars` feature is enabled. The generated Polars schema will be written to a file with the `.pschema.json` suffix appended to the input filename. |  |

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. Namely, it will be processed with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/schema.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/schema.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
