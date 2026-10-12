pub(crate) static USAGE: &str = r#"
Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more.

For example to convert a GeoJSON file into CSV data:

  $ qsv geoconvert file.geojson geojson csv

To use stdin as input instead of a file path, use a dash "-":

  $ qsv prompt -m "Choose a GeoJSON file" -F geojson | qsv geoconvert - geojson csv

To convert a CSV file into GeoJSON data, specify the WKT geometry column with the --geometry flag:

  $ qsv geoconvert file.csv csv geojson --geometry geometry

Alternatively specify the latitude and longitude columns with the --latitude and --longitude flags:

  $ qsv geoconvert file.csv csv geojson --latitude lat --longitude lon

Convert latitude/longitude columns into a WKT geometry column, reading a TSV file:

  $ qsv geoconvert file.tsv csv csv --latitude lat --longitude lon

For CSV input, rows whose geometry cannot be used are skipped: an empty geometry, latitude
or longitude cell, WKT that does not parse, a latitude or longitude that is not a finite
number, or a latitude outside -90..90 or longitude outside -180..180. A warning on stderr
counts the skipped rows and gives the line of the first one. If EVERY row is skipped,
geoconvert fails instead, as the columns are then most likely wrong - swapped, or holding
projected coordinates such as State Plane feet. Latitude and longitude values may be
padded with spaces.

Swapped latitude/longitude columns are only caught when the swapped values fall out of
range (e.g. US West Coast longitudes near -122). Where both values are within +/-90
(e.g. New York: 40.7, -74), swapped columns are NOT detected - check a few rows on a map.

A shapefile needs its .shp, .shx and .dbf files side by side. If its .prj file describes a
projected coordinate system (e.g. NY State Plane), geoconvert warns: the coordinates are
output unchanged, so they are not the WGS-84 longitude/latitude that GeoJSON requires.
Reprojection is not done by geoconvert; reproject the shapefile first, e.g. with
ogr2ogr -t_srs EPSG:4326 out.shp in.shp

See also https://github.com/dathere/qsv/wiki/Geospatial#geoconvert

Usage:
    qsv geoconvert [options] (<input>) (<input-format>) (<output-format>)
    qsv geoconvert --help

geoconvert REQUIRED arguments:
    <input>           The spatial file to convert. To use stdin instead, use a dash "-".
                      Note: SHP input must be a path to a .shp file and cannot use stdin.
    <input-format>    Valid values are "geojson", "shp", and "csv"
    <output-format>   Valid values are:
                      - For GeoJSON input: "csv", "svg", and "geojsonl"
                      - For SHP input: "csv", "geojson", "geojsonl", and "svg"
                      - For CSV input: "geojson", "geojsonl", "svg", and "csv"
                        (CSV output has the geometry as a WKT column)

geoconvert options:
                                 REQUIRED FOR CSV INPUT
    -g, --geometry <geometry>    The name of the column that has WKT geometry.
                                 Alternative to --latitude and --longitude.
    -y, --latitude <col>         The name of the column with northing values.
    -x, --longitude <col>        The name of the column with easting values.

    -l, --max-length <length>    The maximum column length in bytes when the output format
                                 is CSV. Oftentimes, the geometry column is too long to fit
                                 in a CSV file, causing other tools like Python & PostgreSQL
                                 to fail. If a column is too long, it will be truncated to
                                 the specified length and an ellipsis ("...") will be
                                 appended, so a truncated value is up to 3 bytes longer.

Common options:
    -h, --help                   Display this message
    -o, --output <file>          Write output to <file> instead of stdout.
    -d, --delimiter <arg>        The field delimiter for reading CSV input.
                                 Must be a single character. (default: ,)
"#;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write},
    path::Path,
};

use csv::{Reader, StringRecord, Writer};
use geozero::{
    ColumnValue, CoordDimensions, FeatureProcessor, GeomProcessor, GeozeroDatasource,
    ProcessorSink, PropertyProcessor,
    csv::CsvWriter,
    geojson::{GeoJsonLineWriter, GeoJsonWriter},
    svg::SvgWriter,
};
use serde::Deserialize;
use tempfile::NamedTempFile;

use crate::{
    CliError, CliResult,
    config::{Config, Delimiter},
    util,
};

