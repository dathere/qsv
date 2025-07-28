static USAGE: &str = r#"
Rename the columns of a CSV efficiently.

The new column names are given as a comma-separated list of names.
The number of column names given must match the number of columns in the
CSV unless "_all_generic" is used.

Alternatively, you can specify pairs of old and new column names to rename
only specific columns. The format is "old1,new1,old2,new2,...".

  Change the column names of a CSV with three columns:
    $ qsv rename id,name,title

  Rename only specific columns using pairs:
    $ qsv rename oldname,newname,oldcol,newcol

  Replace the column names with generic ones (_col_N):
    $ qsv rename _all_generic

  Add generic column names to a CSV with no headers:
    $ qsv rename _all_generic --no-headers

  Use column names that contains commas and conflict with the separator:
    $ qsv rename '"Date - Opening","Date - Actual Closing"'

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_rename.rs.

Usage:
    qsv rename [options] [--] <headers> [<input>]
    qsv rename --help

rename arguments:
    <headers>              The new headers to use for the CSV.
                           Separate multiple headers with a comma.
                           If "_all_generic" is given, the headers will be renamed
                           to generic column names, where the column name uses
                           the format "_col_N" where N is the 1-based column index.
                           Alternatively, specify pairs of old,new column names
                           to rename only specific columns.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the header will be inserted on top.    
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_headers:     String,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    if args.flag_no_headers {
        // Input has no header row, so read the first record to determine column count
        let mut record = csv::ByteRecord::new();
        if !rdr.read_byte_record(&mut record)? {
            // No data
            return Ok(());
        }
        // Determine new headers
        let num_cols = record.len();
        let new_headers = if args.arg_headers.to_lowercase() == "_all_generic" {
            rename_headers_all_generic(num_cols)
        } else {
            args.arg_headers
        };
        let mut new_rdr = csv::Reader::from_reader(new_headers.as_bytes());
        let new_headers = new_rdr.byte_headers()?.clone();
        if new_headers.len() != num_cols {
            return fail_incorrectusage_clierror!(
                "The length of the CSV columns ({}) is different from the provided header ({}).",
                num_cols,
                new_headers.len()
            );
        }
        wtr.write_record(&new_headers)?;
        wtr.write_record(&record)?;
        while rdr.read_byte_record(&mut record)? {
            wtr.write_record(&record)?;
        }
    } else {
        // Input has a header row, so use the original logic
        let headers = rdr.byte_headers()?;
        let header_parts: Vec<&str> = args.arg_headers.split(',').collect();
        let is_pairs = header_parts.len().is_multiple_of(2)
            && header_parts.len() >= 2
            && header_parts.chunks(2).any(|chunk| {
                chunk.len() == 2
                    && headers
                        .iter()
                        .any(|h| std::str::from_utf8(h).unwrap_or("") == chunk[0])
            });
        let has_matching_old = header_parts.chunks(2).any(|chunk| {
            chunk.len() == 2
                && headers
                    .iter()
                    .any(|h| std::str::from_utf8(h).unwrap_or("") == chunk[0])
        });
        let new_headers = if args.arg_headers.to_lowercase() == "_all_generic" {
            let s = rename_headers_all_generic(headers.len());
            let mut new_rdr = csv::Reader::from_reader(s.as_bytes());
            new_rdr.byte_headers()?.clone()
        } else if is_pairs && has_matching_old {
            if let Ok(renamed_headers) = parse_rename_pairs(&args.arg_headers, headers) {
                renamed_headers
            } else {
                let mut new_rdr = csv::Reader::from_reader(args.arg_headers.as_bytes());
                new_rdr.byte_headers()?.clone()
            }
        } else {
            let mut new_rdr = csv::Reader::from_reader(args.arg_headers.as_bytes());
            let new_headers = new_rdr.byte_headers()?.clone();
            if new_headers.len() != headers.len() {
                return fail_incorrectusage_clierror!(
                    "The length of the CSV headers ({}) is different from the provided one ({}).",
                    headers.len(),
                    new_headers.len()
                );
            }
            new_headers
        };
        wtr.write_record(&new_headers)?;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            wtr.write_record(&record)?;
        }
    }
    Ok(wtr.flush()?)
}

fn parse_rename_pairs(
    pairs_str: &str,
    original_headers: &csv::ByteRecord,
) -> CliResult<csv::ByteRecord> {
    let pairs: Vec<&str> = pairs_str.split(',').collect();
    if !pairs.len().is_multiple_of(2) {
        return fail_incorrectusage_clierror!(
            "Invalid number of arguments for pair-based renaming. Expected even number of values, \
             got {}.",
            pairs.len()
        );
    }

    // Create a mapping from old names to new names
    let mut rename_map = HashMap::new();
    for chunk in pairs.chunks(2) {
        if chunk.len() == 2 {
            // this assert is really just for the compiler to skip bounds checking below
            // per clippy::missing_asserts_for_indexing
            assert!(chunk.len() > 1);
            rename_map.insert(chunk[0], chunk[1]);
        }
    }

    // Create new headers by applying the rename map
    let mut new_headers = csv::ByteRecord::new();
    for header in original_headers {
        let header_str =
            std::str::from_utf8(header).map_err(|_| "Invalid UTF-8 in header".to_string())?;

        if let Some(&new_name) = rename_map.get(header_str) {
            new_headers.push_field(new_name.as_bytes());
        } else {
            new_headers.push_field(header);
        }
    }

    Ok(new_headers)
}

pub fn rename_headers_all_generic(num_of_cols: usize) -> String {
    use std::fmt::Write;

    // we pre-allocate a string with a capacity of 7 characters per column name
    // this is a rough estimate, and should be more than enough
    let mut result = String::with_capacity(num_of_cols * 7);
    for i in 1..=num_of_cols {
        if i > 1 {
            result.push(',');
        }
        // safety: safe to unwrap as we're just using it to append to result string
        write!(result, "_col_{i}").unwrap();
    }
    result
}
