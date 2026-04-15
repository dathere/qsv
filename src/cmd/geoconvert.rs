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
                      - For CSV input: "geojson", "geojsonl", "csv", and "svg"

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
    env,
    fs::{self, File},
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

use crate::{CliError, CliResult, util};

/// Helper function to handle CSV output with max_length truncation
fn process_csv_with_max_length<F>(
    wtr: &mut Box<dyn Write>,
    max_len: usize,
    process_fn: F,
) -> CliResult<()>
where
    F: FnOnce(&mut Box<dyn Write>) -> CliResult<()>,
{
    // Create a temporary file for the CSV output
    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join(format!("qsv_geoconvert_{}.csv", uuid::Uuid::new_v4()));

    // Write the CSV output to the temporary file
    {
        let temp_file = File::create(&temp_file_path)?;
        let temp_writer = BufWriter::new(temp_file);
        let mut temp_box: Box<dyn Write> = Box::new(temp_writer);
        process_fn(&mut temp_box)?;
    } // temp_writer is dropped here, which will flush it

    // Read the temporary file and truncate columns that exceed the max length
    let mut rdr = Reader::from_path(&temp_file_path)?;
    let headers = rdr.headers()?.clone();

    // Create a new CSV writer for the final output
    let mut csv_writer = Writer::from_writer(wtr);
    csv_writer.write_record(&headers)?;

    // Process each record and truncate columns that exceed the max length
    for result in rdr.records() {
        let record = result?;
        let mut truncated_record = Vec::new();

        for value in &record {
            if value.len() > max_len {
                truncated_record.push(format!("{}...", &value[..max_len]));
            } else {
                truncated_record.push(value.to_string());
            }
        }

        csv_writer.write_record(&truncated_record)?;
    }

    // Clean up the temporary file
    fs::remove_file(temp_file_path)?;

    Ok(())
}