/// Truncate a string at the nearest char boundary <= `max_len` bytes,
/// then append an ellipsis. Avoids panicking on multi-byte UTF-8.
///
/// Precondition: `max_len > 0`. Callers in this module only invoke this when
/// `value.len() > max_len`, so a zero `max_len` is meaningless here. We
/// debug-assert it to catch any future misuse during development.
fn truncate_with_ellipsis(value: &str, max_len: usize) -> String {
    debug_assert!(max_len > 0, "truncate_with_ellipsis requires max_len > 0");
    let mut boundary = max_len;
    while boundary > 0 && !value.is_char_boundary(boundary) {
        boundary -= 1;
    }
    format!("{}...", &value[..boundary])
}

/// Helper function to handle CSV output with `max_length` truncation.
/// Buffers geozero's CSV output to a tempfile, then re-emits truncated rows.
/// The tempfile is cleaned up automatically on both success and error paths.
fn process_csv_with_max_length<F>(
    wtr: &mut Box<dyn Write>,
    max_len: usize,
    process_fn: F,
) -> CliResult<()>
where
    F: FnOnce(&mut Box<dyn Write>) -> CliResult<()>,
{
    let temp_file = NamedTempFile::new()?;

    // Write the CSV output to the temporary file. We flush explicitly so a
    // failed flush surfaces as an error here instead of being swallowed by
    // BufWriter's Drop, which would let the reader below silently see a
    // truncated file.
    {
        let temp_writer = BufWriter::new(temp_file.reopen()?);
        let mut temp_box: Box<dyn Write> = Box::new(temp_writer);
        process_fn(&mut temp_box)?;
        temp_box.flush()?;
    }

    // Read the temporary file and truncate columns that exceed the max length
    let mut rdr = Reader::from_path(temp_file.path())?;
    let headers = rdr.headers()?.clone();

    // Create a new CSV writer for the final output
    let mut csv_writer = Writer::from_writer(wtr);
    csv_writer.write_record(&headers)?;

    // Process each record and truncate columns that exceed the max length
    for result in rdr.records() {
        let record = result?;
        let mut truncated_record = Vec::with_capacity(record.len());

        for value in &record {
            if value.len() > max_len {
                truncated_record.push(truncate_with_ellipsis(value, max_len));
            } else {
                truncated_record.push(value.to_string());
            }
        }

        csv_writer.write_record(&truncated_record)?;
    }

    // Flush explicitly so any buffered CSV write errors are surfaced here
    // instead of being silently dropped when callers early-return before
    // the module-level wtr.flush() at the end of run().
    csv_writer.flush()?;

    // temp_file is dropped here, removing the underlying file.
    Ok(())
}

/// Supported input formats for spatial data conversion
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum InputFormat {
    Geojson,
    Shp,
    Csv,
}

/// Supported output formats for spatial data conversion
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    Csv,
    Svg,
    Geojson,
    Geojsonl,
}

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    arg_input_format:  InputFormat,
    arg_output_format: OutputFormat,
    flag_latitude:     Option<String>,
    flag_longitude:    Option<String>,
    flag_geometry:     Option<String>,
    flag_output:       Option<String>,
    flag_max_length:   Option<usize>,
    flag_delimiter:    Option<Delimiter>,
}

impl From<geozero::error::GeozeroError> for CliError {
    fn from(err: geozero::error::GeozeroError) -> CliError {
        match err {
            geozero::error::GeozeroError::GeometryFormat => {
                CliError::IncorrectUsage("Invalid geometry format".to_string())
            },
            geozero::error::GeozeroError::Dataset(msg) => {
                CliError::Other(format!("Dataset error: {msg}"))
            },
            _ => CliError::Other(format!("Geozero error: {err:?}")),
        }
    }
}

impl From<geozero::shp::Error> for CliError {
    fn from(err: geozero::shp::Error) -> CliError {
        CliError::Other(format!("Geozero Shapefile error: {err:?}"))
    }
}

/// Validates that the input file exists and is readable
fn validate_input_file(path: &str) -> CliResult<()> {
    if !Path::new(path).exists() {
        return fail_clierror!("Input file '{}' does not exist", path);
    }
    Ok(())
}

/// Open a buffered input reader from a file path, "-", or stdin (None).
fn open_input_reader(input: Option<&str>) -> CliResult<Box<dyn BufRead>> {
    Ok(match input {
        Some("-") | None => Box::new(BufReader::new(io::stdin())),
        Some(path) => {
            validate_input_file(path)?;
            Box::new(BufReader::new(File::open(path)?))
        },
    })
}

