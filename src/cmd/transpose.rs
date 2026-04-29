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
    /// Resolve --select against the given header record.
    /// Returns Ok(None) when --select was not specified.
    fn parse_select(&self, headers: &ByteRecord) -> CliResult<Option<Vec<usize>>> {
        let Some(ref sel) = self.flag_select else {
            return Ok(None);
        };
        let selection = sel
            .selection(headers, true)
            .map_err(|e| CliError::Other(format!("--select error: {e}")))?;
        if selection.is_empty() {
            return fail_incorrectusage_clierror!("--select resulted in no columns to transpose.");
        }
        Ok(Some(selection.iter().copied().collect()))
    }

    fn wide_to_long(&self) -> CliResult<()> {
        let mut rdr = Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(false)
            .reader()?;
        let mut wtr = self.wconfig().writer()?;

        let headers = rdr.byte_headers()?.clone();
        if headers.is_empty() {
            return fail_incorrectusage_clierror!("CSV file must have at least one column.");
        }

        // --long is required by docopt; defensively report a usage error if absent.
        let selection_str = match self.flag_long.as_deref() {
            Some(s) => s,
            None => {
                return fail_incorrectusage_clierror!(
                    "--long requires a column selection argument."
                );
            },
        };
        let select_cols = SelectColumns::parse(selection_str)
            .map_err(|e| CliError::Other(format!("--long parse error: {e}")))?;
        let selection = select_cols
            .selection(&headers, true)
            .map_err(|e| CliError::Other(format!("--long selection error: {e}")))?;
        if selection.is_empty() {
            return fail_incorrectusage_clierror!(
                "--long resulted in no columns. At least one field column is required."
            );
        }
        let field_column_indices: Vec<usize> = selection.iter().copied().collect();
        let field_column_set: HashSet<usize> = field_column_indices.iter().copied().collect();

        // --select filters which columns become attribute rows (field columns are unaffected)
        let selected_attribute_set: Option<HashSet<usize>> = self
            .parse_select(&headers)?
            .map(|v| v.into_iter().collect());

        // Write output headers
        let mut header_record = ByteRecord::with_capacity(64, 3);
        header_record.push_field(b"field");
        header_record.push_field(b"attribute");
        header_record.push_field(b"value");
        wtr.write_byte_record(&header_record)?;

        // Reusable buffers (allocated once, reused per row).
        let multi_field = field_column_indices.len() > 1;
        let mut field_buf: Vec<u8> = Vec::with_capacity(256);
        let mut output_record = ByteRecord::with_capacity(256, 3);
        let mut data_record = ByteRecord::new();

        while rdr.read_byte_record(&mut data_record)? {
            // Build the field key — borrow the slice for the single-column case to
            // avoid an allocation per row; concatenate into a reused buffer otherwise.
            let field_slice: &[u8] = if multi_field {
                field_buf.clear();
                for (i, &idx) in field_column_indices.iter().enumerate() {
                    if i > 0 {
                        field_buf.push(b'|');
                    }
                    if let Some(v) = data_record.get(idx) {
                        field_buf.extend_from_slice(v);
                    }
                }
                &field_buf
            } else {
                data_record.get(field_column_indices[0]).unwrap_or(b"")
            };

            // Iterate through all columns, skipping field columns and non-selected columns
            for (i, attribute_header) in headers.iter().enumerate() {
                if field_column_set.contains(&i) {
                    continue;
                }
                if let Some(ref sel_set) = selected_attribute_set
                    && !sel_set.contains(&i)
                {
                    continue;
                }
                if let Some(value) = data_record.get(i)
                    && !value.is_empty()
                {
                    output_record.clear();
                    output_record.push_field(field_slice);
                    output_record.push_field(attribute_header);
                    output_record.push_field(value);
                    wtr.write_byte_record(&output_record)?;
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

        // The reader is configured with no_headers(true), so the first record IS the
        // input CSV's header row — we need it to participate in the transpose AND to
        // resolve --select by column name. Collect everything once.
        let all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        let ncols = all.first().map_or(0, ByteRecord::len);

        let empty_rec = ByteRecord::new();
        let headers_for_select = all.first().unwrap_or(&empty_rec);
        let indices: Vec<usize> = self
            .parse_select(headers_for_select)?
            .unwrap_or_else(|| (0..ncols).collect());

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
        // Memory map the file for efficient cross-pass access.
        // No `.populate()` here on purpose — `--multipass` exists to avoid loading
        // the whole dataset into memory, so we let the OS page in lazily.
        let file = File::open(self.arg_input.as_ref().unwrap())?;
        // safety: `run()` only routes here when `input_is_stdin == false`, so
        // `arg_input` names an on-disk file that can be memory-mapped. The
        // `file` binding stays in scope for the rest of this function and all
        // uses of `mmap` are confined to the same scope, so the file handle
        // outlives the mapping. We open the file read-only and only ever read
        // from `&mmap[..]` to feed CSV parsers across passes — this command
        // does not mutate or truncate the file. As with any file-backed mmap,
        // soundness still relies on no other process concurrently truncating
        // or otherwise mutating the file while the mapping is live.
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let rconfig = self.rconfig();

        // Read the first record to determine column count & resolve --select. This
        // also serves as the header row for name-based selection.
        let mut header_rdr = rconfig.from_reader(&mmap[..]);
        let mut headers = ByteRecord::new();
        let _ = header_rdr.read_byte_record(&mut headers)?;
        let ncols = headers.len();
        drop(header_rdr);

        let indices: Vec<usize> = self
            .parse_select(&headers)?
            .unwrap_or_else(|| (0..ncols).collect());

        let mut wtr = self.wconfig().writer()?;
        let mut record = ByteRecord::with_capacity(1024, ncols);

        for i in indices {
            record.clear();

            // Restart parsing of the mmap'd CSV for this output column.
            // The mmap stays mapped across passes, so we get page-cache locality
            // rather than re-reading bytes from disk.
            let mut rdr = rconfig.from_reader(&mmap[..]);
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
        // Wide rows after transpose can be very large; bump the write buffer
        // to amortize syscalls.
        Config::new(self.flag_output.as_ref()).set_write_buffer(DEFAULT_WTR_BUFFER_CAPACITY * 20)
    }

    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(true)
    }
}
