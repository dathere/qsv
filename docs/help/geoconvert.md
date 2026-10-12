# geoconvert

> Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/geoconvert.rs](https://github.com/dathere/qsv/blob/master/src/cmd/geoconvert.rs)** | [🌎](TableOfContents.md#legend "has geospatial capabilities.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Geoconvert Options](#geoconvert-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

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


Convert latitude/longitude columns into a WKT geometry column, reading a TSV file:  

```console
$ qsv geoconvert file.tsv csv csv --latitude lat --longitude lon
```


For CSV input, rows whose geometry cannot be used are skipped: an empty geometry, latitude
or longitude cell, WKT that does not parse, a latitude or longitude that is not a finite
number, or a latitude outside -90..90 or longitude outside -180..180. A warning on stderr
counts the skipped rows and gives the line of the first one. If EVERY row is skipped,
geoconvert fails instead, as the columns are then most likely wrong - swapped, or holding
projected coordinates such as State Plane feet. Latitude and longitude values may be
padded with spaces.

A shapefile needs its .shp, .shx and .dbf files side by side. If its .prj file describes a
projected coordinate system (e.g. NY State Plane), geoconvert warns: the coordinates are
output unchanged, so they are not the WGS-84 longitude/latitude that GeoJSON requires.
Reprojection is not done by geoconvert; reproject the shapefile first, e.g. with
ogr2ogr -t_srs EPSG:4326 out.shp in.shp

See also <https://github.com/dathere/qsv/wiki/Geospatial#geoconvert>


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv geoconvert [options] (<input>) (<input-format>) (<output-format>)
qsv geoconvert --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | The spatial file to convert. To use stdin instead, use a dash "-". Note: SHP input must be a path to a .shp file and cannot use stdin. |
| &nbsp;`<input-format>`&nbsp; | Valid values are "geojson", "shp", and "csv" |
| &nbsp;`<output-format>`&nbsp; | Valid values are:<ul><li>For GeoJSON input: "csv", "svg", and "geojsonl"</li><li>For SHP input: "csv", "geojson", "geojsonl", and "svg"</li><li>For CSV input: "geojson", "geojsonl", "svg", and "csv" (CSV output has the geometry as a WKT column)</li></ul> |

<a name="geoconvert-options"></a>

## Geoconvert Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑g,`<br>`‑‑geometry`&nbsp; | string | The name of the column that has WKT geometry. Alternative to --latitude and --longitude. |  |
| &nbsp;`‑y,`<br>`‑‑latitude`&nbsp; | string | The name of the column with northing values. |  |
| &nbsp;`‑x,`<br>`‑‑longitude`&nbsp; | string | The name of the column with easting values. |  |
| &nbsp;`‑l,`<br>`‑‑max‑length`&nbsp; | integer | The maximum column length in bytes when the output format is CSV. Oftentimes, the geometry column is too long to fit in a CSV file, causing other tools like Python & PostgreSQL to fail. If a column is too long, it will be truncated to the specified length and an ellipsis ("...") will be appended, so a truncated value is up to 3 bytes longer. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV input. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/geoconvert.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/geoconvert.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
