static USAGE: &str = r#"
Convert between various spatial formats and CSV/SVG including GeoJSON, SHP, and more.

For example to convert a GeoJSON file into CSV data:

  $ qsv geoconvert file.geojson geojson csv

To use stdin as input instead of a file path, use a dash "-":

  $ qsv prompt -m "Choose a GeoJSON file" -F geojson | qsv geoconvert - geojson csv

To convert a CSV file into GeoJSON data, specify the WKT geometry column with the --geometry flag:

  $ qsv geoconvert file.csv csv geojson --geometry geometry

Alternatively specify the latitude and longitude columns with the --latitude and --longitude flags:

  $ qsv geoconvert file.csv csv geojson --latitude lat --longitude lon

Usage:
    qsv geoconvert [options] (<input>) (<input-format>) (<output-format>)
    qsv geoconvert --help

geoconvert REQUIRED arguments:
    <input>           The spatial file to convert. To use stdin instead, use a dash "-".
                      Note: SHP input must be a path to a .shp file and cannot use stdin.
    <input-format>    Valid values are "geojson", "shp", and "csv"
    <output-format>   Valid values are:
                      - For GeoJSON input: "csv", "svg", and "geojsonl"
                      - For SHP input: "csv", "geojson", and "geojsonl"
                      - For CSV input: "geojson", "geojsonl", and "svg"
                                       ("csv" only with --max-length, for truncation)

geoconvert options:
                                 REQUIRED FOR CSV INPUT
    -g, --geometry <geometry>    The name of the column that has WKT geometry.
                                 Alternative to --latitude and --longitude.
    -y, --latitude <col>         The name of the column with northing values.
    -x, --longitude <col>        The name of the column with easting values.

    -l, --max-length <length>    The maximum column length when the output format is CSV.
                                 Oftentimes, the geometry column is too long to fit in a
                                 CSV file, causing other tools like Python & PostgreSQL to fail.
                                 If a column is too long, it will be truncated to the specified
                                 length and an ellipsis ("...") will be appended.

Common options:
    -h, --help                   Display this message
    -o, --output <file>          Write output to <file> instead of stdout.
"#;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use csv::{Reader, Writer};
use geozero::{
    GeozeroDatasource,
    csv::CsvWriter,
    geojson::{GeoJsonLineWriter, GeoJsonWriter},
    svg::SvgWriter,
};
use serde::Deserialize;
use tempfile::NamedTempFile;

use crate::{CliError, CliResult, util};

/// Truncate a string at the nearest char boundary <= max_len bytes,
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

