# enum

> Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/enumerate.rs](https://github.com/dathere/qsv/blob/master/src/cmd/enumerate.rs)** | 👆

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Enum Options](#enum-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Add a new column enumerating the lines of a CSV file. This can be useful to keep
track of a specific line order, give a unique identifier to each line or even
make a copy of the contents of a column.

The enum function has four modes of operation:

1. INCREMENT. Add an incremental identifier to each of the lines:
```console
$ qsv enum file.csv
```


2. UUID4. Add a uuid v4 to each of the lines:
```console
$ qsv enum --uuid4 file.csv
```


3. UUID7. Add a uuid v7 to each of the lines:
```console
$ qsv enum --uuid7 file.csv
```


3. CONSTANT. Create a new column filled with a given value:
```console
$ qsv enum --constant 0
```


4. COPY. Copy the contents of a column to a new one:
```console
$ qsv enum --copy names
```


5. HASH. Create a new column with the deterministic hash of the given column/s.
The hash uses the xxHash algorithm and is platform-agnostic.
(see <https://github.com/DoumanAsh/xxhash-rust> for more information):
```console
$ qsv enum --hash 1- // hash all columns, auto-ignores existing "hash" column
```

```console
$ qsv enum --hash col2,col3,col4 // hash specific columns
```

```console
$ qsv enum --hash col2 // hash a single column
```

```console
$ qsv enum --hash /record_id|name|address/ // hash columns that match a regex
```

```console
$ qsv enum --hash !/record_id/ // hash all columns except the record_id column
```


Finally, you should also be able to shuffle the lines of a CSV file by sorting
on the generated uuid4s:
```console
$ qsv enum --uuid4 file.csv | qsv sort -s uuid4 > shuffled.csv
```


This will shuffle the lines of the file.csv file as uuids generated using the v4
specification are random and for practical purposes, are unique (1 in 2^122).
See <https://en.wikipedia.org/wiki/Universally_unique_identifier#Collisions>

However, sorting on uuid7 identifiers will not work as they are time-based
and monotonically increasing, and will not shuffle the lines.


<a name="examples"></a>

## Examples [↩](#nav)

> Add an incremental index column starting from 0 (default)

```console
qsv enum data.csv
```

> Add an incremental index column starting from 100 and incrementing by 10

```console
qsv enum --start 100 --increment 10 data.csv
```

> Add a uuid v4 column

```console
qsv enum --uuid4 data.csv
```

> Add a uuid v7 column

```console
qsv enum --uuid7 data.csv
```

> Add a constant column with the value "active"

```console
qsv enum --constant active data.csv
```

> Add a constant column with null values

```console
qsv enum --constant "<NULL>" data.csv
```

> Add a copy of the "username" column as "username_copy"

```console
qsv enum --copy username data.csv
```

> Add a hash column with the hash of columns "first_name" and "last_name"

```console
qsv enum --hash first_name,last_name data.csv
```

> Add a hash column with the hash of all columns except an existing "hash" column

```console
qsv enum --hash 1- data.csv
```

> Add a hash column with the hash of all columns except "id" and "uuid" columns

```console
qsv enum --hash "!id,!uuid" data.csv
```

> Add a hash column with the hash of all columns that match the regex "record|name|address"

```console
qsv enum --hash "/record|name|address/" data.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_enumerate.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv enum [options] [<input>]
qsv enum --help
```

<a name="enum-options"></a>

## Enum Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-c, --new-column` | string | Name of the column to create. Will default to "index". |  |
| `--start` | string | The value to start the enumeration from. Only applies in Increment mode. (default: 0) |  |
| `--increment` | string | The value to increment the enumeration by. Only applies in Increment mode. (default: 1) |  |
| `--constant` | string | Fill a new column with the given value. Changes the default column name to "constant" unless overridden by --new-column. To specify a null value, pass the literal "<NULL>". |  |
| `--copy` | string | Name of a column to copy. Changes the default column name to "{column}_copy" unless overridden by --new-column. |  |
| `--uuid4` | flag | When set, the column will be populated with uuids (v4) instead of the incremental identifier. Changes the default column name to "uuid4" unless overridden by --new-column. |  |
| `--uuid7` | flag | When set, the column will be populated with uuids (v7) instead of the incremental identifier. uuid v7 is a time-based uuid and is monotonically increasing. See <https://buildkite.com/blog/goodbye-integers-hello-uuids> Changes the default column name to "uuid7" unless overridden by --new-column. |  |
| `--hash` | string | Create a new column filled with the hash of the given column/s. Use "1-" to hash all columns. Changes the default column name to "hash" unless overridden by --new-column. Will remove an existing "hash" column if it exists. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/enumerate.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/enumerate.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
