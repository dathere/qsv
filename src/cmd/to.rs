static USAGE: &str = r#"
Convert CSV files to Parquet, PostgreSQL, SQLite, Excel XLSX, ODS and Data Package.

PARQUET
=======
Convert CSV files to Parquet format. Each input CSV produces a separate .parquet file
in the specified output directory. The output directory will be created if it does not exist.

Requires the `polars` feature to be enabled.

Compression can be specified with --compression (default: zstd).
Supported values: zstd, gzip, snappy, lz4raw, uncompressed.
Use --compress-level to set the compression level for gzip (default: 6) or zstd (default: 3).

Examples:

Convert `file1.csv` and `file2.csv` to parquet files in output_dir/

  $ qsv to parquet output_dir file1.csv file2.csv

Convert all CSVs in a directory to parquet.

  $ qsv to parquet output_dir dir1

Convert files listed in the 'input.infile-list' to parquet.

  $ qsv to parquet output_dir input.infile-list

Convert with snappy compression.

  $ qsv to parquet output_dir --compression snappy file1.csv

Convert with zstd compression at level 10.

  $ qsv to parquet output_dir --compression zstd --compress-level 10 file1.csv

Convert from stdin with a custom filename.

  $ cat data.csv | qsv to parquet output_dir --table mydata -

POSTGRESQL
==========
To convert to postgres you need to supply connection string.
The format is described here - https://docs.rs/postgres/latest/postgres/config/struct.Config.html#examples-1.
Additionally you can use `env=MY_ENV_VAR` and qsv will get the connection string from the
environment variable `MY_ENV_VAR`.

If using the `--dump` option instead of a connection string put a name of a file or `-` for stdout.

Examples:

Load `file1.csv` and `file2.csv' file to local database `test`, with user `testuser`, and password `pass`.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' file1.csv file2.csv

Load same files into a new/existing postgres schema `myschema`

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --schema=myschema file1.csv file2.csv

Load same files into a new/existing postgres database whose connection string is in the
`DATABASE_URL` environment variable.

  $ qsv to postgres 'env=DATABASE_URL' file1.csv file2.csv

Load files inside a directory to a local database 'test' with user `testuser`, password `pass`.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' dir1

Load files listed in the 'input.infile-list' to a local database 'test' with user `testuser`, password `pass`.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' input.infile-list

Drop tables if they exist before loading.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --drop file1.csv file2.csv

Evolve tables if they exist before loading. Read http://datapackage_convert.opendata.coop/evolve.html
to explain how evolving works.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --evolve file1.csv file2.csv

Create dump file.

  $ qsv to postgres --dump dumpfile.sql file1.csv file2.csv

Print dump to stdout.

  $ qsv to postgres --dump - file1.csv file2.csv


SQLITE
======
Convert to sqlite db file. Will be created if it does not exist.

If using the `--dump` option, instead of a sqlite database file, put the name of the dump file or `-` for stdout.

Examples:

Load `file1.csv` and `file2.csv' files to sqlite database `test.db`

  $ qsv to sqlite test.db file1.csv file2.csv

Load all files in dir1 to sqlite database `test.db`

  $ qsv to sqlite test.db dir

Load files listed in the 'mydata.infile-list' to sqlite database `test.db`

  $ qsv to sqlite test.db mydata.infile-list

Drop tables if they exist before loading.

  $ qsv to sqlite test.db --drop file1.csv file2.csv

Evolve tables if they exist. Read http://datapackage_convert.opendata.coop/evolve.html
to explain how evolving is done.

  $ qsv to sqlite test.db --evolve file1.csv file2.csv

Create dump file .

  $ qsv to sqlite --dump dumpfile.sql file1.csv file2.csv

Print dump to stdout.

  $ qsv to sqlite --dump - file1.csv file2.csv


EXCEL XLSX
==========
Convert to new xlsx file.

Examples:

Load `file1.csv` and `file2.csv' into xlsx file.
Will create `output.xlsx`, creating new sheets for each file, with the sheet name being the
filename without the extension. Note the `output.xlsx` will be overwritten if it exists.

  $ qsv to xlsx output.xlsx file1.csv file2.csv

Load all files in dir1 into xlsx file.

  $ qsv to xlsx output.xlsx dir1

Load files listed in the 'ourdata.infile-list' into xlsx file.

  $ qsv to xlsx output.xlsx ourdata.infile-list

Load a single CSV into xlsx with a custom sheet name.

  $ qsv to xlsx output.xlsx --table "Sales Data" file1.csv

Load from stdin with a custom sheet name.

  $ cat data.csv | qsv to xlsx output.xlsx --table "Monthly Report" -

ODS
===
Convert to new ODS (Open Document Spreadsheet) file.

Examples:

Load `file1.csv` and `file2.csv' into ODS file.
Will create `output.ods`, creating new sheets for each file, with the sheet name being the
filename without the extension. Note the `output.ods` will be overwritten if it exists.

  $ qsv to ods output.ods file1.csv file2.csv

