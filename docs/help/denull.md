# denull

> Detect null sentinels — literal text like `NULL` or `N/A` standing in for a missing value, which makes `stats` type a numeric column as String (its `nullcount` stays 0, no quartiles are computed) and silently degrades `viz`, `schema` & `describegpt` downstream. Reports by default; `--apply` blanks the sentinels in the columns it confirmed, per column. Scans once with bounded memory. Numeric sentinels (`-999`) are deliberately NOT detected — they parse as valid numbers and no scan can tell them from real data.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/denull.rs](https://github.com/dathere/qsv/blob/master/src/cmd/denull.rs)**

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Denull Options](#denull-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Detect null sentinels - literal text like "NULL" or "N/A" standing in for a missing
value - that stop a numeric column from being recognized as numeric.

A cell holding the text "NULL" is a VALUE, not a null. `qsv stats` therefore types
the whole column as String, its nullcount stays 0, and no quartiles are computed.
Everything downstream degrades quietly: `viz smart` drops the column, `schema`
declares it a string, and `describegpt` describes a category that isn't one.

denull scans each column ONCE, with bounded memory, and partitions its values into
those that parse as a finite number and those that don't. A column is CONFIRMED when
every non-numeric value it holds is a known null sentinel and at least two distinct
numeric values remain.

A column is REJECTED - with the reason - when it cannot be promoted anyway: another
value is not a sentinel ("OK"), its numbers carry leading zeros and are really codes
("007"), or it buries the sentinel under more than --max-distinct other non-numeric
values.

Only columns worth acting on are listed: those holding a known sentinel, and those
that are predominantly numeric, whose few odd values are candidates for a sentinel
denull does not know yet - name them with --add-vocab. An ordinary categorical is
not a near miss and is not reported; nor is a free-text column that merely happens
to be unpromotable. Use --all-columns to see everything scanned.

The scan is exhaustive, not sampled: a column is never confirmed on the strength of
the values that happen to sort first. A genuine free-text column disqualifies itself
as soon as it accumulates --max-distinct different non-numeric values, so memory
stays flat. A 434 MB, 86-column file peaks at ~40 MB - the same as a type-inference
pass, and ~19x less than an exhaustive frequency table of every distinct value.

By default denull only REPORTS; it never rewrites your data. Pass --apply to
rewrite it, blanking sentinels ONLY in the columns denull CONFIRMED. A column it
REJECTED is copied through untouched, as is every column it did not scan:  

```console
$ qsv denull --apply data.csv -o clean.csv
```

```console
$ qsv stats clean.csv --everything
```


Cleaning is per-column, which is what a single `qsv replace` pass cannot do: it
takes one regex across all selected columns, so it cannot blank "NULL" in one
column and "-" in another while leaving a literal "-" alone in a third.

Numeric sentinels (-999, -9999, 9999) are deliberately NOT detected. They parse as
valid numbers, so no scan can distinguish them from real data - a depth-to-water
reading of -140 ft is an artesian well, not a missing value. Only a human or a
domain-aware model can propose those, and only a human should apply them.

The `sentinels` column lists the sentinel tokens OBSERVED in that column. They are
only safe to remove when the verdict is `confirmed`.


<a name="examples"></a>

## Examples [↩](#nav)

Report every column holding a null sentinel:  
```console
qsv denull data.csv
```

Restrict to a few columns, and emit JSON for a script to consume:  
```console
qsv denull -s HoleDepth,WellDepth,CasingDepth --json data.csv
```

Treat the site-specific "no reading" marker as a sentinel too:  
```console
qsv denull --add-vocab "no reading,not recorded" data.csv
```

Show every scanned column, including those with nothing to report:  
```console
qsv denull --all-columns data.csv
```

Blank the sentinels in every confirmed column; the report goes to stderr:  
```console
qsv denull --apply data.csv -o clean.csv
```

For the tests, see <https://github.com/dathere/qsv/blob/master/tests/test_denull.rs>.

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv denull [options] [<input>]
qsv denull --help
```

<a name="denull-options"></a>

## Denull Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑s,`<br>`‑‑select`&nbsp; | string | Select the columns to scan. See `qsv select --help` for the full selection syntax. |  |
| &nbsp;`‑‑vocab`&nbsp; | string | Comma-separated null sentinel vocabulary, REPLACING the built-in list. Matched case-insensitively after trimming surrounding whitespace. |  |
| &nbsp;`‑‑add‑vocab`&nbsp; | string | Comma-separated tokens to ADD to the built-in list. Use this for site-specific markers. |  |
| &nbsp;`‑‑max‑distinct`&nbsp; | integer | Abandon a column once it holds this many distinct non-numeric values. Guards memory on free-text columns and bounds the report. | `16` |
| &nbsp;`‑‑all‑columns`&nbsp; | flag | Also report columns with nothing to flag. By default only columns with a verdict are listed. |  |
| &nbsp;`‑‑apply`&nbsp; | flag | Rewrite the data instead of only reporting it. Blanks the sentinels in every CONFIRMED column and writes the CSV to <output> (or stdout), sending the report to stderr. Rejected and unscanned columns pass through untouched. Needs a file input, and <output> must not be the input file. |  |
| &nbsp;`‑‑json`&nbsp; | flag | Emit the report as a JSON array instead of CSV. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write the report here instead of stdout. |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will NOT be interpreted as column names. Columns are named col_1, col_2, etc. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/denull.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/denull.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
