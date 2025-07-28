static USAGE: &str = r#"
Find the difference between two CSVs with ludicrous speed.

NOTE: diff does not support stdin. A file path is required for both arguments.
      Further, PRIMARY KEY VALUES MUST BE UNIQUE WITHIN EACH CSV.
      When diffing CSVs with just a single --key column and a stats cache is
      available, diff will automatically validate for primary key uniqueness.
      If more than one --key column is specified, however, this auto-validation
      is not done.

      To check if a CSV has unique primary key values, use `qsv extdedup`
      with the same key columns using the `--select` option:

         qsv extdedup --select keycol data.csv --no-output

      The duplicate count will be printed to stderr.

Examples:

Find the difference between two CSVs:
    qsv diff left.csv right.csv

Find the difference between two CSVs. The right CSV has no headers:
    qsv diff left.csv --no-headers-right right-noheaders.csv

Find the difference between two CSVs. The left CSV uses a tab as the delimiter:
    qsv diff --delimiter-left '\t' left.csv right-tab.tsv
    # or ';' as the delimiter
    qsv diff --delimiter-left ';' left.csv right-semicolon.csv

Find the difference between two CSVs. The output CSV uses a tab as the delimiter
and is written to a file:
    qsv diff -o diff-tab.tsv --delimiter-output '\t' left.csv right.csv
    # or ';' as the delimiter
    qsv diff -o diff-semicolon.csv --delimiter-output ';' left.csv right.csv

Find the difference between two CSVs, comparing records that have the same values
in the first two columns:
    qsv diff --key 0,1 left.csv right.csv

Find the difference between two CSVs, comparing records that have the same values
in the first two columns and sort the result by the first two columns:
    qsv diff -k 0,1 --sort-columns 0,1 left.csv right.csv

Find the difference between two CSVs, but do not output equal field values
in the result (equal field values are replaced with the empty string). Key
field values _will_ appear in the output:
    qsv diff --drop-equal-fields left.csv right.csv

Find the difference between two CSVs, but do not output headers in the result:
    qsv diff --no-headers-output left.csv right.csv

Find the difference between two CSVs. Both CSVs have no headers, but the result should have
headers, so generic headers will be used in the form of: _col_1, _col_2, etc.:
    qsv diff --no-headers-left --no-headers-right left.csv right.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_diff.rs

Usage:
    qsv diff [options] [<input-left>] [<input-right>]
    qsv diff --help

diff options:
    --no-headers-left           When set, the first row will be considered as part of
                                the left CSV to diff. (When not set, the
                                first row is the header row and will be skipped during
                                the diff. It will always appear in the output.)
    --no-headers-right          When set, the first row will be considered as part of
                                the right CSV to diff. (When not set, the
                                first row is the header row and will be skipped during
                                the diff. It will always appear in the output.)
    --no-headers-output         When set, the diff result won't have a header row in
                                its output. If not set and both CSVs have no headers,
                                headers in the result will be: _col_1,_col_2, etc.
    --delimiter-left <arg>      The field delimiter for reading CSV data on the left.
                                Must be a single character. (default: ,)
    --delimiter-right <arg>     The field delimiter for reading CSV data on the right.
                                Must be a single character. (default: ,)
    --delimiter-output <arg>    The field delimiter for writing the CSV diff result.
                                Must be a single character. (default: ,)
    -k, --key <arg...>          The column indices that uniquely identify a record
                                as a comma separated list of 0-based indices, e.g. 0,1,2
                                or column names, e.g. name,age.
                                Note that when selecting columns by name, only the 
                                left CSV's headers are used to match the column names
                                and it is assumed that the right CSV has the same
                                selected column names in the same order as the left CSV.
                                (default: 0)
    --sort-columns <arg...>     The column indices by which the diff result should be
                                sorted as a comma separated list of indices, e.g. 0,1,2
                                or column names, e.g. name,age.
                                Records in the diff result that are marked as "modified"
                                ("delete" and "add" records that have the same key,
                                but have different content) will always be kept together
                                in the sorted diff result and so won't be sorted
                                independently from each other.
                                Note that when selecting columns by name, only the 
                                left CSV's headers are used to match the column names
                                and it is assumed that the right CSV has the same
                                selected column names in the same order as the left CSV.
    --drop-equal-fields         Drop values of equal fields in modified rows of the CSV
                                diff result (and replace them with the empty string).
                                Key field values will not be dropped.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number
                                of CPUs detected.
    --force                     Force diff and ignore stats caches for the left & right CSVs.
                                Otherwise, if available, the stats cache will be used to:
                                 * short-circuit the diff if their fingerprint hashes are
                                   identical.
                                 * check for primary key uniqueness when only one --key
                                   column is specified.

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -d, --delimiter <arg>       Set ALL delimiters to this character.
                                Overrides --delimiter-right, --delimiter-left
                                and --delimiter-output.
