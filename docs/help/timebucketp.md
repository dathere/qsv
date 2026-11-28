# timebucketp

> Bucket time-series data into time windows (e.g. 15-minute, overlapping, or weekly from Monday) and aggregate them, optionally per group. Also computes rolling look-back windows (e.g. a trailing 2-hour average at each row) and can fill empty windows. Complements `sqlp`'s `DATE_TRUNC`, which only buckets by calendar unit.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/timebucketp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/timebucketp.rs)** | [🤯](TableOfContents.md#legend "loads entire CSV into memory, though `dedup`, `stats` & `transpose` have \"streaming\" modes as well.")[🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")[🚀](TableOfContents.md#legend "multithreaded even without an index.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Timebucketp Options](#timebucketp-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Buckets time-series CSV data into time windows using the Polars engine.

Groups rows into windows over a date, datetime or integer index column and
aggregates them. Unlike sqlp's DATE_TRUNC, windows need not be calendar units
(e.g. 15m), can overlap (a --period longer than --every) and can be shifted
(--offset).

WINDOW MODE (default):  
A new window starts every --every and spans --period (default: --every).
Each output row is one window. Windows with no input rows are skipped
unless you set --fill-gaps.

ROLLING MODE (--rolling):  
One window per input row, looking back --period from that row's index
value, e.g. "the average over the preceding 2 hours". By default a row's
window is (index - period, index], so it includes the row itself. The
output has one row per input row.

DURATIONS are an integer followed by a unit, and units combine (e.g. 1h30m):  
ns, us, ms, s, m (minute), h, d, w, mo (calendar month), q (quarter),
y (year), i (index units, for integer index columns only: 3i spans 3).
d, w, mo, q and y are calendar-aware: they respect month lengths.
A leading minus makes a duration negative, e.g. --offset -30m.

The index column is parsed as dates or datetimes. Like the rest of qsv,
ambiguous dates such as 01/03/2024 are read month-first (January 3). Set
QSV_PREFER_DMY to read them day-first, or give the exact format with
the --date-format option. A text column whose every value is a date is
parsed the same way for min, max, mean, median and quantile, so it compares
as dates; first, last and --by keep values as written (unless a pschema.json
types them). Dates need 4-digit years unless --date-format is given. An index
that does not parse as dates is an error, as is an empty index value. An
index that mixes dates and datetimes, as `qsv datefmt` writes midnight
values, is read as datetimes, its dates as midnight.

A pschema.json made by `qsv schema --polars` while QSV_PREFER_DMY was set
marks day/month dates as already parsed, so they stay day-first in later
runs even with QSV_PREFER_DMY unset. After changing QSV_PREFER_DMY, delete
or regenerate the pschema.json. --date-format overrides it for the index
column only.


<a name="examples"></a>

## Examples [↩](#nav)

> hourly sales totals

```console
qsv timebucketp --index ts --every 1h --agg sum:sales sales.csv
```

> 1-hour windows starting every 15 minutes, computed per station

```console
qsv timebucketp -i ts --every 15m --period 1h --by station --agg mean:temp,max:temp readings.csv
```

> weekly row counts, weeks starting on Monday, with empty weeks as 0

```console
qsv timebucketp -i date --every 1w --start-by monday --fill-gaps events.csv
```

> trailing 2-hour average price at each trade

