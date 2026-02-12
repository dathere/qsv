# geoconvert

🌎

> Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/geoconvert.rs](https://github.com/dathere/qsv/blob/master/src/cmd/geoconvert.rs)**

## Description

Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more.

For example to convert a GeoJSON file into CSV data:

```console
$ qsv geoconvert file.geojson geojson csv
```


To use stdin as input instead of a file path, use a dash "-":

```console
$ qsv prompt -m "Choose a GeoJSON file" -F geojson | qsv geoconvert - geojson csv
```


To convert a CSV file into GeoJSON data, specify the WKT geometry column with the --geometry flag:

```console
$ qsv geoconvert file.csv csv geojson --geometry geometry
```


Alternatively specify the latitude and longitude columns with the --latitude and --longitude flags:

```console
$ qsv geoconvert file.csv csv geojson --latitude lat --longitude lon
```



## Usage

```console
qsv geoconvert [options] (<input>) (<input-format>) (<output-format>)
qsv geoconvert --help
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<input>` | The spatial file to convert. To use stdin instead, use a dash "-". Note: SHP input must be a path to a .shp file and cannot use stdin. |
| `<input-format>` | Valid values are "geojson", "shp", and "csv" |
| `<output-format>` | Valid values are: |

## Geoconvert Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-g, --geometry` | string | The name of the column that has WKT geometry. Alternative to --latitude and --longitude. |  |
| `-y, --latitude` | string | The name of the column with northing values. |  |
| `-x, --longitude` | string | The name of the column with easting values. |  |
| `-l, --max-length` | string | The maximum column length when the output format is CSV. Oftentimes, the geometry column is too long to fit in a CSV file, causing other tools like Python & PostgreSQL to fail. If a column is too long, it will be truncated to the specified length and an ellipsis ("...") will be appended. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/geoconvert.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/geoconvert.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