"#;

use std::io::{self, Write};

use csv::ByteRecord;
use csv_diff::{
    csv_diff::CsvByteDiffBuilder, csv_headers::Headers, diff_result::DiffByteRecords,
    diff_row::DiffByteRecord,
};
use serde::Deserialize;

use super::rename::rename_headers_all_generic;
use crate::{
    CliResult,
    clitypes::CliError,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
    util::{SchemaArgs, StatsMode, get_stats_records},
};

#[derive(Deserialize)]
struct Args {
    arg_input_left:         Option<String>,
    arg_input_right:        Option<String>,
    flag_output:            Option<String>,
    flag_jobs:              Option<usize>,
    flag_force:             bool,
    flag_no_headers_left:   bool,
    flag_no_headers_right:  bool,
    flag_no_headers_output: bool,
    flag_delimiter_left:    Option<Delimiter>,
    flag_delimiter_right:   Option<Delimiter>,
    flag_delimiter_output:  Option<Delimiter>,
    flag_key:               Option<String>,
    flag_sort_columns:      Option<String>,
    flag_drop_equal_fields: bool,
    flag_delimiter:         Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // if stats cache is available, perform "smart" validation checks
    if check_stats_cache(&args)? {
        // the stats cache is available and files are identical, short-circuit diff
        // and return immediately
        return Ok(());
    }

    if let Some(delim) = args.flag_delimiter {
        [
            args.flag_delimiter_left,
            args.flag_delimiter_right,
            args.flag_delimiter_output,
        ] = [Some(delim); 3];
    }

    let rconfig_left = Config::new(args.arg_input_left.as_ref())
        .delimiter(args.flag_delimiter_left)
        .no_headers(args.flag_no_headers_left);

    let rconfig_right = Config::new(args.arg_input_right.as_ref())
        .delimiter(args.flag_delimiter_right)
        .no_headers(args.flag_no_headers_right);

    if rconfig_left.is_stdin() || rconfig_right.is_stdin() {
        return fail_incorrectusage_clierror!(
            "diff does not support stdin. A file path is required for both arguments."
        );
    }

    let mut csv_rdr_left = rconfig_left.reader()?;
    let mut csv_rdr_right = rconfig_right.reader()?;

    let headers_left = csv_rdr_left.byte_headers()?;
    let headers_right = csv_rdr_right.byte_headers()?;

