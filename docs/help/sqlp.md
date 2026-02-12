# sqlp

> Run Polars SQL (a PostgreSQL dialect) queries against several CSVs, Parquet, JSONL and Arrow files - converting queries to blazing-fast Polars LazyFrame expressions, processing larger than memory CSV files. Query results can be saved in CSV, JSON, JSONL, Parquet, Apache Arrow IPC and Apache Avro formats.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sqlp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sqlp.rs)**

## Description

Run blazing-fast Polars SQL queries against several CSVs - replete with joins, aggregations,
grouping, table functions, sorting, and more - working on larger than memory CSV files directly,
without having to load it first into a database.

Polars SQL is a PostgreSQL dialect (<https://docs.pola.rs/user-guide/sql/intro/>), converting SQL
queries to ultra-fast Polars LazyFrame expressions (<https://docs.pola.rs/user-guide/lazy/>).

For a list of SQL functions and keywords supported by Polars SQL, see
<https://docs.pola.rs/py-polars/html/reference/sql/index.html> though be aware that it's for
the Python version of Polars, so there will be some minor syntax differences.

Returns the shape of the query result (number of rows, number of columns) to stderr.

Example queries:

```console
$ qsv sqlp data.csv 'select * from data where col1 > 10 order by all desc limit 20'
```


```console
$ qsv sqlp data.csv 'select col1, col2 as friendlyname from data' --format parquet --output data.parquet
```


# enclose column names with spaces in double quotes
```console
$ qsv sqlp data.csv 'select "col 1", "col 2" from data'
```


```console
$ qsv sqlp data.csv data2.csv 'select * from data join data2 on data.colname = data2.colname'
```


```console
$ qsv sqlp data.csv data2.csv 'SELECT col1 FROM data WHERE col1 IN (SELECT col2 FROM data2)'
```


# Use dollar-quoting to avoid escaping reserved characters in literals.
<https://www.postgresql.org/docs/current/sql-syntax-lexical.html#SQL-SYNTAX-DOLLAR-QUOTING>
```console
$ qsv sqlp data.csv "SELECT * FROM data WHERE col1 = $$O'Reilly$$"
```

```console
$ qsv sqlp data.csv 'SELECT * FROM data WHERE col1 = $SomeTag$Diane's horse "Twinkle"$SomeTag$'
```


# Unions and Joins are supported.
```console
$ qsv sqlp data1.csv data2.csv 'SELECT * FROM data1 UNION ALL BY NAME SELECT * FROM data2'
```


```console
$ qsv sqlp tbl_a.csv tbl_b.csv tbl_c.csv "SELECT * FROM tbl_a \
RIGHT ANTI JOIN tbl_b USING (b) \
LEFT SEMI JOIN tbl_c USING (c)"
```


# use "_t_N" aliases to refer to input files, where N is the 1-based index
# of the input file/s. For example, _t_1 refers to the first input file, _t_2
# refers to the second input file, and so on.
```console
$ qsv sqlp data.csv data2.csv 'select * from _t_1 join _t_2 on _t_1.colname = _t_2.colname'
```


```console
$ qsv sqlp data.csv 'SELECT col1, count(*) AS cnt FROM data GROUP BY col1 ORDER BY cnt DESC, col1 ASC'
```


```console
$ qsv sqlp data.csv "select lower(col1), substr(col2, 2, 4) from data WHERE starts_with(col1, 'foo')"
```


```console
$ qsv sqlp data.csv "select COALESCE(NULLIF(col2, ''), 'foo') from data"
```


```console
$ qsv sqlp tbl1.csv "SELECT x FROM tbl1 WHERE x IN (SELECT y FROM tbl1)"
```


# Natural Joins are supported too! (<https://www.w3resource.com/sql/joins/natural-join.php>)
```console
$ qsv sqlp data1.csv data2.csv data3.csv \
"SELECT COLUMNS('^[^:]+$') FROM data1 NATURAL JOIN data2 NATURAL JOIN data3 ORDER BY COMPANY_ID",
```


# Use a SQL script to run a long, complex SQL query or to run SEVERAL SQL queries.
# When running several queries, each query needs to be separated by a semicolon,
# the last query will be returned as the result.
# Typically, earlier queries are used to create tables that can be used in later queries.
# Note that scripts support single-line comments starting with '--' so feel free to
# add comments to your script.
# In long, complex scripts that produce multiple temporary tables, note that you can use
# `truncate table <table_name>;` to free up memory used by temporary tables. Otherwise,
# the memory used by the temporary tables won't be freed until the script finishes.
# See test_sqlp/sqlp_boston311_sql_script() for an example.
```console
$ qsv sqlp data.csv data2.csv data3.csv data4.csv script.sql --format json --output data.json
```


# use Common Table Expressions (CTEs) using WITH to simplify complex queries
```console
$ qsv sqlp people.csv "WITH millennials AS (SELECT * FROM people WHERE age >= 25 and age <= 40) \
SELECT * FROM millennials WHERE STARTS_WITH(name,'C')"
```


# CASE statement
```console
$ qsv sqlp data.csv "select CASE WHEN col1 > 10 THEN 'foo' WHEN col1 > 5 THEN 'bar' ELSE 'baz' END from data"
```

```console
$ qsv sqlp data.csv "select CASE col*5 WHEN 10 THEN 'foo' WHEN 5 THEN 'bar' ELSE 'baz' END from _t_1"
```


# spaceship operator: "<=>" (three-way comparison operator)
#  returns -1 if left < right, 0 if left == right, 1 if left > right
# <https://en.wikipedia.org/wiki/Three-way_comparison#Spaceship_operator>
```console
$ qsv sqlp data.csv data2.csv "select data.c2 <=> data2.c2 from data join data2 on data.c1 = data2.c1"
```


# support ^@ ("starts with"), and ~~ (like) ,~~* (ilike),!~~ (not like),!~~* (not ilike) operators
```console
$ qsv sqlp data.csv "select * from data WHERE col1 ^@ 'foo'"
```

```console
$ qsv sqlp data.csv "select c1 ^@ 'a' AS c1_starts_with_a from data"
```

```console
$ qsv sqlp data.csv "select c1 ~~* '%B' AS c1_ends_with_b_caseinsensitive from data"
```


# support SELECT * ILIKE wildcard syntax
# select all columns from customers where the column contains 'a' followed by an 'e'
# with any characters (or no characters), in between, case-insensitive
# if customers.csv has columns LastName, FirstName, Address, City, State, Zip
# this query will return all columns for all rows except the columns that don't
# contain 'a' followed by an 'e' - i.e. except City and Zip
```console
$ qsv sqlp customers.csv "SELECT * ILIKE '%a%e%' FROM customers ORDER BY LastName, FirstName"
```


# regex operators: "~" (contains pattern, case-sensitive); "~*" (contains pattern, case-insensitive)
#   "!~" (does not contain pattern, case-sensitive); "!~*" (does not contain pattern, case-insensitive)
```console
$ qsv sqlp data.csv "select * from data WHERE col1 ~ '^foo' AND col2 > 10"
```

```console
$ qsv sqlp data.csv "select * from data WHERE col1 !~* 'bar$' AND col2 > 10"
```


# regexp_like function: regexp_like(<string>, <pattern>, <optional flags>)
# returns true if <string> matches <pattern>, false otherwise
#   <optional flags> can be one or more of the following:
#   'c' (case-sensitive - default), 'i' (case-insensitive), 'm' (multiline)
```console
$ qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo') AND col2 > 10"
```

# case-insensitive regexp_like
```console
$ qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo', 'i') AND col2 > 10"
```


# regexp match using a literal pattern
```console
$ qsv sqlp data.csv "select idx,val from data WHERE val regexp '^foo'"
```


# regexp match using patterns from another column
```console
$ qsv sqlp data.csv "select idx,val from data WHERE val regexp pattern_col"
```


# use Parquet, JSONL and Arrow files in SQL queries
```console
$ qsv sqlp data.csv "select * from data join read_parquet('data2.parquet') as t2 ON data.c1 = t2.c1"
```

```console
$ qsv sqlp data.csv "select * from data join read_ndjson('data2.jsonl') as t2 on data.c1 = t2.c1"
```

```console
$ qsv sqlp data.csv "select * from data join read_ipc('data2.arrow') as t2 ON data.c1 = t2.c1"
```

```console
$ qsv sqlp SKIP_INPUT "select * from read_parquet('data.parquet') order by col1 desc limit 100"
```

```console
$ qsv sqlp SKIP_INPUT "select * from read_ndjson('data.jsonl') as t1 join read_ipc('data.arrow') as t2 on t1.c1 = t2.c1"
```


# you can also directly load CSVs using the Polars read_csv() SQL function. This is useful when
# you want to bypass the regular CSV parser (with SKIP_INPUT) and use Polars' multithreaded,
# mem-mapped CSV parser instead - making for dramatically faster queries at the cost of CSV parser
# configurability (i.e. limited to comma delimiter, no CSV comments, etc.).
```console
$ qsv sqlp SKIP_INPUT "select * from read_csv('data.csv') order by col1 desc limit 100"
```


# note that you can also use read_csv() to read compressed files directly
# gzip, zstd and zlib automatic decompression are supported
```console
$ qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.gz')"
```

```console
$ qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.zst')"
```

```console
$ qsv sqlp SKIP_INPUT "select * from read_csv('data.csv.zlib')"
```


# apart from using Polar's table functions, you can also use SKIP_INPUT when the SELECT
# statement doesn't require an input file
```console
$ qsv sqlp SKIP_INPUT "SELECT 1 AS one, '2' AS two, 3.0 AS three"
```


# use stdin as input
$ cat data.csv | qsv sqlp - 'select * from stdin'
$ cat data.csv | qsv sqlp - data2.csv 'select * from stdin join data2 on stdin.col1 = data2.col1'

# automatic snappy decompression/compression
```console
$ qsv sqlp data.csv.sz 'select * from data where col1 > 10' --output result.csv.sz
```


# explain query plan
```console
$ qsv sqlp data.csv 'explain select * from data where col1 > 10 order by col2 desc limit 20'
```


For more examples, see <https://github.com/dathere/qsv/blob/master/tests/test_sqlp.rs>.


## Usage

```console
qsv sqlp [options] <input>... <sql>
qsv sqlp --help
```

## Sqlp Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--format` | string | The output format to use. Valid values are: csv, json, jsonl, parquet, arrow, avro | `csv` |

## Polars Csv Input Parsing Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--try-parsedates` | flag | Automatically try to parse dates/datetimes and time. If parsing fails, columns remain as strings. Note that if dates are not well-formatted in your CSV, that you may want to try to set `--ignore-errors` to relax the CSV parsing of dates. |  |
| `--infer-len` | string | The number of rows to scan when inferring the schema of the CSV. Set to 0 to do a full table scan (warning: can be slow). | `10000` |
| `--cache-schema` | flag | Create and cache Polars schema JSON files. If the schema file/s exists, it will load the schema instead of inferring it (ignoring --infer-len) and attempt to use it for each corresponding Polars "table" with the same file stem. If specified and the schema file/s do not exist, it will check if a stats cache is available. If so, it will use it to derive a Polars schema and save it. If there's no stats cache, it will infer the schema using --infer-len and save the inferred schemas. Each schema file will have the same file stem as the corresponding input file, with the extension ".pschema.json" (data.csv's Polars schema file will be data.pschema.json) NOTE: You can edit the generated schema files to change the Polars schema and cast columns to the desired data type. For example, you can force a Float32 column to be a Float64 column by changing the "Float32" type to "Float64" in the schema file. You can also cast a Float to a Decimal with a desired precision and scale. (e.g. instead of "Float32", use "{Decimal" : [10, 3]}") The valid types are: `Boolean`, `UInt8`, `UInt16`, `UInt32`, `UInt64`, `Int8`, `Int16`, `Int32`, `Int64`, `Float32`, `Float64`, `String`, `Date`, `Datetime`, `Duration`, `Time`, `Null`, `Categorical`, `Decimal` and `Enum`. |  |
| `--streaming` | flag | Use streaming mode when parsing CSVs. This will use less memory but will be slower. Only use this when you get out of memory errors. |  |
| `--low-memory` | flag | Use low memory mode when parsing CSVs. This will use less memory but will be slower. Only use this when you get out of memory errors. |  |
| `--no-optimizations` | flag | Disable non-default query optimizations. This will make queries slower. Use this when you get query errors or to force CSV parsing when there is only one input file, no CSV parsing options are used and its not a SQL script. |  |
| `--truncate-ragged-lines` | flag | Truncate ragged lines when parsing CSVs. If set, rows with more columns than the header will be truncated. If not set, the query will fail. Use this only when you get an error about ragged lines. |  |
| `--ignore-errors` | flag | Ignore errors when parsing CSVs. If set, rows with errors will be skipped. If not set, the query will fail. Only use this when debugging queries, as Polars does batched parsing and will skip the entire batch where the error occurred. To get more detailed error messages, set the environment variable POLARS_BACKTRACE_IN_ERR=1 before running the query. |  |
| `--rnull-values` | string | The comma-delimited list of case-sensitive strings to consider as null values when READING CSV files (e.g. NULL, NONE, <empty string>). Use "<empty string>" to consider an empty string a null value. | `<empty string>` |
| `--decimal-comma` | flag | Use comma as the decimal separator when parsing & writing CSVs. Otherwise, use period as the decimal separator. Note that you'll need to set --delimiter to an alternate delimiter other than the default comma if you are using this option. |  |

## Csv Output Format Only Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--datetime-format` | string | The datetime format to use writing datetimes. See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html> for the list of valid format specifiers. |  |
| `--date-format` | string | The date format to use writing dates. |  |
| `--time-format` | string | The time format to use writing times. |  |
| `--float-precision` | string | The number of digits of precision to use when writing floats. |  |
| `--wnull-value` | string | The string to use when WRITING null values. | `<empty string>` |
| `--compression` | string | The compression codec to use when writing arrow or parquet files. For Arrow, valid values are: zstd, lz4, uncompressed For Avro, valid values are: deflate, snappy, uncompressed (default) For Parquet, valid values are: zstd, lz4raw, gzip, snappy, uncompressed | `zstd` |

## Parquet Output Format Only Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--compress-level` | string | The compression level to use when using zstd or gzip compression. When using zstd, valid values are -7 to 22, with -7 being the lowest compression level and 22 being the highest compression level. When using gzip, valid values are 1-9, with 1 being the lowest compression level and 9 being the highest compression level. Higher compression levels are slower. The zstd default is 3, and the gzip default is 6. |  |
| `--statistics` | flag | Compute column statistics when writing parquet files. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading and writing CSV data. Must be a single character. | `,` |
| `-q, --quiet` | flag | Do not return result shape to stderr. |  |

---
**Source:** [`src/cmd/sqlp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sqlp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