/// Create the output writer. Callers validate their input first, so a failed
/// conversion setup does not leave an empty or partial output file behind.
fn create_writer(output: Option<&str>) -> CliResult<Box<dyn Write>> {
    Ok(match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(BufWriter::new(io::stdout().lock())),
    })
}

/// Where a CSV row's geometry comes from.
enum GeomSource {
    Wkt(usize),
    LatLon { lat: usize, lon: usize },
}

impl GeomSource {
    const fn is_geometry_column(&self, idx: usize) -> bool {
        match *self {
            GeomSource::Wkt(geom) => idx == geom,
            GeomSource::LatLon { lat, lon } => idx == lat || idx == lon,
        }
    }
}

/// A CSV row's usable geometry.
enum RowGeometry<'a> {
    /// (longitude, latitude)
    Point(f64, f64),
    Wkt(&'a str),
}

/// Why a CSV row was skipped.
enum SkipReason {
    Empty,
    Invalid,
    OutOfRange,
}

/// Decide whether a CSV row has usable geometry. WKT is first parsed into a
/// no-op processor, so a malformed cell is skipped before anything is written
/// for its row, instead of failing after half a feature has been emitted.
fn row_geometry<'a>(
    record: &'a StringRecord,
    source: &GeomSource,
) -> Result<RowGeometry<'a>, SkipReason> {
    match *source {
        GeomSource::Wkt(idx) => {
            let wkt = record.get(idx).unwrap_or_default().trim();
            if wkt.is_empty() {
                return Err(SkipReason::Empty);
            }
            let mut bytes = wkt.as_bytes();
            geozero::wkt::read_wkt(&mut bytes, &mut ProcessorSink)
                .map_err(|_| SkipReason::Invalid)?;
            Ok(RowGeometry::Wkt(wkt))
        },
        GeomSource::LatLon { lat, lon } => {
            let lat_str = record.get(lat).unwrap_or_default().trim();
            let lon_str = record.get(lon).unwrap_or_default().trim();
            if lat_str.is_empty() || lon_str.is_empty() {
                return Err(SkipReason::Empty);
            }
            let (Ok(y), Ok(x)) = (lat_str.parse::<f64>(), lon_str.parse::<f64>()) else {
                return Err(SkipReason::Invalid);
            };
            // "NaN" and "inf" parse, but are not coordinates (serde_json would
            // even serialize them as null).
            if !y.is_finite() || !x.is_finite() {
                return Err(SkipReason::Invalid);
            }
            if y.abs() > 90.0 || x.abs() > 180.0 {
                return Err(SkipReason::OutOfRange);
            }
            Ok(RowGeometry::Point(x, y))
        },
    }
}

/// Counts of CSV rows written and skipped.
#[derive(Default)]
struct SkipStats {
    rows:               u64,
    written:            u64,
    empty:              u64,
    invalid:            u64,
    out_of_range:       u64,
    first_skipped_line: Option<u64>,
}

impl SkipStats {
    fn skip(&mut self, reason: &SkipReason, line: u64) {
        match reason {
            SkipReason::Empty => self.empty += 1,
            SkipReason::Invalid => self.invalid += 1,
            SkipReason::OutOfRange => self.out_of_range += 1,
        }
        self.first_skipped_line.get_or_insert(line);
    }

    /// Warn about skipped rows. Fails if every row was skipped, as the geometry
    /// columns are then almost certainly wrong.
    fn report(&self, source: &GeomSource) -> CliResult<()> {
        let skipped = self.empty + self.invalid + self.out_of_range;
        if skipped == 0 {
            return Ok(());
        }
        let latlon = matches!(source, GeomSource::LatLon { .. });
        let mut reasons = Vec::with_capacity(3);
        if self.empty > 0 {
            reasons.push(format!("{} empty", self.empty));
        }
        if self.invalid > 0 {
            let what = if latlon { "coordinates" } else { "WKT" };
            reasons.push(format!("{} invalid {what}", self.invalid));
        }
        if self.out_of_range > 0 {
            reasons.push(format!("{} out of range", self.out_of_range));
        }
        let reasons = reasons.join(", ");
        let first = self.first_skipped_line.unwrap_or_default();

        if self.written == 0 {
            let hint = if latlon && self.out_of_range > 0 {
                " Are --latitude and --longitude swapped, or do they hold projected coordinates \
                 (e.g. State Plane)?"
            } else {
                ""
            };
            return fail_clierror!(
                "All {} rows were skipped for unusable geometry ({reasons}; first at line \
                 {first}).{hint}",
                self.rows
            );
        }

        let hint = if latlon && self.out_of_range * 2 > self.rows {
            " Are --latitude and --longitude swapped?"
        } else {
            ""
        };
        wwarn!(
            "Skipped {skipped} of {} rows with unusable geometry ({reasons}; first at line \
             {first}).{hint}",
            self.rows
        );
        Ok(())
    }
}

