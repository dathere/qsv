# datefmt

> Formats recognized date fields (19 formats recognized) to a specified date format using strftime date format specifiers.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/datefmt.rs](https://github.com/dathere/qsv/blob/master/src/cmd/datefmt.rs)** | 📇🚀👆

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Datefmt Options](#datefmt-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Formats recognized date fields (19 formats recognized) to a specified date format
using strftime date format specifiers.

For recognized date formats, see
<https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats>

See <https://docs.rs/chrono/latest/chrono/format/strftime/> for
accepted date format specifiers for --formatstr.
Defaults to ISO 8601/RFC 3339 format when --formatstr is not specified.
( "%Y-%m-%dT%H:%M:%S%z" - e.g. 2001-07-08T00:34:60.026490+09:30 )


<a name="examples"></a>

## Examples [↩](#nav)

> Format dates in Open Date column to ISO 8601/RFC 3339 format:

```console
qsv datefmt 'Open Date' file.csv
```

> Format multiple date columns in file.csv to ISO 8601/RFC 3339 format:

```console
qsv datefmt 'Open Date,Modified Date,Closed Date' file.csv
```

> Format all columns that end with "_date" case-insensitive in file.csv to ISO 8601/RFC 3339 format:

```console
qsv datefmt '/(?i)_date$/' file.csv
```

> Format dates in OpenDate column using '%Y-%m-%d' format:

```console
qsv datefmt OpenDate --formatstr '%Y-%m-%d' file.csv
```

> Format multiple date columns using '%Y-%m-%d' format:

```console
qsv datefmt OpenDate,CloseDate,ReopenDate --formatstr '%Y-%m-%d' file.csv
```

> Get the week number for OpenDate and store it in the week_number column:

```console
qsv datefmt OpenDate --formatstr '%V' --new-column week_number file.csv
```

> Get the day of the week for several date columns and store it in the corresponding weekday columns:

```console
qsv datefmt OpenDate,CloseDate --formatstr '%u' --rename Open_weekday,Close_weekday file.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_datefmt.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv datefmt [--formatstr=<string>] [options] <column> [<input>]
qsv datefmt --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<column>`&nbsp; | The column/s to apply the date formats to. Note that the <column> argument supports multiple columns. See 'qsv select --help' for the format details. |
| &nbsp;`<input>`&nbsp; | The input file to read from. If not specified, reads from stdin. |

<a name="datefmt-options"></a>

## Datefmt Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-c,`<br>`--new-column`&nbsp; | string | Put the transformed values in a new column instead. |  |
| &nbsp;`-r,`<br>`--rename`&nbsp; | string | New name for the transformed column. |  |
| &nbsp;`--prefer-dmy`&nbsp; | flag | Prefer to parse dates in dmy format. Otherwise, use mdy format. |  |
| &nbsp;`--keep-zero-time`&nbsp; | flag | If a formatted date ends with "T00:00:00+00:00", keep the time instead of removing it. |  |
| &nbsp;`--input-tz=<string>`&nbsp; | string | The timezone to use for the input date if the date does not have timezone specified. The timezone must be a valid IANA timezone name or the string "local" for the local timezone. See <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones> for a list of valid timezone names. | `UTC` |
| &nbsp;`--output-tz=<string>`&nbsp; | string | The timezone to use for the output date. The timezone must be a valid IANA timezone name or the string "local". | `UTC` |
| &nbsp;`--default-tz=<string>`&nbsp; | string | The timezone to use for BOTH input and output dates when they do have timezone. Shortcut for --input-tz and --output-tz set to the same timezone. The timezone must be a valid IANA timezone name or the string "local". |  |
| &nbsp;`--utc`&nbsp; | flag | Shortcut for --input-tz and --output-tz set to UTC. |  |
| &nbsp;`--zulu`&nbsp; | flag | Shortcut for --output-tz set to UTC and --formatstr set to "%Y-%m-%dT%H:%M:%SZ". |  |
| &nbsp;`-R,`<br>`--ts-resolution`&nbsp; | string | The resolution to use when parsing Unix timestamps. Valid values are "sec", "milli", "micro", "nano". | `sec` |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Automatically determined for CSV files with more than 50000 rows. Set to 0 to load all rows in one batch. Set to 1 to force batch optimization even for files with less than 50000 rows. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. |  |

---
**Source:** [`src/cmd/datefmt.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/datefmt.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
