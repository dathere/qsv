# search

> Run a regex over a CSV. Applies the regex to selected fields & shows only matching rows.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/search.rs](https://github.com/dathere/qsv/blob/master/src/cmd/search.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="multithreaded and/or faster when an index (📇) is available.">🏎️</abbr><abbr title="has powerful column selector support. See `select` for syntax.">👆</abbr>

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Search Options](#search-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Filters CSV data by whether the given regex matches a row.

The regex is applied to selected field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.

When --quick is enabled, no output is produced and exitcode 0 is returned on
the first match.

When the CSV is indexed, a faster parallel search is used.


<a name="examples"></a>

## Examples [↩](#nav)

> Search for rows where any field contains the regex 'foo.*bar' (case sensitive)

```console
qsv search 'foo.*bar' data.csv
```

> Case insensitive search for 'error' in the 'message' column

```console
qsv search -i 'error' -s message data.csv
```

> Search for exact matches of 'completed' in the 'status' column

```console
qsv search --exact 'completed' -s status data.csv
```

> Search for literal string 'a.b*c' in all columns

```console
qsv search --literal 'a.b*c' data.csv
```

> Invert match: select rows that do NOT match the regex 'test'

```console
qsv search --invert-match 'test' data.csv
```

> Flag matched rows in a new column named 'match_flag'

```console
qsv search --flag match_flag 'pattern' data.csv
```

> Quick search: return on first match of 'urgent' in the 'subject' column

```console
qsv search --quick 'urgent' -s subject data.csv
```

> Preview the first 5 matches of 'warning' in all columns

```console
qsv search --preview-match 5 'warning' data.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_search.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv search [options] <regex> [<input>]
qsv search --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<regex>`&nbsp; | Regular expression to match. Uses Rust regex syntax. See <https://docs.rs/regex/latest/regex/index.html#syntax> or <https://regex101.com> with the Rust flavor for more info. |
| &nbsp;`<input>`&nbsp; | The CSV file to read. If not given, reads from stdin. |

<a name="search-options"></a>

## Search Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Case insensitive search. This is equivalent to prefixing the regex with '(?i)'. |  |
| &nbsp;`--literal`&nbsp; | flag | Treat the regex as a literal string. This allows you to search for matches that contain regex special characters. |  |
| &nbsp;`--exact`&nbsp; | flag | Match the ENTIRE field exactly. Treats the pattern as a literal string (like --literal) and automatically anchors it to match the complete field value (^pattern$). |  |
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select the columns to search. See 'qsv select -h' for the full syntax. |  |
| &nbsp;`-v,`<br>`--invert-match`&nbsp; | flag | Select only rows that did not match |  |
| &nbsp;`-u,`<br>`--unicode`&nbsp; | flag | Enable unicode support. When enabled, character classes will match all unicode word characters instead of only ASCII word characters. Decreases performance. |  |
| &nbsp;`-f,`<br>`--flag`&nbsp; | string | If given, the command will not filter rows but will instead flag the found rows in a new column named <column>, with the row numbers of the matched rows and 0 for the non-matched rows. If column is named M, only the M column will be written to the output, and only matched rows are returned. |  |
| &nbsp;`-Q,`<br>`--quick`&nbsp; | flag | Return on first match with an exitcode of 0, returning the row number of the first match to stderr. Return exit code 1 if no match is found. No output is produced. |  |
| &nbsp;`--preview-match`&nbsp; | string | Preview the first N matches or all the matches found in N milliseconds, whichever occurs first. Returns the preview to stderr. Output is still written to stdout or --output as usual. Only applicable when CSV is NOT indexed, as it's read sequentially. Forces a sequential search, even if the CSV is indexed. |  |
| &nbsp;`-c,`<br>`--count`&nbsp; | flag | Return number of matches to stderr. |  |
| &nbsp;`--size-limit`&nbsp; | string | Set the approximate size limit (MB) of the compiled regular expression. If the compiled expression exceeds this number, then a compilation error is returned. Modify this only if you're getting regular expression compilation errors. | `50` |
| &nbsp;`--dfa-size-limit`&nbsp; | string | Set the approximate size of the cache (MB) used by the regular expression engine's Discrete Finite Automata. Modify this only if you're getting regular expression compilation errors. | `10` |
| &nbsp;`--json`&nbsp; | flag | Output the result as JSON. Fields are written as key-value pairs. The key is the column name. The value is the field value. The output is a JSON array. If --no-headers is set, then the keys are the column indices (zero-based). Automatically sets --quiet. |  |
| &nbsp;`--not-one`&nbsp; | flag | Use exit code 0 instead of 1 for no match found. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel when the given CSV data has an index. Note that a file handle is opened for each job. When not set, defaults to the number of CPUs detected. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. Only applicable when CSV is NOT indexed. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not return number of matches to stderr. |  |

---
**Source:** [`src/cmd/search.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/search.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