/// An opened CSV input, with its geometry columns resolved.
struct CsvInput {
    rdr:     csv::Reader<Box<dyn io::Read + Send>>,
    headers: StringRecord,
    source:  GeomSource,
}

/// Open the CSV input and resolve its geometry columns. This runs before the
/// output is created, so a misnamed column fails without leaving partial
/// output behind.
fn open_csv_input(args: &Args) -> CliResult<CsvInput> {
    if args.flag_geometry.is_some()
        && (args.flag_latitude.is_some() || args.flag_longitude.is_some())
    {
        return fail_clierror!("Cannot use --geometry flag with --latitude or --longitude.");
    }
    if args.flag_geometry.is_none() && args.flag_latitude.is_some() != args.flag_longitude.is_some()
    {
        return fail_clierror!("--latitude and --longitude must be used together.");
    }
    if args.flag_geometry.is_none() && args.flag_latitude.is_none() {
        return fail_clierror!(
            "Please specify a geometry column with the --geometry option or longitude/latitude \
             with the --latitude and --longitude options."
        );
    }

    if let Some(path) = args.arg_input.as_deref()
        && path != "-"
    {
        validate_input_file(path)?;
    }
    let mut rdr = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(false)
        .reader()?;
    let headers = rdr.headers()?.clone();

    let column = |name: &str, kind: &str| -> CliResult<usize> {
        headers
            .iter()
            .position(|h| h == name)
            .ok_or_else(|| CliError::IncorrectUsage(format!("{kind} column '{name}' not found")))
    };
    let source = match (
        &args.flag_geometry,
        &args.flag_latitude,
        &args.flag_longitude,
    ) {
        (Some(geometry), ..) => GeomSource::Wkt(column(geometry, "Geometry")?),
        (None, Some(lat), Some(lon)) => GeomSource::LatLon {
            lat: column(lat, "Latitude")?,
            lon: column(lon, "Longitude")?,
        },
        _ => unreachable!("geometry flag combinations are validated above"),
    };

    Ok(CsvInput {
        rdr,
        headers,
        source,
    })
}

/// Stream CSV rows into a geozero feature processor (one of the output
/// writers), skipping rows without usable geometry. Feature indexes count
/// written features, not input rows: `GeoJsonWriter` writes the separating
/// comma for any index > 0, so a skipped first row must not leave a gap.
fn write_csv_features<P: FeatureProcessor>(
    input: &mut CsvInput,
    processor: &mut P,
    stats: &mut SkipStats,
) -> CliResult<()> {
    processor.dataset_begin(None)?;
    let mut record = StringRecord::new();
    while input.rdr.read_record(&mut record)? {
        stats.rows += 1;
        // the header is line 1
        let line = record
            .position()
            .map_or(stats.rows + 1, csv::Position::line);
        let geometry = match row_geometry(&record, &input.source) {
            Ok(geometry) => geometry,
            Err(reason) => {
                stats.skip(&reason, line);
                continue;
            },
        };

        let idx = stats.written;
        processor.feature_begin(idx)?;
        processor.properties_begin()?;
        let mut property_idx = 0;
        for (col_idx, (name, value)) in input.headers.iter().zip(record.iter()).enumerate() {
            if input.source.is_geometry_column(col_idx) {
                continue;
            }
            processor.property(property_idx, name, &ColumnValue::String(value))?;
            property_idx += 1;
        }
        processor.properties_end()?;

        processor.geometry_begin()?;
        match geometry {
            RowGeometry::Point(x, y) => {
                processor.point_begin(0)?;
                processor.xy(x, y, 0)?;
                processor.point_end(0)?;
            },
            RowGeometry::Wkt(wkt) => {
                let mut bytes = wkt.as_bytes();
                geozero::wkt::read_wkt(&mut bytes, processor)?;
            },
        }
        processor.geometry_end()?;
        processor.feature_end(idx)?;
        stats.written += 1;
    }
    processor.dataset_end()?;
    Ok(())
}

