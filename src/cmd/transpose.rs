static USAGE: &str = r#"
Transpose the rows/columns of CSV data.

Note that by default this reads all of the CSV data into memory,
and will automatically go into multipass mode if the CSV is larger
than memory.

Usage:
    qsv transpose [options] [<input>]
    qsv transpose --help

transpose options:
    -m, --multipass        Process the transpose by making multiple
                           passes over the dataset. Useful for really
                           big datasets. Consumes memory relative to
                           the number of rows.
                           Note that in general it is faster to
                           process the transpose in memory.
    --long                 Convert wide-format CSV to long format.
                           The first column is treated as the "field"
                           identifier, and remaining columns are treated
                           as "attributes". Output format is three columns:
                           field, attribute, value. Empty values are skipped.
                           Useful for converting stats output and other
                           wide-format CSVs to long format for easier
                           processing, database storage, or FAIR metadata
                           specifications. Mutually exclusive with --multipass.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
                           Ignored when --multipass option is enabled.
"#;

use std::{fs::File, str};

use csv::ByteRecord;
use memmap2::MmapOptions;
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    util,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_multipass: bool,
    flag_long:      bool,
    flag_memcheck:  bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // --long and --multipass are mutually exclusive
    if args.flag_long && args.flag_multipass {
        return fail_incorrectusage_clierror!(
            "The --long and --multipass options are mutually exclusive."
        );
    }

    if args.flag_long {
        return args.wide_to_long();
    }

    let input_is_stdin = match args.arg_input {
        Some(ref s) if s == "-" => true,
        None => true,
        _ => false,
    };

    if args.flag_multipass && !input_is_stdin {
        args.multipass_transpose_streaming()
    } else {
        args.in_memory_transpose()
    }
}

impl Args {
    fn wide_to_long(&self) -> CliResult<()> {
        let mut rdr = Config::new((self).arg_input.as_ref())
            .delimiter((self).flag_delimiter)
            .no_headers(false)
            .reader()?;
        let mut wtr = self.wconfig().writer()?;

        // Read headers - first column is "field", rest are "attributes"
        let headers = rdr.byte_headers()?.clone();
        if headers.is_empty() {
            return fail_incorrectusage_clierror!("CSV file must have at least one column.");
        }

        // Write output headers
        let mut header_record = ByteRecord::with_capacity(64, 3);
        header_record.push_field(b"field");
        header_record.push_field(b"attribute");
        header_record.push_field(b"value");
        wtr.write_byte_record(&header_record)?;

        // Process each record in a streaming fashion to avoid loading all records into memory
        let mut output_record = ByteRecord::with_capacity(256, 3);
        for result in rdr.byte_records() {
            let record = result?;
            if record.is_empty() {
                continue;
            }

            // First column is the field identifier
            let field = &record[0];

            // Iterate through remaining columns (attributes)
            for (i, attribute_header) in headers.iter().enumerate().skip(1) {
                if i < record.len() {
                    let value = &record[i];
                    // Skip empty values
                    if !value.is_empty() {
                        output_record.clear();
                        output_record.push_field(field);
                        output_record.push_field(attribute_header);
                        output_record.push_field(value);
                        wtr.write_byte_record(&output_record)?;
                    }
                }
            }
        }

        Ok(wtr.flush()?)
    }

    fn in_memory_transpose(&self) -> CliResult<()> {
        // we're loading the entire file into memory, we need to check avail mem
        if let Some(path) = self.rconfig().path
            && let Err(e) = util::mem_file_check(&path, false, self.flag_memcheck)
        {
            eprintln!("File too large for in-memory transpose: {e}.\nDoing multipass transpose...");
            return self.multipass_transpose_streaming();
        }

        let mut rdr = self.rconfig().reader()?;
        let mut wtr = self.wconfig().writer()?;
        let nrows = rdr.byte_headers()?.len();

        let all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        let mut record = ByteRecord::with_capacity(1024, nrows);
        for i in 0..nrows {
            record.clear();
            for row in &all {
                record.push_field(&row[i]);
            }
            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }

    fn multipass_transpose_streaming(&self) -> CliResult<()> {
        // Get the number of columns from the first row
        let nrows = self.rconfig().reader()?.byte_headers()?.len();

        // Memory map the file for efficient access
        let file = File::open(self.arg_input.as_ref().unwrap())?;
        // safety: we know we have a file input at this stage
        let mmap = unsafe { MmapOptions::new().populate().map(&file)? };
        let mut wtr = self.wconfig().writer()?;

        let mut record = ByteRecord::with_capacity(1024, nrows);

        for i in 0..nrows {
            record.clear();

            // Create a reader from the memory-mapped data
            // this is more efficient for large files as we reduce I/O
            let mut rdr = self.rconfig().from_reader(&mmap[..]);

            // Read all rows for this column
            for row in rdr.byte_records() {
                let row = row?;
                if i < row.len() {
                    record.push_field(&row[i]);
                }
            }

            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }

    fn wconfig(&self) -> Config {
        Config::new(self.flag_output.as_ref()).set_write_buffer(DEFAULT_WTR_BUFFER_CAPACITY * 20)
    }

    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(true)
    }
}