Load all files in dir1 into ODS file.

  $ qsv to ods output.ods dir1

Load files listed in the 'ourdata.infile-list' into ODS file.

  $ qsv to ods output.ods ourdata.infile-list

Load a single CSV into ODS with a custom sheet name.

  $ qsv to ods output.ods --table "Sales Data" file1.csv

Load from stdin with a custom sheet name.

  $ cat data.csv | qsv to ods output.ods --table "Monthly Report" -

DATA PACKAGE
============
Generate a datapackage, which contains stats and information about what is in the CSV files.

Examples:

Generate a `datapackage.json` file from `file1.csv` and `file2.csv' files.

  $ qsv to datapackage datapackage.json file1.csv file2.csv

Add more stats to datapackage.

  $ qsv to datapackage datapackage.json --stats file1.csv file2.csv

Generate a `datapackage.json` file from all the files in dir1

  $ qsv to datapackage datapackage.json dir1

Generate a `datapackage.json` file from all the files listed in the 'data.infile-list'

  $ qsv to datapackage datapackage.json data.infile-list

For all other conversions you can output the datapackage created by specifying `--print-package`.

  $ qsv to xlsx datapackage.xlsx --stats --print-package file1.csv file2.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_to.rs.

Usage:
    qsv to parquet [options] <destination> [<input>...]
    qsv to postgres [options] <destination> [<input>...]
    qsv to sqlite [options] <destination> [<input>...]
    qsv to xlsx [options] <destination> [<input>...]
    qsv to ods [options] <destination> [<input>...]
    qsv to datapackage [options] <destination> [<input>...]
    qsv to --help

To arguments:
    <destination>           The output target, which varies by subcommand:
                            * parquet: output directory (created if needed)
                            * postgres: connection string or env=VAR_NAME (with --dump: dump file path or - for stdout)
                            * sqlite: database file path (with --dump: dump file path or - for stdout)
                            * xlsx: output .xlsx file path
                            * ods: output .ods file path
                            * datapackage: output .json file path
    <input>...              Input CSV file(s) to convert. Can be file path(s), a directory,
                            an .infile-list file, or `-` for stdin (not supported by
                            parquet subcommand).

To options:
  -k, --print-package     Print statistics as datapackage, by default will print field summary.
  -u, --dump              Create database dump file for use with `psql` or `sqlite3` command line tools
                          (postgres/sqlite only).
  -a, --stats             Produce extra statistics about the data beyond just type guessing.
  -c, --stats-csv <path>  Output stats as CSV to specified file.
  -q, --quiet             Do not print out field summary.
  -s, --schema <arg>      The schema to load the data into. (postgres only).
  --infer-len <rows>      The number of rows to use for schema inference (parquet only).
                          Note that even if a pschema.json file exists for an input file,
                          explicitly specifying infer-len will cause qsv to ignore the pschema.json and
                          infer the schema from the CSV data instead, including when set to 0.
                          Set to 0 to infer from all rows (not recommended for large files).
  --try-parse-dates       Attempt to parse date/datetime columns with polars' date inference logic.
                          This may result in more accurate date parsing, but can be slower on large files.
                          (parquet only).
  -d, --drop              Drop tables before loading new data into them (postgres/sqlite only).
  -e, --evolve            If loading into existing db, alter existing tables so that new data will load.
                          (postgres/sqlite only).
  -i, --pipe              Adjust output format for piped data (omits row counts and field format columns).
  -t, --table <name>      Use this as the table/sheet/file name (postgres/sqlite/xlsx/ods/parquet).
                          Overrides the default name derived from the input filename.
                          When reading from stdin, the default table name is "stdin".
                          Only valid with a single input file.
                          For postgres/sqlite: must start with a letter or underscore,
                          contain only alphanumeric characters and underscores (max 63).
                          For xlsx/ods: used as sheet name (max 31 chars,
                          cannot contain \ / * [ ] : ?).
  -p, --separator <arg>   For xlsx, use this character to help truncate xlsx sheet names.
                          Defaults to space.
      --compression <arg>  Parquet compression codec (parquet only).
                           Valid values: zstd (default), gzip, snappy, lz4raw, uncompressed.
      --compress-level <arg>  Compression level (parquet only).
                              For gzip: 1-9 (default: 6). For zstd: -7 to 22 (default: 3).
                              Ignored for other codecs.
  -A, --all-strings       Convert all fields to strings.
  -j, --jobs <arg>        The number of jobs to run in parallel.
                          When not set, the number of jobs is set to the number of CPUs detected.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

#[cfg(feature = "polars")]
use std::io::{BufReader, Read};
use std::{io::Write, path::PathBuf};

use csvs_convert::{
    DescribeOptions, Options, csvs_to_ods_with_options, csvs_to_postgres_with_options,
    csvs_to_sqlite_with_options, csvs_to_xlsx_with_options, make_datapackage,
};
use log::debug;
#[cfg(feature = "polars")]
use polars::{
    polars_utils::compression::{GzipLevel, ZstdLevel},
    prelude::*,
};
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    config::{self, Delimiter},
    util,
    util::process_input,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    cmd_postgres:         bool,
    cmd_sqlite:           bool,
    cmd_xlsx:             bool,
    cmd_ods:              bool,
    cmd_parquet:          bool,
    cmd_datapackage:      bool,
    arg_destination:      Option<String>,
    arg_input:            Vec<PathBuf>,
    flag_delimiter:       Option<Delimiter>,
    flag_schema:          Option<String>,
    flag_infer_len:       Option<usize>,
    flag_try_parse_dates: bool,
    flag_separator:       Option<String>,
    flag_all_strings:     bool,
    flag_dump:            bool,
    flag_drop:            bool,
    flag_evolve:          bool,
    flag_stats:           bool,
    flag_stats_csv:       Option<String>,
    flag_jobs:            Option<usize>,
    flag_table:           Option<String>,
    flag_compression:     Option<String>,
    flag_compress_level:  Option<i32>,
    flag_print_package:   bool,
    flag_quiet:           bool,
    flag_pipe:            bool,
}

impl From<csvs_convert::Error> for CliError {
    fn from(err: csvs_convert::Error) -> CliError {
        CliError::Other(format!("Conversion error: {err:?}"))
    }
}

impl From<csvs_convert::DescribeError> for CliError {
    fn from(err: csvs_convert::DescribeError) -> CliError {
        CliError::Other(format!("Conversion error: {err:?}"))
    }
}

static EMPTY_STDIN_ERRMSG: &str =
    "No data on stdin. Need to add connection string as first argument then the input CSVs";

#[cfg(feature = "polars")]
static DEFAULT_GZIP_COMPRESSION_LEVEL: u8 = 6;
#[cfg(feature = "polars")]
static DEFAULT_ZSTD_COMPRESSION_LEVEL: i32 = 3;

#[cfg(feature = "polars")]
#[derive(Default, Copy, Clone)]
enum PqtCompression {
    Uncompressed,
    Gzip,
    Snappy,
    #[default]
    Zstd,
    Lz4Raw,
}

#[cfg(feature = "polars")]
impl std::str::FromStr for PqtCompression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "uncompressed" => Ok(PqtCompression::Uncompressed),
            "gzip" => Ok(PqtCompression::Gzip),
            "snappy" => Ok(PqtCompression::Snappy),
            "lz4raw" => Ok(PqtCompression::Lz4Raw),
            "zstd" | "" => Ok(PqtCompression::Zstd),
            _ => Err(format!("Invalid Parquet compression format: {s}")),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    debug!("'to' command running");
    let mut options = Options::builder()
        .delimiter(args.flag_delimiter.map(config::Delimiter::as_byte))
        .schema(args.flag_schema.unwrap_or_default())
        .seperator(args.flag_separator.unwrap_or_else(|| " ".into()))
        .all_strings(args.flag_all_strings)
        .evolve(args.flag_evolve)
        .stats(args.flag_stats)
        .pipe(args.flag_pipe)
        .stats_csv(args.flag_stats_csv.unwrap_or_default())
        .drop(args.flag_drop)
        .threads(util::njobs(args.flag_jobs))
        .build();

    let output;
    let mut arg_input = args.arg_input.clone();
    let tmpdir = tempfile::tempdir()?;

    // validate --table option
    if let Some(ref table_name) = args.flag_table {
        if args.cmd_datapackage {
            return fail_incorrectusage_clierror!(
                "--table cannot be used with the datapackage subcommand."
            );
        }
        if args.cmd_parquet {
            // parquet uses table name as output filename — disallow path separators
            // and Windows-invalid filename characters
            if table_name
                .contains(&['/', '\\', '\0', ':', '*', '?', '"', '<', '>', '|', '[', ']'][..])
            {
                return fail_incorrectusage_clierror!(
                    "--table name cannot contain path separators or special characters (/ \\ : * \
                     ? \" < > | [ ]) for parquet."
                );
            }
        }
        if table_name.is_empty() {
            return fail_incorrectusage_clierror!("--table name must not be empty.");
        }
        if args.cmd_xlsx || args.cmd_ods {
            // xlsx/ods sheet name validation
            // Both xlsx (Excel) and ods (ODS) enforce a 31-character limit on sheet names
            if table_name.chars().count() > 31 {
                return fail_incorrectusage_clierror!(
                    "--table sheet name must not exceed 31 characters for xlsx/ods."
                );
            }
            // Also ensure the sheet name is valid when used as a filesystem filename
            // (Windows-invalid filename characters: < > : \" / \\ | ? *)
            if table_name.contains(&['\\', '/', '*', '[', ']', ':', '?', '<', '>', '"', '|'][..]) {
                return fail_incorrectusage_clierror!(
                    "--table sheet name cannot contain \\ / * [ ] : ? < > \" | characters."
                );
            }
        } else {
            // postgres/sqlite table name validation
            if !table_name.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                return fail_incorrectusage_clierror!(
                    "--table name must start with a letter or underscore."
                );
            }
            if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return fail_incorrectusage_clierror!(
                    "--table name must contain only alphanumeric characters and underscores."
                );
            }
            // PostgreSQL limits identifiers to 63 characters; cap table names accordingly
            if table_name.len() > 63 {
                return fail_incorrectusage_clierror!(
                    "--table name must not exceed 63 characters (PostgreSQL identifier limit)."
                );
            }
        }
    }

    if args.cmd_postgres {
        debug!("converting to PostgreSQL");
        arg_input = process_input(arg_input, &tmpdir, EMPTY_STDIN_ERRMSG)?;
        apply_table_rename(args.flag_table.as_ref(), &mut arg_input, &tmpdir)?;
        if args.flag_dump {
            options.dump_file = args.arg_destination.expect("checked above");
            output = csvs_to_postgres_with_options(String::new(), arg_input, options)?;
        } else {
            output = csvs_to_postgres_with_options(
                args.arg_destination.expect("checked above"),
                arg_input,
                options,
            )?;
        }
        debug!("conversion to PostgreSQL complete");
    } else if args.cmd_sqlite {
        debug!("converting to SQLite");
        arg_input = process_input(arg_input, &tmpdir, EMPTY_STDIN_ERRMSG)?;
        apply_table_rename(args.flag_table.as_ref(), &mut arg_input, &tmpdir)?;
        if args.flag_dump {
            options.dump_file = args.arg_destination.expect("checked above");
            output = csvs_to_sqlite_with_options(String::new(), arg_input, options)?;
        } else {
            output = csvs_to_sqlite_with_options(
                args.arg_destination.expect("checked above"),
                arg_input,
                options,
            )?;
        }
        debug!("conversion to SQLite complete");
    } else if args.cmd_xlsx {
        debug!("converting to Excel XLSX");
        arg_input = process_input(arg_input, &tmpdir, EMPTY_STDIN_ERRMSG)?;
        apply_table_rename(args.flag_table.as_ref(), &mut arg_input, &tmpdir)?;

        output = csvs_to_xlsx_with_options(
            args.arg_destination.expect("checked above"),
            arg_input,
            options,
        )?;
        debug!("conversion to Excel XLSX complete");
    } else if args.cmd_ods {
        debug!("converting to ODS");
        arg_input = process_input(arg_input, &tmpdir, EMPTY_STDIN_ERRMSG)?;
        apply_table_rename(args.flag_table.as_ref(), &mut arg_input, &tmpdir)?;

        output = csvs_to_ods_with_options(
            args.arg_destination.expect("checked above"),
            arg_input,
            options,
        )?;
        debug!("conversion to ODS complete");
    } else if args.cmd_parquet {
        debug!("converting to Parquet");
        arg_input = process_input(arg_input, &tmpdir, "")?;
        apply_table_rename(args.flag_table.as_ref(), &mut arg_input, &tmpdir)?;
        return to_parquet(
            args.arg_destination.as_ref().expect("checked above"),
            arg_input,
            args.flag_delimiter,
            args.flag_compression,
            args.flag_compress_level,
            args.flag_all_strings,
            args.flag_infer_len,
            args.flag_try_parse_dates,
            args.flag_quiet,
        );
    } else if args.cmd_datapackage {
        debug!("creating Data Package");
        arg_input = process_input(arg_input, &tmpdir, EMPTY_STDIN_ERRMSG)?;

        let describe_options = DescribeOptions::builder()
            .delimiter(options.delimiter)
            .stats(options.stats)
            .threads(options.threads)
            .stats_csv(options.stats_csv);
        output = make_datapackage(arg_input, PathBuf::new(), &describe_options.build())?;
        let file = std::fs::File::create(args.arg_destination.expect("checked above"))?;
        serde_json::to_writer_pretty(file, &output)?;
        debug!("Data Package complete");
    } else {
        return fail_clierror!(
            "Need to supply either parquet, xlsx, ods, postgres, sqlite, datapackage as subcommand"
        );
    }

    if args.flag_print_package {
        println!(
            "{}",
            simd_json::to_string_pretty(&output).expect("values should be serializable")
        );
    } else if !args.flag_quiet && !args.flag_dump {
        let empty_array = vec![];
        for resource in output["resources"].as_array().unwrap_or(&empty_array) {
            let mut stdout = std::io::stdout();
            writeln!(&mut stdout)?;
            if args.flag_pipe {
                writeln!(
                    &mut stdout,
                    "Table '{}'",
                    resource["name"].as_str().unwrap_or("")
                )?;
            } else {
                writeln!(
                    &mut stdout,
                    "Table '{}' ({} rows)",
                    resource["name"].as_str().unwrap_or(""),
                    resource["row_count"].as_i64().unwrap_or(0)
                )?;
            }

            writeln!(&mut stdout)?;

            let mut tabwriter = qsv_tabwriter::TabWriter::new(stdout);

            if args.flag_pipe {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    ["Field Name", "Field Type"].join("\t")
                )?;
            } else {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    ["Field Name", "Field Type", "Field Format"].join("\t")
                )?;
            }

            for field in resource["schema"]["fields"]
                .as_array()
                .unwrap_or(&empty_array)
            {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    [
                        field["name"].as_str().unwrap_or(""),
                        field["type"].as_str().unwrap_or(""),
                        field["format"].as_str().unwrap_or("")
                    ]
                    .join("\t")
                )?;
            }
            tabwriter.flush()?;
        }
        let mut stdout = std::io::stdout();
        writeln!(&mut stdout)?;
    }

    Ok(())
}

/// If `--table` is set, copy the input file to a temp directory with the desired name
/// so that the downstream library derives the desired table name from the filename.
/// We copy instead of rename to avoid modifying the user's original file.
fn apply_table_rename(
    flag_table: Option<&String>,
    arg_input: &mut [std::path::PathBuf],
    tmpdir: &tempfile::TempDir,
) -> CliResult<()> {
    if let Some(table_name) = flag_table {
        if arg_input.len() != 1 {
            return fail_incorrectusage_clierror!(
                "--table can only be used with a single input file."
            );
        }
        let path = &mut arg_input[0];
        let extension = path
            .extension()
            .map_or("csv", |ext| ext.to_str().unwrap_or("csv"));
        let new_path = tmpdir.path().join(format!("{table_name}.{extension}"));
        std::fs::copy(&*path, &new_path)?;
        *path = new_path;
    }
    Ok(())
}

#[cfg(feature = "polars")]
fn to_parquet(
    destination: &str,
    arg_input: Vec<PathBuf>,
    flag_delimiter: Option<Delimiter>,
    flag_compression: Option<String>,
    flag_compress_level: Option<i32>,
    flag_all_strings: bool,
    flag_infer_len: Option<usize>,
    flag_try_parse_dates: bool,
    quiet: bool,
) -> CliResult<()> {
    let output_dir = PathBuf::from(&destination);
    std::fs::create_dir_all(&output_dir)?;

    // Parse compression codec
    let compression_str = flag_compression.unwrap_or_default();
    let compression: PqtCompression = match compression_str.parse() {
        Ok(compression) => compression,
        Err(_e) => {
            return fail_incorrectusage_clierror!(
                "invalid --compression value '{compression_str}'. Valid codecs are: uncompressed, \
                 snappy, lz4raw, gzip, zstd."
            );
        },
    };

    let parquet_compression = match compression {
        PqtCompression::Uncompressed => ParquetCompression::Uncompressed,
        PqtCompression::Snappy => ParquetCompression::Snappy,
        PqtCompression::Lz4Raw => ParquetCompression::Lz4Raw,
        PqtCompression::Gzip => {
            let level =
                flag_compress_level.unwrap_or_else(|| DEFAULT_GZIP_COMPRESSION_LEVEL.into());
            if !(1..=9).contains(&level) {
                return fail_incorrectusage_clierror!(
                    "invalid gzip compression level {level}. Valid values are 1 through 9."
                );
            }
            ParquetCompression::Gzip(Some(GzipLevel::try_new(level as u8)?))
        },
        PqtCompression::Zstd => {
            let level = flag_compress_level.unwrap_or(DEFAULT_ZSTD_COMPRESSION_LEVEL);
            if !(-7..=22).contains(&level) {
                return fail_incorrectusage_clierror!(
                    "invalid zstd compression level {level}. Valid values are -7 through 22."
                );
            }
            ParquetCompression::Zstd(Some(ZstdLevel::try_new(level)?))
        },
    };

    let delimiter = flag_delimiter.map_or(b',', config::Delimiter::as_byte);

    for input_path in arg_input {
        let filestem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let output_path = output_dir.join(format!("{filestem}.parquet"));

        if output_path.exists() {
            return fail_clierror!(
                "Output file '{}' already exists. Cannot overwrite.",
                output_path.display()
            );
        }

        debug!(
            "converting {} to {}",
            input_path.display(),
            output_path.display()
        );

        // Use LazyFrame for optimized query planning and efficient column casting
        let input_path_str = input_path.to_string_lossy();
        let mut lazy_csv_reader = LazyCsvReader::new(PlRefPath::new(&*input_path_str))
            .with_has_header(true)
            .with_separator(delimiter);

        if let Some(infer_len) = flag_infer_len {
            // if --infer-len is explicitly set (even to 0), ignore existing schema file
            let infer_len = match infer_len {
                0 => None, // 0 means scan all rows
                some_len => Some(some_len),
            };
            lazy_csv_reader = lazy_csv_reader.with_infer_schema_length(infer_len);
        } else {
            // Check if a .pschema.json schema file exists and is current
            let schema_file = PathBuf::from(format!(
                "{}.pschema.json",
                input_path.canonicalize()?.display()
            ));
            let valid_schema_exists = schema_file.exists()
                && schema_file.metadata()?.modified()? >= input_path.metadata()?.modified()?;

            if valid_schema_exists {
                let file = std::fs::File::open(&schema_file)?;
                let mut buf_reader = BufReader::new(file);
                let mut schema_json = String::with_capacity(100);
                buf_reader.read_to_string(&mut schema_json)?;
                let schema: Schema = serde_json::from_str(&schema_json)?;
                debug!("using schema file: {}", schema_file.display());
                lazy_csv_reader = lazy_csv_reader.with_schema(Some(Arc::new(schema)));
            } else {
                lazy_csv_reader = lazy_csv_reader.with_infer_schema_length(Some(1000));
            }
        }

        let mut lf = lazy_csv_reader
            .with_try_parse_dates(flag_try_parse_dates)
            .finish()?;

        if flag_all_strings {
            lf = lf.with_columns([col("*").cast(DataType::String)]);
        }

        let mut df = lf.collect()?;
        let row_count = df.height();

        let file = std::fs::File::create(&output_path)?;
        ParquetWriter::new(file)
            .with_row_group_size(Some(768 * 768))
            .with_statistics(StatisticsOptions {
                min_value: true,
                max_value: true,
                distinct_count: true,
                null_count: true,
                binary_statistics_truncate_length: Some(64),
            })
            .with_compression(parquet_compression)
            .finish(&mut df)?;

        if !quiet {
            eprintln!("Wrote '{filestem}.parquet' ({row_count} rows)");
        }

        debug!("wrote {}", output_path.display());
    }

    Ok(())
}

#[cfg(not(feature = "polars"))]
fn to_parquet(
    _destination: &str,
    _arg_input: Vec<PathBuf>,
    _flag_delimiter: Option<Delimiter>,
    _flag_compression: Option<String>,
    _flag_compress_level: Option<i32>,
    _flag_all_strings: bool,
    _flag_infer_len: Option<usize>,
    _flag_try_parse_dates: bool,
    _quiet: bool,
) -> CliResult<()> {
    fail_clierror!(
        "The parquet subcommand requires the 'polars' feature.\nPlease install qsv with the \
         'polars' feature enabled."
    )
}