/// The name of the projected coordinate system a shapefile's .prj (WKT)
/// describes, or None if it is geographic (or unrecognized).
fn projected_crs_name(prj: &str) -> Option<&str> {
    let wkt = prj.trim_start();
    let head = wkt.get(..8).unwrap_or(wkt).to_ascii_uppercase();
    if !(head.starts_with("PROJCS[") || head.starts_with("PROJCRS[")) {
        return None;
    }
    Some(wkt.split('"').nth(1).unwrap_or("unknown"))
}

/// An owned copy of a geozero property value, so a feature's properties can be
/// buffered and re-emitted in a fixed order.
enum OwnedValue {
    Byte(i8),
    UByte(u8),
    Bool(bool),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Float(f32),
    Double(f64),
    String(String),
    Json(String),
    DateTime(String),
    Binary(Vec<u8>),
}

impl From<&ColumnValue<'_>> for OwnedValue {
    fn from(value: &ColumnValue<'_>) -> Self {
        match *value {
            ColumnValue::Byte(v) => Self::Byte(v),
            ColumnValue::UByte(v) => Self::UByte(v),
            ColumnValue::Bool(v) => Self::Bool(v),
            ColumnValue::Short(v) => Self::Short(v),
            ColumnValue::UShort(v) => Self::UShort(v),
            ColumnValue::Int(v) => Self::Int(v),
            ColumnValue::UInt(v) => Self::UInt(v),
            ColumnValue::Long(v) => Self::Long(v),
            ColumnValue::ULong(v) => Self::ULong(v),
            ColumnValue::Float(v) => Self::Float(v),
            ColumnValue::Double(v) => Self::Double(v),
            ColumnValue::String(v) => Self::String(v.to_owned()),
            ColumnValue::Json(v) => Self::Json(v.to_owned()),
            ColumnValue::DateTime(v) => Self::DateTime(v.to_owned()),
            ColumnValue::Binary(v) => Self::Binary(v.to_vec()),
        }
    }
}

impl OwnedValue {
    fn as_column_value(&self) -> ColumnValue<'_> {
        match self {
            Self::Byte(v) => ColumnValue::Byte(*v),
            Self::UByte(v) => ColumnValue::UByte(*v),
            Self::Bool(v) => ColumnValue::Bool(*v),
            Self::Short(v) => ColumnValue::Short(*v),
            Self::UShort(v) => ColumnValue::UShort(*v),
            Self::Int(v) => ColumnValue::Int(*v),
            Self::UInt(v) => ColumnValue::UInt(*v),
            Self::Long(v) => ColumnValue::Long(*v),
            Self::ULong(v) => ColumnValue::ULong(*v),
            Self::Float(v) => ColumnValue::Float(*v),
            Self::Double(v) => ColumnValue::Double(*v),
            Self::String(v) => ColumnValue::String(v),
            Self::Json(v) => ColumnValue::Json(v),
            Self::DateTime(v) => ColumnValue::DateTime(v),
            Self::Binary(v) => ColumnValue::Binary(v),
        }
    }
}

/// Sits between geozero's shapefile reader and an output writer, fixing two
/// problems with how the reader drives the writer:
/// - It yields each record's fields in `HashMap` order, which differs from record to record, and
///   leaves out null fields. `CsvWriter` panics unless every feature has the same columns in the
///   same order. So each feature's properties are buffered and re-emitted in the .dbf's field
///   order; with `fill_missing` (CSV), a null field is written as an empty string.
/// - It discards the result of most writer calls (`.ok()`), so a failed write would go unnoticed.
///   The first error is kept for `take_error`.
struct DbfFieldOrder<'a, P> {
    inner:        &'a mut P,
    fields:       Vec<String>,
    values:       Vec<Option<OwnedValue>>,
    fill_missing: bool,
    error:        Option<geozero::error::GeozeroError>,
}

impl<'a, P: FeatureProcessor> DbfFieldOrder<'a, P> {
    fn new(inner: &'a mut P, fields: Vec<String>, fill_missing: bool) -> Self {
        let values = fields.iter().map(|_| None).collect();
        Self {
            inner,
            fields,
            values,
            fill_missing,
            error: None,
        }
    }

    fn track(&mut self, result: geozero::error::Result<()>) -> geozero::error::Result<()> {
        if let Err(err) = result
            && self.error.is_none()
        {
            self.error = Some(err);
        }
        Ok(())
    }

    fn take_error(&mut self) -> CliResult<()> {
        match self.error.take() {
            Some(err) => Err(err.into()),
            None => Ok(()),
        }
    }
}

