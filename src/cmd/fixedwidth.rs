static USAGE: &str = r##"
Converts fixed-width text (fields at fixed byte-column positions, no
delimiters) to CSV.

By default, this expects the input's first line to be a comment enumerating
the 1-based starting byte position of each column, comma-separated and
prefixed with "#" - the same format `qsv table --align leftfwf` produces
(e.g. "#1,10,15"). Every subsequent line is a data record; each field runs
from its starting position up to (but not including) the next column's
starting position, or to the end of the line for the last column. Trailing
whitespace in each field is trimmed.

If the input doesn't have such a header comment - e.g. it comes from an
external system - specify the column positions explicitly with --positions,
or column widths with --widths.

Examples:
    Convert output of `qsv table --align leftfwf` back to CSV:
        qsv table --align leftfwf data.csv | qsv fixedwidth > roundtrip.csv

    Convert a file with explicit 1-based column start positions:
        qsv fixedwidth --positions 1,10,15 mainframe_extract.txt

    Convert a file with explicit column widths instead of positions:
        qsv fixedwidth --widths 9,5,20 mainframe_extract.txt

See also https://github.com/dathere/qsv/wiki/Transform-and-Reshape#fixedwidth

Usage:
    qsv fixedwidth [options] [<input>]
    qsv fixedwidth --help

fixedwidth options:
    --positions <arg>      Comma-separated, 1-based starting byte position of
                           each column (e.g. "1,10,15"). Overrides any "#..."
                           header comment in the input.
    --widths <arg>         Comma-separated width, in bytes, of each column
                           (e.g. "9,5,20"). An alternative to --positions;
                           the two are mutually exclusive.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"##;

use std::io::{BufRead, BufReader};

use serde::Deserialize;

use crate::{CliResult, config::Config, util};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_positions: Option<String>,
    flag_widths:    Option<String>,
    flag_output:    Option<String>,
}

/// Parses a comma-separated list of 1-based positions (e.g. "1,10,15") into
/// 0-based byte offsets.
fn parse_positions(s: &str) -> CliResult<Vec<usize>> {
    let mut positions = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        let pos: usize = part
            .parse()
            .map_err(|_| format!("invalid position {part:?}: must be a positive integer"))?;
        if pos == 0 {
            return fail_incorrectusage_clierror!("column positions are 1-based; got 0");
        }
        positions.push(pos - 1);
    }
    if positions.windows(2).any(|w| w[0] >= w[1]) {
        return fail_incorrectusage_clierror!("column positions must be strictly increasing");
    }
    Ok(positions)
}

/// Parses a comma-separated list of column widths (e.g. "9,5,20") into
/// 0-based starting byte offsets.
fn parse_widths(s: &str) -> CliResult<Vec<usize>> {
    let mut positions = Vec::new();
    let mut offset = 0_usize;
    for part in s.split(',') {
        let part = part.trim();
        let width: usize = part
            .parse()
            .map_err(|_| format!("invalid width {part:?}: must be a positive integer"))?;
        if width == 0 {
            return fail_incorrectusage_clierror!("column widths must be greater than 0");
        }
        positions.push(offset);
        offset += width;
    }
    Ok(positions)
}

/// Splits a line into fields given 0-based starting byte offsets for each
/// column (the last column runs to the end of the line). Each field is
/// right-trimmed, since fixed-width fields are conventionally
/// space-padded.
fn split_line<'a>(line: &'a [u8], positions: &[usize]) -> Vec<&'a [u8]> {
    let mut fields = Vec::with_capacity(positions.len());
    for (i, &start) in positions.iter().enumerate() {
        if start >= line.len() {
            fields.push(&line[0..0]);
            continue;
        }
        let end = positions
            .get(i + 1)
            .copied()
            .unwrap_or(line.len())
            .min(line.len());
        let mut field = &line[start..end];
        while field.last().is_some_and(u8::is_ascii_whitespace) {
            field = &field[..field.len() - 1];
        }
        fields.push(field);
    }
    fields
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.flag_positions.is_some() && args.flag_widths.is_some() {
        return fail_incorrectusage_clierror!("--positions and --widths are mutually exclusive");
    }

    let rconfig = Config::new(args.arg_input.as_ref());
    let mut rdr = BufReader::new(rconfig.io_reader()?);

    let mut positions = match (&args.flag_positions, &args.flag_widths) {
        (Some(p), _) => Some(parse_positions(p)?),
        (_, Some(w)) => Some(parse_widths(w)?),
        (None, None) => None,
    };

    let mut line = Vec::new();
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    loop {
        line.clear();
        let bytes_read = rdr.read_until(b'\n', &mut line)?;
        if bytes_read == 0 {
            break;
        }
        while line.last() == Some(&b'\n') || line.last() == Some(&b'\r') {
            line.pop();
        }

        if positions.is_none() {
            let Some(header_positions) = line
                .strip_prefix(b"#")
                .map(|rest| String::from_utf8_lossy(rest).into_owned())
                .and_then(|rest| parse_positions(&rest).ok())
            else {
                return fail_incorrectusage_clierror!(
                    "no column positions given: pass --positions/--widths, or prefix the input \
                     with a \"#1,10,15\"-style header comment (as produced by `qsv table --align \
                     leftfwf`)"
                );
            };
            positions = Some(header_positions);
            continue;
        }

        let fields = split_line(&line, positions.as_ref().unwrap());
        wtr.write_record(fields)?;
    }

    Ok(wtr.flush()?)
}
