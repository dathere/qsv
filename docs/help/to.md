# to

> Convert CSV files to PostgreSQL, SQLite, Excel (XLSX), LibreOffice Calc (ODS) and Data Package.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/to.rs](https://github.com/dathere/qsv/blob/master/src/cmd/to.rs)** | 🚀🗄️

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [To Options](#to-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Convert CSV files to PostgreSQL, SQLite, Excel XLSX, ODS and Data Package.

### Postgresql

To convert to postgres you need to supply connection string.
The format is described here - <https://docs.rs/postgres/latest/postgres/config/struct.Config.html#examples-1>.
Additionally you can use `env=MY_ENV_VAR` and qsv will get the connection string from the
environment variable `MY_ENV_VAR`.

If using the `--dump` option instead of a connection string put a name of a file or `-` for stdout.


<a name="examples"></a>

## Examples [↩](#nav)

Load `file1.csv` and `file2.csv' file to local database `test`, with user `testuser`, and password `pass`.
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' file1.csv file2.csv
```

Load same files into a new/existing postgres schema `myschema`
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' --schema=myschema file1.csv file2.csv
```

Load same files into a new/existing postgres database whose connection string is in the
`DATABASE_URL` environment variable.
```console
qsv to postgres 'env=DATABASE_URL' file1.csv file2.csv
```

Load files inside a directory to a local database 'test' with user `testuser`, password `pass`.
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' dir1
```

Load files listed in the 'input.infile-list' to a local database 'test' with user `testuser`, password `pass`.
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' input.infile-list
```

Drop tables if they exist before loading.
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' --drop file1.csv file2.csv
```

Evolve tables if they exist before loading. Read <http://datapackage_convert.opendata.coop/evolve.html>
to explain how evolving works.
```console
qsv to postgres 'postgres://testuser:pass@localhost/test' --evolve file1.csv file2.csv
```

Create dump file.
```console
qsv to postgres --dump dumpfile.sql file1.csv file2.csv
```

Print dump to stdout.
```console
qsv to postgres --dump - file1.csv file2.csv
```

### Sqlite

Convert to sqlite db file. Will be created if it does not exist.
If using the `--dump` option, instead of a sqlite database file, put the name of the dump file or `-` for stdout.
Load `file1.csv` and `file2.csv' files to sqlite database `test.db`
```console
qsv to sqlite test.db file1.csv file2.csv
```

Load all files in dir1 to sqlite database `test.db`
```console
qsv to sqlite test.db dir
```

Load files listed in the 'mydata.infile-list' to sqlite database `test.db`
```console
qsv to sqlite test.db mydata.infile-list
```

Drop tables if they exist before loading.
```console
qsv to sqlite test.db --drop file1.csv file2.csv
```

Evolve tables if they exist. Read <http://datapackage_convert.opendata.coop/evolve.html>
to explain how evolving is done.
```console
qsv to sqlite test.db --evolve file1.csv file2.csv
```

Create dump file .
```console
qsv to sqlite --dump dumpfile.sql file1.csv file2.csv
```

Print dump to stdout.
```console
qsv to sqlite --dump - file1.csv file2.csv
```

### Excel XLSX

Convert to new xlsx file.
Example:
Load `file1.csv` and `file2.csv' into xlsx file.
Will create `output.xlsx`, creating new sheets for each file, with the sheet name being the
filename without the extension. Note the `output.xlsx` will be overwritten if it exists.
```console
qsv to xlsx output.xlsx file1.csv file2.csv
```

Load all files in dir1 into xlsx file.
```console
qsv to xlsx output.xlsx dir1
```

Load files listed in the 'ourdata.infile-list' into xlsx file.
```console
qsv to xlsx output.xlsx ourdata.infile-list
```

### ODS

Convert to new ODS (Open Document Spreadsheet) file.
Example:
Load `file1.csv` and `file2.csv' into ODS file.
Will create `output.ods`, creating new sheets for each file, with the sheet name being the
filename without the extension. Note the `output.ods` will be overwritten if it exists.
```console
qsv to ods output.ods file1.csv file2.csv
```

Load all files in dir1 into ODS file.
```console
qsv to ods output.ods dir1
```

Load files listed in the 'ourdata.infile-list' into ODS file.
```console
qsv to ods output.ods ourdata.infile-list
```

### Data Package

Generate a datapackage, which contains stats and information about what is in the CSV files.
Generate a `datapackage.json` file from `file1.csv` and `file2.csv' files.
```console
qsv to datapackage datapackage.json file1.csv file2.csv
```

Add more stats to datapackage.
```console
qsv to datapackage datapackage.json --stats file1.csv file2.csv
```

Generate a `datapackage.json` file from all the files in dir1
```console
qsv to datapackage datapackage.json dir1
```

Generate a `datapackage.json` file from all the files listed in the 'data.infile-list'
```console
qsv to datapackage datapackage.json data.infile-list
```

For all other conversions you can output the datapackage created by specifying `--print-package`.
```console
qsv to xlsx datapackage.xlsx --stats --print-package file1.csv file2.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_to.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv to postgres [options] <postgres> [<input>...]
qsv to sqlite [options] <sqlite> [<input>...]
qsv to xlsx [options] <xlsx> [<input>...]
qsv to ods [options] <ods> [<input>...]
qsv to datapackage [options] <datapackage> [<input>...]
qsv to --help
```

<a name="to-options"></a>

## To Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-k,`<br>`--print-package`&nbsp; | flag | Print statistics as datapackage, by default will print field summary. |  |
| &nbsp;`-u,`<br>`--dump`&nbsp; | flag | Create database dump file for use with `psql` or `sqlite3` command line tools (postgres/sqlite only). |  |
| &nbsp;`-a,`<br>`--stats`&nbsp; | flag | Produce extra statistics about the data beyond just type guessing. |  |
| &nbsp;`-c,`<br>`--stats-csv`&nbsp; | string | Output stats as CSV to specified file. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not print out field summary. |  |
| &nbsp;`-s,`<br>`--schema`&nbsp; | string | The schema to load the data into. (postgres only). |  |
| &nbsp;`-d,`<br>`--drop`&nbsp; | flag | Drop tables before loading new data into them (postgres/sqlite only). |  |
| &nbsp;`-e,`<br>`--evolve`&nbsp; | flag | If loading into existing db, alter existing tables so that new data will load. (postgres/sqlite only). |  |
| &nbsp;`-i,`<br>`--pipe`&nbsp; | flag | Adjust output format for piped data (omits row counts and field format columns). |  |
| &nbsp;`-p,`<br>`--separator`&nbsp; | string | For xlsx, use this character to help truncate xlsx sheet names. Defaults to space. |  |
| &nbsp;`-A,`<br>`--all-strings`&nbsp; | flag | Convert all fields to strings. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/to.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/to.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