/// Helper function to handle CSV output with max_length truncation.
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

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let max_length = args.flag_max_length;
    if let Some(0) = max_length {
        return fail_incorrectusage_clierror!("--max-length must be greater than 0.");
    }

    // Output writer is shared across all input formats.
    let stdout = io::stdout();
    let mut wtr: Box<dyn Write> = if let Some(output_path) = args.flag_output {
        Box::new(BufWriter::new(File::create(output_path)?))
    } else {
        Box::new(BufWriter::new(stdout.lock()))
    };

    match args.arg_input_format {
        InputFormat::Geojson => {
            let mut buf_reader = open_input_reader(args.arg_input.as_deref())?;
            let mut geometry = geozero::geojson::GeoJsonReader(&mut buf_reader);

            match args.arg_output_format {
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            let mut processor = CsvWriter::new(writer);
                            geometry.process(&mut processor)?;
                            Ok(())
                        })?;
                        return Ok(());
                    }
                    let mut processor = CsvWriter::new(&mut wtr);
                    geometry.process(&mut processor)?;
                },
                OutputFormat::Svg => {
                    let mut processor = SvgWriter::new(&mut wtr, false);
                    geometry.process(&mut processor)?;
                },
                OutputFormat::Geojsonl => {
                    let mut processor = GeoJsonLineWriter::new(&mut wtr);
                    geometry.process(&mut processor)?;
                },
                OutputFormat::Geojson => {
                    return fail_clierror!("Converting GeoJSON to GeoJSON is not supported");
                },
            }
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
            let mut shp_buf_reader = BufReader::new(File::open(shp_input_path)?);
            let mut reader = geozero::shp::ShpReader::new(&mut shp_buf_reader)?;
            let mut shx_reader = BufReader::new(File::open(&shx_path)?);
            let mut dbf_reader = BufReader::new(File::open(&dbf_path)?);
            reader.add_index_source(&mut shx_reader)?;
            reader.add_dbf_source(&mut dbf_reader)?;

            // Stream features directly into wtr — avoids buffering the entire
            // output as Vec<u8> + UTF-8 round-trip for large shapefiles.
            // Per-feature errors are propagated (fail-fast) so malformed SHP
            // records don't silently produce truncated output.
            match args.arg_output_format {
                OutputFormat::Geojson => {
                    for feature in reader.iter_features(&mut GeoJsonWriter::new(&mut wtr))? {
                        feature?;
                    }
                },
                OutputFormat::Geojsonl => {
                    for feature in reader.iter_features(&mut GeoJsonLineWriter::new(&mut wtr))? {
                        feature?;
                    }
                },
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            for feature in reader.iter_features(&mut CsvWriter::new(writer))? {
                                feature?;
                            }
                            Ok(())
                        })?;
                        return Ok(());
                    }
                    for feature in reader.iter_features(&mut CsvWriter::new(&mut wtr))? {
                        feature?;
                    }
                },
                OutputFormat::Svg => {
                    return fail_clierror!("Converting SHP to SVG is not supported");
                },
            }
        },
        InputFormat::Csv => {
            // Validate flag combinations up front so users get a clear error
            // before we open the input.
            if args.flag_geometry.is_some()
                && (args.flag_latitude.is_some() || args.flag_longitude.is_some())
            {
                return fail_clierror!(
                    "Cannot use --geometry flag with --latitude or --longitude."
                );
            }
            if args.flag_geometry.is_none()
                && args.flag_latitude.is_some() != args.flag_longitude.is_some()
            {
                return fail_clierror!("--latitude and --longitude must be used together.");
            }

            let buf_reader = open_input_reader(args.arg_input.as_deref())?;

            if let Some(geometry_col) = args.flag_geometry {
                let mut csv_in = geozero::csv::CsvReader::new(&geometry_col, buf_reader);

                match args.arg_output_format {
                    OutputFormat::Geojson => {
                        let mut processor = GeoJsonWriter::new(&mut wtr);
                        csv_in.process(&mut processor)?;
                    },
                    OutputFormat::Geojsonl => {
                        let mut processor = GeoJsonLineWriter::new(&mut wtr);
                        csv_in.process(&mut processor)?;
                    },
                    OutputFormat::Svg => {
                        let mut processor = SvgWriter::new(&mut wtr, false);
                        csv_in.process(&mut processor)?;
                    },
                    OutputFormat::Csv => {
                        if let Some(max_len) = max_length {
                            process_csv_with_max_length(&mut wtr, max_len, |writer| {
                                let mut processor = CsvWriter::new(writer);
                                csv_in.process(&mut processor)?;
                                Ok(())
                            })?;
                            return Ok(());
                        }
                        return fail_clierror!(
                            "Converting CSV to CSV is only supported with --max-length."
                        );
                    },
                }
            } else if let (Some(y_col), Some(x_col)) = (args.flag_latitude, args.flag_longitude) {
                let mut rdr = csv::Reader::from_reader(buf_reader);
                let headers = rdr.headers()?.clone();
                let mut feature_collection =
                    serde_json::json!({"type": "FeatureCollection", "features": []});

                let latitude_col_index =
                    headers.iter().position(|y| y == y_col).ok_or_else(|| {
                        CliError::IncorrectUsage(format!("Latitude column '{y_col}' not found"))
                    })?;
                let longitude_col_index =
                    headers.iter().position(|x| x == x_col).ok_or_else(|| {
                        CliError::IncorrectUsage(format!("Longitude column '{x_col}' not found"))
                    })?;

                for result in rdr.records() {
                    let record = result?;
                    let mut feature =
                        serde_json::json!({"type": "Feature", "geometry": {}, "properties": {}});

                    let latitude_value = record
                        .get(latitude_col_index)
                        .ok_or_else(|| CliError::Other("Missing latitude value".to_string()))?
                        .parse::<f64>()
                        .map_err(|e| CliError::Other(format!("Invalid latitude value: {e}")))?;
                    let longitude_value = record
                        .get(longitude_col_index)
                        .ok_or_else(|| CliError::Other("Missing longitude value".to_string()))?
                        .parse::<f64>()
                        .map_err(|e| CliError::Other(format!("Invalid longitude value: {e}")))?;

                    let geometry_obj = feature
                        .get_mut("geometry")
                        .and_then(serde_json::Value::as_object_mut)
                        .ok_or_else(|| {
                            CliError::IncorrectUsage("Invalid geometry object".to_string())
                        })?;
                    geometry_obj.insert("type".to_string(), serde_json::Value::from("Point"));
                    // GeoJSON RFC 7946 §3.1.1: coordinates are [longitude, latitude].
                    geometry_obj.insert(
                        "coordinates".to_string(),
                        serde_json::Value::from(vec![longitude_value, latitude_value]),
                    );

                    let properties_obj = feature
                        .get_mut("properties")
                        .and_then(serde_json::Value::as_object_mut)
                        .ok_or_else(|| CliError::Other("Invalid properties object".to_string()))?;
                    for (index, value) in record.iter().enumerate() {
                        if index != longitude_col_index && index != latitude_col_index {
                            let new_key = headers
                                .get(index)
                                .ok_or_else(|| {
                                    CliError::Other(format!("Missing header at index {index}"))
                                })?
                                .to_string();
                            properties_obj.insert(new_key, serde_json::Value::from(value));
                        }
                    }

                    let features_array = feature_collection
                        .get_mut("features")
                        .and_then(serde_json::Value::as_array_mut)
                        .ok_or_else(|| CliError::Other("Invalid features array".to_string()))?;
                    features_array.push(feature);
                }

                let fc_string = feature_collection.to_string();
                let mut geometry = geozero::geojson::GeoJson(&fc_string);
                match args.arg_output_format {
                    OutputFormat::Csv => {
                        if let Some(max_len) = max_length {
                            process_csv_with_max_length(&mut wtr, max_len, |writer| {
                                let mut processor = CsvWriter::new(writer);
                                geometry.process(&mut processor)?;
                                Ok(())
                            })?;
                            return Ok(());
                        }
                        let mut processor = CsvWriter::new(&mut wtr);
                        geometry.process(&mut processor)?;
                    },
                    OutputFormat::Svg => {
                        let mut processor = SvgWriter::new(&mut wtr, false);
                        geometry.process(&mut processor)?;
                    },
                    OutputFormat::Geojsonl => {
                        let mut processor = GeoJsonLineWriter::new(&mut wtr);
                        geometry.process(&mut processor)?;
                    },
                    OutputFormat::Geojson => {
                        wtr.write_all(fc_string.as_bytes())?;
                    },
                }
            } else {
                return fail_clierror!(
                    "Please specify a geometry column with the --geometry option or \
                     longitude/latitude with the --latitude and --longitude options."
                );
            }
        },
    }

    Ok(wtr.flush()?)
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
