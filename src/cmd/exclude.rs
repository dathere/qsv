static USAGE: &str = r#"
Removes a set of CSV data from another set based on the specified columns.

Also can compute the intersection of two CSV sets with the -v flag.

Matching is always done by ignoring leading and trailing whitespace. By default,
matching is done case sensitively, but this can be disabled with the --ignore-case
flag.

The columns arguments specify the columns to match for each input. Columns can
be referenced by name or index, starting at 1. Specify multiple columns by
separating them with a comma. Specify a range of columns with `-`. Both
columns1 and columns2 must specify exactly the same number of columns.
(See 'qsv select --help' for the full syntax.)

Either <input1> or <input2> can be set to `-` to read from stdin, but not both.

Examples:

  # Remove all records in previously-processed.csv from records.csv
  qsv exclude id records.csv id previously-processed.csv

  # Remove all records in previously-processed.csv matching on multiple columns
  qsv exclude col1,col2 records.csv col1,col2 previously-processed.csv

  # Remove all records in previously-processed.csv matching on column ranges
  qsv exclude col1-col5 records.csv col1-col5 previously-processed.csv

  # Remove all records in previously-processed.csv with the same id from records.csv
  # and write to new-records.csv
  qsv exclude id records.csv id previously-processed.csv > new-records.csv

  # Remove all records in previously-processed.csv with the same id from records.csv
  # and write to new-records.csv
  qsv exclude id records.csv id previously-processed.csv --output new-records.csv

  # Get the intersection of records.csv and previously-processed.csv on id column
  # (i.e., only records present in both files)
  qsv exclude -v id records.csv id previously-processed.csv -o intersection.csv

  # Do a case insensitive exclusion on the id column
  qsv exclude --ignore-case id records.csv id previously-processed.csv

  # Read records.csv from stdin
  cat records.csv | qsv exclude id - id previously-processed.csv

  # Chain exclude with sort to create a new sorted records file without previously processed records
  qsv exclude id records.csv id previously-processed.csv | \
      qsv sort > new-sorted-records.csv

  # Chain exclude with sort and dedup to create a new sorted deduped records file
  qsv exclude id records.csv id previously-processed.csv | qsv sort | \
      qsv --sorted dedup > new-sorted-deduped-records.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_exclude.rs.

Usage:
    qsv exclude [options] <columns1> <input1> <columns2> <input2>
    qsv exclude --help

input arguments:
    <input1> is the file from which data will be removed.
    <input2> is the file containing the data to be removed from <input1>
     e.g. 'qsv exclude id records.csv id previously-processed.csv'
    Either input may be set to `-` to read from stdin, but not both.

exclude options:
    -i, --ignore-case      When set, matching is done case insensitively.
    -v, --invert           When set, matching rows will be the only ones included,
                           forming set intersection, instead of the ones discarded.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load <input2>
                           into memory using CONSERVATIVE heuristics.
"#;

use std::{io, path::Path, str};

use foldhash::{HashSet, HashSetExt};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter, SeekRead},
    select::{SelectColumns, Selection},
    util,
    util::ByteString,
};

const VALUE_SET_INITIAL_CAPACITY: usize = 10_000;

#[derive(Deserialize)]
struct Args {
    arg_columns1:     SelectColumns,
    arg_input1:       String,
    arg_columns2:     SelectColumns,
    arg_input2:       String,
    flag_invert:      bool,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_ignore_case: bool,
    flag_delimiter:   Option<Delimiter>,
    flag_memcheck:    bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.arg_input1 == "-" && args.arg_input2 == "-" {
        return fail_incorrectusage_clierror!(
            "Only one of <input1> or <input2> may be set to `-` to read from stdin."
        );
    }

    let mut state = args.new_io_state()?;
    state.write_headers()?;
    state.exclude(args.flag_invert)
}

struct IoState<R: io::Read, W: io::Write> {
    wtr:        csv::Writer<W>,
    rdr1:       csv::Reader<R>,
    sel1:       Selection,
    rdr2:       csv::Reader<R>,
    sel2:       Selection,
    no_headers: bool,
    casei:      bool,
}

impl<R: io::Read, W: io::Write> IoState<R, W> {
    fn write_headers(&mut self) -> CliResult<()> {
        if !self.no_headers {
            let headers = self.rdr1.byte_headers()?.clone();
            self.wtr.write_record(&headers)?;
        }
        Ok(())
    }

    fn exclude(mut self, invert: bool) -> CliResult<()> {
        let values = build_value_set(self.rdr2, &self.sel2, self.casei)?;
        let mut row = csv::ByteRecord::new();
        while self.rdr1.read_byte_record(&mut row)? {
            let key = get_row_key(&self.sel1, &row, self.casei);
            let matched = values.contains(&key);
            if matched == invert {
                self.wtr.write_record(row.iter())?;
            }
        }
        Ok(())
    }
}

impl Args {
    fn new_io_state(
        &self,
    ) -> CliResult<IoState<Box<dyn SeekRead + 'static>, Box<dyn io::Write + 'static>>> {
        let rconf1 = Config::new(Some(self.arg_input1.clone()).as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.arg_columns1.clone());
        let rconf2 = Config::new(Some(self.arg_input2.clone()).as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.arg_columns2.clone());

        // input2 is fully loaded into memory; guard against OOM.
        if let Some(path) = rconf2.path.as_ref() {
            util::mem_file_check(Path::new(path), false, self.flag_memcheck)?;
        }

        let mut rdr1 = rconf1.reader_file_stdin()?;
        let mut rdr2 = rconf2.reader_file_stdin()?;
        let (sel1, sel2) = self.get_selections(&rconf1, &mut rdr1, &rconf2, &mut rdr2)?;
        Ok(IoState {
            wtr: Config::new(self.flag_output.as_ref()).writer()?,
            rdr1,
            sel1,
            rdr2,
            sel2,
            no_headers: rconf1.no_headers,
            casei: self.flag_ignore_case,
        })
    }

    #[allow(clippy::unused_self)]
    fn get_selections<R: io::Read>(
        &self,
        rconf1: &Config,
        rdr1: &mut csv::Reader<R>,
        rconf2: &Config,
        rdr2: &mut csv::Reader<R>,
    ) -> CliResult<(Selection, Selection)> {
        let headers1 = rdr1.byte_headers()?;
        let headers2 = rdr2.byte_headers()?;
        let select1 = rconf1.selection(headers1)?;
        let select2 = rconf2.selection(headers2)?;
        if select1.len() != select2.len() {
            return fail_incorrectusage_clierror!(
                "Column selections must have the same number of columns, but found column \
                 selections with {} and {} columns.",
                select1.len(),
                select2.len()
            );
        }
        Ok((select1, select2))
    }
}

fn build_value_set<R: io::Read>(
    mut rdr: csv::Reader<R>,
    sel: &Selection,
    casei: bool,
) -> CliResult<HashSet<Vec<ByteString>>> {
    let mut values: HashSet<Vec<ByteString>> = HashSet::with_capacity(VALUE_SET_INITIAL_CAPACITY);
    let mut row = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut row)? {
        values.insert(get_row_key(sel, &row, casei));
    }
    Ok(values)
}

#[inline]
fn get_row_key(sel: &Selection, row: &csv::ByteRecord, casei: bool) -> Vec<ByteString> {
    sel.select(row).map(|v| util::transform(v, casei)).collect()
}
