# template

> Renders a template using CSV data with the Mini Jinja template engine (Example).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/template.rs](https://github.com/dathere/qsv/blob/master/src/cmd/template.rs)** | 📇🚀🔣📚⛩️![CKAN](../images/ckan.png)

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Template Options](#template-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Renders a template using CSV data with the MiniJinja template engine.
<https://docs.rs/minijinja/latest/minijinja/>

This command processes each row of the CSV file, making the column values available as variables.
Each row is rendered using the template. Column headers become variable names, with non-alphanumeric
characters converted to underscore (_).

Templates use Jinja2 syntax (<https://jinja.palletsprojects.com/en/stable/templates/>)
and can access an extensive library of built-in filters/functions, with additional ones
from minijinja_contrib <https://docs.rs/minijinja-contrib/latest/minijinja_contrib/>.
Additional qsv custom filters are also documented at the end of this file.

If the <outdir> argument is specified, it will create a file for each row in <outdir>, with
the filename rendered using --outfilename option.
Otherwise, ALL the rendered rows will be sent to STDOUT or the designated --output.

Example:
data.csv:
"first name","last name",balance,"loyalty points",active,us_state
alice,jones,100.50,1000,true,TX
bob,smith,200.75,2000,false,CA
john,doe,10,1,true,NJ

NOTE: All variables are of type String and will need to be cast with the `|float` or `|int`
filters for math operations and when a MiniJinja filter/function requires it.
qsv's custom filters (substr, format_float, human_count, human_float_count, round_banker &
str_to_bool) do not require casting for convenience.

template.tpl
{% set us_state_lookup_loaded = register_lookup("us_states", "dathere://us-states-example.csv") -%}
Dear {{ first_name|title }} {{ last_name|title }}!
Your account balance is {{ balance|format_float(2) }}
with {{ loyalty_points|human_count }} point{{ loyalty_points|int|pluralize }}!
{# This is a comment and will not be rendered. The closing minus sign in this
block tells MiniJinja to trim whitespaces -#}
{% if us_state_lookup_loaded -%}
{% if us_state not in ["DE", "CA"] -%}
{% set tax_rate = us_state|lookup("us_states", "Sales Tax (2023)")|float -%}
State: {{ us_state|lookup("us_states", "Name") }} {{us_state}} Tax Rate: {{ tax_rate }}%
{% set loyalty_value = loyalty_points|int / 100 -%}
{%- set tax_amount = loyalty_value * (tax_rate / 100) -%}
{%- set loyalty_value = loyalty_value - tax_amount -%}
Value of Points: {{ loyalty_value }}
{% else %}
{% set loyalty_value = 0 -%}
{% endif %}
Final Balance: {{ (balance|int - loyalty_value)|format_float(2) }}
{% endif %}
Status: {% if active|to_bool %}Active{% else %}Inactive{% endif %}

```console
$ qsv template --template-file template.tpl data.csv
```


For more examples, see <https://github.com/dathere/qsv/blob/master/tests/test_template.rs>.
For a relatively complex MiniJinja template, see <https://github.com/dathere/qsv/blob/master/scripts/template.tpl>


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv template [options] [--template <str> | --template-file <file>] [<input>] [<outdir> | --output <file>]
qsv template --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The CSV file to read. If not given, input is read from STDIN. |
| &nbsp;`<outdir>`&nbsp; | The directory where the output files will be written. If it does not exist, it will be created. If not set, output will be sent to stdout or the specified --output. When writing to <outdir>, files are organized into subdirectories of --outsubdir-size (default: 1000) files each to avoid filesystem navigation & performance issues. |

<a name="template-options"></a>

## Template Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--template`&nbsp; | string | MiniJinja template string to use (alternative to --template-file) |  |
| &nbsp;`-t,`<br>`--template-file`&nbsp; | string | MiniJinja template file to use |  |
| &nbsp;`-j,`<br>`--globals-json`&nbsp; | string | A JSON file containing global variables to make available in templates. The JSON properties can be accessed in templates using the "qsv_g" namespace (e.g. {{qsv_g.school_name}}, {{qsv_g.year}}). This allows sharing common values across all template renders. |  |
| &nbsp;`--outfilename`&nbsp; | string | MiniJinja template string to use to create the filename of the output files to write to <outdir>. If set to just QSV_ROWNO, the filestem is set to the current rowno of the record, padded with leading zeroes, with the ".txt" extension (e.g. 001.txt, 002.txt, etc.) Note that all the fields, including QSV_ROWNO, are available when defining the filename template. | `QSV_ROWNO` |
| &nbsp;`--outsubdir-size`&nbsp; | string | The number of files per subdirectory in <outdir>. | `1000` |
| &nbsp;`--customfilter-error`&nbsp; | string | The value to return when a custom filter returns an error. Use "<empty string>" to return an empty string. | `<FILTER_ERROR>` |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |
| &nbsp;`--timeout`&nbsp; | string | Timeout for downloading lookups on URLs. | `30` |
| &nbsp;`--cache-dir`&nbsp; | string | The directory to use for caching downloaded lookup resources. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. | `~/.qsv-cache` |
| &nbsp;`--ckan-api`&nbsp; | string | The URL of the CKAN API to use for downloading lookup resources with the "ckan://" scheme. If the QSV_CKAN_API envvar is set, it will be used instead. | `https://data.dathere.com/api/3/action` |
| &nbsp;`--ckan-token`&nbsp; | string | The CKAN API token to use. Only required if downloading private resources. If the QSV_CKAN_TOKEN envvar is set, it will be used instead. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Templates must use numeric 1-based indices with the "_c" prefix.(e.g. col1: {{_c1}} col2: {{_c2}}) |  |
| &nbsp;`--delimiter`&nbsp; | string | Field separator for reading CSV | `,` |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. |  |

---
**Source:** [`src/cmd/template.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/template.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