    let primary_key_cols: Vec<usize> = match args.flag_key {
        None => vec![0],
        Some(s) => {
            // check if the key is a comma separated list of numbers
            if s.chars().all(|c: char| c.is_numeric() || c == ',') {
                s.split(',')
                    .map(str::parse::<usize>)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| CliError::Other(err.to_string()))?
            } else {
                // check if the key is a comma separated list of column names
                let left_key_indices = s.col_names_to_indices(',', headers_left, "left")?;

                // now check if the right CSV has the same selected colnames in the same locations
                let right_key_indices = s.col_names_to_indices(',', headers_right, "right")?;

                if left_key_indices != right_key_indices {
                    return fail_incorrectusage_clierror!(
                        "Column names on left and right CSVs do not match.\nUse `qsv select` to \
                         reorder the columns on the right CSV to match the order of the left \
                         CSV.\nThe key column indices on the left CSV are in index \
                         locations:\n{left_key_indices:?}\nand on the right CSV \
                         are:\n{right_key_indices:?}",
                    );
                }
                left_key_indices
            }
        },
    };

    let sort_cols = args
        .flag_sort_columns
        .map(|s| {
            // check if the sort columns are a comma separated list of numbers
            if s.chars().all(|c: char| c.is_numeric() || c == ',') {
                s.split(',')
                    .map(str::parse::<usize>)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| CliError::Other(err.to_string()))
            } else {
                // check if the sort columns is a comma separated list of column names
                let left_sort_indices = s.col_names_to_indices(',', headers_left, "left")?;

                // now check if the right CSV has the same selected colnames in the same locations
                let right_sort_indices = s.col_names_to_indices(',', headers_right, "right")?;

                if left_sort_indices != right_sort_indices {
                    return fail_incorrectusage_clierror!(
                        "Column names on left and right CSVs do not match.\nUse `qsv select` to \
                         reorder the columns on the right CSV to match the order of the left \
                         CSV.\nThe sort column indices on the left CSV are in index \
                         locations:\n{left_sort_indices:?}\nand on the right CSV \
                         are:\n{right_sort_indices:?}",
                    );
                }
                Ok(left_sort_indices)
            }
        })
        .transpose()?;

    let wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter_output)
        .writer()?;

    util::njobs(args.flag_jobs);

    // ===== DIFF PROCESSING =====

    let Ok(csv_diff) = CsvByteDiffBuilder::new()
        .primary_key_columns(primary_key_cols.clone())
        .build()
    else {
        return fail_clierror!("Cannot instantiate diff");
    };

    let mut diff_byte_records = csv_diff
        .diff(csv_rdr_left.into(), csv_rdr_right.into())
        .try_to_diff_byte_records()?;

    match sort_cols {
        Some(sort_cols) => {
            diff_byte_records
                .sort_by_columns(sort_cols)
                .map_err(|e| CliError::Other(e.to_string()))?;
        },
        None => {
            diff_byte_records.sort_by_line();
        },
    }

    let mut csv_diff_writer = CsvDiffWriter::new(
        wtr,
        args.flag_no_headers_output,
        args.flag_drop_equal_fields,
        primary_key_cols,
    );
    Ok(csv_diff_writer.write_diff_byte_records(diff_byte_records)?)
}

