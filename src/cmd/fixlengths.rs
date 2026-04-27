static USAGE: &str = r#"
Transforms CSV data so that all records have the same length. The length is
the length of the longest record in the data (not counting trailing empty fields,
but at least 1). Records with smaller lengths are padded with empty fields.

This requires two complete scans of the CSV data: one for determining the
record size and one for the actual transform. Because of this, the input
given must be a file and not stdin.

Alternatively, if --length is set, then all records are forced to that length.
This requires a single pass and can be done with stdin.

Usage:
    qsv fixlengths [options] [<input>]
    qsv fixlengths --help

fixlengths options:
    -l, --length <arg>     Forcefully set the length of each record. If a
                           record is not the size given, then it is truncated
                           or expanded as appropriate.
    -r, --remove-empty     Remove empty columns.
    -i, --insert <pos>     If empty fields need to be inserted, insert them
                           at <pos>. If <pos> is zero, then it is inserted
                           at the end of each record. If <pos> is negative, it
                           is inserted from the END of each record going backwards.
                           If <pos> is positive, it is inserted from the BEGINNING
                           of each record going forward. [default: 0]
    --quote <arg>          The quote character to use. [default: "]
    --escape <arg>         The escape character to use. When not specified,
                           quotes are escaped by doubling them.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -q, --quiet            Don't print removed column information.
"#;

use std::cmp;

use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    flag_length:       Option<usize>,
    flag_insert:       i16,
    flag_remove_empty: bool,
    flag_quiet:        bool,
    flag_quote:        Delimiter,
    flag_escape:       Option<Delimiter>,
    flag_output:       Option<String>,
    flag_delimiter:    Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut config = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .quote(args.flag_quote.as_byte())
        .no_headers(true)
        .flexible(true);

    if let Some(escape) = args.flag_escape {
        config = config.escape(Some(escape.as_byte())).double_quote(false);
    }

    if config.is_stdin() && args.flag_remove_empty {
        return fail_incorrectusage_clierror!(
            "<stdin> cannot be used with --remove-empty. Please specify a file path."
        );
    }

    // First determine if we need to identify existing empty columns
    let col_is_empty_vec_opt = if args.flag_remove_empty {
        // First pass: identify universally-empty columns. The vector grows
        // with the widest record seen, since the reader is flexible and
        // records may have varying widths.
        let mut rdr = config.reader()?;
        let mut record = csv::ByteRecord::new();
        let mut col_is_empty_vec: Vec<bool> = Vec::new();

        while rdr.read_byte_record(&mut record)? {
            if record.len() > col_is_empty_vec.len() {
                col_is_empty_vec.resize(record.len(), true);
            }
            for (i, field) in record.iter().enumerate() {
                if !field.is_empty() {
                    col_is_empty_vec[i] = false;
                }
            }
        }

        // Count and report removed columns
        let empty_count = col_is_empty_vec.iter().filter(|&&x| x).count();
        if !args.flag_quiet && empty_count > 0 {
            eprintln!("Removed {empty_count} empty column(s)");
        }
        Some(col_is_empty_vec)
    } else {
        None
    };

    let length = if let Some(length) = args.flag_length {
        if length == 0 {
            return fail_incorrectusage_clierror!("Length must be greater than 0.");
        }
        length
    } else if args.flag_remove_empty {
        // When removing empty columns, we need to determine the maximum length
        // after filtering out the empty columns. This requires another pass through the data.
        let mut rdr = config.reader()?;
        let mut record = csv::ByteRecord::new();
        let mut maxlen = 0_usize;
        let col_is_empty_vec = col_is_empty_vec_opt.as_ref().unwrap();

        while rdr.read_byte_record(&mut record)? {
            // col_is_empty_vec covers the widest record observed in pass 1, so
            // every i < record.len() is in bounds — no need for get_unchecked.
            let filtered_len = record
                .iter()
                .enumerate()
                .filter(|(i, _)| !col_is_empty_vec[*i])
                .count();
            maxlen = cmp::max(maxlen, filtered_len);
        }
        maxlen
    } else {
        if config.is_stdin() {
            return fail_incorrectusage_clierror!(
                "<stdin> cannot be used when --length is not set. Please specify a file path."
            );
        }
        let mut maxlen = 0_usize;
        let mut rdr = config.reader()?;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            let mut nonempty_count = 0;
            for (index, field) in record.iter().enumerate() {
                if index == 0 || !field.is_empty() {
                    nonempty_count = index + 1;
                }
            }
            maxlen = cmp::max(maxlen, nonempty_count);
        }
        maxlen
    };

    let mut rdr = config.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    let mut record = csv::ByteRecord::new();
    let mut record_work = csv::ByteRecord::new();
    let mut filtered_record = csv::ByteRecord::new();

    // Widen to isize so `length + flag_insert` doesn't silently truncate when
    // the auto-detected length exceeds i16::MAX (e.g. very wide CSVs).
    let flag_insert = args.flag_insert as isize;
    let insert_pos: isize = if flag_insert < 0 {
        length as isize + flag_insert
    } else {
        flag_insert
    };

    while rdr.read_byte_record(&mut record)? {
        // First remove existing empty columns if needed.
        if let Some(ref is_empty) = col_is_empty_vec_opt {
            filtered_record.clear();
            for (i, field) in record.iter().enumerate() {
                // is_empty covers the widest record seen in pass 1.
                if !is_empty[i] {
                    filtered_record.push_field(field);
                }
            }
            std::mem::swap(&mut record, &mut filtered_record);
        }

        // Then handle length adjustments by comparing target length with current record length.
        let current_len = record.len();
        if length > current_len {
            // Record is too short - need to add empty fields.
            if flag_insert == 0 {
                // No insert position specified - append empty fields at end.
                record.extend((0..length - current_len).map(|_| b""));
            } else {
                // Insert empty fields at specified position.
                record_work.clear();
                for (field_idx, field) in (1_isize..).zip(&record) {
                    if field_idx == insert_pos {
                        // When we reach insert position, add all needed empty fields.
                        record_work.extend((0..length - current_len).map(|_| b""));
                    }
                    record_work.push_field(field);
                }
                if record_work.len() < length {
                    // Insert position was past end of record - pad at the end.
                    record_work.extend((0..length - record_work.len()).map(|_| b""));
                }
                std::mem::swap(&mut record, &mut record_work);
            }
        } else if length < current_len {
            // Record is too long - truncate to target length.
            record.truncate(length);
        }
        wtr.write_byte_record(&record)?;
    }
    Ok(wtr.flush()?)
}
