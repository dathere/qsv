static USAGE: &str = r#"
Prints the fields of the first row in the CSV data.

These names can be used in commands like 'select' to refer to columns in the
CSV data.

Note that multiple CSV files may be given to this command. This is useful with
the --union flag.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_headers.rs.

Usage:
    qsv headers [options] [<input>...]
    qsv headers --help

headers arguments:
    <input>...             The CSV file(s) to read. Use '-' for standard input.
                           If input is a directory, all files in the directory will
                           be read as input.
                           If the input is a file with a '.infile-list' extension,
                           the file will be read as a list of input files.
                           If the input are snappy-compressed files(s), it will be
                           decompressed automatically.

headers options:
    -j, --just-names       Only show the header names (hide column index).
                           This is automatically enabled if more than one
                           input is given.
    -J, --just-count       Only show the number of headers.
    --union                Shows the union of headers across all inputs
                           (deduplicated).
    --trim                 Trim leading/trailing space, tab, and quote
                           characters from header name.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{io, path::PathBuf};

use qsv_tabwriter::TabWriter;
use serde::Deserialize;

use crate::{CliResult, config::Delimiter, util};

#[derive(Deserialize)]
struct Args {
    arg_input:       Vec<PathBuf>,
    flag_just_names: bool,
    flag_just_count: bool,
    flag_union:      bool,
    flag_trim:       bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    let tmpdir = tempfile::tempdir()?;

    // If no input is provided, default to stdin
    if args.arg_input.is_empty() {
        args.arg_input.push(PathBuf::from("-"));
    }

    args.arg_input = util::process_input(args.arg_input, &tmpdir, "")?;
    let configs = util::many_configs(&args.arg_input, args.flag_delimiter, true, false)?;

    let num_inputs = configs.len();
    if num_inputs > 1 {
        args.flag_just_names = true;
    }
    let mut headers: Vec<Vec<u8>> = vec![];
    for conf in configs {
        let mut rdr = conf.reader()?;
        for header in rdr.byte_headers()? {
            if !args.flag_union || !headers.iter().any(|h| h.as_slice() == header) {
                headers.push(header.to_vec());
            }
        }
    }

    let mut wtr: Box<dyn io::Write> = if args.flag_just_names || args.flag_just_count {
        Box::new(io::stdout())
    } else {
        Box::new(TabWriter::new(io::stdout()))
    };
    if args.flag_just_count {
        writeln!(wtr, "{}", headers.len())?;
    } else {
        for (i, header) in headers.iter().enumerate() {
            if !args.flag_just_names {
                write!(&mut wtr, "{}\t", i + 1)?;
            }
            if args.flag_trim {
                let mut h: &[u8] = header;
                while matches!(h.first(), Some(b'"' | b' ' | b'\t')) {
                    h = &h[1..];
                }
                while matches!(h.last(), Some(b'"' | b' ' | b'\t')) {
                    h = &h[..h.len() - 1];
                }
                wtr.write_all(h)?;
            } else {
                wtr.write_all(header)?;
            }
            wtr.write_all(b"\n")?;
        }
    }
    Ok(wtr.flush()?)
}
