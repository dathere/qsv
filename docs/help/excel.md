# excel

> Exports a specified Excel/ODS sheet to a CSV file.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/excel.rs](https://github.com/dathere/qsv/blob/master/src/cmd/excel.rs)**

## Description

Exports a specified Excel/ODS sheet to a CSV file.
The first non-empty row of a sheet is assumed to be the header row.


## Examples

> Export the first sheet of an Excel file to a CSV file:

```console
qsv excel input.xlsx --output output.csv
```

> Export the first sheet of an ODS file to a CSV file:

```console
qsv excel input.ods -o output.csv
```

> Export the first sheet of an Excel file to a CSV file with a custom delimiter:

```console
qsv excel input.xlsx -d ";" > output.csv
```

> Export a sheet by name (case-insensitive):

```console
qsv excel --sheet "Sheet 3" input.xlsx
```

> Export a sheet by index:

> this exports the 3nd sheet (0-based index)

```console
qsv excel -s 2 input.xlsx
```

> Export the last sheet (negative index)):

```console
qsv excel -s -1 input.xlsx
```

> Export the second to last sheet:

```console
qsv excel -s -2 input.xls
```

> Export a table named "Table1" in an XLSX file. Note that --sheet is not required

> as the table definition includes the sheet.

```console
qsv excel --table "Table1" input.xlsx
```

> Export a range of cells in the first sheet:

```console
qsv excel --range C3:T25 input.xlsx
```

> Export a named range in the workbook. Note that --sheet is not required

> as named ranges include the sheet.

```console
qsv excel --range MyRange input.xlsx
```

> Export a range of cells in the second sheet:

```console
qsv excel --range C3:T25 -s 1 input.xlsx
```

> Export a range of cells in a sheet by name.

> Note the range name must be enclosed in single quotes in certain shells

> as it may contain special characters like ! and $:

```console
qsv excel --range 'Sheet2!C3:T25' input.xlsx
```

> Export the cell C3 in the first sheet:

```console
qsv excel --cell C3 input.xlsx
```

> Export a single cell from a specific sheet:

```console
qsv excel --cell 'Sheet2!C3' input.xlsx
```

> Export metadata for all sheets in CSV format:

```console
qsv excel --metadata csv input.xlsx
```

> Export metadata in short CSV mode which is much faster

> but doesn't contain as much metadata

```console
qsv excel --metadata short input.xlsx
```

> Export metadata for all sheets in JSON format:

```console
qsv excel --metadata json input.xlsx
```

> Export metadata to pretty-printed JSON - first letter is capital J

```console
qsv excel --metadata JSON input.xlsx
```

> Export metadata in short, minified JSON mode - first letter is capital S

```console
qsv excel --metadata Short input.xlsx
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_excel.rs).


## Usage

```console
qsv excel [options] [<input>]
qsv excel --help
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<input>` | The spreadsheet file to read. Use "-" to read from stdin. Supported formats: xls, xlsx, xlsm, xlsb, ods. |

## Excel Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --sheet` | string | Name (case-insensitive) or zero-based index of sheet to export. Negative indices start from the end (-1 = last sheet). If the sheet cannot be found, qsv will read the first sheet. | `0` |
| `--header-row` | string | The header row. Set if other than the first non-empty row of the sheet. |  |
| `--metadata` | string | Outputs workbook metadata in CSV or JSON format: index, sheet_name, headers, type, visible, column_count, row_count, safe_headers, safe_headers_count, unsafe_headers, unsafe_headers_count and duplicate_headers_count, names, name_count, tables, table_count. headers is a list of the first row which is presumed to be the header row. type is the sheet type (WorkSheet, DialogSheet, MacroSheet, ChartSheet, Vba). visible is the sheet visibility (Visible, Hidden, VeryHidden). row_count includes all rows, including the first row. safe_headers is a list of headers with "safe"(PostgreSQL-ready) names. unsafe_headers is a list of headers with "unsafe" names. duplicate_headers_count is a count of duplicate header names. names is a list of defined names in the workbook, with the associated formula. name_count is the number of defined names in the workbook. tables is a list of tables in the workbook, along with the sheet where the table is found, the columns and the column_count.  (XLSX only) table_count is the number of tables in the workbook.  (XLSX only) | `none` |
| `--table` | string | An Excel table (case-insensitive) to extract to a CSV. Only valid for XLSX files. The --sheet option is ignored as a table could be in any sheet. Overrides --range option. |  |
| `--range` | string | An Excel format range - like RangeName, C:T, C3:T25 or 'Sheet1!C3:T25' to extract to the CSV. If the specified range contains the required sheet, the --sheet option is ignored. If the range is not found, qsv will exit with an error. |  |
| `--cell` | string | A single cell reference - like C3 or 'Sheet1!C3' to extract. This is a convenience option equivalent to --range C3:C3. If both --cell and --range are specified, --cell takes precedence. |  |
| `--error-format` | string | The format to use when formatting error cells. There are 3 formats: | `code` |
| `-, -` | flag | (#DIV/0!; #N/A; #NAME?; #NULL!; #NUM!; #REF!; #VALUE!; #DATA!) |  |
| `--flexible` | flag | Continue even if the number of columns is different from row to row. |  |
| `--trim` | flag | Trim all fields so that leading & trailing whitespaces are removed. Also removes embedded linebreaks. |  |
| `--date-format` | string | Optional date format to use when formatting dates. See https://docs.rs/chrono/latest/chrono/format/strftime/index.html for the full list of supported format specifiers. Note that if a date format is invalid, qsv will fall back and return the date as if no date-format was specified. |  |
| `--keep-zero-time` | flag | Keep the time part of a date-time field if it is 00:00:00. By default, qsv will remove the time part if it is 00:00:00. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The delimiter to use when writing CSV data. Must be a single character. | `,` |
| `-q, --quiet` | flag | Do not display export summary message. |  |

---
**Source:** [`src/cmd/excel.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/excel.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
