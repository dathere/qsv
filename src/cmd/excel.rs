static USAGE: &str = r#"
Exports a specified Excel/ODS sheet to a CSV file.

NOTE: Excel stores dates as number of days since 1900.
https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487

Because of this, this command uses a --dates-whitelist to determine if it
will attempt to transform a numeric value to an ISO 8601 date based on its name.

If the column name satisfies the whitelist and a row value for a candidate date column
is a float - it will infer a date for whole numbers and a datetime for float values with
fractional components (e.g. 40729 is 2011-07-05, 37145.354166666664 is 2001-09-11 8:30:00).

We need a whitelist so we know to only do this date conversions for date fields and
not all columns with numeric values.

Note however that with XLSX files, qsv will automatically process a cell as a date, even if its
not its not in the --dates-whitelist, if the cell's format has been explicitly set to date.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_excel.rs.

Usage:
    qsv excel [options] [<input>]
    qsv excel --help

Excel options:
    -s, --sheet <name/index>   Name or zero-based index of sheet to export.
                               Negative indices start from the end (-1 = last sheet). 
                               If the sheet cannot be found, qsv will read the first sheet.
                               [default: 0]
    --metadata                 Creates a CSV of workbook metadata with five columns - 
                               index, sheet_name, headers, num_columns & num_rows.
                               Note that headers is a semicolon-delimited list of the first row
                               (which is presumably, but not necessarily the header/column names)
                               and num_rows includes all rows, including the first row.
                               All other Excel options are ignored.
    --flexible                 Continue even if the number of columns is different 
                               from the previous record.
    --trim                     Trim all fields so that leading & trailing whitespaces are removed.
                               Also removes embedded linebreaks.
    --safe-header-names        Make database-safe header names - i.e. duplicate header names will 
                               have a sequential suffix (_n) appended.
                               Leading/trailing spaces will be trimmed.
                               Whitespace/non-alphanumeric characters will be replaced with _.
                               If a column name starts with a digit, the digit is replaced with a _.
                               Maximum length is 60 characters.
    --dates-whitelist <list>   The case-insensitive patterns to look for when 
                               shortlisting columns for date processing.
                               i.e. if the column's name has any of these patterns,
                               it is interpreted as a date column.

                               Otherwise, Excel date columns that do not satisfy the
                               whitelist will be returned as number of days since 1900.

                               Set to "all" to interpret ALL numeric columns as date types.
                               Note that this will cause false positive date conversions
                               for all numeric columns that are not dates.

                               Conversely, set to "none" to stop date processing altogether.

                               If the list is all integers, its interpreted as the zero-based
                               index of all the date columns for date processing.
                               [default: date,time,due,opened,closed]                               

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
"#;

use std::{cmp, path::PathBuf};