/// This function checks if the stats cache is available and if it is, performs "smart"
/// validation checks on the input files.
///
/// First, it check if the current options allow us to leverage the stats cache.
/// If so, it checks if the stats cache is available.
/// If it is, the function returns true if the files are identical per their fingerprint hashes,
/// allowing us to short-circuit the diff.
/// If the files are not identical, it performs additional "smart" validation checks.
fn check_stats_cache(args: &Args) -> Result<bool, CliError> {
    if args.flag_force
        || (args.flag_no_headers_left || args.flag_no_headers_right)
        || args
            .flag_key
            .as_ref()
            .is_some_and(|k| k.split(',').count() > 0)
    {
        // if force is set, or if no headers are set, or more than 1 key is set,
        // do not use stats cache
        return Ok(false);
    }

    // Set stats config for left file
    let left_schema_args = SchemaArgs {
        arg_input:            args.arg_input_left.clone(),
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        flag_jobs:            None,
        flag_polars:          false,
        flag_memcheck:        false,
        flag_force:           args.flag_force,
        flag_prefer_dmy:      false,
        flag_dates_whitelist: String::new(),
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_pattern_columns: SelectColumns::parse("")?,
        flag_stdout:          false,
    };

    // Set stats config for right file using same args
    let right_schema_args = SchemaArgs {
        arg_input: args.arg_input_right.clone(),
        ..left_schema_args.clone()
    };

    // Get stats records for both files
    if let (
        Ok((left_csv_fields, left_stats, left_dataset_stats)),
        Ok((_, right_stats, right_dataset_stats)),
    ) = (
        get_stats_records(&left_schema_args, StatsMode::Frequency),
        get_stats_records(&right_schema_args, StatsMode::Frequency),
    ) {
        // check if dataset stats are empty
        // if so, return false and proceed to "regular" diff processing
        if left_dataset_stats.is_empty() || right_dataset_stats.is_empty() {
            return Ok(false);
        }

        // If both files' fingerprint hashes match, files are identical. Short-circuit diff
        if left_dataset_stats.get("qsv__fingerprint_hash")
            == right_dataset_stats.get("qsv__fingerprint_hash")
        {
            return Ok(true);
        }

        // Check if row counts match, if we don't have a row count
        // stop validation and return false
        let left_dataset_rowcount = if let Some(rc) = left_dataset_stats.get("qsv__rowcount") {
            rc.parse::<f64>().unwrap_or_default() as u64
        } else {
            return Ok(false);
        };

        let right_dataset_rowcount = if let Some(rc) = right_dataset_stats.get("qsv__rowcount") {
            rc.parse::<f64>().unwrap_or_default() as u64
        } else {
            return Ok(false);
        };

        if left_dataset_rowcount != right_dataset_rowcount {
            return fail_incorrectusage_clierror!(
                "The number of rows in the left ({left_dataset_rowcount}) and right \
                 ({right_dataset_rowcount}) CSVs do not match."
            );
        }

        // If key column specified, check if it has all unique values in both files
        let mut colname_used_for_key = false;
        if let Some(key_col) = &args.flag_key {
            let idx = if key_col.chars().all(char::is_numeric) {
                key_col
                    .parse::<usize>()
                    .map_err(|err| CliError::Other(err.to_string()))?
            } else {
                // Handle column name case...
                colname_used_for_key = true;
                left_csv_fields
                    .iter()
                    .position(|field| field == key_col.as_bytes())
                    .unwrap_or_default()
            };

            // Check cardinality equals row count for key column in left file
            if let Some(left_col) = left_stats.get(idx)
                && left_col.cardinality != left_dataset_rowcount
            {
                return fail_incorrectusage_clierror!(
                    "Primary key values in left CSV are not unique in column {colname} \
                     (cardinality: {left_cardinality} != rowcount: {left_rowcount}). Use `qsv \
                     extdedup --select {colname} {left_input} --no-output` to check duplicates.",
                    colname = if colname_used_for_key {
                        key_col.to_string()
                    } else {
                        idx.to_string()
                    },
                    left_cardinality = left_col.cardinality,
                    left_rowcount = left_dataset_rowcount,
                    left_input = args.arg_input_left.as_ref().unwrap()
                );
            }

            // Check cardinality equals row count for key column in right file
            if let Some(right_col) = right_stats.get(idx)
                && right_col.cardinality != right_dataset_rowcount
            {
                return fail_incorrectusage_clierror!(
                    "Primary key values in right CSV are not unique in column {colname} \
                     (cardinality: {right_cardinality} != rowcount: {right_rowcount}). Use `qsv \
                     extdedup --select {colname} {right_input} --no-output` to check duplicates.",
                    colname = if colname_used_for_key {
                        key_col.to_string()
                    } else {
                        idx.to_string()
                    },
                    right_cardinality = right_col.cardinality,
                    right_rowcount = right_dataset_rowcount,
                    right_input = args.arg_input_right.as_ref().unwrap()
                );
            }
        }
    }
    Ok(false)
}

trait StringExt {
    fn col_names_to_indices<C: Into<char>>(
        &self,
        col_names_split_by: C,
        headers: &ByteRecord,
        msg_left_or_right: &str,
    ) -> Result<Vec<usize>, CliError>;
}

impl StringExt for String {
    fn col_names_to_indices<C: Into<char>>(
        &self,
        col_names_split_by: C,
        headers: &ByteRecord,
        msg_left_or_right: &str,
    ) -> Result<Vec<usize>, CliError> {
        self.split(col_names_split_by.into())
            .map(|col_name| {
                headers
                    .iter()
                    .position(|h| h == col_name.as_bytes())
                    .ok_or_else(|| {
                        CliError::Other(format!(
                            "Column name '{col_name}' not found on {msg_left_or_right} CSV"
                        ))
                    })
            })
            .collect::<Result<Vec<usize>, _>>()
    }
}

struct CsvDiffWriter<W: Write> {
    csv_writer:        csv::Writer<W>,
    no_headers:        bool,
    drop_equal_fields: bool,
    key_fields:        Vec<usize>,
}

impl<W: Write> CsvDiffWriter<W> {
    fn new(
        csv_writer: csv::Writer<W>,
        no_headers: bool,
        drop_equal_fields: bool,
        key_fields: impl IntoIterator<Item = usize>,
    ) -> Self {
        Self {
            csv_writer,
            no_headers,
            drop_equal_fields,
            key_fields: key_fields.into_iter().collect(),
        }
    }

