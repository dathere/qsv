# replace

> Replace CSV data using a regex. Applies the regex to each field individually.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/replace.rs](https://github.com/dathere/qsv/blob/master/src/cmd/replace.rs)** | 📇👆🏎️

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Replace Options](#replace-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Replace occurrences of a pattern across a CSV file.

You can of course match groups using parentheses and use those in
the replacement string. But don't forget to escape your $ in bash by using a
backslash or by wrapping the replacement string into single quotes:

```console
$ qsv replace 'hel(lo)' 'hal$1' file.csv
```

```console
$ qsv replace "hel(lo)" "hal\$1" file.csv
```


Returns exitcode 0 when replacements are done, returning number of replacements to stderr.
Returns exitcode 1 when no replacements are done, unless the '--not-one' flag is used.

When the CSV is indexed, a faster parallel replace is used.
If there were any replacements, the index will be refreshed.


<a name="examples"></a>

## Examples [↩](#nav)

Replace all occurrences of 'hello' with 'world' in the file.csv file.
```console
qsv replace 'hello' 'world' file.csv
```

Replace all occurrences of 'hello' with 'world' in the file.csv file
and save the output to the file.out file.
```console
qsv replace 'hello' 'world' file.csv -o file.out
```

Replace all occurrences of 'hello' case insensitive with 'world'
in the file.csv file.
```console
qsv replace 'hello' 'world' file.csv -i
```

Replace all valid email addresses (using a regex)
with '<EMAIL>' in the file.csv file.
```console
qsv replace '([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})' \
'<EMAIL>' file.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_replace.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv replace [options] <pattern> <replacement> [<input>]
qsv replace --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<pattern>` | Regular expression pattern to match. Uses Rust regex syntax. See <https://docs.rs/regex/latest/regex/index.html#syntax> or <https://regex101.com> with the Rust flavor for more info. |
| `<input>` | The CSV file to read. If not given, reads from stdin. |
| `<replacement>` | Replacement string. Set to '<NULL>' if you want to replace matches with ''. |

<a name="replace-options"></a>

## Replace Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-i, --ignore-case` | flag | Case insensitive search. This is equivalent to prefixing the regex with '(?i)'. |  |
| `--literal` | flag | Treat the regex pattern as a literal string. This allows you to search for matches that contain regex special characters. |  |
| `--exact` | flag | Match the ENTIRE field exactly. Treats the pattern as a literal string (like --literal) and automatically anchors it to match the complete field value (^pattern$). |  |
| `-s, --select` | string | Select the columns to search. See 'qsv select -h' for the full syntax. |  |
| `-u, --unicode` | flag | Enable unicode support. When enabled, character classes will match all unicode word characters instead of only ASCII word characters. Decreases performance. |  |
| `--size-limit` | string | Set the approximate size limit (MB) of the compiled regular expression. If the compiled expression exceeds this number, then a compilation error is returned. | `50` |
| `--dfa-size-limit` | string | Set the approximate size of the cache (MB) used by the regular expression engine's Discrete Finite Automata. | `10` |
| `--not-one` | flag | Use exit code 0 instead of 1 for no replacement found. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel when the given CSV data has an index. Note that a file handle is opened for each job. When not set, defaults to the number of CPUs detected. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `-p, --progressbar` | flag | Show progress bars. Not valid for stdin. |  |
| `-q, --quiet` | flag | Do not print number of replacements to stderr. |  |

---
**Source:** [`src/cmd/replace.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/replace.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
