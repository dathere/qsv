# applydp

> applydp is a slimmed-down version of `apply` with only Datapusher+ relevant subcommands/operations (`qsvdp` binary variant only).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/applydp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/applydp.rs)** | 📇🚀🔣👆 ![CKAN](../images/ckan.png)

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Applydp Options](#applydp-options) | [Operations Options](#operations-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

applydp is a slimmed-down version of apply specifically created for Datapusher+.
It "applies" a series of transformation functions to given CSV column/s. This can be used to
perform typical data-wrangling tasks and/or to harmonize some values, etc.

It has three subcommands:
1. operations*   - 18 string, format & regex operators.
2. emptyreplace* - replace empty cells with <--replacement> string.
3. dynfmt        - Dynamically constructs a new column from other columns using
the <--formatstr> template.
* subcommand is multi-column capable.

OPERATIONS (multi-column capable)
Multiple operations can be applied, with the comma-delimited operation series
applied in order:

trim => Trim the cell
trim,upper => Trim the cell, then transform to uppercase

Operations support multi-column transformations. Just make sure the
number of transformed columns with the --rename option is the same. e.g.:

```console
$ qsv applydp operations trim,upper col1,col2,col3 -r newcol1,newcol2,newcol3 file.csv
```


It has 18 supported operations:

* len: Return string length
* lower: Transform to lowercase
* upper: Transform to uppercase
* squeeze: Compress consecutive whitespaces
* squeeze0: Remove whitespace
* trim: Trim (drop whitespace left & right of the string)
* ltrim: Left trim whitespace
* rtrim: Right trim whitespace
* mtrim: Trims --comparand matches left & right of the string (Rust trim_matches)
* mltrim: Left trim --comparand matches (Rust trim_start_matches)
* mrtrim: Right trim --comparand matches (Rust trim_end_matches)
* strip_prefix: Removes specified prefix in --comparand
* strip_suffix: Remove specified suffix in --comparand
* escape - escape (Rust escape_default)
* replace: Replace all matches of a pattern (using --comparand)
with a string (using --replacement) (Rust replace)
* regex_replace: Replace all regex matches in --comparand w/ --replacement.
Specify <NULL> as --replacement to remove matches.
* round: Round numeric values to the specified number of decimal places using
Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
Specify the number of decimal places with --formatstr (default: 3).
* copy: Mark a column for copying

EMPTYREPLACE (multi-column capable)
Replace empty cells with <--replacement> string.
Non-empty cells are not modified. See the `fill` command for more complex empty field operations.

### Dynfmt

Dynamically constructs a new column from other columns using the <--formatstr> template.
The template can contain arbitrary characters. To insert a column value, enclose the
column name in curly braces, replacing all non-alphanumeric characters with underscores.

If you need to dynamically construct a column with more complex formatting requirements and
computed values, check out the py command to take advantage of Python's f-string formatting.


<a name="examples"></a>

## Examples [↩](#nav)

### OPERATIONS

> Trim, then transform to uppercase the surname field.

```console
qsv applydp operations trim,upper surname file.csv
```

> Trim, then transform to uppercase the surname field and rename the column uppercase_clean_surname.

```console
qsv applydp operations trim,upper surname -r uppercase_clean_surname file.csv
```

> Trim, then transform to uppercase the surname field and

> save it to a new column named uppercase_clean_surname.

```console
qsv applydp operations trim,upper surname -c uppercase_clean_surname file.csv
```

> Trim, squeeze, then transform to uppercase in place ALL fields that end with "_name"

```console
qsv applydp operations trim,squeeze,upper \_name$\ file.csv
```

> Trim, then transform to uppercase the firstname and surname fields and

> rename the columns ufirstname and usurname.

```console
qsv applydp operations trim,upper firstname,surname -r ufirstname,usurname file.csv
```

> Trim parentheses & brackets from the description field.

```console
qsv applydp operations mtrim description --comparand '()<>' file.csv
```

> Replace ' and ' with ' & ' in the description field.

```console
qsv applydp replace description --comparand ' and ' --replacement ' & ' file.csv
```

> You can also use this subcommand command to make a copy of a column:

```console
qsv applydp operations copy col_to_copy -c col_copy file.csv
```

### EMPTYREPLACE

> Replace empty cells in file.csv Measurement column with 'None'.

```console
qsv applydp emptyreplace Measurement --replacement None file.csv
```

> Replace empty cells in file.csv Measurement column with 'Unknown Measurement'.

```console
qsv applydp emptyreplace Measurement --replacement 'Unknown Measurement' file.csv
```

> Replace empty cells in file.csv M1,M2 and M3 columns with 'None'.

```console
qsv applydp emptyreplace M1,M2,M3 --replacement None file.csv
```

> Replace all empty cells in file.csv for columns that start with 'Measurement' with 'None'.

```console
qsv applydp emptyreplace '/^Measurement/' --replacement None file.csv
```

> Replace all empty cells in file.csv for columns that start with 'observation'

> case insensitive with 'None'.

```console
qsv applydp emptyreplace --replacement None '/(?i)^observation/' file.csv
```

### DYNFMT

> Create a new column 'mailing address' from 'house number', 'street', 'city' and 'zip-code' columns:

```console
qsv applydp dynfmt --formatstr '{house_number} {street}, {city} {zip_code} USA' -c 'mailing address' file.csv
```

> Create a new column 'FullName' from 'FirstName', 'MI', and 'LastName' columns:

```console
qsv applydp dynfmt --formatstr 'Sir/Madam {FirstName} {MI}. {LastName}' -c FullName file.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_applydp.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv applydp operations <operations> [options] <column> [<input>]
qsv applydp emptyreplace --replacement=<string> [options] <column> [<input>]
qsv applydp dynfmt --formatstr=<string> [options] --new-column=<name> [<input>]
qsv applydp --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<column>`&nbsp; | The column/s to apply the transformation to. Note that the <column> argument supports multiple columns for the operations & emptyreplace subcommands. See 'qsv select --help' for the format details. |
| &nbsp;`<operations>`&nbsp; | The operation/s to apply. |
| &nbsp;`<column>`&nbsp; | The column/s to apply the operations to. |
| &nbsp;`<column>`&nbsp; | The column/s to check for emptiness. |
| &nbsp;`<input>`&nbsp; | The input file to read from. If not specified, reads from stdin. |

<a name="applydp-options"></a>

## Applydp Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-c,`<br>`--new-column`&nbsp; | string | Put the transformed values in a new column instead. |  |
| &nbsp;`-r,`<br>`--rename`&nbsp; | string | New name for the transformed column. |  |
| &nbsp;`-C,`<br>`--comparand=<string>`&nbsp; | string | The string to compare against for replace & similarity operations. |  |
| &nbsp;`-R,`<br>`--replacement=<string>`&nbsp; | string | The string to use for the replace & emptyreplace operations. |  |
| &nbsp;`-f,`<br>`--formatstr=<string>`&nbsp; | string | This option is used by several subcommands: |  |

<a name="operations-options"></a>

## Operations Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/applydp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/applydp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