use calamine::{open_workbook_auto, DataType, Range, Reader};
use log::{debug, info};
use serde::Deserialize;
use thousands::Separable;

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:              String,
    flag_sheet:             String,
    flag_metadata:          bool,
    flag_flexible:          bool,
    flag_trim:              bool,
    flag_safe_header_names: bool,
    flag_dates_whitelist:   String,
    flag_output:            Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let path = &args.arg_input;

    let sce = PathBuf::from(path.to_ascii_lowercase());
    let mut ods_flag = false;
    match sce.extension().and_then(std::ffi::OsStr::to_str) {
        Some("xls" | "xlsx" | "xlsm" | "xlsb") => (),
        Some("ods") => ods_flag = true,
        _ => {
            return fail!(
                "The excel command supports the following workbook formats - xls, xlsx, xlsm, \
                 xlsb and ods."
            );
        }
    };

    let mut workbook = match open_workbook_auto(path) {
        Ok(workbook) => workbook,
        Err(e) => {
            let es = e.to_string();
            // password protected errors come in different flavors for Excel
            if es.starts_with("Xls error: Cfb error")
                || es.starts_with("Xlsx error: Zip error: invalid Zip archive")
            {
                return fail_clierror!("{path} may be a password-protected workbook: {e}.");
            }
            return fail_clierror!("Cannot open workbook: {e}.");
        }
    };

    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        if ods_flag {
            return fail_clierror!("{path} may be password protected.");
        };
        return fail!("No sheets found.");
    }
    let num_sheets = sheet_names.len();
    let sheet_vec = sheet_names.to_owned();

    let mut wtr = Config::new(&args.flag_output)
        .flexible(args.flag_flexible)
        .writer()?;
    let mut record = csv::StringRecord::new();

    // create the metadata report CSV to stdout then exit
    if args.flag_metadata {
        record.push_field("index");
        record.push_field("sheet_name");
        record.push_field("headers");
        record.push_field("num_columns");
        record.push_field("num_rows");
        wtr.write_record(&record)?;
        #[allow(clippy::needless_range_loop)]
        for i in 0..num_sheets {
            record.clear();
            record.push_field(&i.to_string());
            let sheet_name = sheet_vec[i].clone();
            record.push_field(&sheet_name);

            let range = match workbook.worksheet_range_at(i) {
                Some(result) => {
                    if let Ok(result) = result {
                        result
                    } else {
                        return fail_clierror!("Cannot retrieve range from {}", sheet_name);
                    }
                }
                None => Range::empty(),
            };

            if range.is_empty() {
                record.push_field("");
                record.push_field("0");
                record.push_field("0");
            } else {
                let (num_rows, num_columns) = range.get_size();
                let mut sheet_rows = range.rows();
                let first_row = sheet_rows.next().unwrap();
                let first_row_str = first_row
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(";");
                record.push_field(&first_row_str);
                record.push_field(&num_columns.to_string());
                record.push_field(&num_rows.to_string());
            }

            wtr.write_record(&record)?;
        }
        wtr.flush()?;
        log::info!("listed sheet names: {sheet_vec:?}");
        return Ok(());
    }

    // convert sheet_names to lowercase so we can do a case-insensitive compare
    let mut lower_sheet_names: Vec<String> = Vec::with_capacity(num_sheets);
    for s in sheet_names {
        lower_sheet_names.push(s.to_lowercase());
    }

    // if --sheet name was passed, see if its a valid sheet name.
    let mut sheet = if lower_sheet_names.contains(&args.flag_sheet.to_lowercase()) {
        args.flag_sheet
    } else {
        // otherwise, if --sheet is a number, its a zero-based index, fetch it
        if let Ok(sheet_index) = args.flag_sheet.parse::<i32>() {
            if sheet_index >= 0 {
                if sheet_index as usize <= sheet_names.len() {
                    sheet_names[sheet_index as usize].to_string()
                } else {
                    return fail_clierror!(
                        "sheet index {sheet_index} is greater than number of sheets {}",
                        sheet_names.len()
                    );
                }
            } else {
                // if its a negative number, start from the end
                // i.e -1 is the last sheet; -2 = 2nd to last sheet
                sheet_names[cmp::max(
                    0,
                    cmp::min(
                        num_sheets - 1,
                        num_sheets.abs_diff(sheet_index.unsigned_abs() as usize),
                    ),
                )]
                .to_string()
            }
        } else {
            // failing all else, get the first sheet
            let first_sheet = sheet_names[0].to_string();
            debug!(
                r#"Invalid sheet "{}". Using the first sheet "{}" instead."#,
                args.flag_sheet, first_sheet
            );
            first_sheet
        }
    };
    let lower_sheet = sheet.to_lowercase();
    let sheet_index = if let Some(idx) = lower_sheet_names.iter().position(|s| *s == lower_sheet) {
        // set to actual name of the sheet, not the one passed using the --sheet option,
        // as we process the option case insensitively
        sheet = sheet_names[idx].clone();
        idx
    } else {
        return fail_clierror!("Cannot get sheet index for {sheet}");
    };

    let range = match workbook.worksheet_range_at(sheet_index) {
        Some(result) => {
            if let Ok(result) = result {
                result
            } else {
                return fail_clierror!("Cannot retrieve range from {sheet}");
            }
        }
        None => Range::empty(),
    };

    let whitelist_lower = args.flag_dates_whitelist.to_lowercase();
    info!("using date-whitelist: {whitelist_lower}");

    // an all number whitelist means we're being given
    // the column indices of the date column names
    let mut all_numbers_whitelist = true;

    let mut dates_whitelist =
        itertools::Itertools::collect_vec(whitelist_lower.split(',').map(|s| {
            if all_numbers_whitelist && s.parse::<u16>().is_err() {
                all_numbers_whitelist = false;
                info!("NOT a column index dates whitelist");
            }
            s.trim().to_string()
        }));
    // we sort the whitelist, so we can do the faster binary_search() instead of contains()
    // with an all_numbers_whitelist
    if all_numbers_whitelist {
        dates_whitelist.sort_unstable();
    }

    let mut trimmed_record = csv::StringRecord::new();
    let mut date_flag: Vec<bool> = Vec::new();
    let mut cell_date_flag;
    let mut float_val = 0_f64;
    let mut float_flag;
    // use u32 as Excel/ODS can only handle 1,048,576 rows anyway
    // https://support.microsoft.com/en-us/office/excel-specifications-and-limits-1672b34d-7043-467e-8e27-269d656771c3
    // https://wiki.documentfoundation.org/Faq/Calc/022
    let mut row_count = 0_u32;

    for (row_idx, row) in range.rows().enumerate() {
        record.clear();
        for (col_idx, cell) in row.iter().enumerate() {
            if row_idx == 0 {
                // its the header row, check the dates whitelist
                let col_name = cell.get_string().unwrap_or_default();
                record.push_field(col_name);
                match whitelist_lower.as_str() {
                    // "all" - all numeric fields are to be treated as dates
                    "all" => date_flag.insert(col_idx, true),
                    // "none" - date processing will not be attempted
                    "none" => date_flag.insert(col_idx, false),
                    // check if the column name is in the dates_whitelist
                    _ => date_flag.insert(
                        col_idx,
                        if all_numbers_whitelist {
                            dates_whitelist.binary_search(&col_idx.to_string()).is_ok()
                        } else {
                            let mut date_found = false;
                            let col_name_lower = col_name.to_lowercase();
                            for whitelist_item in &dates_whitelist {
                                if col_name_lower.contains(whitelist_item) {
                                    date_found = true;
                                    log::info!("date-whitelisted: {col_name}");
                                    break;
                                }
                            }
                            date_found
                        },
                    ),
                }
                continue;
            }
            cell_date_flag = false;
            float_flag = false;
            match *cell {
                DataType::Empty => record.push_field(""),
                DataType::String(ref s) => record.push_field(s),
                DataType::DateTime(ref f) => {
                    float_val = *f;
                    float_flag = true;
                    cell_date_flag = true;
                }
                DataType::Float(ref f) => {
                    float_val = *f;
                    float_flag = true;
                    cell_date_flag = date_flag[col_idx];
                }
                DataType::Int(ref i) => record.push_field(&i.to_string()),
                DataType::Error(ref e) => record.push_field(&format!("{e:?}")),
                DataType::Bool(ref b) => record.push_field(&b.to_string()),
            };
            // dates are stored as numbers in Excel
            // that's why we need the --dates-whitelist, so we can convert the number to a date.
            // however, with the XLSX format, we can get a cell's format as an attribute. So we can
            // automatically process a cell as a date, even if its column is NOT in the whitelist
            if float_flag {
                if cell_date_flag {
                    if float_val.fract() > 0.0 {
                        record.push_field({
                            &cell.as_datetime().map_or_else(
                                || format!("ERROR: Cannot convert {float_val} to datetime"),
                                |dt| format!("{dt}"),
                            )
                        });
                    } else {
                        record.push_field({
                            &cell.as_date().map_or_else(
                                || format!("ERROR: Cannot convert {float_val} to date"),
                                |d| format!("{d}"),
                            )
                        });
                    };
                } else {
                    record.push_field(&float_val.to_string());
                }
            }
        }

        // if this is the header/column row and --safe-header-names option is true
        if row_count == 0 && args.flag_safe_header_names {
            record.trim();
            let mut temp_record = csv::StringRecord::new();
            for header in &record {
                // maximum length is 60 characters
                // we do 60 even though postgresql can do up to 63
                // just in case we have duplicate header names
                // and safe_header_names appends a sequence suffix
                temp_record
                    .push_field(&header[..header.chars().map(char::len_utf8).take(60).sum()]);
            }
            let safe_headers = util::safe_header_names(&temp_record, true);
            record.clear();
            for header_name in safe_headers {
                record.push_field(&header_name);
            }
        }

        if args.flag_trim {
            record.trim();
            trimmed_record.clear();
            record.iter().for_each(|field| {
                if field.contains('\n') {
                    trimmed_record.push_field(&field.to_string().replace('\n', " "));
                } else {
                    trimmed_record.push_field(field);
                }
            });
            wtr.write_record(&trimmed_record)?;
        } else {
            wtr.write_record(&record)?;
        }
        row_count += 1;
    }
    wtr.flush()?;

    let end_msg = format!(
        "{} {}-column rows exported from \"{sheet}\"",
        // don't count the header in row count
        row_count.saturating_sub(1).separate_with_commas(),
        record.len().separate_with_commas(),
    );
    winfo!("{end_msg}");

    Ok(())
}
