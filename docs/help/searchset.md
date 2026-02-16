# searchset

> _Run multiple regexes over a CSV in a single pass._ Applies the regexes to each field individually & shows only matching rows.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/searchset.rs](https://github.com/dathere/qsv/blob/master/src/cmd/searchset.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="multithreaded and/or faster when an index (📇) is available.">🏎️</abbr><abbr title="has powerful column selector support. See `select` for syntax.">👆</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Searchset Options](#searchset-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Filters CSV data by whether the given regex set matches a row.

Unlike the search operation, this allows regex matching of multiple regexes
in a single pass.

The regexset-file is a plain text file with multiple regexes, with a regex on
each line.

The regex set is applied to each field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.

When --quick is enabled, no output is produced and exitcode 0 is returned on
the first match.

When the CSV is indexed, a faster parallel search is used.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_searchset.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv searchset [options] (<regexset-file>) [<input>]
qsv searchset --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<regexset-file>`&nbsp; | The file containing regular expressions to match, with a regular expression on each line. See <https://docs.rs/regex/latest/regex/index.html#syntax> or <https://regex101.com> with the Rust flavor for regex syntax. |
| &nbsp;`<input>`&nbsp; | The CSV file to read. If not given, reads from stdin. |

<a name="searchset-options"></a>

## Searchset Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Case insensitive search. This is equivalent to prefixing the regex with '(?i)'. |  |
| &nbsp;`--literal`&nbsp; | flag | Treat the regex as a literal string. This allows you to search for matches that contain regex special characters. |  |
| &nbsp;`--exact`&nbsp; | flag | Match the ENTIRE field exactly. Treats the pattern as a literal string (like --literal) and automatically anchors it to match the complete field value (^pattern$). |  |
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select the columns to search. See 'qsv select -h' for the full syntax. |  |
| &nbsp;`-v,`<br>`--invert-match`&nbsp; | flag | Select only rows that did not match |  |
| &nbsp;`-u,`<br>`--unicode`&nbsp; | flag | Enable unicode support. When enabled, character classes will match all unicode word characters instead of only ASCII word characters. Decreases performance. |  |
| &nbsp;`-f,`<br>`--flag`&nbsp; | string | If given, the command will not filter rows but will instead flag the found rows in a new column named <column>. For each found row, <column> is set to the row number of the row, followed by a semicolon, then a list of the matching regexes. |  |
| &nbsp;`--flag-matches-only`&nbsp; | flag | When --flag is enabled, only rows that match are sent to output. Rows that do not match are filtered. |  |
| &nbsp;`--unmatched-output`&nbsp; | string | When --flag-matches-only is enabled, output the rows that did not match to <file>. |  |
| &nbsp;`-Q,`<br>`--quick`&nbsp; | flag | Return on first match with an exitcode of 0, returning the row number of the first match to stderr. Return exit code 1 if no match is found. No output is produced. Ignored if --json is enabled. |  |
| &nbsp;`-c,`<br>`--count`&nbsp; | flag | Return number of matches to stderr. Ignored if --json is enabled. |  |
| &nbsp;`-j,`<br>`--json`&nbsp; | flag | Return number of matches, number of rows with matches, and number of rows to stderr in JSON format. |  |
| &nbsp;`--size-limit`&nbsp; | string | Set the approximate size limit (MB) of the compiled regular expression. If the compiled expression exceeds this number, then a compilation error is returned. Modify this only if you're getting regular expression compilation errors. | `50` |
| &nbsp;`--dfa-size-limit`&nbsp; | string | Set the approximate size of the cache (MB) used by the regular expression engine's Discrete Finite Automata. Modify this only if you're getting regular expression compilation errors. | `10` |
| &nbsp;`--not-one`&nbsp; | flag | Use exit code 0 instead of 1 for no match found. |  |
| &nbsp;`--jobs`&nbsp; | string | The number of jobs to run in parallel when the given CSV data has an index. Note that a file handle is opened for each job. When not set, defaults to the number of CPUs detected. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not return number of matches to stderr. |  |

---
**Source:** [`src/cmd/searchset.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/searchset.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