impl<P: FeatureProcessor> PropertyProcessor for DbfFieldOrder<'_, P> {
    fn property(
        &mut self,
        _idx: usize,
        name: &str,
        value: &ColumnValue,
    ) -> geozero::error::Result<bool> {
        if let Some(pos) = self.fields.iter().position(|field| field == name) {
            self.values[pos] = Some(value.into());
        }
        Ok(false)
    }
}

impl<P: FeatureProcessor> FeatureProcessor for DbfFieldOrder<'_, P> {
    fn dataset_begin(&mut self, name: Option<&str>) -> geozero::error::Result<()> {
        let result = self.inner.dataset_begin(name);
        self.track(result)
    }

    fn dataset_end(&mut self) -> geozero::error::Result<()> {
        let result = self.inner.dataset_end();
        self.track(result)
    }

    fn feature_begin(&mut self, idx: u64) -> geozero::error::Result<()> {
        let result = self.inner.feature_begin(idx);
        self.track(result)
    }

    fn feature_end(&mut self, idx: u64) -> geozero::error::Result<()> {
        let result = self.inner.feature_end(idx);
        self.track(result)
    }

    fn properties_begin(&mut self) -> geozero::error::Result<()> {
        self.values.iter_mut().for_each(|value| *value = None);
        let result = self.inner.properties_begin();
        self.track(result)
    }

    fn properties_end(&mut self) -> geozero::error::Result<()> {
        let mut result = Ok(());
        let mut idx = 0;
        for (name, value) in self.fields.iter().zip(&self.values) {
            let written = match value {
                Some(value) => self.inner.property(idx, name, &value.as_column_value()),
                None if self.fill_missing => {
                    self.inner.property(idx, name, &ColumnValue::String(""))
                },
                None => continue,
            };
            if let Err(err) = written {
                result = Err(err);
                break;
            }
            idx += 1;
        }
        let result = result.and_then(|()| self.inner.properties_end());
        self.track(result)
    }

    fn geometry_begin(&mut self) -> geozero::error::Result<()> {
        let result = self.inner.geometry_begin();
        self.track(result)
    }

    fn geometry_end(&mut self) -> geozero::error::Result<()> {
        let result = self.inner.geometry_end();
        self.track(result)
    }
}

impl<P: FeatureProcessor> GeomProcessor for DbfFieldOrder<'_, P> {
    fn dimensions(&self) -> CoordDimensions {
        self.inner.dimensions()
    }

    fn multi_dim(&self) -> bool {
        self.inner.multi_dim()
    }

    fn srid(&mut self, srid: Option<i32>) -> geozero::error::Result<()> {
        self.inner.srid(srid)
    }

    fn xy(&mut self, x: f64, y: f64, idx: usize) -> geozero::error::Result<()> {
        self.inner.xy(x, y, idx)
    }

    fn coordinate(
        &mut self,
        x: f64,
        y: f64,
        z: Option<f64>,
        m: Option<f64>,
        t: Option<f64>,
        tm: Option<u64>,
        idx: usize,
    ) -> geozero::error::Result<()> {
        self.inner.coordinate(x, y, z, m, t, tm, idx)
    }

    fn empty_point(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.empty_point(idx)
    }

    fn point_begin(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.point_begin(idx)
    }

    fn point_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.point_end(idx)
    }

    fn multipoint_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.multipoint_begin(size, idx)
    }

    fn multipoint_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.multipoint_end(idx)
    }

    fn linestring_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        self.inner.linestring_begin(tagged, size, idx)
    }

    fn linestring_end(&mut self, tagged: bool, idx: usize) -> geozero::error::Result<()> {
        self.inner.linestring_end(tagged, idx)
    }

    fn multilinestring_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.multilinestring_begin(size, idx)
    }

    fn multilinestring_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.multilinestring_end(idx)
    }

    fn polygon_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        self.inner.polygon_begin(tagged, size, idx)
    }

    fn polygon_end(&mut self, tagged: bool, idx: usize) -> geozero::error::Result<()> {
        self.inner.polygon_end(tagged, idx)
    }

    fn multipolygon_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.multipolygon_begin(size, idx)
    }

    fn multipolygon_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.multipolygon_end(idx)
    }

    fn geometrycollection_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.geometrycollection_begin(size, idx)
    }

    fn geometrycollection_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.geometrycollection_end(idx)
    }

    fn circularstring_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.circularstring_begin(size, idx)
    }

    fn circularstring_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.circularstring_end(idx)
    }

    fn compoundcurve_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.compoundcurve_begin(size, idx)
    }

    fn compoundcurve_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.compoundcurve_end(idx)
    }

    fn curvepolygon_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.curvepolygon_begin(size, idx)
    }

    fn curvepolygon_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.curvepolygon_end(idx)
    }

    fn multicurve_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.multicurve_begin(size, idx)
    }

    fn multicurve_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.multicurve_end(idx)
    }

    fn multisurface_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.multisurface_begin(size, idx)
    }

    fn multisurface_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.multisurface_end(idx)
    }

    fn triangle_begin(
        &mut self,
        tagged: bool,
        size: usize,
        idx: usize,
    ) -> geozero::error::Result<()> {
        self.inner.triangle_begin(tagged, size, idx)
    }

    fn triangle_end(&mut self, tagged: bool, idx: usize) -> geozero::error::Result<()> {
        self.inner.triangle_end(tagged, idx)
    }

    fn polyhedralsurface_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.polyhedralsurface_begin(size, idx)
    }

    fn polyhedralsurface_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.polyhedralsurface_end(idx)
    }

    fn tin_begin(&mut self, size: usize, idx: usize) -> geozero::error::Result<()> {
        self.inner.tin_begin(size, idx)
    }

    fn tin_end(&mut self, idx: usize) -> geozero::error::Result<()> {
        self.inner.tin_end(idx)
    }
}

