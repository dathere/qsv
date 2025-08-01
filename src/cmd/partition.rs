static USAGE: &str = r#"
Partitions the given CSV data into chunks based on the value of a column.

See `split` command to split a CSV data by row count, by number of chunks or
by kb-size.

The files are written to the output directory with filenames based on the
values in the partition column and the `--filename` flag.

EXAMPLE:

Partition nyc311.csv file into separate files based on the value of the
"Borough" column in the current directory:
  $ qsv partition Borough . --filename "nyc311-{}.csv" nyc311.csv

will create the following files, each containing the data for each borough:
    nyc311-Bronx.csv
    nyc311-Brooklyn.csv
    nyc311-Manhattan.csv
    nyc311-Queens.csv
    nyc311-Staten_Island.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_partition.rs.

Usage:
    qsv partition [options] <column> <outdir> [<input>]
    qsv partition --help

partition arguments:
    <column>                 The column to use as a key for partitioning.
                             You can use the `--select` option to select
                             the column by name or index, but only one
                             column can be used for partitioning.
                             See `select` command for more details.
    <outdir>                 The directory to write the output files to.
    <input>                  The CSV file to read from. If not specified, then
                             the input will be read from stdin.

partition options:
    --filename <filename>    A filename template to use when constructing the
                             names of the output files.  The string '{}' will
                             be replaced by a value based on the partition column,
                             but sanitized for shell safety.
                             [default: {}.csv]
    -p, --prefix-length <n>  Truncate the partition column after the
                             specified number of bytes when creating the
                             output file.
    --drop                   Drop the partition column from results.

Common options:
    -h, --help               Display this message
    -n, --no-headers         When set, the first row will NOT be interpreted
                             as column names. Otherwise, the first row will
                             appear in all chunks as the header row.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use std::{
    collections::{HashSet, hash_map::Entry},
    fs, io,
    path::Path,
};

use foldhash::{HashMap, HashMapExt};
use regex::Regex;
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util::{self, FilenameTemplate},
};

#[derive(Clone, Deserialize)]
struct Args {
    arg_column:         SelectColumns,
    arg_input:          Option<String>,
    arg_outdir:         String,
    flag_filename:      FilenameTemplate,
    flag_prefix_length: Option<usize>,
    flag_drop:          bool,
    flag_no_headers:    bool,
    flag_delimiter:     Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    fs::create_dir_all(&args.arg_outdir)?;

    // It would be nice to support efficient parallel partitions, but doing
    // so would involve more complicated inter-thread communication, with
    // multiple readers and writers, and some way of passing buffers
    // between them.
    args.sequential_partition()
}

impl Args {
    /// Configuration for our reader.
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.arg_column.clone())
    }

    /// Get the column to use as a key.
    #[allow(clippy::unused_self)]
    fn key_column(&self, rconfig: &Config, headers: &csv::ByteRecord) -> CliResult<usize> {
        let select_cols = rconfig.selection(headers)?;
        if select_cols.len() == 1 {
            Ok(select_cols[0])
        } else {
            fail!("can only partition on one column")
        }
    }

    /// A basic sequential partition.
    fn sequential_partition(&self) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();
        let key_col = self.key_column(&rconfig, &headers)?;
        let mut r#gen = WriterGenerator::new(self.flag_filename.clone());

        let mut writers: HashMap<Vec<u8>, BoxedWriter> = HashMap::new();
        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            // Decide what file to put this in.
            let column = &row[key_col];
            let key = match self.flag_prefix_length {
                // We exceed --prefix-length, so ignore the extra bytes.
                Some(len) if len < column.len() => &column[0..len],
                _ => column,
            };
            let mut entry = writers.entry(key.to_vec());
            let wtr =
                match entry {
                    Entry::Occupied(ref mut occupied) => occupied.get_mut(),
                    Entry::Vacant(vacant) => {
                        // We have a new key, so make a new writer.
                        let mut wtr = r#gen.writer(&*self.arg_outdir, key)?;
                        if !rconfig.no_headers {
                            if self.flag_drop {
                                wtr.write_record(headers.iter().enumerate().filter_map(
                                    |(i, e)| if i == key_col { None } else { Some(e) },
                                ))?;
                            } else {
                                wtr.write_record(&headers)?;
                            }
                        }
                        vacant.insert(wtr)
                    },
                };
            if self.flag_drop {
                wtr.write_record(
                    row.iter().enumerate().filter_map(
                        |(i, e)| {
                            if i == key_col { None } else { Some(e) }
                        },
                    ),
                )?;
            } else {
                wtr.write_byte_record(&row)?;
            }
            wtr.flush()?;
        }
        Ok(())
    }
}

type BoxedWriter = csv::Writer<Box<dyn io::Write + 'static>>;

/// Generates unique filenames based on CSV values.
struct WriterGenerator {
    template:      FilenameTemplate,
    counter:       usize,
    used:          HashSet<String>,
    non_word_char: Regex,
}

impl WriterGenerator {
    fn new(template: FilenameTemplate) -> WriterGenerator {
        WriterGenerator {
            template,
            counter: 1,
            used: HashSet::new(),
            non_word_char: Regex::new(r"\W").unwrap(),
        }
    }

    /// Create a CSV writer for `key`.  Does not add headers.
    fn writer<P>(&mut self, path: P, key: &[u8]) -> io::Result<BoxedWriter>
    where
        P: AsRef<Path>,
    {
        let unique_value = self.unique_value(key);
        self.template.writer(path.as_ref(), &unique_value)
    }

    /// Generate a unique value for `key`, suitable for use in a
    /// "shell-safe" filename.  If you pass `key` twice, you'll get two
    /// different values.
    fn unique_value(&mut self, key: &[u8]) -> String {
        // Sanitize our key.
        let utf8 = String::from_utf8_lossy(key);
        let safe = self.non_word_char.replace_all(&utf8, "").into_owned();
        let base = if safe.is_empty() {
            "empty".to_owned()
        } else {
            safe
        };

        // Now check for collisions.
        if self.used.contains(&base) {
            loop {
                let candidate = format!("{}_{}", &base, self.counter);
                self.counter = self.counter.checked_add(1).unwrap_or_else(|| {
                    // We'll run out of other things long before we ever
                    // reach this, but we'll check just for correctness and
                    // completeness.
                    panic!("Cannot generate unique value")
                });
                if !self.used.contains(&candidate) {
                    self.used.insert(candidate.clone());
                    return candidate;
                }
            }
        } else {
            self.used.insert(base.clone());
            base
        }
    }
}
