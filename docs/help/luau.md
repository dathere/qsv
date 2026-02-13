# luau

> Create multiple new computed columns, filter rows, compute aggregations and build complex data pipelines by executing a Luau 0.708 expression/script for every row of a CSV file (sequential mode), or using random access with an index (random access mode). Can process a single Luau expression or full-fledged data-wrangling scripts using lookup tables with discrete BEGIN, MAIN and END sections. It is not just another qsv command, it is qsv's Domain-specific Language (DSL) with numerous qsv-specific helper functions to build production data pipelines.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/luau.rs](https://github.com/dathere/qsv/blob/master/src/cmd/luau.rs)** | 📇🌐🔣📚 ![CKAN](../images/ckan.png)

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Luau Options](#luau-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Create multiple new computed columns, filter rows or compute aggregations by
executing a Luau 0.708 script for every row (SEQUENTIAL MODE) or for
specified rows (RANDOM ACCESS MODE) of a CSV file.

Luau is not just another qsv command. It is qsv's Domain-Specific Language (DSL)
for data-wrangling. 👑

The executed Luau has 3 ways to reference row columns (as strings):
1. Directly by using column name (e.g. Amount), can be disabled with --no-globals
2. Indexing col variable by column name: col.Amount or col["Total Balance"]
3. Indexing col variable by column 1-based index: col[1], col[2], etc.
This is only available with the --colindex or --no-headers options.

Of course, if your input has no headers, then 3. will be the only available
option.

It has two subcommands:
map     - Create new columns by mapping the result of a Luau script for each row.
filter  - Filter rows by executing a Luau script for each row. Rows that return
true are kept, the rest are filtered out.

Some examples:

Sum numeric columns 'a' and 'b' and call new column 'c'
```console
$ qsv luau map c "a + b"
```

```console
$ qsv luau map c "col.a + col['b']"
```

```console
$ qsv luau map c --colindex "col[1] + col[2]"
```


There is some magic in the previous example as 'a' and 'b' are passed in
as strings (not numbers), but Luau still manages to add them up.
A more explicit way of doing it, is by using the tonumber() function.
See <https://luau-lang.org/library> for a list of built-in functions.
```console
$ qsv luau map c "tonumber(a) + tonumber(b)"
```


Add running total column for Amount
```console
$ qsv luau map Total "tot = (tot or 0) + Amount; return tot"
```


Or use the --begin and --end options to compute the running & grand totals
```console
$ qsv luau map Total --begin "tot = 0; gtotal = 0" \
"tot = tot + Amount; gtotal = gtotal + tot; return tot" --end "return gtotal"
```


Add running total column for Amount when previous balance was 900
```console
$ qsv luau map Total "tot = (tot or 900) + Amount; return tot"
```


Use the qsv_cumsum() helper function to compute the running total.
See <https://github.com/dathere/qsv/wiki/Luau-Helper-Functions-Examples> for more examples.

```console
$ qsv luau map Total "qsv_cumsum(Amount)"
```


Convert Amount to always-positive AbsAmount and Type (debit/credit) columns
```console
$ qsv luau map Type \
"if tonumber(Amount) < 0 then return 'debit' else return 'credit' end" | \
qsv luau map AbsAmount "math.abs(tonumber(Amount))"
```


Map multiple new columns in one pass
```console
$ qsv luau map newcol1,newcol2,newcol3 "{cola + 1, colb + 2, colc + 3}"
```


Filter some rows based on numerical filtering
```console
$ qsv luau filter "tonumber(a) > 45"
```

```console
$ qsv luau filter "tonumber(a) >= tonumber(b)"
```


PATTERN MATCHING WITH string.find AND OTHER STRING FUNCTIONS:
Lua/Luau string functions like string.find, string.match, string.gsub use
PATTERN MATCHING by default, where certain characters have special meanings:
( ) . % + - * ? [ ] ^ $

If you need to search for these characters literally, you have two options:

1. Escape special characters with % (percent sign):
```console
$ qsv luau filter "string.find(Name, 'John %(Jr%)')"
```

```console
$ qsv luau map dots "string.gsub(col.text, '%%.', '')"
```


2. Use plain text mode (4th parameter = true):
```console
$ qsv luau filter "string.find(Name, 'John (Jr)', 1, true)"
```

```console
$ qsv luau map match "string.find(col.text, 'Mr.', 1, true)"
```


Common gotchas:
- Parentheses in names like "Jane (Smith)" need escaping or plain mode
- Dots in email addresses, URLs, or decimal numbers
- Hyphens in phone numbers or dates

For more on Lua patterns: <https://www.lua.org/manual/5.4/manual.html#6.4.1>

Typing long scripts on the command line gets tiresome rather quickly. Use the
"file:" prefix or the ".lua/.luau" file extension to read non-trivial scripts
from the filesystem.

In the following example, both the BEGIN and END scripts have the lua/luau file
extension so they are read from the filesystem.  With the debitcredit.script file,
we use the "file:" prefix to read it from the filesystem.

```console
$ qsv luau map Type -B init.lua file:debitcredit.script -E end.luau
```


With "luau map", if the MAIN script is invalid for a row, "<ERROR>" followed by a
detailed error message is returned for that row.
With "luau filter", if the MAIN script is invalid for a row, that row is not filtered.

If any row has an invalid result, an exitcode of 1 is returned and an error count
is logged.

SPECIAL VARIABLES:
"_IDX" - a READ-only variable that is zero during the BEGIN script and
set to the current row number during the MAIN & END scripts.

It is primarily used in SEQUENTIAL MODE when the CSV has no index or you
wish to process the CSV sequentially.

"_INDEX" - a READ/WRITE variable that enables RANDOM ACCESS MODE when used in
a script. Using "_INDEX" in a script switches qsv to RANDOM ACCESS MODE
where setting it to a row number will change the current row to the
specified row number. It will only work, however, if the CSV has an index.

When using _INDEX, the MAIN script will keep looping and evaluate the row
specified by _INDEX until _INDEX is set to an invalid row number
(e.g. <= zero or to a value greater than _ROWCOUNT).

If the CSV has no index, qsv will abort with an error unless "qsv_autoindex()"
is called in the BEGIN script to create an index.

"_ROWCOUNT" - a READ-only variable which is zero during the BEGIN & MAIN scripts,
and set to the rowcount during the END script when the CSV has no index
(SEQUENTIAL MODE).

When using _INDEX and the CSV has an index, _ROWCOUNT will be set to the
rowcount of the CSV file, even from the BEGINning
(RANDOM ACCESS MODE).

"_LASTROW" - a READ-only variable that is set to the last row number of the CSV.
Like _INDEX, it will also trigger RANDOM ACCESS MODE if used in a script.

Similarly, if the CSV has no index, qsv will also abort with an error unless
"qsv_autoindex()" is called in the BEGIN script to create an index.

For security and safety reasons as a purpose-built embeddable interpreter,
Luau's standard library is relatively minimal (<https://luau-lang.org/library>).
That's why qsv bundles & preloads LuaDate v2.2.1 as date manipulation is a common task.
See <https://tieske.github.io/date/> on how to use the LuaDate library.

Additional libraries can be loaded using Luau's "require" function.
See <https://github.com/LewisJEllis/awesome-lua> for a list of other libraries.

With the judicious use of "require", the BEGIN script & special variables, one can
create variables, tables, arrays & functions that can be used for complex aggregation
operations in the END script.

SCRIPT DEVELOPMENT TIPS:
When developing Luau scripts, be sure to take advantage of the "qsv_log" function to
debug your script. It will log messages at the level (INFO, WARN, ERROR, DEBUG, TRACE)
specified by the QSV_LOG_LEVEL environment variable (see docs/Logging.md for details).

At the DEBUG level, the log messages will be more verbose to facilitate debugging.
It will also skip precompiling the MAIN script to bytecode so you can see more
detailed error messages with line numbers.

Bear in mind that qsv strips comments from Luau scripts before executing them.
This is done so qsv doesn't falsely trigger on special variables mentioned in comments.
When checking line numbers in DEBUG mode, be sure to refer to the comment-stripped
scripts in the log file, not the original commented scripts.

There are more Luau helper functions in addition to "qsv_log", notably the powerful
"qsv_register_lookup" which allows you to "lookup" values against other
CSVs on the filesystem, a URL, datHere's lookup repo or CKAN instances.

Detailed descriptions of these helpers can be found in the "setup_helpers" section at
the bottom of this file and on the Wiki (<https://github.com/dathere/qsv/wiki>)

For more detailed examples, see <https://github.com/dathere/qsv/blob/master/tests/test_luau.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv luau map [options] -n <main-script> [<input>]
qsv luau map [options] <new-columns> <main-script> [<input>]
qsv luau filter [options] <main-script> [<input>]
qsv luau map --help
qsv luau filter --help
qsv luau --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<new-columns>`&nbsp; | is a comma-separated list of new computed columns to add to the CSV when using "luau map". The new columns are added to the CSV after the existing columns, unless the --remap option is used. |

<a name="luau-options"></a>

## Luau Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-g,`<br>`--no-globals`&nbsp; | flag | Don't create Luau global variables for each column, only `col`. Useful when some column names mask standard Luau globals and to increase PERFORMANCE. Note: access to Luau globals thru _G remains even with -g. |  |
| &nbsp;`--colindex`&nbsp; | flag | Create a 1-based column index. Useful when some column names mask standard Luau globals. Automatically enabled with --no-headers. |  |
| &nbsp;`-r,`<br>`--remap`&nbsp; | flag | Only the listed new columns are written to the output CSV. Only applies to "map" subcommand. |  |
| &nbsp;`-B,`<br>`--begin`&nbsp; | string | Luau script/file to execute in the BEGINning, before processing the CSV with the main-script. Typically used to initialize global variables. Takes precedence over an embedded BEGIN script. If <script> begins with "file:" or ends with ".luau/.lua", it's interpreted as a filepath from which to load the script. |  |
| &nbsp;`-E,`<br>`--end`&nbsp; | string | Luau script/file to execute at the END, after processing the CSV with the main-script. Typically used for aggregations. The output of the END script is sent to stderr. Takes precedence over an embedded END script. If <script> begins with "file:" or ends with ".luau/.lua", it's interpreted as a filepath from which to load the script. |  |
| &nbsp;`--max-errors`&nbsp; | string | The maximum number of errors to tolerate before aborting. Set to zero to disable error limit. | `10` |
| &nbsp;`--timeout`&nbsp; | string | Timeout for downloading lookup_tables using the qsv_register_lookup() helper function. | `60` |
| &nbsp;`--ckan-api`&nbsp; | string | The URL of the CKAN API to use for downloading lookup_table resources using the qsv_register_lookup() helper function with the "ckan://" scheme. If the QSV_CKAN_API envvar is set, it will be used instead. | `https://data.dathere.com/api/3/action` |
| &nbsp;`--ckan-token`&nbsp; | string | The CKAN API token to use. Only required if downloading private resources. If the QSV_CKAN_TOKEN envvar is set, it will be used instead. |  |
| &nbsp;`--cache-dir`&nbsp; | string | The directory to use for caching downloaded lookup_table resources using the qsv_register_lookup() helper function. If the directory does not exist, qsv will attempt to create it. If the QSV_CACHE_DIR envvar is set, it will be used instead. | `~/.qsv-cache` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Automatically enables --colindex option. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. Ignored in qsvdp. In SEQUENTIAL MODE, the progress bar will show the number of rows processed. In RANDOM ACCESS MODE, the progress bar will show the position of the current row being processed. Enabling this option will also suppress stderr output from the END script. |  |

---
**Source:** [`src/cmd/luau.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/luau.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
