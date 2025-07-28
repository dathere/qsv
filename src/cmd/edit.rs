static USAGE: &str = r#"
Replace the value of a cell specified by its row and column.

For example we have the following CSV file named items.csv:

item,color
shoes,blue
flashlight,gray

To output the data with the color of the shoes as green instead of blue, run:

qsv edit items.csv color 0 green

The following is returned as output:

item,color
shoes,green
flashlight,gray

You may also choose to specify the column name by its index (in this case 1).
Specifying a column as a number is prioritized by index rather than name.
If there is no newline (\n) at the end of the input data, it may be added to the output.

Usage:
    qsv edit [options] <input> <column> <row> <value>
    qsv edit --help

edit arguments:
    input                  The file from which to edit a cell value. Use '-' for standard input.
                           Must be either CSV, TSV, TAB, or SSV data.
    column                 The cell's column name or index. Indices start from the first column as 0.
                           Providing a value of underscore (_) selects the last column.
    row                    The cell's row index. Indices start from the first non-header row as 0.
    value                  The new value to replace the old cell content with.

edit options:
    -i, --in-place         Overwrite the input file data with the output.
                           The input file is renamed to a .bak file in the same directory.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       Start row indices from the header row as 0 (allows editing the header row).
"#;

use csv::Writer;
use serde::Deserialize;
use tempfile::NamedTempFile;

use crate::{CliResult, config::Config, util};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_column:      String,
    arg_row:         usize,
    arg_value:       String,
    flag_in_place:   bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let input = args.arg_input;
    let column = args.arg_column;
    let row = args.arg_row;
    let in_place = args.flag_in_place;
    let value = args.arg_value;
    let no_headers = args.flag_no_headers;
    let mut tempfile = NamedTempFile::new()?;

    // Build the CSV reader and iterate over each record.
    let conf = Config::new(input.as_ref()).no_headers(true);
    let mut rdr = conf.reader()?;
    let mut wtr: Writer<Box<dyn std::io::Write>> = if in_place {
        csv::Writer::from_writer(Box::new(tempfile.as_file_mut()))
    } else {
        Config::new(args.flag_output.as_ref()).writer()?
    };

    let headers = rdr.headers()?;
    let mut column_index: Option<usize> = None;
    if column == "_" {
        column_index = Some(headers.len() - 1);
    } else if let Ok(c) = column.parse::<usize>() {
        column_index = Some(c);
    } else {
        for (i, header) in headers.iter().enumerate() {
            if column.as_str() == header {
                column_index = Some(i);
                break;
            }
        }
    }
    if column_index.is_none() {
        return fail_clierror!("Invalid column selected.");
    }

    let mut record = csv::ByteRecord::new();
    #[allow(clippy::bool_to_int_with_if)]
    let mut current_row: usize = if no_headers { 1 } else { 0 };
    while rdr.read_byte_record(&mut record)? {
        if row + 1 == current_row {
            for (current_col, field) in record.iter().enumerate() {
                if column_index == Some(current_col) {
                    wtr.write_field(&value)?;
                } else {
                    wtr.write_field(field)?;
                }
            }
            wtr.write_record(None::<&[u8]>)?;
        } else {
            wtr.write_byte_record(&record)?;
        }
        current_row += 1;
    }

    wtr.flush()?;
    drop(wtr);

    if in_place && let Some(input_path_string) = input {
        let input_path = std::path::Path::new(&input_path_string);
        if let Some(input_extension_osstr) = input_path.extension() {
            let mut backup_extension = input_extension_osstr.to_string_lossy().to_string();
            backup_extension.push_str(".bak");
            std::fs::rename(input_path, input_path.with_extension(backup_extension))?;
            std::fs::copy(tempfile.path(), input_path)?;
        }
    }

    Ok(())
}