    fn write_headers(&mut self, headers: &Headers, num_columns: Option<&usize>) -> csv::Result<()> {
        match (headers.headers_left(), headers.headers_right()) {
            (Some(lbh), Some(_rbh)) => {
                // currently, `diff` can only handle two CSVs that have the same
                // headers ordering, so in this case we can either choose the left
                // or right headers, because both are the same
                if !self.no_headers {
                    lbh.write_diffresult_header(&mut self.csv_writer)?;
                }
            },
            (Some(bh), None) | (None, Some(bh)) => {
                if !self.no_headers {
                    bh.write_diffresult_header(&mut self.csv_writer)?;
                }
            },
            (None, None) => {
                if let (Some(&num_cols), false) = (num_columns.filter(|&&c| c > 0), self.no_headers)
                {
                    let headers_generic = rename_headers_all_generic(num_cols);
                    let mut new_rdr = csv::Reader::from_reader(headers_generic.as_bytes());
                    let new_headers = new_rdr.byte_headers()?;
                    new_headers.write_diffresult_header(&mut self.csv_writer)?;
                }
            },
        }

        Ok(())
    }

    fn write_diff_byte_records(&mut self, diff_byte_records: DiffByteRecords) -> io::Result<()> {
        self.write_headers(
            diff_byte_records.headers(),
            diff_byte_records.num_columns().as_ref(),
        )?;
        for dbr in diff_byte_records {
            self.write_diff_byte_record(&dbr)?;
        }
        self.csv_writer.flush()?;
        Ok(())
    }

    #[inline]
    fn write_diff_byte_record(&mut self, diff_byte_record: &DiffByteRecord) -> csv::Result<()> {
        let add_sign: &[u8] = &b"+"[..];
        let remove_sign: &[u8] = &b"-"[..];

        match diff_byte_record {
            DiffByteRecord::Add(add) => {
                let mut vec = vec![add_sign];
                vec.extend(add.byte_record());
                self.csv_writer.write_record(vec)
            },
            DiffByteRecord::Modify {
                delete,
                add,
                field_indices,
            } => {
                let vec_del = if self.drop_equal_fields {
                    self.fill_modified_and_drop_equal_fields(
                        remove_sign,
                        delete.byte_record(),
                        field_indices.as_slice(),
                    )
                } else {
                    let mut tmp = vec![remove_sign];
                    tmp.extend(delete.byte_record());
                    tmp
                };

                self.csv_writer.write_record(vec_del)?;

                let vec_add = if self.drop_equal_fields {
                    self.fill_modified_and_drop_equal_fields(
                        add_sign,
                        add.byte_record(),
                        field_indices.as_slice(),
                    )
                } else {
                    let mut tmp = vec![add_sign];
                    tmp.extend(add.byte_record());
                    tmp
                };

                self.csv_writer.write_record(vec_add)
            },
            DiffByteRecord::Delete(del) => {
                let mut vec = vec![remove_sign];
                vec.extend(del.byte_record());
                self.csv_writer.write_record(vec)
            },
        }
    }

    fn fill_modified_and_drop_equal_fields<'a>(
        &self,
        prefix: &'a [u8],
        byte_record: &'a ByteRecord,
        modified_field_indices: &[usize],
    ) -> Vec<&'a [u8]> {
        let mut vec_to_fill = {
            // We start out with all fields set to an empty byte slice
            // (which end up as our equal fields).
            let mut tmp = vec![&b""[..]; byte_record.len() + 1 /* + 1, because we need to store our additional prefix*/];
            tmp.as_mut_slice()[0] = prefix;
            tmp
        };
        // key field values and modified field values should appear in the output
        for &key_field in self.key_fields.iter().chain(modified_field_indices) {
            // + 1 here, because of the prefix value (see above)
            vec_to_fill[key_field + 1] = &byte_record[key_field];
        }

        vec_to_fill
    }
}

trait WriteDiffResultHeader {
    fn write_diffresult_header<W: Write>(&self, csv_writer: &mut csv::Writer<W>)
    -> csv::Result<()>;
}

impl WriteDiffResultHeader for csv::ByteRecord {
    fn write_diffresult_header<W: Write>(
        &self,
        csv_writer: &mut csv::Writer<W>,
    ) -> csv::Result<()> {
        if !self.is_empty() {
            let mut new_header = vec![&b"diffresult"[..]];
            new_header.extend(self);
            csv_writer.write_record(new_header)?;
        }
        Ok(())
    }
}
