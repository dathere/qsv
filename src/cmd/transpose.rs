static USAGE: &str = r#"
Transpose the rows/columns of CSV data.

Usage:
    qsv transpose [options] [<input>]
    qsv transpose --help

Examples:
    # Transpose data in-memory.
    $ qsv transpose data.csv

    # Transpose data using multiple passes. For large datasets.
    $ qsv transpose data.csv --multipass

    # Convert CSV to "long" format using the first column as the "field" identifier
    $ qsv transpose data.csv --long 1

    # use the columns "name" & "age" as the "field" identifier
    $ qsv transpose --long "name,age" data.csv

    # use the columns 1 & 3 as the "field" identifier
    $ qsv transpose --long 1,3 data.csv

    # use the columns 1 to 3 as the "field" identifier
    $ qsv transpose --long 1-3 data.csv

    # use all columns starting with "name" as the "field" identifier
    $ qsv transpose --long /^name/ data.csv

See https://github.com/dathere/qsv/blob/master/tests/test_transpose.rs for more examples.

transpose options:
    -m, --multipass        Process the transpose by making multiple passes
                           over the dataset. Consumes memory relative to
                           the number of rows.
                           Note that in general it is faster to
                           process the transpose in memory.
                           Useful for really big datasets as the default
                           is to read the entire dataset into memory.
    -s, --select <arg>     Select a subset of columns to transpose.
                           When used with --long, this filters which columns
                           become attribute rows (the field columns are unaffected).
                           See 'qsv select --help' for the full selection syntax.
    --long <selection>     Convert wide-format CSV to "long" format.
                           Output format is three columns:
                           field, attribute, value. Empty values are skipped.
                           Mutually exclusive with --multipass.
                           
                           The <selection> argument is REQUIRED when using --long,
                           it specifies which column(s) to use as the "field" identifier.
                           It uses the same selection syntax as 'qsv select':
                           * Column names: --long varname or --long "column name"
                           * Column indices (1-based): --long 5 or --long 2,3
                           * Ranges: --long 1-4 or --long 3-
                           * Regex patterns: --long /^prefix/
                           * Comma-separated: --long var1,var2 or --long 1,3,5
                           Multiple field columns are concatenated with | separator.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
                           Ignored when --multipass or --long option is enabled.
"#;

use std::{fs::File, str};

use csv::ByteRecord;
use foldhash::HashSet;
use memmap2::MmapOptions;
use serde::Deserialize;