```console
qsv timebucketp -i ts --rolling --period 2h --agg mean:price trades.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_timebucketp.rs).

See also <https://github.com/dathere/qsv/wiki/SQL-and-Polars#timebucketp>

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv timebucketp [options] <input>
qsv timebucketp --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The input CSV file. The file must have headers. If the file has a pschema.json file, it is used to type the columns unless --infer-len is set to a value other than the default. Stdin is not supported. A snappy-compressed (.sz) input is decompressed first, and an --output ending in .sz is snappy-compressed. Lines starting with QSV_COMMENT_CHAR are skipped. QSV_POLARS_FLOAT_PRECISION and QSV_OUTPUT_BOM apply to the output, as in sqlp and pivotp. |

<a name="timebucketp-options"></a>

## Timebucketp Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑i,`<br>`‑‑index`&nbsp; | string | The date, datetime or integer column that defines the windows. Required. The output keeps its name. |  |
| &nbsp;`‑‑every`&nbsp; | string | Start a new window at this interval. Required in window mode. |  |
| &nbsp;`‑‑period`&nbsp; | string | The length of each window. Defaults to --every in window mode. Required in rolling mode. |  |
| &nbsp;`‑‑offset`&nbsp; | string | Shift the window boundaries by this duration. Defaults to 0 in window mode, and to minus --period in rolling mode (a look-back window). |  |
| &nbsp;`‑‑date‑format`&nbsp; | string | The strptime format of a text --index column, e.g. %m/%d/%Y or "%d.%m.%Y %H:%M". Without time fields the index is read as dates, otherwise as datetimes. |  |
| &nbsp;`‑b,`<br>`‑‑by`&nbsp; | string | Also group by these columns (comma-separated). Windows are computed separately for each group. |  |
| &nbsp;`‑a,`<br>`‑‑agg`&nbsp; | string | Comma-separated aggregations, as func:column pairs, e.g. sum:sales,mean:temp. The functions are: first, last, sum, min, max, mean, median, quantile@<p> (alias q@<p>, p in [0, 1]), len. Each is output as <column>_<func>, e.g. sales_sum, or temp_quantile_0.95 for quantile@0.95. len takes no column: it counts the rows in each window and is output as "count". | `len` |
| &nbsp;`‑‑label`&nbsp; | string | The value written to the index column for each window. Window mode only. One of: left      - the window's start (the default) right     - the window's end datapoint - the window's first index value |  |
| &nbsp;`‑‑closed`&nbsp; | string | Which window boundaries are inclusive: left, right, both or none. Defaults to left in window mode and to right in rolling mode. |  |
| &nbsp;`‑‑start‑by`&nbsp; | string | Where the first window starts. Window mode only. |  |
| &nbsp;`‑‑boundaries`&nbsp; | flag | Add _lower_boundary and _upper_boundary columns before the index column. Window mode only. |  |
| &nbsp;`‑‑fill‑gaps`&nbsp; | flag | Also emit the empty windows between each group's first and last window: count, sum and len are 0 there, and every other aggregation is empty. Window mode only. Requires the default left labels, and cannot be combined with --start-by datapoint or --boundaries. |  |
| &nbsp;`‑‑rolling`&nbsp; | flag | Use rolling mode (see above). |  |
| &nbsp;`‑‑sorted`&nbsp; | flag | The input is already sorted by --index, within each group when using --by, so skip timebucketp's sort. Unsorted input is then an error, never a wrong result. |  |
| &nbsp;`‑‑infer‑len`&nbsp; | integer | Number of rows to scan when inferring the schema. Set to 0 to scan the entire file. | `10000` |
| &nbsp;`‑‑decimal‑comma`&nbsp; | flag | Use comma as decimal separator when READING the input. Note that you will need to specify an alternate --delimiter. |  |
| &nbsp;`‑‑ignore‑errors`&nbsp; | flag | Read CSV values that can't be parsed as their column's type as empty (null) instead of failing. Their rows are kept. This does not relax the index: its values must still parse as dates (or integers), with none empty. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading/writing CSV data. Must be a single character. If not set, QSV_DEFAULT_DELIMITER is used, else ','. A .tsv/.tab (tab) or .ssv (semicolon) extension on the input or --output file sets that file's delimiter. (default: ,) |  |
| &nbsp;`‑q,`<br>`‑‑quiet`&nbsp; | flag | Do not print the result's shape to stderr. |  |

---
**Source:** [`src/cmd/timebucketp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/timebucketp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