/// Stream a shapefile's features into an output writer, with each feature's
/// properties in the .dbf's field order (see `DbfFieldOrder`).
fn convert_shp<T: Read + Seek, P: FeatureProcessor>(
    reader: geozero::shp::ShpReader<T>,
    writer: &mut P,
    fields: Vec<String>,
    fill_missing: bool,
) -> CliResult<()> {
    let mut ordered = DbfFieldOrder::new(writer, fields, fill_missing);
    for feature in reader.iter_features(&mut ordered)? {
        feature?;
    }
    ordered.take_error()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let max_length = args.flag_max_length;
    if max_length == Some(0) {
        return fail_incorrectusage_clierror!("--max-length must be greater than 0.");
    }
    if args.arg_input_format == InputFormat::Geojson
        && args.arg_output_format == OutputFormat::Geojson
    {
        return fail_clierror!("Converting GeoJSON to GeoJSON is not supported");
    }

    match args.arg_input_format {
        InputFormat::Geojson => {
            let mut buf_reader = open_input_reader(args.arg_input.as_deref())?;
            let mut wtr = create_writer(args.flag_output.as_deref())?;
            let mut geometry = geozero::geojson::GeoJsonReader(&mut buf_reader);

            match args.arg_output_format {
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            geometry.process(&mut CsvWriter::new(writer))?;
                            Ok(())
                        })?;
                    } else {
                        geometry.process(&mut CsvWriter::new(&mut wtr))?;
                    }
                },
                OutputFormat::Svg => {
                    geometry.process(&mut SvgWriter::new(&mut wtr, false))?;
                },
                OutputFormat::Geojsonl => {
                    geometry.process(&mut GeoJsonLineWriter::new(&mut wtr))?;
                },
                OutputFormat::Geojson => unreachable!("rejected above"),
            }
            Ok(wtr.flush()?)
        },
        InputFormat::Shp => {
            let shp_input_path = match args.arg_input.as_deref() {
                Some("-") | None => {
                    return fail_clierror!("SHP input argument must be a path to a .shp file.");
                },
                Some(p) => Path::new(p),
            };
            let shx_path = shp_input_path.with_extension("shx");
            let dbf_path = shp_input_path.with_extension("dbf");
            for component in [shp_input_path, shx_path.as_path(), dbf_path.as_path()] {
                if !component.is_file() {
                    return fail_clierror!(
                        "Shapefile component '{}' not found. A shapefile needs its .shp, .shx and \
                         .dbf files side by side.",
                        component.display()
                    );
                }
            }
            let prj_path = shp_input_path.with_extension("prj");
            if let Ok(prj) = fs::read_to_string(&prj_path)
                && let Some(crs) = projected_crs_name(&prj)
            {
                wwarn!(
                    "{}: the coordinates are in the projected CRS \"{crs}\", not WGS-84 \
                     longitude/latitude, and are output unchanged. GeoJSON requires WGS-84 - \
                     reproject first, e.g. ogr2ogr -t_srs EPSG:4326 out.shp {}",
                    prj_path.display(),
                    shp_input_path.display()
                );
            }

            let mut shp_buf_reader = BufReader::new(File::open(shp_input_path)?);
            let mut reader = geozero::shp::ShpReader::new(&mut shp_buf_reader)?;
            let mut shx_reader = BufReader::new(File::open(&shx_path)?);
            let mut dbf_reader = BufReader::new(File::open(&dbf_path)?);
            reader.add_index_source(&mut shx_reader)?;
            reader.add_dbf_source(&mut dbf_reader)?;
            let fields: Vec<String> = reader
                .dbf_fields()?
                .iter()
                .map(|field| field.name().to_string())
                .collect();
            let mut wtr = create_writer(args.flag_output.as_deref())?;

            // Features are streamed directly into wtr, so large shapefiles are
            // not buffered. Per-feature errors are propagated (fail-fast) so
            // malformed SHP records don't silently produce truncated output.
            match args.arg_output_format {
                OutputFormat::Geojson => {
                    convert_shp(reader, &mut GeoJsonWriter::new(&mut wtr), fields, false)?;
                },
                OutputFormat::Geojsonl => {
                    convert_shp(reader, &mut GeoJsonLineWriter::new(&mut wtr), fields, false)?;
                },
                OutputFormat::Svg => {
                    convert_shp(reader, &mut SvgWriter::new(&mut wtr, false), fields, false)?;
                },
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            convert_shp(reader, &mut CsvWriter::new(writer), fields, true)
                        })?;
                    } else {
                        convert_shp(reader, &mut CsvWriter::new(&mut wtr), fields, true)?;
                    }
                },
            }
            Ok(wtr.flush()?)
        },
        InputFormat::Csv => {
            let mut input = open_csv_input(&args)?;
            let mut wtr = create_writer(args.flag_output.as_deref())?;
            let mut stats = SkipStats::default();

            match args.arg_output_format {
                OutputFormat::Geojson => {
                    write_csv_features(&mut input, &mut GeoJsonWriter::new(&mut wtr), &mut stats)?;
                },
                OutputFormat::Geojsonl => {
                    write_csv_features(
                        &mut input,
                        &mut GeoJsonLineWriter::new(&mut wtr),
                        &mut stats,
                    )?;
                },
                OutputFormat::Svg => {
                    write_csv_features(
                        &mut input,
                        &mut SvgWriter::new(&mut wtr, false),
                        &mut stats,
                    )?;
                },
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            write_csv_features(&mut input, &mut CsvWriter::new(writer), &mut stats)
                        })?;
                    } else {
                        write_csv_features(&mut input, &mut CsvWriter::new(&mut wtr), &mut stats)?;
                    }
                },
            }
            wtr.flush()?;
            stats.report(&input.source)
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Regression for the prior `String::replace(".shp", ...)` bug: paths that
    /// contain `.shp` in a directory component must not have those occurrences
    /// rewritten when computing sibling extensions. `Path::with_extension`
    /// only touches the file extension, so `archive.shp.bak/data.shp` is
    /// resolved correctly.
    #[test]
    fn shp_sibling_paths_with_extension() {
        let p = Path::new("archive.shp.bak/data.shp");
        assert_eq!(
            p.with_extension("shx"),
            Path::new("archive.shp.bak/data.shx")
        );
        assert_eq!(
            p.with_extension("dbf"),
            Path::new("archive.shp.bak/data.dbf")
        );

        let p2 = Path::new("/tmp/dir/file.shp");
        assert_eq!(p2.with_extension("shx"), Path::new("/tmp/dir/file.shx"));
    }

    #[test]
    fn truncate_with_ellipsis_ascii() {
        assert_eq!(truncate_with_ellipsis("abcdefghij", 5), "abcde...");
    }

    #[test]
    fn truncate_with_ellipsis_multibyte() {
        // 'é' is 2 bytes (0xC3 0xA9). In "Café au lait" the byte layout is
        // C=0 a=1 f=2 é=3..5 (boundary at 5). max_len=4 falls inside 'é';
        // naive byte-slicing would panic. We must walk back to byte 3.
        let got = truncate_with_ellipsis("Café au lait", 4);
        assert_eq!(got, "Caf...");
    }

    #[test]
    fn truncate_with_ellipsis_em_dash() {
        // '—' is 3 bytes (U+2014). max_len=2 lands inside it; walk back to 1.
        let got = truncate_with_ellipsis("a—b", 2);
        assert_eq!(got, "a...");
    }
}