use crate::{
    CliError, CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    select::SelectColumns,
    util,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_multipass: bool,
    flag_select:    Option<SelectColumns>,
    flag_long:      Option<String>,
    flag_memcheck:  bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // --long and --multipass are mutually exclusive
    if args.flag_long.is_some() && args.flag_multipass {
        return fail_incorrectusage_clierror!(
            "The --long and --multipass options are mutually exclusive."
        );
    }

    if args.flag_long.is_some() {
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
        let mut rdr = Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(false)
            .reader()?;
        let mut wtr = self.wconfig().writer()?;

        // Read headers
        let headers = rdr.byte_headers()?.clone();
        if headers.is_empty() {
            return fail_incorrectusage_clierror!("CSV file must have at least one column.");
        }

        // Determine which columns to use as field columns
        let field_column_indices: Vec<usize> = if let Some(ref selection_str) = self.flag_long {
            let select_cols = SelectColumns::parse(selection_str)
                .map_err(|e| CliError::Other(format!("Invalid column selection: {e}")))?;
            let selection = select_cols
                .selection(&headers, true)
                .map_err(|e| CliError::Other(format!("Column selection error: {e}")))?;
            if selection.is_empty() {
                return fail_incorrectusage_clierror!(
                    "Column selection resulted in no columns. At least one field column is \
                     required."
                );
            }
            selection.iter().copied().collect()
        } else {
            unreachable!("Should not happen as docopt --long <selection> is required.");
        };

        // Create a set of field column indices for efficient lookup
        let field_column_set: HashSet<usize> = field_column_indices.iter().copied().collect();

        // Determine selected attribute columns if --select is specified
        // --select filters which columns become attribute rows (field columns are unaffected)
        let selected_attribute_set: Option<HashSet<usize>> = if let Some(ref sel) = self.flag_select
        {
            let selection = sel
                .selection(&headers, true)
                .map_err(|e| CliError::Other(format!("Column selection error: {e}")))?;
            if selection.is_empty() {
                return fail_incorrectusage_clierror!(
                    "--select resulted in no columns to transpose."
                );
            }
            Some(selection.iter().copied().collect())
        } else {
            None
        };

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

            // Build the field value (concatenated if multiple columns)
            let field_bytes: Vec<u8> = if field_column_indices.len() == 1 {
                // Single field column - use directly
                let idx = field_column_indices[0];
                if idx < record.len() {
                    record[idx].to_vec()
                } else {
                    Vec::new()
                }
            } else {
                // Multiple field columns - concatenate with | separator
                let mut concatenated = Vec::new();
                for (i, &idx) in field_column_indices.iter().enumerate() {
                    if i > 0 {
                        concatenated.push(b'|');
                    }
                    if idx < record.len() {
                        concatenated.extend_from_slice(&record[idx]);
                    }
                }
                concatenated
            };

            // Iterate through all columns, skipping field columns and non-selected columns
            for (i, attribute_header) in headers.iter().enumerate() {
                // Skip if this is a field column
                if field_column_set.contains(&i) {
                    continue;
                }
                // Skip if --select is specified and this column is not in the selection
                if let Some(ref sel_set) = selected_attribute_set
                    && !sel_set.contains(&i)
                {
                    continue;
                }
                if i < record.len() {
                    let value = &record[i];
                    // Skip empty values
                    if !value.is_empty() {
                        output_record.clear();
                        output_record.push_field(&field_bytes);
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

        // Get selected column indices if --select is specified
        let selected_indices = self.get_selected_indices()?;

        let mut rdr = self.rconfig().reader()?;
        let mut wtr = self.wconfig().writer()?;
        let nrows = rdr.byte_headers()?.len();

        let all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;

        // Determine which column indices to transpose
        let indices: Vec<usize> = match &selected_indices {
            Some(sel) => sel.clone(),
            None => (0..nrows).collect(),
        };

        let mut record = ByteRecord::with_capacity(1024, all.len());
        for i in indices {
            record.clear();
            for row in &all {
                if i < row.len() {
                    record.push_field(&row[i]);
                }
            }
            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }

    fn multipass_transpose_streaming(&self) -> CliResult<()> {
        // Get selected column indices if --select is specified
        let selected_indices = self.get_selected_indices()?;

        // Get the number of columns from the first row
        let nrows = self.rconfig().reader()?.byte_headers()?.len();

        // Determine which column indices to transpose
        let indices: Vec<usize> = match &selected_indices {
            Some(sel) => sel.clone(),
            None => (0..nrows).collect(),
        };

        // Memory map the file for efficient access
        let file = File::open(self.arg_input.as_ref().unwrap())?;
        // safety: we know we have a file input at this stage
        let mmap = unsafe { MmapOptions::new().populate().map(&file)? };
        let mut wtr = self.wconfig().writer()?;

        let mut record = ByteRecord::with_capacity(1024, nrows);

        for i in indices {
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

    /// Compute which column indices should be transposed based on the --select flag.
    /// Returns None if no selection was specified (meaning all columns).
    fn get_selected_indices(&self) -> CliResult<Option<Vec<usize>>> {
        if let Some(ref sel) = self.flag_select {
            // Read headers with no_headers(false) to get actual column names
            let mut rdr = Config::new(self.arg_input.as_ref())
                .delimiter(self.flag_delimiter)
                .no_headers(false)
                .reader()?;
            let headers = rdr.byte_headers()?.clone();
            let selection = sel
                .selection(&headers, true)
                .map_err(|e| CliError::Other(format!("Column selection error: {e}")))?;
            if selection.is_empty() {
                return fail_incorrectusage_clierror!(
                    "--select resulted in no columns to transpose."
                );
            }
            Ok(Some(selection.iter().copied().collect()))
        } else {
            Ok(None)
        }
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