/// Supported input formats for spatial data conversion
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum InputFormat {
    Geojson,
    // Geojsonl,
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

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let max_length = args.flag_max_length;

    let mut buf_reader: Box<dyn BufRead> = if let Some(input_path) = args.arg_input.clone() {
        if &input_path == "-" {
            Box::new(BufReader::new(std::io::stdin()))
        } else {
            validate_input_file(&input_path)?;
            Box::new(BufReader::new(File::open(&input_path)?))
        }
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };
    // Create buffered writer for output
    let stdout = io::stdout();
    let mut wtr: Box<dyn Write> = if let Some(output_path) = args.flag_output {
        Box::new(BufWriter::new(File::create(output_path)?))
    } else {
        Box::new(BufWriter::new(stdout.lock()))
    };
    // Convert the input data to the specified output format
    match args.arg_input_format {
        InputFormat::Geojson => {
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
                    // If max_length is not set, write directly to the output
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
        // InputFormat::Geojsonl => {
        //     let mut geometry = geozero::geojson::GeoJsonLineReader::new(&mut buf_reader);
        //     match args.arg_output_format {
        //         OutputFormat::Csv => {
        //             let mut processor = CsvWriter::new(&mut wtr);
        //             geometry.process(&mut processor)?
        //         },
        //         OutputFormat::Svg => {
        //             let mut processor = SvgWriter::new(&mut wtr, false);
        //             geometry.process(&mut processor)?
        //         },
        //         OutputFormat::Geojson => {
        //             let mut processor = GeoJsonWriter::new(&mut wtr);
        //             geometry.process(&mut processor)?
        //         },
        //         OutputFormat::Geojsonl => {
        //             return fail_clierror!("Converting GeoJSON Lines to GeoJSON Lines is not
        // supported");         }
        //     };
        // },
        InputFormat::Shp => {
            let shp_input_path = if let Some(shp_input_path) = args.arg_input {
                if shp_input_path == "-" {
                    return fail_clierror!("SHP input argument must be a path to a .shp file.");
                }
                shp_input_path
            } else {
                return fail_clierror!("SHP input argument must be a path to a .shp file.");
            };
            let mut buf_reader = BufReader::new(File::open(&shp_input_path)?);
            let mut reader = geozero::shp::ShpReader::new(&mut buf_reader)?;
            let mut input_reader =
                BufReader::new(File::open(shp_input_path.replace(".shp", ".shx"))?);
            let mut dbf_reader =
                BufReader::new(File::open(shp_input_path.replace(".shp", ".dbf"))?);
            reader.add_index_source(&mut input_reader)?;
            reader.add_dbf_source(&mut dbf_reader)?;

            let output_string = match args.arg_output_format {
                OutputFormat::Geojson => {
                    let mut json: Vec<u8> = Vec::new();
                    let _ = reader
                        .iter_features(&mut GeoJsonWriter::new(&mut json))?
                        .collect::<Vec<_>>();
                    String::from_utf8(json)
                        .map_err(|e| CliError::Other(format!("Invalid UTF-8 in output: {e}")))?
                },
                OutputFormat::Geojsonl => {
                    let mut json: Vec<u8> = Vec::new();
                    let _ = reader
                        .iter_features(&mut GeoJsonLineWriter::new(&mut json))?
                        .collect::<Vec<_>>();
                    String::from_utf8(json)
                        .map_err(|e| CliError::Other(format!("Invalid UTF-8 in output: {e}")))?
                },
                OutputFormat::Csv => {
                    if let Some(max_len) = max_length {
                        process_csv_with_max_length(&mut wtr, max_len, |writer| {
                            let mut csv: Vec<u8> = Vec::new();
                            let _ = reader
                                .iter_features(&mut CsvWriter::new(&mut csv))?
                                .collect::<Vec<_>>();
                            writer.write_all(&csv)?;
                            Ok(())
                        })?;
                        return Ok(());
                    }
                    // If max_length is not set, write directly to the output
                    let mut csv: Vec<u8> = Vec::new();
                    let _ = reader
                        .iter_features(&mut CsvWriter::new(&mut csv))?
                        .collect::<Vec<_>>();
                    String::from_utf8(csv)
                        .map_err(|e| CliError::Other(format!("Invalid UTF-8 in output: {e}")))?
                },
                OutputFormat::Svg => {
                    return fail_clierror!("Converting SHP to SVG is not supported");
                },
            };

            // Only write to the output if we haven't already written to it
            if args.arg_output_format != OutputFormat::Csv || max_length.is_none() {
                wtr.write_all(output_string.as_bytes())?;
            }
        },
        InputFormat::Csv => {
            if args.flag_geometry.is_some()
                && (args.flag_latitude.is_some() || args.flag_longitude.is_some())
            {
                return fail_clierror!(
                    "Cannot use --geometry flag with --latitude or --longitude."
                );
            }
            if let Some(geometry_col) = args.flag_geometry {
                let mut csv = geozero::csv::CsvReader::new(&geometry_col, buf_reader);

                match args.arg_output_format {
                    OutputFormat::Geojson => {
                        let mut processor = GeoJsonWriter::new(&mut wtr);
                        csv.process(&mut processor)?;
                    },
                    OutputFormat::Geojsonl => {
                        let mut processor = GeoJsonLineWriter::new(&mut wtr);
                        csv.process(&mut processor)?;
                    },
                    OutputFormat::Svg => {
                        let mut processor = SvgWriter::new(&mut wtr, false);
                        csv.process(&mut processor)?;
                    },
                    OutputFormat::Csv => {
                        if let Some(max_len) = max_length {
                            process_csv_with_max_length(&mut wtr, max_len, |writer| {
                                let mut processor = CsvWriter::new(writer);
                                csv.process(&mut processor)?;
                                Ok(())
                            })?;
                            return Ok(());
                        }
                        return fail_clierror!("Converting CSV to CSV is not supported");
                    },
                }
            } else {
                if let Some(y_col) = args.flag_latitude
                    && let Some(x_col) = args.flag_longitude
                {
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
                            CliError::IncorrectUsage(format!(
                                "Longitude column '{x_col}' not found"
                            ))
                        })?;

                    for result in rdr.records() {
                        let record = result?;
                        let mut feature = serde_json::json!({"type": "Feature", "geometry": {}, "properties": {}});

                        // Add lat/lon coordinates geometry
                        let latitude_value = record
                            .get(latitude_col_index)
                            .ok_or_else(|| CliError::Other("Missing latitude value".to_string()))?
                            .parse::<f64>()
                            .map_err(|e| CliError::Other(format!("Invalid latitude value: {e}")))?;
                        let longitude_value = record
                            .get(longitude_col_index)
                            .ok_or_else(|| CliError::Other("Missing longitude value".to_string()))?
                            .parse::<f64>()
                            .map_err(|e| {
                                CliError::Other(format!("Invalid longitude value: {e}"))
                            })?;

                        let geometry = feature.get_mut("geometry").ok_or_else(|| {
                            CliError::IncorrectUsage("Missing geometry object".to_string())
                        })?;
                        let geometry_obj = geometry.as_object_mut().ok_or_else(|| {
                            CliError::IncorrectUsage("Invalid geometry object".to_string())
                        })?;
                        geometry_obj.insert("type".to_string(), serde_json::Value::from("Point"));
                        geometry_obj.insert(
                            "coordinates".to_string(),
                            serde_json::Value::from(vec![latitude_value, longitude_value]),
                        );

                        // Add properties
                        for (index, value) in record.iter().enumerate() {
                            if index != longitude_col_index && index != latitude_col_index {
                                let properties =
                                    feature.get_mut("properties").ok_or_else(|| {
                                        CliError::Other("Missing properties object".to_string())
                                    })?;
                                let properties_obj =
                                    properties.as_object_mut().ok_or_else(|| {
                                        CliError::Other("Invalid properties object".to_string())
                                    })?;
                                let new_key = headers
                                    .get(index)
                                    .ok_or_else(|| {
                                        CliError::Other(format!("Missing header at index {index}"))
                                    })?
                                    .to_string();
                                let new_value = serde_json::Value::from(value);
                                properties_obj.insert(new_key, new_value);
                            }
                        }

                        // Add Feature to FeatureCollection
                        let features = feature_collection
                            .get_mut("features")
                            .ok_or_else(|| CliError::Other("Missing features array".to_string()))?;
                        let features_array = features
                            .as_array_mut()
                            .ok_or_else(|| CliError::Other("Invalid features array".to_string()))?;
                        features_array.push(feature);
                    }

                    // Write FeatureCollection
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
                            // If max_length is not set, write directly to the output
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
                    return Ok(());
                }
                return fail_clierror!(
                    "Please specify a geometry column with the --geometry option or \
                     longitude/latitude with the --latitude and --longitude options."
                );
            }
        },
    }

    // wtr.write_all(output_string.as_bytes())?;
    Ok(wtr.flush()?)
}
