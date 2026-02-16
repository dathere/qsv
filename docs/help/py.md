# py

> Create a new computed column or filter rows by evaluating a Python expression on every row of a CSV file. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting, [with the ability to evaluate Python expressions as well](https://github.com/dathere/qsv/blob/4cd00dca88addf0d287247fa27d40563b6d46985/src/cmd/python.rs#L23-L31). [Requires Python 3.8 or greater](https://github.com/dathere/qsv/blob/master/docs/INTERPRETERS.md#building-qsv-with-python-feature).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/python.rs](https://github.com/dathere/qsv/blob/master/src/cmd/python.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="requires UTF-8 encoded input.">🔣</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Py Options](#py-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Create a new computed column or filter rows by evaluating a Python expression on
every row of a CSV file.

The executed Python has 4 ways to reference cell values (as strings):
1. Directly by using column name (e.g. amount) as a local variable. If a column
name has spaces and other special characters, they are replaced with underscores
(e.g. "unit cost" -> unit_cost, "test-units/sec" -> test_units_sec)
2. Indexing cell value by column name as an attribute: col.amount
3. Indexing cell value by column name as a key: col["amount"]
4. Indexing cell value by column position: col[0]

Of course, if your input has no headers, then 4. will be the only available
option.

Some usage examples:

Sum numeric columns 'a' and 'b' and call new column 'c'
```console
$ qsv py map c "int(a) + int(b)"
```

```console
$ qsv py map c "int(col.a) + int(col['b'])"
```

```console
$ qsv py map c "int(col[0]) + int(col[1])"
```


Use Python f-strings to calculate using multiple columns (qty, fruit & "unit cost")
and format into a new column 'formatted'
```console
$ qsv py map formatted 'f"{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}"'
```


You can even have conditionals in your f-string:
```console
$ qsv py map formatted \
'f"""{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}. Its quite {"cheap" if ((float(unit_cost) * float(qty)) < 20.0) else "expensive"}!"""'
```


Note how we needed to use triple double quotes for the f-string, so we can use the literals
"cheap" and "expensive" in the f-string expression.

Strip and prefix cell values
```console
$ qsv py map prefixed "'clean_' + a.strip()"
```


Filter some lines based on numerical filtering
```console
$ qsv py filter "int(a) > 45"
```


Load helper file with function to compute Fibonacci sequence of the column "num_col"
```console
$ qsv py map --helper fibonacci.py fib qsv_uh.fibonacci(num_col) data.csv
```


Below is a detailed example of the --helper option:

Use case:
Need to calculate checksum/md5sum of some columns. First column (c1) is "id", and do md5sum of
the rest of the columns (c2, c3 and c4).

Given test.csv:
c1,c2,c3,c4
1,a2,a3,a4
2,b2,b3,b4
3,c2,c3,c4

and hashhelper.py:
import hashlib
def md5hash (*args):
s = ",".join(args)
return(hashlib.md5(s.encode('utf-8')).hexdigest())

with the following command:
```console
$ qsv py map --helper hashhelper.py hashcol 'qsv_uh.md5hash(c2,c3,c4)' test.csv
```


we get:
c1,c2,c3,c4,hashcol
1,a2,a3,a4,cb675342ed940908eef0844d17c35fab
2,b2,b3,b4,7d594b33f82bdcbc1cfa6f924a84c4cd
3,c2,c3,c4,6eabbfdbfd9ab6ae7737fb2b82f6a1af

Note that qsv with the `python` feature enabled will panic on startup even if you're not
using the `py` command if Python's shared libraries are not found.

Also, the following Python modules are automatically loaded and available to the user -
builtins, math, random & datetime. The user can import additional modules with the --helper option,
with the ability to use any Python module that's installed in the current Python virtualenv.

The Python expression is evaluated on a per record basis.
With "py map", if the expression is invalid for a record, "<ERROR>" is returned for that record.
With "py filter", if the expression is invalid for a record, that record is not filtered.

If any record has an invalid result, an exitcode of 1 is returned and an error count is logged.

For more extensive examples, see <https://github.com/dathere/qsv/blob/master/tests/test_py.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv py map [options] -n <expression> [<input>]
qsv py map [options] <new-column> <expression> [<input>]
qsv py map --helper <file> [options] <new-column> <expression> [<input>]
qsv py filter [options] <expression> [<input>]
qsv py map --help
qsv py filter --help
qsv py --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<expression>`&nbsp; | Can either be a Python expression, or if it starts with "file:" or ends with ".py" - the filepath from which to load the Python expression. Note that argument expects a SINGLE expression, and not a full-blown Python script. Use the --helper option to load helper code that you can call from the expression. |

<a name="py-options"></a>

## Py Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-f,`<br>`--helper`&nbsp; | string | File containing Python code that's loaded into the qsv_uh Python module. Functions with a return statement in the file can be called with the prefix "qsv_uh". The returned value is used in the map or filter operation. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to process before releasing memory and acquiring a new GILpool. Set to 0 to process the entire file in one batch. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Namely, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. |  |

---
**Source:** [`src/cmd/python.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/python.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
